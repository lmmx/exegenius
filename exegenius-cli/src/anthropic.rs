//! [`Model`] over the Anthropic Messages API, with structured outputs, streaming, and an
//! on-disk response cache keyed by the SHA-256 of the request body.

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::Duration;

use exegenius_core::config::ModelConfig;
use exegenius_core::model::{Model, Prompt};
use exegenius_core::{Error, Result};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use ureq::Agent;
use ureq::tls::{PemItem, RootCerts, TlsConfig, parse_pem};

const DEFAULT_URL: &str = "https://api.anthropic.com";
const FALLBACK_BETA: &str = "server-side-fallback-2026-07-01";
const RETRIES: u32 = 4;

pub struct Anthropic {
    agent: Agent,
    url: String,
    key: String,
    config: ModelConfig,
    cache_dir: PathBuf,
}

impl Anthropic {
    /// Reads the API key from `ANTHROPIC_API_KEY` and an optional endpoint override from
    /// `EXEGENIUS_API_URL`. Trusts the certificates in `SSL_CERT_FILE` when it is set.
    pub fn from_env(config: ModelConfig, cache_dir: PathBuf) -> anyhow::Result<Self> {
        let key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| anyhow::anyhow!("ANTHROPIC_API_KEY is not set"))?;
        let url = std::env::var("EXEGENIUS_API_URL").unwrap_or_else(|_| DEFAULT_URL.into());
        let mut builder = Agent::config_builder()
            .http_status_as_error(false)
            .timeout_connect(Some(Duration::from_secs(30)));
        if let Some(roots) = cert_file_roots()? {
            builder = builder.tls_config(TlsConfig::builder().root_certs(roots).build());
        }
        Ok(Self {
            agent: builder.build().into(),
            url: url.trim_end_matches('/').to_string(),
            key,
            config,
            cache_dir,
        })
    }

    fn body(&self, prompt: &Prompt) -> Value {
        let mut content = Vec::new();
        if let Some(context) = &prompt.context {
            content.push(json!({
                "type": "text",
                "text": context,
                "cache_control": {"type": "ephemeral"},
            }));
        }
        content.push(json!({"type": "text", "text": prompt.task}));
        let mut body = json!({
            "model": self.config.name,
            "max_tokens": self.config.max_tokens,
            "stream": true,
            "system": prompt.system,
            "messages": [{"role": "user", "content": content}],
            "output_config": {
                "effort": self.config.effort,
                "format": {"type": "json_schema", "schema": prompt.schema},
            },
        });
        if self.use_fallbacks() {
            body["fallbacks"] = json!("default");
        }
        body
    }

    /// Haiku has no server-side fallback, so the parameter is only sent for other models.
    fn use_fallbacks(&self) -> bool {
        self.config.fallbacks && !self.config.name.contains("haiku")
    }

    fn send(&self, body: &Value) -> Result<String> {
        let mut attempt = 0;
        loop {
            attempt += 1;
            let mut request = self
                .agent
                .post(format!("{}/v1/messages", self.url))
                .header("x-api-key", &self.key)
                .header("anthropic-version", "2023-06-01");
            if self.use_fallbacks() {
                request = request.header("anthropic-beta", FALLBACK_BETA);
            }
            let response = request
                .send_json(body)
                .map_err(|e| Error::Model(e.to_string()))?;
            let status = response.status().as_u16();
            if status == 200 {
                return read_stream(response.into_body().into_reader());
            }
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());
            let text = response.into_body().read_to_string().unwrap_or_default();
            let retryable = matches!(status, 408 | 429 | 500 | 502 | 503 | 504 | 529);
            if !retryable || attempt > RETRIES {
                return Err(Error::Model(format!("HTTP {status}: {text}")));
            }
            let wait = retry_after.unwrap_or(5 * 3u64.pow(attempt - 1));
            eprintln!("  model: HTTP {status}, retrying in {wait}s ({attempt}/{RETRIES})");
            std::thread::sleep(Duration::from_secs(wait));
        }
    }
}

impl Model for Anthropic {
    fn complete(&self, prompt: &Prompt) -> Result<Value> {
        let body = self.body(prompt);
        let bytes = serde_json::to_vec(&body).map_err(|e| Error::Model(e.to_string()))?;
        let hash: String = Sha256::digest(&bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        let path = self.cache_dir.join(format!("{hash}.json"));
        if let Ok(cached) = std::fs::read_to_string(&path)
            && let Ok(value) = serde_json::from_str(&cached)
        {
            return Ok(value);
        }
        let text = self.send(&body)?;
        let value: Value = serde_json::from_str(&text)
            .map_err(|e| Error::Response(format!("{e}; response text: {text}")))?;
        std::fs::create_dir_all(&self.cache_dir).map_err(|source| Error::Io {
            path: self.cache_dir.clone(),
            source,
        })?;
        let pretty = serde_json::to_string_pretty(&value).unwrap_or(text);
        std::fs::write(&path, pretty).map_err(|source| Error::Io {
            path: path.clone(),
            source,
        })?;
        Ok(value)
    }
}

/// Collect the text of a streamed response (server-sent events), failing on refusals,
/// truncation and stream errors.
fn read_stream(reader: impl std::io::Read) -> Result<String> {
    let mut text = String::new();
    let mut stop_reason = None;
    let (mut input, mut cached, mut output) = (0, 0, 0);
    for line in BufReader::new(reader).lines() {
        let line = line.map_err(|e| Error::Model(format!("reading the stream: {e}")))?;
        let Some(data) = line.strip_prefix("data:") else {
            continue;
        };
        let Ok(event) = serde_json::from_str::<Value>(data.trim()) else {
            continue;
        };
        match event["type"].as_str() {
            Some("message_start") => {
                let usage = &event["message"]["usage"];
                input = usage["input_tokens"].as_u64().unwrap_or(0);
                cached = usage["cache_read_input_tokens"].as_u64().unwrap_or(0);
            }
            Some("content_block_delta") if event["delta"]["type"] == "text_delta" => {
                text.push_str(event["delta"]["text"].as_str().unwrap_or(""));
            }
            Some("message_delta") => {
                if let Some(reason) = event["delta"]["stop_reason"].as_str() {
                    stop_reason = Some(reason.to_string());
                }
                output = event["usage"]["output_tokens"].as_u64().unwrap_or(output);
            }
            Some("error") => {
                return Err(Error::Model(format!("stream error: {}", event["error"])));
            }
            _ => {}
        }
    }
    eprintln!("  model: {input} input tokens ({cached} from cache), {output} output tokens");
    match stop_reason.as_deref() {
        Some("end_turn") => Ok(text),
        Some("max_tokens") => Err(Error::Model(
            "the response hit max_tokens; raise [model] max_tokens in exegenius.toml".into(),
        )),
        Some("refusal") => Err(Error::Model("the model declined the request".into())),
        Some(other) => Err(Error::Model(format!("unexpected stop reason {other:?}"))),
        None => Err(Error::Model(
            "the stream ended without a stop reason".into(),
        )),
    }
}

/// Root certificates from `SSL_CERT_FILE`, if set.
fn cert_file_roots() -> anyhow::Result<Option<RootCerts>> {
    let Ok(path) = std::env::var("SSL_CERT_FILE") else {
        return Ok(None);
    };
    let pem =
        std::fs::read(&path).map_err(|e| anyhow::anyhow!("reading SSL_CERT_FILE {path}: {e}"))?;
    let certs: Vec<_> = parse_pem(&pem)
        .filter_map(|item| match item {
            Ok(PemItem::Certificate(cert)) => Some(cert),
            _ => None,
        })
        .collect();
    if certs.is_empty() {
        anyhow::bail!("SSL_CERT_FILE {path} contains no certificates");
    }
    Ok(Some(RootCerts::new_with_certs(&certs)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_text_is_concatenated() {
        let stream = "event: message_start\n\
            data: {\"type\":\"message_start\",\"message\":{\"usage\":{\"input_tokens\":10}}}\n\n\
            data: {\"type\":\"content_block_start\",\"index\":0,\"content_block\":{\"type\":\"thinking\"}}\n\
            data: {\"type\":\"content_block_delta\",\"delta\":{\"type\":\"thinking_delta\",\"thinking\":\"x\"}}\n\
            data: {\"type\":\"content_block_delta\",\"delta\":{\"type\":\"text_delta\",\"text\":\"{\\\"a\\\"\"}}\n\
            data: {\"type\":\"content_block_delta\",\"delta\":{\"type\":\"text_delta\",\"text\":\": 1}\"}}\n\
            data: {\"type\":\"message_delta\",\"delta\":{\"stop_reason\":\"end_turn\"},\"usage\":{\"output_tokens\":5}}\n";
        assert_eq!(read_stream(stream.as_bytes()).unwrap(), "{\"a\": 1}");
    }

    #[test]
    fn refusals_and_truncation_fail() {
        let refusal =
            "data: {\"type\":\"message_delta\",\"delta\":{\"stop_reason\":\"refusal\"}}\n";
        assert!(read_stream(refusal.as_bytes()).is_err());
        let truncated =
            "data: {\"type\":\"message_delta\",\"delta\":{\"stop_reason\":\"max_tokens\"}}\n";
        assert!(read_stream(truncated.as_bytes()).is_err());
    }
}

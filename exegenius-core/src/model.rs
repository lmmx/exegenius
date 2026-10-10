//! The boundary between the pipeline and a language model.

use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::error::{Error, Result};

/// One model request. `context` is a large block shared by many requests in a step (the
/// sources), which an implementation can mark for prompt caching; `task` varies per call.
#[derive(Debug, Clone)]
pub struct Prompt {
    pub system: String,
    pub context: Option<String>,
    pub task: String,
    /// JSON schema the response must follow.
    pub schema: Value,
}

pub trait Model {
    /// Return a JSON value that follows `prompt.schema`.
    fn complete(&self, prompt: &Prompt) -> Result<Value>;
}

/// Send a prompt whose response schema is derived from `T`, and parse the response as `T`.
pub fn ask<T: DeserializeOwned + JsonSchema>(
    model: &dyn Model,
    system: &str,
    context: Option<&str>,
    task: &str,
) -> Result<T> {
    let prompt = Prompt {
        system: system.to_string(),
        context: context.map(str::to_string),
        task: task.to_string(),
        schema: schema_of::<T>(),
    };
    let value = model.complete(&prompt)?;
    serde_json::from_value(value).map_err(|e| Error::Response(e.to_string()))
}

/// JSON schema for `T`, restricted to what structured outputs accept: every object closed
/// (`additionalProperties: false`) with all its properties required, `oneOf` rewritten as
/// `enum` (for unit enums) or `anyOf`, and no numeric or length constraints or formats.
pub fn schema_of<T: JsonSchema>() -> Value {
    let mut schema = serde_json::to_value(schemars::schema_for!(T)).unwrap_or(Value::Null);
    if let Value::Object(map) = &mut schema {
        map.remove("$schema");
    }
    restrict(&mut schema);
    schema
}

const UNSUPPORTED: &[&str] = &[
    "minimum",
    "maximum",
    "exclusiveMinimum",
    "exclusiveMaximum",
    "multipleOf",
    "minLength",
    "maxLength",
    "minItems",
    "maxItems",
    "uniqueItems",
];

fn restrict(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for k in UNSUPPORTED {
                map.remove(*k);
            }
            if map.get("format").and_then(Value::as_str).is_some() {
                map.remove("format");
            }
            if let Some(Value::Array(variants)) = map.remove("oneOf") {
                if variants.iter().all(|v| v.get("const").is_some()) {
                    let values: Vec<Value> = variants.iter().map(|v| v["const"].clone()).collect();
                    let described: Vec<String> = variants
                        .iter()
                        .filter_map(|v| {
                            let desc = v.get("description")?.as_str()?;
                            Some(format!("{}: {desc}", v["const"].as_str().unwrap_or("")))
                        })
                        .collect();
                    map.insert("type".into(), Value::String("string".into()));
                    map.insert("enum".into(), Value::Array(values));
                    if !described.is_empty() && !map.contains_key("description") {
                        map.insert("description".into(), Value::String(described.join("\n")));
                    }
                } else {
                    map.insert("anyOf".into(), Value::Array(variants));
                }
            }
            if let Some(Value::Object(props)) = map.get("properties") {
                let required: Vec<Value> = props.keys().map(|k| Value::String(k.clone())).collect();
                map.insert("required".into(), Value::Array(required));
                map.insert("additionalProperties".into(), Value::Bool(false));
            }
            for v in map.values_mut() {
                restrict(v);
            }
        }
        Value::Array(items) => items.iter_mut().for_each(restrict),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::AuditResponse;
    use crate::extract::ExtractResponse;

    fn objects(value: &Value, out: &mut Vec<serde_json::Map<String, Value>>) {
        match value {
            Value::Object(map) => {
                if map.contains_key("properties") {
                    out.push(map.clone());
                }
                map.values().for_each(|v| objects(v, out));
            }
            Value::Array(items) => items.iter().for_each(|v| objects(v, out)),
            _ => {}
        }
    }

    #[test]
    fn unit_enums_become_string_enums() {
        let schema = schema_of::<AuditResponse>();
        let kind = &schema["$defs"]["GapKind"];
        assert_eq!(kind["type"], "string");
        assert_eq!(kind["enum"], serde_json::json!(["term", "symbol", "step"]));
        assert!(kind["description"].as_str().unwrap().starts_with("term: "));
    }

    #[test]
    fn schemas_are_closed_and_fully_required() {
        for schema in [schema_of::<ExtractResponse>(), schema_of::<AuditResponse>()] {
            assert!(schema.get("$schema").is_none());
            let mut found = Vec::new();
            objects(&schema, &mut found);
            assert!(!found.is_empty());
            for obj in found {
                assert_eq!(obj["additionalProperties"], Value::Bool(false));
                let props = obj["properties"].as_object().unwrap();
                assert_eq!(obj["required"].as_array().unwrap().len(), props.len());
            }
            let text = schema.to_string();
            assert!(!text.contains("\"format\"") && !text.contains("oneOf"));
        }
    }
}

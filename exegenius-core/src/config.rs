//! The project configuration file, `exegenius.toml`.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{Error, Result, read};

pub const FILE_NAME: &str = "exegenius.toml";

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Title of the generated wiki; defaults to the target concept's name.
    #[serde(default)]
    pub title: Option<String>,
    /// What the wiki should explain, in plain words.
    pub target: String,
    /// Maximum number of audit rounds.
    #[serde(default = "default_rounds")]
    pub rounds: u32,
    pub reader: Reader,
    pub sources: Vec<SourceSpec>,
}

fn default_rounds() -> u32 {
    3
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reader {
    /// What the reader already knows, in plain words.
    pub knows: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceSpec {
    pub id: String,
    /// Path to a plain text or markdown file, relative to the project directory.
    pub path: PathBuf,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let config: Config = toml::from_str(&read(path)?).map_err(|source| Error::Config {
            path: path.to_path_buf(),
            source,
        })?;
        config.check(path)?;
        Ok(config)
    }

    fn check(&self, path: &Path) -> Result<()> {
        let invalid = |msg: String| Error::Invalid(format!("{}: {msg}", path.display()));
        if self.target.trim().is_empty() {
            return Err(invalid("`target` is empty".into()));
        }
        if self.sources.is_empty() {
            return Err(invalid("no [[sources]] given".into()));
        }
        let mut seen = HashSet::new();
        for source in &self.sources {
            if !seen.insert(source.id.as_str()) {
                return Err(invalid(format!("duplicate source id `{}`", source.id)));
            }
        }
        Ok(())
    }
}

pub const TEMPLATE: &str = r#"# exegenius project configuration

# Title of the generated wiki (defaults to the target concept's name)
# title = "Integer multiplication below n log n"

# What the wiki should explain, in plain words
target = "The main result of the paper"

# Maximum number of audit rounds
rounds = 3

[reader]
# What the reader already knows, in plain words
knows = "Undergraduate computer science: big-O notation, polynomials, complex numbers"

# One entry per source document (plain text or markdown; convert PDFs to text first)
[[sources]]
id = "paper"
path = "sources/paper.txt"
# title = "Paper title"
# url = "https://example.org/paper"
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_parses() {
        let config: Config = toml::from_str(TEMPLATE).unwrap();
        config.check(Path::new(FILE_NAME)).unwrap();
        assert_eq!(config.rounds, 3);
        assert_eq!(config.sources[0].path, PathBuf::from("sources/paper.txt"));
    }

    #[test]
    fn rounds_default_to_three() {
        let config: Config = toml::from_str(
            "target = \"t\"\n[reader]\nknows = \"k\"\n[[sources]]\nid = \"a\"\npath = \"a.txt\"\n",
        )
        .unwrap();
        assert_eq!(config.rounds, 3);
    }
}

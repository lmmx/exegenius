//! The project configuration file, `exegenius.toml`.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{Error, Result, read};

pub const FILE_NAME: &str = "exegenius.toml";

/// The `target` in [`TEMPLATE`], which `exegenius next` refuses to start from.
pub const TEMPLATE_TARGET: &str = "The main result of the paper";

/// The `reader.knows` in [`TEMPLATE`], which `exegenius next` also refuses.
pub const TEMPLATE_KNOWS: &str =
    "Undergraduate computer science: big-O notation, polynomials, complex numbers";

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
    /// Simulated readers run once the wiki is written.
    #[serde(default)]
    pub simulation: Simulation,
    /// Further reader profiles for the simulation, beside `reader`.
    #[serde(default)]
    pub readers: Vec<Persona>,
    #[serde(default)]
    pub site: Site,
    /// A pilot: a run on the part of the target that fits in a few pages.
    #[serde(default)]
    pub pilot: Option<Pilot>,
}

/// A small run, to see whether the audit rounds converge before a full run.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pilot {
    /// Most pages the pilot plans when it starts; extend may add more.
    pub pages: usize,
}

/// Settings for the built site.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Site {
    /// Whether source references on pages start out shown or hidden; readers can toggle
    /// them either way.
    #[serde(default)]
    pub citations: Visibility,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    #[default]
    Hidden,
    Shown,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Simulation {
    /// Rounds of simulated reading: each round but the last is followed by repairs.
    /// 0 turns the simulation off.
    #[serde(default = "default_simulation_rounds")]
    pub rounds: u32,
    /// Most pages a simulated reader may open for one goal.
    #[serde(default = "default_budget")]
    pub budget: u32,
    /// Give the concepts the wiki teaches invented names in what simulated readers see.
    #[serde(default = "default_true")]
    pub nonce: bool,
    /// After the last round, hide this many of the sentences that carried most key points,
    /// each with a control sentence, and rerun the goals they supported. 0 skips it.
    #[serde(default)]
    pub ablations: u32,
}

impl Default for Simulation {
    fn default() -> Self {
        Self {
            rounds: default_simulation_rounds(),
            budget: default_budget(),
            nonce: true,
            ablations: 0,
        }
    }
}

fn default_simulation_rounds() -> u32 {
    2
}

fn default_budget() -> u32 {
    15
}

fn default_true() -> bool {
    true
}

/// A reader profile for the simulation.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Persona {
    /// Short name, used in task ids and the report.
    pub name: String,
    /// What this reader already knows, in plain words.
    pub knows: String,
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
    /// The simulated readers: the configured reader, then any further profiles.
    pub fn personas(&self) -> Vec<Persona> {
        let mut personas = vec![Persona {
            name: "reader".into(),
            knows: self.reader.knows.clone(),
        }];
        personas.extend(self.readers.iter().cloned());
        personas
    }

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
        if self.pilot.as_ref().is_some_and(|p| p.pages == 0) {
            return Err(invalid("[pilot] pages must be at least 1".into()));
        }
        let mut seen = HashSet::new();
        for source in &self.sources {
            if !seen.insert(source.id.as_str()) {
                return Err(invalid(format!("duplicate source id `{}`", source.id)));
            }
        }
        let mut names = HashSet::new();
        for persona in self.personas() {
            let name = crate::text::slug(&persona.name);
            if name.is_empty() || !names.insert(name) {
                return Err(invalid(format!(
                    "reader names must be distinct and non-empty: `{}`",
                    persona.name
                )));
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
# url = "https://example.org/paper"   # a DOI, arXiv or stable URL, so readers can check it

# Simulated readers, run once the wiki is written: each tries to answer goal questions
# using only the wiki, and what they get stuck on is repaired before the next round.
# [simulation]
# rounds = 2      # 0 turns the simulation off
# budget = 15     # most pages a reader may open per goal
# nonce = true    # invented names for the concepts the wiki teaches
# ablations = 0   # pivotal sentences to hide, each with a control, after the last round

# Source references on pages ("see p. 5"): hidden until a reader shows them, or shown
# [site]
# citations = "hidden"

# A pilot: a small run on the part of the target whose pages fit in `pages`, to see in
# `exegenius status` whether the audit rounds converge before paying for a full run.
# Remove it to go on to the full wiki.
# [pilot]
# pages = 8

# Further reader profiles to simulate, beside [reader]
# [[readers]]
# name = "newcomer"
# knows = "High-school algebra"
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_parses() {
        let config: Config = toml::from_str(TEMPLATE).unwrap();
        config.check(Path::new(FILE_NAME)).unwrap();
        assert_eq!(config.rounds, 3);
        assert_eq!(config.target, TEMPLATE_TARGET);
        assert_eq!(config.reader.knows, TEMPLATE_KNOWS);
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

    #[test]
    fn pilot_needs_a_page() {
        let base =
            "target = \"t\"\n[reader]\nknows = \"k\"\n[[sources]]\nid = \"a\"\npath = \"a.txt\"\n";
        let config: Config = toml::from_str(&format!("{base}[pilot]\npages = 8\n")).unwrap();
        assert_eq!(config.pilot.map(|p| p.pages), Some(8));
        let empty: Config = toml::from_str(&format!("{base}[pilot]\npages = 0\n")).unwrap();
        assert!(empty.check(Path::new(FILE_NAME)).is_err());
    }
}

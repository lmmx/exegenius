//! The concept graph: concepts, prerequisite edges, supporting quotes, the symbols each
//! page binds and what it uses each prerequisite for, and the work the audit has queued
//! (candidate concepts and page rewrites).

use std::collections::{BTreeMap, HashMap, HashSet};

use petgraph::algo::tarjan_scc;
use petgraph::graphmap::DiGraphMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::relations::Edge;
use crate::symbols::{Binding, PageBinding};
use crate::text::key;

/// One concept. This is also the shape of a concept in an extract answer, so the field
/// docs double as instructions in the task file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Concept {
    /// Lowercase slug, unique across the graph, e.g. "chirp-identity".
    pub id: String,
    /// Display name in plain text, e.g. "Chirp identity". Write symbols in Unicode (κ, ω,
    /// ≤, n log n), not LaTeX or spelled-out Greek: names appear in the navigation, which
    /// does not render mathematics.
    pub name: String,
    /// Only for a name with mathematics in it: the name with the mathematics in LaTeX, e.g.
    /// "Polynomial ring modulo $y^r + 1$". It is the heading of the concept's page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// What each symbol in the title's mathematics denotes, one binding per symbol. A reader
    /// who knows the concept is shown these, so give every symbol, however minor.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub symbols: Vec<Binding>,
    /// Other names and abbreviations the sources use for this concept.
    pub aliases: Vec<String>,
    /// One precise sentence saying what the concept is, in plain text with Unicode symbols
    /// (it is shown as a tooltip on links to the concept).
    pub gloss: String,
    /// Ids of the concepts a reader must understand before this one.
    pub requires: Vec<String>,
    /// True if the reader profile already covers this concept.
    pub known: bool,
    /// Short quotes copied exactly from the sources that state or use this concept.
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    /// Id of the source the quote comes from.
    pub source: String,
    /// Text copied character for character from that source.
    pub quote: String,
}

/// A term the audit found that matches no concept, waiting for `extract` to resolve it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Candidate {
    pub term: String,
    pub page: String,
    pub quote: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Graph {
    pub target: String,
    pub concepts: Vec<Concept>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub candidates: Vec<Candidate>,
    /// Pages queued for rewriting, with the notes to pass to the next write.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub rewrite: BTreeMap<String, Vec<String>>,
    /// The symbols each page binds, as its writer stated them.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub bindings: BTreeMap<String, Vec<PageBinding>>,
    /// What each page uses its prerequisite pages for, as its writer stated it.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub relations: BTreeMap<String, Vec<Edge>>,
    /// In a pilot, the concept the pilot explains in place of the target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pilot: Option<String>,
    /// What the reader profile covers for these sources, from the background task.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
}

impl Graph {
    /// The concept this run explains: the pilot's concept in a pilot, else the target.
    pub fn goal(&self) -> &str {
        self.pilot.as_deref().unwrap_or(&self.target)
    }

    pub fn get(&self, id: &str) -> Option<&Concept> {
        self.concepts.iter().find(|c| c.id == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Concept> {
        self.concepts.iter_mut().find(|c| c.id == id)
    }

    /// Whether `to` is reachable from `from` along `requires` edges.
    pub fn reaches(&self, from: &str, to: &str) -> bool {
        let mut stack = vec![from];
        let mut seen = HashSet::new();
        while let Some(id) = stack.pop() {
            if id == to {
                return true;
            }
            if seen.insert(id)
                && let Some(c) = self.get(id)
            {
                stack.extend(c.requires.iter().map(String::as_str));
            }
        }
        false
    }

    /// Add the prerequisite edge `page -> concept`. Returns false, leaving the graph
    /// unchanged, if the edge already exists or would create a cycle.
    pub fn add_requires(&mut self, page: &str, concept: &str) -> bool {
        if page == concept || self.reaches(concept, page) {
            return false;
        }
        match self.get_mut(page) {
            Some(c) if !c.requires.iter().any(|r| r == concept) => {
                c.requires.push(concept.to_string());
                true
            }
            _ => false,
        }
    }

    pub fn queue_rewrite(&mut self, page: &str, note: Option<String>) {
        let notes = self.rewrite.entry(page.to_string()).or_default();
        if let Some(note) = note
            && !notes.contains(&note)
        {
            notes.push(note);
        }
    }

    /// Structural errors: duplicate ids, unknown `requires` ids, a missing target, cycles.
    pub fn check(&self) -> Vec<String> {
        let mut errors = Vec::new();
        let mut ids = HashSet::new();
        for c in &self.concepts {
            if c.id.is_empty() {
                errors.push(format!("concept {:?} has an empty id", c.name));
            } else if !ids.insert(c.id.as_str()) {
                errors.push(format!("duplicate concept id `{}`", c.id));
            }
        }
        for c in &self.concepts {
            for r in &c.requires {
                if !ids.contains(r.as_str()) {
                    errors.push(format!(
                        "`{}` requires `{r}`, which is not a concept id",
                        c.id
                    ));
                }
            }
        }
        if !ids.contains(self.target.as_str()) {
            errors.push(format!("target `{}` is not a concept id", self.target));
        }
        if let Some(pilot) = &self.pilot
            && !ids.contains(pilot.as_str())
        {
            errors.push(format!("pilot `{pilot}` is not a concept id"));
        }
        let mut g = DiGraphMap::<&str, ()>::new();
        for c in &self.concepts {
            g.add_node(c.id.as_str());
            for r in &c.requires {
                if ids.contains(r.as_str()) {
                    g.add_edge(c.id.as_str(), r.as_str(), ());
                }
            }
        }
        for component in tarjan_scc(&g) {
            let cyclic = component.len() > 1 || g.contains_edge(component[0], component[0]);
            if cyclic {
                let mut members = component.clone();
                members.sort_unstable();
                errors.push(format!("prerequisite cycle among: {}", members.join(", ")));
            }
        }
        errors
    }
}

/// Lookup from term keys (names, aliases, ids) to concept ids.
#[derive(Debug, Default)]
pub struct TermIndex(HashMap<String, String>);

impl TermIndex {
    pub fn new<'a>(concepts: impl IntoIterator<Item = &'a Concept>) -> Self {
        let mut index = Self::default();
        for c in concepts {
            index.insert(c);
        }
        index
    }

    pub fn insert(&mut self, c: &Concept) {
        let names = std::iter::once(&c.name)
            .chain(&c.aliases)
            .chain(std::iter::once(&c.id));
        for name in names {
            let k = key(name);
            if !k.is_empty() {
                self.0.entry(k).or_insert_with(|| c.id.clone());
            }
        }
    }

    /// The concept a term names, trying a trailing-"s" plural as a fallback.
    pub fn lookup(&self, term: &str) -> Option<&str> {
        let k = key(term);
        self.0
            .get(&k)
            .or_else(|| k.strip_suffix('s').and_then(|s| self.0.get(s)))
            .map(String::as_str)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub fn concept(id: &str, requires: &[&str], known: bool) -> Concept {
        Concept {
            id: id.to_string(),
            name: id.replace('-', " "),
            title: None,
            symbols: vec![],
            aliases: vec![],
            gloss: format!("gloss of {id}"),
            requires: requires.iter().map(|s| s.to_string()).collect(),
            known,
            evidence: vec![],
        }
    }

    pub fn graph(target: &str, concepts: Vec<Concept>) -> Graph {
        Graph {
            target: target.to_string(),
            concepts,
            ..Graph::default()
        }
    }

    #[test]
    fn check_reports_dangling_target_and_cycles() {
        let g = graph(
            "missing",
            vec![
                concept("a", &["b"], false),
                concept("b", &["a"], false),
                concept("c", &["nope"], false),
            ],
        );
        let errors = g.check();
        assert!(errors.iter().any(|e| e.contains("`nope`")));
        assert!(errors.iter().any(|e| e.contains("target `missing`")));
        assert!(errors.iter().any(|e| e.contains("cycle among: a, b")));
    }

    #[test]
    fn add_requires_refuses_cycles() {
        let mut g = graph(
            "a",
            vec![concept("a", &["b"], false), concept("b", &[], false)],
        );
        assert!(!g.add_requires("b", "a"));
        assert!(!g.add_requires("a", "b"));
        assert!(g.check().is_empty());
    }

    #[test]
    fn term_index_matches_aliases_and_plurals() {
        let mut c = concept("discrete-fourier-transform", &[], false);
        c.aliases.push("DFT".into());
        let index = TermIndex::new([&c]);
        assert_eq!(index.lookup("dft"), Some("discrete-fourier-transform"));
        assert_eq!(index.lookup("DFTs"), Some("discrete-fourier-transform"));
        assert_eq!(
            index.lookup("Discrete Fourier transform"),
            Some("discrete-fourier-transform")
        );
        assert_eq!(index.lookup("FFT"), None);
    }
}

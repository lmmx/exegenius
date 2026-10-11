//! The background task: what the reader profile covers, made concrete for these sources.
//!
//! A profile is a few words ("undergraduate computer science: big-O notation, polynomials")
//! that stand for a level of study, and its examples do not list all it covers. Read as a
//! complete list, it makes every agent downstream treat the rest of that level's teaching as
//! unknown, and each audit round turns it into new pages. The background task reads the
//! sources once, after extraction, and says what a reader at the profile's level has been
//! taught that the sources rely on:
//!
//! - a summary of the subjects and how far into each, shown with the profile wherever the
//!   profile is shown;
//! - known concepts, added to the graph, which audits drop as terms and extend resolves to.
//!
//! The task only adds background the graph lacks. It cannot mark a concept extraction
//! planned a page for as known, so it can stop the graph growing but never shrink the wiki.
//! Code checks names, never what the profile covers. `next` shows the result once before
//! going on, so that a person following the run can correct it
//! ([`crate::task::redo_background`]).

use std::collections::HashSet;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::Persona;
use crate::error::{Error, Result};
use crate::graph::{Concept, Graph, TermIndex};
use crate::project::Project;
use crate::text::slug;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BackgroundAnswer {
    /// Two to four sentences: the subjects this reader has studied that bear on the sources,
    /// and how far into each.
    pub summary: String,
    /// Concepts the reader knows that the sources rely on and no existing concept covers.
    pub known: Vec<KnownConcept>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct KnownConcept {
    /// Display name in plain text, e.g. "Fourier series".
    pub name: String,
    /// Other names and abbreviations for it, in the sources or in standard texts.
    pub aliases: Vec<String>,
    /// One precise sentence saying what it is.
    pub gloss: String,
}

/// A background answer, as applied to the graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Background {
    pub summary: String,
    /// Ids of the known concepts it added.
    pub added: Vec<String>,
}

pub const BACKGROUND_INSTRUCTIONS: &str = "\
Say what a specific reader already knows, for these sources. The reader profile describes \
the reader in a few words, and its examples do not list everything it covers: it stands for \
a level of study. Make it concrete for the sources.

The sources were written for readers of their own, and use without explanation what those \
readers know. Read the sources and find that background: the notions, notation and standard \
results they use without defining. For each, decide whether a reader at the profile's level \
has been taught it, in the profile's subject or in the subjects that subject rests on (an \
undergraduate degree in physics teaches linear algebra, Fourier series and probability as \
well as mechanics, whether or not a profile names them).

- The reader knows what their level of study teaches to everyone at that level. Do not \
withhold it because the profile does not name it.
- The reader does not know what is taught only in some courses, or in later or specialist \
study, nor what the sources introduce, define or prove: that is what the wiki explains. When \
unsure, count it as not known.
- The existing concepts were planned by an earlier task and stay as they are: this answer \
only adds background they lack.
- `summary`: the subjects and how far into each this reader has gone, as they bear on the \
sources. Everyone who writes or checks the wiki is shown it with the profile.
- `known`: each concept from that background that the reader knows and no existing concept \
covers, with the aliases a writer might use for it. Include what the sources use and what an \
explanation of them would naturally use. Known concepts get no page.";

/// The background task, with a person's correction of an earlier answer, if any.
pub fn background_task(project: &Project, graph: &Graph, correction: Option<&str>) -> String {
    let concepts = graph
        .concepts
        .iter()
        .map(|c| {
            let known = if c.known {
                " (known to the reader)"
            } else {
                ""
            };
            format!("- `{}`: {}{known}: {}", c.id, c.name, c.gloss)
        })
        .collect::<Vec<_>>()
        .join("\n");
    let correction = correction.map_or(String::new(), |c| {
        format!(
            "## Correction\n\nA person following the run said this of an earlier answer; \
             follow it:\n\n{}\n\n",
            c.trim()
        )
    });
    format!(
        "## Target\n\n{}\n\n## Reader profile\n\n{}\n\n{correction}## Existing concepts\n\n\
         {concepts}\n\n## Sources (read them from disk)\n\n{}\n",
        project.config.target.trim(),
        project.config.reader.knows.trim(),
        project.sources_listing()
    )
}

/// Check a background answer and apply it to the graph.
pub fn apply_background(graph: &mut Graph, answer: BackgroundAnswer) -> Result<()> {
    let mut errors = Vec::new();
    let summary = answer.summary.trim();
    if summary.is_empty() {
        errors.push("the summary is empty".to_string());
    }
    let existing = TermIndex::new(&graph.concepts);
    let mut ids: HashSet<String> = graph.concepts.iter().map(|c| c.id.clone()).collect();
    let mut added = Vec::new();
    for k in &answer.known {
        let id = slug(&k.name);
        if id.is_empty() {
            errors.push(format!("known concept {:?} has no name", k.name));
            continue;
        }
        if let Some(c) = std::iter::once(&k.name)
            .chain(&k.aliases)
            .find_map(|t| existing.lookup(t))
        {
            errors.push(format!(
                "\"{}\" is the existing concept `{c}`, which stays as planned: leave it out",
                k.name
            ));
            continue;
        }
        if !ids.insert(id.clone()) {
            errors.push(format!("\"{}\" is listed twice", k.name));
            continue;
        }
        added.push(Concept {
            id,
            name: k.name.trim().to_string(),
            title: None,
            symbols: Vec::new(),
            aliases: k.aliases.clone(),
            gloss: k.gloss.trim().to_string(),
            requires: Vec::new(),
            known: true,
            evidence: Vec::new(),
        });
    }
    if !errors.is_empty() {
        return Err(Error::Invalid(format!(
            "answer rejected:\n{}",
            errors
                .iter()
                .map(|e| format!("- {e}"))
                .collect::<Vec<_>>()
                .join("\n")
        )));
    }
    graph.background = Some(Background {
        summary: summary.to_string(),
        added: added.iter().map(|c| c.id.clone()).collect(),
    });
    graph.concepts.extend(added);
    Ok(())
}

/// Take a background answer out of the graph, so that the task can be given again.
pub fn undo(graph: &mut Graph) {
    if let Some(background) = graph.background.take() {
        graph.concepts.retain(|c| !background.added.contains(&c.id));
    }
}

/// The reader profile, with what the background task said it covers for these sources.
pub fn profile(project: &Project, graph: &Graph) -> String {
    let knows = project.config.reader.knows.trim();
    match &graph.background {
        Some(b) => format!("{knows}\n\nFor these sources: {}", b.summary),
        None => knows.to_string(),
    }
}

/// The simulated readers, with the configured reader's profile made concrete as in
/// [`profile`]; further profiles are simulated as written.
pub fn personas(project: &Project, graph: &Graph) -> Vec<Persona> {
    let mut personas = project.config.personas();
    if let Some(reader) = personas.first_mut() {
        reader.knows = profile(project, graph);
    }
    personas
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::tests::{concept, graph};

    fn known(name: &str, aliases: &[&str]) -> KnownConcept {
        KnownConcept {
            name: name.into(),
            aliases: aliases.iter().map(|a| a.to_string()).collect(),
            gloss: "A gloss.".into(),
        }
    }

    fn two() -> Graph {
        graph(
            "target",
            vec![
                concept("target", &["modular-arithmetic"], false),
                concept("modular-arithmetic", &[], false),
            ],
        )
    }

    #[test]
    fn background_adds_known_concepts_and_can_be_undone() {
        let mut g = two();
        let answer = BackgroundAnswer {
            summary: "Linear algebra and elementary number theory.".into(),
            known: vec![known("Linear map", &["linear transformation"])],
        };
        apply_background(&mut g, answer).unwrap();
        let map = g.get("linear-map").unwrap();
        assert!(map.known && map.requires.is_empty());
        assert!(!g.get("modular-arithmetic").unwrap().known);
        assert_eq!(g.background.as_ref().unwrap().added, vec!["linear-map"]);
        undo(&mut g);
        assert!(g.background.is_none());
        assert_eq!(g, two());
    }

    #[test]
    fn background_leaves_planned_concepts_as_they_are() {
        let mut g = two();
        let answer = BackgroundAnswer {
            summary: " ".into(),
            known: vec![known("Clock arithmetic", &["modular arithmetic"])],
        };
        let error = apply_background(&mut g, answer).unwrap_err().to_string();
        assert!(error.contains("summary is empty"), "{error}");
        assert!(
            error.contains("existing concept `modular-arithmetic`, which stays as planned"),
            "{error}"
        );
        assert_eq!(g, two());
    }
}

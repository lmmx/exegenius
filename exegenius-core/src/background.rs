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
//! - known concepts, added to the graph, which audits drop as terms and extend resolves to;
//! - existing concepts the profile covers, which become known.
//!
//! The level comes from the sources as well as the profile: what the sources use without
//! explaining is what their own readers know, and the task decides which of it this reader
//! knows too. Code checks ids, never what the profile covers.

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
    /// Ids of existing concepts the reader knows that are not yet marked known.
    pub covered: Vec<String>,
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

- The reader knows what their level of study standardly teaches. Do not withhold it because \
the profile does not name it: a wiki that explains what its reader was taught is padded, and \
every page it adds needs pages of its own.
- The reader does not know what is taught only in later or specialist study, nor what the \
sources introduce, define or prove: that is what the wiki explains.
- `summary`: the subjects and how far into each this reader has gone, as they bear on the \
sources. Everyone who writes or checks the wiki is shown it with the profile.
- `known`: each concept from that background that the reader knows and no existing concept \
covers, with the aliases a writer might use for it. Include what the sources use and what an \
explanation of them would naturally use. Known concepts get no page.
- `covered`: ids of existing concepts the reader knows that are not marked known. Leave out \
the target and anything the sources introduce.";

pub fn background_task(project: &Project, graph: &Graph) -> String {
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
    format!(
        "## Target\n\n{}\n\n## Reader profile\n\n{}\n\n## Existing concepts\n\n{concepts}\n\n\
         ## Sources (read them from disk)\n\n{}\n",
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
                "\"{}\" is the existing concept `{c}`: list its id under `covered` if the \
                 reader knows it, and leave it out of `known`",
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
    let covered: Vec<String> = answer.covered.iter().map(|c| slug(c)).collect();
    for id in &covered {
        if graph.get(id).is_none() {
            errors.push(format!(
                "`{id}` under `covered` is not an existing concept id"
            ));
        } else if *id == graph.target || Some(id) == graph.pilot.as_ref() {
            errors.push(format!(
                "`{id}` is what the wiki explains, so the reader cannot know it"
            ));
        }
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
    for id in &covered {
        if let Some(c) = graph.get_mut(id) {
            c.known = true;
            c.requires.clear();
        }
    }
    graph.concepts.extend(added);
    graph.background = Some(summary.to_string());
    Ok(())
}

/// The reader profile, with what the background task said it covers for these sources.
pub fn profile(project: &Project, graph: &Graph) -> String {
    let knows = project.config.reader.knows.trim();
    match &graph.background {
        Some(summary) => format!("{knows}\n\nFor these sources: {summary}"),
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

    #[test]
    fn background_adds_known_concepts_and_marks_covered_ones() {
        let mut g = graph(
            "target",
            vec![
                concept("target", &["modular-arithmetic"], false),
                concept("modular-arithmetic", &[], false),
            ],
        );
        let answer = BackgroundAnswer {
            summary: "Linear algebra and elementary number theory.".into(),
            known: vec![known("Linear map", &["linear transformation"])],
            covered: vec!["modular-arithmetic".into()],
        };
        apply_background(&mut g, answer).unwrap();
        let map = g.get("linear-map").unwrap();
        assert!(map.known && map.requires.is_empty());
        assert!(g.get("modular-arithmetic").unwrap().known);
        assert_eq!(
            g.background.as_deref(),
            Some("Linear algebra and elementary number theory.")
        );
    }

    #[test]
    fn background_rejects_the_target_and_concepts_already_in_the_graph() {
        let mut g = graph(
            "target",
            vec![
                concept("target", &["modular-arithmetic"], false),
                concept("modular-arithmetic", &[], false),
            ],
        );
        let answer = BackgroundAnswer {
            summary: " ".into(),
            known: vec![known("Clock arithmetic", &["modular arithmetic"])],
            covered: vec!["target".into(), "nope".into()],
        };
        let error = apply_background(&mut g, answer).unwrap_err().to_string();
        assert!(error.contains("summary is empty"), "{error}");
        assert!(
            error.contains("existing concept `modular-arithmetic`"),
            "{error}"
        );
        assert!(
            error.contains("`target` is what the wiki explains"),
            "{error}"
        );
        assert!(error.contains("`nope`"), "{error}");
        assert!(g.background.is_none());
    }
}

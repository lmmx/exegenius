//! Step 1: decompose the sources into a concept graph, and later resolve the candidate
//! concepts that audits find.
//!
//! The model proposes concepts; code cleans them (slug ids, drop self-edges and quotes not
//! found verbatim in their source), checks the graph, and sends one repair call carrying
//! the exact errors if the check fails.

use std::collections::{BTreeMap, HashSet};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Log;
use crate::error::{Error, Result};
use crate::graph::{Concept, Graph, TermIndex};
use crate::model::{Model, ask};
use crate::project::{Project, Source, find_source};
use crate::text::{contains_quote, key, slug};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExtractResponse {
    /// Id of the concept that is the target result.
    pub target: String,
    pub concepts: Vec<Concept>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExtendResponse {
    /// New concepts only; existing concepts must not be repeated.
    pub concepts: Vec<Concept>,
    /// For every unresolved term, the id of the concept (existing or new) that covers it.
    pub resolutions: Vec<Resolution>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Resolution {
    /// The unresolved term, exactly as given.
    pub term: String,
    /// Id of the concept that covers it.
    pub concept: String,
}

const EXTRACT_SYSTEM: &str = "\
You decompose source documents into the concepts a specific reader must understand to reach \
a target result. Your output is a prerequisite graph that will be turned into a wiki with one \
explanatory page per concept the reader does not already know.

Rules:
- Make one concept per idea that deserves its own explanatory page. Fold minor terms into the \
gloss of the concept that uses them instead of giving them their own concept.
- Cover everything the target depends on in the sources, including named techniques, \
constructions, lemmas and notation-heavy objects. Leave out material the target does not depend on.
- `requires` lists the concepts a reader must understand before this one. Use only ids that \
appear in your output. The graph must have no cycles.
- Mark `known: true` for concepts the reader profile already covers. Include known concepts \
that other concepts require, so prerequisites bottom out in what the reader knows, and give \
known concepts `requires: []`.
- `evidence` quotes must be copied from the sources character for character (short phrases or \
single sentences), with the id of the source they come from. Every concept the reader does not \
know needs at least one quote. Known concepts may have none.
- `target` is the id of the concept for the target result.";

const EXTEND_SYSTEM: &str = "\
You extend a prerequisite graph of concepts built from source documents. An audit of the wiki \
found terms that a specific reader cannot follow and that match no existing concept. For each \
term, either point it at an existing concept that covers it, or add a new concept.

Rules:
- Add a new concept only when no existing concept covers the term. Give each new concept a \
fresh id that is not already used.
- New concepts may require existing concepts or each other; the graph must stay acyclic.
- Mark `known: true` if the reader profile already covers the concept, with `requires: []`.
- `evidence` quotes must be copied from the sources character for character, with their source \
id. A concept that does not appear in the sources may have no quotes.
- Return a resolution for every term.";

fn describe_reader(project: &Project) -> String {
    format!(
        "# Target\n\n{}\n\n# Reader profile\n\n{}\n",
        project.config.target.trim(),
        project.config.reader.knows.trim()
    )
}

/// Slug ids and edges, drop self-edges, duplicate aliases, and quotes not found verbatim.
fn clean(concepts: &mut [Concept], sources: &[Source], warnings: &mut Vec<String>) {
    for c in concepts.iter_mut() {
        c.id = slug(&c.id);
        let mut seen = HashSet::new();
        let id = c.id.clone();
        c.requires = c
            .requires
            .iter()
            .map(|r| slug(r))
            .filter(|r| *r != id && seen.insert(r.clone()))
            .collect();
        let name_key = key(&c.name);
        let mut seen = HashSet::new();
        c.aliases
            .retain(|a| !a.trim().is_empty() && key(a) != name_key && seen.insert(key(a)));
        c.evidence.retain(|e| {
            let found = find_source(sources, &e.source)
                .is_some_and(|s| contains_quote(&s.normalised, &e.quote));
            if !found {
                warnings.push(format!(
                    "`{id}`: dropped a quote not found verbatim in source `{}`: {:?}",
                    e.source, e.quote
                ));
            }
            found
        });
        if !c.known && c.evidence.is_empty() {
            warnings.push(format!("`{id}`: no supporting quote from the sources"));
        }
    }
}

fn error_list(errors: &[String]) -> String {
    errors
        .iter()
        .map(|e| format!("- {e}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn repair_task(task: &str, previous: &impl Serialize, errors: &[String]) -> String {
    let previous = serde_json::to_string_pretty(previous).unwrap_or_default();
    format!(
        "{task}\n\n# Your previous output\n\n{previous}\n\n# Validation errors\n\n{}\n\n\
         Return the complete corrected output.",
        error_list(errors)
    )
}

/// Decompose the sources into a new graph. Returns the graph and any warnings.
pub fn extract(project: &Project, model: &dyn Model, log: Log) -> Result<(Graph, Vec<String>)> {
    let context = project.sources_context();
    let task = format!(
        "{}\nDecompose the sources into concepts for this reader and target.",
        describe_reader(project)
    );
    log("extract: decomposing sources");
    let mut response: ExtractResponse = ask(model, EXTRACT_SYSTEM, Some(&context), &task)?;
    for attempt in 0..2 {
        let mut warnings = Vec::new();
        clean(&mut response.concepts, &project.sources, &mut warnings);
        let graph = Graph {
            target: slug(&response.target),
            concepts: response.concepts.clone(),
            ..Graph::default()
        };
        let errors = graph.check();
        if errors.is_empty() {
            return Ok((graph, warnings));
        }
        if attempt == 1 {
            return Err(Error::Invalid(format!(
                "extraction still fails validation after a repair:\n{}",
                error_list(&errors)
            )));
        }
        log(&format!(
            "extract: {} validation errors, asking for a repair",
            errors.len()
        ));
        response = ask(
            model,
            EXTRACT_SYSTEM,
            Some(&context),
            &repair_task(&task, &response, &errors),
        )?;
    }
    unreachable!("the loop returns on its second pass")
}

/// Resolve the graph's candidate terms into existing or new concepts, add the edges from
/// the pages that used them, and queue those pages for rewriting. Returns warnings.
pub fn extend(
    project: &Project,
    graph: &mut Graph,
    model: &dyn Model,
    log: Log,
) -> Result<Vec<String>> {
    if graph.candidates.is_empty() {
        return Ok(Vec::new());
    }
    // One entry per distinct term, with up to three example uses.
    let mut terms: BTreeMap<String, (String, Vec<String>)> = BTreeMap::new();
    for c in &graph.candidates {
        let entry = terms
            .entry(key(&c.term))
            .or_insert_with(|| (c.term.clone(), Vec::new()));
        if entry.1.len() < 3 {
            entry.1.push(format!("on page `{}`: {:?}", c.page, c.quote));
        }
    }
    let existing = graph
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
    let unresolved = terms
        .values()
        .map(|(term, uses)| format!("- {term:?}\n  - {}", uses.join("\n  - ")))
        .collect::<Vec<_>>()
        .join("\n");
    let task = format!(
        "{}\n# Existing concepts\n\n{existing}\n\n# Unresolved terms\n\n{unresolved}\n",
        describe_reader(project)
    );
    let context = project.sources_context();
    log(&format!(
        "extract: resolving {} candidate terms",
        terms.len()
    ));
    let mut response: ExtendResponse = ask(model, EXTEND_SYSTEM, Some(&context), &task)?;
    for attempt in 0..2 {
        let mut warnings = Vec::new();
        clean(&mut response.concepts, &project.sources, &mut warnings);
        let mut errors = Vec::new();
        for c in &response.concepts {
            if graph.get(&c.id).is_some() {
                errors.push(format!("new concept id `{}` is already used", c.id));
            }
        }
        let mut merged = graph.clone();
        merged.concepts.extend(response.concepts.iter().cloned());
        errors.extend(merged.check());
        for r in &response.resolutions {
            if merged.get(&slug(&r.concept)).is_none() {
                errors.push(format!(
                    "resolution of {:?} names `{}`, which is not a concept id",
                    r.term, r.concept
                ));
            }
        }
        if errors.is_empty() {
            apply_resolutions(&mut merged, &response.resolutions, &mut warnings);
            *graph = merged;
            return Ok(warnings);
        }
        if attempt == 1 {
            return Err(Error::Invalid(format!(
                "candidate resolution still fails validation after a repair:\n{}",
                error_list(&errors)
            )));
        }
        log(&format!(
            "extract: {} validation errors, asking for a repair",
            errors.len()
        ));
        response = ask(
            model,
            EXTEND_SYSTEM,
            Some(&context),
            &repair_task(&task, &response, &errors),
        )?;
    }
    unreachable!("the loop returns on its second pass")
}

/// Turn each resolved candidate into an edge from its page (or a rewrite note when the
/// edge would create a cycle), record the term as an alias, and clear the candidates.
fn apply_resolutions(graph: &mut Graph, resolutions: &[Resolution], warnings: &mut Vec<String>) {
    let resolved: BTreeMap<String, String> = resolutions
        .iter()
        .map(|r| (key(&r.term), slug(&r.concept)))
        .collect();
    for candidate in std::mem::take(&mut graph.candidates) {
        let Some(concept_id) = resolved.get(&key(&candidate.term)) else {
            warnings.push(format!(
                "no resolution for {:?}; asking page `{}` to explain it inline",
                candidate.term, candidate.page
            ));
            graph.queue_rewrite(
                &candidate.page,
                Some(format!("Explain {:?} where it is used.", candidate.term)),
            );
            continue;
        };
        let index = TermIndex::new(graph.get(concept_id));
        if index.lookup(&candidate.term).is_none()
            && let Some(c) = graph.get_mut(concept_id)
        {
            c.aliases.push(candidate.term.clone());
        }
        let Some(concept) = graph.get(concept_id).cloned() else {
            continue;
        };
        if concept.known || concept.id == candidate.page {
            continue;
        }
        if graph.add_requires(&candidate.page, &concept.id) {
            graph.queue_rewrite(
                &candidate.page,
                Some(format!(
                    "{:?} is the concept [{}]({}.md), now a prerequisite: link it where the \
                     term is used.",
                    candidate.term, concept.name, concept.id
                )),
            );
        } else if !graph
            .get(&candidate.page)
            .is_some_and(|p| p.requires.contains(&concept.id))
        {
            graph.queue_rewrite(
                &candidate.page,
                Some(format!(
                    "Explain {:?} inline: its concept depends on this page, so it cannot be \
                     a prerequisite.",
                    candidate.term
                )),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Evidence;
    use crate::graph::tests::concept;

    #[test]
    fn clean_slugs_ids_and_drops_unverified_quotes() {
        let sources = vec![Source::new(
            "lowrank",
            None,
            None,
            "converts these through a chirp\nidentity to convolution.".into(),
        )];
        let mut c = concept("Chirp Identity", &["DFT", "Chirp Identity", "dft"], false);
        c.evidence = vec![
            Evidence {
                source: "lowrank".into(),
                quote: "through a chirp identity".into(),
            },
            Evidence {
                source: "lowrank".into(),
                quote: "invented sentence".into(),
            },
            Evidence {
                source: "nope".into(),
                quote: "through".into(),
            },
        ];
        let mut concepts = vec![c];
        let mut warnings = Vec::new();
        clean(&mut concepts, &sources, &mut warnings);
        assert_eq!(concepts[0].id, "chirp-identity");
        assert_eq!(concepts[0].requires, vec!["dft"]);
        assert_eq!(concepts[0].evidence.len(), 1);
        assert_eq!(warnings.len(), 2);
    }
}

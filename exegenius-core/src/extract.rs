//! Extract tasks: decompose the sources into a concept graph, and later resolve the
//! candidate concepts that audits find.
//!
//! The agent proposes concepts; code slugs ids and edges, rejects quotes not found verbatim
//! in their source, and checks the graph (duplicates, unknown ids, missing target, cycles).
//! A rejected answer comes back with the exact errors for the agent to fix.

use std::collections::{BTreeMap, HashSet};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::graph::{Concept, Graph, TermIndex};
use crate::project::{Project, Source, find_source};
use crate::text::{contains_quote, key, slug};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExtractAnswer {
    /// Id of the concept that is the target result.
    pub target: String,
    pub concepts: Vec<Concept>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExtendAnswer {
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

pub const EXTRACT_INSTRUCTIONS: &str = "\
Decompose the source documents into the concepts a specific reader must understand to reach \
a target result. Your answer is a prerequisite graph that becomes a wiki with one explanatory \
page per concept the reader does not already know.

- Make one concept per idea that deserves its own explanatory page. Fold minor terms into the \
gloss of the concept that uses them instead of giving them their own concept.
- Cover everything the target depends on in the sources, including named techniques, \
constructions, lemmas and notation-heavy objects. Leave out material the target does not \
depend on.
- `requires` lists the concepts a reader must understand before this one. Use only ids that \
appear in your answer. The graph must have no cycles.
- Mark `known: true` for concepts the reader profile already covers. Include known concepts \
that other concepts require, so prerequisites bottom out in what the reader knows, and give \
known concepts `requires: []`.
- `evidence` quotes must be copied from the sources character for character (a phrase or one \
sentence), with the id of the source they come from. Submit rejects any quote it cannot find \
in that source (it ignores differences in whitespace and in typographic quotes and dashes). \
Every concept the reader does not know needs at least one quote; known concepts may have none.
- A name with mathematics in it (\"Polynomial ring modulo y^r + 1\") needs a `title`: the \
name with the mathematics in LaTeX (\"Polynomial ring modulo $y^r + 1$\"), and in `symbols` a \
binding for every symbol in that mathematics: a display name of one to four words (\"record \
length\"), its meaning in one sentence, and whether it denotes something directly or counts \
instances of something (for a count, say what it counts: \"the number of complex \
coefficients in a record\", not \"a power of two\"). A reader who knows the concept is shown \
these bindings and no other explanation of its symbols. Names without mathematics need \
neither.
- `target` is the id of the concept for the target result.";

pub const EXTEND_INSTRUCTIONS: &str = "\
Extend a prerequisite graph of concepts built from source documents. Audits of the wiki \
reported terms that they judged a specific reader cannot follow and that match no existing \
concept. For each term, either point it at an existing concept that covers it, or add a new \
concept.

- Auditors can misjudge what the reader profile covers. A term the profile covers, whether or \
not it names it, is known to the reader: point it at a known concept, or add one with `known: \
true`. Known concepts get no page.
- Add a new concept only when no existing concept covers the term. Give each new concept an \
id that is not already used.
- New concepts may require existing concepts or each other; the graph must stay acyclic.
- Mark `known: true` if the reader profile already covers the concept, with `requires: []`.
- `evidence` quotes must be copied from the sources character for character, with their \
source id. A concept that does not appear in the sources may have no quotes.
- A name with mathematics in it needs a `title` with the mathematics in LaTeX and a binding \
in `symbols` for every symbol in it, as in the extract task.
- Give a resolution for every term.";

fn describe(project: &Project) -> String {
    format!(
        "## Target\n\n{}\n\n## Reader profile\n\n{}\n\n## Sources (read them from disk)\n\n{}\n",
        project.config.target.trim(),
        project.config.reader.knows.trim(),
        project.sources_listing()
    )
}

pub fn extract_task(project: &Project) -> String {
    describe(project)
}

/// Slug ids and edges, drop self-edges and duplicate aliases; quotes not found verbatim
/// are errors, concepts the reader does not know without quotes are warnings.
fn clean(
    concepts: &mut [Concept],
    sources: &[Source],
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    for c in concepts.iter_mut() {
        c.id = slug(&c.id);
        let id = c.id.clone();
        let mut seen = HashSet::new();
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
        for e in &c.evidence {
            match find_source(sources, &e.source) {
                None => errors.push(format!(
                    "`{id}`: evidence names source `{}`, which is not a source id",
                    e.source
                )),
                Some(s) if !contains_quote(&s.normalised, &e.quote) => errors.push(format!(
                    "`{id}`: quote not found verbatim in source `{}`: \"{}\"",
                    e.source, e.quote
                )),
                Some(_) => {}
            }
        }
        errors.extend(crate::symbols::check_name(c));
        if !c.known && c.evidence.is_empty() {
            warnings.push(format!("`{id}`: no supporting quote from the sources"));
        }
    }
}

fn rejected(errors: Vec<String>) -> Error {
    let list = errors
        .iter()
        .map(|e| format!("- {e}"))
        .collect::<Vec<_>>()
        .join("\n");
    Error::Invalid(format!("answer rejected:\n{list}"))
}

/// Check an extract answer and turn it into a graph. Returns the graph and warnings.
pub fn apply_extract(project: &Project, mut answer: ExtractAnswer) -> Result<(Graph, Vec<String>)> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    clean(
        &mut answer.concepts,
        &project.sources,
        &mut errors,
        &mut warnings,
    );
    let graph = Graph {
        target: slug(&answer.target),
        concepts: answer.concepts,
        ..Graph::default()
    };
    errors.extend(graph.check());
    if errors.is_empty() {
        Ok((graph, warnings))
    } else {
        Err(rejected(errors))
    }
}

pub fn extend_task(project: &Project, graph: &Graph) -> String {
    // One entry per distinct term, with up to three example uses.
    let mut terms: BTreeMap<String, (String, Vec<String>)> = BTreeMap::new();
    for c in &graph.candidates {
        let entry = terms
            .entry(key(&c.term))
            .or_insert_with(|| (c.term.clone(), Vec::new()));
        if entry.1.len() < 3 {
            entry
                .1
                .push(format!("on page `{}`: \"{}\"", c.page, c.quote));
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
        .map(|(term, uses)| format!("- \"{term}\"\n  - {}", uses.join("\n  - ")))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "{}\n## Existing concepts\n\n{existing}\n\n## Unresolved terms\n\n{unresolved}\n",
        describe(project)
    )
}

/// Check an extend answer against the graph, merge it, and turn each candidate into an edge
/// from its page or a rewrite note. Returns warnings.
pub fn apply_extend(
    project: &Project,
    graph: &mut Graph,
    mut answer: ExtendAnswer,
) -> Result<Vec<String>> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    clean(
        &mut answer.concepts,
        &project.sources,
        &mut errors,
        &mut warnings,
    );
    for c in &answer.concepts {
        if graph.get(&c.id).is_some() {
            errors.push(format!("new concept id `{}` is already used", c.id));
        }
    }
    let mut merged = graph.clone();
    merged.concepts.extend(answer.concepts);
    errors.extend(merged.check());
    for r in &answer.resolutions {
        if merged.get(&slug(&r.concept)).is_none() {
            errors.push(format!(
                "resolution of \"{}\" names `{}`, which is not a concept id",
                r.term, r.concept
            ));
        }
    }
    let asked: HashSet<String> = graph.candidates.iter().map(|c| key(&c.term)).collect();
    let answered: HashSet<String> = answer.resolutions.iter().map(|r| key(&r.term)).collect();
    let mut missing: Vec<&String> = asked.difference(&answered).collect();
    missing.sort();
    for term in missing {
        errors.push(format!("no resolution for the term \"{term}\""));
    }
    if !errors.is_empty() {
        return Err(rejected(errors));
    }
    apply_resolutions(&mut merged, &answer.resolutions);
    *graph = merged;
    Ok(warnings)
}

/// Turn each resolved candidate into an edge from its page (or a rewrite note when the
/// edge would create a cycle), record the term as an alias, and clear the candidates.
fn apply_resolutions(graph: &mut Graph, resolutions: &[Resolution]) {
    let resolved: BTreeMap<String, String> = resolutions
        .iter()
        .map(|r| (key(&r.term), slug(&r.concept)))
        .collect();
    for candidate in std::mem::take(&mut graph.candidates) {
        let Some(concept_id) = resolved.get(&key(&candidate.term)) else {
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
                    "\"{}\" is the concept [{}]({}.md), now a prerequisite: link it where the \
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
                    "Explain \"{}\" inline: its concept depends on this page, so it cannot be \
                     a prerequisite.",
                    candidate.term
                )),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::graph::Evidence;
    use crate::graph::tests::concept;

    #[test]
    fn clean_slugs_ids_and_rejects_unverified_quotes() {
        let sources = vec![Source::new(
            "lowrank",
            None,
            None,
            Path::new("s.md"),
            "converts these through a chirp\nidentity to convolution.",
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
        let (mut errors, mut warnings) = (Vec::new(), Vec::new());
        clean(&mut concepts, &sources, &mut errors, &mut warnings);
        assert_eq!(concepts[0].id, "chirp-identity");
        assert_eq!(concepts[0].requires, vec!["dft"]);
        assert_eq!(errors.len(), 2);
        assert!(warnings.is_empty());
    }
}

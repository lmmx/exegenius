//! Audit tasks: list what the reader cannot follow on one page.
//!
//! The reader's knowledge is a set of concepts that code maintains: the concepts the profile
//! covers plus every page before this one in reading order. The auditor is told the reader
//! knows exactly that set, and that everything else it knows is unknown to the reader. Code
//! then:
//!
//! 1. rejects the answer if a quote is not on the page (the auditor fixes or drops it),
//! 2. drops terms the reader already knows,
//! 3. turns a term naming a concept in the graph into a prerequisite edge (or a rewrite
//!    note when the edge exists already or would create a cycle),
//! 4. turns any other term into a candidate concept for an extend task, and symbols and
//!    steps into rewrite notes.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result, read_json, write_json};
use crate::graph::{Candidate, Graph, TermIndex};
use crate::plan::Plan;
use crate::project::Project;
use crate::text::{contains_quote, normalise};
use crate::write::{link, names};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum GapKind {
    /// A named concept the reader does not know, which the page neither explains nor links.
    Term,
    /// Notation used without being defined on the page.
    Symbol,
    /// An inference, calculation or claim the reader cannot check from what the page gives.
    Step,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Gap {
    /// The term or symbol, or a short label for the step.
    pub term: String,
    /// The passage on the page where the gap occurs, copied exactly (a phrase or one sentence).
    pub quote: String,
    pub kind: GapKind,
    /// Why this reader cannot follow it.
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AuditAnswer {
    /// Every gap on the page; empty if the reader can follow all of it.
    pub gaps: Vec<Gap>,
}

/// What code did with one reported gap.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Outcome {
    pub term: String,
    pub kind: GapKind,
    pub quote: String,
    pub note: String,
    pub action: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageAudit {
    pub page: String,
    /// Gaps that led to a graph edit, a candidate concept or a rewrite note.
    pub gaps: Vec<Outcome>,
    /// Gaps code discarded, with the reason as the action.
    pub dropped: Vec<Outcome>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Round {
    pub round: u32,
    pub pages: Vec<PageAudit>,
}

impl Round {
    pub fn gap_count(&self) -> usize {
        self.pages.iter().map(|p| p.gaps.len()).sum()
    }
}

pub const AUDIT_INSTRUCTIONS: &str = "\
Audit one page of an explanatory wiki on behalf of a specific reader. Use only this task \
file: do not read the sources, other pages or any other file.

The reader knows exactly the concepts listed as known, together with what the reader profile \
states. Everything else that you know, the reader does not know: treat it as unknown to them, \
however elementary it seems to you. You are simulating that reader's lack of knowledge, so use \
your own knowledge only to recognise what the reader is missing.

A term, symbol or step is followable if the reader knows it, the page explains it, or the page \
links it to another wiki page. Report every term, symbol or step on the page that the reader \
cannot follow:
- term: a named concept the reader does not know, which the page neither explains nor links.
- symbol: notation used without being defined on the page.
- step: an inference, calculation or claim the reader cannot check from what the page gives.

Copy each quote from the page exactly, character for character, and keep it short: submit \
rejects quotes it cannot find on the page. Do not report anything the reader can follow. \
Answer with an empty list if the reader can follow the whole page.";

pub fn audit_task(project: &Project, graph: &Graph, plan: &Plan, i: usize, body: &str) -> String {
    let c = graph
        .get(&plan.order[i])
        .expect("planned pages are concepts");
    let pages: Vec<String> = plan
        .order
        .iter()
        .filter_map(|id| graph.get(id))
        .map(|p| format!("- {}", link(p)))
        .collect();
    format!(
        "## Reader profile\n\n{}\n\n## Known concepts\n\n{}\n\n## Pages in the wiki\n\n{}\n\n\
         ## Page to audit\n\n````markdown\n# {}\n\n{}\n````\n",
        project.config.reader.knows.trim(),
        names(&plan.known_at(graph, i)),
        pages.join("\n"),
        c.name,
        crate::cite::strip(body).trim()
    )
}

/// Check an audit answer for page `plan.order[i]` and apply its gaps to the graph.
pub fn apply_audit(
    graph: &mut Graph,
    plan: &Plan,
    i: usize,
    body: &str,
    answer: AuditAnswer,
) -> Result<PageAudit> {
    let page = &plan.order[i];
    let normalised = normalise(&crate::cite::strip(body));
    let missing: Vec<String> = answer
        .gaps
        .iter()
        .filter(|g| !contains_quote(&normalised, &g.quote))
        .map(|g| format!("- quote not found on the page: \"{}\"", g.quote))
        .collect();
    if !missing.is_empty() {
        return Err(Error::Invalid(format!(
            "answer rejected:\n{}\nCopy quotes exactly from the page, or remove those gaps.",
            missing.join("\n")
        )));
    }
    let known = TermIndex::new(plan.known_at(graph, i));
    Ok(apply_gaps(graph, page, &known, answer.gaps))
}

fn apply_gaps(graph: &mut Graph, page: &str, known: &TermIndex, gaps: Vec<Gap>) -> PageAudit {
    let mut audit = PageAudit {
        page: page.to_string(),
        gaps: Vec::new(),
        dropped: Vec::new(),
    };
    for gap in gaps {
        let outcome = |action: String| Outcome {
            term: gap.term.clone(),
            kind: gap.kind,
            quote: gap.quote.clone(),
            note: gap.note.clone(),
            action,
        };
        if gap.kind != GapKind::Step && known.lookup(&gap.term).is_some() {
            audit
                .dropped
                .push(outcome("dropped: the reader already knows this".into()));
            continue;
        }
        let action = match gap.kind {
            GapKind::Step => {
                graph.queue_rewrite(
                    page,
                    Some(format!(
                        "The reader cannot follow this step: \"{}\" ({})",
                        gap.quote, gap.note
                    )),
                );
                "rewrite: explain the step".to_string()
            }
            GapKind::Symbol => {
                graph.queue_rewrite(
                    page,
                    Some(format!(
                        "Define the symbol {} where it is first used: \"{}\"",
                        gap.term, gap.quote
                    )),
                );
                "rewrite: define the symbol".to_string()
            }
            GapKind::Term => term_action(graph, page, &gap),
        };
        audit.gaps.push(outcome(action));
    }
    audit
}

fn term_action(graph: &mut Graph, page: &str, gap: &Gap) -> String {
    let index = TermIndex::new(&graph.concepts);
    let Some(target) = index.lookup(&gap.term).map(str::to_string) else {
        graph.candidates.push(Candidate {
            term: gap.term.clone(),
            page: page.to_string(),
            quote: gap.quote.clone(),
        });
        return "candidate concept".to_string();
    };
    let concept = graph.get(&target).expect("indexed concepts exist").clone();
    if concept.id == page {
        graph.queue_rewrite(
            page,
            Some(format!(
                "The reader cannot follow the page's own subject as used here: \"{}\" ({})",
                gap.quote, gap.note
            )),
        );
        return "rewrite: explain the page's own concept".to_string();
    }
    let already = graph
        .get(page)
        .is_some_and(|p| p.requires.contains(&concept.id));
    if already {
        graph.queue_rewrite(
            page,
            Some(format!(
                "Link {} where \"{}\" is used: \"{}\"",
                link(&concept),
                gap.term,
                gap.quote
            )),
        );
        return format!("rewrite: link prerequisite `{}`", concept.id);
    }
    if graph.add_requires(page, &concept.id) {
        graph.queue_rewrite(
            page,
            Some(format!(
                "{} is now a prerequisite: link it where \"{}\" is used: \"{}\"",
                link(&concept),
                gap.term,
                gap.quote
            )),
        );
        return format!("added prerequisite `{}`", concept.id);
    }
    graph.queue_rewrite(
        page,
        Some(format!(
            "Explain \"{}\" inline where it is used: \"{}\" (its concept depends on this page, \
             so it cannot be a prerequisite)",
            gap.term, gap.quote
        )),
    );
    "rewrite: explain inline (an edge would create a cycle)".to_string()
}

/// Add a page's audit to its round in `audit.json`.
pub fn record(project: &Project, round: u32, page: PageAudit) -> Result<()> {
    let path = project.audit_path();
    let mut rounds: Vec<Round> = if path.exists() {
        read_json(&path)?
    } else {
        Vec::new()
    };
    match rounds.iter_mut().find(|r| r.round == round) {
        Some(r) => {
            r.pages.retain(|p| p.page != page.page);
            r.pages.push(page);
        }
        None => rounds.push(Round {
            round,
            pages: vec![page],
        }),
    }
    write_json(&path, &rounds)
}

pub fn load_rounds(project: &Project) -> Result<Vec<Round>> {
    let path = project.audit_path();
    if path.exists() {
        read_json(&path)
    } else {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::tests::{concept, graph};

    fn gap(term: &str, quote: &str, kind: GapKind) -> Gap {
        Gap {
            term: term.into(),
            quote: quote.into(),
            kind,
            note: "n".into(),
        }
    }

    #[test]
    fn gaps_become_edges_candidates_notes_or_drops() {
        let mut g = graph(
            "page",
            vec![
                concept("page", &[], false),
                concept("dft", &[], false),
                concept("later", &["page"], false),
                concept("big-o", &[], true),
            ],
        );
        let plan = Plan {
            order: vec!["page".into()],
        };
        let body = "Uses the DFT, big O, a later idea, dyadic numbers and $\\kappa$.";
        let answer = |gaps| AuditAnswer { gaps };
        let bad = apply_audit(
            &mut g.clone(),
            &plan,
            0,
            body,
            answer(vec![gap("x", "not on the page", GapKind::Step)]),
        );
        assert!(bad.is_err());
        let audit = apply_audit(
            &mut g,
            &plan,
            0,
            body,
            answer(vec![
                gap("DFT", "Uses the DFT", GapKind::Term),
                gap("big O", "big O", GapKind::Term),
                gap("later", "a later idea", GapKind::Term),
                gap("dyadic", "dyadic numbers", GapKind::Term),
                gap("κ", "$\\kappa$", GapKind::Symbol),
            ]),
        )
        .unwrap();
        let actions: Vec<&str> = audit.gaps.iter().map(|o| o.action.as_str()).collect();
        assert_eq!(
            actions,
            vec![
                "added prerequisite `dft`",
                "rewrite: explain inline (an edge would create a cycle)",
                "candidate concept",
                "rewrite: define the symbol",
            ]
        );
        assert_eq!(audit.dropped.len(), 1);
        assert_eq!(g.get("page").unwrap().requires, vec!["dft"]);
        assert_eq!(g.candidates[0].term, "dyadic");
        assert_eq!(g.rewrite["page"].len(), 3);
        assert!(g.check().is_empty());
    }
}

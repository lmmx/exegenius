//! Step 4: audit each page on behalf of the reader.
//!
//! The reader's knowledge is a set of concepts that code maintains: the concepts the
//! profile covers plus every page before this one in reading order. The model is told the
//! reader knows exactly that set, and that everything else it knows is unknown to the
//! reader; it lists what the reader cannot follow. Code then:
//!
//! 1. drops gaps whose quote is not on the page,
//! 2. drops terms the reader already knows,
//! 3. turns a term naming a concept in the graph into a prerequisite edge (or a rewrite
//!    note when the edge exists already or would create a cycle),
//! 4. turns any other term into a candidate concept for `extract`, and symbols and steps
//!    into rewrite notes.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Log;
use crate::error::{Result, read_json, write_json};
use crate::graph::{Candidate, Graph, TermIndex};
use crate::model::{Model, ask};
use crate::plan::Plan;
use crate::project::Project;
use crate::text::{contains_quote, normalise};
use crate::write::link;

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
pub struct AuditResponse {
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
    pub round: usize,
    pub pages: Vec<PageAudit>,
}

impl Round {
    pub fn gap_count(&self) -> usize {
        self.pages.iter().map(|p| p.gaps.len()).sum()
    }
}

const AUDIT_SYSTEM: &str = "\
You audit one page of an explanatory wiki on behalf of a specific reader.

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

Copy each quote from the page exactly, character for character, and keep it short. Do not \
report anything the reader can follow. Return an empty list if the reader can follow the \
whole page.";

fn audit_task(project: &Project, graph: &Graph, plan: &Plan, i: usize, body: &str) -> String {
    let c = graph
        .get(&plan.order[i])
        .expect("planned pages are concepts");
    let known: Vec<String> = plan
        .known_at(graph, i)
        .iter()
        .map(|k| {
            if k.aliases.is_empty() {
                format!("- {}", k.name)
            } else {
                format!("- {} (also: {})", k.name, k.aliases.join(", "))
            }
        })
        .collect();
    let pages: Vec<String> = plan
        .order
        .iter()
        .filter_map(|id| graph.get(id))
        .map(|p| format!("- {}", link(p)))
        .collect();
    format!(
        "# Reader profile\n\n{}\n\n# Known concepts\n\n{}\n\n# Pages in the wiki\n\n{}\n\n\
         # Page to audit\n\n# {}\n\n{}\n",
        project.config.reader.knows.trim(),
        known.join("\n"),
        pages.join("\n"),
        c.name,
        body.trim()
    )
}

/// Audit every planned page that has a body, apply the outcomes to the graph, and append
/// the round to `audit.json`.
pub fn audit_pages(
    project: &Project,
    graph: &mut Graph,
    plan: &Plan,
    model: &dyn Model,
    log: Log,
) -> Result<Round> {
    let path = project.audit_path();
    let mut rounds: Vec<Round> = if path.exists() {
        read_json(&path)?
    } else {
        Vec::new()
    };
    let mut round = Round {
        round: rounds.len() + 1,
        pages: Vec::new(),
    };
    for (i, id) in plan.order.iter().enumerate() {
        let Some(body) = project.page_body(id)? else {
            continue;
        };
        let task = audit_task(project, graph, plan, i, &body);
        let response: AuditResponse = ask(model, AUDIT_SYSTEM, None, &task)?;
        let known = TermIndex::new(plan.known_at(graph, i));
        let page = apply_gaps(graph, id, &body, &known, response.gaps);
        log(&format!(
            "audit [{}/{}] {id}: {} gaps ({} dropped)",
            i + 1,
            plan.order.len(),
            page.gaps.len(),
            page.dropped.len()
        ));
        round.pages.push(page);
    }
    project.save_graph(graph)?;
    rounds.push(round.clone());
    write_json(&path, &rounds)?;
    Ok(round)
}

fn apply_gaps(
    graph: &mut Graph,
    page: &str,
    body: &str,
    known: &TermIndex,
    gaps: Vec<Gap>,
) -> PageAudit {
    let normalised_body = normalise(body);
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
        if !contains_quote(&normalised_body, &gap.quote) {
            audit
                .dropped
                .push(outcome("dropped: quote not found on the page".into()));
            continue;
        }
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
                        "The reader cannot follow this step: {:?} ({})",
                        gap.quote, gap.note
                    )),
                );
                "rewrite: explain the step".to_string()
            }
            GapKind::Symbol => {
                graph.queue_rewrite(
                    page,
                    Some(format!(
                        "Define the symbol {} where it is first used: {:?}",
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
                "The reader cannot follow the page's own subject as used here: {:?} ({})",
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
                "Link {} where {:?} is used: {:?}",
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
                "{} is now a prerequisite: link it where {:?} is used: {:?}",
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
            "Explain {:?} inline where it is used: {:?} (its concept depends on this page, \
             so it cannot be a prerequisite)",
            gap.term, gap.quote
        )),
    );
    "rewrite: explain inline (an edge would create a cycle)".to_string()
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
        let known = TermIndex::new(g.concepts.iter().filter(|c| c.known));
        let body = "Uses the DFT, big O, a later idea, dyadic numbers and $\\kappa$.";
        let audit = apply_gaps(
            &mut g,
            "page",
            body,
            &known,
            vec![
                gap("DFT", "Uses the DFT", GapKind::Term),
                gap("big O", "big O", GapKind::Term),
                gap("later", "a later idea", GapKind::Term),
                gap("dyadic", "dyadic numbers", GapKind::Term),
                gap("κ", "$\\kappa$", GapKind::Symbol),
                gap("x", "not on the page", GapKind::Step),
            ],
        );
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
        assert_eq!(audit.dropped.len(), 2);
        assert_eq!(g.get("page").unwrap().requires, vec!["dft"]);
        assert_eq!(g.candidates[0].term, "dyadic");
        assert_eq!(g.rewrite["page"].len(), 3);
        assert!(g.check().is_empty());
    }
}

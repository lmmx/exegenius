//! Aha moments: the sentences that close a chain of reasoning.
//!
//! On Hamami & Morris's account an aha moment is a reconnection: the point where a step's
//! link to the main objective becomes visible. Self-report is no evidence of one, so the
//! candidates come from what readers and the pages do, each method kept apart, since they
//! find different things (a key step, a payoff, a reconnection):
//!
//! - **attribution**: after grading, an attribution task names the reader citation that
//!   supports each covered key point, so the report can count which sentences carry key
//!   points across runs;
//! - **agreement**: each reader names the key idea of the page it relied on most, with a
//!   quote; readers who quote the same sentence agree, and a page on which readers disagree
//!   has no clear key idea;
//! - **structure**: sentences that link pages under different main steps of the argument
//!   join distant parts of it.
//!
//! Attribution is a separate task, not part of grading: grading is blind, and showing the
//! readers' citations would tell their answers from the no-wiki baseline, which has none.

use std::collections::BTreeMap;

use pulldown_cmark::{Event, Options, Parser, Tag};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::argument::{Argument, Node};
use crate::graph::Concept;
use crate::simulate::{Hidden, Outcome, Simulation, rejected};
use crate::text::{key, normalise};
use crate::write::link_target;

pub const ATTRIBUTE_INSTRUCTIONS: &str = "\
Say which quotation supports each key point a reader's answer makes. Each answer below was \
graded already: the key points it covers are listed under it, with the quotations the reader \
gave from the pages they read.

- For each answer, and each key point listed as covered, give the number of the quotation \
that supports it: the one the point rests on, not merely one on the same subject.
- If no quotation supports a covered key point, leave that key point out.
- Do not regrade: judge only the support for the key points listed.";

/// The quotation that supports one covered key point.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Support {
    /// Number of the key point.
    pub point: usize,
    /// Number of the quotation under the answer.
    pub quotation: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Attribution {
    /// Number of the answer, as listed.
    pub answer: usize,
    pub support: Vec<Support>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AttributeAnswer {
    /// One entry for every answer listed.
    pub answers: Vec<Attribution>,
}

/// The runs of the current round on `goal` whose covered key points need attributing:
/// graded, with key points covered and citations to attribute them to.
pub fn to_attribute(simulation: &Simulation, goal: usize) -> Vec<String> {
    simulation
        .round_runs(simulation.round)
        .filter(|r| r.goal == goal && r.outcome == Outcome::Answered && r.support.is_none())
        .filter(|r| r.covered.as_ref().is_some_and(|c| !c.is_empty()) && !r.citations.is_empty())
        .map(|r| r.task.clone())
        .collect()
}

pub fn attribute_task(simulation: &Simulation, goal: usize, tasks: &[String]) -> String {
    let goal_ = &simulation.goals[goal];
    let mut out = format!("## Question\n\n{}\n\n## Key points\n\n", goal_.question);
    for (i, point) in goal_.key_points.iter().enumerate() {
        out.push_str(&format!("{}. {}\n", i + 1, point.point));
    }
    out.push_str("\n## Answers\n");
    for (n, task) in tasks.iter().enumerate() {
        let Some(run) = simulation.runs.iter().find(|r| &r.task == task) else {
            continue;
        };
        let covered: Vec<String> = run
            .covered
            .iter()
            .flatten()
            .map(|k| (k + 1).to_string())
            .collect();
        out.push_str(&format!(
            "\n### Answer {}\n\n{}\n\nCovers key points: {}\n\nQuotations:\n\n",
            n + 1,
            run.answer.trim(),
            covered.join(", ")
        ));
        for (c, (_, quote)) in run.citations.iter().enumerate() {
            out.push_str(&format!("{}. \"{}\"\n", c + 1, quote.trim()));
        }
    }
    out
}

/// Check an attribution and record it on the runs.
pub fn apply_attribution(
    simulation: &mut Simulation,
    tasks: &[String],
    answer: AttributeAnswer,
) -> crate::error::Result<()> {
    let mut errors = Vec::new();
    for n in 1..=tasks.len() {
        match answer.answers.iter().filter(|a| a.answer == n).count() {
            1 => {}
            0 => errors.push(format!("no entry for answer {n}")),
            _ => errors.push(format!("answer {n} has more than one entry")),
        }
    }
    for a in &answer.answers {
        let Some(run) = tasks
            .get(a.answer.wrapping_sub(1))
            .and_then(|t| simulation.runs.iter().find(|r| &r.task == t))
        else {
            errors.push(format!("there is no answer {}", a.answer));
            continue;
        };
        let covered = run.covered.clone().unwrap_or_default();
        let mut seen = Vec::new();
        for s in &a.support {
            if !covered.contains(&s.point.wrapping_sub(1)) {
                errors.push(format!(
                    "answer {}: key point {} is not listed as covered",
                    a.answer, s.point
                ));
            }
            if s.quotation == 0 || s.quotation > run.citations.len() {
                errors.push(format!(
                    "answer {}: quotation {} does not exist (1 to {})",
                    a.answer,
                    s.quotation,
                    run.citations.len()
                ));
            }
            if seen.contains(&s.point) {
                errors.push(format!(
                    "answer {}: key point {} has two quotations",
                    a.answer, s.point
                ));
            }
            seen.push(s.point);
        }
    }
    if !errors.is_empty() {
        return Err(rejected(errors));
    }
    for a in answer.answers {
        let task = &tasks[a.answer - 1];
        if let Some(run) = simulation.runs.iter_mut().find(|r| &r.task == task) {
            run.support = Some(
                a.support
                    .iter()
                    .map(|s| (s.point - 1, s.quotation - 1))
                    .collect(),
            );
        }
    }
    Ok(())
}

/// A sentence readers relied on, with how often it carried a key point.
#[derive(Debug, Clone, PartialEq)]
pub struct Pivot {
    pub page: String,
    pub quote: String,
    /// Key points it carried, over all runs.
    pub points: usize,
    /// Distinct runs it carried a key point in.
    pub runs: usize,
}

/// The sentences that carried key points, by how many key points they carried, most first.
pub fn pivots(simulation: &Simulation) -> Vec<Pivot> {
    let mut found: BTreeMap<(String, String), (String, usize, Vec<&str>)> = BTreeMap::new();
    for run in &simulation.runs {
        for &(_, c) in run.support.iter().flatten() {
            let Some((page, quote)) = run.citations.get(c) else {
                continue;
            };
            let entry = found
                .entry((page.clone(), key(quote)))
                .or_insert_with(|| (quote.clone(), 0, Vec::new()));
            entry.1 += 1;
            if !entry.2.contains(&run.task.as_str()) {
                entry.2.push(&run.task);
            }
        }
    }
    let mut pivots: Vec<Pivot> = found
        .into_iter()
        .map(|((page, _), (quote, points, runs))| Pivot {
            page,
            quote,
            points,
            runs: runs.len(),
        })
        .collect();
    pivots.sort_by(|a, b| b.points.cmp(&a.points).then(b.runs.cmp(&a.runs)));
    pivots
}

/// Whether two quotes from one page are the same sentence, or one lies inside the other,
/// compared as plain text: readers quote links as they saw them, with opaque handles.
pub fn same_passage(a: &str, b: &str) -> bool {
    let plain = |s: &str| normalise(&crate::relations::plain(s)).to_lowercase();
    let (a, b) = (plain(a), plain(b));
    !a.is_empty() && !b.is_empty() && (a.contains(&b) || b.contains(&a))
}

/// The key ideas readers named for one page: their statements and quotes, and whether any
/// two readers quoted the same passage.
pub struct KeyIdeas<'a> {
    pub page: &'a str,
    pub ideas: Vec<(&'a str, &'a str)>,
    pub agree: bool,
}

/// The key ideas readers of `round` (or of every round, for `None`) named, per page, leaving
/// out runs on held-out goals when `repair_only`.
pub fn key_ideas(
    simulation: &Simulation,
    round: Option<u32>,
    repair_only: bool,
) -> Vec<KeyIdeas<'_>> {
    let mut by_page: BTreeMap<&str, Vec<(&str, &str)>> = BTreeMap::new();
    for run in &simulation.runs {
        if round.is_some_and(|r| r != run.round)
            || (repair_only && simulation.goals[run.goal].held_out)
        {
            continue;
        }
        if let Some(idea) = &run.key_idea {
            by_page
                .entry(&idea.page)
                .or_default()
                .push((&idea.statement, &idea.quote));
        }
    }
    by_page
        .into_iter()
        .map(|(page, ideas)| {
            let agree = ideas
                .iter()
                .enumerate()
                .any(|(i, (_, a))| ideas[i + 1..].iter().any(|(_, b)| same_passage(a, b)));
            KeyIdeas { page, ideas, agree }
        })
        .collect()
}

/// A sentence that links pages under different main steps of the argument.
#[derive(Debug, Clone, PartialEq)]
pub struct Joining {
    pub page: String,
    pub sentence: String,
    /// The pages it links that sit under another main step.
    pub links: Vec<String>,
}

/// The main step of the argument a page sits under: the root's child whose subtree presents
/// the page, or, for a page that presents no step, the first step that relies on it.
fn main_step<'a>(argument: &'a Argument, id: &str) -> Option<&'a Node> {
    let top = |node: &Node| {
        argument
            .root
            .children
            .iter()
            .find(|c| c.walk().iter().any(|(_, n)| std::ptr::eq(*n, node)))
    };
    let node = argument
        .module(id)
        .map(|(_, n)| n)
        .or_else(|| argument.users(id).into_iter().next())?;
    top(node)
}

/// The sentences of a markdown body: split after a full stop, question mark or exclamation
/// mark followed by a space and a capital, or at a blank line.
fn sentences(body: &str) -> Vec<&str> {
    let mut out = Vec::new();
    for paragraph in body.split("\n\n") {
        let mut start = 0;
        let bytes = paragraph.as_bytes();
        for (i, c) in paragraph.char_indices() {
            if matches!(c, '.' | '?' | '!')
                && bytes.get(i + 1) == Some(&b' ')
                && paragraph[i + 2..]
                    .chars()
                    .next()
                    .is_some_and(char::is_uppercase)
            {
                out.push(paragraph[start..=i].trim());
                start = i + 2;
            }
        }
        let rest = paragraph[start..].trim();
        if !rest.is_empty() {
            out.push(rest);
        }
    }
    out
}

/// Sentences that link a page sitting under a different main step of the argument from
/// their own page: candidates for reconnections, which join distant parts of the argument.
pub fn joining(argument: &Argument, pages: &[(&Concept, String)]) -> Vec<Joining> {
    let mut found = Vec::new();
    for (c, body) in pages {
        let Some(home) = main_step(argument, &c.id) else {
            continue;
        };
        for sentence in sentences(&crate::cite::strip(body)) {
            let mut links: Vec<String> = Vec::new();
            for event in Parser::new_ext(sentence, Options::ENABLE_MATH) {
                if let Event::Start(Tag::Link { dest_url, .. }) = event
                    && let Some(id) = link_target(&dest_url).filter(|id| !id.is_empty())
                    && id != c.id
                    && main_step(argument, id).is_some_and(|other| !std::ptr::eq(other, home))
                    && !links.iter().any(|l| l == id)
                {
                    links.push(id.to_string());
                }
            }
            if !links.is_empty() {
                found.push(Joining {
                    page: c.id.clone(),
                    sentence: sentence.to_string(),
                    links,
                });
            }
        }
    }
    found
}

/// A sentence to hide in an ablation run, and the goals to rerun without it.
#[derive(Debug, Clone, PartialEq)]
pub struct Target {
    pub hidden: Hidden,
    pub goals: Vec<usize>,
}

/// The ablation targets: for each of the `n` sentences that carried most key points, the
/// sentence itself and a control from the same page, each with the goals the sentence
/// supported. The control is the sentence of the page closest in length that no reader
/// cited, within half to twice the candidate's length; a candidate with no such control is
/// skipped, since its effect could not be told from the effect of removing any sentence.
pub fn ablation_targets(
    simulation: &Simulation,
    pages: &[(&Concept, String)],
    n: usize,
) -> Vec<Target> {
    let mut targets = Vec::new();
    for pivot in pivots(simulation).iter().take(n) {
        let Some((_, body)) = pages.iter().find(|(c, _)| c.id == pivot.page) else {
            continue;
        };
        let body = crate::cite::strip(body);
        let all = sentences(&body);
        let Some(candidate) = all.iter().find(|s| same_passage(s, &pivot.quote)) else {
            continue;
        };
        let cited = |sentence: &str| {
            simulation.runs.iter().any(|r| {
                r.citations
                    .iter()
                    .any(|(page, quote)| page == &pivot.page && same_passage(sentence, quote))
            })
        };
        let length = candidate.chars().count();
        let control = all
            .iter()
            .filter(|s| *s != candidate && !cited(s))
            .filter(|s| (length / 2..=length * 2).contains(&s.chars().count()))
            .min_by_key(|s| s.chars().count().abs_diff(length));
        let Some(control) = control else {
            continue;
        };
        let mut goals: Vec<usize> = simulation
            .runs
            .iter()
            .filter(|r| {
                r.support.iter().flatten().any(|&(_, c)| {
                    r.citations.get(c).is_some_and(|(page, quote)| {
                        page == &pivot.page && same_passage(quote, &pivot.quote)
                    })
                })
            })
            .map(|r| r.goal)
            .collect();
        goals.sort();
        goals.dedup();
        for (sentence, control) in [(candidate, false), (control, true)] {
            targets.push(Target {
                hidden: Hidden {
                    page: pivot.page.clone(),
                    sentence: sentence.to_string(),
                    control,
                },
                goals: goals.clone(),
            });
        }
    }
    targets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sentences_split_after_stops_before_capitals() {
        assert_eq!(
            sentences("One step. Two, e.g. this. And $x.y$ three!\n\nNew paragraph"),
            vec![
                "One step.",
                "Two, e.g. this.",
                "And $x.y$ three!",
                "New paragraph"
            ]
        );
    }

    #[test]
    fn sentences_linking_other_main_steps_join_the_argument() {
        use crate::argument::Intention;
        use crate::graph::tests::concept;
        let step = |id: &str, page: &str| Node {
            id: id.into(),
            intention: Intention::Infer,
            claim: id.into(),
            from: vec![],
            method: None,
            motivation: None,
            salient: None,
            adds: vec![],
            page: Some(page.into()),
            quote: None,
            deferred: false,
            children: vec![],
        };
        let mut root = step("S0", "t");
        root.children = vec![step("S1", "a"), step("S2", "b")];
        root.children[0].from = vec!["k".into()];
        let argument = Argument {
            planned: vec![],
            root,
        };
        let (a, k) = (concept("a", &[], false), concept("k", &[], false));
        let pages = vec![
            (
                &a,
                "Uses [k](k.md) here. This is why [b](b.md) works. Plain.".to_string(),
            ),
            (&k, "Defined for [a](a.md).".to_string()),
        ];
        let found = joining(&argument, &pages);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].sentence, "This is why [b](b.md) works.");
        assert_eq!(found[0].links, vec!["b"]);
    }

    #[test]
    fn passages_match_inside_each_other() {
        assert!(same_passage("so it is exact", "So it is exact on a tape"));
        assert!(!same_passage("so it is exact", "it rounds"));
        assert!(!same_passage("", "it rounds"));
        assert!(same_passage(
            "the [DFT](kur) splits",
            "So the [DFT](dft.md) splits it."
        ));
    }
}

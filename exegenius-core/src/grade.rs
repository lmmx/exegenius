//! Grade tasks: score the simulated readers' answers to one goal, blind, against the goal's
//! key points; then turn what the readers could not do into rewrite notes.
//!
//! The grader sees the question, the key points and the answers under letters, in an
//! order fixed by a hash of the task ids, so it cannot tell a reader's answer from the
//! no-wiki baseline or one reader profile from another.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::Result;
use crate::graph::Graph;
use crate::simulate::{Goal, HOME, Level, Outcome, Simulation, rejected};

pub const GRADE_INSTRUCTIONS: &str = "\
Grade answers to a question against its key points. Each answer was written by a different \
reader; grade each one on its own.

- For each answer, list the numbers of the key points it makes, correctly, in its own words \
or the key point's. Credit a key point only if the answer states it or clearly implies it; a \
vague gesture towards it does not count.
- List the numbers of the key points an answer contradicts: it states the opposite, or \
something incompatible. A key point an answer neither makes nor contradicts is missing; it \
needs no list.
- List any other claims in an answer that are wrong, briefly.
- An answer that says it cannot answer covers no key points.
- Grade what is written, not what the writer may have meant.";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GradeAnswer {
    pub grades: Vec<Grade>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Grade {
    /// The answer's letter.
    pub label: String,
    /// Numbers of the key points the answer makes.
    pub covered: Vec<usize>,
    /// Numbers of the key points the answer contradicts.
    pub contradicted: Vec<usize>,
    /// Wrong claims in the answer that contradict no key point.
    pub wrong: Vec<String>,
}

fn letter(i: usize) -> String {
    let mut label = String::new();
    let mut n = i;
    loop {
        label.insert(0, (b'A' + (n % 26) as u8) as char);
        if n < 26 {
            break;
        }
        n = n / 26 - 1;
    }
    label
}

/// The answers to grade for a goal in the current round, in blind order: task ids of the
/// runs that answered and of the ungraded baseline. Runs that gave up need no grader.
pub fn to_grade(simulation: &Simulation, goal: usize) -> Vec<String> {
    let mut tasks: Vec<String> = simulation
        .round_runs(simulation.round)
        .filter(|r| r.goal == goal && r.covered.is_none() && r.outcome == Outcome::Answered)
        .map(|r| r.task.clone())
        .chain(
            simulation
                .baselines
                .iter()
                .filter(|b| b.goal == goal && b.covered.is_none())
                .map(|b| b.task.clone()),
        )
        .collect();
    tasks.sort_by_key(|t| Sha256::digest(t.as_bytes()).to_vec());
    tasks
}

fn answer_text(simulation: &Simulation, task: &str) -> String {
    simulation
        .runs
        .iter()
        .find(|r| r.task == task)
        .map(|r| r.answer.clone())
        .or_else(|| {
            simulation
                .baselines
                .iter()
                .find(|b| b.task == task)
                .map(|b| b.answer.clone())
        })
        .unwrap_or_default()
}

pub fn grade_task(simulation: &Simulation, goal: &Goal, tasks: &[String]) -> String {
    let mut out = format!("## Question\n\n{}\n\n## Key points\n\n", goal.question);
    for (i, point) in goal.key_points.iter().enumerate() {
        out.push_str(&format!("{}. {}\n", i + 1, point.point));
    }
    out.push_str("\n## Answers\n");
    for (i, task) in tasks.iter().enumerate() {
        out.push_str(&format!(
            "\n### Answer {}\n\n{}\n",
            letter(i),
            answer_text(simulation, task).trim()
        ));
    }
    out
}

/// Check the grades and record them on the runs and baselines they belong to.
pub fn apply_grade(
    simulation: &mut Simulation,
    goal: usize,
    tasks: &[String],
    answer: GradeAnswer,
) -> Result<()> {
    let points = simulation.goals[goal].key_points.len();
    let labels: Vec<String> = (0..tasks.len()).map(letter).collect();
    let mut errors = Vec::new();
    for label in &labels {
        match answer.grades.iter().filter(|g| &g.label == label).count() {
            1 => {}
            0 => errors.push(format!("no grade for answer {label}")),
            _ => errors.push(format!("answer {label} is graded more than once")),
        }
    }
    for g in &answer.grades {
        if !labels.contains(&g.label) {
            errors.push(format!("there is no answer {}", g.label));
        }
        for &n in g.covered.iter().chain(&g.contradicted) {
            if n == 0 || n > points {
                errors.push(format!(
                    "answer {}: key point {n} does not exist (1 to {points})",
                    g.label
                ));
            }
        }
        if let Some(n) = g.covered.iter().find(|n| g.contradicted.contains(n)) {
            errors.push(format!(
                "answer {}: key point {n} is both covered and contradicted",
                g.label
            ));
        }
    }
    if !errors.is_empty() {
        return Err(rejected(errors));
    }
    for g in answer.grades {
        let task = &tasks[labels.iter().position(|l| *l == g.label).expect("checked")];
        let indices = |numbers: &[usize]| {
            let mut indices: Vec<usize> = numbers.iter().map(|n| n - 1).collect();
            indices.sort();
            indices.dedup();
            indices
        };
        let (covered, contradicted) = (indices(&g.covered), indices(&g.contradicted));
        if let Some(run) = simulation.runs.iter_mut().find(|r| &r.task == task) {
            run.covered = Some(covered);
            run.contradicted = contradicted;
            run.wrong = g.wrong;
        } else if let Some(b) = simulation.baselines.iter_mut().find(|b| &b.task == task) {
            b.covered = Some(covered);
            b.contradicted = contradicted;
            b.wrong = g.wrong;
        }
    }
    Ok(())
}

/// Queue rewrite notes from a finished round: every stuck point, every page a reader gave
/// up on, and, for each goal, the key points no reader managed. Runs on held-out goals are
/// left out, so held-out goals keep measuring what the wiki teaches rather than what it was
/// repaired to answer. Returns the pages queued.
pub fn repair(simulation: &Simulation, graph: &mut Graph, round: u32) -> Vec<String> {
    let mut pages = Vec::new();
    let mut queue = |graph: &mut Graph, page: &str, note: String| {
        if page != HOME && graph.get(page).is_some() {
            graph.queue_rewrite(page, Some(note));
            if !pages.iter().any(|p| p == page) {
                pages.push(page.to_string());
            }
        }
    };
    for run in simulation.round_runs(round) {
        let goal = &simulation.goals[run.goal];
        if goal.held_out {
            continue;
        }
        let question = &goal.question;
        for s in &run.stuck {
            let level = s.level.map_or(String::new(), |l| match l {
                Level::Chain => " (the reader followed the steps but not why they lead where \
                                 they do: say what this step is for)"
                    .to_string(),
                l => format!(" (stuck at the {} level)", l.name()),
            });
            queue(
                graph,
                &s.page,
                format!(
                    "A simulated reader ({}) trying to answer \"{question}\" got stuck at \
                     \"{}\": {}{level}",
                    run.persona, s.quote, s.note
                ),
            );
        }
        if run.outcome == Outcome::GaveUp
            && let Some(last) = run.path.last()
        {
            queue(
                graph,
                last,
                format!(
                    "A simulated reader ({}) trying to answer \"{question}\" gave up on this \
                     page: {}",
                    run.persona, run.reason
                ),
            );
        }
    }
    for (i, goal) in simulation.goals.iter().enumerate() {
        if goal.held_out {
            continue;
        }
        let runs: Vec<_> = simulation
            .round_runs(round)
            .filter(|r| r.goal == i)
            .collect();
        if runs.is_empty() {
            continue;
        }
        let missed: Vec<&str> = goal
            .key_points
            .iter()
            .enumerate()
            .filter(|(k, _)| {
                !runs
                    .iter()
                    .any(|r| r.covered.as_ref().is_some_and(|c| c.contains(k)))
            })
            .map(|(_, p)| p.point.as_str())
            .collect();
        if missed.is_empty() {
            continue;
        }
        for page in &goal.pages {
            queue(
                graph,
                page,
                format!(
                    "Simulated readers trying to answer \"{}\" could not work out: {}. A reader \
                     with the stated background should be able to get this from this page and \
                     the pages it builds on.",
                    goal.question,
                    missed.join("; ")
                ),
            );
        }
    }
    pages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letters() {
        assert_eq!(
            [0, 1, 25, 26, 27].map(letter),
            ["A", "B", "Z", "AA", "AB"].map(String::from)
        );
    }
}

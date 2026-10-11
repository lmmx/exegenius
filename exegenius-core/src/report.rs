//! The simulation report, `exegenius/report.md`: how the simulated readers did, round by
//! round, and where they got stuck. It is a working document for the author, kept out of
//! `docs/`: the goals test the wiki and are not part of it.

use std::collections::BTreeMap;

use crate::config::Config;
use crate::graph::Graph;
use crate::simulate::{HOME, Outcome, Run, Simulation};

fn name<'a>(graph: &'a Graph, id: &'a str) -> &'a str {
    if id == HOME {
        return "Front page";
    }
    graph.get(id).map_or(id, |c| c.name.as_str())
}

fn coverage(covered: &Option<Vec<usize>>, points: usize) -> Option<f64> {
    covered
        .as_ref()
        .map(|c| c.len() as f64 / points.max(1) as f64)
}

fn percent(values: &[f64]) -> String {
    if values.is_empty() {
        return "–".into();
    }
    format!(
        "{:.0}%",
        100.0 * values.iter().sum::<f64>() / values.len() as f64
    )
}

/// Mean share of key points covered by the graded runs of `round`.
pub fn round_coverage(simulation: &Simulation, round: u32) -> Option<f64> {
    let values: Vec<f64> = simulation
        .round_runs(round)
        .filter_map(|r| coverage(&r.covered, simulation.goals[r.goal].key_points.len()))
        .collect();
    (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}

/// Mean share of key points the no-wiki baselines covered.
pub fn baseline_coverage(simulation: &Simulation) -> Option<f64> {
    let values: Vec<f64> = simulation
        .baselines
        .iter()
        .filter_map(|b| coverage(&b.covered, simulation.goals[b.goal].key_points.len()))
        .collect();
    (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}

fn path(graph: &Graph, run: &Run) -> String {
    let mut steps: Vec<String> = run
        .path
        .iter()
        .map(|id| name(graph, id).to_string())
        .collect();
    if run.outcome == Outcome::GaveUp {
        steps.push("(gave up)".into());
    }
    steps.join(" → ")
}

pub fn report(config: &Config, graph: &Graph, simulation: &Simulation) -> String {
    let mut out = String::from("# Simulated readers\n\n");
    out.push_str(&format!(
        "Each simulated reader tried to answer one goal question using only the wiki, seen \
         through `exegenius read` and `exegenius search`, with at most {} pages per goal{}. \
         A no-wiki baseline answered each question from the reader's background alone. A \
         blind grader scored every answer against the goal's key points. Stuck points, \
         give-ups and key points no reader managed became rewrite notes after every round \
         but the last.\n\nReaders:\n\n",
        config.simulation.budget,
        if config.simulation.nonce {
            ", and the concepts the wiki teaches under invented names"
        } else {
            ""
        }
    ));
    for persona in config.personas() {
        out.push_str(&format!(
            "- **{}**: {}\n",
            persona.name,
            persona.knows.trim()
        ));
    }

    let rounds: Vec<u32> = {
        let mut r: Vec<u32> = simulation.runs.iter().map(|r| r.round).collect();
        r.sort();
        r.dedup();
        r
    };
    out.push_str(
        "\n## Summary\n\n| Round | Runs | Answered | Gave up | Key points covered | Leaks |\n\
         |---|---|---|---|---|---|\n",
    );
    for &round in &rounds {
        let runs: Vec<&Run> = simulation.round_runs(round).collect();
        let answered = runs
            .iter()
            .filter(|r| r.outcome == Outcome::Answered)
            .count();
        let values: Vec<f64> = runs
            .iter()
            .filter_map(|r| coverage(&r.covered, simulation.goals[r.goal].key_points.len()))
            .collect();
        out.push_str(&format!(
            "| {round} | {} | {answered} | {} | {} | {} |\n",
            runs.len(),
            runs.len() - answered,
            percent(&values),
            runs.iter().filter(|r| !r.leaks.is_empty()).count()
        ));
    }
    let baseline: Vec<f64> = simulation
        .baselines
        .iter()
        .filter_map(|b| coverage(&b.covered, simulation.goals[b.goal].key_points.len()))
        .collect();
    out.push_str(&format!(
        "| baseline (no wiki) | {} | | | {} | |\n",
        simulation.baselines.len(),
        percent(&baseline)
    ));

    for (g, goal) in simulation.goals.iter().enumerate() {
        out.push_str(&format!(
            "\n## Goal {}: {}\n\n{} goal. Key points:\n\n",
            g + 1,
            goal.question,
            match goal.kind {
                crate::simulate::GoalKind::Recall => "Recall",
                crate::simulate::GoalKind::Transfer => "Transfer",
            }
        ));
        for (i, point) in goal.key_points.iter().enumerate() {
            out.push_str(&format!("{}. {point}\n", i + 1));
        }
        let pages: Vec<&str> = goal.pages.iter().map(|p| name(graph, p)).collect();
        out.push_str(&format!("\nPages it needs: {}.\n", pages.join(", ")));
        for b in simulation.baselines.iter().filter(|b| b.goal == g) {
            out.push_str(&format!(
                "\nBaseline: {} of {} key points. \"{}\"\n",
                b.covered
                    .as_ref()
                    .map_or("ungraded".into(), |c| c.len().to_string()),
                goal.key_points.len(),
                b.answer.trim()
            ));
        }
        for run in simulation.runs.iter().filter(|r| r.goal == g) {
            let start = if run.start == HOME {
                "front page"
            } else {
                "deep link"
            };
            let score = match &run.covered {
                Some(c) => format!("{} of {} key points", c.len(), goal.key_points.len()),
                None => "ungraded".into(),
            };
            out.push_str(&format!(
                "\n### Round {}, {}, from the {start}: {}, {score}\n\n",
                run.round,
                run.persona,
                match run.outcome {
                    Outcome::Answered => "answered",
                    Outcome::GaveUp => "gave up",
                }
            ));
            out.push_str(&format!("Path: {}\n", path(graph, run)));
            if !run.searches.is_empty() {
                let searches: Vec<String> = run
                    .searches
                    .iter()
                    .map(|s| format!("\"{}\" ({} found)", s.query, s.found.len()))
                    .collect();
                out.push_str(&format!("\nSearches: {}\n", searches.join(", ")));
            }
            if run.outcome == Outcome::Answered {
                out.push_str(&format!("\nAnswer: {}\n", run.answer.trim()));
            }
            out.push_str(&format!("\nReason: {}\n", run.reason.trim()));
            for (page, quote, note) in &run.stuck {
                out.push_str(&format!(
                    "\n- Stuck on {}: \"{quote}\" — {note}",
                    name(graph, page)
                ));
            }
            if !run.stuck.is_empty() {
                out.push('\n');
            }
            for wrong in &run.wrong {
                out.push_str(&format!("\n- Wrong: {wrong}"));
            }
            if !run.wrong.is_empty() {
                out.push('\n');
            }
            if !run.leaks.is_empty() {
                let leaks: Vec<&str> = run.leaks.iter().map(|id| name(graph, id)).collect();
                out.push_str(&format!(
                    "\nPossible leaks (real names the reader was not shown): {}\n",
                    leaks.join(", ")
                ));
            }
        }
    }

    let mut hotspots: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for run in &simulation.runs {
        for (page, _, _) in &run.stuck {
            hotspots.entry(page).or_default().0 += 1;
        }
        if run.outcome == Outcome::GaveUp
            && let Some(last) = run.path.last()
        {
            hotspots.entry(last).or_default().1 += 1;
        }
    }
    if !hotspots.is_empty() {
        let mut rows: Vec<_> = hotspots.into_iter().collect();
        rows.sort_by_key(|(_, (s, g))| std::cmp::Reverse(s + g));
        out.push_str("\n## Where readers got stuck\n\n| Page | Stuck points | Gave up here |\n|---|---|---|\n");
        for (page, (stuck, gave_up)) in rows {
            out.push_str(&format!(
                "| {} | {stuck} | {gave_up} |\n",
                name(graph, page)
            ));
        }
    }
    out
}

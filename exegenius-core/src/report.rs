//! The simulation report, `exegenius/report.md`: how the simulated readers did, round by
//! round, and where they got stuck. It is a working document for the author, kept out of
//! `docs/`: the goals test the wiki and are not part of it.

use std::collections::BTreeMap;

use crate::aha::Joining;
use crate::config::Config;
use crate::graph::Graph;
use crate::simulate::{HOME, Level, Outcome, Role, Run, Simulation};

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

fn mean(values: &[f64]) -> Option<f64> {
    (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}

/// Mean share of key points covered by the graded runs of `round` on repair goals, or on
/// held-out goals.
pub fn round_coverage(simulation: &Simulation, round: u32, held_out: bool) -> Option<f64> {
    let values: Vec<f64> = simulation
        .round_runs(round)
        .filter(|r| simulation.goals[r.goal].held_out == held_out)
        .filter_map(|r| coverage(&r.covered, simulation.goals[r.goal].key_points.len()))
        .collect();
    mean(&values)
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

/// Signs of fragmentation and of collapse, round by round, and the comparison across reader
/// profiles that shows whether explicitness for one reader costs another.
fn layering(graph: &Graph, config: &Config, simulation: &Simulation) -> String {
    let mut out = String::new();
    let mut rounds: Vec<u32> = simulation.runs.iter().map(|r| r.round).collect();
    rounds.sort();
    rounds.dedup();
    if !rounds.is_empty() {
        out.push_str(
            "\n## Navigation\n\nSigns of fragmentation: pages opened per run (the front page \
             aside), the share of opened pages a reader left without citing, and runs on goals \
             that need several pages that cited only one.\n\n\
             | Round | Pages opened per run | Opened, not cited | Multi-page goals answered from one page |\n\
             |---|---|---|---|\n",
        );
        for &round in &rounds {
            let runs: Vec<&Run> = simulation
                .round_runs(round)
                .filter(|r| r.hidden.is_none())
                .collect();
            let opened: Vec<Vec<&str>> = runs
                .iter()
                .map(|r| {
                    let mut pages: Vec<&str> = r
                        .path
                        .iter()
                        .map(String::as_str)
                        .filter(|p| *p != HOME)
                        .collect();
                    pages.sort();
                    pages.dedup();
                    pages
                })
                .collect();
            let per_run: Vec<f64> = opened.iter().map(|p| p.len() as f64).collect();
            let (mut total, mut uncited) = (0, 0);
            for (run, pages) in runs.iter().zip(&opened) {
                total += pages.len();
                uncited += pages
                    .iter()
                    .filter(|p| !run.citations.iter().any(|(c, _)| c == *p))
                    .count();
            }
            let multi: Vec<&&Run> = runs
                .iter()
                .filter(|r| r.outcome == Outcome::Answered)
                .filter(|r| simulation.goals[r.goal].pages.len() > 1)
                .collect();
            let single = multi
                .iter()
                .filter(|r| {
                    let mut cited: Vec<&str> =
                        r.citations.iter().map(|(p, _)| p.as_str()).collect();
                    cited.sort();
                    cited.dedup();
                    cited.len() == 1
                })
                .count();
            out.push_str(&format!(
                "| {round} | {} | {} | {single} of {} |\n",
                mean(&per_run).map_or("–".into(), |m| format!("{m:.1}")),
                percent(&[uncited as f64 / total.max(1) as f64]),
                multi.len()
            ));
        }
    }
    if simulation.words.len() > 1 {
        let (first, last) = (
            simulation.words.iter().next().expect("two rounds"),
            simulation.words.iter().next_back().expect("two rounds"),
        );
        out.push_str(
            "\n## Page length by round\n\nA sign of collapse: page bodies growing as repairs \
             add text, round by round.\n\n| Round | Words | Pages |\n|---|---|---|\n",
        );
        for (round, words) in &simulation.words {
            out.push_str(&format!(
                "| {round} | {} | {} |\n",
                words.values().sum::<usize>(),
                words.len()
            ));
        }
        let mut growth: Vec<(&String, usize, usize)> = last
            .1
            .iter()
            .filter_map(|(page, &after)| Some((page, *first.1.get(page)?, after)))
            .filter(|(_, before, after)| after > before)
            .collect();
        growth.sort_by_key(|(_, before, after)| std::cmp::Reverse(after - before));
        if !growth.is_empty() {
            out.push_str(&format!(
                "\nPages that grew most from round {} to round {}:\n\n",
                first.0, last.0
            ));
            for (page, before, after) in growth.iter().take(5) {
                out.push_str(&format!(
                    "- {}: {before} → {after} words\n",
                    name(graph, page)
                ));
            }
        }
    }
    let personas = config.personas();
    if personas.len() > 1 {
        out.push_str(
            "\n## By reader profile\n\nThe expertise-reversal test: if the wiki's explicitness \
             helps one profile and slows another (more pages opened, fewer key points), it has \
             gone too far for the second.\n\n\
             | Reader | Runs | Answered | Key points covered | Pages opened per run |\n\
             |---|---|---|---|---|\n",
        );
        for persona in &personas {
            let runs: Vec<&Run> = simulation
                .runs
                .iter()
                .filter(|r| r.persona == persona.name && r.hidden.is_none())
                .collect();
            let covered: Vec<f64> = runs
                .iter()
                .filter_map(|r| coverage(&r.covered, simulation.goals[r.goal].key_points.len()))
                .collect();
            let opened: Vec<f64> = runs
                .iter()
                .map(|r| r.path.iter().filter(|p| *p != HOME).count() as f64)
                .collect();
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                persona.name,
                runs.len(),
                runs.iter()
                    .filter(|r| r.outcome == Outcome::Answered)
                    .count(),
                percent(&covered),
                mean(&opened).map_or("–".into(), |m| format!("{m:.1}"))
            ));
        }
    }
    out
}

/// Coverage by goal kind against the baseline, by key point role, and stuck points by level.
fn profiles(simulation: &Simulation) -> String {
    let mut out = String::new();
    let goal = |g: usize| &simulation.goals[g];
    let mut kinds: BTreeMap<&str, (usize, Vec<f64>, Vec<f64>)> = BTreeMap::new();
    for (g, goal) in simulation.goals.iter().enumerate() {
        let entry = kinds.entry(goal.kind.name()).or_default();
        entry.0 += 1;
        let points = goal.key_points.len();
        entry.1.extend(
            simulation
                .runs
                .iter()
                .filter(|r| r.goal == g)
                .filter_map(|r| coverage(&r.covered, points)),
        );
        entry.2.extend(
            simulation
                .baselines
                .iter()
                .filter(|b| b.goal == g)
                .filter_map(|b| coverage(&b.covered, points)),
        );
    }
    if !kinds.is_empty() {
        out.push_str(
            "\n## By goal kind\n\nAll rounds. The gap between readers and the baseline shows \
             what the wiki adds for each kind of understanding.\n\n\
             | Kind | Goals | Readers | Baseline |\n|---|---|---|---|\n",
        );
        for (kind, (goals, readers, baseline)) in kinds {
            out.push_str(&format!(
                "| {kind} | {goals} | {} | {} |\n",
                percent(&readers),
                percent(&baseline)
            ));
        }
    }
    let mut roles: BTreeMap<Role, (usize, usize, usize)> = BTreeMap::new();
    for run in simulation.runs.iter().filter(|r| r.covered.is_some()) {
        for (k, point) in goal(run.goal).key_points.iter().enumerate() {
            let entry = roles.entry(point.role).or_default();
            entry.0 += 1;
            if run.covered.as_ref().is_some_and(|c| c.contains(&k)) {
                entry.1 += 1;
            }
            if run.contradicted.contains(&k) {
                entry.2 += 1;
            }
        }
    }
    if !roles.is_empty() {
        out.push_str(
            "\n## By key point role\n\nGraded runs, all rounds. Readers who make claims and \
             conclusions but miss reasons point to pages that state results without their \
             justification.\n\n| Role | Graded | Covered | Contradicted |\n|---|---|---|---|\n",
        );
        for (role, (graded, covered, contradicted)) in roles {
            out.push_str(&format!(
                "| {} | {graded} | {} | {contradicted} |\n",
                role.name(),
                percent(&[covered as f64 / graded.max(1) as f64])
            ));
        }
    }
    let mut levels: BTreeMap<Option<Level>, usize> = BTreeMap::new();
    for s in simulation.runs.iter().flat_map(|r| &r.stuck) {
        *levels.entry(s.level).or_default() += 1;
    }
    if !levels.is_empty() {
        out.push_str("\n## Stuck points by level\n\n| Level | Stuck points |\n|---|---|\n");
        for (level, n) in levels {
            out.push_str(&format!(
                "| {} | {n} |\n",
                level.map_or("not given", Level::name)
            ));
        }
    }
    out
}

/// The candidates for aha moments, each method apart: the sentences that carried key
/// points, the key ideas readers named, and the sentences that join distant parts of the
/// argument.
fn moments(graph: &Graph, simulation: &Simulation, joining: &[Joining]) -> String {
    let mut out = String::new();
    let pivots = crate::aha::pivots(simulation);
    if !pivots.is_empty() {
        out.push_str(
            "\n## Sentences that carried key points\n\nFrom the attribution tasks: the \
             quotations that supported covered key points, most first. Citing a sentence is not \
             learning from it, so these are candidates.\n\n\
             | Page | Sentence | Key points | Runs |\n|---|---|---|---|\n",
        );
        for p in pivots.iter().take(15) {
            out.push_str(&format!(
                "| {} | \"{}\" | {} | {} |\n",
                name(graph, &p.page),
                p.quote.replace('|', "\\|"),
                p.points,
                p.runs
            ));
        }
    }
    let ideas = crate::aha::key_ideas(simulation, None, false);
    if !ideas.is_empty() {
        out.push_str(
            "\n## Key ideas readers named\n\nPer page, the key ideas readers named, with \
             whether two of them quoted the same passage.\n",
        );
        for page in ideas {
            out.push_str(&format!(
                "\n- **{}** ({}):",
                name(graph, page.page),
                if page.agree {
                    "agreement"
                } else {
                    "no agreement"
                }
            ));
            for (statement, quote) in page.ideas {
                out.push_str(&format!("\n  - {statement} (\"{quote}\")"));
            }
            out.push('\n');
        }
    }
    if !joining.is_empty() {
        out.push_str(
            "\n## Sentences that join distant parts of the argument\n\nSentences that link a \
             page under a different main step of the argument from their own page: candidates \
             for reconnections.\n\n",
        );
        for j in joining {
            let links: Vec<&str> = j.links.iter().map(|l| name(graph, l)).collect();
            out.push_str(&format!(
                "- {}: \"{}\" (links {})\n",
                name(graph, &j.page),
                j.sentence,
                links.join(", ")
            ));
        }
    }
    out
}

/// Shares of all key points, and of the reason and conclusion key points, covered by the
/// graded runs on `goal` that `keep` selects.
fn shares(simulation: &Simulation, goal: usize, keep: &dyn Fn(&Run) -> bool) -> Option<(f64, f64)> {
    let points = &simulation.goals[goal].key_points;
    let relational: Vec<usize> = points
        .iter()
        .enumerate()
        .filter(|(_, p)| matches!(p.role, Role::Reason | Role::Conclusion))
        .map(|(k, _)| k)
        .collect();
    let runs: Vec<&Run> = simulation
        .runs
        .iter()
        .filter(|r| r.goal == goal && r.covered.is_some() && keep(r))
        .collect();
    let all: Vec<f64> = runs
        .iter()
        .filter_map(|r| coverage(&r.covered, points.len()))
        .collect();
    let rel: Vec<f64> = runs
        .iter()
        .map(|r| {
            let covered = r.covered.as_deref().unwrap_or_default();
            let hit = relational.iter().filter(|k| covered.contains(k)).count();
            hit as f64 / relational.len().max(1) as f64
        })
        .collect();
    Some((mean(&all)?, mean(&rel)?))
}

/// The ablation runs: for each hidden sentence and goal, coverage before (the last round)
/// and after hiding it; and the candidates whose hiding cost more reason and conclusion
/// key points than hiding their control did, in reading order.
fn ablations(graph: &Graph, simulation: &Simulation, rounds: u32, order: &[String]) -> String {
    let mut hidden: Vec<&crate::simulate::Hidden> = Vec::new();
    for h in simulation.runs.iter().filter_map(|r| r.hidden.as_ref()) {
        if !hidden.contains(&h) {
            hidden.push(h);
        }
    }
    if hidden.is_empty() {
        return String::new();
    }
    let mut out = String::from(
        "\n## Ablations\n\nAfter the last round, readers reran the goals that the sentences \
         carrying most key points had supported, with one sentence hidden at a time, and with \
         a control sentence of similar length from the same page that no reader cited. \
         \"Relational\" counts reason and conclusion key points: a sentence doing relational \
         work costs those more than its control does.\n\n\
         | Page | Hidden sentence | Control | Goal | Covered before | after | Relational before | after |\n\
         |---|---|---|---|---|---|---|---|\n",
    );
    // The relational drop for each hidden sentence, averaged over its goals.
    let mut drops: Vec<(&crate::simulate::Hidden, f64)> = Vec::new();
    for h in &hidden {
        let mut goals: Vec<usize> = simulation
            .runs
            .iter()
            .filter(|r| r.hidden.as_ref() == Some(*h))
            .map(|r| r.goal)
            .collect();
        goals.dedup();
        let mut drop = Vec::new();
        for g in goals {
            let before = shares(simulation, g, &|r| r.round == rounds && r.hidden.is_none());
            let after = shares(simulation, g, &|r| r.hidden.as_ref() == Some(*h));
            let cell = |v: Option<(f64, f64)>, rel: bool| {
                v.map_or("–".to_string(), |(a, r)| {
                    format!("{:.0}%", 100.0 * if rel { r } else { a })
                })
            };
            out.push_str(&format!(
                "| {} | \"{}\" | {} | {} | {} | {} | {} | {} |\n",
                name(graph, &h.page),
                h.sentence.replace('|', "\\|"),
                if h.control { "yes" } else { "no" },
                g + 1,
                cell(before, false),
                cell(after, false),
                cell(before, true),
                cell(after, true)
            ));
            if let (Some(b), Some(a)) = (before, after) {
                drop.push(b.1 - a.1);
            }
        }
        if let Some(d) = mean(&drop) {
            drops.push((h, d));
        }
    }
    let mut confirmed: Vec<&crate::simulate::Hidden> = drops
        .iter()
        .filter(|(h, d)| {
            !h.control
                && drops
                    .iter()
                    .find(|(c, _)| c.control && c.page == h.page)
                    .is_some_and(|(_, control)| d > control)
        })
        .map(|(h, _)| *h)
        .collect();
    let position = |id: &str| order.iter().position(|o| o == id).unwrap_or(usize::MAX);
    confirmed.sort_by_key(|h| position(&h.page));
    out.push_str(
        "\n## Key steps\n\nThe hidden sentences whose loss cost readers more reason and \
         conclusion key points than their control's, in reading order.\n\n",
    );
    if confirmed.is_empty() {
        out.push_str("None so far.\n");
    }
    for h in confirmed {
        out.push_str(&format!("- {}: \"{}\"\n", name(graph, &h.page), h.sentence));
    }
    out
}

pub fn report(
    config: &Config,
    graph: &Graph,
    simulation: &Simulation,
    joining: &[Joining],
    order: &[String],
) -> String {
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
        "\n## Summary\n\nKey points covered are reported separately for repair goals, whose \
         results feed the rewrites, and held-out goals, whose results never do. Only the \
         held-out figure measures whether the wiki teaches beyond the questions it was \
         repaired against.\n\n\
         | Round | Runs | Answered | Gave up | Key points covered | Held-out key points | Leaks |\n\
         |---|---|---|---|---|---|---|\n",
    );
    for &round in &rounds {
        let runs: Vec<&Run> = simulation.round_runs(round).collect();
        let answered = runs
            .iter()
            .filter(|r| r.outcome == Outcome::Answered)
            .count();
        let values = |held_out: bool| -> Vec<f64> {
            runs.iter()
                .filter(|r| simulation.goals[r.goal].held_out == held_out)
                .filter_map(|r| coverage(&r.covered, simulation.goals[r.goal].key_points.len()))
                .collect()
        };
        out.push_str(&format!(
            "| {round} | {} | {answered} | {} | {} | {} | {} |\n",
            runs.len(),
            runs.len() - answered,
            percent(&values(false)),
            percent(&values(true)),
            runs.iter().filter(|r| !r.leaks.is_empty()).count()
        ));
    }
    let baseline: Vec<f64> = simulation
        .baselines
        .iter()
        .filter_map(|b| coverage(&b.covered, simulation.goals[b.goal].key_points.len()))
        .collect();
    out.push_str(&format!(
        "| baseline (no wiki) | {} | | | {} | | |\n",
        simulation.baselines.len(),
        percent(&baseline)
    ));
    out.push_str(&profiles(simulation));
    out.push_str(&layering(graph, config, simulation));

    for (g, goal) in simulation.goals.iter().enumerate() {
        out.push_str(&format!(
            "\n## Goal {}: {}\n\nKind: {}{}. Key points:\n\n",
            g + 1,
            goal.question,
            goal.kind.name(),
            if goal.held_out { ", held out" } else { "" }
        ));
        for (i, point) in goal.key_points.iter().enumerate() {
            out.push_str(&format!(
                "{}. {} ({})\n",
                i + 1,
                point.point,
                point.role.name()
            ));
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
            for s in &run.stuck {
                let level = s
                    .level
                    .map_or(String::new(), |l| format!(" [{}]", l.name()));
                out.push_str(&format!(
                    "\n- Stuck on {}{level}: \"{}\" — {}",
                    name(graph, &s.page),
                    s.quote,
                    s.note
                ));
            }
            if !run.stuck.is_empty() {
                out.push('\n');
            }
            for &k in &run.contradicted {
                out.push_str(&format!(
                    "\n- Contradicts key point {}: {}",
                    k + 1,
                    goal.key_points[k].point
                ));
            }
            for wrong in &run.wrong {
                out.push_str(&format!("\n- Wrong: {wrong}"));
            }
            if !run.wrong.is_empty() || !run.contradicted.is_empty() {
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
        for s in &run.stuck {
            hotspots.entry(&s.page).or_default().0 += 1;
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
    out.push_str(&moments(graph, simulation, joining));
    out.push_str(&ablations(
        graph,
        simulation,
        config.simulation.rounds,
        order,
    ));
    let predicates = crate::relations::predicates(graph);
    if !predicates.is_empty() {
        out.push_str(
            "\n## Relations between pages\n\nThe predicates writers gave for what a page uses \
             each prerequisite for, most frequent first. Vague predicates (\"uses\", \"relates \
             to\") at the top mark pages that rely on a prerequisite without saying how.\n\n\
             | Predicate | Edges |\n|---|---|\n",
        );
        for (predicate, n) in predicates {
            out.push_str(&format!("| {predicate} | {n} |\n"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulate::{Goal, GoalKind, KeyPoint};

    fn run(task: &str, persona: &str, path: &[&str], covered: Vec<usize>) -> Run {
        Run {
            task: task.into(),
            round: 1,
            goal: 0,
            persona: persona.into(),
            start: HOME.into(),
            outcome: Outcome::Answered,
            path: path.iter().map(|p| p.to_string()).collect(),
            searches: vec![],
            answer: "a".into(),
            citations: vec![("p".into(), "q".into())],
            stuck: vec![],
            reason: "r".into(),
            leaks: vec![],
            covered: Some(covered),
            contradicted: vec![],
            wrong: vec![],
            support: None,
            key_idea: None,
            hidden: None,
        }
    }

    #[test]
    fn reader_profiles_are_compared() {
        let config: Config = toml::from_str(
            "target = \"t\"\nsources = []\n[reader]\nknows = \"k\"\n\
             [[readers]]\nname = \"expert\"\nknows = \"more\"\n",
        )
        .unwrap();
        let graph = Graph::default();
        let simulation = Simulation {
            goals: vec![Goal {
                question: "q".into(),
                kind: GoalKind::Summary,
                key_points: vec![
                    KeyPoint {
                        point: "a".into(),
                        role: Role::Claim,
                    },
                    KeyPoint {
                        point: "b".into(),
                        role: Role::Reason,
                    },
                ],
                pages: vec!["p".into(), "s".into()],
                held_out: false,
            }],
            runs: vec![
                run("1", "reader", &[HOME, "p", "s"], vec![0, 1]),
                run("2", "expert", &[HOME, "p", "s", "u", "v"], vec![0]),
            ],
            ..Simulation::default()
        };
        let out = layering(&graph, &config, &simulation);
        assert!(out.contains("| reader | 1 | 1 | 100% | 2.0 |"), "{out}");
        assert!(out.contains("| expert | 1 | 1 | 50% | 4.0 |"), "{out}");
        // Each run cites one page of the goal's two, and opened others without citing them.
        assert!(out.contains("| 1 | 3.0 | 67% | 2 of 2 |"), "{out}");
    }
}

use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::{Parser, Subcommand};
use exegenius_core::audit::load_rounds;
use exegenius_core::build::{BuildReport, build};
use exegenius_core::config;
use exegenius_core::coverage::{Coverage, argument_owners, concept_owners, coverage};
use exegenius_core::graph::Graph;
use exegenius_core::outline;
use exegenius_core::plan::{Plan, plan};
use exegenius_core::project::Project;
use exegenius_core::runlog;
use exegenius_core::task::{self, Kind, Next};

const SKILL: &str = include_str!("skill.md");
const SKILL_PATH: &str = ".claude/skills/exegenius/SKILL.md";
/// Claude Code subagent definitions: the model and the only tools each task may use.
const SUBAGENTS: [(&str, &str); 3] = [
    (
        ".claude/agents/exegenius-task.md",
        include_str!("agents/exegenius-task.md"),
    ),
    (
        ".claude/agents/exegenius-planner.md",
        include_str!("agents/exegenius-planner.md"),
    ),
    (
        ".claude/agents/exegenius-reader.md",
        include_str!("agents/exegenius-reader.md"),
    ),
];
/// Project settings for Claude Code, where `init` adds the deny rules below.
const SETTINGS_PATH: &str = ".claude/settings.json";
/// Bash commands denied to every agent in the project, subagents included: they stop
/// processes by name or pattern, so one task's agent would stop the commands of tasks running
/// beside it. A deny rule matches inside compound commands but not through `bash -c` or
/// `env`, so it is a guard, not a boundary; task files also ask agents not to do this.
const DENIED: [&str; 2] = ["Bash(pkill *)", "Bash(killall *)"];
/// Claude Code's default limit on subagents running at once; spawns past it are refused.
const MAX_SUBAGENTS: usize = 20;

/// Vercel project settings that build the zensical site from a GitHub repository.
const VERCEL_JSON: &str = r#"{
  "framework": null,
  "installCommand": "python3 -m venv .venv && .venv/bin/python -m pip install zensical",
  "buildCommand": ".venv/bin/python -m zensical build",
  "outputDirectory": "site"
}
"#;

/// Build output that does not belong in the repository.
const GITIGNORE: &[&str] = &["site/", ".venv/"];

/// Turn source documents into a wiki of linked explanatory pages for a stated reader.
///
/// exegenius makes no model calls. It hands out tasks (`next`) for an agent such as Claude
/// Code to do, and checks and applies the answers (`submit`).
#[derive(Parser)]
#[command(name = "exegenius", version, after_help = AGENTS)]
struct Cli {
    /// Project directory, containing exegenius.toml
    #[arg(short = 'C', long = "dir", default_value = ".", global = true)]
    dir: PathBuf,
    #[command(subcommand)]
    command: Option<Command>,
}

const AGENTS: &str = "\
For agents: run `exegenius next` in the folder to turn into a wiki, do what it prints, and \
repeat until it prints `done`. It sets the project up first if needed.";

const SETUP: &str = "\
No exegenius.toml here yet. Set up the project, then run `exegenius next` again:

1. Run `exegenius init`.
2. Move the source material into sources/ as text or markdown: move (do not copy) the files
   already in this folder, converting PDFs to text.
3. Edit exegenius.toml: `target` (what the wiki should explain), `reader.knows` (what the
   reader already knows) and one [[sources]] entry per source file. If the user did not say,
   infer the target and reader from the material.";

#[derive(Subcommand)]
enum Command {
    /// Create exegenius.toml and sources/ if missing, and (re)write the Claude Code skill
    Init {
        /// Also add vercel.json and .gitignore entries, so a GitHub repository of this
        /// folder deploys the wiki on Vercel
        #[arg(long)]
        vercel: bool,
    },
    /// Print the current batch of tasks, or build the docs when nothing is left
    Next,
    /// Check and apply the answer to a task
    Submit {
        /// Task id, as printed by `next`
        id: String,
    },
    /// Open a wiki page for a pending simulated-reader task
    Read {
        /// Task id, as printed by `next`
        id: String,
        /// Page handle, from a link or a search result, or `home`
        page: String,
    },
    /// Search the wiki for a pending simulated-reader task
    Search {
        /// Task id, as printed by `next`
        id: String,
        /// Words that must all appear on the page
        #[arg(required = true)]
        words: Vec<String>,
    },
    /// Give the background task again, with a correction of what it said the reader knows
    /// (only right after `next` shows the background)
    Background {
        /// What to correct, in plain words
        #[arg(long)]
        redo: String,
    },
    /// Show progress
    Status,
    /// Print the pages needed to reach the target, in reading order
    Plan,
    /// Write docs/ and zensical.toml from the pages written so far
    Build,
}

fn vercel(dir: &Path) -> anyhow::Result<()> {
    let path = dir.join("vercel.json");
    if path.exists() {
        println!("kept {}", path.display());
    } else {
        std::fs::write(&path, VERCEL_JSON)?;
        println!("created {}", path.display());
    }
    let path = dir.join(".gitignore");
    let existing = if path.exists() {
        std::fs::read_to_string(&path)?
    } else {
        String::new()
    };
    let missing: Vec<&str> = GITIGNORE
        .iter()
        .copied()
        .filter(|entry| !existing.lines().any(|l| l.trim() == *entry))
        .collect();
    if !missing.is_empty() {
        let mut text = existing;
        if !text.is_empty() && !text.ends_with('\n') {
            text.push('\n');
        }
        for entry in missing {
            text.push_str(entry);
            text.push('\n');
        }
        std::fs::write(&path, text)?;
        println!("updated {}", path.display());
    }
    println!(
        "to deploy: push this folder to a GitHub repository, import it at \
         https://vercel.com/new, and deploy; every push then redeploys the wiki"
    );
    Ok(())
}

/// The subagent definition a task goes to. Planning tasks, which decide what the wiki
/// explains and where, go to a larger model than the writing and auditing tasks; simulated
/// readers to their own definition.
fn subagent(kind: &Kind) -> &'static str {
    match kind {
        Kind::Extract | Kind::Background | Kind::Argument | Kind::Extend => "exegenius-planner",
        Kind::Read { .. } | Kind::Baseline { .. } => "exegenius-reader",
        _ => "exegenius-task",
    }
}

/// Add [`DENIED`] to `permissions.deny` in the project's Claude Code settings, keeping
/// everything already there. A settings file that is not a JSON object is left alone.
fn deny_rules(dir: &Path) -> anyhow::Result<()> {
    let path = dir.join(SETTINGS_PATH);
    let mut settings: serde_json::Value = if path.exists() {
        match serde_json::from_str(&std::fs::read_to_string(&path)?) {
            Ok(value) => value,
            Err(e) => {
                println!(
                    "kept {} (not valid JSON: {e}); add {} to permissions.deny by hand",
                    path.display(),
                    DENIED.join(" and ")
                );
                return Ok(());
            }
        }
    } else {
        serde_json::json!({})
    };
    let deny = settings
        .as_object_mut()
        .map(|o| {
            o.entry("permissions")
                .or_insert_with(|| serde_json::json!({}))
        })
        .and_then(|p| p.as_object_mut())
        .map(|p| p.entry("deny").or_insert_with(|| serde_json::json!([])))
        .and_then(|d| d.as_array_mut());
    let Some(deny) = deny else {
        println!(
            "kept {} (permissions.deny is not a list); add {} to it by hand",
            path.display(),
            DENIED.join(" and ")
        );
        return Ok(());
    };
    let missing: Vec<&str> = DENIED
        .iter()
        .copied()
        .filter(|rule| !deny.iter().any(|r| r == rule))
        .collect();
    if missing.is_empty() {
        println!("kept {}", path.display());
        return Ok(());
    }
    deny.extend(missing.iter().map(|r| serde_json::json!(r)));
    std::fs::create_dir_all(path.parent().expect("settings path has a parent"))?;
    std::fs::write(&path, serde_json::to_string_pretty(&settings)? + "\n")?;
    println!(
        "updated {} (denied: {})",
        path.display(),
        missing.join(", ")
    );
    Ok(())
}

fn init(dir: &Path) -> anyhow::Result<()> {
    let config = dir.join(config::FILE_NAME);
    if config.exists() {
        println!("kept {}", config.display());
    } else {
        std::fs::write(&config, config::TEMPLATE)?;
        println!("created {}", config.display());
    }
    std::fs::create_dir_all(dir.join("sources"))?;
    let skill = dir.join(SKILL_PATH);
    std::fs::create_dir_all(skill.parent().expect("skill path has a parent"))?;
    std::fs::write(&skill, SKILL)?;
    println!("wrote {}", skill.display());
    for (path, definition) in SUBAGENTS {
        let path = dir.join(path);
        std::fs::create_dir_all(path.parent().expect("agent path has a parent"))?;
        std::fs::write(&path, definition)?;
        println!("wrote {}", path.display());
    }
    deny_rules(dir)?;
    println!(
        "next: put the sources in sources/, set target, reader and [[sources]] in {}, then \
         run `exegenius next` and follow what it prints until it prints `done`",
        config::FILE_NAME
    );
    Ok(())
}

fn require_graph(project: &Project) -> anyhow::Result<Graph> {
    project
        .load_graph()?
        .context("no concept graph yet: run `exegenius next` and do the extract task")
}

fn print_plan(graph: &Graph, plan: &Plan) {
    for (i, id) in plan.order.iter().enumerate() {
        let name = graph.get(id).map(|c| c.name.as_str()).unwrap_or("");
        println!("{:>3}. {id}  {name}", i + 1);
    }
    let known = graph.concepts.iter().filter(|c| c.known).count();
    println!(
        "{} pages to the target; {known} concepts assumed known",
        plan.order.len()
    );
}

fn print_build(project: &Project, report: &BuildReport) {
    for warning in &report.warnings {
        println!("warning: {warning}");
    }
    for path in &report.removed {
        println!("removed stale {}", path.display());
    }
    if !report.missing.is_empty() {
        println!(
            "{} planned pages have no body yet: {}",
            report.missing.len(),
            report.missing.join(", ")
        );
    }
    let mut extras = Vec::new();
    if let Some(n) = report.parts {
        extras.push(format!("{n} parts"));
    }
    if report.glossary > 0 {
        extras.push(format!("{} in the glossary", report.glossary));
    }
    if report.symbols > 0 {
        extras.push(format!("{} symbols on the notation page", report.symbols));
    }
    let extras = if extras.is_empty() {
        String::new()
    } else {
        format!(" ({})", extras.join(", "))
    };
    if let Some(path) = &report.report {
        println!("simulated readers: see {}", path.display());
    }
    println!(
        "wrote {} pages{extras} to {} and {}; view with `zensical serve`",
        report.pages.len(),
        project.docs_dir().display(),
        project.zensical_path().display()
    );
}

fn next(project: &Project) -> anyhow::Result<()> {
    match task::next(project)? {
        Next::Done(report) => {
            println!("done");
            print_build(project, &report);
            if !project.root.join("vercel.json").exists() {
                println!(
                    "to publish on Vercel: run `exegenius init --vercel`, push this folder to \
                     a GitHub repository, and import it at https://vercel.com/new"
                );
            }
        }
        Next::Background { profile, known } => {
            println!(
                "The reader, as every later task will be told:\n\n{profile}\n\nKnown to the \
                 reader, so given no page: {}\n\nIf a person is following this run, show them \
                 this and ask whether it describes the reader they mean. If they correct it, run \
                 `exegenius background --redo \"<their correction>\"`, then `exegenius next`. \
                 If no one is following the run (for example under `claude -p`), run `exegenius \
                 next` now.",
                known.join(", ")
            );
        }
        Next::Tasks { round, tasks } => {
            let mut labels: Vec<&str> = Vec::new();
            for t in &tasks {
                if !labels.contains(&t.label()) {
                    labels.push(t.label());
                }
            }
            let phase = match tasks[0].kind {
                Kind::Audit { .. } => format!("audit round {round}"),
                _ => labels.join(" and "),
            };
            println!("{} pending task(s): {phase}\n", tasks.len());
            for t in &tasks {
                let answer = project.root.join(t.answer());
                let answer = match t.fields() {
                    Some(fields) => format!(
                        "{} and {}",
                        answer.display(),
                        project.root.join(fields).display()
                    ),
                    None => answer.display().to_string(),
                };
                let agent = subagent(&t.kind);
                println!(
                    "{id}\n  subagent: {agent}\n  prompt: Do the exegenius task in {file}: \
                     follow its instructions, write the answer to {answer}, then run \
                     `exegenius submit {id}` in {root}. If submit rejects the answer, fix the \
                     answer and submit again until it is accepted. Reply with only \
                     \"accepted\" or the last error.\n",
                    id = t.id,
                    file = project.root.join(t.file()).display(),
                    root = project.root.display(),
                );
            }
            let parallel = if tasks.len() > MAX_SUBAGENTS {
                format!(
                    "The tasks are independent: give each prompt to its own subagent of the type \
                     named (written to .claude/agents/ by `exegenius init`). Claude Code runs at \
                     most {MAX_SUBAGENTS} subagents at once by default, so start \
                     {MAX_SUBAGENTS} (one Agent tool call per task, in a single message) and \
                     start another each time one finishes. "
                )
            } else if tasks.len() > 1 {
                "The tasks are independent: give each prompt to its own subagent of the type \
                 named (written to .claude/agents/ by `exegenius init`), all at once (one Agent \
                 tool call per task, all in a single message). "
                    .to_string()
            } else {
                "Give the prompt to a subagent of the type named, or do the task yourself. "
                    .to_string()
            };
            let fresh = match tasks[0].kind {
                Kind::Audit { .. } => {
                    "Each audit task needs a fresh subagent that has seen nothing else. Do not \
                     do audits yourself: you have read the sources and the pages, so you cannot \
                     stand in for the reader. "
                }
                Kind::Read { .. } | Kind::Baseline { .. } => {
                    "These are simulated readers: each needs a fresh exegenius-reader subagent, \
                     which runs on the smallest model, since a less capable reader is a closer \
                     stand-in for someone new to the subject. Never do them yourself. "
                }
                Kind::Grade { .. } => {
                    "Each grade task needs a fresh subagent that has seen nothing else. "
                }
                _ => "",
            };
            println!("{parallel}{fresh}When all are accepted, run `exegenius next` again.");
        }
    }
    Ok(())
}

/// Where a planning stage's quotes fall in each source, as `exegenius status` shows it.
fn print_coverage(what: &str, coverage: &[Coverage]) {
    println!(
        "source coverage of {what} (where the quotes fall, not whether the proof is covered):"
    );
    for c in coverage {
        for line in c.summary(5) {
            println!("  {line}");
        }
    }
}

fn status(project: &Project) -> anyhow::Result<()> {
    let state = task::load(project)?;
    let Some(graph) = project.load_graph()? else {
        println!(
            "no concept graph yet; pending tasks: {}",
            state.pending.len()
        );
        return Ok(());
    };
    let plan = plan(&graph, &project.sources)?;
    let known = graph.concepts.iter().filter(|c| c.known).count();
    let mut written = 0;
    for id in &plan.order {
        if project.page_body(id)?.is_some() {
            written += 1;
        }
    }
    let target = graph
        .get(&graph.target)
        .map(|c| c.name.as_str())
        .unwrap_or("");
    println!("target: {} ({target})", graph.target);
    if let Some(pilot) = &graph.pilot {
        let name = graph.get(pilot).map_or("", |c| c.name.as_str());
        println!("pilot: {pilot} ({name}), the part of the target this run explains");
    }
    if let Some(background) = &graph.background {
        println!("reader, for these sources: {}", background.summary);
    }
    println!(
        "concepts: {} ({known} known to the reader); pages: {written} written of {} planned",
        graph.concepts.len(),
        plan.order.len()
    );
    print_coverage(
        "the concept graph's quotes",
        &coverage(&project.sources, &concept_owners(&graph.concepts)),
    );
    if let Some(argument) = exegenius_core::argument::load(project)? {
        print_coverage(
            "the argument's quotes",
            &coverage(
                &project.sources,
                &argument_owners(&argument.root, Some(&graph)),
            ),
        );
    }
    for round in exegenius_core::convergence::summarise(&load_rounds(project)?) {
        let per_page = |n: usize| n as f64 / round.pages.max(1) as f64;
        println!(
            "audit round {}: {} gaps over {} pages (per page: term {:.1}, step {:.1}, symbol \
             {:.1})",
            round.round,
            round.gaps.total(),
            round.pages,
            per_page(round.gaps.term),
            per_page(round.gaps.step),
            per_page(round.gaps.symbol)
        );
        if round.repeated > 0 {
            let change = |before: usize, after: usize, new: usize| {
                format!("{before} -> {after} ({new} new)")
            };
            println!(
                "  on the {} pages also audited in round {}: term {}, step {}, symbol {}",
                round.repeated,
                round.round - 1,
                change(round.before.term, round.after.term, round.new.term),
                change(round.before.step, round.after.step, round.new.step),
                change(round.before.symbol, round.after.symbol, round.new.symbol)
            );
        }
    }
    let cut = load_rounds(project)?
        .iter()
        .flat_map(|r| &r.pages)
        .flat_map(|p| &p.gaps)
        .filter(|g| g.action == exegenius_core::audit::CITE_OR_CUT)
        .count();
    if cut > 0 {
        println!(
            "steps inside reasoning a rewrite added for an earlier step, sent back to be cited \
             or cut: {cut}"
        );
    }
    if let Some(reason) = &state.stopped {
        println!("audit rounds stopped early: {reason}");
    }
    if !project.config.stop_early {
        for s in exegenius_core::convergence::summarise(&load_rounds(project)?) {
            if let Some(reason) = exegenius_core::convergence::stalled(&s) {
                println!(
                    "`stop_early` is off; with it on, the audit rounds would have stopped {reason}"
                );
            }
        }
    }
    let last = exegenius_core::convergence::summarise(&load_rounds(project)?)
        .last()
        .map_or(0, |s| s.gaps.total());
    if state.done && last > 0 {
        println!(
            "the last audit round's {last} gaps are left on the pages, since no audit would \
             check rewrites for them; their notes were dropped when the rounds ended, so \
             raising `rounds` now does not fix them: set `rounds` higher before a run to have \
             them fixed"
        );
    }
    println!(
        "candidate concepts: {}; pages queued for rewriting: {}",
        graph.candidates.len(),
        graph.rewrite.len()
    );
    let outline = outline::current(project, &plan)?;
    match &outline {
        Some(outline) => println!(
            "outline: {} parts, {} pages in the glossary",
            outline.parts.len(),
            outline.glossary.len()
        ),
        None => println!("outline: not yet (handed out once the pages are final)"),
    }
    let edges: usize = plan
        .order
        .iter()
        .filter_map(|id| graph.get(id))
        .map(|c| c.requires.iter().filter(|r| plan.contains(r)).count())
        .sum();
    let related: usize = graph.relations.values().map(Vec::len).sum();
    println!("relations: {related} of {edges} prerequisite edges between pages");
    let bound: usize = graph.bindings.values().map(Vec::len).sum();
    let overloads = exegenius_core::notation::load(project)?;
    let unjudged = exegenius_core::notation::unjudged(project, &graph, &plan)?.len();
    let waiting = if overloads.rounds >= exegenius_core::notation::ROUNDS {
        "left unjudged after the last overloads round"
    } else {
        "awaiting a verdict"
    };
    println!(
        "symbols: {bound} bound on pages; letters with two meanings: {} confirmed, {unjudged} \
         {waiting}",
        overloads.judged.iter().filter(|j| !j.same).count()
    );
    let support = exegenius_core::support::load(project)?;
    if !support.judged.is_empty() {
        let count = |v: exegenius_core::support::Verdict| {
            support.judged.iter().filter(|j| j.label == v).count()
        };
        use exegenius_core::support::Verdict;
        println!(
            "citation support: {} marks judged: {} supported, {} partial, {} unsupported, {} \
             contradicted",
            support.judged.len(),
            count(Verdict::Supported),
            count(Verdict::Partial),
            count(Verdict::Unsupported),
            count(Verdict::Contradicted)
        );
        let (both, agree) = support.agreement();
        if both > 0 {
            println!("  a second judge agreed on {agree} of {both} marks");
        }
    }
    let unlinked: Vec<&str> = project
        .sources
        .iter()
        .filter(|s| s.url.is_none())
        .map(|s| s.id.as_str())
        .collect();
    if !unlinked.is_empty() {
        println!(
            "sources with no url (a DOI, arXiv or stable URL lets readers check them): {}",
            unlinked.join(", ")
        );
    }
    let simulation = exegenius_core::simulate::load(project)?;
    if !simulation.goals.is_empty() {
        let held_out = simulation.goals.iter().filter(|g| g.held_out).count();
        println!(
            "simulated readers: {} goals, {held_out} of them held out",
            simulation.goals.len()
        );
        let mut rounds: Vec<u32> = simulation.runs.iter().map(|r| r.round).collect();
        rounds.dedup();
        for round in rounds {
            let runs = simulation.round_runs(round).count();
            let covered = |held_out| {
                exegenius_core::report::round_coverage(&simulation, round, held_out)
                    .map_or("ungraded".to_string(), |c| format!("{:.0}%", 100.0 * c))
            };
            println!(
                "  round {round}: {runs} runs; key points covered: {} on repair goals, {} on \
                 held-out goals",
                covered(false),
                covered(true)
            );
        }
        if let Some(c) = exegenius_core::report::baseline_coverage(&simulation) {
            println!("  baseline (no wiki): {:.0}% of key points", 100.0 * c);
        }
    }
    let events = runlog::load(project);
    if !events.is_empty() {
        println!("run log ({}):", project.log_path().display());
        for k in runlog::summarise(&events) {
            println!(
                "  {}: {} handed out, {} accepted ({} at the first submit), {} rejected \
                 submits{}{}",
                k.kind,
                k.issued,
                k.accepted,
                k.first_time,
                k.rejected,
                if k.coverage > 0 {
                    format!(" ({} for coverage)", k.coverage)
                } else {
                    String::new()
                },
                if k.errors > 0 {
                    format!(", {} submits not checked", k.errors)
                } else {
                    String::new()
                }
            );
        }
    }
    let rounds = project.config.simulation.rounds;
    let ablated = project.config.simulation.ablations == 0 || simulation.ablated;
    let simulated = rounds == 0 || (simulation.round > rounds && ablated);
    let issued = state.next_id - 1;
    let tasks = format!(
        "tasks handed out: {issued} ({} accepted, {} pending)",
        issued as usize - state.pending.len(),
        state.pending.len()
    );
    if state.done {
        println!("{tasks}; audit rounds over");
    } else {
        println!(
            "{tasks}; audit round {} of at most {}",
            state.round, project.config.rounds
        );
    }
    if !state.pending.is_empty() {
        let ids: Vec<&str> = state.pending.iter().map(|t| t.id.as_str()).collect();
        println!("pending tasks: {}", ids.join(", "));
    } else if state.done
        && outline.is_some()
        && (unjudged == 0 || overloads.rounds >= exegenius_core::notation::ROUNDS)
        && simulated
        && graph.rewrite.is_empty()
    {
        println!("state: done (docs/ and zensical.toml are built)");
    } else {
        println!("state: in progress; run `exegenius next`");
    }
    Ok(())
}

fn run(cli: Cli) -> anyhow::Result<()> {
    let Some(command) = cli.command else {
        println!("{AGENTS}");
        return Ok(());
    };
    match command {
        Command::Init { vercel: false } => return init(&cli.dir),
        Command::Init { vercel: true } => {
            init(&cli.dir)?;
            return vercel(&cli.dir);
        }
        Command::Next if !cli.dir.join(config::FILE_NAME).exists() => {
            println!("{SETUP}");
            return Ok(());
        }
        _ => {}
    }
    let project = Project::load(&cli.dir)
        .context("fix exegenius.toml: target, reader.knows, and [[sources]] paths to text files")?;
    // The template values matter only before the extract task, which bakes them in.
    if let Command::Next = command
        && !project.graph_path().exists()
    {
        let mut template = Vec::new();
        if project.config.target == config::TEMPLATE_TARGET {
            template.push("target");
        }
        if project.config.reader.knows == config::TEMPLATE_KNOWS {
            template.push("reader.knows");
        }
        if !template.is_empty() {
            println!(
                "exegenius.toml still has the template {}: set target (what the wiki should \
                 explain) and reader.knows (what its reader already knows) for this material, \
                 asking the user if they said nothing about the reader, then run \
                 `exegenius next` again.",
                template.join(" and ")
            );
            return Ok(());
        }
    }
    match command {
        Command::Init { .. } => unreachable!("handled above"),
        Command::Next => next(&project)?,
        Command::Submit { id } => {
            let warnings = task::submit(&project, &id)?;
            println!("accepted {id}");
            for warning in warnings {
                println!("warning: {warning}");
            }
        }
        Command::Read { id, page } => print!("{}", task::read_page(&project, &id, &page)?),
        Command::Search { id, words } => {
            println!("{}", task::search_pages(&project, &id, &words.join(" "))?)
        }
        Command::Background { redo } => {
            task::redo_background(&project, &redo)?;
            println!("the background task will be given again; run `exegenius next`");
        }
        Command::Status => status(&project)?,
        Command::Plan => {
            let graph = require_graph(&project)?;
            print_plan(&graph, &plan(&graph, &project.sources)?);
        }
        Command::Build => {
            let graph = require_graph(&project)?;
            let plan = plan(&graph, &project.sources)?;
            print_build(&project, &build(&project, &graph, &plan)?);
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planning_tasks_go_to_the_planner() {
        for kind in [
            Kind::Extract,
            Kind::Background,
            Kind::Argument,
            Kind::Extend,
        ] {
            assert_eq!(subagent(&kind), "exegenius-planner", "{kind:?}");
        }
        let page = || "p".to_string();
        for kind in [
            Kind::Write { page: page() },
            Kind::Audit {
                page: page(),
                input: "i".into(),
            },
            Kind::Outline,
            Kind::Overloads,
            Kind::Goals { held_out: false },
        ] {
            assert_eq!(subagent(&kind), "exegenius-task", "{kind:?}");
        }
        let reader = Kind::Baseline {
            goal: 0,
            persona: "reader".into(),
        };
        assert_eq!(subagent(&reader), "exegenius-reader");
    }

    #[test]
    fn init_denies_stopping_processes_by_pattern_and_keeps_existing_settings() {
        let dir = std::env::temp_dir().join(format!("exegenius-deny-{}", std::process::id()));
        let settings = dir.join(SETTINGS_PATH);
        std::fs::create_dir_all(settings.parent().unwrap()).unwrap();
        std::fs::write(
            &settings,
            r#"{"model": "x", "permissions": {"deny": ["Bash(rm -rf *)", "Bash(killall *)"]}}"#,
        )
        .unwrap();
        init(&dir).unwrap();
        init(&dir).unwrap();
        let value: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&settings).unwrap()).unwrap();
        assert_eq!(value["model"], "x");
        assert_eq!(
            value["permissions"]["deny"],
            serde_json::json!(["Bash(rm -rf *)", "Bash(killall *)", "Bash(pkill *)"])
        );
        // A file that is not JSON is left as it is.
        std::fs::write(&settings, "not json").unwrap();
        init(&dir).unwrap();
        assert_eq!(std::fs::read_to_string(&settings).unwrap(), "not json");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn init_writes_the_planner_on_sonnet() {
        let dir = std::env::temp_dir().join(format!("exegenius-init-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        init(&dir).unwrap();
        let read =
            |name: &str| std::fs::read_to_string(dir.join(".claude/agents").join(name)).unwrap();
        let planner = read("exegenius-planner.md");
        assert!(planner.contains("name: exegenius-planner\n"), "{planner}");
        assert!(planner.contains("\nmodel: sonnet\n"), "{planner}");
        assert!(read("exegenius-task.md").contains("\nmodel: inherit\n"));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}

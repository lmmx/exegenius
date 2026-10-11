use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::{Parser, Subcommand};
use exegenius_core::audit::load_rounds;
use exegenius_core::build::{BuildReport, build};
use exegenius_core::config;
use exegenius_core::graph::Graph;
use exegenius_core::outline;
use exegenius_core::plan::{Plan, plan};
use exegenius_core::project::Project;
use exegenius_core::task::{self, Kind, Next};

const SKILL: &str = include_str!("skill.md");
const SKILL_PATH: &str = ".claude/skills/exegenius/SKILL.md";

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
                println!(
                    "{id}\n  prompt: Do the exegenius task in {file}: follow its instructions, \
                     write the answer to {answer}, then run `exegenius submit {id}` in {root}. \
                     If submit rejects the answer, fix the answer and submit again until it is \
                     accepted. Reply with only \"accepted\" or the last error.\n",
                    id = t.id,
                    file = project.root.join(t.file()).display(),
                    root = project.root.display(),
                );
            }
            let parallel = if tasks.len() > 1 {
                "The tasks are independent: give each prompt to its own subagent, all at once \
                 (in Claude Code: one Agent tool call per task, all in a single message). "
            } else {
                "Give the prompt to a subagent, or do the task yourself. "
            };
            let fresh = match tasks[0].kind {
                Kind::Audit { .. } => {
                    "Each audit task needs a fresh subagent that has seen nothing else. Do not \
                     do audits yourself: you have read the sources and the pages, so you cannot \
                     stand in for the reader. "
                }
                Kind::Read { .. } | Kind::Baseline { .. } => {
                    "These are simulated readers: each needs a fresh subagent that has seen \
                     nothing else, on the smallest model available (in Claude Code: model \
                     haiku), since a less capable reader is a closer stand-in for someone new \
                     to the subject. Never do them yourself. "
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
    println!(
        "concepts: {} ({known} known to the reader); pages: {written} written of {} planned",
        graph.concepts.len(),
        plan.order.len()
    );
    for round in load_rounds(project)? {
        println!(
            "audit round {}: {} gaps over {} pages",
            round.round,
            round.gap_count(),
            round.pages.len()
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
    let judged = exegenius_core::notation::load(project)?.judged;
    let unjudged = exegenius_core::notation::unjudged(project, &graph, &plan)?.len();
    println!(
        "symbols: {bound} bound on pages; letters with two meanings: {} confirmed, {unjudged} \
         awaiting a verdict",
        judged.iter().filter(|j| !j.same).count()
    );
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
    let rounds = project.config.simulation.rounds;
    let ablated = project.config.simulation.ablations == 0 || simulation.ablated;
    let simulated = rounds == 0 || (simulation.round > rounds && ablated);
    if !state.pending.is_empty() {
        let ids: Vec<&str> = state.pending.iter().map(|t| t.id.as_str()).collect();
        println!("pending tasks: {}", ids.join(", "));
    } else if state.done
        && outline.is_some()
        && unjudged == 0
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

use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::{Parser, Subcommand};
use exegenius_core::audit::load_rounds;
use exegenius_core::build::{BuildReport, build};
use exegenius_core::config;
use exegenius_core::graph::Graph;
use exegenius_core::plan::{Plan, plan};
use exegenius_core::project::Project;
use exegenius_core::task::{self, Kind, Next};

const SKILL: &str = include_str!("skill.md");
const SKILL_PATH: &str = ".claude/skills/exegenius/SKILL.md";

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
2. Put the source material in sources/ as text or markdown, moving or converting the files
   already in this folder.
3. Edit exegenius.toml: `target` (what the wiki should explain), `reader.knows` (what the
   reader already knows) and one [[sources]] entry per source file. If the user did not say,
   infer the target and reader from the material.";

#[derive(Subcommand)]
enum Command {
    /// Create exegenius.toml and sources/ if missing, and (re)write the Claude Code skill
    Init,
    /// Print the current batch of tasks, or build the docs when nothing is left
    Next,
    /// Check and apply the answer to a task
    Submit {
        /// Task id, as printed by `next`
        id: String,
    },
    /// Show progress
    Status,
    /// Print the pages needed to reach the target, in reading order
    Plan,
    /// Write docs/ and zensical.toml from the pages written so far
    Build,
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
    println!(
        "wrote {} pages to {} and {}; view with `zensical serve`",
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
        }
        Next::Tasks { round, tasks } => {
            let label = tasks[0].label();
            let phase = match tasks[0].kind {
                Kind::Audit { .. } => format!("audit round {round}"),
                _ => label.to_string(),
            };
            println!("{} pending task(s): {phase}\n", tasks.len());
            for t in &tasks {
                println!(
                    "{id}\n  prompt: Do the exegenius task in {file}: follow its instructions, \
                     write the answer to {answer}, then run `exegenius submit {id}` in {root}. \
                     If submit rejects the answer, fix the answer file and submit again until \
                     it is accepted. Reply with only \"accepted\" or the last error.\n",
                    id = t.id,
                    file = project.root.join(t.file()).display(),
                    answer = project.root.join(t.answer()).display(),
                    root = project.root.display(),
                );
            }
            let parallel = if tasks.len() > 1 {
                "The tasks are independent: give each prompt to its own subagent, in parallel. "
            } else {
                "Give the prompt to a subagent, or do the task yourself. "
            };
            let fresh = if matches!(tasks[0].kind, Kind::Audit { .. }) {
                "Each audit task needs a fresh subagent that has seen nothing else. "
            } else {
                ""
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
    if state.done {
        println!("state: done (docs/ and zensical.toml are built)");
    } else if !state.pending.is_empty() {
        let ids: Vec<&str> = state.pending.iter().map(|t| t.id.as_str()).collect();
        println!("pending tasks: {}", ids.join(", "));
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
        Command::Init => return init(&cli.dir),
        Command::Next if !cli.dir.join(config::FILE_NAME).exists() => {
            println!("{SETUP}");
            return Ok(());
        }
        _ => {}
    }
    let project = Project::load(&cli.dir)
        .context("fix exegenius.toml: target, reader.knows, and [[sources]] paths to text files")?;
    if let Command::Next = command
        && project.config.target == config::TEMPLATE_TARGET
    {
        println!(
            "exegenius.toml still has the template target: set target, reader.knows and \
                  [[sources]] for this material, then run `exegenius next` again."
        );
        return Ok(());
    }
    match command {
        Command::Init => unreachable!("handled above"),
        Command::Next => next(&project)?,
        Command::Submit { id } => {
            let warnings = task::submit(&project, &id)?;
            println!("accepted {id}");
            for warning in warnings {
                println!("warning: {warning}");
            }
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

mod anthropic;

use std::path::{Path, PathBuf};

use anyhow::{Context, bail};
use clap::{Parser, Subcommand};
use exegenius_core::audit::audit_pages;
use exegenius_core::build::build;
use exegenius_core::config;
use exegenius_core::extract::{extend, extract};
use exegenius_core::graph::Graph;
use exegenius_core::model::Model;
use exegenius_core::plan::{Plan, plan};
use exegenius_core::project::Project;
use exegenius_core::write::write_pages;

use crate::anthropic::Anthropic;

/// Turn source documents into a wiki of linked explanatory pages for a stated reader.
#[derive(Parser)]
#[command(name = "exegenius", version)]
struct Cli {
    /// Project directory, containing exegenius.toml
    #[arg(short = 'C', long = "dir", default_value = ".", global = true)]
    dir: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create exegenius.toml and sources/ in the project directory
    Init,
    /// Decompose the sources into a concept graph, or resolve candidate concepts from audits
    Extract {
        /// Discard the existing graph and decompose the sources again
        #[arg(long)]
        fresh: bool,
    },
    /// Print the pages needed to reach the target, in reading order
    Plan,
    /// Write the pages that are missing or queued for rewriting
    Write,
    /// Audit every page against what the reader knows at that point
    Audit,
    /// Write docs/ and zensical.toml
    Build,
    /// Extract, then write and audit for up to N rounds, apply the last fixes, and build
    Run {
        /// Maximum number of audit rounds
        #[arg(long, default_value_t = 3)]
        rounds: usize,
    },
}

fn log(message: &str) {
    eprintln!("{message}");
}

fn warn_all(warnings: &[String]) {
    for warning in warnings {
        eprintln!("warning: {warning}");
    }
}

fn model(project: &Project) -> anyhow::Result<Anthropic> {
    Anthropic::from_env(project.config.model.clone(), project.cache_dir())
}

fn require_graph(project: &Project) -> anyhow::Result<Graph> {
    project
        .load_graph()?
        .context("no concept graph yet: run `exegenius extract` first")
}

/// Extract a new graph if there is none (or `fresh`), otherwise resolve any candidates.
fn ensure_graph(project: &Project, model: &dyn Model, fresh: bool) -> anyhow::Result<Graph> {
    let mut graph = match project.load_graph()? {
        Some(graph) if !fresh => graph,
        _ => {
            let (graph, warnings) = extract(project, model, &mut log)?;
            warn_all(&warnings);
            project.save_graph(&graph)?;
            let unknown = graph.concepts.iter().filter(|c| !c.known).count();
            log(&format!(
                "extract: {} concepts ({unknown} not known to the reader), target `{}`",
                graph.concepts.len(),
                graph.target
            ));
            return Ok(graph);
        }
    };
    if !graph.candidates.is_empty() {
        let warnings = extend(project, &mut graph, model, &mut log)?;
        warn_all(&warnings);
        project.save_graph(&graph)?;
    }
    Ok(graph)
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

fn do_build(project: &Project, graph: &Graph, plan: &Plan) -> anyhow::Result<()> {
    let report = build(project, graph, plan)?;
    warn_all(&report.warnings);
    for path in &report.removed {
        log(&format!("build: removed stale {}", path.display()));
    }
    if !report.missing.is_empty() {
        log(&format!(
            "build: {} planned pages have no body yet (run `exegenius write`): {}",
            report.missing.len(),
            report.missing.join(", ")
        ));
    }
    log(&format!(
        "build: wrote {} pages to {} and {}; view with `zensical serve`",
        report.pages.len(),
        project.docs_dir().display(),
        project.zensical_path().display()
    ));
    Ok(())
}

fn init(dir: &Path) -> anyhow::Result<()> {
    let path = dir.join(config::FILE_NAME);
    if path.exists() {
        bail!("{} already exists", path.display());
    }
    std::fs::create_dir_all(dir.join("sources"))?;
    std::fs::write(&path, config::TEMPLATE)?;
    log(&format!(
        "created {} and {}/; add source files and edit the target and reader",
        path.display(),
        dir.join("sources").display()
    ));
    Ok(())
}

fn run(cli: Cli) -> anyhow::Result<()> {
    if let Command::Init = cli.command {
        return init(&cli.dir);
    }
    let project = Project::load(&cli.dir)?;
    match cli.command {
        Command::Init => unreachable!("handled above"),
        Command::Extract { fresh } => {
            let model = model(&project)?;
            let graph = ensure_graph(&project, &model, fresh)?;
            print_plan(&graph, &plan(&graph, &project.sources)?);
        }
        Command::Plan => {
            let graph = require_graph(&project)?;
            print_plan(&graph, &plan(&graph, &project.sources)?);
        }
        Command::Write => {
            let model = model(&project)?;
            let mut graph = require_graph(&project)?;
            let plan = plan(&graph, &project.sources)?;
            let report = write_pages(&project, &mut graph, &plan, &model, &mut log)?;
            warn_all(&report.warnings);
            log(&format!("write: {} pages written", report.written.len()));
        }
        Command::Audit => {
            let model = model(&project)?;
            let mut graph = require_graph(&project)?;
            let plan = plan(&graph, &project.sources)?;
            let round = audit_pages(&project, &mut graph, &plan, &model, &mut log)?;
            log(&format!(
                "audit round {}: {} gaps; {} candidate concepts, {} pages queued for rewriting",
                round.round,
                round.gap_count(),
                graph.candidates.len(),
                graph.rewrite.len()
            ));
        }
        Command::Build => {
            let graph = require_graph(&project)?;
            let plan = plan(&graph, &project.sources)?;
            do_build(&project, &graph, &plan)?;
        }
        Command::Run { rounds } => {
            let model = model(&project)?;
            let mut graph = ensure_graph(&project, &model, false)?;
            let mut clean = false;
            for _ in 0..rounds {
                graph = ensure_graph(&project, &model, false)?;
                let plan = plan(&graph, &project.sources)?;
                let report = write_pages(&project, &mut graph, &plan, &model, &mut log)?;
                warn_all(&report.warnings);
                let round = audit_pages(&project, &mut graph, &plan, &model, &mut log)?;
                log(&format!(
                    "audit round {}: {} gaps over {} pages",
                    round.round,
                    round.gap_count(),
                    round.pages.len()
                ));
                if round.gap_count() == 0 {
                    clean = true;
                    break;
                }
            }
            if !clean {
                graph = ensure_graph(&project, &model, false)?;
                let plan = plan(&graph, &project.sources)?;
                let report = write_pages(&project, &mut graph, &plan, &model, &mut log)?;
                warn_all(&report.warnings);
                if !report.written.is_empty() {
                    log(&format!(
                        "run: applied the last audit's fixes to {} pages; these are not yet \
                         audited (run `exegenius run` again to continue)",
                        report.written.len()
                    ));
                }
            }
            let plan = plan(&graph, &project.sources)?;
            do_build(&project, &graph, &plan)?;
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

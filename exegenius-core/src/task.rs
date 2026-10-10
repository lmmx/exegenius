//! The task loop: [`next`] decides what to hand out, [`submit`] checks and applies answers.
//!
//! Tasks are handed out in batches. Every task in a batch is independent of the others, so
//! an agent can work on them in parallel. A new batch is computed only once every task in
//! the current one has been submitted. Precedence, highest first:
//!
//! 1. extract (no graph yet)
//! 2. extend (audits left candidate concepts)
//! 3. write (pages with no body, or queued for rewriting)
//! 4. audit (pages whose audit input changed since they were last audited); a round ends
//!    when its last audit is submitted, and the loop stops after a round with no gaps or
//!    after `rounds` rounds
//!
//! When nothing is left, `next` hands out one outline task (parts and front page) if the
//! pages have no current outline, and then builds the docs. Writes queued by the last audit
//! round are still handed out (and not audited) before the outline.

use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::audit::{AUDIT_INSTRUCTIONS, AuditAnswer, apply_audit, audit_task, record};
use crate::build::{BuildReport, build};
use crate::error::{Error, Result, read, read_json, write, write_json};
use crate::extract::{
    EXTEND_INSTRUCTIONS, EXTRACT_INSTRUCTIONS, ExtendAnswer, ExtractAnswer, apply_extend,
    apply_extract, extend_task, extract_task,
};
use crate::graph::Graph;
use crate::outline::{OUTLINE_INSTRUCTIONS, OutlineAnswer, apply_outline, outline_task};
use crate::plan::{Plan, plan};
use crate::project::Project;
use crate::write::{WRITE_INSTRUCTIONS, apply_write, needs_write, write_task};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Kind {
    Extract,
    Extend,
    Write { page: String },
    Audit { page: String, input: String },
    Outline,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    #[serde(flatten)]
    pub kind: Kind,
}

impl Task {
    pub fn label(&self) -> &'static str {
        match self.kind {
            Kind::Extract => "extract",
            Kind::Extend => "extend",
            Kind::Write { .. } => "write",
            Kind::Audit { .. } => "audit",
            Kind::Outline => "outline",
        }
    }

    /// Task file, relative to the project directory.
    pub fn file(&self) -> PathBuf {
        PathBuf::from("exegenius/tasks").join(format!("{}.md", self.id))
    }

    /// Answer file, relative to the project directory.
    pub fn answer(&self) -> PathBuf {
        let ext = match self.kind {
            Kind::Write { .. } => "md",
            _ => "json",
        };
        PathBuf::from("exegenius/tasks").join(format!("{}.answer.{ext}", self.id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub next_id: u32,
    /// Current audit round, from 1.
    pub round: u32,
    /// Gaps found so far in the current round.
    pub round_gaps: usize,
    /// Tasks handed out and not yet submitted.
    pub pending: Vec<Task>,
    /// For each page, the hash of the audit input it was last audited with.
    pub audited: BTreeMap<String, String>,
    pub done: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            next_id: 1,
            round: 1,
            round_gaps: 0,
            pending: Vec::new(),
            audited: BTreeMap::new(),
            done: false,
        }
    }
}

fn load_state(project: &Project) -> Result<State> {
    let path = project.state_path();
    if path.exists() {
        read_json(&path)
    } else {
        Ok(State::default())
    }
}

/// Exclusive access to the project state, so parallel submits do not lose updates.
struct Lock(PathBuf);

impl Lock {
    fn acquire(project: &Project) -> Result<Self> {
        let path = project.lock_path();
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|source| Error::Io {
                path: dir.to_path_buf(),
                source,
            })?;
        }
        let start = Instant::now();
        loop {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(_) => return Ok(Self(path)),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    if start.elapsed() > Duration::from_secs(120) {
                        return Err(Error::Invalid(format!(
                            "{} has been held for two minutes; if no exegenius command is \
                             running, delete it",
                            path.display()
                        )));
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(source) => return Err(Error::Io { path, source }),
            }
        }
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// JSON schema for an answer type, for the task file.
pub fn schema_of<T: JsonSchema>() -> Value {
    let mut schema = serde_json::to_value(schemars::schema_for!(T)).unwrap_or(Value::Null);
    if let Value::Object(map) = &mut schema {
        map.remove("$schema");
    }
    schema
}

fn hash(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .take(12)
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn render(project: &Project, task: &Task, instructions: &str, input: &str) -> String {
    let answer = task.answer();
    let format = match task.kind {
        Kind::Extract => json_format::<ExtractAnswer>(),
        Kind::Extend => json_format::<ExtendAnswer>(),
        Kind::Audit { .. } => json_format::<AuditAnswer>(),
        Kind::Outline => json_format::<OutlineAnswer>(),
        Kind::Write { .. } => "The answer is the page body as markdown (not JSON).".to_string(),
    };
    format!(
        "# exegenius task {id}\n\nProject directory: `{root}` (paths below are relative to it)\n\n\
         {instructions}\n\n{input}\n## Answer\n\n{format}\n\nWrite the answer to `{answer}`, \
         then run `exegenius submit {id}` in the project directory. If submit rejects the \
         answer, fix the answer file and submit again.\n",
        id = task.id,
        root = project.root.display(),
        answer = answer.display(),
    )
}

fn json_format<T: JsonSchema>() -> String {
    let schema = serde_json::to_string_pretty(&schema_of::<T>()).unwrap_or_default();
    format!("The answer is one JSON object following this schema:\n\n```json\n{schema}\n```")
}

/// What `next` hands out.
#[derive(Debug)]
pub enum Next {
    Tasks { round: u32, tasks: Vec<Task> },
    Done(BuildReport),
}

/// Write the task files for a new batch, record it as pending, and return it.
fn issue(project: &Project, state: &mut State, items: Vec<(Kind, &str, String)>) -> Result<Next> {
    for (kind, instructions, input) in items {
        let label = match &kind {
            Kind::Extract => "extract".to_string(),
            Kind::Extend => "extend".to_string(),
            Kind::Write { page } => format!("write-{page}"),
            Kind::Audit { page, .. } => format!("audit-{page}"),
            Kind::Outline => "outline".to_string(),
        };
        let task = Task {
            id: format!("{:04}-{label}", state.next_id),
            kind,
        };
        state.next_id += 1;
        write(
            &project.root.join(task.file()),
            &render(project, &task, instructions, &input),
        )?;
        state.pending.push(task);
    }
    write_json(&project.state_path(), &*state)?;
    Ok(Next::Tasks {
        round: state.round,
        tasks: state.pending.clone(),
    })
}

/// Hand out the current batch of tasks, computing a new one if every task has been
/// submitted, or build the docs when nothing is left.
pub fn next(project: &Project) -> Result<Next> {
    let _lock = Lock::acquire(project)?;
    let mut state = load_state(project)?;
    if !state.pending.is_empty() {
        return Ok(Next::Tasks {
            round: state.round,
            tasks: state.pending,
        });
    }
    loop {
        let Some(graph) = project.load_graph()? else {
            let input = extract_task(project);
            return issue(
                project,
                &mut state,
                vec![(Kind::Extract, EXTRACT_INSTRUCTIONS, input)],
            );
        };
        if !graph.candidates.is_empty() {
            let input = extend_task(project, &graph);
            return issue(
                project,
                &mut state,
                vec![(Kind::Extend, EXTEND_INSTRUCTIONS, input)],
            );
        }
        let plan = plan(&graph, &project.sources)?;
        let writes = needs_write(project, &graph, &plan)?;
        if !writes.is_empty() {
            let mut items = Vec::new();
            for page in writes {
                let i = plan.order.iter().position(|p| *p == page).expect("planned");
                items.push((
                    Kind::Write { page },
                    WRITE_INSTRUCTIONS,
                    write_task(project, &graph, &plan, i)?,
                ));
            }
            return issue(project, &mut state, items);
        }
        if state.done {
            if crate::outline::current(project, &plan)?.is_none() {
                let input = outline_task(project, &graph, &plan);
                return issue(
                    project,
                    &mut state,
                    vec![(Kind::Outline, OUTLINE_INSTRUCTIONS, input)],
                );
            }
            write_json(&project.state_path(), &state)?;
            return Ok(Next::Done(build(project, &graph, &plan)?));
        }
        if state.round > project.config.rounds {
            state.done = true;
            continue;
        }
        let mut items = Vec::new();
        for (i, page) in plan.order.iter().enumerate() {
            let body = project.page_body(page)?.unwrap_or_default();
            let input = audit_task(project, &graph, &plan, i, &body);
            let input_hash = hash(&input);
            if state.audited.get(page) != Some(&input_hash) {
                items.push((
                    Kind::Audit {
                        page: page.clone(),
                        input: input_hash,
                    },
                    AUDIT_INSTRUCTIONS,
                    input,
                ));
            }
        }
        if !items.is_empty() {
            return issue(project, &mut state, items);
        }
        // Every page has been audited against its current input.
        state.done = true;
    }
}

fn parse<T: DeserializeOwned>(path: &Path, text: &str) -> Result<T> {
    serde_json::from_str(text).map_err(|e| {
        Error::Invalid(format!(
            "answer rejected: {} is not valid for this task: {e}",
            path.display()
        ))
    })
}

fn find_page(plan: &Plan, page: &str) -> Result<usize> {
    plan.order
        .iter()
        .position(|p| p == page)
        .ok_or_else(|| Error::Invalid(format!("page `{page}` is no longer in the plan")))
}

/// Check and apply the answer to a pending task. Returns warnings; a rejected answer is an
/// error listing what to fix, and the task stays pending.
pub fn submit(project: &Project, id: &str) -> Result<Vec<String>> {
    let _lock = Lock::acquire(project)?;
    let mut state = load_state(project)?;
    let Some(index) = state.pending.iter().position(|t| t.id == id) else {
        return Err(Error::Invalid(format!(
            "`{id}` is not a pending task (run `exegenius next` to see the pending tasks)"
        )));
    };
    let task = state.pending[index].clone();
    let path = project.root.join(task.answer());
    if !path.exists() {
        return Err(Error::Invalid(format!(
            "no answer yet: write it to {}",
            task.answer().display()
        )));
    }
    let text = read(&path)?;
    let graph = project.load_graph()?;
    let warnings = match &task.kind {
        Kind::Extract => {
            let (graph, warnings) = apply_extract(project, parse(&path, &text)?)?;
            project.save_graph(&graph)?;
            warnings
        }
        Kind::Extend => {
            let mut graph = require(graph)?;
            let warnings = apply_extend(project, &mut graph, parse(&path, &text)?)?;
            project.save_graph(&graph)?;
            warnings
        }
        Kind::Write { page } => {
            let mut graph = require(graph)?;
            let plan = plan(&graph, &project.sources)?;
            find_page(&plan, page)?;
            let warnings = apply_write(project, &mut graph, &plan, page, &text)?;
            project.save_graph(&graph)?;
            warnings
        }
        Kind::Audit { page, input } => {
            let mut graph = require(graph)?;
            let plan = plan(&graph, &project.sources)?;
            let i = find_page(&plan, page)?;
            let body = project.page_body(page)?.unwrap_or_default();
            let audit = apply_audit(&mut graph, &plan, i, &body, parse(&path, &text)?)?;
            state.round_gaps += audit.gaps.len();
            state.audited.insert(page.clone(), input.clone());
            record(project, state.round, audit)?;
            project.save_graph(&graph)?;
            Vec::new()
        }
        Kind::Outline => {
            let graph = require(graph)?;
            let plan = plan(&graph, &project.sources)?;
            apply_outline(project, &graph, &plan, parse(&path, &text)?)?;
            Vec::new()
        }
    };
    state.pending.remove(index);
    let audits_left = state
        .pending
        .iter()
        .any(|t| matches!(t.kind, Kind::Audit { .. }));
    if matches!(task.kind, Kind::Audit { .. }) && !audits_left {
        // The round is over: finish if it found nothing, else start the next one.
        if state.round_gaps == 0 {
            state.done = true;
        } else {
            state.round += 1;
            state.round_gaps = 0;
        }
    }
    write_json(&project.state_path(), &state)?;
    Ok(warnings)
}

fn require(graph: Option<Graph>) -> Result<Graph> {
    graph.ok_or_else(|| Error::Invalid("no concept graph yet".into()))
}

pub fn load(project: &Project) -> Result<State> {
    load_state(project)
}

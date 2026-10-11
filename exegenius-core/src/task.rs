//! The task loop: [`next`] decides what to hand out, [`submit`] checks and applies answers.
//!
//! Tasks are handed out in batches. Every task in a batch is independent of the others, so
//! an agent can work on them in parallel. A new batch is computed only once every task in
//! the current one has been submitted. Precedence, highest first:
//!
//! 1. extract (no graph yet)
//! 2. extend (audits left candidate concepts)
//! 3. argument (no plan of the argument for the current pages)
//! 4. write (pages with no body, or queued for rewriting)
//! 5. audit (pages whose audit input changed since they were last audited); a round ends
//!    when its last audit is submitted, and the loop stops after a round with no gaps or
//!    after `rounds` rounds
//!
//! When nothing is left, `next` hands out the outline task (parts, glossary and front page)
//! if the pages have no current outline, and the overloads task if pages rebind letters
//! with no verdict yet, in one batch, and then builds the docs. Writes queued by the last
//! audit round, or by overload verdicts, are still handed out (and not audited) before the
//! outline.

use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::argument::{ARGUMENT_INSTRUCTIONS, ArgumentAnswer, apply_argument, argument_task};
use crate::audit::{AUDIT_INSTRUCTIONS, AuditAnswer, apply_audit, audit_task, record};
use crate::build::{BuildReport, build};
use crate::error::{Error, Result, read, read_json, write, write_json};
use crate::extract::{
    EXTEND_INSTRUCTIONS, EXTRACT_INSTRUCTIONS, ExtendAnswer, ExtractAnswer, apply_extend,
    apply_extract, extend_task, extract_task,
};
use crate::grade::{GRADE_INSTRUCTIONS, GradeAnswer, apply_grade, grade_task, repair, to_grade};
use crate::graph::Graph;
use crate::notation::{OVERLOADS_INSTRUCTIONS, OverloadsAnswer, apply_overloads, overloads_task};
use crate::outline::{OUTLINE_INSTRUCTIONS, OutlineAnswer, apply_outline, outline_task};
use crate::plan::{Plan, plan};
use crate::project::Project;
use crate::simulate::{
    self, BASELINE_INSTRUCTIONS, Baseline, BaselineAnswer, GOALS_INSTRUCTIONS, GoalsAnswer, HOME,
    Lens, READ_INSTRUCTIONS, ReadAnswer, apply_goals, apply_read, baseline_task, deep_start,
    goals_task, read_task,
};
use crate::text::slug;
use crate::write::{WRITE_INSTRUCTIONS, WriteAnswer, apply_write, needs_write, write_task};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Kind {
    Extract,
    Extend,
    Argument,
    Write {
        page: String,
    },
    Audit {
        page: String,
        input: String,
    },
    Outline,
    Overloads,
    Goals {
        /// Goals whose readers' results never feed repairs.
        #[serde(default)]
        held_out: bool,
    },
    Read {
        goal: usize,
        persona: String,
        start: String,
    },
    Baseline {
        goal: usize,
        persona: String,
    },
    Grade {
        goal: usize,
        /// The answers' task ids, in the order the grader sees them.
        tasks: Vec<String>,
    },
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
            Kind::Argument => "argument",
            Kind::Write { .. } => "write",
            Kind::Audit { .. } => "audit",
            Kind::Outline => "outline",
            Kind::Overloads => "overloads",
            Kind::Goals { .. } => "goals",
            Kind::Read { .. } => "read",
            Kind::Baseline { .. } => "baseline",
            Kind::Grade { .. } => "grade",
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

    /// For a write task, the JSON file with the answer's structured fields (the body is
    /// the markdown answer file), relative to the project directory.
    pub fn fields(&self) -> Option<PathBuf> {
        matches!(self.kind, Kind::Write { .. })
            .then(|| PathBuf::from("exegenius/tasks").join(format!("{}.answer.json", self.id)))
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
        Kind::Argument => json_format::<ArgumentAnswer>(),
        Kind::Audit { .. } => json_format::<AuditAnswer>(),
        Kind::Outline => json_format::<OutlineAnswer>(),
        Kind::Overloads => json_format::<OverloadsAnswer>(),
        Kind::Goals { .. } => json_format::<GoalsAnswer>(),
        Kind::Read { .. } => json_format::<ReadAnswer>(),
        Kind::Baseline { .. } => json_format::<BaselineAnswer>(),
        Kind::Grade { .. } => json_format::<GradeAnswer>(),
        Kind::Write { .. } => format!(
            "The answer is two files: the page body as markdown (not JSON), and a JSON object \
             with the symbols the page binds and its relations to its prerequisite pages. In \
             JSON strings, write each backslash of LaTeX \
             twice (`\"\\\\kappa\"`). The JSON follows this schema:\n\n```json\n{}\n```",
            serde_json::to_string_pretty(&schema_of::<WriteAnswer>()).unwrap_or_default()
        ),
    };
    let files = match task.fields() {
        Some(fields) => format!(
            "the body to `{}` and the JSON to `{}`",
            answer.display(),
            fields.display()
        ),
        None => format!("the answer to `{}`", answer.display()),
    };
    format!(
        "# exegenius task {id}\n\nProject directory: `{root}` (paths below are relative to it)\n\n\
         {instructions}\n\n{input}\n## Answer\n\n{format}\n\nWrite {files}, then run \
         `exegenius submit {id}` in the project directory. If submit rejects the answer, fix \
         the answer and submit again.\n",
        id = task.id,
        root = project.root.display(),
    )
}

fn json_format<T: JsonSchema>() -> String {
    let schema = serde_json::to_string_pretty(&schema_of::<T>()).unwrap_or_default();
    format!("The answer is one JSON object following this schema:\n\n```json\n{schema}\n```")
}

/// Stands for the task's id in task input, which is written before the id is known.
const TASK_ID: &str = "{{task-id}}";

enum Step {
    Issue(Vec<(Kind, &'static str, String)>),
    /// A round ended and its repairs were queued: hand out the rewrites.
    Repaired,
    Finished,
}

/// The next step of the simulation, once the wiki is written: goals, then for each round
/// the reader tasks (with the baselines in the first round), then the grade tasks; when a
/// round is graded, repairs are queued unless it was the last round.
fn simulation_step(project: &Project, graph: &mut Graph, plan: &Plan) -> Result<Step> {
    let rounds = project.config.simulation.rounds;
    if rounds == 0 {
        return Ok(Step::Finished);
    }
    let mut sim = simulate::load(project)?;
    // Repair goals and held-out goals come from separate tasks, so neither sees the other.
    // A simulation that started before held-out goals existed carries on without them.
    let mut items = Vec::new();
    for held_out in [false, true] {
        let missing = !sim.goals.iter().any(|g| g.held_out == held_out);
        if missing && sim.runs.is_empty() {
            let input = goals_task(project, graph, plan, held_out)?;
            items.push((Kind::Goals { held_out }, GOALS_INSTRUCTIONS, input));
        }
    }
    if !items.is_empty() {
        return Ok(Step::Issue(items));
    }
    if sim.round > rounds {
        return Ok(Step::Finished);
    }
    let outline = crate::outline::current(project, plan)?;
    let lens = Lens::new(project, graph, plan, outline.as_ref())?;
    let personas = project.config.personas();
    if sim.round_runs(sim.round).next().is_none() {
        let mut items = Vec::new();
        for (g, goal) in sim.goals.iter().enumerate() {
            for persona in &personas {
                for start in [HOME.to_string(), deep_start(&lens, goal)] {
                    let input = read_task(project, &lens, TASK_ID, goal, persona, &start);
                    items.push((
                        Kind::Read {
                            goal: g,
                            persona: persona.name.clone(),
                            start,
                        },
                        READ_INSTRUCTIONS,
                        input,
                    ));
                }
                if sim.baselines.iter().all(|b| b.goal != g) {
                    items.push((
                        Kind::Baseline {
                            goal: g,
                            persona: persona.name.clone(),
                        },
                        BASELINE_INSTRUCTIONS,
                        baseline_task(&lens, goal, persona),
                    ));
                }
            }
        }
        return Ok(Step::Issue(items));
    }
    let mut items = Vec::new();
    for (g, goal) in sim.goals.iter().enumerate() {
        let tasks = to_grade(&sim, g);
        if !tasks.is_empty() {
            let input = grade_task(&sim, goal, &tasks);
            items.push((Kind::Grade { goal: g, tasks }, GRADE_INSTRUCTIONS, input));
        }
    }
    if !items.is_empty() {
        return Ok(Step::Issue(items));
    }
    let round = sim.round;
    sim.round += 1;
    let repaired = round < rounds && !repair(&sim, graph, round).is_empty();
    if repaired {
        project.save_graph(graph)?;
    }
    simulate::save(project, &sim)?;
    Ok(if repaired {
        Step::Repaired
    } else {
        Step::Finished
    })
}

/// Lens and trail access for a pending reader task.
fn reader(project: &Project, id: &str) -> Result<()> {
    let state = load_state(project)?;
    match state.pending.iter().find(|t| t.id == id) {
        Some(Task {
            kind: Kind::Read { .. },
            ..
        }) => Ok(()),
        _ => Err(Error::Invalid(format!(
            "`{id}` is not a pending read task (run `exegenius next` to see the pending tasks)"
        ))),
    }
}

/// `exegenius read`: open a page for a pending reader task.
pub fn read_page(project: &Project, id: &str, handle: &str) -> Result<String> {
    let _lock = Lock::acquire(project)?;
    reader(project, id)?;
    let graph = require(project.load_graph()?)?;
    let plan = plan(&graph, &project.sources)?;
    let outline = crate::outline::current(project, &plan)?;
    let lens = Lens::new(project, &graph, &plan, outline.as_ref())?;
    simulate::read(project, &lens, id, handle)
}

/// `exegenius search`: search the wiki for a pending reader task.
pub fn search_pages(project: &Project, id: &str, query: &str) -> Result<String> {
    let _lock = Lock::acquire(project)?;
    reader(project, id)?;
    let graph = require(project.load_graph()?)?;
    let plan = plan(&graph, &project.sources)?;
    let outline = crate::outline::current(project, &plan)?;
    let lens = Lens::new(project, &graph, &plan, outline.as_ref())?;
    simulate::search(project, &lens, id, query)
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
            Kind::Argument => "argument".to_string(),
            Kind::Write { page } => format!("write-{page}"),
            Kind::Audit { page, .. } => format!("audit-{page}"),
            Kind::Outline => "outline".to_string(),
            Kind::Overloads => "overloads".to_string(),
            Kind::Goals { held_out: false } => "goals".to_string(),
            Kind::Goals { held_out: true } => "held-out-goals".to_string(),
            Kind::Read {
                goal,
                persona,
                start,
            } => {
                let start = if start == HOME { "home" } else { "deep" };
                format!("read-g{}-{}-{start}", goal + 1, slug(persona))
            }
            Kind::Baseline { goal, persona } => {
                format!("baseline-g{}-{}", goal + 1, slug(persona))
            }
            Kind::Grade { goal, .. } => format!("grade-g{}", goal + 1),
        };
        let task = Task {
            id: format!("{:04}-{label}", state.next_id),
            kind,
        };
        state.next_id += 1;
        let input = input.replace(TASK_ID, &task.id);
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
        let Some(mut graph) = project.load_graph()? else {
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
        if crate::argument::current(project, &plan)?.is_none() {
            let input = argument_task(project, &graph, &plan)?;
            return issue(
                project,
                &mut state,
                vec![(Kind::Argument, ARGUMENT_INSTRUCTIONS, input)],
            );
        }
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
            let mut items = Vec::new();
            if crate::outline::current(project, &plan)?.is_none() {
                let input = outline_task(project, &graph, &plan)?;
                items.push((Kind::Outline, OUTLINE_INSTRUCTIONS, input));
            }
            let candidates = crate::notation::unjudged(project, &graph, &plan)?;
            if !candidates.is_empty() {
                let input = overloads_task(&graph, &candidates);
                items.push((Kind::Overloads, OVERLOADS_INSTRUCTIONS, input));
            }
            if !items.is_empty() {
                return issue(project, &mut state, items);
            }
            match simulation_step(project, &mut graph, &plan)? {
                Step::Issue(items) => return issue(project, &mut state, items),
                Step::Repaired => continue,
                Step::Finished => {}
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
        Kind::Argument => {
            let graph = require(graph)?;
            let plan = plan(&graph, &project.sources)?;
            apply_argument(project, &graph, &plan, parse(&path, &text)?)?
        }
        Kind::Write { page } => {
            let mut graph = require(graph)?;
            let plan = plan(&graph, &project.sources)?;
            find_page(&plan, page)?;
            let fields_path = project
                .root
                .join(task.fields().expect("write tasks have a fields file"));
            if !fields_path.exists() {
                return Err(Error::Invalid(format!(
                    "no answer yet: write the symbols the page binds to {} (an empty list if \
                     it binds none)",
                    task.fields()
                        .expect("write tasks have a fields file")
                        .display()
                )));
            }
            let fields = parse(&fields_path, &read(&fields_path)?)?;
            let warnings = apply_write(project, &mut graph, &plan, page, &text, fields)?;
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
        Kind::Overloads => {
            let mut graph = require(graph)?;
            let plan = plan(&graph, &project.sources)?;
            let candidates = crate::notation::unjudged(project, &graph, &plan)?;
            apply_overloads(project, &mut graph, &candidates, parse(&path, &text)?)?;
            project.save_graph(&graph)?;
            Vec::new()
        }
        Kind::Goals { held_out } => {
            let graph = require(graph)?;
            let plan = plan(&graph, &project.sources)?;
            let mut sim = simulate::load(project)?;
            let goals = apply_goals(project, &plan, parse(&path, &text)?, *held_out)?;
            // Repair goals first, then held-out goals; no run refers to a goal yet.
            sim.goals.retain(|g| g.held_out != *held_out);
            if *held_out {
                sim.goals.extend(goals);
            } else {
                sim.goals.splice(0..0, goals);
            }
            simulate::save(project, &sim)?;
            Vec::new()
        }
        Kind::Read {
            goal,
            persona,
            start,
        } => {
            let graph = require(graph)?;
            let plan = plan(&graph, &project.sources)?;
            let outline = crate::outline::current(project, &plan)?;
            let lens = Lens::new(project, &graph, &plan, outline.as_ref())?;
            let mut sim = simulate::load(project)?;
            let answer: ReadAnswer = parse(&path, &text)?;
            let assignment = simulate::Assignment {
                task: &task.id,
                round: sim.round,
                goal: *goal,
                persona,
                start,
            };
            let mut run = apply_read(project, &lens, assignment, answer)?;
            if run.outcome == simulate::Outcome::GaveUp {
                run.covered = Some(Vec::new());
            }
            let leaks = run.leaks.clone();
            sim.runs.push(run);
            simulate::save(project, &sim)?;
            leaks
                .iter()
                .map(|id| format!("the answer names `{id}` by its real name, not the one shown"))
                .collect()
        }
        Kind::Baseline { goal, .. } => {
            let graph = require(graph)?;
            let plan = plan(&graph, &project.sources)?;
            let outline = crate::outline::current(project, &plan)?;
            let lens = Lens::new(project, &graph, &plan, outline.as_ref())?;
            let answer: BaselineAnswer = parse(&path, &text)?;
            if answer.answer.trim().is_empty() {
                return Err(Error::Invalid(
                    "answer rejected: the answer is empty; say so if the background does not \
                     cover the question"
                        .into(),
                ));
            }
            let mut sim = simulate::load(project)?;
            sim.baselines.push(Baseline {
                task: task.id.clone(),
                goal: *goal,
                answer: lens.unrename(answer.answer.trim()),
                covered: None,
                contradicted: Vec::new(),
                wrong: Vec::new(),
            });
            simulate::save(project, &sim)?;
            Vec::new()
        }
        Kind::Grade { goal, tasks } => {
            let mut sim = simulate::load(project)?;
            apply_grade(&mut sim, *goal, tasks, parse(&path, &text)?)?;
            simulate::save(project, &sim)?;
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

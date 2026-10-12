//! The task loop: [`next`] decides what to hand out, [`submit`] checks and applies answers.
//!
//! Tasks are handed out in batches. Every task in a batch is independent of the others, so
//! an agent can work on them in parallel. A new batch is computed only once every task in
//! the current one has been submitted. Precedence, highest first:
//!
//! 1. extract (no graph yet)
//! 2. background (no account yet of what the reader profile covers for these sources); once
//!    it is accepted, `next` shows it once instead of tasks, so a person following the run
//!    can correct it with [`redo_background`]
//! 3. extend (audits left candidate concepts)
//! 4. argument (no plan of the argument for the current pages)
//! 5. write (pages with no body, or queued for rewriting)
//! 6. audit (pages whose audit input changed since they were last audited); a round ends
//!    when its last audit is submitted, and the loop stops after a round with no gaps or
//!    after `rounds` rounds
//!
//! When the audit rounds end, the last round's rewrite notes and candidate concepts are
//! dropped: no audit would check the rewrites, so every page stays as it was last audited.
//! When nothing is left, `next` hands out the outline task (parts, glossary and front page)
//! if the pages have no current outline, and the overloads task if pages rebind letters
//! with no verdict yet, in one batch, and then builds the docs. Writes queued by overload
//! verdicts and support checks are still handed out.

use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::aha::{
    ATTRIBUTE_INSTRUCTIONS, AttributeAnswer, apply_attribution, attribute_task, to_attribute,
};
use crate::argument::{ARGUMENT_INSTRUCTIONS, ArgumentAnswer, apply_argument, argument_task};
use crate::audit::{AUDIT_INSTRUCTIONS, AuditAnswer, apply_audit, audit_task, record};
use crate::background::{
    BACKGROUND_INSTRUCTIONS, BackgroundAnswer, apply_background, background_task,
};
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
    Background,
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
    Support {
        page: String,
        /// The second, independent judge of a sampled page.
        #[serde(default)]
        second: bool,
        /// The marks to label, in the order the task lists them.
        marks: Vec<crate::support::Mark>,
    },
    Goals {
        /// Goals whose readers' results never feed repairs.
        #[serde(default)]
        held_out: bool,
    },
    Read {
        goal: usize,
        persona: String,
        start: String,
        /// For an ablation run: the sentence hidden from the reader.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        hide: Option<simulate::Hidden>,
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
    Attribute {
        goal: usize,
        /// The runs' task ids, in the order they are listed.
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
            Kind::Background => "background",
            Kind::Extend => "extend",
            Kind::Argument => "argument",
            Kind::Write { .. } => "write",
            Kind::Audit { .. } => "audit",
            Kind::Outline => "outline",
            Kind::Overloads => "overloads",
            Kind::Support { .. } => "support",
            Kind::Goals { .. } => "goals",
            Kind::Read { .. } => "read",
            Kind::Baseline { .. } => "baseline",
            Kind::Grade { .. } => "grade",
            Kind::Attribute { .. } => "attribute",
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
    /// Why the audit rounds ended before `rounds`, if rewrites stopped reducing gaps.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopped: Option<String>,
    /// Whether `next` has shown the background answer.
    #[serde(default)]
    pub background_shown: bool,
    /// A person's correction of the last background answer, for the task given again.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction: Option<String>,
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
            stopped: None,
            background_shown: false,
            correction: None,
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
        Kind::Background => json_format::<BackgroundAnswer>(),
        Kind::Extend => json_format::<ExtendAnswer>(),
        Kind::Argument => json_format::<ArgumentAnswer>(),
        Kind::Audit { .. } => json_format::<AuditAnswer>(),
        Kind::Outline => json_format::<OutlineAnswer>(),
        Kind::Overloads => json_format::<OverloadsAnswer>(),
        Kind::Support { .. } => json_format::<crate::support::SupportAnswer>(),
        Kind::Goals { .. } => json_format::<GoalsAnswer>(),
        Kind::Read { .. } => json_format::<ReadAnswer>(),
        Kind::Baseline { .. } => json_format::<BaselineAnswer>(),
        Kind::Grade { .. } => json_format::<GradeAnswer>(),
        Kind::Attribute { .. } => json_format::<AttributeAnswer>(),
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
    /// A round ended: work out the next step again (the rewrites its repairs queued, or
    /// the ablation round).
    Again,
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
    // After the last round comes one ablation round, if configured.
    let ablating = sim.round > rounds;
    if ablating && (project.config.simulation.ablations == 0 || sim.ablated) {
        return Ok(Step::Finished);
    }
    let outline = crate::outline::current(project, plan)?;
    let lens = Lens::new(project, graph, plan, outline.as_ref())?;
    let personas = crate::background::personas(project, graph);
    if ablating && sim.round_runs(sim.round).next().is_none() {
        let prepared = crate::build::prepare(project, graph, plan, outline.as_ref())?;
        let n = project.config.simulation.ablations as usize;
        let targets = crate::aha::ablation_targets(&sim, &prepared.pages, n);
        let mut items = Vec::new();
        for target in &targets {
            let lens = Lens::new(project, graph, plan, outline.as_ref())?
                .hiding(Some(target.hidden.clone()));
            for &g in &target.goals {
                let goal = &sim.goals[g];
                for persona in &personas {
                    let start = deep_start(&lens, goal);
                    let input = read_task(project, &lens, TASK_ID, goal, persona, &start);
                    let kind = Kind::Read {
                        goal: g,
                        persona: persona.name.clone(),
                        start,
                        hide: Some(target.hidden.clone()),
                    };
                    items.push((kind, READ_INSTRUCTIONS, input));
                }
            }
        }
        if items.is_empty() {
            sim.ablated = true;
            simulate::save(project, &sim)?;
            return Ok(Step::Finished);
        }
        return Ok(Step::Issue(items));
    }
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
                            hide: None,
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
        // Page lengths as this round's readers find them, to see whether repairs make pages
        // grow round by round.
        let mut words = BTreeMap::new();
        for id in &plan.order {
            if let Some(body) = project.page_body(id)? {
                words.insert(
                    id.clone(),
                    crate::cite::strip(&body).split_whitespace().count(),
                );
            }
        }
        sim.words.insert(sim.round, words);
        simulate::save(project, &sim)?;
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
    if ablating {
        sim.ablated = true;
        simulate::save(project, &sim)?;
        return Ok(Step::Finished);
    }
    for g in 0..sim.goals.len() {
        let tasks = to_attribute(&sim, g);
        if !tasks.is_empty() {
            let input = attribute_task(&sim, g, &tasks);
            items.push((
                Kind::Attribute { goal: g, tasks },
                ATTRIBUTE_INSTRUCTIONS,
                input,
            ));
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
    let ablation = sim.round > rounds && project.config.simulation.ablations > 0;
    Ok(if repaired || ablation {
        Step::Again
    } else {
        Step::Finished
    })
}

/// Lens and trail access for a pending reader task.
/// The sentence hidden from a pending reader task, if any; an error if `id` is not one.
fn reader(project: &Project, id: &str) -> Result<Option<simulate::Hidden>> {
    let state = load_state(project)?;
    match state.pending.iter().find(|t| t.id == id) {
        Some(Task {
            kind: Kind::Read { hide, .. },
            ..
        }) => Ok(hide.clone()),
        _ => Err(Error::Invalid(format!(
            "`{id}` is not a pending read task (run `exegenius next` to see the pending tasks)"
        ))),
    }
}

/// `exegenius read`: open a page for a pending reader task.
pub fn read_page(project: &Project, id: &str, handle: &str) -> Result<String> {
    let _lock = Lock::acquire(project)?;
    let hide = reader(project, id)?;
    let graph = require(project.load_graph()?)?;
    let plan = plan(&graph, &project.sources)?;
    let outline = crate::outline::current(project, &plan)?;
    let lens = Lens::new(project, &graph, &plan, outline.as_ref())?.hiding(hide);
    simulate::read(project, &lens, id, handle)
}

/// `exegenius search`: search the wiki for a pending reader task.
pub fn search_pages(project: &Project, id: &str, query: &str) -> Result<String> {
    let _lock = Lock::acquire(project)?;
    let hide = reader(project, id)?;
    let graph = require(project.load_graph()?)?;
    let plan = plan(&graph, &project.sources)?;
    let outline = crate::outline::current(project, &plan)?;
    let lens = Lens::new(project, &graph, &plan, outline.as_ref())?.hiding(hide);
    simulate::search(project, &lens, id, query)
}

/// What `next` hands out.
#[derive(Debug)]
pub enum Next {
    Tasks {
        round: u32,
        tasks: Vec<Task>,
    },
    /// The background answer, shown once before the run goes on: the reader profile as it
    /// will be given to every later task, and the known concepts it added.
    Background {
        profile: String,
        known: Vec<String>,
    },
    Done(BuildReport),
}

/// Write the task files for a new batch, record it as pending, and return it.
fn issue(project: &Project, state: &mut State, items: Vec<(Kind, &str, String)>) -> Result<Next> {
    for (kind, instructions, input) in items {
        let label = match &kind {
            Kind::Extract => "extract".to_string(),
            Kind::Background => "background".to_string(),
            Kind::Extend => "extend".to_string(),
            Kind::Argument => "argument".to_string(),
            Kind::Write { page } => format!("write-{page}"),
            Kind::Audit { page, .. } => format!("audit-{page}"),
            Kind::Outline => "outline".to_string(),
            Kind::Overloads => "overloads".to_string(),
            Kind::Support {
                page,
                second: false,
                ..
            } => format!("support-{page}"),
            Kind::Support {
                page, second: true, ..
            } => format!("support-{page}-second"),
            Kind::Goals { held_out: false } => "goals".to_string(),
            Kind::Goals { held_out: true } => "held-out-goals".to_string(),
            Kind::Read {
                goal,
                persona,
                start,
                hide,
            } => {
                let start = match (hide, start == HOME) {
                    (Some(h), _) if h.control => "control",
                    (Some(_), _) => "ablate",
                    (None, true) => "home",
                    (None, false) => "deep",
                };
                format!("read-g{}-{}-{start}", goal + 1, slug(persona))
            }
            Kind::Baseline { goal, persona } => {
                format!("baseline-g{}-{}", goal + 1, slug(persona))
            }
            Kind::Grade { goal, .. } => format!("grade-g{}", goal + 1),
            Kind::Attribute { goal, .. } => format!("attribute-g{}", goal + 1),
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
        // The pilot is chosen once the background has settled which concepts the reader
        // knows, since known concepts get no page.
        let Some(background) = &graph.background else {
            let input = background_task(project, &graph, state.correction.as_deref());
            return issue(
                project,
                &mut state,
                vec![(Kind::Background, BACKGROUND_INSTRUCTIONS, input)],
            );
        };
        if !state.background_shown {
            state.background_shown = true;
            write_json(&project.state_path(), &state)?;
            let known = background
                .added
                .iter()
                .filter_map(|id| graph.get(id))
                .map(|c| c.name.clone())
                .collect();
            return Ok(Next::Background {
                profile: crate::background::profile(project, &graph),
                known,
            });
        }
        if let Some(pilot) = &project.config.pilot
            && graph.pilot.is_none()
        {
            graph.pilot = crate::plan::pilot(&graph, &project.sources, pilot.pages)?;
            if graph.pilot.is_some() {
                project.save_graph(&graph)?;
            }
        }
        if !graph.candidates.is_empty() {
            let limit = crate::extract::new_pages(&crate::audit::load_rounds(project)?);
            let input = extend_task(project, &graph, limit);
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
            let candidates = if crate::notation::load(project)?.rounds < crate::notation::ROUNDS {
                crate::notation::unjudged(project, &graph, &plan)?
            } else {
                Vec::new()
            };
            if !candidates.is_empty() {
                let input = overloads_task(&graph, &candidates);
                items.push((Kind::Overloads, OVERLOADS_INSTRUCTIONS, input));
            }
            let pages = crate::support::unjudged(project, &plan)?;
            for (i, (page, marks)) in pages.into_iter().enumerate() {
                let input = crate::support::support_task(project, &marks);
                let second = i == 0 && crate::support::sampled(project, &page)?;
                let copies: &[bool] = if second { &[false, true] } else { &[false] };
                for &second in copies {
                    items.push((
                        Kind::Support {
                            page: page.clone(),
                            second,
                            marks: marks.clone(),
                        },
                        crate::support::SUPPORT_INSTRUCTIONS,
                        input.clone(),
                    ));
                }
            }
            if !items.is_empty() {
                return issue(project, &mut state, items);
            }
            match simulation_step(project, &mut graph, &plan)? {
                Step::Issue(items) => return issue(project, &mut state, items),
                Step::Again => continue,
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
        Kind::Background => {
            let mut graph = require(graph)?;
            apply_background(&mut graph, parse(&path, &text)?)?;
            project.save_graph(&graph)?;
            Vec::new()
        }
        Kind::Extend => {
            let mut graph = require(graph)?;
            let limit = crate::extract::new_pages(&crate::audit::load_rounds(project)?);
            let warnings = apply_extend(project, &mut graph, parse(&path, &text)?, limit)?;
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
            let levels = crate::audit::levels(crate::argument::load(project)?.as_ref(), page);
            let audit = apply_audit(&mut graph, &plan, i, &body, parse(&path, &text)?, levels)?;
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
        Kind::Support {
            page,
            second,
            marks,
        } => {
            let mut graph = require(graph)?;
            crate::support::apply_support(
                project,
                &mut graph,
                page,
                marks,
                *second,
                parse(&path, &text)?,
            )?;
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
            hide,
        } => {
            let graph = require(graph)?;
            let plan = plan(&graph, &project.sources)?;
            let outline = crate::outline::current(project, &plan)?;
            let lens = Lens::new(project, &graph, &plan, outline.as_ref())?.hiding(hide.clone());
            let mut sim = simulate::load(project)?;
            let answer: ReadAnswer = parse(&path, &text)?;
            let assignment = simulate::Assignment {
                task: &task.id,
                hidden: hide.as_ref(),
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
        Kind::Attribute { tasks, .. } => {
            let mut sim = simulate::load(project)?;
            apply_attribution(&mut sim, tasks, parse(&path, &text)?)?;
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
        // The round is over: finish if it found nothing, if rewrites stopped reducing gaps,
        // or if it was the last, else start the next one.
        let stall = crate::convergence::summarise(&crate::audit::load_rounds(project)?)
            .into_iter()
            .find(|s| s.round == state.round)
            .and_then(|s| crate::convergence::stalled(&s))
            .filter(|_| project.config.stop_early);
        if state.round_gaps == 0 {
            state.done = true;
        } else if stall.is_some() || state.round >= project.config.rounds {
            state.done = true;
            state.stopped = stall;
            let mut graph = require(project.load_graph()?)?;
            graph.rewrite.clear();
            graph.note_depth.clear();
            graph.candidates.clear();
            project.save_graph(&graph)?;
        } else {
            state.round += 1;
            state.round_gaps = 0;
        }
    }
    write_json(&project.state_path(), &state)?;
    Ok(warnings)
}

/// `exegenius background --redo`: take the background answer back out of the graph and give
/// the task again with a person's correction. Only before any later task has been handed out,
/// since later tasks were given the answer.
pub fn redo_background(project: &Project, correction: &str) -> Result<()> {
    let _lock = Lock::acquire(project)?;
    let mut state = load_state(project)?;
    let mut graph = require(project.load_graph()?)?;
    let later = !state.pending.is_empty()
        || graph.pilot.is_some()
        || project.argument_path().exists()
        || project.audit_path().exists();
    if graph.background.is_none() || later {
        return Err(Error::Invalid(
            "the background can be redone only right after `next` shows it, before any \
             later task"
                .into(),
        ));
    }
    if correction.trim().is_empty() {
        return Err(Error::Invalid(
            "say what to correct in the background".into(),
        ));
    }
    crate::background::undo(&mut graph);
    project.save_graph(&graph)?;
    state.background_shown = false;
    state.correction = Some(correction.trim().to_string());
    write_json(&project.state_path(), &state)
}

fn require(graph: Option<Graph>) -> Result<Graph> {
    graph.ok_or_else(|| Error::Invalid("no concept graph yet".into()))
}

pub fn load(project: &Project) -> Result<State> {
    load_state(project)
}

//! Simulated readers: the wiki as a game against a reader with a stated background.
//!
//! Once the wiki is written, each simulated reader (a fresh subagent, given one reader
//! profile and one goal question) tries to answer the question using only the wiki. It
//! sees the wiki only through `exegenius read` and `exegenius search`, which serve pages
//! by opaque handles and log every step, so its path through the wiki is recorded rather
//! than reported. With `nonce`, the concepts the wiki teaches carry invented names in
//! everything the reader sees (names of several words and name-like words such as
//! acronyms; single ordinary words are left alone), so recall that depends on a concept's name is cut off, and
//! a reader that uses a real name in its answer has shown knowledge from outside the wiki.
//!
//! A matched baseline answers each question with no wiki at all. A blind grader then
//! scores every answer against the goal's key points (see [`crate::grade`]).

use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::Range;

use pulldown_cmark::{Event, LinkType, Options, Parser, Tag, TagEnd};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::build::prepare;
use crate::config::Persona;
use crate::error::{Error, Result, read_json, write_json};
use crate::graph::{Concept, Graph};
use crate::outline::Outline;
use crate::plan::Plan;
use crate::project::Project;
use crate::text::{contains_quote, normalise};
use crate::write::link_target;

/// The handle of the front page.
pub const HOME: &str = "home";

pub const GOALS_INSTRUCTIONS: &str = "\
Write goal questions for testing whether an explanatory wiki teaches its reader. Simulated \
readers with the reader's background will each try to answer one question using only the \
wiki, and a grader will check their answers against your key points. The questions are \
never shown in the wiki.

- Write the number of goals the task asks for: at least one about the target result itself, \
the rest about the main steps on the way to it.
- `kind`: one of the kinds in the schema, each a kind of understanding a reader of a proof \
can show (after Mejía-Ramos et al.'s assessment model of proof comprehension, and Hamami and \
Morris on why a proof proceeds as it does). `meaning` and `status` test single statements; \
the rest test how statements fit together. Make at least half of the goals of the other \
kinds, include at least one `summary`, `structure`, `transfer` or `example` goal, and at least \
one `motivation` goal. Use the argument's steps below for `motivation`, `structure` and \
`summary` goals. A motivation the argument marks as not stated by the sources is a good \
test of whether the wiki supplies it, and each striking feature listed there calls for a \
`justification` goal asking why it holds.
- `key_points`: 2 to 5 short, checkable statements that a full answer must make, accurate to \
the sources, each with a `role`: the `claim` asserted, the `reason` it holds, the \
`conclusion` that follows, or the `background` it rests on. A goal about why something holds \
needs at least one `reason`. Phrase key points of the kinds other than `meaning` and \
`status` in your own words: submit rejects such a key point if it appears word for word on a \
page, since a reader could copy it without understanding.
- `pages`: the ids of the pages a reader needs to answer the question.
- Ask what this reader could answer after reading those pages well, not trivia.";

pub const READ_INSTRUCTIONS: &str = "\
Simulate a reader using a wiki to answer a question. The point is to find out whether the \
wiki gets this reader there, so behave like this reader, not like an expert.

- The reader knows exactly the background stated below. Everything else you know, the reader \
does not. When a page relies on something outside that background, the reader has to find \
it in the wiki: follow a link, search, or go back to an earlier page.
- The wiki may give the concepts it teaches invented names. Treat them as new words: do not \
guess which real terms they stand for, and do not use real names for them in your answer.
- See the wiki only through the two commands below. Use no other tools except to write \
your answer file and run `exegenius submit`: do not open files, search the web, or start \
sessions or agents.
- Record every place you got stuck: the page, a short quote copied from it, what you could \
not follow, and its level: a symbol or term you could not decode, a statement whose status \
you could not tell, a single step you could not follow, a chain whose steps you followed \
without seeing why they lead where they do, or something you could not connect to anything \
you know. Include places you got past only by guessing.
- Answer from the pages alone. Support each step of the answer with a quote copied exactly \
from a page you opened. If the pages do not get you to an answer within the page budget, \
give up and say where it broke down.
- If you answer, also name the key idea of the page you relied on most: say it in a \
sentence, and quote the passage of the page that carries it.
- An honest `gave_up` is worth more than a fluent answer the pages do not support.";

pub const BASELINE_INSTRUCTIONS: &str = "\
Answer a question as a reader with exactly the background stated below, with no access to \
any other material. This answer is the baseline that answers written with the help of a \
wiki are compared against.

- Use only what that background covers. Everything else you know, the reader does not.
- Some names in the question may be invented: do not guess what they stand for.
- Use no tools except to write your answer file and run `exegenius submit`: do not open \
files, search the web, or start sessions or agents. If the background does not let the \
reader answer, say so plainly instead of guessing.";

/// What a goal question asks of the reader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum GoalKind {
    /// What a symbol or term denotes: "What does r count?"
    Meaning,
    /// What part a statement plays: a definition, an assumption, a condition, a claim.
    Status,
    /// Why a step or claim holds.
    Justification,
    /// Summarise an argument or construction in a few sentences.
    Summary,
    /// How parts of the argument relate: what one step contributes to another.
    Structure,
    /// Apply or extend: what changes in a new case, what a result implies.
    Transfer,
    /// Work a small example through.
    Example,
    /// Why the argument takes this route: why this method or construction.
    Motivation,
    /// Where the argument stops being routine.
    NonRoutine,
    /// Why the obvious approach fails.
    ObviousFails,
    /// What something is or states: the kind goals had before the kinds above.
    #[schemars(skip)]
    Recall,
}

impl GoalKind {
    /// Whether the goal tests how statements fit together, rather than single statements.
    pub fn holistic(self) -> bool {
        !matches!(
            self,
            GoalKind::Meaning | GoalKind::Status | GoalKind::Recall
        )
    }

    pub fn name(self) -> &'static str {
        match self {
            GoalKind::Meaning => "meaning",
            GoalKind::Status => "status",
            GoalKind::Justification => "justification",
            GoalKind::Summary => "summary",
            GoalKind::Structure => "structure",
            GoalKind::Transfer => "transfer",
            GoalKind::Example => "example",
            GoalKind::Motivation => "motivation",
            GoalKind::NonRoutine => "non-routine",
            GoalKind::ObviousFails => "obvious-fails",
            GoalKind::Recall => "recall",
        }
    }
}

/// The part a key point plays in a full answer.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// What is asserted.
    Claim,
    /// Why it holds.
    Reason,
    /// What follows from it.
    Conclusion,
    /// What it rests on.
    Background,
}

impl Role {
    pub fn name(self) -> &'static str {
        match self {
            Role::Claim => "claim",
            Role::Reason => "reason",
            Role::Conclusion => "conclusion",
            Role::Background => "background",
        }
    }
}

/// A short checkable statement a full answer must make.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct KeyPoint {
    pub point: String,
    pub role: Role,
}

/// Key points were plain strings before they had roles; those read as claims.
impl<'de> Deserialize<'de> for KeyPoint {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Full {
            point: String,
            role: Role,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Stored {
            Full(Full),
            Text(String),
        }
        Ok(match Stored::deserialize(deserializer)? {
            Stored::Full(Full { point, role }) => KeyPoint { point, role },
            Stored::Text(point) => KeyPoint {
                point,
                role: Role::Claim,
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Goal {
    pub question: String,
    pub kind: GoalKind,
    /// Short checkable statements a full answer must make, each with its role.
    pub key_points: Vec<KeyPoint>,
    /// Ids of the pages needed to answer.
    pub pages: Vec<String>,
    /// Set by code for goals from the held-out goals task, which never feed repairs.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    #[schemars(skip)]
    pub held_out: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GoalsAnswer {
    pub goals: Vec<Goal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Answered,
    GaveUp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Citation {
    /// Handle of a page you opened.
    pub page: String,
    /// Text copied exactly from that page.
    pub quote: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Stuck {
    /// Handle of the page where you got stuck.
    pub page: String,
    /// Text copied exactly from that page, where the trouble starts.
    pub quote: String,
    /// What you could not follow.
    pub note: String,
    pub level: Level,
}

/// Where understanding broke down, from Spratte's levels of proof comprehension.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    /// A symbol or term I could not decode.
    Symbol,
    /// I could not tell what a statement's status was: definition, assumption or claim.
    Statement,
    /// I could not follow one inference.
    Step,
    /// I followed the steps but not why they lead where they do.
    Chain,
    /// I could not connect it to anything I already know.
    Beyond,
}

impl Level {
    pub fn name(self) -> &'static str {
        match self {
            Level::Symbol => "symbol",
            Level::Statement => "statement",
            Level::Step => "step",
            Level::Chain => "chain",
            Level::Beyond => "beyond",
        }
    }
}

/// A place a reader got stuck, as recorded.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StuckPoint {
    pub page: String,
    pub quote: String,
    pub note: String,
    /// Missing for stuck points recorded before levels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
}

/// Stuck points were (page, quote, note) triples before they had levels.
impl<'de> Deserialize<'de> for StuckPoint {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Full {
            page: String,
            quote: String,
            note: String,
            level: Option<Level>,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Stored {
            Full(Full),
            Triple(String, String, String),
        }
        Ok(match Stored::deserialize(deserializer)? {
            Stored::Full(Full {
                page,
                quote,
                note,
                level,
            }) => StuckPoint {
                page,
                quote,
                note,
                level,
            },
            Stored::Triple(page, quote, note) => StuckPoint {
                page,
                quote,
                note,
                level: None,
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReadAnswer {
    pub outcome: Outcome,
    /// The answer to the question; empty if you gave up.
    pub answer: String,
    /// Quotes supporting the answer.
    pub citations: Vec<Citation>,
    /// Every place you got stuck.
    pub stuck: Vec<Stuck>,
    /// Why you gave up, or how sure you are of the answer.
    pub reason: String,
    /// If you answered: the key idea of the page you relied on most.
    #[serde(default)]
    pub key_idea: Option<KeyIdeaAnswer>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct KeyIdeaAnswer {
    /// Handle of the page you relied on most.
    pub page: String,
    /// The key idea of that page, in one sentence of your own.
    pub statement: String,
    /// The passage of the page that carries it, copied exactly.
    pub quote: String,
}

/// A sentence hidden from a reader, to see what readers lose without it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hidden {
    pub page: String,
    pub sentence: String,
    /// A control sentence: one from the same page that no reader relied on, hidden to show
    /// what removing any sentence of that length costs.
    pub control: bool,
}

/// A reader's statement of a page's key idea, with names and handles translated back.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyIdea {
    pub page: String,
    pub statement: String,
    pub quote: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BaselineAnswer {
    /// The answer, or a plain statement that the background does not cover it.
    pub answer: String,
}

/// One search a reader made.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Search {
    pub query: String,
    /// Page ids found, best first.
    pub found: Vec<String>,
}

/// A reader's steps so far, while its task is pending.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Trail {
    /// Page ids opened, in order (`home` for the front page).
    pub path: Vec<String>,
    pub searches: Vec<Search>,
}

/// A finished reader run, with names and handles translated back.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Run {
    pub task: String,
    pub round: u32,
    pub goal: usize,
    pub persona: String,
    /// Page id the reader started at (`home` for the front page).
    pub start: String,
    pub outcome: Outcome,
    pub path: Vec<String>,
    pub searches: Vec<Search>,
    pub answer: String,
    /// (page id, quote)
    pub citations: Vec<(String, String)>,
    pub stuck: Vec<StuckPoint>,
    pub reason: String,
    /// Ids of concepts the reader named by their real names, which it cannot have read.
    pub leaks: Vec<String>,
    /// Indices of the key points the grader found in the answer.
    pub covered: Option<Vec<usize>>,
    /// Indices of the key points the answer contradicts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contradicted: Vec<usize>,
    pub wrong: Vec<String>,
    /// For each covered key point a citation supports: (key point, citation) indices, once
    /// attributed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support: Option<Vec<(usize, usize)>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_idea: Option<KeyIdea>,
    /// The sentence hidden from this reader, in an ablation run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<Hidden>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Baseline {
    pub task: String,
    pub goal: usize,
    pub answer: String,
    pub covered: Option<Vec<usize>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contradicted: Vec<usize>,
    pub wrong: Vec<String>,
}

/// The simulation's state: goals, the current round, and every run so far.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Simulation {
    pub goals: Vec<Goal>,
    /// The round being run, from 1; past the configured rounds when finished.
    pub round: u32,
    pub runs: Vec<Run>,
    pub baselines: Vec<Baseline>,
    /// Whether the ablation runs after the last round are done.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub ablated: bool,
    /// The word count of each page body when each round's readers set out, by round.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub words: BTreeMap<u32, BTreeMap<String, usize>>,
}

impl Default for Simulation {
    fn default() -> Self {
        Self {
            goals: Vec::new(),
            round: 1,
            runs: Vec::new(),
            baselines: Vec::new(),
            ablated: false,
            words: BTreeMap::new(),
        }
    }
}

impl Simulation {
    pub fn round_runs(&self, round: u32) -> impl Iterator<Item = &Run> {
        self.runs.iter().filter(move |r| r.round == round)
    }
}

pub fn load(project: &Project) -> Result<Simulation> {
    let path = project.simulation_path();
    if path.exists() {
        read_json(&path)
    } else {
        Ok(Simulation::default())
    }
}

pub fn save(project: &Project, simulation: &Simulation) -> Result<()> {
    write_json(&project.simulation_path(), simulation)
}

pub fn load_trail(project: &Project, task: &str) -> Result<Trail> {
    let path = project.trail_path(task);
    if path.exists() {
        read_json(&path)
    } else {
        Ok(Trail::default())
    }
}

pub fn save_trail(project: &Project, task: &str, trail: &Trail) -> Result<()> {
    write_json(&project.trail_path(task), trail)
}

fn digest(text: &str) -> Vec<u8> {
    Sha256::digest(text.as_bytes()).to_vec()
}

/// An invented word built from the bytes of `seed`: alternating consonants and vowels.
fn nonce_word(seed: &[u8], syllables: usize) -> String {
    const CONSONANTS: &[u8] = b"bdfgklmnprstvz";
    const VOWELS: &[u8] = b"aeiou";
    let mut word = String::new();
    for (i, pair) in seed.chunks(2).take(syllables).enumerate() {
        word.push(CONSONANTS[pair[0] as usize % CONSONANTS.len()] as char);
        word.push(VOWELS[pair[1] as usize % VOWELS.len()] as char);
        if i + 1 == syllables {
            word.push(CONSONANTS[pair[1] as usize / 8 % CONSONANTS.len()] as char);
        }
    }
    word
}

/// How a reader sees the wiki: pages behind opaque handles, and (with nonce names) the
/// concepts the wiki teaches under invented names.
pub struct Lens<'a> {
    graph: &'a Graph,
    /// Pages with a body, in site order, with their bodies as the site shows them.
    pages: Vec<(&'a Concept, String)>,
    handles: BTreeMap<String, String>,
    ids: HashMap<String, String>,
    /// Real name (and alias) → invented word, longest first.
    renames: Vec<(String, String)>,
    /// Invented word → concept id.
    nonces: HashMap<String, String>,
    title: String,
    home: String,
    /// Each page's symbol accounting from its latest audit, for the inherited symbols.
    uses: HashMap<String, Vec<crate::audit::SymbolUse>>,
    /// The plan of the argument, if it was made for these pages.
    argument: Option<crate::argument::Argument>,
    /// A sentence left out of its page, for an ablation run.
    hidden: Option<Hidden>,
}

impl<'a> Lens<'a> {
    pub fn new(
        project: &Project,
        graph: &'a Graph,
        plan: &Plan,
        outline: Option<&Outline>,
    ) -> Result<Self> {
        let prepared = prepare(project, graph, plan, outline)?;
        let corpus = normalise(
            &prepared
                .pages
                .iter()
                .map(|(c, b)| format!("{} {b}", c.name))
                .collect::<Vec<_>>()
                .join(" "),
        )
        .to_lowercase();

        let mut handles = BTreeMap::new();
        let mut ids = HashMap::new();
        let mut taken: HashSet<String> = [HOME.to_string()].into();
        let mut renames = Vec::new();
        let mut nonces = HashMap::new();
        for (c, _) in &prepared.pages {
            let seed = digest(&c.id);
            let mut handle = nonce_word(&seed, 1);
            let mut n = 2;
            while taken.contains(&handle) {
                handle = nonce_word(&digest(&format!("{}{n}", c.id)), 1);
                n += 1;
            }
            taken.insert(handle.clone());
            handles.insert(c.id.clone(), handle.clone());
            ids.insert(handle, c.id.clone());
            if project.config.simulation.nonce {
                let mut word = nonce_word(&seed[8..], 3);
                let mut n = 2;
                while taken.contains(&word) || corpus.contains(&word) {
                    word = nonce_word(&digest(&format!("{}#{n}", c.id)), 3);
                    n += 1;
                }
                taken.insert(word.clone());
                // A single ordinary word ("Record", "row") is left alone: renaming it would
                // also rename its everyday uses ("records" as a verb), and it gives little
                // away.
                for name in std::iter::once(&c.name).chain(&c.aliases) {
                    let name = name.trim();
                    if name.split_whitespace().count() > 1 || crate::autolink::is_name_like(name) {
                        renames.push((name.to_string(), word.clone()));
                    }
                }
                nonces.insert(word, c.id.clone());
            }
        }
        renames.sort_by_key(|(name, _)| std::cmp::Reverse(name.chars().count()));

        let title = crate::build::title(project, graph);
        let mut lens = Self {
            graph,
            pages: prepared.pages,
            handles,
            ids,
            renames,
            nonces,
            title,
            home: String::new(),
            uses: crate::audit::latest_symbols(project)?,
            argument: crate::argument::current(project, plan)?,
            hidden: None,
        };
        lens.home = lens.render_home(project, outline);
        Ok(lens)
    }

    /// The same lens, with `hidden`'s sentence left out of its page.
    pub fn hiding(mut self, hidden: Option<Hidden>) -> Self {
        self.hidden = hidden;
        self
    }

    pub fn handle(&self, id: &str) -> Option<&str> {
        if id == HOME {
            return Some(HOME);
        }
        self.handles.get(id).map(String::as_str)
    }

    /// The page id behind a handle (`home` for the front page).
    pub fn id(&self, handle: &str) -> Option<&str> {
        if handle == HOME {
            return Some(HOME);
        }
        self.ids.get(handle).map(String::as_str)
    }

    /// Replace the real names of taught concepts with their invented words.
    pub fn rename(&self, text: &str) -> String {
        if self.renames.is_empty() {
            return text.to_string();
        }
        map_text(text, |t| replace_terms(t, &self.renames, true))
    }

    /// Replace invented words with the real names, for graders and the report.
    pub fn unrename(&self, text: &str) -> String {
        let back: Vec<(String, String)> = self
            .nonces
            .iter()
            .filter_map(|(word, id)| Some((word.clone(), self.graph.get(id)?.name.clone())))
            .collect();
        replace_terms(text, &back, false)
    }

    /// Ids of taught concepts named in `text` by a real name of two or more words, or an
    /// acronym, which a reader who saw only invented names cannot have read.
    pub fn leaks(&self, text: &str) -> Vec<String> {
        let mut found = Vec::new();
        for (name, word) in &self.renames {
            let telling = name.split_whitespace().count() > 1
                || name.chars().filter(|c| c.is_uppercase()).count() > 1;
            let id = &self.nonces[word];
            if telling && !found.contains(id) && !find_all(text, name).is_empty() {
                found.push(id.clone());
            }
        }
        found
    }

    fn links(&self, text: &str) -> String {
        let mut edits: Vec<(Range<usize>, String)> = Vec::new();
        for (event, range) in Parser::new_ext(text, Options::ENABLE_MATH).into_offset_iter() {
            let Event::Start(Tag::Link {
                link_type: LinkType::Inline,
                dest_url,
                ..
            }) = event
            else {
                continue;
            };
            let handle = match link_target(&dest_url) {
                Some("index") => HOME,
                Some(id) if !id.is_empty() => match self.handles.get(id) {
                    Some(h) => h.as_str(),
                    None => continue,
                },
                _ => continue,
            };
            let source = &text[range.clone()];
            if let Some(split) = source.rfind("](") {
                edits.push((range, format!("{}]({handle})", &source[..split])));
            }
        }
        let mut out = text.to_string();
        for (range, edit) in edits.into_iter().rev() {
            out.replace_range(range, &edit);
        }
        out
    }

    fn link(&self, c: &Concept) -> String {
        format!("[{}]({})", c.name, self.handles[&c.id])
    }

    /// A link to a page by id, if the page is shown.
    fn link_id(&self, id: &str) -> Option<String> {
        self.graph
            .get(id)
            .filter(|c| self.handles.contains_key(&c.id))
            .map(|c| self.link(c))
    }

    fn render_home(&self, project: &Project, outline: Option<&Outline>) -> String {
        let mut out = format!("# {}\n\n", self.title);
        let built: HashMap<&str, &Concept> = self
            .pages
            .iter()
            .map(|(c, _)| (c.id.as_str(), *c))
            .collect();
        let list = |ids: &[String]| {
            ids.iter()
                .filter_map(|id| built.get(id.as_str()))
                .map(|c| format!("- {}", self.link(c)))
                .collect::<Vec<_>>()
                .join("\n")
        };
        let argument = self.argument.as_ref().map_or(String::new(), |a| {
            let link = |id: &str| self.link_id(id);
            format!(
                "## The argument\n\n{}\n",
                crate::argument::overview(a, &link)
            )
        });
        match outline {
            Some(o) => {
                out.push_str(&format!("{}\n\n{argument}## Contents\n", o.overview));
                for part in &o.parts {
                    out.push_str(&format!(
                        "\n### {}\n\n{}\n\n{}\n",
                        part.title,
                        part.intro,
                        list(&part.pages)
                    ));
                }
                if !o.glossary.is_empty() {
                    out.push_str(&format!("\n### Glossary\n\n{}\n", list(&o.glossary)));
                }
            }
            None => {
                let ids: Vec<String> = self.pages.iter().map(|(c, _)| c.id.clone()).collect();
                out.push_str(&format!(
                    "Written for a reader who knows: {}\n\n{argument}## Pages\n\n{}\n",
                    project.config.reader.knows.trim(),
                    list(&ids)
                ));
            }
        }
        self.rename(&self.links(&out))
    }

    /// A page as the reader sees it, or `None` for an unknown handle.
    pub fn page(&self, handle: &str) -> Option<String> {
        if handle == HOME {
            return Some(self.home.clone());
        }
        let id = self.ids.get(handle)?;
        let (c, body) = self.pages.iter().find(|(c, _)| &c.id == id)?;
        let mut out = format!("# {}\n\n", c.name);
        let before: Vec<(String, Option<&str>)> = c
            .requires
            .iter()
            .filter_map(|r| self.graph.get(r))
            .filter(|p| self.handles.contains_key(&p.id))
            .map(|p| {
                let relation = crate::relations::find(self.graph, &c.id, &p.id);
                (self.link(p), relation.map(|e| e.predicate.as_str()))
            })
            .collect();
        let known: Vec<&str> = c
            .requires
            .iter()
            .filter_map(|r| self.graph.get(r))
            .filter(|p| p.known)
            .map(|p| p.name.as_str())
            .collect();
        if !before.is_empty() {
            out.push_str(&format!(
                "{}\n",
                crate::relations::sentence(&c.name, &before)
            ));
        }
        if !known.is_empty() {
            out.push_str(&format!("Assumes: {}.\n", known.join(", ")));
        }
        if let Some(argument) = &self.argument {
            let link = |id: &str| self.link_id(id);
            let trail = crate::argument::trail(argument, id, &link);
            if !trail.is_empty() {
                out.push_str(&format!("Where this fits:\n{}\n", trail.join("\n")));
            }
        }
        let uses = self.uses.get(id).map(Vec::as_slice).unwrap_or_default();
        let symbols: Vec<String> = crate::notation::inherited(self.graph, uses, body)
            .iter()
            .map(|b| {
                let from = b.origin.concept();
                let from = match self.handles.get(&from.id) {
                    Some(_) => self.link(from),
                    None => from.name.clone(),
                };
                format!(
                    "- ${}$ ({}): {} From {from}.",
                    b.symbol,
                    b.name,
                    b.meaning.trim()
                )
            })
            .collect();
        if !symbols.is_empty() {
            out.push_str(&format!(
                "Symbols from earlier pages:\n{}\n",
                symbols.join("\n")
            ));
        }
        let mut body = crate::cite::strip(body);
        if let Some(h) = self.hidden.as_ref().filter(|h| &h.page == id) {
            body = body.replacen(&h.sentence, "", 1);
        }
        out.push_str(&format!("\n{}\n", body.trim()));
        let used_by: Vec<String> = self
            .pages
            .iter()
            .filter(|(d, _)| d.requires.contains(&c.id))
            .map(|(d, _)| self.link(d))
            .collect();
        if !used_by.is_empty() {
            out.push_str(&format!("\nUsed by: {}.\n", used_by.join(", ")));
        }
        out.push_str(&format!("\n[Front page]({HOME})\n"));
        Some(self.rename(&self.links(&out)))
    }

    /// Pages matching every word of `query`, best first: (id, title as seen, snippet).
    pub fn search(&self, query: &str) -> Vec<(String, String, String)> {
        let words: Vec<String> = query.split_whitespace().map(|w| w.to_lowercase()).collect();
        if words.is_empty() {
            return Vec::new();
        }
        let mut hits = Vec::new();
        for (c, _) in &self.pages {
            let text = self.page(&self.handles[&c.id]).unwrap_or_default();
            let lower = text.to_lowercase();
            if !words.iter().all(|w| lower.contains(w)) {
                continue;
            }
            let title = text
                .lines()
                .next()
                .unwrap_or("")
                .trim_start_matches("# ")
                .to_string();
            let score: usize = words
                .iter()
                .map(|w| {
                    lower.matches(w.as_str()).count()
                        + 5 * title.to_lowercase().matches(w.as_str()).count()
                })
                .sum();
            let snippet = text
                .lines()
                .skip(1)
                .find(|l| words.iter().any(|w| l.to_lowercase().contains(w)))
                .unwrap_or("")
                .chars()
                .take(160)
                .collect::<String>();
            hits.push((score, c.id.clone(), title, snippet));
        }
        hits.sort_by_key(|h| std::cmp::Reverse(h.0));
        hits.into_iter()
            .take(5)
            .map(|(_, id, title, snippet)| (id, title, snippet))
            .collect()
    }
}

/// Apply `f` to the plain text of a markdown document: not code, mathematics or link
/// destinations.
fn map_text(text: &str, f: impl Fn(&str) -> String) -> String {
    let mut edits: Vec<(Range<usize>, String)> = Vec::new();
    let mut code = 0usize;
    for (event, range) in Parser::new_ext(text, Options::ENABLE_MATH).into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(_)) => code += 1,
            Event::End(TagEnd::CodeBlock) => code = code.saturating_sub(1),
            Event::Text(t) if code == 0 && text.get(range.clone()) == Some(&*t) => {
                let mapped = f(&t);
                if mapped != *t {
                    edits.push((range, mapped));
                }
            }
            _ => {}
        }
    }
    let mut out = text.to_string();
    for (range, edit) in edits.into_iter().rev() {
        out.replace_range(range, &edit);
    }
    out
}

/// Byte ranges of whole-word, ASCII-case-insensitive matches of `term` in `text`,
/// including a plural ending.
fn find_all(text: &str, term: &str) -> Vec<Range<usize>> {
    let hay = text.to_ascii_lowercase();
    let needle = term.to_ascii_lowercase();
    let boundary = |c: Option<char>| c.is_none_or(|c| !c.is_alphanumeric());
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(i) = hay[from..].find(&needle) {
        let start = from + i;
        let mut end = start + needle.len();
        from = end;
        if !boundary(text[..start].chars().next_back()) {
            continue;
        }
        for plural in ["es", "s"] {
            if text[end..].starts_with(plural)
                && boundary(text[end + plural.len()..].chars().next())
            {
                end += plural.len();
                break;
            }
        }
        if boundary(text[end..].chars().next()) {
            found.push(start..end);
        }
    }
    found
}

/// Replace each whole-word match of the terms (longest first) with its replacement,
/// keeping a plural ending and, with `capitalise`, an initial capital.
fn replace_terms(text: &str, terms: &[(String, String)], capitalise: bool) -> String {
    let mut claimed: Vec<(Range<usize>, String)> = Vec::new();
    for (term, replacement) in terms {
        for range in find_all(text, term) {
            if claimed
                .iter()
                .any(|(r, _)| r.start < range.end && range.start < r.end)
            {
                continue;
            }
            let matched = &text[range.clone()];
            let plural = &matched[term.len().min(matched.len())..];
            let mut word = replacement.clone();
            if capitalise && matched.chars().next().is_some_and(char::is_uppercase) {
                let mut chars = word.chars();
                word = chars
                    .next()
                    .map(|c| c.to_uppercase().chain(chars).collect())
                    .unwrap_or_default();
            }
            claimed.push((range, format!("{word}{plural}")));
        }
    }
    claimed.sort_by_key(|(r, _)| r.start);
    let mut out = text.to_string();
    for (range, edit) in claimed.into_iter().rev() {
        out.replace_range(range, &edit);
    }
    out
}

/// The goal count a goals task asks for: repair goals, or held-out goals, which are never
/// used to repair the wiki.
pub fn goal_count(held_out: bool) -> std::ops::RangeInclusive<usize> {
    if held_out { 1..=3 } else { 3..=6 }
}

pub fn goals_task(project: &Project, graph: &Graph, plan: &Plan, held_out: bool) -> Result<String> {
    let count = goal_count(held_out);
    let mut task = if held_out {
        format!(
            "## Held-out goals\n\nWrite {} to {} goals. They are held out: readers' results on \
             them are never used to repair the wiki, so they measure whether the wiki teaches \
             rather than whether it was repaired to answer its own test questions. Another task \
             writes the goals used for repairs; you do not see them.\n\n",
            count.start(),
            count.end()
        )
    } else {
        format!(
            "## Goals\n\nWrite {} to {} goals.\n\n",
            count.start(),
            count.end()
        )
    };
    if let Some(target) = graph.get(&graph.target) {
        task.push_str(&format!(
            "## Target\n\n{}: {}\n\n",
            target.name, target.gloss
        ));
    }
    task.push_str(&format!(
        "## Reader\n\n{}\n\n## Pages, in reading order\n\n",
        project.config.reader.knows.trim()
    ));
    for id in &plan.order {
        let c = graph.get(id).expect("planned pages are concepts");
        task.push_str(&format!("- `{}`: {} — {}\n", c.id, c.name, c.gloss));
    }
    if let Some(argument) = crate::argument::current(project, plan)? {
        task.push_str("\n## The argument's steps\n\n");
        for (path, n) in argument.root.walk() {
            if n.intention != crate::argument::Intention::Show {
                continue;
            }
            let indent = "  ".repeat(path.len());
            let page = n
                .page
                .as_deref()
                .map_or(String::new(), |p| format!(" (`{p}`)"));
            task.push_str(&format!("{indent}- {}{page}\n", n.claim));
            if let Some(method) = &n.method {
                task.push_str(&format!("{indent}  - How: {method}\n"));
            }
            if let Some(feature) = &n.salient {
                task.push_str(&format!("{indent}  - Striking feature: {feature}\n"));
            }
            if let Some(m) = &n.motivation {
                let source = match m.source {
                    crate::argument::Provenance::Explicit => "stated by the sources",
                    crate::argument::Provenance::Recoverable => "recoverable from the sources",
                    crate::argument::Provenance::Reconstructed => "not stated by the sources",
                };
                task.push_str(&format!("{indent}  - Why ({source}): {}\n", m.text.trim()));
            }
        }
    }
    task.push_str(&format!(
        "\nThe page bodies are in `exegenius/pages/<id>.md`.\n\n## Sources\n\n{}\n",
        project.sources_listing()
    ));
    Ok(task)
}

/// Check the goals from a goals task, and mark them held out if they are.
pub fn apply_goals(
    project: &Project,
    plan: &Plan,
    answer: GoalsAnswer,
    held_out: bool,
) -> Result<Vec<Goal>> {
    let mut errors = Vec::new();
    let count = goal_count(held_out);
    // One goal over the count is let through: the count is guidance for cost, not a rule.
    if answer.goals.len() < *count.start() || answer.goals.len() > count.end() + 1 {
        errors.push(format!(
            "give {} to {} goals, not {}",
            count.start(),
            count.end(),
            answer.goals.len()
        ));
    }
    let mut bodies = Vec::new();
    for id in &plan.order {
        if let Some(body) = project.page_body(id)? {
            bodies.push((id.clone(), normalise(&body)));
        }
    }
    for (i, goal) in answer.goals.iter().enumerate() {
        let n = i + 1;
        if goal.question.trim().is_empty() {
            errors.push(format!("goal {n} has no question"));
        }
        if goal.key_points.is_empty() || goal.key_points.iter().any(|k| k.point.trim().is_empty()) {
            errors.push(format!("goal {n} needs non-empty key points"));
        }
        if goal.pages.is_empty() {
            errors.push(format!("goal {n} names no pages"));
        }
        for page in &goal.pages {
            if !plan.contains(page) {
                errors.push(format!("goal {n}: `{page}` is not a page of this wiki"));
            }
        }
        if goal.kind == GoalKind::Recall {
            errors.push(format!(
                "goal {n}: `recall` is no longer a kind; use `meaning` or `status`"
            ));
        }
        if goal.kind.holistic() {
            for point in goal.key_points.iter().map(|k| &k.point) {
                if let Some((id, _)) = bodies
                    .iter()
                    .find(|(_, body)| point.trim().len() >= 20 && contains_quote(body, point))
                {
                    errors.push(format!(
                        "goal {n}: {} key point \"{}\" appears word for word on `{id}`; \
                         phrase it in your own words",
                        goal.kind.name(),
                        point.trim()
                    ));
                }
            }
        }
    }
    if !errors.is_empty() {
        return Err(rejected(errors));
    }
    Ok(answer
        .goals
        .into_iter()
        .map(|g| Goal { held_out, ..g })
        .collect())
}

pub(crate) fn rejected(errors: Vec<String>) -> Error {
    Error::Invalid(format!(
        "answer rejected:\n{}",
        errors
            .iter()
            .map(|e| format!("- {e}"))
            .collect::<Vec<_>>()
            .join("\n")
    ))
}

/// The page a deep-linked reader starts at: the goal's latest page in site order.
pub fn deep_start(lens: &Lens, goal: &Goal) -> String {
    lens.pages
        .iter()
        .rev()
        .find(|(c, _)| goal.pages.contains(&c.id))
        .map(|(c, _)| c.id.clone())
        .unwrap_or_else(|| HOME.to_string())
}

pub fn read_task(
    project: &Project,
    lens: &Lens,
    task: &str,
    goal: &Goal,
    persona: &Persona,
    start: &str,
) -> String {
    let start = lens.handle(start).unwrap_or(HOME);
    let arrival = if start == HOME {
        "You arrive at the wiki's front page."
    } else {
        "You arrive on a page deep inside the wiki, as if from a search engine."
    };
    format!(
        "## The reader\n\n{knows}\n\n## The question\n\n{question}\n\n## Reading the wiki\n\n\
         {arrival} Run these in the project directory:\n\n\
         - `exegenius read {task} {start}` opens the first page. `exegenius read {task} <page>` \
         opens any page by its handle: links look like `[text](handle)`, and `{home}` is the front \
         page.\n\
         - `exegenius search {task} <words>` lists pages containing all the words.\n\n\
         You may open at most {budget} pages (opening a page again counts). Everything you open \
         is logged.\n",
        knows = persona.knows.trim(),
        question = lens.rename(&goal.question),
        home = HOME,
        budget = project.config.simulation.budget,
    )
}

pub fn baseline_task(lens: &Lens, goal: &Goal, persona: &Persona) -> String {
    format!(
        "## The reader\n\n{}\n\n## The question\n\n{}\n",
        persona.knows.trim(),
        lens.rename(&goal.question)
    )
}

/// Open a page for a pending reader task: check the budget, log the step, return the page.
pub fn read(project: &Project, lens: &Lens, task: &str, handle: &str) -> Result<String> {
    let mut trail = load_trail(project, task)?;
    let budget = project.config.simulation.budget as usize;
    if trail.path.len() >= budget {
        return Err(Error::Invalid(format!(
            "page budget spent ({budget} pages): write your answer now, or give up"
        )));
    }
    let page = lens.page(handle).ok_or_else(|| {
        Error::Invalid(format!(
            "no page `{handle}`: use a handle from a link, a search result, or `{HOME}`"
        ))
    })?;
    trail
        .path
        .push(lens.id(handle).expect("served handle").to_string());
    save_trail(project, task, &trail)?;
    Ok(format!(
        "{page}\n---\npage {} of {budget}\n",
        trail.path.len()
    ))
}

/// Search for a pending reader task: log the query, return the matches as seen.
pub fn search(project: &Project, lens: &Lens, task: &str, query: &str) -> Result<String> {
    let mut trail = load_trail(project, task)?;
    let budget = project.config.simulation.budget as usize;
    if trail.searches.len() >= budget {
        return Err(Error::Invalid(format!(
            "search budget spent ({budget} searches): write your answer now, or give up"
        )));
    }
    let hits = lens.search(query);
    trail.searches.push(Search {
        query: query.to_string(),
        found: hits.iter().map(|(id, _, _)| id.clone()).collect(),
    });
    save_trail(project, task, &trail)?;
    if hits.is_empty() {
        return Ok(format!("no pages contain all of: {query}\n"));
    }
    Ok(hits
        .iter()
        .map(|(id, title, snippet)| format!("[{title}]({})\n    {snippet}", lens.handles[id]))
        .collect::<Vec<_>>()
        .join("\n"))
}

/// Which reader task an answer belongs to.
pub struct Assignment<'a> {
    pub task: &'a str,
    pub hidden: Option<&'a Hidden>,
    pub round: u32,
    pub goal: usize,
    pub persona: &'a str,
    pub start: &'a str,
}

/// Check a reader's answer against its trail and turn it into a run.
pub fn apply_read(
    project: &Project,
    lens: &Lens,
    assignment: Assignment,
    answer: ReadAnswer,
) -> Result<Run> {
    let Assignment {
        task,
        hidden,
        round,
        goal,
        persona,
        start,
    } = assignment;
    let trail = load_trail(project, task)?;
    let visited: HashSet<&str> = trail.path.iter().map(String::as_str).collect();
    let mut errors = Vec::new();
    if trail.path.is_empty() {
        errors.push(format!(
            "no pages opened yet: start with `exegenius read {task} {}`",
            lens.handle(start).unwrap_or(HOME)
        ));
    }
    if answer.outcome == Outcome::Answered {
        if answer.answer.trim().is_empty() {
            errors.push("outcome is `answered` but the answer is empty".into());
        }
        if answer.citations.is_empty() {
            errors.push("an answer needs at least one citation from a page you opened".into());
        }
    }
    if answer.reason.trim().is_empty() {
        errors.push("`reason` is empty".into());
    }
    let mut check = |what: &str, handle: &str, quote: &str| -> Option<String> {
        let Some(id) = lens.id(handle) else {
            errors.push(format!("{what}: no page `{handle}`"));
            return None;
        };
        if !visited.contains(id) {
            errors.push(format!("{what}: you did not open `{handle}`"));
            return None;
        }
        let page = normalise(&lens.page(handle).unwrap_or_default());
        if !contains_quote(&page, quote) {
            errors.push(format!(
                "{what}: \"{}\" is not on `{handle}` word for word",
                quote.trim()
            ));
            return None;
        }
        Some(id.to_string())
    };
    let mut citations = Vec::new();
    for c in &answer.citations {
        if let Some(id) = check("citation", &c.page, &c.quote) {
            citations.push((id, lens.unrename(c.quote.trim())));
        }
    }
    let mut stuck = Vec::new();
    for s in &answer.stuck {
        if let Some(page) = check("stuck point", &s.page, &s.quote) {
            stuck.push(StuckPoint {
                page,
                quote: lens.unrename(s.quote.trim()),
                note: lens.unrename(s.note.trim()),
                level: Some(s.level),
            });
        }
    }
    let key_idea = answer.key_idea.as_ref().and_then(|k| {
        check("key idea", &k.page, &k.quote).map(|page| KeyIdea {
            page,
            statement: lens.unrename(k.statement.trim()),
            quote: lens.unrename(k.quote.trim()),
        })
    });
    if !errors.is_empty() {
        return Err(rejected(errors));
    }
    let said = format!(
        "{} {} {}",
        answer.answer,
        answer.reason,
        answer
            .stuck
            .iter()
            .map(|s| s.note.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    );
    Ok(Run {
        task: task.to_string(),
        round,
        goal,
        persona: persona.to_string(),
        start: start.to_string(),
        outcome: answer.outcome,
        path: trail.path,
        searches: trail.searches,
        answer: lens.unrename(answer.answer.trim()),
        citations,
        stuck,
        reason: lens.unrename(answer.reason.trim()),
        leaks: lens.leaks(&said),
        covered: None,
        contradicted: Vec::new(),
        wrong: Vec::new(),
        support: None,
        key_idea,
        hidden: hidden.cloned(),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn records_from_before_roles_and_levels_still_load() {
        let goal: Goal = serde_json::from_value(serde_json::json!({
            "question": "q", "kind": "transfer", "key_points": ["old point"], "pages": ["p"]
        }))
        .unwrap();
        assert_eq!(goal.key_points[0].role, Role::Claim);
        assert!(!goal.held_out);
        let stuck: StuckPoint = serde_json::from_value(serde_json::json!(["p", "q", "n"])).unwrap();
        assert_eq!((stuck.page.as_str(), stuck.level), ("p", None));
        let typed: std::result::Result<KeyPoint, _> =
            serde_json::from_value(serde_json::json!({"point": "x", "role": "claim", "extra": 1}));
        assert!(typed.is_err());
    }

    use super::*;

    #[test]
    fn terms_are_replaced_as_whole_words_keeping_case_and_plurals() {
        let terms = vec![
            ("cyclic convolution".to_string(), "vorapil".to_string()),
            ("DFT".to_string(), "kelimo".to_string()),
        ];
        assert_eq!(
            replace_terms(
                "Cyclic convolutions and a cyclic convolution; the DFT, not DFTX.",
                &terms,
                true
            ),
            "Vorapils and a vorapil; the Kelimo, not DFTX."
        );
        let mapped = map_text("The DFT of `DFT` and $DFT$.", |t| {
            replace_terms(t, &terms, true)
        });
        assert_eq!(mapped, "The Kelimo of `DFT` and $DFT$.");
    }

    #[test]
    fn nonce_words_alternate_consonants_and_vowels() {
        let word = nonce_word(&digest("dft"), 3);
        assert_eq!(word.len(), 7);
        assert!(word.chars().all(|c| c.is_ascii_lowercase()));
        assert_eq!(word, nonce_word(&digest("dft"), 3));
    }
}

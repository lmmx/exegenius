//! The argument: the plan of the source's proof, as a tree of proving intentions.
//!
//! Hamami & Morris (Review of Symbolic Logic 2021; Noûs 2024) describe a proof as the
//! execution of a plan: a tree whose nodes are intentions to show a claim, each carried out
//! by a method that splits it into sub-intentions, down to intentions to infer one step. The
//! written proof reports the plan's execution, so its inferred steps appear in the order of
//! a depth-first walk of the tree; understanding the proof means being able to trace each
//! step back to the main objective. This module holds that tree for the wiki's sources, as
//! `exegenius/argument.json` (the name avoids [`crate::plan`], which is the reading order).
//!
//! An agent reconstructs the tree from the sources before the pages are written. Code checks
//! what it can:
//!
//! 1. the root shows the target, and the target page presents it;
//! 2. the infer steps quote the sources word for word, in the order a depth-first walk
//!    visits them, except steps marked as reported out of order;
//! 3. every show step has a method, a motivation and sub-steps, and a motivation said to be
//!    explicit in the sources quotes them;
//! 4. every hypothesis is a concept the reader knows, or a page before the step's page;
//!    every symbol a step introduces occurs in the mathematics of its subtree's claims;
//! 5. every page is the module of one step or a hypothesis of some step.
//!
//! A motivation records whether the sources state it, leave it to be worked out, or not at
//! all, so a plausible plan cannot pass off reasoning the sources never give as theirs.

use std::collections::{HashMap, HashSet};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{Result, read_json, write_json};
use crate::graph::{Evidence, Graph};
use crate::plan::Plan;
use crate::project::{Project, Source, find_source};
use crate::symbols::{self, Binding};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Intention {
    /// Show a claim by a method that splits it into sub-steps.
    Show,
    /// Infer one claim in a single step the sources report.
    Infer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Provenance {
    /// The sources state the reason; quote them.
    Explicit,
    /// The sources do not state it, but a reader can work it out from what they say.
    Recoverable,
    /// The sources neither state it nor make it recoverable: you supply it.
    Reconstructed,
}

/// Why a method suits the claim it shows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Motivation {
    /// The reason, in one or two sentences.
    pub text: String,
    /// Whether the sources state the reason, leave it to be worked out, or neither.
    pub source: Provenance,
    /// For an explicit motivation, the sources' words, copied exactly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<Evidence>,
}

/// One step of the argument.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Node {
    /// A short label, unique in the tree, such as "S0" or "S2.1".
    pub id: String,
    pub intention: Intention,
    /// What the step shows or infers, in one sentence, with mathematics in LaTeX ($...$).
    pub claim: String,
    /// Ids of the concepts the step relies on: known concepts, or pages before the step's
    /// page.
    #[serde(default)]
    pub from: Vec<String>,
    /// For a show step: how the claim is shown, in a phrase ("reduce to a cyclic
    /// convolution and bound its cost").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// For a show step: why that method suits the claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motivation: Option<Motivation>,
    /// For a step whose claim has a feature a reader would find striking (a symmetry, a
    /// uniformity across cases, a surprising value or bound): that feature. The page should
    /// explain why it holds, from a matching feature of the method.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub salient: Option<String>,
    /// Symbols the step introduces, in force in its sub-steps.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adds: Vec<Binding>,
    /// Id of the page that presents this step and its sub-steps, if one does.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// For an infer step: the step as the sources report it, copied exactly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<Evidence>,
    /// For an infer step the sources report out of order, before or after the steps around
    /// it.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub deferred: bool,
    /// For a show step: its sub-steps, in the order the method takes them.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ArgumentAnswer {
    /// The main step: to show the target result.
    pub root: Node,
}

/// The accepted argument, and the pages it was made for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Argument {
    /// The planned pages the argument was made for, sorted.
    pub planned: Vec<String>,
    pub root: Node,
}

impl Node {
    /// This node and its descendants, depth first, left to right, each with its path of
    /// ancestors from the root.
    pub fn walk(&self) -> Vec<(Vec<&Node>, &Node)> {
        let mut out = Vec::new();
        let mut stack: Vec<(Vec<&Node>, &Node)> = vec![(Vec::new(), self)];
        while let Some((path, node)) = stack.pop() {
            for child in node.children.iter().rev() {
                let mut down = path.clone();
                down.push(node);
                stack.push((down, child));
            }
            out.push((path, node));
        }
        out
    }
}

impl Argument {
    /// The step page `id` presents, with its ancestors from the root.
    pub fn module(&self, id: &str) -> Option<(Vec<&Node>, &Node)> {
        self.root
            .walk()
            .into_iter()
            .find(|(_, n)| n.page.as_deref() == Some(id))
    }

    /// The steps that rely on concept `id`.
    pub fn users(&self, id: &str) -> Vec<&Node> {
        self.root
            .walk()
            .into_iter()
            .map(|(_, n)| n)
            .filter(|n| n.from.iter().any(|f| f == id))
            .collect()
    }
}

/// The lines of a "Where this fits" note for page `id`: each step from the main result down
/// to the one the page presents, or the steps that rely on the page's concept. `link` turns
/// a page id into a markdown link, or `None` for a page not shown.
pub fn trail(argument: &Argument, id: &str, link: &dyn Fn(&str) -> Option<String>) -> Vec<String> {
    let on = |n: &Node| {
        n.page
            .as_deref()
            .filter(|p| *p != id)
            .and_then(link)
            .map_or(String::new(), |l| format!(" ({l})"))
    };
    match argument.module(id) {
        Some((path, node)) if !path.is_empty() => {
            let mut lines: Vec<String> = path
                .iter()
                .enumerate()
                .map(|(i, a)| {
                    let how = a
                        .method
                        .as_deref()
                        .map_or(String::new(), |m| format!(" — how: {m}"));
                    format!("{}. {}{how}{}", i + 1, a.claim, on(a))
                })
                .collect();
            lines.push(format!("{}. **This page:** {}", path.len() + 1, node.claim));
            lines
        }
        Some(_) => Vec::new(),
        None => argument
            .users(id)
            .iter()
            .map(|n| format!("- Relied on to show: {}{}", n.claim, on(n)))
            .collect(),
    }
}

/// The top two levels of the argument, as markdown: the main claim, its method and
/// motivation, and the steps it splits into.
pub fn overview(argument: &Argument, link: &dyn Fn(&str) -> Option<String>) -> String {
    let root = &argument.root;
    let mut out = format!("**Main result:** {}\n\n", root.claim.trim());
    if let Some(method) = &root.method {
        out.push_str(&format!("**How:** {}", method.trim()));
        if let Some(m) = &root.motivation {
            out.push_str(&format!(" **Why:** {}", m.text.trim()));
        }
        out.push_str("\n\n");
    }
    for (i, child) in root.children.iter().enumerate() {
        let how = child
            .method
            .as_deref()
            .map_or(String::new(), |m| format!(" — how: {m}"));
        let on = child
            .page
            .as_deref()
            .and_then(link)
            .map_or(String::new(), |l| format!(" ({l})"));
        out.push_str(&format!("{}. {}{how}{on}\n", i + 1, child.claim));
    }
    out
}

fn sorted(plan: &Plan) -> Vec<String> {
    let mut pages = plan.order.clone();
    pages.sort();
    pages
}

/// The stored argument, whether or not it was made for the pages of `plan`.
pub fn load(project: &Project) -> Result<Option<Argument>> {
    let path = project.argument_path();
    if path.exists() {
        Ok(Some(read_json(&path)?))
    } else {
        Ok(None)
    }
}

/// The stored argument, if it was made for the pages of `plan`.
pub fn current(project: &Project, plan: &Plan) -> Result<Option<Argument>> {
    Ok(load(project)?.filter(|a| a.planned == sorted(plan)))
}

pub const ARGUMENT_INSTRUCTIONS: &str = "\
Reconstruct the plan of the argument the sources make for the target result, as a tree of \
steps. Writers of the wiki's pages are given their page's step, and readers are shown where \
each page fits, so the plan says what each page is for.

- The root is the intention to show the target result. Each `show` step names a `method` \
that splits its claim into sub-steps, and a `motivation`: why that method suits the claim. \
Its `children` are the sub-steps, in the order the method takes them. An `infer` step is a \
single step the sources report: give its `quote`, copied exactly, and no children.
- Read the plan top down, as a reader of the sources would: \"we show that\", \"it suffices \
to\", \"the key step is\" and section structure mark the steps. Go down to the level the \
pages need: every page below is either the module that presents one step (`page`) or a \
concept some step relies on (`from`). Deeper steps of a page's own proof can stop at an \
`infer` step quoting where the sources carry them out.
- The sources report the plan in order: the infer steps' quotes must appear in the sources in \
the order of a depth-first walk of the tree (submit checks it). Mark an infer step \
`deferred` if the sources report it out of order.
- `motivation.source`: `explicit` if the sources state why the method is used (quote them), \
`recoverable` if a reader could work it out from what they say, `reconstructed` if you supply \
it. Do not mark a motivation explicit unless you can quote it. Writers are told to give \
readers every reconstructed motivation, since the sources will not.
- `from`: the concepts a step relies on, as ids: known concepts, or pages that come before \
the step's page in the reading order below. A step's page is its own `page` or the nearest \
ancestor's.
- `salient`: for a step whose claim has a feature a reader would find striking (a symmetry, \
a uniformity across cases, a surprising value or bound), that feature. A proof explains a \
result only relative to such a feature (Lange), so its page is asked to explain why the \
feature holds. Most steps have none.
- `adds`: symbols a step introduces for its sub-steps, each occurring in the mathematics \
($...$) of the claims below it, with a display name, meaning and relation as for concept \
names.
- The target page presents the root. Give each other page at most one step; a page that \
presents no step must be in some step's `from`.";

pub fn argument_task(project: &Project, graph: &Graph, plan: &Plan) -> Result<String> {
    let mut task = String::new();
    if let Some(target) = graph.get(graph.goal()) {
        task.push_str(&format!(
            "## Target\n\n`{}`: {}: {}\n\n",
            target.id, target.name, target.gloss
        ));
    }
    task.push_str(&format!(
        "## Reader\n\n{}\n\n## Pages, in reading order\n\n",
        crate::background::profile(project, graph)
    ));
    for id in &plan.order {
        let c = graph.get(id).expect("planned pages are concepts");
        task.push_str(&format!("- `{}`: {}: {}\n", c.id, c.name, c.gloss));
    }
    let known: Vec<String> = graph
        .concepts
        .iter()
        .filter(|c| c.known)
        .map(|c| format!("- `{}`: {}", c.id, c.name))
        .collect();
    if !known.is_empty() {
        task.push_str(&format!(
            "\n## Concepts the reader knows\n\n{}\n",
            known.join("\n")
        ));
    }
    task.push_str(&format!(
        "\n## Sources (read them from disk)\n\n{}\n",
        project.sources_listing()
    ));
    if let Some(previous) = load(project)? {
        let json = serde_json::to_string_pretty(&previous.root).unwrap_or_default();
        task.push_str(&format!(
            "\n## Previous plan (revise it: the pages have changed since)\n\n```json\n{json}\n```\n"
        ));
    }
    Ok(task)
}

/// Where a quote starts in the sources: (source index, byte offset in its normalised text).
fn position(sources: &[Source], e: &Evidence) -> Option<(usize, usize)> {
    let (index, source) = sources.iter().enumerate().find(|(_, s)| s.id == e.source)?;
    let at = source.normalised.find(&crate::text::normalise(&e.quote))?;
    Some((index, at))
}

/// Problems with a quote from the sources.
fn check_quote(sources: &[Source], e: &Evidence, what: &str, errors: &mut Vec<String>) {
    match find_source(sources, &e.source) {
        None => errors.push(format!("{what}: `{}` is not a source id", e.source)),
        Some(s) if s.locate(&e.quote).is_none() => errors.push(format!(
            "{what}: quote not found in source `{}` word for word: \"{}\"",
            e.source, e.quote
        )),
        Some(_) => {}
    }
}

/// Check an argument and store it. Returns warnings.
pub fn apply_argument(
    project: &Project,
    graph: &Graph,
    plan: &Plan,
    answer: ArgumentAnswer,
) -> Result<Vec<String>> {
    let (errors, warnings) = check(graph, plan, &project.sources, &answer.root);
    if !errors.is_empty() {
        return Err(crate::simulate::rejected(errors));
    }
    write_json(
        &project.argument_path(),
        &Argument {
            planned: sorted(plan),
            root: answer.root,
        },
    )?;
    Ok(warnings)
}

/// Errors and warnings for an argument rooted at `root`.
fn check(
    graph: &Graph,
    plan: &Plan,
    sources: &[Source],
    root: &Node,
) -> (Vec<String>, Vec<String>) {
    let nodes = root.walk();
    let position_of: HashMap<&str, usize> = plan
        .order
        .iter()
        .enumerate()
        .map(|(i, p)| (p.as_str(), i))
        .collect();
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // 1. The root.
    if root.intention != Intention::Show {
        errors.push("the root must be a `show` step".into());
    }
    if root.page.as_deref() != Some(graph.goal()) {
        errors.push(format!(
            "the root's page must be the target page `{}`",
            graph.goal()
        ));
    }

    let mut ids = HashSet::new();
    let mut pages: HashMap<&str, &str> = HashMap::new();
    for (path, n) in &nodes {
        let what = format!("step `{}`", n.id);
        if n.id.trim().is_empty() {
            errors.push(format!("a step with claim \"{}\" has no id", n.claim));
        } else if !ids.insert(n.id.as_str()) {
            errors.push(format!("{what} appears twice"));
        }
        if n.claim.trim().is_empty() {
            errors.push(format!("{what} has no claim"));
        }
        // 3. Structure.
        match n.intention {
            Intention::Show => {
                if n.children.is_empty() {
                    errors.push(format!(
                        "{what} is a show step with no sub-steps; make it an infer step with \
                         a quote, or give its sub-steps"
                    ));
                }
                if n.method.as_deref().is_none_or(|m| m.trim().is_empty()) {
                    errors.push(format!("{what} has no method"));
                }
                match &n.motivation {
                    None => errors.push(format!("{what} has no motivation")),
                    Some(m) => {
                        if m.text.trim().is_empty() {
                            errors.push(format!("{what}: the motivation is empty"));
                        }
                        match (&m.quote, m.source) {
                            (Some(q), _) => check_quote(
                                sources,
                                q,
                                &format!("{what}'s motivation"),
                                &mut errors,
                            ),
                            (None, Provenance::Explicit) => errors.push(format!(
                                "{what}: an explicit motivation needs a quote from the sources"
                            )),
                            (None, _) => {}
                        }
                    }
                }
            }
            Intention::Infer => {
                if !n.children.is_empty() {
                    errors.push(format!(
                        "{what} is an infer step with sub-steps; make it a show step"
                    ));
                }
                match &n.quote {
                    None => errors.push(format!(
                        "{what} is an infer step with no quote from the sources"
                    )),
                    Some(q) => check_quote(sources, q, &what, &mut errors),
                }
            }
        }
        // 5. Modules.
        if let Some(page) = &n.page {
            if !position_of.contains_key(page.as_str()) {
                errors.push(format!("{what}: `{page}` is not a page of this wiki"));
            } else if let Some(other) = pages.insert(page, &n.id) {
                errors.push(format!(
                    "`{page}` presents both step `{other}` and {what}; give a page one step"
                ));
            }
        }
        // 4. Hypotheses, against the step's page: its own or its nearest ancestor's.
        let page = std::iter::once(*n)
            .chain(path.iter().rev().copied())
            .find_map(|a| a.page.as_deref())
            .and_then(|p| position_of.get(p).map(|&i| (p, i)));
        for f in &n.from {
            match graph.get(f) {
                None => errors.push(format!("{what}: `{f}` in `from` is not a concept id")),
                Some(c) if c.known => {}
                Some(c) => match (position_of.get(c.id.as_str()), page) {
                    (None, _) => errors.push(format!(
                        "{what}: `{f}` in `from` is neither known nor a page of this wiki"
                    )),
                    (Some(&i), Some((p, at))) if i >= at => errors.push(format!(
                        "{what}: `{f}` in `from` does not come before the step's page `{p}` in \
                         the reading order"
                    )),
                    _ => {}
                },
            }
        }
        let below: Vec<String> = n
            .walk()
            .iter()
            .flat_map(|(_, d)| symbols::math(&d.claim))
            .collect();
        for b in &n.adds {
            if !symbols::occurs(&b.symbol, &below) {
                errors.push(format!(
                    "{what}: `{}` in `adds` occurs in none of the claims at or below it",
                    b.symbol
                ));
            }
        }
        // A step's page reads before its ancestors' pages, as the target comes last.
        if let Some(page) = &n.page
            && let Some(&at) = position_of.get(page.as_str())
        {
            for a in path {
                if let Some(above) = &a.page
                    && position_of.get(above.as_str()).is_some_and(|&i| i < at)
                {
                    warnings.push(format!(
                        "step `{}` is presented on `{page}`, which comes after `{above}`, the \
                         page of the step it serves: readers meet the step's use before the \
                         step",
                        n.id
                    ));
                }
            }
        }
    }

    // 2. Report order.
    let mut last: Option<((usize, usize), &str)> = None;
    for (_, n) in &nodes {
        if n.intention != Intention::Infer || n.deferred {
            continue;
        }
        let Some(at) = n.quote.as_ref().and_then(|q| position(sources, q)) else {
            continue;
        };
        if let Some((before, id)) = last
            && at < before
        {
            errors.push(format!(
                "step `{}` quotes the sources before step `{id}`, which comes earlier in the \
                 plan: the sources report the plan depth first, so reorder the steps, or mark \
                 `{}` deferred if the sources report it out of order",
                n.id, n.id
            ));
        }
        last = Some((at, &n.id));
    }

    let used: HashSet<&str> = nodes
        .iter()
        .flat_map(|(_, n)| n.page.iter().chain(&n.from).map(String::as_str))
        .collect();
    let unused: Vec<&str> = plan
        .order
        .iter()
        .map(String::as_str)
        .filter(|p| !used.contains(p))
        .collect();
    if !unused.is_empty() {
        errors.push(format!(
            "pages that neither present a step nor appear in a step's `from`: {}",
            unused.join(", ")
        ));
    }

    (errors, warnings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::tests::{concept, graph};

    fn quote(text: &str) -> Option<Evidence> {
        Some(Evidence {
            source: "s".into(),
            quote: text.into(),
        })
    }

    fn infer(id: &str, text: &str) -> Node {
        Node {
            id: id.into(),
            intention: Intention::Infer,
            claim: format!("claim {id}"),
            from: vec![],
            method: None,
            motivation: None,
            salient: None,
            adds: vec![],
            page: None,
            quote: quote(text),
            deferred: false,
            children: vec![],
        }
    }

    fn show(id: &str, page: Option<&str>, children: Vec<Node>) -> Node {
        Node {
            id: id.into(),
            intention: Intention::Show,
            claim: format!("claim {id} with $T$"),
            from: vec![],
            method: Some("split".into()),
            motivation: Some(Motivation {
                text: "it works".into(),
                source: Provenance::Reconstructed,
                quote: None,
            }),
            salient: None,
            adds: vec![],
            page: page.map(str::to_string),
            quote: None,
            deferred: false,
            children,
        }
    }

    fn fixture() -> (Graph, Plan, Vec<Source>) {
        let g = graph(
            "t",
            vec![
                concept("a", &[], false),
                concept("b", &["a"], false),
                concept("t", &["b"], false),
                concept("k", &[], true),
            ],
        );
        let plan = Plan {
            order: vec!["a".into(), "b".into(), "t".into()],
        };
        let sources = vec![Source::new(
            "s",
            None,
            None,
            std::path::Path::new("s.txt"),
            "first step. second step. third step.",
        )];
        (g, plan, sources)
    }

    #[test]
    fn a_sound_argument_passes() {
        let (g, plan, sources) = fixture();
        let mut root = show(
            "S0",
            Some("t"),
            vec![
                show("S1", Some("b"), vec![infer("S1.1", "first step")]),
                infer("S2", "second step"),
            ],
        );
        root.from = vec!["a".into(), "k".into()];
        root.adds = vec![Binding {
            symbol: "T".into(),
            name: "length".into(),
            meaning: "The length.".into(),
            relation: crate::symbols::Relation::Count,
        }];
        let (errors, warnings) = check(&g, &plan, &sources, &root);
        assert!(
            errors.is_empty() && warnings.is_empty(),
            "{errors:?} {warnings:?}"
        );
        let argument = Argument {
            planned: vec![],
            root,
        };
        let (path, node) = argument.module("b").unwrap();
        assert_eq!((path.len(), node.id.as_str()), (1, "S1"));
        let link = |id: &str| Some(format!("[{id}]({id}.md)"));
        assert_eq!(
            trail(&argument, "b", &link),
            vec![
                "1. claim S0 with $T$ — how: split ([t](t.md))",
                "2. **This page:** claim S1 with $T$"
            ]
        );
        assert_eq!(
            trail(&argument, "a", &link),
            vec!["- Relied on to show: claim S0 with $T$ ([t](t.md))"]
        );
    }

    #[test]
    fn out_of_order_steps_and_bad_structure_are_rejected() {
        let (g, plan, sources) = fixture();
        let mut explicit = show("S1", Some("b"), vec![infer("S1.1", "third step")]);
        explicit.motivation.as_mut().unwrap().source = Provenance::Explicit;
        explicit.from = vec!["t".into()];
        let mut root = show(
            "S0",
            Some("a"),
            vec![
                explicit,
                infer("S2", "first step"),
                show("S3", None, vec![]),
            ],
        );
        root.intention = Intention::Show;
        let (errors, _) = check(&g, &plan, &sources, &root);
        let has = |text: &str| errors.iter().any(|e| e.contains(text));
        assert!(
            has("the root's page must be the target page `t`"),
            "{errors:?}"
        );
        assert!(has("an explicit motivation needs a quote"), "{errors:?}");
        assert!(
            has("`t` in `from` does not come before the step's page `b`"),
            "{errors:?}"
        );
        assert!(
            has("step `S3` is a show step with no sub-steps"),
            "{errors:?}"
        );
        assert!(
            has("step `S2` quotes the sources before step `S1.1`"),
            "{errors:?}"
        );
        // Marking the out-of-order step deferred is allowed.
        root.children[1].deferred = true;
        let (errors, _) = check(&g, &plan, &sources, &root);
        assert!(
            !errors
                .iter()
                .any(|e| e.contains("quotes the sources before"))
        );
    }
}

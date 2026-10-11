//! Outline task: once the pages are final, group them into parts, fold minor pages into a
//! glossary, and write the front page.
//!
//! The agent proposes the parts and the glossary and writes the overview and part intros.
//! Code checks that every page is in exactly one part or the glossary and that no page in a
//! part comes before a part holding one of its prerequisites, then orders each part's pages
//! by the plan, so the outline's order (part by part) is a valid reading order. Glossary
//! entries are reference material: they sit outside the reading order, and links to them
//! lead to their entry on the glossary page.

use std::collections::{HashMap, HashSet};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result, read_json, write_json};
use crate::graph::Graph;
use crate::plan::Plan;
use crate::project::Project;
use crate::write::{fix_links, link};

pub const OUTLINE_INSTRUCTIONS: &str = "\
Organise the finished pages of an explanatory wiki into parts, and write its front page. The \
wiki explains a target result to a specific reader, one concept per page.

- Group the pages into parts that the reader works through in order, each a coherent stage \
of the explanation (for example: background, the main construction, the analysis). Aim for \
3 to 8 parts; a small wiki may need fewer.
- Put every page in exactly one part, or in the glossary. Each page's prerequisites (listed \
below) must be in the glossary, the same part or an earlier one; submit rejects an outline \
that breaks this. Pages within a part are shown in reading order automatically, so their \
order in your list does not matter.
- Glossary: minor pages that define a term in a few sentences rather than explain an idea, \
such as a short page on a piece of terminology that one or two other pages use. They move \
to a glossary page, out of the reading order; links to them still work. Keep every page that \
explains a method, construction, result or argument in a part. Never put the target in the \
glossary. The word count and the number of pages using each page are listed to help.
- If the argument's main steps are listed below, base the parts on them where the \
prerequisites allow: a part per main step, with the pages that present its sub-steps, and \
background parts for the concepts the steps rely on.
- Part titles: 2 to 5 words, plain text, no numbering.
- Part intros: 2 to 4 sentences on what the part covers and why the reader needs it for what \
comes next. Link pages as `[Name](id.md)`.
- Overview: the front page, 150 to 350 words, for the reader described below. Say what the \
target result is and why it matters, then the route the wiki takes to it, part by part, \
naming the key ideas and linking their pages as `[Name](id.md)`. No headings.
- Write for this reader: concrete and specific, with no filler and no rhetorical flourishes. \
Mathematics in LaTeX ($...$). Links to anything other than the pages listed are rejected; \
external URLs are allowed.";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OutlineAnswer {
    /// The front page, as markdown, with no headings.
    pub overview: String,
    /// The parts, in reading order.
    pub parts: Vec<Part>,
    /// Ids of minor pages to move to the glossary.
    #[serde(default)]
    pub glossary: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Part {
    /// Short title, plain text, no numbering.
    pub title: String,
    /// What the part covers and why the reader needs it, as markdown.
    pub intro: String,
    /// Ids of the pages in this part.
    pub pages: Vec<String>,
}

/// The accepted outline, with each part's pages in reading order, and the pages it covers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Outline {
    /// The planned pages the outline was made for, sorted.
    pub planned: Vec<String>,
    pub overview: String,
    pub parts: Vec<Part>,
    /// Pages folded into the glossary, in plan order.
    #[serde(default)]
    pub glossary: Vec<String>,
}

impl Outline {
    /// Whether the outline covers exactly the pages of `plan`.
    pub fn is_current(&self, plan: &Plan) -> bool {
        self.planned == sorted(plan)
    }

    /// The pages in reading order: part by part.
    pub fn order(&self) -> Vec<String> {
        self.parts.iter().flat_map(|p| p.pages.clone()).collect()
    }
}

fn sorted(plan: &Plan) -> Vec<String> {
    let mut pages = plan.order.clone();
    pages.sort();
    pages
}

/// The stored outline, if it covers the pages of `plan`.
pub fn current(project: &Project, plan: &Plan) -> Result<Option<Outline>> {
    let path = project.outline_path();
    if !path.exists() {
        return Ok(None);
    }
    let outline: Outline = read_json(&path)?;
    Ok(outline.is_current(plan).then_some(outline))
}

pub fn outline_task(project: &Project, graph: &Graph, plan: &Plan) -> Result<String> {
    let mut task = String::new();
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
        let needs: Vec<&str> = c
            .requires
            .iter()
            .filter(|r| plan.contains(r))
            .map(String::as_str)
            .collect();
        let needs = if needs.is_empty() {
            String::new()
        } else {
            format!(" Needs: {}.", needs.join(", "))
        };
        let words = project
            .page_body(id)?
            .map_or(0, |b| b.split_whitespace().count());
        let used_by = plan
            .order
            .iter()
            .filter_map(|d| graph.get(d))
            .filter(|d| d.requires.contains(id))
            .count();
        task.push_str(&format!(
            "- `{}`: {} — {}{needs} ({words} words, used by {used_by})\n",
            c.id,
            link(c),
            c.gloss
        ));
    }
    if let Some(argument) = crate::argument::current(project, plan)? {
        task.push_str("\n## The argument's main steps\n\n");
        for step in &argument.root.children {
            let pages: Vec<String> = step
                .walk()
                .iter()
                .filter_map(|(_, n)| n.page.as_ref())
                .map(|p| format!("`{p}`"))
                .collect();
            let pages = if pages.is_empty() {
                String::new()
            } else {
                format!(" Pages: {}.", pages.join(", "))
            };
            task.push_str(&format!("- {}{pages}\n", step.claim));
        }
    }
    task.push_str(
        "\nThe page bodies are in `exegenius/pages/<id>.md`; read them if the glosses are \
         not enough to write the overview.\n",
    );
    Ok(task)
}

/// Check an outline and store it.
pub fn apply_outline(
    project: &Project,
    graph: &Graph,
    plan: &Plan,
    answer: OutlineAnswer,
) -> Result<Outline> {
    let pages: HashSet<String> = plan.order.iter().cloned().collect();
    let mut errors = Vec::new();
    if answer.overview.trim().is_empty() {
        errors.push("the overview is empty".to_string());
    }
    if answer.parts.is_empty() {
        errors.push("there are no parts".to_string());
    }
    let mut part_of: HashMap<&str, usize> = HashMap::new();
    for (i, part) in answer.parts.iter().enumerate() {
        let n = i + 1;
        if part.title.trim().is_empty() {
            errors.push(format!("part {n} has no title"));
        }
        if part.intro.trim().is_empty() {
            errors.push(format!("part {n} has no intro"));
        }
        if part.pages.is_empty() {
            errors.push(format!("part {n} has no pages"));
        }
        for page in &part.pages {
            if !pages.contains(page) {
                errors.push(format!("part {n}: `{page}` is not a page of this wiki"));
            } else if let Some(other) = part_of.insert(page, i) {
                errors.push(format!(
                    "`{page}` is in part {} and part {n}; put it in one part",
                    other + 1
                ));
            }
        }
    }
    let mut glossary: HashSet<&str> = HashSet::new();
    for page in &answer.glossary {
        if !pages.contains(page) {
            errors.push(format!("glossary: `{page}` is not a page of this wiki"));
        } else if *page == graph.target {
            errors.push(format!(
                "`{page}` is the target and cannot go in the glossary"
            ));
        } else if let Some(i) = part_of.get(page.as_str()) {
            errors.push(format!(
                "`{page}` is in part {} and the glossary; put it in one",
                i + 1
            ));
        } else if !glossary.insert(page) {
            errors.push(format!("`{page}` is in the glossary twice"));
        }
    }
    let missing: Vec<&str> = plan
        .order
        .iter()
        .filter(|p| !part_of.contains_key(p.as_str()) && !glossary.contains(p.as_str()))
        .map(String::as_str)
        .collect();
    if !missing.is_empty() {
        errors.push(format!(
            "pages in no part and not in the glossary: {}",
            missing.join(", ")
        ));
    }
    for (page, &i) in &part_of {
        let c = graph.get(page).expect("planned pages are concepts");
        for r in &c.requires {
            if let Some(&j) = part_of.get(r.as_str())
                && j > i
            {
                errors.push(format!(
                    "`{page}` (part {}) needs `{r}`, which is in a later part ({})",
                    i + 1,
                    j + 1
                ));
            }
        }
    }
    let texts = std::iter::once(("the overview".to_string(), &answer.overview)).chain(
        answer
            .parts
            .iter()
            .enumerate()
            .map(|(i, p)| (format!("part {} intro", i + 1), &p.intro)),
    );
    for (what, text) in texts {
        for dest in fix_links(text, &pages).1 {
            errors.push(format!(
                "{what}: link to \"{dest}\" is not a page of this wiki"
            ));
        }
    }
    if !errors.is_empty() {
        errors.sort();
        return Err(Error::Invalid(format!(
            "answer rejected:\n{}",
            errors
                .iter()
                .map(|e| format!("- {e}"))
                .collect::<Vec<_>>()
                .join("\n")
        )));
    }

    let position: HashMap<&str, usize> = plan
        .order
        .iter()
        .enumerate()
        .map(|(i, p)| (p.as_str(), i))
        .collect();
    let parts = answer
        .parts
        .into_iter()
        .map(|mut part| {
            part.title = part.title.trim().to_string();
            part.intro = part.intro.trim().to_string();
            part.pages.sort_by_key(|p| position[p.as_str()]);
            part
        })
        .collect();
    let mut glossary = answer.glossary;
    glossary.sort_by_key(|p| position[p.as_str()]);
    let outline = Outline {
        planned: sorted(plan),
        overview: answer.overview.trim().to_string(),
        parts,
        glossary,
    };
    write_json(&project.outline_path(), &outline)?;
    Ok(outline)
}

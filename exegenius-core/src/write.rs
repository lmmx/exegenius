//! Step 3: write each page body, in reading order.
//!
//! The model writes the body only. Code removes any link to a page that does not exist,
//! warns about prerequisites left unlinked, and later adds the title, prerequisites,
//! backlinks and sources from the graph (see [`crate::build`]).

use std::collections::HashSet;
use std::ops::Range;

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Log;
use crate::error::Result;
use crate::graph::{Concept, Graph};
use crate::model::{Model, ask};
use crate::plan::Plan;
use crate::project::Project;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PageBody {
    /// Markdown body of the page: no title heading, no prerequisites list, no sources section.
    pub body: String,
}

const WRITE_SYSTEM: &str = "\
You write one page of an explanatory wiki. The wiki explains a target result from source \
documents to a specific reader, one concept per page, in reading order.

Rules:
- Explain the page's concept so that this reader can follow it using only what they already \
understand (listed), what this page itself explains, and the linked prerequisite pages.
- Define any other technical term or symbol on this page, where it is first used.
- Link each prerequisite page the first time you rely on it, using the exact markdown link \
given. You may also link the other pages listed. Never link to anything else in the wiki.
- Stay accurate to the sources. State only what the sources support or what is standard \
background; do not invent results, constants or attributions.
- Be concrete and specific. Give a small worked example where it makes the idea clearer. \
No filler, no rhetorical flourishes, no announcing what the page will do.
- Write mathematics in LaTeX: $...$ inline, $$...$$ on its own lines for display.
- Do not write a title heading, a prerequisites list or a sources section: those are added \
automatically. Use ## subheadings only if the page is long.";

pub fn link(c: &Concept) -> String {
    format!("[{}]({}.md)", c.name, c.id)
}

fn names(concepts: &[&Concept]) -> String {
    concepts
        .iter()
        .map(|c| {
            if c.aliases.is_empty() {
                format!("- {}", c.name)
            } else {
                format!("- {} (also: {})", c.name, c.aliases.join(", "))
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn page_task(
    project: &Project,
    graph: &Graph,
    plan: &Plan,
    i: usize,
    previous: Option<&str>,
) -> String {
    let c = graph
        .get(&plan.order[i])
        .expect("planned pages are concepts");
    let mut task = format!(
        "# Reader\n\n{}\n\nThe reader already understands these concepts:\n\n{}\n\n",
        project.config.reader.knows.trim(),
        names(&plan.known_at(graph, i)),
    );
    task.push_str(&format!(
        "# This page\n\nConcept: {}\nGloss: {}\n",
        c.name, c.gloss
    ));
    if !c.aliases.is_empty() {
        task.push_str(&format!("Also called: {}\n", c.aliases.join(", ")));
    }
    let prerequisites: Vec<&Concept> = c.requires.iter().filter_map(|r| graph.get(r)).collect();
    let linked: Vec<String> = prerequisites
        .iter()
        .filter(|p| plan.contains(&p.id))
        .map(|p| format!("- {}: {}", link(p), p.gloss))
        .collect();
    if !linked.is_empty() {
        task.push_str(&format!(
            "\n# Prerequisite pages (link each on first use, exactly as written)\n\n{}\n",
            linked.join("\n")
        ));
    }
    let others: Vec<String> = plan
        .order
        .iter()
        .filter(|id| *id != &c.id && !c.requires.contains(id))
        .filter_map(|id| graph.get(id))
        .map(|o| format!("- {}", link(o)))
        .collect();
    if !others.is_empty() {
        task.push_str(&format!(
            "\n# Other pages you may link to\n\n{}\n",
            others.join("\n")
        ));
    }
    if !c.evidence.is_empty() {
        let quotes: Vec<String> = c
            .evidence
            .iter()
            .map(|e| format!("- {:?} ({})", e.quote, e.source))
            .collect();
        task.push_str(&format!(
            "\n# Evidence from the sources\n\n{}\n",
            quotes.join("\n")
        ));
    }
    if let (Some(notes), Some(previous)) = (graph.rewrite.get(&c.id), previous) {
        if !notes.is_empty() {
            let notes: Vec<String> = notes.iter().map(|n| format!("- {n}")).collect();
            task.push_str(&format!(
                "\n# Problems a reader audit found in the previous version (fix all of them)\n\n{}\n",
                notes.join("\n")
            ));
        }
        task.push_str(&format!("\n# Previous version\n\n{}\n", previous.trim()));
    }
    task.push_str("\nWrite the page body.");
    task
}

#[derive(Debug, Default)]
pub struct WriteReport {
    pub written: Vec<String>,
    pub warnings: Vec<String>,
}

/// Write every planned page that has no body yet or is queued for rewriting.
pub fn write_pages(
    project: &Project,
    graph: &mut Graph,
    plan: &Plan,
    model: &dyn Model,
    log: Log,
) -> Result<WriteReport> {
    let context = project.sources_context();
    let pages: HashSet<String> = plan.order.iter().cloned().collect();
    let mut report = WriteReport::default();
    for (i, id) in plan.order.iter().enumerate() {
        let previous = project.page_body(id)?;
        if previous.is_some() && !graph.rewrite.contains_key(id) {
            continue;
        }
        log(&format!("write [{}/{}] {id}", i + 1, plan.order.len()));
        let task = page_task(project, graph, plan, i, previous.as_deref());
        let page: PageBody = ask(model, WRITE_SYSTEM, Some(&context), &task)?;
        let (body, removed, linked) = fix_links(&page.body, &pages);
        for dest in removed {
            report.warnings.push(format!(
                "`{id}`: removed a link to {dest:?}, which is not a page"
            ));
        }
        let c = graph.get(id).expect("planned pages are concepts");
        for r in &c.requires {
            if pages.contains(r) && !linked.contains(r) {
                report.warnings.push(format!(
                    "`{id}`: prerequisite `{r}` is not linked in the body"
                ));
            }
        }
        project.save_page_body(id, &body)?;
        graph.rewrite.remove(id);
        project.save_graph(graph)?;
        report.written.push(id.clone());
    }
    Ok(report)
}

/// The page id a link destination points to, `Some("")` for an external or same-page link,
/// and `None` for a relative link that is not a page.
fn link_target(dest: &str) -> Option<&str> {
    if dest.contains("://") || dest.starts_with("mailto:") || dest.starts_with('#') {
        return Some("");
    }
    let path = dest.split('#').next().unwrap_or(dest);
    let path = path.strip_prefix("./").unwrap_or(path);
    path.strip_suffix(".md")
}

/// Replace links to anything other than a known page (or `index.md`, or an external URL)
/// with their text. Returns the fixed body, the removed destinations, and the linked pages.
pub fn fix_links(body: &str, pages: &HashSet<String>) -> (String, Vec<String>, HashSet<String>) {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_MATH;
    let mut removals: Vec<(Range<usize>, String)> = Vec::new();
    let mut removed = Vec::new();
    let mut linked = HashSet::new();
    let mut open: Option<(Range<usize>, String)> = None;
    for (event, range) in Parser::new_ext(body, options).into_offset_iter() {
        match event {
            Event::Start(Tag::Link { dest_url, .. }) => match link_target(&dest_url) {
                Some("") => {}
                Some(id) if id == "index" || pages.contains(id) => {
                    linked.insert(id.to_string());
                }
                _ if open.is_none() => {
                    removed.push(dest_url.to_string());
                    open = Some((range, String::new()));
                }
                _ => {}
            },
            Event::End(TagEnd::Link) => {
                if let Some(done) = open.take() {
                    removals.push(done);
                }
            }
            Event::Text(t) => {
                if let Some((_, text)) = open.as_mut() {
                    text.push_str(&t);
                }
            }
            Event::Code(t) => {
                if let Some((_, text)) = open.as_mut() {
                    text.push_str(&format!("`{t}`"));
                }
            }
            Event::InlineMath(t) => {
                if let Some((_, text)) = open.as_mut() {
                    text.push_str(&format!("${t}$"));
                }
            }
            _ => {}
        }
    }
    let mut fixed = body.to_string();
    for (range, text) in removals.into_iter().rev() {
        fixed.replace_range(range, &text);
    }
    (fixed, removed, linked)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn broken_links_become_text() {
        let pages: HashSet<String> = ["dft".to_string()].into();
        let body = "Use the [DFT](dft.md#def) and the [chirp $z$ trick](chirp.md), \
                    see [paper](https://example.org) or [home](index.md) or [x](notes.txt).";
        let (fixed, removed, linked) = fix_links(body, &pages);
        assert_eq!(
            fixed,
            "Use the [DFT](dft.md#def) and the chirp $z$ trick, \
             see [paper](https://example.org) or [home](index.md) or x."
        );
        assert_eq!(removed, vec!["chirp.md", "notes.txt"]);
        assert!(linked.contains("dft") && linked.contains("index"));
    }
}

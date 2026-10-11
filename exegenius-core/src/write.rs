//! Write tasks: one page body per needed concept, in reading order.
//!
//! The agent writes the body as markdown, and lists in a JSON file the symbols the page
//! binds (see [`crate::symbols`]). The two are separate files because LaTeX inside JSON
//! strings is easily mangled without an error: `\frac` in a JSON string is a form feed
//! followed by "rac". Code rejects links to anything that is not a page and bindings it
//! cannot find on the page, and later adds the title, prerequisites, backlinks and sources
//! from the graph (see [`crate::build`]).

use std::collections::HashSet;
use std::ops::Range;

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::graph::{Concept, Graph};
use crate::plan::Plan;
use crate::project::Project;
use crate::symbols::{self, PageBinding};

/// The structured part of a write answer; the body is the markdown answer file.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WriteAnswer {
    /// Every symbol the page binds: each symbol it introduces and says what it denotes.
    /// Leave out symbols listed as already bound, unless the page gives them a new meaning.
    /// Empty if the page introduces no symbols.
    pub symbols: Vec<PageBinding>,
}

pub const WRITE_INSTRUCTIONS: &str = "\
Write one page of an explanatory wiki. The wiki explains a target result from source \
documents to a specific reader, one concept per page, in reading order.

- Open with one or two sentences that say what the concept is and what it is for on the way \
to the target (the pages that build on it are listed below), so a reader arriving from a link \
knows at once why the page matters.
- Explain the page's concept so that this reader can follow it using only what they already \
understand (listed below), what this page itself explains, and the linked prerequisite pages.
- Define any other technical term on this page, where it is first used.
- Say what each symbol denotes where you introduce it: what it stands for, and for a size or \
count, what it counts (\"$r$, the number of complex coefficients in a record\", not \"a power \
of two $r$\"). A use in an example (\"take $r = 4$\") does not say what $r$ denotes. Symbols \
listed as already bound can be used without restating them: the page shows the reader their \
meanings. Using one of those letters with another meaning confuses readers: pick another \
letter, or say on the page that the letter changes meaning here.
- Link each prerequisite page the first time you rely on it, using the exact markdown link \
given. You may also link the other pages listed. Submit rejects links to anything else in \
the wiki; external URLs are allowed.
- Stay accurate to the sources: read the parts you need (search for the evidence quotes). \
State only what the sources support or what is standard background; do not invent results, \
constants or attributions.
- Back each specific claim taken from a source (a result, a constant, a construction step) \
with the source text it rests on, right after the claim: \
`<cite data-source=\"id\">text copied word for word from that source</cite>`. Readers see \
these as \"(see p. 5)\"-style references, hidden until they ask for them. Submit rejects \
text not found in the named source. Do not cite common background.
- Be concrete and specific. Give a small worked example where it makes the idea clearer. \
No filler, no rhetorical flourishes, no announcing what the page will do.
- Write mathematics in LaTeX: $...$ inline, $$...$$ on its own lines for display.
- Do not write a title heading, a prerequisites list or a sources section: those are added \
automatically. Use ## subheadings only if the page is long.";

pub fn link(c: &Concept) -> String {
    format!("[{}]({}.md)", c.name, c.id)
}

/// Concepts as a markdown list of names and aliases, with what the symbols in each name
/// denote.
pub(crate) fn names(concepts: &[&Concept]) -> String {
    concepts
        .iter()
        .map(|c| {
            let mut line = if c.aliases.is_empty() {
                format!("- {}", c.name)
            } else {
                format!("- {} (also: {})", c.name, c.aliases.join(", "))
            };
            for b in symbols::of_name(c) {
                line.push_str(&format!("\n  - {}", b.describe()));
            }
            line
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Planned pages with no body yet or queued for rewriting, in reading order.
pub fn needs_write(project: &Project, graph: &Graph, plan: &Plan) -> Result<Vec<String>> {
    let mut pages = Vec::new();
    for id in &plan.order {
        if graph.rewrite.contains_key(id) || project.page_body(id)?.is_none() {
            pages.push(id.clone());
        }
    }
    Ok(pages)
}

pub fn write_task(project: &Project, graph: &Graph, plan: &Plan, i: usize) -> Result<String> {
    let c = graph
        .get(&plan.order[i])
        .expect("planned pages are concepts");
    let mut task = format!(
        "## Reader\n\n{}\n\nThe reader already understands these concepts:\n\n{}\n\n",
        project.config.reader.knows.trim(),
        names(&plan.known_at(graph, i)),
    );
    task.push_str(&format!(
        "## This page\n\nConcept: {}\nGloss: {}\n",
        c.name, c.gloss
    ));
    if !c.aliases.is_empty() {
        task.push_str(&format!("Also called: {}\n", c.aliases.join(", ")));
    }
    if let Some(title) = &c.title {
        task.push_str(&format!("Heading: {title}\n"));
    }
    let own: Vec<String> = symbols::of_name(c)
        .map(|b| format!("- {}", b.describe()))
        .collect();
    if !own.is_empty() {
        task.push_str(&format!(
            "\nThe symbols in this page's heading denote (say so on the page):\n\n{}\n",
            own.join("\n")
        ));
    }
    let bound: Vec<String> = symbols::in_force(graph, plan, i)
        .iter()
        .filter(|b| matches!(b.origin, symbols::Origin::Page(_)))
        .map(|b| {
            let on = link(b.origin.concept());
            format!(
                "- ${}$ ({}), bound on {on}: {}",
                b.symbol,
                b.name,
                b.meaning.trim()
            )
        })
        .collect();
    if !bound.is_empty() {
        task.push_str(&format!(
            "\n## Symbols already bound on earlier pages\n\nThe symbols in the names of the \
             concepts the reader knows are listed with those concepts above.\n\n{}\n",
            bound.join("\n")
        ));
    }
    let linked: Vec<String> = c
        .requires
        .iter()
        .filter_map(|r| graph.get(r))
        .filter(|p| plan.contains(&p.id))
        .map(|p| format!("- {}: {}", link(p), p.gloss))
        .collect();
    if !linked.is_empty() {
        task.push_str(&format!(
            "\n## Prerequisite pages (link each on first use, exactly as written)\n\n{}\n",
            linked.join("\n")
        ));
    }
    let dependents: Vec<String> = plan
        .order
        .iter()
        .filter_map(|id| graph.get(id))
        .filter(|d| d.requires.contains(&c.id))
        .map(|d| format!("- {}: {}", d.name, d.gloss))
        .collect();
    if !dependents.is_empty() {
        task.push_str(&format!(
            "\n## Pages that build on this one\n\n{}\n",
            dependents.join("\n")
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
            "\n## Other pages you may link to\n\n{}\n",
            others.join("\n")
        ));
    }
    if !c.evidence.is_empty() {
        let quotes: Vec<String> = c
            .evidence
            .iter()
            .map(|e| format!("- \"{}\" ({})", e.quote, e.source))
            .collect();
        task.push_str(&format!(
            "\n## Evidence from the sources\n\n{}\n",
            quotes.join("\n")
        ));
    }
    task.push_str(&format!(
        "\n## Sources (read from disk as needed)\n\n{}\n",
        project.sources_listing()
    ));
    if let Some(previous) = project.page_body(&c.id)? {
        let notes = graph.rewrite.get(&c.id).cloned().unwrap_or_default();
        if !notes.is_empty() {
            let notes: Vec<String> = notes.iter().map(|n| format!("- {n}")).collect();
            task.push_str(&format!(
                "\n## Problems a reader audit found in the previous version (fix all of them)\n\n{}\n",
                notes.join("\n")
            ));
        }
        task.push_str(&format!(
            "\n## Previous version (revise it)\n\n{}\n",
            previous.trim()
        ));
    }
    Ok(task)
}

/// Check a page body and the symbols it binds, and store them. Returns warnings.
pub fn apply_write(
    project: &Project,
    graph: &mut Graph,
    plan: &Plan,
    page: &str,
    answer: &str,
    mut fields: WriteAnswer,
) -> Result<Vec<String>> {
    let body = strip_title(answer);
    if body.trim().is_empty() {
        return Err(Error::Invalid(
            "answer rejected: the page body is empty".into(),
        ));
    }
    let mut errors = crate::cite::check(body, &project.sources);
    errors.extend(symbols::check_page(body, &mut fields.symbols));
    if !errors.is_empty() {
        let list = errors
            .iter()
            .map(|e| format!("- {e}"))
            .collect::<Vec<_>>()
            .join("\n");
        return Err(Error::Invalid(format!("answer rejected:\n{list}")));
    }
    let pages: HashSet<String> = plan.order.iter().cloned().collect();
    let (_, removed, linked) = fix_links(body, &pages);
    if !removed.is_empty() {
        let list = removed
            .iter()
            .map(|d| format!("- link to \"{d}\" is not a page of this wiki"))
            .collect::<Vec<_>>()
            .join("\n");
        return Err(Error::Invalid(format!(
            "answer rejected:\n{list}\nLink only the pages listed in the task, or use plain text."
        )));
    }
    let mut warnings = Vec::new();
    let c = graph.get(page).expect("planned pages are concepts");
    for r in &c.requires {
        if pages.contains(r) && !linked.contains(r) {
            warnings.push(format!(
                "`{page}`: prerequisite `{r}` is not linked in the body"
            ));
        }
    }
    project.save_page_body(page, &format!("{}\n", body.trim()))?;
    graph.rewrite.remove(page);
    if fields.symbols.is_empty() {
        graph.bindings.remove(page);
    } else {
        graph.bindings.insert(page.to_string(), fields.symbols);
    }
    Ok(warnings)
}

/// Drop a leading `# Title` line, which the build adds itself.
fn strip_title(body: &str) -> &str {
    let trimmed = body.trim_start();
    match trimmed.strip_prefix("# ") {
        Some(rest) => rest.split_once('\n').map_or("", |(_, after)| after),
        None => trimmed,
    }
}

/// The page id a link destination points to, `Some("")` for an external or same-page link,
/// and `None` for a relative link that is not a page.
pub(crate) fn link_target(dest: &str) -> Option<&str> {
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

    #[test]
    fn leading_title_is_stripped() {
        assert_eq!(strip_title("\n# DFT\n\nBody."), "\nBody.");
        assert_eq!(strip_title("Body # not a title"), "Body # not a title");
    }
}

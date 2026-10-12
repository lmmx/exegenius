//! Write tasks: one page body per needed concept, in reading order.
//!
//! The agent writes the body as markdown, and lists in a JSON file the symbols the page
//! binds (see [`crate::symbols`]) and what the page uses each prerequisite page for (see
//! [`crate::relations`]). The two are separate files because LaTeX inside JSON
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
use crate::relations::{self, Edge};
use crate::symbols::{self, PageBinding};

/// The structured part of a write answer; the body is the markdown answer file.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WriteAnswer {
    /// Every symbol the page binds: each symbol it introduces and says what it denotes.
    /// Leave out symbols listed as already bound, unless the page gives them a new meaning.
    /// Empty if the page introduces no symbols.
    pub symbols: Vec<PageBinding>,
    /// One relation for each prerequisite page: what this page uses it for. Empty if the
    /// page has no prerequisite pages.
    pub relations: Vec<Edge>,
}

pub const WRITE_INSTRUCTIONS: &str = "\
Write one page of an explanatory wiki. The wiki explains a target result from source \
documents to a specific reader, one concept per page, in reading order.

- Write for this reader's next uncertainty: at each point, give what they need to follow, \
and no more. The task lists the step of the argument this page presents, its motivations, \
and the pages around it. They are material for deciding what to say, not a list to cover: \
explain the transitions this reader would find hard, and leave familiar ones to ordinary \
sentences.
- Open with a sentence that says what the concept is and what it is for on the way to the \
target.
- If the page presents a step of the argument, say how its method shows its claim, and give \
its main motivation in a sentence. Add another motivation only if the reader would be stuck \
without it.
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
- For each prerequisite page, give a relation: the prerequisite's id, a predicate, and the \
sentence that links or names it, copied exactly. Use the sentence where the page relies on \
it; do not add a sentence for the purpose. The page shows the relation as \"<this page's \
concept> *<predicate>* <prerequisite>\", so pick a predicate that makes that a true \
sentence, with this page's concept as subject: \"is reduced to\", \"computes its transforms \
by\", \"is based on\", \"follows from\", \"bounds the cost of\". Build the predicate from the \
sentence's own words (other forms of a word count: \"reduces\" for \"reduced\"), and pick one \
that says what the prerequisite does for this page: \"uses\" says little.
- Stay accurate to the sources. The paragraphs around the evidence are given below; read \
further in the sources only if they are not enough. State only what the sources support or \
what is standard background; do not invent results, constants or attributions.
- Cite the results and constants a reader may want to check, right after the claim: \
`<cite data-source=\"id\">text copied word for word from that source</cite>`. Readers see \
these as \"(see p. 5)\"-style references, hidden until they ask for them, so the sentence \
must read complete without the quoted words: write \"the cost is $O(n \\log n)$ <cite …>…\
</cite>\", not \"the paper states the cost as <cite …>…</cite>\". Submit rejects a citation \
right after \"that\", \"as\", \"states\", \"says\", \"is\", \"gives\", \"reads\" or a colon, \
and text not found in the named source. Do not cite common background.
- A citation says where a claim comes from; it does not explain it. If the page's concept is \
an identity, a construction or a definition, state it. Explain how the concept works and how \
it shows its claim. For a result the page uses from prior work, say what the result states \
and why it applies here, and cite the source for its proof: the page need not reproduce it.
- Keep the page short. The task gives its word limit, counted without mathematics and \
citations, and submit rejects a longer page. Give a small worked example where it makes the \
idea clearer. No filler, no rhetorical flourishes, no announcing what the page will do.
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

/// What the writer of page `id` is told about the argument: the step the page presents,
/// and where it sits, or the steps that rely on the page's concept.
fn argument_section(argument: &crate::argument::Argument, graph: &Graph, id: &str) -> String {
    use crate::argument::{Intention, Provenance};
    let on = |n: &crate::argument::Node| {
        n.page
            .as_deref()
            .and_then(|p| graph.get(p))
            .map_or(String::new(), |p| format!(" (on {})", link(p)))
    };
    let Some((path, node)) = argument.module(id) else {
        let users: Vec<String> = argument
            .users(id)
            .iter()
            .map(|n| format!("- {}{}", n.claim, on(n)))
            .collect();
        if users.is_empty() {
            return String::new();
        }
        return format!(
            "\n## Where the argument relies on this page\n\n{}\n",
            users.join("\n")
        );
    };
    let mut out = format!(
        "\n## This page in the argument\n\nThe step: {}\n",
        node.claim
    );
    if node.intention == Intention::Show {
        if let Some(method) = &node.method {
            out.push_str(&format!("How: {method}\n"));
        }
        if let Some(m) = &node.motivation {
            let source = match (m.source, &m.quote) {
                (Provenance::Explicit, Some(q)) => {
                    format!("the sources say so: \"{}\" ({})", q.quote, q.source)
                }
                (Provenance::Explicit, None) => "the sources say so".to_string(),
                (Provenance::Recoverable, _) => {
                    "the sources leave this for the reader to work out: spell it out".to_string()
                }
                (Provenance::Reconstructed, _) => {
                    "the sources do not say this: give the reader the reason".to_string()
                }
            };
            out.push_str(&format!("Why: {} ({source})\n", m.text.trim()));
        }
        if let Some(feature) = &node.salient {
            out.push_str(&format!(
                "Striking feature: {feature} (explain why it holds, from the method)\n"
            ));
        }
        if !node.children.is_empty() {
            let steps: Vec<String> = node
                .children
                .iter()
                .map(|c| format!("- {}{}", c.claim, on(c)))
                .collect();
            out.push_str(&format!(
                "\nIts sub-steps, in order:\n\n{}\n",
                steps.join("\n")
            ));
        }
    }
    if !path.is_empty() {
        let chain: Vec<String> = path
            .iter()
            .map(|a| {
                let how = a
                    .method
                    .as_deref()
                    .map_or(String::new(), |m| format!("; how: {m}"));
                format!("- {}{how}{}", a.claim, on(a))
            })
            .collect();
        out.push_str(&format!(
            "\nIt serves these steps, from the main result down:\n\n{}\n",
            chain.join("\n")
        ));
    }
    out
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
        crate::background::profile(project, graph),
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
    if let Some(argument) = crate::argument::current(project, plan)? {
        task.push_str(&argument_section(&argument, graph, &c.id));
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
        .map(|p| {
            format!(
                "- {}: {}\n  Shown as: {} *<predicate>* {}",
                link(p),
                p.gloss,
                c.name,
                p.name
            )
        })
        .collect();
    if !linked.is_empty() {
        task.push_str(&format!(
            "\n## Prerequisite pages (link each on first use, exactly as written, and give a \
             relation for each)\n\n{}\n",
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
            .map(|e| {
                let mut entry = format!("- \"{}\" ({})", e.quote, e.source);
                if let Some(paragraph) = passage(project, &e.source, &e.quote) {
                    entry.push_str(&format!("\n\n  > {paragraph}\n"));
                }
                entry
            })
            .collect();
        task.push_str(&format!(
            "\n## Evidence from the sources, with the paragraph around each quote\n\n{}\n",
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
        let words = prose_words(&previous);
        task.push_str(&format!(
            "\n## Previous version (revise it)\n\nIt has {words} words, not counting \
             mathematics and citations. Fix the problems by restructuring rather than \
             appending: submit rejects a rewrite over {} words. Pages that grow with every \
             repair stop being self-contained explanations of one concept. Submit also rejects \
             letters in the mathematics that the previous version did not use, unless the \
             symbols they make are bound: a new formula brings new symbols for the reader to \
             learn.\n\n{}\n",
            budget(words),
            previous.trim()
        ));
    } else {
        task.push_str(&format!(
            "\n## Length\n\nAt most {FIRST_DRAFT_WORDS} words, not counting mathematics and \
             citations: submit rejects a longer page.\n"
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
    let c = graph.get(page).expect("planned pages are concepts");
    let prerequisites: Vec<&Concept> = c
        .requires
        .iter()
        .filter(|r| plan.contains(r))
        .filter_map(|r| graph.get(r))
        .collect();
    let pages: HashSet<String> = plan.order.iter().cloned().collect();
    let (_, removed, linked) = fix_links(body, &pages);
    let mut errors: Vec<String> = removed
        .iter()
        .map(|d| {
            format!(
                "link to \"{d}\" is not a page of this wiki; link only the pages listed in \
                 the task, or use plain text"
            )
        })
        .collect();
    errors.extend(crate::cite::check(body, &project.sources));
    errors.extend(crate::cite::dangling(body));
    let words = prose_words(body);
    let previous = project.page_body(page)?;
    if let Some(previous) = &previous {
        let i = plan.order.iter().position(|p| p == page).expect("planned");
        let in_force = symbols::in_force(graph, plan, i);
        let bound: Vec<&str> = in_force
            .iter()
            .map(|b| b.symbol)
            .chain(symbols::of_name(c).map(|b| b.symbol))
            .chain(fields.symbols.iter().map(|b| b.symbol.as_str()))
            .collect();
        let new = symbols::introduced(previous, body, &bound);
        if !new.is_empty() {
            errors.push(format!(
                "the rewrite uses letters the previous version did not: {}. Bind each symbol \
                 they make in `symbols`, with the sentence that says what it denotes, or make \
                 the point without them",
                new.iter()
                    .map(|l| format!("${l}$"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }
    match &previous {
        Some(previous) => {
            let before = prose_words(previous);
            if words > budget(before) {
                errors.push(format!(
                    "the rewrite has {words} words, over the budget of {} (the previous version \
                     had {before}), not counting mathematics and citations: fix the problems by \
                     restructuring, and cut what this reader does not need",
                    budget(before)
                ));
            }
        }
        None if words > FIRST_DRAFT_WORDS => errors.push(format!(
            "the page has {words} words, over the limit of {FIRST_DRAFT_WORDS}, not counting \
             mathematics and citations: keep what this reader needs to follow the page"
        )),
        None => {}
    }
    errors.extend(symbols::check_page(body, &mut fields.symbols));
    if previous.is_some() {
        let lost: Vec<String> =
            crate::audit::relied_on(&crate::audit::latest_symbols(project)?, page)
                .into_iter()
                .filter(|(_, on)| plan.contains(on))
                .filter(|(s, _)| !fields.symbols.iter().any(|b| symbols::same(&b.symbol, s)))
                .map(|(s, on)| format!("${s}$ (used on `{on}`)"))
                .collect();
        if !lost.is_empty() {
            errors.push(format!(
                "the rewrite no longer binds symbols that later pages take their meaning from: \
                 {}. Keep each, with its meaning. If a note asks for another letter, keep this \
                 one and say on the page that it means something else on the earlier page",
                lost.join(", ")
            ));
        }
    }
    errors.extend(relations::check(
        body,
        &prerequisites,
        &mut fields.relations,
    ));
    if !errors.is_empty() {
        let list = errors
            .iter()
            .map(|e| format!("- {e}"))
            .collect::<Vec<_>>()
            .join("\n");
        return Err(Error::Invalid(format!("answer rejected:\n{list}")));
    }
    let warnings: Vec<String> = prerequisites
        .iter()
        .filter(|p| !linked.contains(&p.id))
        .map(|p| {
            format!(
                "`{page}`: prerequisite `{}` is not linked in the body",
                p.id
            )
        })
        .collect();
    let answers_steps = graph
        .rewrite
        .get(page)
        .is_some_and(|notes| notes.iter().any(|n| n.starts_with(crate::audit::STEP_NOTE)));
    match &previous {
        Some(previous) if answers_steps => {
            graph
                .added
                .insert(page.to_string(), crate::audit::added(previous, body));
        }
        _ => {
            graph.added.remove(page);
        }
    }
    project.save_page_body(page, &format!("{}\n", body.trim()))?;
    graph.rewrite.remove(page);
    if fields.symbols.is_empty() {
        graph.bindings.remove(page);
    } else {
        graph.bindings.insert(page.to_string(), fields.symbols);
    }
    if fields.relations.is_empty() {
        graph.relations.remove(page);
    } else {
        graph.relations.insert(page.to_string(), fields.relations);
    }
    Ok(warnings)
}

/// Most words of prose in a first draft. The 0.1 run's pages had a median of 221; the
/// 0.5.0 run's first drafts, with no limit, had a median of 621.
pub const FIRST_DRAFT_WORDS: usize = 350;

/// The most words a rewrite of a page of `words` words may have: a quarter more, and at
/// least 50 more for short pages.
fn budget(words: usize) -> usize {
    words + (words / 4).max(50)
}

/// Words of prose in a page body: citation marks and mathematics are not counted.
pub fn prose_words(body: &str) -> usize {
    let text = crate::cite::strip(body);
    Parser::new_ext(&text, Options::ENABLE_MATH)
        .filter_map(|event| match event {
            Event::Text(t) => Some(t.split_whitespace().count()),
            _ => None,
        })
        .sum()
}

/// The longest passage given around an evidence quote, in bytes.
const PASSAGE: usize = 1500;

/// The paragraph of source `source` around `quote`, cut to about [`PASSAGE`] bytes around
/// the quote when the paragraph is longer.
fn passage(project: &Project, source: &str, quote: &str) -> Option<String> {
    let s = project.sources.iter().find(|s| s.id == source)?;
    let text = crate::error::read(&project.root.join(&s.path)).ok()?;
    let at = crate::text::locate(&text, quote)?;
    let mut start = text[..at.start].rfind("\n\n").map_or(0, |i| i + 2);
    let mut end = text[at.end..]
        .find("\n\n")
        .map_or(text.len(), |i| at.end + i);
    if end - start > PASSAGE {
        let margin = PASSAGE.saturating_sub(at.len()) / 2;
        start = start.max(at.start.saturating_sub(margin));
        end = end.min(at.end + margin);
        while !text.is_char_boundary(start) {
            start += 1;
        }
        while !text.is_char_boundary(end) {
            end -= 1;
        }
    }
    let passage = text[start..end]
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    Some(passage)
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
    fn prose_words_leave_out_mathematics_and_citations() {
        let body = "Take $r = 4$ records.\n\n$$\\sum_{i=1}^{r} x_i$$\n\nThe sum is exact \
                    <cite data-source=\"p\">the sum is exact for all inputs</cite>.";
        assert_eq!(prose_words(body), 6);
    }

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

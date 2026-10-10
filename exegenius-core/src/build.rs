//! Write the docs tree and `zensical.toml`.
//!
//! Pages are plain markdown with no frontmatter. Everything around the body comes from the
//! graph and the outline: the title, a "before this page" note linking the prerequisites, a
//! diagram of the page's connections, the pages that use it, and the source quotes. Links
//! to pages carry the target's gloss as their title, which the theme shows as a tooltip,
//! and the first unlinked mention of an earlier page is linked (see [`crate::autolink`]).
//!
//! The outline may fold minor pages into a glossary: they are rendered as entries of
//! `docs/glossary.md`, and links to them lead to their entry. The notation task's symbols
//! become `docs/notation.md`.
//!
//! With an outline, the pages are grouped into parts: each part gets a landing page and a
//! section in the nav, and the front page carries the overview and a map of the parts.
//! Without one (`exegenius build` before the outline task), the nav is the flat reading
//! order.
//!
//! Files written by a previous build that are no longer generated are removed (tracked in
//! `exegenius/built.json`); other files in `docs/` are left alone.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::ops::Range;
use std::path::PathBuf;

use pulldown_cmark::{Event, LinkType, Options, Parser, Tag};
use toml_edit::{Array, DocumentMut, InlineTable, Item, Table, Value};

use crate::autolink::{self, autolink};
use crate::error::{Error, Result, read, read_json, write, write_json};
use crate::graph::{Concept, Graph};
use crate::notation::Notation;
use crate::outline::{Outline, Part};
use crate::plan::Plan;
use crate::project::{Project, find_source};
use crate::text::slug;
use crate::write::{fix_links, link_target};

const GLOSSARY: &str = "glossary.md";
const NOTATION: &str = "notation.md";

pub const MATHJAX_CONFIG: &str = "javascripts/mathjax.js";
pub const MATHJAX_CDN: &str = "https://unpkg.com/mathjax@3/es5/tex-mml-chtml.js";

const MATHJAX_JS: &str = r#"window.MathJax = {
  tex: {
    inlineMath: [["\\(", "\\)"]],
    displayMath: [["\\[", "\\]"]],
    processEscapes: true,
    processEnvironments: true
  },
  options: {
    ignoreHtmlClass: ".*|",
    processHtmlClass: "arithmatex"
  }
};

document$.subscribe(() => {
  MathJax.startup.output.clearCache()
  MathJax.typesetClear()
  MathJax.texReset()
  MathJax.typesetPromise()
})
"#;

/// Theme features set when `zensical.toml` has none: section landing pages, breadcrumbs,
/// previous/next links in reading order, and tooltips for link titles.
pub const FEATURES: &[&str] = &[
    "content.tooltips",
    "navigation.footer",
    "navigation.indexes",
    "navigation.instant",
    "navigation.instant.progress",
    "navigation.path",
    "navigation.sections",
    "navigation.top",
    "navigation.tracking",
    "search.highlight",
    "toc.follow",
];

#[derive(Debug, Default)]
pub struct BuildReport {
    pub pages: Vec<String>,
    /// Planned pages with no body yet (run `write`).
    pub missing: Vec<String>,
    /// Number of parts, if the pages have an outline.
    pub parts: Option<usize>,
    /// Pages folded into the glossary.
    pub glossary: usize,
    /// Entries on the notation page.
    pub symbols: usize,
    /// The simulation report, if simulated readers have run.
    pub report: Option<PathBuf>,
    /// Files from a previous build that were removed.
    pub removed: Vec<PathBuf>,
    pub warnings: Vec<String>,
}

/// One entry of the nav: a page, or a part with its landing page and pages.
#[derive(Debug, Clone, PartialEq)]
pub enum NavItem {
    Page {
        name: String,
        file: String,
    },
    Section {
        title: String,
        index: String,
        pages: Vec<(String, String)>,
    },
}

pub fn title(project: &Project, graph: &Graph) -> String {
    project
        .config
        .title
        .clone()
        .or_else(|| graph.get(&graph.target).map(|c| c.name.clone()))
        .unwrap_or_else(|| "exegenius".to_string())
}

/// What rendering needs: the graph, the pages that have a body, and where links point.
struct Site<'a> {
    project: &'a Project,
    graph: &'a Graph,
    /// Pages with a body: the reading order, then the glossary entries.
    pages: Vec<&'a Concept>,
    built: HashSet<&'a str>,
    /// Pages folded into the glossary.
    glossary: HashSet<&'a str>,
}

impl Site<'_> {
    /// Markdown destination of a page, relative to `docs/`.
    fn href(&self, id: &str) -> String {
        if self.glossary.contains(id) {
            format!("glossary.md#{id}")
        } else {
            format!("{id}.md")
        }
    }

    /// URL of a rendered page, for diagrams. `prefix` leads from the rendered page's URL
    /// to the site root: generated pages are one directory deep with directory URLs, the
    /// front page is at the root.
    fn url(&self, id: &str, prefix: &str) -> String {
        if self.glossary.contains(id) {
            format!("{prefix}glossary/#{id}")
        } else {
            format!("{prefix}{id}/")
        }
    }

    /// A markdown link to a page, with the gloss as its title. `prefix` leads from the
    /// linking file's directory to `docs/`.
    fn titled(&self, c: &Concept, prefix: &str) -> String {
        format!(
            "[{}]({prefix}{} \"{}\")",
            c.name,
            self.href(&c.id),
            attribute(&c.gloss)
        )
    }

    fn dependents(&self, id: &str) -> Vec<&Concept> {
        self.pages
            .iter()
            .copied()
            .filter(|d| d.requires.iter().any(|r| r == id))
            .collect()
    }

    fn prerequisites(&self, c: &Concept) -> (Vec<&Concept>, Vec<&Concept>) {
        c.requires
            .iter()
            .filter_map(|r| self.graph.get(r))
            .filter(|p| p.known || self.built.contains(p.id.as_str()))
            .partition(|p| !p.known)
    }
}

fn attribute(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace('"', "'")
}

/// Give every inline link to a page its gloss as title, point links to glossary pages at
/// their entry, and prefix relative `.md` links with `prefix`.
fn decorate(body: &str, site: &Site, prefix: &str) -> String {
    let mut edits: Vec<(Range<usize>, String)> = Vec::new();
    for (event, range) in Parser::new_ext(body, Options::ENABLE_MATH).into_offset_iter() {
        let Event::Start(Tag::Link {
            link_type: LinkType::Inline,
            dest_url,
            ..
        }) = event
        else {
            continue;
        };
        let Some(id) = link_target(&dest_url).filter(|id| !id.is_empty()) else {
            continue;
        };
        let source = &body[range.clone()];
        let Some(split) = source.rfind("](") else {
            continue;
        };
        let title = site
            .graph
            .get(id)
            .map(|c| format!(" \"{}\"", attribute(&c.gloss)))
            .unwrap_or_default();
        let dest = if site.glossary.contains(id) {
            site.href(id)
        } else {
            dest_url.to_string()
        };
        edits.push((
            range,
            format!("{}]({prefix}{dest}{title})", &source[..split]),
        ));
    }
    let mut out = body.to_string();
    for (range, text) in edits.into_iter().rev() {
        out.replace_range(range, &text);
    }
    out
}

/// A mermaid flowchart of `nodes` and the edges among them; nodes link to their pages.
/// `prefix` is as for [`Site::url`].
fn diagram(
    site: &Site,
    nodes: &[&Concept],
    edges: &[(usize, usize)],
    current: Option<usize>,
    prefix: &str,
) -> String {
    let mut out = String::from("```mermaid\nflowchart LR\n");
    for (i, c) in nodes.iter().enumerate() {
        out.push_str(&format!("  n{i}[\"{}\"]\n", c.name.replace('"', "#quot;")));
        if current == Some(i) {
            out.push_str(&format!("  class n{i} current\n"));
        } else {
            out.push_str(&format!(
                "  click n{i} href \"{}\" \"{}\"\n",
                site.url(&c.id, prefix),
                attribute(&c.gloss)
            ));
        }
    }
    for (a, b) in edges {
        out.push_str(&format!("  n{a} --> n{b}\n"));
    }
    if current.is_some() {
        out.push_str("  classDef current stroke-width:3px,font-weight:bold\n");
    }
    out.push_str("```\n");
    out
}

/// Edges among `nodes` from each node's prerequisites.
fn edges_among(nodes: &[&Concept]) -> Vec<(usize, usize)> {
    let index: HashMap<&str, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, c)| (c.id.as_str(), i))
        .collect();
    let mut edges = Vec::new();
    for (b, c) in nodes.iter().enumerate() {
        for r in &c.requires {
            if let Some(&a) = index.get(r.as_str()) {
                edges.push((a, b));
            }
        }
    }
    edges
}

fn list(items: &[String]) -> String {
    match items {
        [] => String::new(),
        [one] => one.clone(),
        [init @ .., last] => format!("{} and {last}", init.join(", ")),
    }
}

fn indent(text: &str) -> String {
    text.lines()
        .map(|l| {
            if l.is_empty() {
                String::new()
            } else {
                format!("    {l}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_page(site: &Site, c: &Concept, body: &str) -> String {
    let mut out = format!("# {}\n\n", c.name);
    let (pages, known) = site.prerequisites(c);
    if !pages.is_empty() || !known.is_empty() {
        let mut note = Vec::new();
        if !pages.is_empty() {
            let links: Vec<String> = pages.iter().map(|p| site.titled(p, "")).collect();
            note.push(format!("This page builds on {}.", list(&links)));
        }
        if !known.is_empty() {
            let names: Vec<String> = known.iter().map(|k| k.name.clone()).collect();
            note.push(format!("Assumed known: {}.", list(&names)));
        }
        out.push_str(&format!(
            "!!! info \"Before this page\"\n\n    {}\n\n",
            note.join(" ")
        ));
    }
    out.push_str(&decorate(body.trim(), site, ""));
    out.push('\n');

    let used_by = site.dependents(&c.id);
    if !pages.is_empty() || !used_by.is_empty() {
        let mut nodes: Vec<&Concept> = pages.clone();
        nodes.push(c);
        let me = nodes.len() - 1;
        nodes.extend(used_by.iter().copied());
        let edges: Vec<(usize, usize)> = (0..me)
            .map(|i| (i, me))
            .chain((me + 1..nodes.len()).map(|i| (me, i)))
            .collect();
        out.push_str("\n## Connections\n\n");
        out.push_str(&diagram(site, &nodes, &edges, Some(me), "../"));
        if !used_by.is_empty() {
            out.push_str("\nUsed by:\n\n");
            for d in &used_by {
                out.push_str(&format!("- {} — {}\n", site.titled(d, ""), d.gloss));
            }
        }
    }
    if !c.evidence.is_empty() {
        let mut quotes = Vec::new();
        for e in &c.evidence {
            let cite = find_source(&site.project.sources, &e.source)
                .map(|s| s.citation())
                .unwrap_or_else(|| e.source.clone());
            quotes.push(format!("> {}\n>\n> — {cite}", e.quote.trim()));
        }
        out.push_str(&format!(
            "\n??? quote \"From the sources\"\n\n{}\n",
            indent(&quotes.join("\n\n"))
        ));
    }
    out
}

fn render_part(site: &Site, part: &Part, pages: &[&Concept]) -> String {
    let mut out = format!(
        "# {}\n\n{}\n\n## Pages\n\n",
        part.title,
        decorate(&part.intro, site, "../")
    );
    for (i, c) in pages.iter().enumerate() {
        out.push_str(&format!(
            "{}. {} — {}\n",
            i + 1,
            site.titled(c, "../"),
            c.gloss
        ));
    }
    let edges = edges_among(pages);
    if !edges.is_empty() {
        out.push_str("\n## Map\n\nArrows point from a page to the pages that build on it.\n\n");
        out.push_str(&diagram(site, pages, &edges, None, "../"));
    }
    out
}

/// Shift a body's headings down a level, so they nest under its glossary entry.
fn demote(body: &str) -> String {
    let mut fenced = false;
    let mut out = Vec::new();
    for line in body.lines() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
        }
        if !fenced && line.starts_with('#') {
            out.push(format!("#{line}"));
        } else {
            out.push(line.to_string());
        }
    }
    out.join("\n")
}

fn render_glossary(site: &Site, entries: &[(&Concept, String)]) -> String {
    let mut out = String::from(
        "# Glossary\n\nShort definitions of terms the pages use, outside the reading order.\n",
    );
    for (c, body) in entries {
        out.push_str(&format!(
            "\n## {} {{ #{} }}\n\n{}\n",
            c.name,
            c.id,
            decorate(&demote(body.trim()), site, "")
        ));
        let used_by: Vec<String> = site
            .dependents(&c.id)
            .iter()
            .map(|d| site.titled(d, ""))
            .collect();
        if !used_by.is_empty() {
            out.push_str(&format!("\nUsed by {}.\n", list(&used_by)));
        }
    }
    out
}

fn render_notation(site: &Site, notation: &Notation) -> String {
    let mut out = String::from(
        "# Notation\n\nThe symbols used across the pages, in the order they are introduced.\n\n",
    );
    let position: HashMap<&str, usize> = site
        .pages
        .iter()
        .enumerate()
        .map(|(i, c)| (c.id.as_str(), i))
        .collect();
    let mut symbols: Vec<_> = notation.symbols.iter().collect();
    symbols.sort_by_key(|s| position.get(s.page.as_str()).copied().unwrap_or(usize::MAX));
    for symbol in symbols {
        let page = site
            .graph
            .get(&symbol.page)
            .filter(|c| site.built.contains(c.id.as_str()))
            .map(|c| format!(" — {}", site.titled(c, "")))
            .unwrap_or_default();
        out.push_str(&format!(
            "- ${}$ — {}{page}\n",
            symbol.symbol.trim(),
            symbol.meaning.trim()
        ));
    }
    out
}

/// Part-level edges, reduced to those not implied by others: part `a` precedes part `b`
/// if a page in `b` requires a page in `a`.
fn part_edges(site: &Site, outline: &Outline) -> Vec<(usize, usize)> {
    let part_of: HashMap<&str, usize> = outline
        .parts
        .iter()
        .enumerate()
        .flat_map(|(i, p)| p.pages.iter().map(move |id| (id.as_str(), i)))
        .collect();
    let n = outline.parts.len();
    let mut direct = vec![vec![false; n]; n];
    for c in &site.pages {
        let Some(&b) = part_of.get(c.id.as_str()) else {
            continue;
        };
        for r in &c.requires {
            if let Some(&a) = part_of.get(r.as_str())
                && a < b
            {
                direct[a][b] = true;
            }
        }
    }
    // Parts are in reading order, so every edge goes forward: reachability in one pass
    // from the last part back.
    let mut reach = direct.clone();
    for a in (0..n).rev() {
        for b in a + 1..n {
            if direct[a][b] {
                let further = reach[b].clone();
                for (r, f) in reach[a].iter_mut().zip(further) {
                    *r |= f;
                }
            }
        }
    }
    let mut edges = Vec::new();
    for a in 0..n {
        for b in a + 1..n {
            let implied = (a + 1..b).any(|k| reach[a][k] && reach[k][b]);
            if direct[a][b] && !implied {
                edges.push((a, b));
            }
        }
    }
    edges
}

fn render_index(site: &Site, outline: Option<(&Outline, &[String])>, notation: bool) -> String {
    let project = site.project;
    let graph = site.graph;
    let mut out = format!("# {}\n\n", title(project, graph));
    let target = graph.get(&graph.target);
    let knows = project.config.reader.knows.trim().trim_end_matches('.');
    let Some((outline, slugs)) = outline else {
        if let Some(target) = target {
            out.push_str(&format!(
                "This wiki explains {}. {}\n\n",
                site.titled(target, ""),
                target.gloss
            ));
        }
        out.push_str(&format!(
            "It is written for a reader who knows: {knows}.\n\n## Reading order\n\n"
        ));
        for (i, c) in site.pages.iter().enumerate() {
            out.push_str(&format!(
                "{}. {} — {}\n",
                i + 1,
                site.titled(c, ""),
                c.gloss
            ));
        }
        return finish_index(site, out);
    };

    out.push_str(&decorate(&outline.overview, site, ""));
    out.push_str("\n\n");
    let goal = target
        .map(|t| format!(" lead to {}", site.titled(t, "")))
        .unwrap_or_default();
    let mut reference = Vec::new();
    if !site.glossary.is_empty() {
        reference.push(format!("the [glossary]({GLOSSARY}) defines minor terms"));
    }
    if notation {
        reference.push(format!("the [notation]({NOTATION}) page lists the symbols"));
    }
    let reference = match reference.as_slice() {
        [] => String::new(),
        _ => format!(" For reference, {}.", list(&reference)),
    };
    out.push_str(&format!(
        "!!! tip \"How to read this wiki\"\n\n    Written for a reader who knows: {knows}. \
         The {} pages, in {} parts,{goal}. Read them in order with the links at the bottom \
         of each page, or start anywhere: each page opens with what it builds on. Hover \
         over a link to see a one-line definition.{reference}\n\n## Contents\n\n",
        site.pages.len() - site.glossary.len(),
        outline.parts.len(),
    ));
    let built: HashMap<&str, &Concept> = site.pages.iter().map(|c| (c.id.as_str(), *c)).collect();
    for (i, (part, slug)) in outline.parts.iter().zip(slugs).enumerate() {
        out.push_str(&format!(
            "### {}. [{}]({slug}/index.md)\n\n{}\n\n",
            i + 1,
            part.title,
            decorate(&part.intro, site, "")
        ));
        let links: Vec<String> = part
            .pages
            .iter()
            .filter_map(|id| built.get(id.as_str()))
            .map(|c| site.titled(c, ""))
            .collect();
        out.push_str(&format!("{}\n\n", links.join(" · ")));
    }
    if !site.glossary.is_empty() {
        let links: Vec<String> = site
            .pages
            .iter()
            .filter(|c| site.glossary.contains(c.id.as_str()))
            .map(|c| site.titled(c, ""))
            .collect();
        out.push_str(&format!(
            "### [Glossary]({GLOSSARY})\n\n{}\n\n",
            links.join(" · ")
        ));
    }
    if notation {
        out.push_str(&format!(
            "### [Notation]({NOTATION})\n\nThe symbols used across the pages, with where \
             each is introduced.\n\n"
        ));
    }
    let edges = part_edges(site, outline);
    if !edges.is_empty() {
        out.push_str("## Map of the parts\n\n```mermaid\nflowchart LR\n");
        for (i, (part, slug)) in outline.parts.iter().zip(slugs).enumerate() {
            out.push_str(&format!(
                "  p{i}[\"{}. {}\"]\n  click p{i} href \"{slug}/\"\n",
                i + 1,
                part.title.replace('"', "#quot;")
            ));
        }
        for (a, b) in edges {
            out.push_str(&format!("  p{a} --> p{b}\n"));
        }
        out.push_str("```\n\n");
    }
    finish_index(site, out)
}

fn finish_index(site: &Site, mut out: String) -> String {
    let assumed: Vec<&Concept> = site
        .graph
        .concepts
        .iter()
        .filter(|k| k.known && site.pages.iter().any(|p| p.requires.contains(&k.id)))
        .collect();
    if !assumed.is_empty() {
        out.push_str("\n## Assumed known\n\n");
        for k in assumed {
            out.push_str(&format!("- {} — {}\n", k.name, k.gloss));
        }
    }
    out.push_str("\n## Sources\n\n");
    for s in &site.project.sources {
        out.push_str(&format!("- {}\n", s.citation()));
    }
    out
}

/// Directory names for the parts' landing pages: slugs of the titles, made unique and
/// distinct from page ids (a part directory and a page would share a URL).
fn part_slugs(outline: &Outline, pages: &HashSet<String>) -> Vec<String> {
    let mut taken: HashSet<String> = pages.clone();
    taken.insert("index".into());
    taken.insert("javascripts".into());
    let mut slugs = Vec::new();
    for (i, part) in outline.parts.iter().enumerate() {
        let base = match slug(&part.title) {
            s if s.is_empty() => format!("part-{}", i + 1),
            s => s,
        };
        let mut candidate = base.clone();
        let mut n = 2;
        while taken.contains(&candidate) {
            candidate = format!("{base}-{n}");
            n += 1;
        }
        taken.insert(candidate.clone());
        slugs.push(candidate);
    }
    slugs
}

/// Page bodies as the site shows them.
pub struct Prepared<'a> {
    /// Pages with a body, in reading order and then the glossary entries.
    pub pages: Vec<(&'a Concept, String)>,
    /// Pages folded into the glossary.
    pub glossary: HashSet<&'a str>,
    /// Planned pages with no body yet.
    pub missing: Vec<String>,
    pub warnings: Vec<String>,
}

/// The page bodies in the order the site shows them, with links to pages no longer planned
/// turned into text and the first mentions of earlier pages linked.
pub fn prepare<'a>(
    project: &Project,
    graph: &'a Graph,
    plan: &Plan,
    outline: Option<&Outline>,
) -> Result<Prepared<'a>> {
    let planned: HashSet<String> = plan.order.iter().cloned().collect();
    let order: Vec<String> = match outline {
        Some(o) => o.order().into_iter().chain(o.glossary.clone()).collect(),
        None => plan.order.clone(),
    };
    let folded: HashSet<&str> = outline
        .iter()
        .flat_map(|o| o.glossary.iter().map(String::as_str))
        .collect();
    let mut prepared = Prepared {
        pages: Vec::new(),
        glossary: HashSet::new(),
        missing: Vec::new(),
        warnings: Vec::new(),
    };
    for id in &order {
        let Some(body) = project.page_body(id)? else {
            prepared.missing.push(id.clone());
            continue;
        };
        // Bodies were written against an earlier plan; drop links to pages no longer in it.
        let (body, removed, _) = fix_links(&body, &planned);
        for dest in removed {
            prepared.warnings.push(format!(
                "`{id}`: link to \"{dest}\" rendered as text (not a page)"
            ));
        }
        let c = graph.get(id).expect("planned pages are concepts");
        if folded.contains(id.as_str()) {
            prepared.glossary.insert(c.id.as_str());
        }
        prepared.pages.push((c, body));
    }
    // Autolinks point back in the reading order (or to the glossary): a mention of a page
    // the reader has not reached yet is informal use, and often of a different sense.
    let terms = autolink::terms(prepared.pages.iter().map(|(c, _)| *c), |c| {
        format!("{}.md", c.id)
    });
    let position: HashMap<String, usize> = prepared
        .pages
        .iter()
        .enumerate()
        .map(|(i, (c, _))| (c.id.clone(), i))
        .collect();
    for (c, body) in prepared.pages.iter_mut() {
        let earlier: Vec<autolink::Term> = terms
            .iter()
            .filter(|t| {
                prepared.glossary.contains(t.id.as_str()) || position[&t.id] < position[&c.id]
            })
            .cloned()
            .collect();
        *body = autolink(body, &c.id, &earlier);
    }
    Ok(prepared)
}

pub fn build(project: &Project, graph: &Graph, plan: &Plan) -> Result<BuildReport> {
    let docs = project.docs_dir();
    let mut report = BuildReport::default();
    let mut written: BTreeSet<String> = BTreeSet::new();
    let planned: HashSet<String> = plan.order.iter().cloned().collect();
    let outline = crate::outline::current(project, plan)?;
    let notation = crate::notation::current(project, plan)?;
    let prepared = prepare(project, graph, plan, outline.as_ref())?;
    report.missing = prepared.missing;
    report.warnings = prepared.warnings;
    let site = Site {
        project,
        graph,
        pages: prepared.pages.iter().map(|(c, _)| *c).collect(),
        built: prepared.pages.iter().map(|(c, _)| c.id.as_str()).collect(),
        glossary: prepared.glossary,
    };
    let mut entries = Vec::new();
    for (c, body) in prepared.pages {
        report.pages.push(c.id.clone());
        if site.glossary.contains(c.id.as_str()) {
            entries.push((c, body));
            continue;
        }
        let file = format!("{}.md", c.id);
        write(&docs.join(&file), &render_page(&site, c, &body))?;
        written.insert(file);
    }

    let entry = |c: &Concept| (c.name.clone(), format!("{}.md", c.id));
    let mut nav = Vec::new();
    let slugs = outline
        .as_ref()
        .map(|o| part_slugs(o, &planned))
        .unwrap_or_default();
    if let Some(outline) = &outline {
        report.parts = Some(outline.parts.len());
        for (part, slug) in outline.parts.iter().zip(&slugs) {
            let pages: Vec<&Concept> = site
                .pages
                .iter()
                .copied()
                .filter(|c| part.pages.contains(&c.id))
                .collect();
            let file = format!("{slug}/index.md");
            write(&docs.join(&file), &render_part(&site, part, &pages))?;
            written.insert(file.clone());
            nav.push(NavItem::Section {
                title: part.title.clone(),
                index: file,
                pages: pages.iter().map(|c| entry(c)).collect(),
            });
        }
    } else {
        for c in &site.pages {
            let (name, file) = entry(c);
            nav.push(NavItem::Page { name, file });
        }
    }
    if !entries.is_empty() {
        report.glossary = entries.len();
        write(&docs.join(GLOSSARY), &render_glossary(&site, &entries))?;
        written.insert(GLOSSARY.to_string());
        nav.push(NavItem::Page {
            name: "Glossary".into(),
            file: GLOSSARY.into(),
        });
    }
    if let Some(notation) = notation.as_ref().filter(|n| !n.symbols.is_empty()) {
        report.symbols = notation.symbols.len();
        write(&docs.join(NOTATION), &render_notation(&site, notation))?;
        written.insert(NOTATION.to_string());
        nav.push(NavItem::Page {
            name: "Notation".into(),
            file: NOTATION.into(),
        });
    }
    let index = render_index(
        &site,
        outline.as_ref().map(|o| (o, slugs.as_slice())),
        report.symbols > 0,
    );
    write(&docs.join("index.md"), &index)?;
    written.insert("index.md".to_string());

    let mathjax = docs.join(MATHJAX_CONFIG);
    if !mathjax.exists() {
        write(&mathjax, MATHJAX_JS)?;
    }

    let manifest = project.manifest_path();
    if manifest.exists() {
        let previous: Vec<String> = read_json(&manifest)?;
        for file in previous.into_iter().filter(|f| !written.contains(f)) {
            let path = docs.join(&file);
            if path.exists() {
                std::fs::remove_file(&path).map_err(|source| Error::Io {
                    path: path.clone(),
                    source,
                })?;
                report.removed.push(path);
            }
        }
    }
    write_json(&manifest, &written.iter().collect::<Vec<_>>())?;

    let simulation = crate::simulate::load(project)?;
    if !simulation.runs.is_empty() {
        let path = project.report_path();
        write(
            &path,
            &crate::report::report(&project.config, graph, &simulation),
        )?;
        report.report = Some(path);
    }

    let path = project.zensical_path();
    let existing = if path.exists() {
        Some(read(&path)?)
    } else {
        None
    };
    let (toml, warnings) = zensical_toml(existing.as_deref(), &title(project, graph), &nav)
        .map_err(|source| Error::Zensical {
            path: path.clone(),
            source,
        })?;
    report.warnings.extend(warnings);
    write(&path, &toml)?;
    Ok(report)
}

fn nav_array(items: &[NavItem]) -> Array {
    let page = |name: &str, file: &str| {
        let mut entry = InlineTable::new();
        entry.insert(name, Value::from(file));
        Value::from(entry)
    };
    let mut entries = Array::new();
    entries.push("index.md");
    for item in items {
        match item {
            NavItem::Page { name, file } => entries.push(page(name, file)),
            NavItem::Section {
                title,
                index,
                pages,
            } => {
                let mut children = Array::new();
                children.push(index.as_str());
                for (name, file) in pages {
                    children.push(page(name, file));
                }
                for child in children.iter_mut() {
                    child.decor_mut().set_prefix("\n    ");
                }
                children.set_trailing_comma(true);
                children.set_trailing("\n  ");
                let mut section = InlineTable::new();
                section.insert(title, Value::from(children));
                entries.push(section);
            }
        }
    }
    for entry in entries.iter_mut() {
        entry.decor_mut().set_prefix("\n  ");
    }
    entries.set_trailing_comma(true);
    entries.set_trailing("\n");
    entries
}

/// Create or update `zensical.toml`: set `project.nav`, add the MathJax scripts to
/// `project.extra_javascript`, and fill in `site_name`, `docs_dir` and the theme features
/// only if they are missing. Everything else in an existing file is kept as written.
pub fn zensical_toml(
    existing: Option<&str>,
    site_name: &str,
    nav: &[NavItem],
) -> std::result::Result<(String, Vec<String>), toml_edit::TomlError> {
    let mut doc: DocumentMut = existing.unwrap_or("").parse()?;
    let mut warnings = Vec::new();
    if !doc.contains_key("project") || !doc["project"].is_table() {
        doc["project"] = Item::Table(Table::new());
    }
    let project = doc["project"].as_table_mut().expect("project is a table");

    if !project.contains_key("site_name") {
        project["site_name"] = toml_edit::value(site_name);
    }
    match project.get("docs_dir").and_then(Item::as_str) {
        None => project["docs_dir"] = toml_edit::value("docs"),
        Some("docs") => {}
        Some(other) => warnings.push(format!(
            "zensical.toml sets docs_dir = \"{other}\", but exegenius writes to \"docs\""
        )),
    }
    project["nav"] = toml_edit::value(nav_array(nav));

    let scripts = project
        .entry("extra_javascript")
        .or_insert(toml_edit::value(Array::new()));
    match scripts.as_array_mut() {
        Some(array) => {
            for script in [MATHJAX_CONFIG, MATHJAX_CDN] {
                if !array.iter().any(|v| v.as_str() == Some(script)) {
                    array.push(script);
                }
            }
        }
        None => warnings.push(
            "zensical.toml extra_javascript is not an array; MathJax scripts not added".into(),
        ),
    }

    let theme = project.entry("theme").or_insert(Item::Table(Table::new()));
    match theme.as_table_like_mut() {
        Some(theme) if !theme.contains_key("features") => {
            theme.insert(
                "features",
                toml_edit::value(Array::from_iter(FEATURES.iter().copied())),
            );
        }
        Some(_) => {}
        None => warnings.push("zensical.toml theme is not a table; features not set".into()),
    }
    Ok((doc.to_string(), warnings))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nav() -> Vec<NavItem> {
        vec![
            NavItem::Page {
                name: "Chirp identity".into(),
                file: "chirp-identity.md".into(),
            },
            NavItem::Section {
                title: "Background".into(),
                index: "background/index.md".into(),
                pages: vec![("DFT".into(), "dft.md".into())],
            },
        ]
    }

    #[test]
    fn new_zensical_toml() {
        let (toml, warnings) = zensical_toml(None, "Below n log n", &nav()).unwrap();
        assert!(warnings.is_empty());
        let doc: toml::Table = toml::from_str(&toml).unwrap();
        let project = doc["project"].as_table().unwrap();
        assert_eq!(project["site_name"].as_str(), Some("Below n log n"));
        assert_eq!(project["docs_dir"].as_str(), Some("docs"));
        let nav = project["nav"].as_array().unwrap();
        assert_eq!(nav[0].as_str(), Some("index.md"));
        assert_eq!(nav[1]["Chirp identity"].as_str(), Some("chirp-identity.md"));
        let section = nav[2]["Background"].as_array().unwrap();
        assert_eq!(section[0].as_str(), Some("background/index.md"));
        assert_eq!(section[1]["DFT"].as_str(), Some("dft.md"));
        assert_eq!(project["extra_javascript"].as_array().unwrap().len(), 2);
        let features = project["theme"]["features"].as_array().unwrap();
        assert_eq!(features.len(), FEATURES.len());
    }

    #[test]
    fn existing_zensical_toml_keeps_user_settings() {
        let existing = "# mine\n[project]\nsite_name = \"Mine\"\nnav = [\"old.md\"]\n\
                        extra_javascript = [\"javascripts/mathjax.js\", \"x.js\"]\n\n\
                        [project.theme]\nfeatures = [\"navigation.instant\"]\n";
        let (toml, _) = zensical_toml(Some(existing), "Ignored", &nav()).unwrap();
        assert!(toml.starts_with("# mine\n"));
        let doc: toml::Table = toml::from_str(&toml).unwrap();
        let project = doc["project"].as_table().unwrap();
        assert_eq!(project["site_name"].as_str(), Some("Mine"));
        assert_eq!(project["nav"].as_array().unwrap().len(), 3);
        let scripts: Vec<&str> = project["extra_javascript"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str())
            .collect();
        assert_eq!(scripts, vec!["javascripts/mathjax.js", "x.js", MATHJAX_CDN]);
        assert_eq!(project["theme"]["features"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn links_get_glosses_prefixes_and_glossary_anchors() {
        let graph: Graph = serde_json::from_value(serde_json::json!({
            "target": "dft",
            "concepts": [
                {"id": "dft", "name": "DFT", "aliases": [], "known": false,
                 "gloss": "A \"linear\" map.", "requires": [], "evidence": []},
                {"id": "row", "name": "Row", "aliases": [], "known": false,
                 "gloss": "A block.", "requires": [], "evidence": []},
            ],
        }))
        .unwrap();
        let project = Project {
            root: PathBuf::from("."),
            config: crate::config::Config {
                title: None,
                target: "t".into(),
                rounds: 1,
                reader: crate::config::Reader { knows: "k".into() },
                sources: vec![],
                simulation: Default::default(),
                readers: vec![],
            },
            sources: vec![],
        };
        let site = Site {
            project: &project,
            graph: &graph,
            pages: graph.concepts.iter().collect(),
            built: ["dft", "row"].into(),
            glossary: ["row"].into(),
        };
        let body = "See [the DFT](dft.md#def), [a row](row.md), [home](index.md), \
                    [web](https://x.org) and `[code](dft.md)`.";
        assert_eq!(
            decorate(body, &site, "../"),
            "See [the DFT](../dft.md#def \"A 'linear' map.\"), \
             [a row](../glossary.md#row \"A block.\"), [home](../index.md), \
             [web](https://x.org) and `[code](dft.md)`."
        );
        assert_eq!(site.url("row", "../"), "../glossary/#row");
        assert_eq!(
            demote("# A\n```\n# not\n```\n## B"),
            "## A\n```\n# not\n```\n### B"
        );
    }

    #[test]
    fn part_slugs_avoid_pages_and_each_other() {
        let part = |title: &str| Part {
            title: title.into(),
            intro: "i".into(),
            pages: vec![],
        };
        let outline = Outline {
            planned: vec![],
            overview: String::new(),
            parts: vec![
                part("Background"),
                part("Background"),
                part("DFT"),
                part("∑"),
            ],
            glossary: vec![],
        };
        let pages: HashSet<String> = ["dft".to_string()].into();
        assert_eq!(
            part_slugs(&outline, &pages),
            vec!["background", "background-2", "dft-2", "part-4"]
        );
    }
}

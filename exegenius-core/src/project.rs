//! A project directory: configuration, sources, and the state exegenius keeps.
//!
//! ```text
//! exegenius.toml         configuration
//! sources/               source documents (paths given in exegenius.toml)
//! exegenius/graph.json   concept graph, symbols each page binds, candidate concepts,
//!                        queued rewrites
//! exegenius/state.json   task state: issued tasks, audit round, audited pages
//! exegenius/log.jsonl    run log: each task handed out and each submit (see runlog)
//! exegenius/tasks/       task files, their answer files, and a scratch directory per task
//! exegenius/pages/       page bodies
//! exegenius/audit.json   audit results, one record per round
//! exegenius/argument.json the plan of the sources' argument
//! exegenius/outline.json parts, glossary and front page
//! exegenius/overloads.json verdicts on letters bound to two meanings
//! exegenius/support.json labels on whether citation marks support their claims
//! exegenius/simulation.json simulated readers: goals, runs, grades
//! exegenius/trails/      pages each simulated reader opened
//! exegenius/report.md    how the simulated readers did
//! docs/, zensical.toml   build output
//! ```

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::config::{self, Config};
use crate::error::{Result, read, read_json, write, write_json};
use crate::graph::Graph;
use crate::text::normalise;

/// A loaded source document.
#[derive(Debug, Clone)]
pub struct Source {
    pub id: String,
    pub title: String,
    pub url: Option<String>,
    /// Path relative to the project directory.
    pub path: PathBuf,
    /// The text passed through [`normalise`], for verbatim quote checks.
    pub normalised: String,
    /// Offsets in `normalised` where each page starts, if the text has page breaks
    /// (form feeds, as `pdftotext` writes them).
    pages: Vec<usize>,
    /// The section headings, in order.
    sections: Vec<Heading>,
    /// Whether the headings are marked up (LaTeX sectioning commands or markdown `#`), as
    /// opposed to found by the numbered-line heuristic, which in text from `pdftotext` finds
    /// table-of-contents entries and equation lines and misses headings split over lines.
    marked_up: bool,
    /// Theorem-like statements: LaTeX environments declared with `\newtheorem`.
    statements: Vec<Statement>,
    /// Displayed equations: LaTeX `equation`, `align`, `gather`, `multline`, `eqnarray` and
    /// `displaymath` environments, starred or not.
    displays: Vec<Display>,
    /// For each non-blank line, where it starts in `normalised` and its line number.
    lines: Vec<(usize, usize)>,
    /// Lines in the text, blank ones included.
    line_count: usize,
}

/// A displayed equation in a source.
#[derive(Debug, Clone, PartialEq)]
pub struct Display {
    /// Where the display starts in the normalised text.
    pub offset: usize,
    /// Line number in the source, from 1.
    pub line: usize,
    /// Its LaTeX label, if it has one: a display the source refers back to.
    pub label: Option<String>,
}

/// A section heading in a source.
#[derive(Debug, Clone, PartialEq)]
pub struct Heading {
    /// Where the heading starts in the normalised text.
    pub offset: usize,
    /// Line number in the source, from 1.
    pub line: usize,
    /// 1 for a section, 2 for a subsection, and so on.
    pub level: usize,
    /// The heading, with its number if it has one ("§3.2 Gaussian resampling").
    pub title: String,
}

/// A theorem-like statement in a source: a theorem, lemma, definition and so on.
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// Where the statement starts in the normalised text.
    pub offset: usize,
    /// Line number in the source, from 1.
    pub line: usize,
    /// What it is, as the source names it ("Lemma").
    pub kind: String,
    /// Its LaTeX label, if it has one.
    pub label: Option<String>,
}

/// A section heading line: a markdown heading, or a numbered heading such as
/// "3.2 Gaussian resampling", with its level.
fn heading(line: &str) -> Option<(String, usize)> {
    if let Some(rest) = line.strip_prefix('#') {
        let title = rest.trim_start_matches('#').trim();
        let level = line.len() - line.trim_start_matches('#').len();
        return (!title.is_empty()).then(|| (title.to_string(), level));
    }
    let (number, title) = line.split_once(' ')?;
    let number = number.trim_end_matches('.');
    let numbered = !number.is_empty()
        && number
            .split('.')
            .all(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()));
    let title = title.trim();
    (numbered
        && line.chars().count() <= 80
        && title.chars().next().is_some_and(char::is_uppercase)
        && !title.ends_with('.'))
    .then(|| (format!("§{number} {title}"), number.split('.').count()))
}

/// How far LaTeX's section numbering has got: the counters of sections, subsections and
/// subsubsections, and whether `\appendix` has turned sections into letters.
#[derive(Default)]
struct Numbering {
    counters: [u32; 3],
    appendix: bool,
}

/// The text of a LaTeX group that starts just after its opening brace.
fn group(body: &str) -> Option<&str> {
    let mut depth = 1;
    let (end, _) = body.char_indices().find(|&(_, c)| {
        match c {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
        depth == 0
    })?;
    Some(&body[..end])
}

/// A LaTeX sectioning command, numbered as LaTeX numbers it: `\subsection{Gaussian
/// resampling}` after the seventh `\section` is "§7.2 Gaussian resampling", and sections
/// after `\appendix` are lettered. A starred command is unnumbered.
fn latex_heading(line: &str, numbering: &mut Numbering) -> Option<(String, usize)> {
    let rest = line.strip_prefix('\\')?;
    if rest.starts_with("appendix") && !rest["appendix".len()..].starts_with(char::is_alphabetic) {
        numbering.appendix = true;
        numbering.counters = [0; 3];
        return None;
    }
    let commands = ["section", "subsection", "subsubsection"];
    let level = commands.iter().position(|c| {
        rest.strip_prefix(c)
            .is_some_and(|r| r.starts_with(['{', '*', '[']))
    })?;
    let after = &rest[commands[level].len()..];
    let (starred, after) = match after.strip_prefix('*') {
        Some(after) => (true, after),
        None => (false, after),
    };
    let after = match after.strip_prefix('[') {
        Some(short) => short.split_once(']')?.1,
        None => after,
    };
    let title = group(after.strip_prefix('{')?)?.trim();
    if starred {
        return Some((title.to_string(), level + 1));
    }
    let counters = &mut numbering.counters;
    counters[level] += 1;
    counters[level + 1..].iter_mut().for_each(|n| *n = 0);
    let number: Vec<String> = counters[..=level]
        .iter()
        .enumerate()
        .map(|(i, &n)| match (i, numbering.appendix) {
            (0, true) => char::from_u32('A' as u32 + n - 1).map_or(n.to_string(), String::from),
            _ => n.to_string(),
        })
        .collect();
    Some((format!("§{} {title}", number.join(".")), level + 1))
}

/// The theorem-like environments a LaTeX source declares with `\newtheorem`, each with its
/// printed name: `\newtheorem{lemma}[theorem]{Lemma}` declares `lemma`, printed "Lemma".
fn declared_theorems(text: &str) -> BTreeMap<String, String> {
    let mut declared = BTreeMap::new();
    for (at, _) in text.match_indices("\\newtheorem") {
        let rest = text[at + "\\newtheorem".len()..].trim_start_matches('*');
        let Some(name) = rest.strip_prefix('{').and_then(group) else {
            continue;
        };
        let mut rest = &rest[name.len() + 2..];
        // An optional counter it shares, before or after the printed name.
        if let Some(shared) = rest.strip_prefix('[') {
            rest = shared.split_once(']').map_or("", |(_, r)| r);
        }
        if let Some(printed) = rest.strip_prefix('{').and_then(group) {
            declared.insert(name.trim().to_string(), printed.trim().to_string());
        }
    }
    declared
}

/// A theorem-like statement starting on this line: `\begin{lemma}` for an environment the
/// source declares, with the `\label` on the same line if there is one.
fn statement(line: &str, declared: &BTreeMap<String, String>) -> Option<(String, Option<String>)> {
    let name = group(line.strip_prefix("\\begin{")?)?;
    let kind = declared.get(name.trim_end_matches('*'))?;
    let label = line
        .split_once("\\label{")
        .and_then(|(_, l)| group(l))
        .map(str::to_string);
    Some((kind.clone(), label))
}

/// The environment of a displayed equation that starts on this line.
fn display(line: &str) -> Option<String> {
    let name = line.strip_prefix("\\begin{").and_then(group)?;
    let bare = name.trim_end_matches('*');
    [
        "equation",
        "align",
        "gather",
        "multline",
        "eqnarray",
        "displaymath",
    ]
    .contains(&bare)
    .then(|| name.to_string())
}

impl Source {
    pub fn new(id: &str, title: Option<&str>, url: Option<&str>, path: &Path, text: &str) -> Self {
        // `normalise` collapses all whitespace, so the normalised text is the normalised
        // lines (pieces between newlines and form feeds) joined by single spaces.
        let mut pieces = Vec::new();
        let mut pages = vec![0];
        let mut sections = Vec::new();
        let mut statements = Vec::new();
        // For each statement, the index of its piece.
        let mut at_piece = Vec::new();
        let mut displays = Vec::new();
        // For each display, the index of its piece and its environment.
        let mut display_at = Vec::new();
        let mut lines = Vec::new();
        let mut numbering = Numbering::default();
        let mut marked_up = false;
        let declared = declared_theorems(text);
        let mut offset = 0;
        let mut line_count = 0;
        for (n, line) in text.lines().enumerate() {
            line_count = n + 1;
            for (p, part) in line.split('\u{c}').enumerate() {
                if p > 0 {
                    pages.push(offset);
                }
                let piece = normalise(part);
                if piece.is_empty() {
                    continue;
                }
                let latex = latex_heading(&piece, &mut numbering);
                let markdown = latex.is_none() && piece.starts_with('#');
                let found = latex.or_else(|| heading(&piece));
                marked_up |= found.is_some() && (markdown || piece.starts_with('\\'));
                if let Some((title, level)) = found {
                    sections.push(Heading {
                        offset,
                        line: n + 1,
                        level,
                        title,
                    });
                } else if let Some(env) = display(&piece) {
                    display_at.push((pieces.len(), env));
                    displays.push(Display {
                        offset,
                        line: n + 1,
                        label: None,
                    });
                } else if let Some((kind, label)) = statement(&piece, &declared) {
                    at_piece.push(pieces.len());
                    statements.push(Statement {
                        offset,
                        line: n + 1,
                        kind,
                        label,
                    });
                }
                lines.push((offset, n + 1));
                offset += piece.len() + 1;
                pieces.push(piece);
            }
        }
        if pages.len() == 1 {
            pages.clear();
        }
        // A `\label` on the line after `\begin{lemma}`.
        for (s, &i) in statements.iter_mut().zip(&at_piece) {
            if s.label.is_none() {
                s.label = pieces
                    .get(i + 1)
                    .and_then(|next| next.strip_prefix("\\label{"))
                    .and_then(group)
                    .map(str::to_string);
            }
        }
        // A display's `\label`, anywhere up to its `\end`.
        for (d, (i, env)) in displays.iter_mut().zip(&display_at) {
            let end = format!("\\end{{{env}");
            for piece in &pieces[*i..] {
                if let Some(label) = piece.split_once("\\label{").and_then(|(_, l)| group(l)) {
                    d.label = Some(label.to_string());
                    break;
                }
                if piece.contains(&end) {
                    break;
                }
            }
        }
        Self {
            id: id.to_string(),
            title: title.unwrap_or(id).to_string(),
            url: url.map(str::to_string),
            path: path.to_path_buf(),
            normalised: pieces.join(" "),
            pages,
            sections,
            marked_up,
            statements,
            displays,
            lines,
            line_count,
        }
    }

    /// Where `quote` is in the source: `None` if it is not there word for word, otherwise
    /// a locator such as "§3.2 Gaussian resampling, p. 5", if the source has sections or
    /// page breaks.
    pub fn locate(&self, quote: &str) -> Option<Option<String>> {
        let quote = normalise(quote);
        if quote.is_empty() {
            return None;
        }
        let at = self.normalised.find(&quote)?;
        let mut parts = Vec::new();
        if let Some(h) = self.sections.iter().rev().find(|h| h.offset <= at) {
            parts.push(h.title.clone());
        }
        if !self.pages.is_empty() {
            let page = self.pages.iter().filter(|&&o| o <= at).count();
            parts.push(format!("p. {page}"));
        }
        Some((!parts.is_empty()).then(|| parts.join(", ")))
    }

    /// The section headings, in order.
    pub fn sections(&self) -> &[Heading] {
        &self.sections
    }

    /// Whether the section headings are marked up in the source, rather than guessed from
    /// numbered lines.
    pub fn marked_up(&self) -> bool {
        self.marked_up
    }

    /// The theorem-like statements, in order.
    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }

    /// The displayed equations, in order.
    pub fn displays(&self) -> &[Display] {
        &self.displays
    }

    /// Lines in the source text, blank ones included.
    pub fn line_count(&self) -> usize {
        self.line_count
    }

    /// The line of the source, from 1, that an offset in the normalised text falls on.
    pub fn line_at(&self, offset: usize) -> usize {
        let i = self.lines.partition_point(|&(o, _)| o <= offset);
        i.checked_sub(1).map_or(1, |i| self.lines[i].1)
    }

    /// Where `quote` occurs in the normalised text, each match's start, in order.
    pub fn occurrences(&self, quote: &str) -> Vec<usize> {
        let quote = normalise(quote);
        if quote.is_empty() {
            return Vec::new();
        }
        self.normalised
            .match_indices(&quote)
            .map(|(at, _)| at)
            .collect()
    }

    /// Markdown citation: a link when the source has a URL.
    pub fn citation(&self) -> String {
        match &self.url {
            Some(url) => format!("[{}]({url})", self.title),
            None => self.title.clone(),
        }
    }
}

pub fn find_source<'a>(sources: &'a [Source], id: &str) -> Option<&'a Source> {
    sources.iter().find(|s| s.id == id)
}

#[derive(Debug, Clone)]
pub struct Project {
    pub root: PathBuf,
    pub config: Config,
    pub sources: Vec<Source>,
}

impl Project {
    pub fn load(root: &Path) -> Result<Self> {
        // Absolute, so task files work from any working directory.
        let root = &std::fs::canonicalize(root).map_err(|source| crate::error::Error::Io {
            path: root.to_path_buf(),
            source,
        })?;
        let config = Config::load(&root.join(config::FILE_NAME))?;
        let sources = config
            .sources
            .iter()
            .map(|spec| {
                let text = read(&root.join(&spec.path))?;
                Ok(Source::new(
                    &spec.id,
                    spec.title.as_deref(),
                    spec.url.as_deref(),
                    &spec.path,
                    &text,
                ))
            })
            .collect::<Result<_>>()?;
        Ok(Self {
            root: root.to_path_buf(),
            config,
            sources,
        })
    }

    pub fn state_dir(&self) -> PathBuf {
        self.root.join("exegenius")
    }

    pub fn graph_path(&self) -> PathBuf {
        self.state_dir().join("graph.json")
    }

    pub fn state_path(&self) -> PathBuf {
        self.state_dir().join("state.json")
    }

    pub fn log_path(&self) -> PathBuf {
        self.state_dir().join("log.jsonl")
    }

    pub fn lock_path(&self) -> PathBuf {
        self.state_dir().join("lock")
    }

    pub fn audit_path(&self) -> PathBuf {
        self.state_dir().join("audit.json")
    }

    pub fn tasks_dir(&self) -> PathBuf {
        self.state_dir().join("tasks")
    }

    pub fn outline_path(&self) -> PathBuf {
        self.state_dir().join("outline.json")
    }

    pub fn argument_path(&self) -> PathBuf {
        self.state_dir().join("argument.json")
    }

    pub fn support_path(&self) -> PathBuf {
        self.state_dir().join("support.json")
    }

    pub fn overloads_path(&self) -> PathBuf {
        self.state_dir().join("overloads.json")
    }

    pub fn simulation_path(&self) -> PathBuf {
        self.state_dir().join("simulation.json")
    }

    /// Steps of a pending reader task.
    pub fn trail_path(&self, task: &str) -> PathBuf {
        self.state_dir().join("trails").join(format!("{task}.json"))
    }

    pub fn report_path(&self) -> PathBuf {
        self.state_dir().join("report.md")
    }

    pub fn manifest_path(&self) -> PathBuf {
        self.state_dir().join("built.json")
    }

    pub fn docs_dir(&self) -> PathBuf {
        self.root.join("docs")
    }

    pub fn zensical_path(&self) -> PathBuf {
        self.root.join("zensical.toml")
    }

    fn page_path(&self, id: &str) -> PathBuf {
        self.state_dir().join("pages").join(format!("{id}.md"))
    }

    /// The concept graph, if extracted. A pilot recorded in it is dropped when the
    /// configuration no longer asks for one, so the run goes on to the full target.
    pub fn load_graph(&self) -> Result<Option<Graph>> {
        let path = self.graph_path();
        if !path.exists() {
            return Ok(None);
        }
        let mut graph: Graph = read_json(&path)?;
        if self.config.pilot.is_none() {
            graph.pilot = None;
        }
        Ok(Some(graph))
    }

    pub fn save_graph(&self, graph: &Graph) -> Result<()> {
        write_json(&self.graph_path(), graph)
    }

    pub fn page_body(&self, id: &str) -> Result<Option<String>> {
        let path = self.page_path(id);
        if path.exists() {
            Ok(Some(read(&path)?))
        } else {
            Ok(None)
        }
    }

    pub fn save_page_body(&self, id: &str, body: &str) -> Result<()> {
        write(&self.page_path(id), body)
    }

    /// The sources as a markdown list of files to read.
    pub fn sources_listing(&self) -> String {
        self.sources
            .iter()
            .map(|s| {
                format!(
                    "- `{}`: source id `{}`, \"{}\"",
                    s.path.display(),
                    s.id,
                    s.title
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latex_sections_locate_quotes() {
        let text = "\\section{Introduction}\nWe multiply.\n\\section{Transforms}\n\
                    \\subsection*{Notation}\nLet $F$ be a transform.\n\
                    \\subsection[Resampling]{Gaussian resampling of $\\mathbb{C}^s$}\n\
                    We resample the grid.\n";
        let source = Source::new("paper", None, None, Path::new("p.tex"), text);
        let at = |quote: &str| source.locate(quote).flatten();
        assert_eq!(at("We multiply."), Some("§1 Introduction".into()));
        assert_eq!(at("Let $F$ be"), Some("Notation".into()));
        assert_eq!(
            at("We resample the grid."),
            Some("§2.1 Gaussian resampling of $\\mathbb{C}^s$".into())
        );
    }

    #[test]
    fn headings_have_levels_and_appendix_sections_are_lettered() {
        let titles = |text: &str| {
            let s = Source::new("s", None, None, Path::new("s"), text);
            let t: Vec<(String, usize)> = s
                .sections()
                .iter()
                .map(|h| (h.title.clone(), h.level))
                .collect();
            (t, s.marked_up())
        };
        let latex =
            "\\section{Intro}\nA.\n  \\appendix\n\\section{Packed additions}\n\\subsection{Rows}\n";
        assert_eq!(
            titles(latex),
            (
                vec![
                    ("§1 Intro".into(), 1),
                    ("§A Packed additions".into(), 1),
                    ("§A.1 Rows".into(), 2)
                ],
                true
            )
        );
        let markdown = "# Paper\n## Transforms\nText.\n";
        assert_eq!(
            titles(markdown),
            (vec![("Paper".into(), 1), ("Transforms".into(), 2)], true)
        );
        let numbered = "3 Transforms\n3.1 Gaussian resampling\n";
        assert_eq!(
            titles(numbered),
            (
                vec![
                    ("§3 Transforms".into(), 1),
                    ("§3.1 Gaussian resampling".into(), 2)
                ],
                false
            )
        );
    }

    #[test]
    fn lines_are_counted_across_page_breaks() {
        let text = "first page\n\u{c}second page\n\nthird\u{c}fourth\n";
        let s = Source::new("s", None, None, Path::new("s.txt"), text);
        assert_eq!(s.locate("second page"), Some(Some("p. 2".into())));
        assert_eq!(s.locate("fourth"), Some(Some("p. 3".into())));
        let line = |q: &str| s.line_at(s.normalised.find(q).unwrap());
        assert_eq!((line("first"), line("second"), line("fourth")), (1, 2, 4));
        assert_eq!(s.line_count(), 4);
    }
}

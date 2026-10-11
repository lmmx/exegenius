//! A project directory: configuration, sources, and the state exegenius keeps.
//!
//! ```text
//! exegenius.toml         configuration
//! sources/               source documents (paths given in exegenius.toml)
//! exegenius/graph.json   concept graph, symbols each page binds, candidate concepts,
//!                        queued rewrites
//! exegenius/state.json   task state: issued tasks, audit round, audited pages
//! exegenius/tasks/       task files and their answer files
//! exegenius/pages/       page bodies
//! exegenius/audit.json   audit results, one record per round
//! exegenius/argument.json the plan of the sources' argument
//! exegenius/outline.json parts, glossary and front page
//! exegenius/overloads.json verdicts on letters bound to two meanings
//! exegenius/simulation.json simulated readers: goals, runs, grades
//! exegenius/trails/      pages each simulated reader opened
//! exegenius/report.md    how the simulated readers did
//! docs/, zensical.toml   build output
//! ```

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
    /// Offsets in `normalised` of section headings, with the heading.
    sections: Vec<(usize, String)>,
}

/// A section heading line: a markdown heading, or a numbered heading such as
/// "3.2 Gaussian resampling".
fn heading(line: &str) -> Option<String> {
    if let Some(rest) = line.strip_prefix('#') {
        let title = rest.trim_start_matches('#').trim();
        return (!title.is_empty()).then(|| title.to_string());
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
    .then(|| format!("§{number} {title}"))
}

impl Source {
    pub fn new(id: &str, title: Option<&str>, url: Option<&str>, path: &Path, text: &str) -> Self {
        // `normalise` collapses all whitespace, so the normalised text is the normalised
        // lines (pieces between newlines and form feeds) joined by single spaces.
        let mut pieces = Vec::new();
        let mut pages = vec![0];
        let mut sections = Vec::new();
        let mut offset = 0;
        for (p, page) in text.split('\u{c}').enumerate() {
            if p > 0 {
                pages.push(offset);
            }
            for line in page.lines() {
                let piece = normalise(line);
                if piece.is_empty() {
                    continue;
                }
                if let Some(h) = heading(&piece) {
                    sections.push((offset, h));
                }
                offset += piece.len() + 1;
                pieces.push(piece);
            }
        }
        if pages.len() == 1 {
            pages.clear();
        }
        Self {
            id: id.to_string(),
            title: title.unwrap_or(id).to_string(),
            url: url.map(str::to_string),
            path: path.to_path_buf(),
            normalised: pieces.join(" "),
            pages,
            sections,
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
        if let Some((_, h)) = self.sections.iter().rev().find(|(o, _)| *o <= at) {
            parts.push(h.clone());
        }
        if !self.pages.is_empty() {
            let page = self.pages.iter().filter(|&&o| o <= at).count();
            parts.push(format!("p. {page}"));
        }
        Some((!parts.is_empty()).then(|| parts.join(", ")))
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

    pub fn load_graph(&self) -> Result<Option<Graph>> {
        let path = self.graph_path();
        if path.exists() {
            Ok(Some(read_json(&path)?))
        } else {
            Ok(None)
        }
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

//! A project directory: configuration, sources, and the state exegenius keeps.
//!
//! ```text
//! exegenius.toml         configuration
//! sources/               source documents (paths given in exegenius.toml)
//! exegenius/graph.json   concept graph, candidate concepts, queued rewrites
//! exegenius/state.json   task state: issued tasks, audit round, audited pages
//! exegenius/tasks/       task files and their answer files
//! exegenius/pages/       page bodies
//! exegenius/audit.json   audit results, one record per round
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
}

impl Source {
    pub fn new(id: &str, title: Option<&str>, url: Option<&str>, path: &Path, text: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.unwrap_or(id).to_string(),
            url: url.map(str::to_string),
            path: path.to_path_buf(),
            normalised: normalise(text),
        }
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

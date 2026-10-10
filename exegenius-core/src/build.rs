//! Step 5: write the docs tree and `zensical.toml`.
//!
//! Pages are plain markdown with no frontmatter. The title, prerequisites, backlinks and
//! sources sections come from the graph; only the body comes from the model. Files written
//! by a previous build that are no longer part of the plan are removed (tracked in
//! `exegenius/built.json`); other files in `docs/` are left alone.

use std::collections::{BTreeSet, HashSet};
use std::path::PathBuf;

use toml_edit::{Array, DocumentMut, InlineTable, Item, Table, Value};

use crate::error::{Error, Result, read, read_json, write, write_json};
use crate::graph::{Concept, Graph};
use crate::plan::Plan;
use crate::project::{Project, find_source};
use crate::write::{fix_links, link};

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

#[derive(Debug, Default)]
pub struct BuildReport {
    pub pages: Vec<String>,
    /// Planned pages with no body yet (run `write`).
    pub missing: Vec<String>,
    /// Files from a previous build that were removed.
    pub removed: Vec<PathBuf>,
    pub warnings: Vec<String>,
}

pub fn title(project: &Project, graph: &Graph) -> String {
    project
        .config
        .title
        .clone()
        .or_else(|| graph.get(&graph.target).map(|c| c.name.clone()))
        .unwrap_or_else(|| "exegenius".to_string())
}

pub fn build(project: &Project, graph: &Graph, plan: &Plan) -> Result<BuildReport> {
    let docs = project.docs_dir();
    let mut report = BuildReport::default();
    let mut written: BTreeSet<String> = BTreeSet::new();
    let pages: HashSet<String> = plan.order.iter().cloned().collect();

    for id in &plan.order {
        let Some(body) = project.page_body(id)? else {
            report.missing.push(id.clone());
            continue;
        };
        // Bodies were written against an earlier plan; drop links to pages no longer in it.
        let (body, removed, _) = fix_links(&body, &pages);
        for dest in removed {
            report.warnings.push(format!(
                "`{id}`: link to {dest:?} rendered as text (not a page)"
            ));
        }
        let c = graph.get(id).expect("planned pages are concepts");
        let file = format!("{id}.md");
        write(
            &docs.join(&file),
            &render_page(project, graph, plan, c, &body),
        )?;
        written.insert(file);
        report.pages.push(id.clone());
    }
    write(
        &docs.join("index.md"),
        &render_index(project, graph, plan, &report.pages),
    )?;
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

    let nav: Vec<(String, String)> = report
        .pages
        .iter()
        .filter_map(|id| graph.get(id))
        .map(|c| (c.name.clone(), format!("{}.md", c.id)))
        .collect();
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

fn render_page(project: &Project, graph: &Graph, plan: &Plan, c: &Concept, body: &str) -> String {
    let mut out = format!("# {}\n\n{}\n", c.name, body.trim());
    let prerequisites: Vec<String> = c
        .requires
        .iter()
        .filter_map(|r| graph.get(r))
        .map(|p| {
            if plan.contains(&p.id) {
                format!("- {}", link(p))
            } else {
                format!("- {} (assumed known)", p.name)
            }
        })
        .collect();
    if !prerequisites.is_empty() {
        out.push_str(&format!(
            "\n## Prerequisites\n\n{}\n",
            prerequisites.join("\n")
        ));
    }
    let used_by: Vec<String> = plan
        .order
        .iter()
        .filter_map(|id| graph.get(id))
        .filter(|d| d.requires.contains(&c.id))
        .map(|d| format!("- {}", link(d)))
        .collect();
    if !used_by.is_empty() {
        out.push_str(&format!("\n## Used by\n\n{}\n", used_by.join("\n")));
    }
    if !c.evidence.is_empty() {
        out.push_str("\n## Sources\n");
        for e in &c.evidence {
            let cite = find_source(&project.sources, &e.source)
                .map(|s| s.citation())
                .unwrap_or_else(|| e.source.clone());
            out.push_str(&format!("\n> {}\n>\n> — {cite}\n", e.quote.trim()));
        }
    }
    out
}

fn render_index(project: &Project, graph: &Graph, plan: &Plan, pages: &[String]) -> String {
    let mut out = format!("# {}\n\n", title(project, graph));
    if let Some(target) = graph.get(&graph.target) {
        out.push_str(&format!(
            "This wiki explains {}: {}\n\n",
            link(target),
            target.gloss
        ));
    }
    out.push_str(&format!(
        "It is written for a reader who knows: {}\n\n## Reading order\n\n",
        project.config.reader.knows.trim()
    ));
    for (i, c) in pages.iter().filter_map(|id| graph.get(id)).enumerate() {
        out.push_str(&format!("{}. {} — {}\n", i + 1, link(c), c.gloss));
    }
    let assumed: Vec<&Concept> = graph
        .concepts
        .iter()
        .filter(|k| {
            k.known
                && plan
                    .order
                    .iter()
                    .filter_map(|id| graph.get(id))
                    .any(|p| p.requires.contains(&k.id))
        })
        .collect();
    if !assumed.is_empty() {
        out.push_str("\n## Assumed known\n\n");
        for k in assumed {
            out.push_str(&format!("- {} — {}\n", k.name, k.gloss));
        }
    }
    out.push_str("\n## Sources\n\n");
    for s in &project.sources {
        out.push_str(&format!("- {}\n", s.citation()));
    }
    out
}

/// Create or update `zensical.toml`: set `project.nav` to the reading order, add the
/// MathJax scripts to `project.extra_javascript`, and fill in `site_name` and `docs_dir`
/// only if they are missing. Everything else in an existing file is kept as written.
pub fn zensical_toml(
    existing: Option<&str>,
    site_name: &str,
    nav: &[(String, String)],
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
            "zensical.toml sets docs_dir = {other:?}, but exegenius writes to \"docs\""
        )),
    }

    let mut entries = Array::new();
    entries.push("index.md");
    for (name, file) in nav {
        let mut entry = InlineTable::new();
        entry.insert(name, Value::from(file.as_str()));
        entries.push(entry);
    }
    for entry in entries.iter_mut() {
        entry.decor_mut().set_prefix("\n  ");
    }
    entries.set_trailing_comma(true);
    entries.set_trailing("\n");
    project["nav"] = toml_edit::value(entries);

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
    Ok((doc.to_string(), warnings))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nav() -> Vec<(String, String)> {
        vec![
            ("Chirp identity".into(), "chirp-identity.md".into()),
            ("DFT".into(), "dft.md".into()),
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
        assert_eq!(project["extra_javascript"].as_array().unwrap().len(), 2);
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
        assert!(project["theme"]["features"].is_array());
    }
}

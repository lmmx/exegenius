//! End-to-end run of the pipeline against a scripted model: extract, write, audit (which
//! finds an unknown term), resolve the candidate, rewrite, audit clean, build.

use std::cell::RefCell;
use std::path::Path;

use exegenius_core::audit::audit_pages;
use exegenius_core::build::build;
use exegenius_core::extract::{extend, extract};
use exegenius_core::model::{Model, Prompt};
use exegenius_core::plan::plan;
use exegenius_core::project::Project;
use exegenius_core::write::write_pages;
use exegenius_core::{Error, Result};
use serde_json::{Value, json};

const SOURCE: &str = "The discrete Fourier transform maps a sequence to its spectrum.\n\
Convolution of two sequences multiplies their transforms.\n\
The algorithm converts these through a chirp identity to convolution.\n\
Intermediate arithmetic is exact on dyadic real and imaginary parts.\n";

const CONFIG: &str = r#"
title = "Chirp test"
target = "The chirp identity"
[reader]
knows = "Complex numbers"
[[sources]]
id = "notes"
path = "sources/notes.txt"
title = "Notes"
url = "https://example.org/notes"
"#;

#[derive(Default)]
struct Scripted {
    audits: RefCell<usize>,
    calls: RefCell<Vec<String>>,
}

fn concept(id: &str, name: &str, requires: &[&str], known: bool, quote: Option<&str>) -> Value {
    json!({
        "id": id,
        "name": name,
        "aliases": [],
        "gloss": format!("What {name} is."),
        "requires": requires,
        "known": known,
        "evidence": quote.map(|q| vec![json!({"source": "notes", "quote": q})]).unwrap_or_default(),
    })
}

fn page_name(task: &str, marker: &str) -> String {
    task.split(marker)
        .nth(1)
        .and_then(|rest| rest.lines().next())
        .unwrap_or_default()
        .trim()
        .to_string()
}

impl Model for Scripted {
    fn complete(&self, prompt: &Prompt) -> Result<Value> {
        let system = &prompt.system;
        let task = &prompt.task;
        let step = if system.starts_with("You decompose") {
            "extract"
        } else if system.starts_with("You extend") {
            "extend"
        } else if system.starts_with("You write") {
            "write"
        } else if system.starts_with("You audit") {
            "audit"
        } else {
            return Err(Error::Model("unexpected prompt".into()));
        };
        assert!(prompt.schema.is_object());
        self.calls.borrow_mut().push(step.to_string());
        Ok(match step {
            "extract" => json!({
                "target": "Chirp Identity",
                "concepts": [
                    concept("complex", "Complex numbers", &[], true, None),
                    concept("dft", "Discrete Fourier transform", &["complex"], false,
                        Some("The discrete Fourier transform maps a sequence")),
                    concept("convolution", "Convolution", &[], false,
                        Some("Convolution of two sequences")),
                    concept("chirp-identity", "Chirp identity", &["dft", "convolution"], false,
                        Some("converts these through a chirp identity to convolution.")),
                ],
            }),
            "extend" => {
                assert!(task.contains("\"dyadic\""));
                json!({
                    "concepts": [concept("dyadic-rational", "Dyadic rational", &[], false,
                        Some("exact on dyadic real and imaginary parts"))],
                    "resolutions": [{"term": "dyadic", "concept": "dyadic-rational"}],
                })
            }
            "write" => {
                let name = page_name(task, "Concept: ");
                let body = match name.as_str() {
                    "Chirp identity" if task.contains("Previous version") => {
                        assert!(task.contains("[Dyadic rational](dyadic-rational.md)"));
                        "Combine the [DFT](dft.md) and [convolution](convolution.md); values \
                         are [dyadic](dyadic-rational.md), so $nk$ is exact."
                    }
                    "Chirp identity" => {
                        "Combine the [DFT](dft.md) and [convolution](convolution.md) with \
                         dyadic scaling; see [elsewhere](bogus.md)."
                    }
                    _ => "A short explanation with $x^2$.",
                };
                json!({"body": body})
            }
            "audit" => {
                *self.audits.borrow_mut() += 1;
                let name = page_name(task, "# Page to audit\n\n# ");
                let first_round = !task.contains("dyadic-rational.md");
                if name == "Chirp identity" && first_round {
                    json!({"gaps": [
                        {"term": "dyadic", "quote": "dyadic scaling", "kind": "term",
                         "note": "not defined"},
                        {"term": "DFT", "quote": "not on the page", "kind": "term",
                         "note": "invented quote"},
                    ]})
                } else {
                    json!({"gaps": []})
                }
            }
            _ => unreachable!(),
        })
    }
}

fn setup(dir: &Path) {
    std::fs::create_dir_all(dir.join("sources")).unwrap();
    std::fs::write(dir.join("exegenius.toml"), CONFIG).unwrap();
    std::fs::write(dir.join("sources/notes.txt"), SOURCE).unwrap();
}

#[test]
fn pipeline_end_to_end() {
    let temp = tempfile::tempdir().unwrap();
    let dir = std::env::var_os("EXEGENIUS_TEST_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| temp.path().to_path_buf());
    let _ = std::fs::remove_dir_all(&dir);
    setup(&dir);
    let project = Project::load(&dir).unwrap();
    let model = Scripted::default();
    let mut log = |_: &str| {};

    let (mut graph, warnings) = extract(&project, &model, &mut log).unwrap();
    assert!(warnings.is_empty(), "{warnings:?}");
    assert_eq!(graph.target, "chirp-identity");
    project.save_graph(&graph).unwrap();

    let p = plan(&graph, &project.sources).unwrap();
    assert_eq!(p.order, vec!["dft", "convolution", "chirp-identity"]);
    let report = write_pages(&project, &mut graph, &p, &model, &mut log).unwrap();
    assert_eq!(report.written.len(), 3);
    assert!(report.warnings.iter().any(|w| w.contains("bogus.md")));
    assert!(
        !project
            .page_body("chirp-identity")
            .unwrap()
            .unwrap()
            .contains("bogus")
    );

    let round = audit_pages(&project, &mut graph, &p, &model, &mut log).unwrap();
    assert_eq!(round.gap_count(), 1);
    assert_eq!(round.pages[2].dropped.len(), 1);
    assert_eq!(graph.candidates.len(), 1);

    extend(&project, &mut graph, &model, &mut log).unwrap();
    assert!(graph.candidates.is_empty());
    assert!(
        graph
            .get("chirp-identity")
            .unwrap()
            .requires
            .contains(&"dyadic-rational".into())
    );
    assert!(graph.rewrite.contains_key("chirp-identity"));

    let p = plan(&graph, &project.sources).unwrap();
    assert_eq!(
        p.order,
        vec!["dft", "convolution", "dyadic-rational", "chirp-identity"]
    );
    let report = write_pages(&project, &mut graph, &p, &model, &mut log).unwrap();
    let mut written = report.written.clone();
    written.sort();
    assert_eq!(written, vec!["chirp-identity", "dyadic-rational"]);
    assert!(graph.rewrite.is_empty());

    let round = audit_pages(&project, &mut graph, &p, &model, &mut log).unwrap();
    assert_eq!(round.round, 2);
    assert_eq!(round.gap_count(), 0);

    let report = build(&project, &graph, &p).unwrap();
    assert_eq!(report.pages.len(), 4);
    let page = std::fs::read_to_string(dir.join("docs/chirp-identity.md")).unwrap();
    assert!(page.starts_with("# Chirp identity\n\n"));
    assert!(page.contains("## Prerequisites\n\n- [Discrete Fourier transform](dft.md)"));
    assert!(page.contains("- [Dyadic rational](dyadic-rational.md)"));
    assert!(page.contains("> — [Notes](https://example.org/notes)"));
    let dft = std::fs::read_to_string(dir.join("docs/dft.md")).unwrap();
    assert!(dft.contains("- Complex numbers (assumed known)"));
    assert!(dft.contains("## Used by\n\n- [Chirp identity](chirp-identity.md)"));
    let index = std::fs::read_to_string(dir.join("docs/index.md")).unwrap();
    assert!(index.contains("## Assumed known\n\n- Complex numbers"));
    let zensical = std::fs::read_to_string(dir.join("zensical.toml")).unwrap();
    assert!(zensical.contains("{ \"Chirp identity\" = \"chirp-identity.md\" }"));
    assert!(dir.join("docs/javascripts/mathjax.js").exists());

    // Re-running every step with an unchanged graph makes no further write calls.
    let calls = model.calls.borrow().len();
    write_pages(&project, &mut graph, &p, &model, &mut log).unwrap();
    assert_eq!(model.calls.borrow().len(), calls);

    // A page that leaves the plan is removed from docs/ on the next build.
    graph
        .get_mut("chirp-identity")
        .unwrap()
        .requires
        .retain(|r| r != "dyadic-rational");
    let p = plan(&graph, &project.sources).unwrap();
    let report = build(&project, &graph, &p).unwrap();
    assert_eq!(report.removed.len(), 1);
    assert!(!dir.join("docs/dyadic-rational.md").exists());
    assert!(
        report
            .warnings
            .iter()
            .any(|w| w.contains("dyadic-rational.md"))
    );
    let page = std::fs::read_to_string(dir.join("docs/chirp-identity.md")).unwrap();
    assert!(page.contains("values are dyadic, so"));
    assert_eq!(*model.audits.borrow(), 7);
}

//! The task loop end to end, with scripted answers standing in for the agent: extract,
//! write, audit (finding an unknown term), extend, rewrite, a clean audit round, build.

use std::path::Path;

use exegenius_core::project::Project;
use exegenius_core::task::{Kind, Next, Task, next, submit};
use serde_json::json;

const SOURCE: &str = "The discrete Fourier transform maps a sequence to its spectrum.\n\
Convolution of two sequences multiplies their transforms.\n\
The algorithm converts these through a chirp identity to convolution.\n\
Intermediate arithmetic is exact on dyadic real and imaginary parts.\n";

fn setup(dir: &Path, rounds: u32) -> Project {
    std::fs::create_dir_all(dir.join("sources")).unwrap();
    std::fs::write(
        dir.join("exegenius.toml"),
        format!(
            "title = \"Chirp test\"\ntarget = \"The chirp identity\"\nrounds = {rounds}\n\
             [reader]\nknows = \"Complex numbers\"\n[[sources]]\nid = \"notes\"\n\
             path = \"sources/notes.txt\"\ntitle = \"Notes\"\nurl = \"https://example.org/notes\"\n"
        ),
    )
    .unwrap();
    std::fs::write(dir.join("sources/notes.txt"), SOURCE).unwrap();
    Project::load(dir).unwrap()
}

fn concept(
    id: &str,
    name: &str,
    requires: &[&str],
    known: bool,
    quote: Option<&str>,
) -> serde_json::Value {
    json!({
        "id": id, "name": name, "aliases": [], "gloss": format!("What {name} is."),
        "requires": requires, "known": known,
        "evidence": quote.map(|q| vec![json!({"source": "notes", "quote": q})]).unwrap_or_default(),
    })
}

fn extract_answer(chirp_quote: &str) -> String {
    json!({
        "target": "Chirp Identity",
        "concepts": [
            concept("complex", "Complex numbers", &[], true, None),
            concept("dft", "Discrete Fourier transform", &["complex"], false,
                Some("The discrete Fourier transform maps a sequence")),
            concept("convolution", "Convolution", &[], false, Some("Convolution of two sequences")),
            concept("chirp-identity", "Chirp identity", &["dft", "convolution"], false,
                Some(chirp_quote)),
        ],
    })
    .to_string()
}

fn tasks(next: Next) -> Vec<Task> {
    match next {
        Next::Tasks { tasks, .. } => tasks,
        Next::Done(_) => panic!("expected tasks"),
    }
}

fn answer(project: &Project, task: &Task, text: &str) {
    std::fs::write(project.root.join(task.answer()), text).unwrap();
}

fn page(task: &Task) -> &str {
    match &task.kind {
        Kind::Write { page } | Kind::Audit { page, .. } => page,
        _ => "",
    }
}

fn read(project: &Project, path: impl AsRef<Path>) -> String {
    std::fs::read_to_string(project.root.join(path)).unwrap()
}

#[test]
fn task_loop_end_to_end() {
    let temp = tempfile::tempdir().unwrap();
    let project = setup(temp.path(), 3);

    // Extract: an invented quote is rejected and the task stays pending.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch.len(), 1);
    assert_eq!(batch[0].id, "0001-extract");
    let file = read(&project, batch[0].file());
    assert!(file.contains("sources/notes.txt") && file.contains("exegenius submit 0001-extract"));
    answer(&project, &batch[0], &extract_answer("an invented sentence"));
    let err = submit(&project, "0001-extract").unwrap_err().to_string();
    assert!(err.contains("quote not found verbatim"), "{err}");
    assert_eq!(tasks(next(&project).unwrap()), batch);
    answer(
        &project,
        &batch[0],
        &extract_answer("through a chirp identity to convolution"),
    );
    submit(&project, "0001-extract").unwrap();

    // Write: three independent tasks; a link to a missing page is rejected.
    let batch = tasks(next(&project).unwrap());
    let pages: Vec<&str> = batch.iter().map(page).collect();
    assert_eq!(pages, vec!["dft", "convolution", "chirp-identity"]);
    for t in &batch {
        let body = if page(t) == "chirp-identity" {
            "Combine the [DFT](dft.md) and [convolution](convolution.md) with dyadic scaling."
        } else {
            "# Title to strip\n\nA short explanation with $x^2$."
        };
        if page(t) == "chirp-identity" {
            answer(&project, t, "See [elsewhere](bogus.md).");
            let err = submit(&project, &t.id).unwrap_err().to_string();
            assert!(err.contains("bogus.md"), "{err}");
        }
        answer(&project, t, body);
        submit(&project, &t.id).unwrap();
    }
    assert!(
        !project
            .page_body("dft")
            .unwrap()
            .unwrap()
            .contains("Title to strip")
    );

    // Audit round 1, submitted concurrently: one unknown term on the chirp page.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch.len(), 3);
    assert!(batch.iter().all(|t| matches!(t.kind, Kind::Audit { .. })));
    let audit_file = read(&project, batch[2].file());
    assert!(audit_file.contains("with dyadic scaling") && audit_file.contains("- Complex numbers"));
    std::thread::scope(|s| {
        for t in &batch {
            let project = &project;
            s.spawn(move || {
                let gaps = if page(t) == "chirp-identity" {
                    json!({"gaps": [{"term": "dyadic", "quote": "dyadic scaling",
                                     "kind": "term", "note": "not defined"}]})
                } else {
                    json!({"gaps": []})
                };
                answer(project, t, &gaps.to_string());
                submit(project, &t.id).unwrap();
            });
        }
    });

    // Extend: the candidate becomes a concept and a prerequisite of the chirp page.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch[0].kind, Kind::Extend);
    assert!(read(&project, batch[0].file()).contains("\"dyadic\""));
    answer(
        &project,
        &batch[0],
        &json!({
            "concepts": [concept("dyadic-rational", "Dyadic rational", &[], false,
                Some("exact on dyadic real and imaginary parts"))],
            "resolutions": [{"term": "dyadic", "concept": "dyadic-rational"}],
        })
        .to_string(),
    );
    submit(&project, &batch[0].id).unwrap();

    // Write: the new page and the rewrite of the chirp page, which sees the audit notes.
    let batch = tasks(next(&project).unwrap());
    let mut pages: Vec<&str> = batch.iter().map(page).collect();
    pages.sort();
    assert_eq!(pages, vec!["chirp-identity", "dyadic-rational"]);
    for t in &batch {
        let body = if page(t) == "chirp-identity" {
            let file = read(&project, t.file());
            assert!(file.contains("Previous version") && file.contains("now a prerequisite"));
            "Combine the [DFT](dft.md) and [convolution](convolution.md); values are \
             [dyadic](dyadic-rational.md), so $nk$ is exact."
        } else {
            "A number of the form $m/2^k$."
        };
        answer(&project, t, body);
        submit(&project, &t.id).unwrap();
    }

    // Audit round 2: every page's input changed (a page was added); no gaps.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch.len(), 4);
    for t in &batch {
        answer(&project, t, "{\"gaps\": []}");
        submit(&project, &t.id).unwrap();
    }

    // Done: the docs are built.
    let Next::Done(report) = next(&project).unwrap() else {
        panic!("expected done");
    };
    assert_eq!(report.pages.len(), 4);
    let chirp = read(&project, "docs/chirp-identity.md");
    assert!(chirp.starts_with("# Chirp identity\n\n"));
    assert!(chirp.contains("- [Dyadic rational](dyadic-rational.md)"));
    assert!(chirp.contains("> — [Notes](https://example.org/notes)"));
    let dft = read(&project, "docs/dft.md");
    assert!(dft.contains("- Complex numbers (assumed known)"));
    assert!(dft.contains("## Used by\n\n- [Chirp identity](chirp-identity.md)"));
    assert!(
        read(&project, "zensical.toml").contains("{ \"Chirp identity\" = \"chirp-identity.md\" }")
    );
    let audit = read(&project, "exegenius/audit.json");
    assert!(audit.contains("\"round\": 2") && audit.contains("candidate concept"));
    assert!(matches!(next(&project).unwrap(), Next::Done(_)));
    assert!(submit(&project, "0001-extract").is_err());
}

#[test]
fn last_round_fixes_are_written_then_built() {
    let temp = tempfile::tempdir().unwrap();
    let project = setup(temp.path(), 1);
    let batch = tasks(next(&project).unwrap());
    answer(&project, &batch[0], &extract_answer("a chirp identity"));
    submit(&project, &batch[0].id).unwrap();
    for t in tasks(next(&project).unwrap()) {
        answer(&project, &t, "Some explanation.");
        submit(&project, &t.id).unwrap();
    }
    for t in tasks(next(&project).unwrap()) {
        let gaps = json!({"gaps": [{"term": "step", "quote": "Some explanation",
                                    "kind": "step", "note": "too vague"}]});
        answer(&project, &t, &gaps.to_string());
        submit(&project, &t.id).unwrap();
    }
    // The single round found gaps: the rewrites are handed out, then the docs are built
    // without a second audit round.
    let rewrites = tasks(next(&project).unwrap());
    assert_eq!(rewrites.len(), 3);
    assert!(
        rewrites
            .iter()
            .all(|t| matches!(t.kind, Kind::Write { .. }))
    );
    for t in &rewrites {
        answer(&project, t, "A clearer explanation.");
        submit(&project, &t.id).unwrap();
    }
    assert!(matches!(next(&project).unwrap(), Next::Done(_)));
}

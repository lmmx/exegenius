//! The task loop end to end, with scripted answers standing in for the agent: extract,
//! write, audit (finding an unknown term), extend, rewrite, a clean audit round, outline,
//! build; and the simulated readers: goals, reading through the CLI, a blind grade, repairs
//! and a second round.

use std::path::Path;

use exegenius_core::project::Project;
use exegenius_core::task::{Kind, Next, Task, next, read_page, search_pages, submit};
use serde_json::json;

const SOURCE: &str = "The discrete Fourier transform maps a sequence to its spectrum.\n\
Convolution of two sequences multiplies their transforms.\n\
The algorithm converts these through a chirp identity to convolution.\n\
Intermediate arithmetic is exact on dyadic real and imaginary parts.\n";

fn setup(dir: &Path, rounds: u32) -> Project {
    setup_with(dir, rounds, "[simulation]\nrounds = 0\n")
}

fn setup_with(dir: &Path, rounds: u32, extra: &str) -> Project {
    std::fs::create_dir_all(dir.join("sources")).unwrap();
    std::fs::write(
        dir.join("exegenius.toml"),
        format!(
            "title = \"Chirp test\"\ntarget = \"The chirp identity\"\nrounds = {rounds}\n\
             [reader]\nknows = \"Complex numbers\"\n[[sources]]\nid = \"notes\"\n\
             path = \"sources/notes.txt\"\ntitle = \"Notes\"\nurl = \"https://example.org/notes\"\n\
             {extra}"
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
        } else if page(t) == "convolution" {
            "Convolution is what the discrete Fourier transform makes pointwise, as in $x^2$."
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

    // Outline and notation, handed out together.
    let batch = tasks(next(&project).unwrap());
    let kinds: Vec<&Kind> = batch.iter().map(|t| &t.kind).collect();
    assert_eq!(kinds, vec![&Kind::Outline, &Kind::Notation]);
    let (outline_task, notation_task) = (&batch[0], &batch[1]);

    // Outline: a page before its prerequisite's part, the target in the glossary and a page
    // left out are rejected; then two parts, with the dyadic page folded into the glossary.
    let file = read(&project, outline_task.file());
    assert!(file.contains("`chirp-identity`") && file.contains("Needs: dft, convolution"));
    assert!(file.contains("words, used by 1)"));
    let outline = |first: &[&str], second: &[&str], glossary: &[&str]| {
        json!({
            "overview": "The [chirp identity](chirp-identity.md) turns a DFT into a convolution.",
            "parts": [
                {"title": "Background", "intro": "Transforms and products.", "pages": first},
                {"title": "The identity", "intro": "Uses the [DFT](dft.md).", "pages": second},
            ],
            "glossary": glossary,
        })
        .to_string()
    };
    answer(
        &project,
        outline_task,
        &outline(
            &["chirp-identity"],
            &["dft", "convolution"],
            &["chirp-identity"],
        ),
    );
    let err = submit(&project, &outline_task.id).unwrap_err().to_string();
    assert!(
        err.contains("needs `dft`, which is in a later part"),
        "{err}"
    );
    assert!(
        err.contains("is the target and cannot go in the glossary"),
        "{err}"
    );
    assert!(
        err.contains("pages in no part and not in the glossary: dyadic-rational"),
        "{err}"
    );
    answer(
        &project,
        outline_task,
        &outline(
            &["convolution", "dft"],
            &["chirp-identity"],
            &["dyadic-rational"],
        ),
    );
    submit(&project, &outline_task.id).unwrap();

    // Notation: a symbol missing from its page's mathematics is rejected.
    let symbols = |second: &str| {
        json!({"symbols": [
            {"symbol": "2^k", "meaning": "a power of two", "page": "dyadic-rational"},
            {"symbol": second, "meaning": "the exponent", "page": "chirp-identity"},
        ]})
        .to_string()
    };
    answer(&project, notation_task, &symbols("\\omega"));
    let err = submit(&project, &notation_task.id).unwrap_err().to_string();
    assert!(
        err.contains("`\\omega` does not appear in the mathematics of `chirp-identity`"),
        "{err}"
    );
    answer(&project, notation_task, &symbols("n k"));
    submit(&project, &notation_task.id).unwrap();

    // Done: the docs are built, grouped into the two parts, with a glossary and notation.
    let Next::Done(report) = next(&project).unwrap() else {
        panic!("expected done");
    };
    assert_eq!(report.pages.len(), 4);
    assert_eq!(report.parts, Some(2));
    assert_eq!((report.glossary, report.symbols), (1, 2));
    let chirp = read(&project, "docs/chirp-identity.md");
    assert!(chirp.starts_with("# Chirp identity\n\n!!! info \"Before this page\""));
    let dyadic = "(glossary.md#dyadic-rational \"What Dyadic rational is.\")";
    assert!(
        chirp.contains(&format!("[Dyadic rational]{dyadic}")),
        "{chirp}"
    );
    assert!(chirp.contains(&format!("[dyadic]{dyadic}")));
    assert!(chirp.contains("click n2 href \"../glossary/#dyadic-rational\""));
    assert!(!project.root.join("docs/dyadic-rational.md").exists());
    let glossary = read(&project, "docs/glossary.md");
    assert!(glossary.contains("## Dyadic rational { #dyadic-rational }\n\nA number of the form"));
    assert!(glossary.contains("Used by [Chirp identity](chirp-identity.md"));
    let convolution = read(&project, "docs/convolution.md");
    assert!(convolution.contains(
        "the [discrete Fourier transform](dft.md \"What Discrete Fourier transform is.\")"
    ));
    let notation = read(&project, "docs/notation.md");
    assert!(notation.contains(&format!(
        "- $2^k$ — a power of two — [Dyadic rational]{dyadic}"
    )));
    assert!(chirp.contains("```mermaid\nflowchart LR\n"));
    assert!(chirp.contains("    > — [Notes](https://example.org/notes)"));
    let dft = read(&project, "docs/dft.md");
    assert!(dft.contains("Assumed known: Complex numbers."));
    assert!(dft.contains("Used by:\n\n- [Chirp identity](chirp-identity.md"));
    let part = read(&project, "docs/background/index.md");
    assert!(part.starts_with(
        "# Background\n\nTransforms and products.\n\n## Pages\n\n1. [Discrete Fourier transform]"
    ));
    let intro = read(&project, "docs/the-identity/index.md");
    assert!(intro.contains("Uses the [DFT](../dft.md \"What Discrete Fourier transform is.\")"));
    let index = read(&project, "docs/index.md");
    assert!(index.contains("The [chirp identity](chirp-identity.md \"What Chirp identity is.\")"));
    assert!(index.contains("### 2. [The identity](the-identity/index.md)"));
    assert!(index.contains("p0 --> p1"));
    assert!(index.contains("the [glossary](glossary.md) defines minor terms"));
    assert!(index.contains("### [Notation](notation.md)"));
    let toml = read(&project, "zensical.toml");
    assert!(
        toml.contains("{ Background = [\n    \"background/index.md\","),
        "{toml}"
    );
    assert!(toml.contains("navigation.indexes"));
    assert!(toml.contains("{ Glossary = \"glossary.md\" }") && toml.contains("Notation"));
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
    let last = tasks(next(&project).unwrap());
    assert_eq!(last[0].kind, Kind::Outline);
    let parts = json!({"overview": "o", "parts": [{"title": "All", "intro": "i",
                       "pages": ["dft", "convolution", "chirp-identity"]}]});
    answer(&project, &last[0], &parts.to_string());
    submit(&project, &last[0].id).unwrap();
    answer(&project, &last[1], "{\"symbols\": []}");
    submit(&project, &last[1].id).unwrap();
    assert!(matches!(next(&project).unwrap(), Next::Done(_)));
}

/// Handles of the links in a page as a simulated reader sees it.
fn handles(page: &str) -> Vec<String> {
    page.split("](")
        .skip(1)
        .filter_map(|rest| rest.split(')').next())
        .map(str::to_string)
        .collect()
}

#[test]
fn simulated_readers_end_to_end() {
    let temp = tempfile::tempdir().unwrap();
    let project = setup_with(temp.path(), 1, "[simulation]\nrounds = 2\nbudget = 3\n");
    let batch = tasks(next(&project).unwrap());
    answer(&project, &batch[0], &extract_answer("a chirp identity"));
    submit(&project, &batch[0].id).unwrap();
    for t in tasks(next(&project).unwrap()) {
        let body = match page(&t) {
            "chirp-identity" => "The chirp identity splits $nk$ using the [DFT](dft.md).",
            "dft" => "The discrete Fourier transform evaluates at roots of unity.",
            _ => "Convolution adds products along diagonals.",
        };
        answer(&project, &t, body);
        submit(&project, &t.id).unwrap();
    }
    for t in tasks(next(&project).unwrap()) {
        answer(&project, &t, "{\"gaps\": []}");
        submit(&project, &t.id).unwrap();
    }
    for t in tasks(next(&project).unwrap()) {
        let text = match t.kind {
            Kind::Outline => json!({"overview": "The [chirp identity](chirp-identity.md).",
                "parts": [{"title": "All", "intro": "i",
                           "pages": ["dft", "convolution", "chirp-identity"]}]}),
            _ => json!({"symbols": []}),
        };
        answer(&project, &t, &text.to_string());
        submit(&project, &t.id).unwrap();
    }

    // Goals: a transfer key point copied from a page is rejected.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch[0].kind, Kind::Goals);
    let goals = |point: &str| {
        json!({"goals": [{"question": "How does the chirp identity use the DFT?",
            "kind": "transfer", "key_points": [point, "It is about roots of unity"],
            "pages": ["chirp-identity", "dft"]}]})
        .to_string()
    };
    answer(
        &project,
        &batch[0],
        &goals("The discrete Fourier transform evaluates at roots"),
    );
    let err = submit(&project, &batch[0].id).unwrap_err().to_string();
    assert!(err.contains("appears word for word on `dft`"), "{err}");
    answer(&project, &batch[0], &goals("It rewrites the product nk"));
    submit(&project, &batch[0].id).unwrap();

    // Round 1: a reader from the front page, one deep-linked, and the baseline.
    let batch = tasks(next(&project).unwrap());
    let ids: Vec<&str> = batch.iter().map(|t| t.id.as_str()).collect();
    assert!(ids[0].ends_with("read-g1-reader-home") && ids[1].ends_with("read-g1-reader-deep"));
    assert!(ids[2].ends_with("baseline-g1-reader"));
    let file = read(&project, batch[0].file());
    assert!(
        !file.contains("chirp identity"),
        "the question is renamed: {file}"
    );
    assert!(file.contains(&format!("exegenius read {} home", batch[0].id)));

    // The reader sees invented names and handles, within its budget.
    let home = read_page(&project, &batch[0].id, "home").unwrap();
    assert!(!home.to_lowercase().contains("chirp identity"), "{home}");
    let chirp = handles(&home)[0].clone();
    let seen = read_page(&project, &batch[0].id, &chirp).unwrap();
    assert!(
        seen.contains("splits $nk$") && !seen.contains("dft.md"),
        "{seen}"
    );
    let nonce = seen
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("# ")
        .to_string();
    assert!(!nonce.to_lowercase().contains("chirp"));
    assert!(
        search_pages(&project, &batch[0].id, "chirp")
            .unwrap()
            .starts_with("no pages")
    );
    assert!(
        search_pages(&project, &batch[0].id, &nonce)
            .unwrap()
            .contains(&chirp)
    );
    read_page(&project, &batch[0].id, "home").unwrap();
    let err = read_page(&project, &batch[0].id, &chirp)
        .unwrap_err()
        .to_string();
    assert!(err.contains("page budget spent"), "{err}");
    assert!(read_page(&project, &batch[2].id, "home").is_err());

    // A citation must be on a page the reader opened; naming a real name is a leak.
    let quote = format!("The {} splits", nonce.to_lowercase());
    let reader = |cite_page: &str| {
        json!({"outcome": "answered", "answer": "The chirp identity rewrites nk.",
            "citations": [{"page": cite_page, "quote": quote}],
            "stuck": [{"page": chirp, "quote": "using the", "note": "what the DFT does here"}],
            "reason": "fairly sure"})
        .to_string()
    };
    answer(&project, &batch[0], &reader("home"));
    let err = submit(&project, &batch[0].id).unwrap_err().to_string();
    assert!(err.contains("is not on `home` word for word"), "{err}");
    answer(&project, &batch[0], &reader(&chirp));
    let warnings = submit(&project, &batch[0].id).unwrap();
    assert!(
        warnings[0].contains("names `chirp-identity` by its real name"),
        "{warnings:?}"
    );
    let deep = read_page(&project, &batch[1].id, &chirp).unwrap();
    assert!(deep.contains("Builds on"));
    let gave_up = json!({"outcome": "gave_up", "answer": "", "citations": [], "stuck": [],
                         "reason": "never found what nk is"});
    answer(&project, &batch[1], &gave_up.to_string());
    submit(&project, &batch[1].id).unwrap();
    answer(
        &project,
        &batch[2],
        "{\"answer\": \"The background does not cover this.\"}",
    );
    submit(&project, &batch[2].id).unwrap();

    // Grading: two answers, blind; a grade for a missing key point is rejected.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch.len(), 1);
    let file = read(&project, batch[0].file());
    assert!(file.contains("### Answer A") && file.contains("### Answer B"));
    assert!(file.contains("The chirp identity rewrites nk."));
    let reader_first = file.find("rewrites nk").unwrap() < file.find("does not cover").unwrap();
    let (r, b) = if reader_first { ("A", "B") } else { ("B", "A") };
    let grades = |n: usize| {
        json!({"grades": [{"label": r, "covered": [n], "wrong": []},
                          {"label": b, "covered": [], "wrong": []}]})
        .to_string()
    };
    answer(&project, &batch[0], &grades(3));
    let err = submit(&project, &batch[0].id).unwrap_err().to_string();
    assert!(err.contains("key point 3 does not exist"), "{err}");
    answer(&project, &batch[0], &grades(1));
    submit(&project, &batch[0].id).unwrap();

    // Repairs: the stuck point, the give-up and the missed key point become rewrite notes.
    let batch = tasks(next(&project).unwrap());
    assert!(batch.iter().all(|t| matches!(t.kind, Kind::Write { .. })));
    let chirp_write = batch.iter().find(|t| page(t) == "chirp-identity").unwrap();
    let file = read(&project, chirp_write.file());
    assert!(
        file.contains("got stuck at \"using the\": what the DFT does here"),
        "{file}"
    );
    assert!(file.contains("gave up on this page: never found what nk is"));
    assert!(file.contains("could not work out: It is about roots of unity"));
    for t in &batch {
        answer(
            &project,
            t,
            "A clearer page, with the [DFT](dft.md) where needed.",
        );
        submit(&project, &t.id).unwrap();
    }

    // Round 2: readers only (the baseline is kept); both give up; then done.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch.len(), 2);
    assert!(batch.iter().all(|t| matches!(t.kind, Kind::Read { .. })));
    for t in &batch {
        read_page(&project, &t.id, "home").unwrap();
        answer(&project, t, &gave_up.to_string());
        submit(&project, &t.id).unwrap();
    }
    assert!(matches!(next(&project).unwrap(), Next::Done(_)));
    let simulation = read(&project, "exegenius/simulation.json");
    assert_eq!(simulation.matches("\"task\":").count(), 5);
    let report = read(&project, "exegenius/report.md");
    assert!(report.contains("| 1 | 2 | 1 | 1 | 25% | 1 |"), "{report}");
    assert!(report.contains("| 2 | 2 | 0 | 2 | 0% | 0 |"), "{report}");
    assert!(report.contains("| baseline (no wiki) | 1 | | | 0% | |"));
    assert!(report.contains("Path: Front page → Chirp identity → Front page"));
    assert!(report.contains("- Stuck on Chirp identity: \"using the\" — what the DFT does here"));
    assert!(report.contains("Leaks (real names the reader could not have read): Chirp identity"));
    assert!(!project.root.join("docs/report.md").exists());
}

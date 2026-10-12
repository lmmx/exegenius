//! The task loop end to end, with scripted answers standing in for the agent: extract,
//! write (with the symbols each page binds), audit (finding an unknown term, and accounting
//! for every symbol), extend, rewrite, a clean audit round, outline and an overload verdict,
//! build; and the simulated readers: goals, reading through the CLI, a blind grade, repairs
//! and a second round.

use std::path::Path;

use exegenius_core::project::Project;
use exegenius_core::task::{
    Kind, Next, Task, next, read_page, redo_background, search_pages, submit,
};
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

/// The plan of the argument: to show the chirp identity, from its prerequisites, by two
/// steps the notes report in this order (or the reverse).
fn argument_answer(from: &[&str], reversed: bool) -> String {
    let mut steps = vec![
        json!({"id": "S1", "intention": "infer", "claim": "Convolution multiplies transforms.",
               "quote": {"source": "notes", "quote": "Convolution of two sequences multiplies"}}),
        json!({"id": "S2", "intention": "infer", "claim": "The chirp identity reaches a convolution.",
               "quote": {"source": "notes", "quote": "through a chirp identity to convolution"}}),
    ];
    if reversed {
        steps.reverse();
    }
    json!({"root": {
        "id": "S0", "intention": "show", "page": "chirp-identity", "from": from,
        "claim": "The chirp identity turns a DFT into a convolution.",
        "method": "rewrite the transform as a convolution",
        "motivation": {"text": "Convolutions are fast.", "source": "explicit",
                       "quote": {"source": "notes", "quote": "converts these through a chirp identity"}},
        "children": steps,
    }})
    .to_string()
}

/// Answer the background task `next` hands out after the extract, adding no concepts, and
/// pass the one `next` that shows it.
fn background(project: &Project) {
    let batch = tasks(next(project).unwrap());
    assert_eq!(batch[0].kind, Kind::Background);
    let answer_text = json!({"summary": "Complex arithmetic.", "known": []});
    answer(project, &batch[0], &answer_text.to_string());
    submit(project, &batch[0].id).unwrap();
    assert!(matches!(next(project).unwrap(), Next::Background { .. }));
}

/// Answer the argument task `next` hands out after an extract or extend.
fn argue(project: &Project, from: &[&str]) {
    let batch = tasks(next(project).unwrap());
    assert_eq!(batch[0].kind, Kind::Argument);
    answer(project, &batch[0], &argument_answer(from, false));
    submit(project, &batch[0].id).unwrap();
}

fn tasks(next: Next) -> Vec<Task> {
    match next {
        Next::Tasks { tasks, .. } => tasks,
        Next::Done(_) | Next::Background { .. } => panic!("expected tasks"),
    }
}

/// An audit answer with no gaps and no symbols.
const CLEAN: &str = "{\"symbols\": [], \"gaps\": []}";

fn answer(project: &Project, task: &Task, text: &str) {
    std::fs::write(project.root.join(task.answer()), text).unwrap();
    // Write tasks also take the symbols the page binds and its relations: none, unless the
    // test gave some.
    if let Some(fields) = task.fields() {
        let path = project.root.join(fields);
        if !path.exists() {
            std::fs::write(path, "{\"symbols\": [], \"relations\": []}").unwrap();
        }
    }
}

fn fields(
    project: &Project,
    task: &Task,
    symbols: serde_json::Value,
    relations: serde_json::Value,
) {
    let path = project.root.join(task.fields().unwrap());
    let fields = json!({ "symbols": symbols, "relations": relations });
    std::fs::write(path, fields.to_string()).unwrap();
}

fn bindings(project: &Project, task: &Task, symbols: serde_json::Value) {
    fields(project, task, symbols, json!([]));
}

fn relation(prerequisite: &str, predicate: &str, quote: &str) -> serde_json::Value {
    json!({"prerequisite": prerequisite, "predicate": predicate, "quote": quote})
}

/// The sentence a chirp page gives for its two first prerequisites, and their relations.
const CHIRP_USES: &str =
    "It is computed with the [DFT](dft.md) and reduced to [convolution](convolution.md)";

fn chirp_relations() -> Vec<serde_json::Value> {
    vec![
        relation(
            "dft",
            "is computed with",
            "It is computed with the [DFT](dft.md)",
        ),
        relation(
            "convolution",
            "is reduced to",
            "reduced to [convolution](convolution.md)",
        ),
    ]
}

fn binding(symbol: &str, name: &str, quote: &str) -> serde_json::Value {
    json!({"symbol": symbol, "name": name, "meaning": format!("The {name}."),
           "relation": "direct", "quote": quote})
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

    // Background: a planned concept cannot be made known; a concept the sources rely on that
    // the profile covers is added as known. `next` then shows the reader once; a correction
    // takes the answer back out and gives the task again with it.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch[0].kind, Kind::Background);
    assert!(read(&project, batch[0].file()).contains("`dft`: Discrete Fourier transform"));
    let known = |name: &str| {
        json!({
            "summary": "Complex arithmetic and sequences, as in a first course.",
            "known": [{"name": name, "aliases": ["list"], "gloss": "An ordered list."}],
        })
        .to_string()
    };
    answer(&project, &batch[0], &known("Convolution"));
    let err = submit(&project, &batch[0].id).unwrap_err().to_string();
    assert!(err.contains("stays as planned"), "{err}");
    answer(&project, &batch[0], &known("Sequence"));
    submit(&project, &batch[0].id).unwrap();
    let Next::Background {
        profile,
        known: shown,
    } = next(&project).unwrap()
    else {
        panic!("expected the background to be shown");
    };
    assert!(profile.starts_with("Complex numbers\n\nFor these sources: Complex arithmetic"));
    assert_eq!(shown, vec!["Sequence"]);
    redo_background(&project, "Sequences are not known.").unwrap();
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch[0].kind, Kind::Background);
    let file = read(&project, batch[0].file());
    assert!(file.contains("## Correction") && file.contains("Sequences are not known."));
    assert!(
        !file.contains("`sequence`"),
        "the earlier answer is taken back out"
    );
    let none = json!({"summary": "Complex arithmetic only.", "known": []});
    answer(&project, &batch[0], &none.to_string());
    submit(&project, &batch[0].id).unwrap();
    assert!(matches!(next(&project).unwrap(), Next::Background { .. }));

    // Argument: steps out of the notes' order are rejected; then the plan is accepted.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch[0].kind, Kind::Argument);
    answer(
        &project,
        &batch[0],
        &argument_answer(&["dft", "convolution"], true),
    );
    let err = submit(&project, &batch[0].id).unwrap_err().to_string();
    assert!(
        err.contains("step `S1` quotes the sources before step `S2`"),
        "{err}"
    );
    answer(
        &project,
        &batch[0],
        &argument_answer(&["dft", "convolution"], false),
    );
    submit(&project, &batch[0].id).unwrap();

    // Write: three independent tasks; a link to a missing page is rejected, and so are a
    // missing symbols file and a binding whose quote does not contain its symbol.
    let batch = tasks(next(&project).unwrap());
    let pages: Vec<&str> = batch.iter().map(page).collect();
    assert_eq!(pages, vec!["dft", "convolution", "chirp-identity"]);
    for t in &batch {
        let body = if page(t) == "chirp-identity" {
            "It is computed with the [DFT](dft.md) and reduced to [convolution](convolution.md), \
             with dyadic scaling of $n$ points."
        } else if page(t) == "convolution" {
            "Convolution is what the discrete Fourier transform makes pointwise, as in $x^2$."
        } else {
            "# Title to strip\n\nHere $x$ is the input sequence, so $x^2$ is its square."
        };
        if page(t) == "chirp-identity" {
            answer(&project, t, "See [elsewhere](bogus.md).");
            let err = submit(&project, &t.id).unwrap_err().to_string();
            assert!(err.contains("bogus.md"), "{err}");
            assert!(err.contains("missing: `dft`, `convolution`"), "{err}");
            fields(&project, t, json!([]), json!(chirp_relations()));
        }
        if page(t) == "dft" {
            std::fs::write(project.root.join(t.answer()), body).unwrap();
            let err = submit(&project, &t.id).unwrap_err().to_string();
            assert!(err.contains("write the symbols the page binds"), "{err}");
            bindings(
                &project,
                t,
                json!([binding("x", "input sequence", "its square")]),
            );
            let err = submit(&project, &t.id).unwrap_err().to_string();
            assert!(
                err.contains("`x`: the quote must cover the symbol"),
                "{err}"
            );
            let quote = "Here $x$ is the input sequence";
            bindings(
                &project,
                t,
                json!([binding("$x$", "input sequence", quote)]),
            );
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

    // Audit round 1, submitted concurrently: one unknown term on the chirp page. The dft
    // audit must account for the symbol its writer bound.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch.len(), 3);
    assert!(batch.iter().all(|t| matches!(t.kind, Kind::Audit { .. })));
    let audit_file = read(&project, batch[2].file());
    assert!(audit_file.contains("with dyadic scaling") && audit_file.contains("- Complex numbers"));
    answer(&project, &batch[0], CLEAN);
    let err = submit(&project, &batch[0].id).unwrap_err().to_string();
    assert!(err.contains("missing: x"), "{err}");
    let x_here = json!({"symbol": "x", "anchor": {"kind": "here",
                        "quote": "Here $x$ is the input sequence"}});
    std::thread::scope(|s| {
        for t in &batch {
            let project = &project;
            let x_here = &x_here;
            s.spawn(move || {
                let audit = match page(t) {
                    "chirp-identity" => json!({"symbols": [], "gaps": [{"term": "dyadic",
                        "quote": "dyadic scaling", "kind": "term", "note": "not defined"}]}),
                    "dft" => json!({"symbols": [x_here], "gaps": []}),
                    _ => json!({"symbols": [], "gaps": []}),
                };
                answer(project, t, &audit.to_string());
                submit(project, &t.id).unwrap();
            });
        }
    });

    // Extend: the candidate becomes a concept and a prerequisite of the chirp page. Its
    // title has mathematics, whose symbols the concept binds; a symbol not in the title is
    // rejected.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch[0].kind, Kind::Extend);
    assert!(read(&project, batch[0].file()).contains("\"dyadic\""));
    let extend = |symbol: &str| {
        let mut dyadic = concept(
            "dyadic-rational",
            "Dyadic rational",
            &[],
            false,
            Some("exact on dyadic real and imaginary parts"),
        );
        dyadic["title"] = json!("Dyadic rational $m/2^k$");
        dyadic["symbols"] = json!([{"symbol": symbol, "name": "exponent",
            "meaning": "The power of two in the denominator.", "relation": "direct"}]);
        json!({
            "concepts": [dyadic],
            "resolutions": [{"term": "dyadic", "concept": "dyadic-rational"}],
        })
        .to_string()
    };
    // Round 1 audited three pages, so extend may add one page.
    assert!(read(&project, batch[0].file()).contains("does not know: at most 1,"));
    let mut two: serde_json::Value = serde_json::from_str(&extend("k")).unwrap();
    two["concepts"]
        .as_array_mut()
        .unwrap()
        .push(concept("real", "Real part", &[], false, None));
    answer(&project, &batch[0], &two.to_string());
    let err = submit(&project, &batch[0].id).unwrap_err().to_string();
    assert!(
        err.contains("does not know: 2, over the limit of 1"),
        "{err}"
    );
    answer(&project, &batch[0], &extend("q"));
    let err = submit(&project, &batch[0].id).unwrap_err().to_string();
    assert!(
        err.contains("`q` does not occur in the mathematics of the title"),
        "{err}"
    );
    answer(&project, &batch[0], &extend("k"));
    submit(&project, &batch[0].id).unwrap();

    // The pages changed, so the argument is planned again, covering the new page.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch[0].kind, Kind::Argument);
    assert!(read(&project, batch[0].file()).contains("## Previous plan"));
    answer(
        &project,
        &batch[0],
        &argument_answer(&["dft", "convolution"], false),
    );
    let err = submit(&project, &batch[0].id).unwrap_err().to_string();
    assert!(
        err.contains("present a step nor appear in a step's `from`: dyadic-rational"),
        "{err}"
    );
    answer(
        &project,
        &batch[0],
        &argument_answer(&["dft", "convolution", "dyadic-rational"], false),
    );
    submit(&project, &batch[0].id).unwrap();

    // Write: the new page and the rewrite of the chirp page, which sees the audit notes.
    let batch = tasks(next(&project).unwrap());
    let mut pages: Vec<&str> = batch.iter().map(page).collect();
    pages.sort();
    assert_eq!(pages, vec!["chirp-identity", "dyadic-rational"]);
    // The chirp writer sees the symbols already bound: in the dyadic concept's name, and on
    // the dft page. It binds k again, with another meaning.
    for t in &batch {
        let body = if page(t) == "chirp-identity" {
            let file = read(&project, t.file());
            assert!(file.contains("Previous version") && file.contains("now a prerequisite"));
            assert!(file.contains(
                "## This page in the argument\n\nThe step: The chirp identity turns a DFT into \
                 a convolution.\nHow: rewrite the transform as a convolution\nWhy: Convolutions \
                 are fast. (the sources say so: \"converts these through a chirp identity\""
            ));
            assert!(
                file.contains("- Dyadic rational (also: dyadic)\n  - $k$ (exponent): The power")
            );
            assert!(
                file.contains(
                    "## Symbols already bound on earlier pages\n\nThe symbols in the names"
                )
            );
            assert!(file.contains(
                "- $x$ (input sequence), bound on [Discrete Fourier transform](dft.md): The input"
            ));
            answer(
                &project,
                t,
                "Exact <cite data-source=\"notes\">made up</cite>.",
            );
            let err = submit(&project, &t.id).unwrap_err().to_string();
            assert!(err.contains("\"made up\" is not in that source"), "{err}");
            let k = "Here $k$ is the frequency index";
            let mut relations = chirp_relations();
            relations.push(relation(
                "dyadic-rational",
                "values are kept",
                "values of $x$ are kept [dyadic]",
            ));
            fields(
                &project,
                t,
                json!([binding("k", "frequency index", k)]),
                json!(relations),
            );
            let body = |product: &str| {
                format!(
                    "It is computed with the [DFT](dft.md) and reduced to \
                     [convolution](convolution.md); values of $x$ are kept \
                     [dyadic](dyadic-rational.md). Here $k$ is the frequency index. So \
                     ${product}$ is exact <cite data-source=\"notes\">exact on dyadic real and \
                     imaginary parts</cite>."
                )
            };
            // A letter the previous version did not use, in no binding, is rejected.
            answer(&project, t, &body("mk"));
            let err = submit(&project, &t.id).unwrap_err().to_string();
            assert!(err.contains("did not: $m$"), "{err}");
            answer(&project, t, &body("nk"));
            submit(&project, &t.id).unwrap();
            continue;
        } else {
            "A number of the form $m/2^k$."
        };
        answer(&project, t, body);
        submit(&project, &t.id).unwrap();
    }

    // Audit round 2: every page's input changed (a page was added); no gaps. The chirp
    // audit lists the bindings of its symbols from earlier pages, and its evidence for x
    // must name the page that binds it.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch.len(), 4);
    let chirp = batch.iter().find(|t| page(t) == "chirp-identity").unwrap();
    let file = read(&project, chirp.file());
    assert!(
        file.contains("## Symbols bound on earlier pages\n\n- $x$ (input sequence)"),
        "{file}"
    );
    let chirp_audit = |x_page: &str| {
        json!({"symbols": [
            {"symbol": "x", "anchor": {"kind": "page", "page": x_page}},
            {"symbol": "k", "anchor": {"kind": "here", "quote": "Here $k$ is the frequency index"}},
            {"symbol": "n", "anchor": {"kind": "background", "note": "a count"}},
        ], "gaps": []})
        .to_string()
    };
    answer(&project, chirp, &chirp_audit("convolution"));
    let err = submit(&project, &chirp.id).unwrap_err().to_string();
    assert!(
        err.contains("`convolution` is not an earlier page that binds it"),
        "{err}"
    );
    for t in &batch {
        let audit = match page(t) {
            "chirp-identity" => chirp_audit("dft"),
            "dft" => json!({"symbols": [x_here], "gaps": []}).to_string(),
            _ => CLEAN.to_string(),
        };
        answer(&project, t, &audit);
        submit(&project, &t.id).unwrap();
    }

    // Outline, overloads and support checks, handed out together: the chirp page binds k,
    // which the name of the dyadic concept it builds on also binds, and has a citation mark,
    // which two independent judges label, since it is the first page checked.
    let batch = tasks(next(&project).unwrap());
    let labels: Vec<&str> = batch.iter().map(|t| t.label()).collect();
    assert_eq!(labels, vec!["outline", "overloads", "support", "support"]);
    let (outline_task, overloads_task) = (&batch[0], &batch[1]);
    let file = read(&project, batch[2].file());
    assert!(
        file.contains("Claim: So $nk$ is exact\n\nQuotation, from Notes: \"exact on dyadic"),
        "{file}"
    );
    let label = |verdict: &str, reason: &str| {
        json!({"labels": [{"mark": 1, "label": verdict, "reason": reason}]}).to_string()
    };
    answer(&project, &batch[2], &label("unsupported", ""));
    let err = submit(&project, &batch[2].id).unwrap_err().to_string();
    assert!(
        err.contains("quotation 1: say what is missing or wrong"),
        "{err}"
    );
    answer(
        &project,
        &batch[2],
        &label("unsupported", "the passage says nothing about nk."),
    );
    submit(&project, &batch[2].id).unwrap();
    answer(&project, &batch[3], &label("supported", ""));
    submit(&project, &batch[3].id).unwrap();
    let file = read(&project, overloads_task.file());
    assert!(
        file.contains("1. $k$\n   - on `chirp-identity`: $k$ (frequency index)")
            && file.contains("   - in the name of Dyadic rational: $k$ (exponent)"),
        "{file}"
    );

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

    // Overloads: every candidate needs a verdict. A letter with two meanings becomes a
    // rewrite note, and the rewrite is handed out (not audited) before the build.
    answer(&project, overloads_task, "{\"verdicts\": []}");
    let err = submit(&project, &overloads_task.id)
        .unwrap_err()
        .to_string();
    assert!(err.contains("no verdict on candidate 1"), "{err}");
    let verdict = json!({"verdicts": [{"candidate": 1, "same": false,
                                       "note": "use j for the frequency index"}]});
    answer(&project, overloads_task, &verdict.to_string());
    submit(&project, &overloads_task.id).unwrap();
    let batch = tasks(next(&project).unwrap());
    assert_eq!(page(&batch[0]), "chirp-identity");
    let file = read(&project, batch[0].file());
    assert!(
        file.contains(
            "$k$ means frequency index here, but exponent in the name of Dyadic rational, \
             which this page builds on: use j for the frequency index"
        ),
        "{file}"
    );
    assert!(file.contains(
        "The quotation does not support the claim it is attached to (unsupported): \"So $nk$ \
         is exact\" cites \"exact on dyadic real and imaginary parts\": the passage says \
         nothing about nk."
    ));
    let mut relations = chirp_relations();
    relations.push(relation(
        "dyadic-rational",
        "values are kept",
        "values of $x$ are kept [dyadic]",
    ));
    fields(
        &project,
        &batch[0],
        json!([binding(
            "j",
            "frequency index",
            "Here $j$ is the frequency index"
        )]),
        json!(relations),
    );
    // The notes are about a letter and a citation, and no audit follows: a claim on
    // anything else is refused.
    let page_text = "It is computed with the [DFT](dft.md) and reduced to \
                     [convolution](convolution.md); values of $x$ are kept \
                     [dyadic](dyadic-rational.md). Here $j$ is the frequency index. So $nj$ is \
                     exact <cite data-source=\"notes\">exact on dyadic real and imaginary \
                     parts</cite>.";
    answer(
        &project,
        &batch[0],
        &format!("{page_text} The sequence $x$ needs no padding before it is transformed."),
    );
    let err = submit(&project, &batch[0].id).unwrap_err().to_string();
    assert!(
        err.contains("may not add other claims: \"The sequence $x$ needs no padding"),
        "{err}"
    );
    answer(&project, &batch[0], page_text);
    submit(&project, &batch[0].id).unwrap();

    // The rewritten claim is a new mark, so the page gets its second and last check.
    let batch = tasks(next(&project).unwrap());
    let labels: Vec<&str> = batch.iter().map(|t| t.label()).collect();
    assert_eq!(labels, vec!["support"]);
    answer(
        &project,
        &batch[0],
        &label("partial", "only the exactness."),
    );
    submit(&project, &batch[0].id).unwrap();
    let support = read(&project, "exegenius/support.json");
    assert!(support.contains("\"partial\"") && support.contains("\"second\""));

    // Done: the docs are built, grouped into the two parts, with a glossary and notation.
    let Next::Done(report) = next(&project).unwrap() else {
        panic!("expected done");
    };
    assert_eq!(report.pages.len(), 4);
    assert_eq!(report.parts, Some(2));
    assert_eq!((report.glossary, report.symbols), (1, 1));
    let chirp = read(&project, "docs/chirp-identity.md");
    assert!(chirp.starts_with("# Chirp identity\n\n!!! info \"Before this page\""));
    // x is shown with its meaning; k, which the page no longer uses, is not.
    let dft_link = "[Discrete Fourier transform](dft.md \"What Discrete Fourier transform is.\")";
    assert!(
        chirp.contains(&format!(
            "??? note \"Symbols from earlier pages\"\n\n    - $x$ — input sequence: The input \
             sequence. (on {dft_link})\n\n"
        )),
        "{chirp}"
    );
    assert!(
        chirp.contains("is exact<cite class=\"exegenius-ref\" data-source=\"notes\"")
            && chirp.contains("> (see Notes)</cite>."),
        "{chirp}"
    );
    assert!(project.root.join("docs/stylesheets/exegenius.css").exists());
    let dyadic = "(glossary.md#dyadic-rational \"What Dyadic rational is.\")";
    assert!(
        chirp.contains(
            "    Chirp identity *is computed with* [Discrete Fourier transform](dft.md \"It is \
             computed with the DFT\"), *is reduced to* [Convolution](convolution.md \"reduced \
             to convolution\") and *values are kept* [Dyadic rational](glossary.md#dyadic-rational \
             \"values of $x$ are kept dyadic\")."
        ),
        "{chirp}"
    );
    assert!(chirp.contains(&format!("[dyadic]{dyadic}")));
    assert!(chirp.contains("click n2 href \"../glossary/#dyadic-rational\""));
    assert!(!project.root.join("docs/dyadic-rational.md").exists());
    let glossary = read(&project, "docs/glossary.md");
    assert!(
        glossary
            .contains("## Dyadic rational $m/2^k$ { #dyadic-rational }\n\nA number of the form")
    );
    assert!(glossary.contains("Used by [Chirp identity](chirp-identity.md"));
    let convolution = read(&project, "docs/convolution.md");
    assert!(convolution.contains(
        "the [discrete Fourier transform](dft.md \"What Discrete Fourier transform is.\")"
    ));
    let notation = read(&project, "docs/notation.md");
    assert!(
        notation.contains(&format!(
            "- $x$ — **input sequence**: The input sequence. Bound on {dft_link}; used on \
             [Chirp identity](chirp-identity.md \"What Chirp identity is.\")."
        )),
        "{notation}"
    );
    assert!(chirp.contains("```mermaid\nflowchart LR\n"));
    assert!(chirp.contains("    > — [Notes](https://example.org/notes)"));
    let dft = read(&project, "docs/dft.md");
    assert!(dft.contains("Assumed known: Complex numbers."));
    assert!(
        dft.contains(
            "!!! abstract \"Where this fits\"\n\n    - Relied on to show: The chirp identity \
             turns a DFT into a convolution. ([Chirp identity](chirp-identity.md"
        ),
        "{dft}"
    );
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
    assert!(index.contains(
        "## The argument\n\n**Main result:** The chirp identity turns a DFT into a convolution.\n\n\
         **How:** rewrite the transform as a convolution **Why:** Convolutions are fast.\n\n\
         1. Convolution multiplies transforms.\n2. The chirp identity reaches a convolution.\n"
    ));
    assert!(index.find("## The argument") < index.find("## Contents"));
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
fn last_round_notes_are_dropped_then_built() {
    let temp = tempfile::tempdir().unwrap();
    let project = setup(temp.path(), 1);
    let batch = tasks(next(&project).unwrap());
    answer(&project, &batch[0], &extract_answer("a chirp identity"));
    submit(&project, &batch[0].id).unwrap();
    background(&project);
    argue(&project, &["dft", "convolution"]);
    for t in tasks(next(&project).unwrap()) {
        if page(&t) == "chirp-identity" {
            fields(&project, &t, json!([]), json!(chirp_relations()));
            answer(&project, &t, &format!("Some explanation. {CHIRP_USES}."));
        } else {
            answer(&project, &t, "Some explanation.");
        }
        submit(&project, &t.id).unwrap();
    }
    for t in tasks(next(&project).unwrap()) {
        let gaps = json!({"symbols": [], "gaps": [{"term": "step", "quote": "Some explanation",
                                    "kind": "step", "note": "too vague"}]});
        answer(&project, &t, &gaps.to_string());
        submit(&project, &t.id).unwrap();
    }
    // The single round found gaps, but no audit would check rewrites for them: the notes
    // are dropped, the pages stay as audited, and the outline comes next.
    let last = tasks(next(&project).unwrap());
    assert_eq!(last[0].kind, Kind::Outline);
    let parts = json!({"overview": "o", "parts": [{"title": "All", "intro": "i",
                       "pages": ["dft", "convolution", "chirp-identity"]}]});
    answer(&project, &last[0], &parts.to_string());
    submit(&project, &last[0].id).unwrap();
    assert!(matches!(next(&project).unwrap(), Next::Done(_)));
}

#[test]
fn rewrites_keep_symbols_later_pages_rely_on() {
    let temp = tempfile::tempdir().unwrap();
    let project = setup(temp.path(), 2);
    let batch = tasks(next(&project).unwrap());
    answer(&project, &batch[0], &extract_answer("a chirp identity"));
    submit(&project, &batch[0].id).unwrap();
    background(&project);
    argue(&project, &["dft", "convolution"]);
    let x_quote = "Here $x$ is the input sequence";
    for t in tasks(next(&project).unwrap()) {
        match page(&t) {
            "dft" => {
                bindings(
                    &project,
                    &t,
                    json!([binding("x", "input sequence", x_quote)]),
                );
                answer(&project, &t, &format!("{x_quote}. Some explanation."));
            }
            "chirp-identity" => {
                fields(&project, &t, json!([]), json!(chirp_relations()));
                answer(&project, &t, &format!("{CHIRP_USES}. It acts on $x$."));
            }
            _ => answer(&project, &t, "Some explanation."),
        }
        submit(&project, &t.id).unwrap();
    }
    // The chirp page takes x from the DFT page; the DFT page has a step to explain.
    for t in tasks(next(&project).unwrap()) {
        let x = |anchor: serde_json::Value| json!([{"symbol": "x", "anchor": anchor}]);
        let audit = match page(&t) {
            "dft" => json!({"symbols": x(json!({"kind": "here", "quote": x_quote})),
                            "gaps": [{"term": "step", "quote": "Some explanation",
                                      "kind": "step", "note": "too vague"}]}),
            "chirp-identity" => {
                json!({"symbols": x(json!({"kind": "page", "page": "dft"})), "gaps": []})
            }
            _ => json!({"symbols": [], "gaps": []}),
        };
        answer(&project, &t, &audit.to_string());
        submit(&project, &t.id).unwrap();
    }
    let batch = tasks(next(&project).unwrap());
    assert_eq!(page(&batch[0]), "dft");
    bindings(&project, &batch[0], json!([]));
    answer(
        &project,
        &batch[0],
        "A clearer explanation, with no symbols.",
    );
    let err = submit(&project, &batch[0].id).unwrap_err().to_string();
    assert!(
        err.contains(
            "no longer binds symbols that later pages take their meaning from: $x$ \
                      (used on `chirp-identity`)"
        ),
        "{err}"
    );
    bindings(
        &project,
        &batch[0],
        json!([binding("x", "input sequence", x_quote)]),
    );
    answer(
        &project,
        &batch[0],
        &format!("{x_quote}. A clearer explanation."),
    );
    submit(&project, &batch[0].id).unwrap();
}

#[test]
fn overload_verdicts_can_change_the_earlier_page() {
    let temp = tempfile::tempdir().unwrap();
    let project = setup(temp.path(), 1);
    let batch = tasks(next(&project).unwrap());
    answer(&project, &batch[0], &extract_answer("a chirp identity"));
    submit(&project, &batch[0].id).unwrap();
    background(&project);
    argue(&project, &["dft", "convolution"]);
    let (x_input, x_chirp) = (
        "Here $x$ is the input sequence",
        "Here $x$ is the chirp factor",
    );
    for t in tasks(next(&project).unwrap()) {
        match page(&t) {
            "dft" => {
                bindings(
                    &project,
                    &t,
                    json!([binding("x", "input sequence", x_input)]),
                );
                answer(&project, &t, &format!("{x_input}."));
            }
            "chirp-identity" => {
                fields(
                    &project,
                    &t,
                    json!([binding("x", "chirp factor", x_chirp)]),
                    json!(chirp_relations()),
                );
                answer(&project, &t, &format!("{CHIRP_USES}. {x_chirp}."));
            }
            _ => answer(&project, &t, "Some explanation."),
        }
        submit(&project, &t.id).unwrap();
    }
    for t in tasks(next(&project).unwrap()) {
        let here =
            |quote: &str| json!([{"symbol": "x", "anchor": {"kind": "here", "quote": quote}}]);
        let symbols = match page(&t) {
            "dft" => here(x_input),
            "chirp-identity" => here(x_chirp),
            _ => json!([]),
        };
        answer(
            &project,
            &t,
            &json!({"symbols": symbols, "gaps": []}).to_string(),
        );
        submit(&project, &t.id).unwrap();
    }
    // The chirp page binds x again with another meaning. The judge may send the note to the
    // earlier page, which binds x on its page, but not to a page outside the candidate.
    let batch = tasks(next(&project).unwrap());
    let overloads = batch.iter().find(|t| t.kind == Kind::Overloads).unwrap();
    let verdict = |change: &str| {
        json!({"verdicts": [{"candidate": 1, "same": false, "note": "use y for the input",
                             "change": change}]})
        .to_string()
    };
    answer(&project, overloads, &verdict("convolution"));
    let err = submit(&project, &overloads.id).unwrap_err().to_string();
    assert!(
        err.contains("`change` is `chirp-identity` or `dft`"),
        "{err}"
    );
    answer(&project, overloads, &verdict("dft"));
    submit(&project, &overloads.id).unwrap();
    let judged = read(&project, "exegenius/overloads.json");
    assert!(judged.contains("\"changed\": \"dft\""), "{judged}");
    for t in batch.iter().filter(|t| t.kind == Kind::Outline) {
        let parts = json!({"overview": "o", "parts": [{"title": "All", "intro": "i",
                           "pages": ["dft", "convolution", "chirp-identity"]}]});
        answer(&project, t, &parts.to_string());
        submit(&project, &t.id).unwrap();
    }
    let batch = tasks(next(&project).unwrap());
    assert_eq!(page(&batch[0]), "dft");
    let file = read(&project, batch[0].file());
    assert!(
        file.contains("$x$ means input sequence here, but chirp factor on [Chirp"),
        "{file}"
    );
    assert!(
        file.contains("which builds on this page: use y for the input"),
        "{file}"
    );
}

/// Two audit rounds in which each page's step gap is reported again on a sentence the
/// rewrite kept, so rewrites do not halve the gaps; then what `next` hands out.
fn second_round_without_halving(stop_early: bool) -> Vec<Task> {
    let temp = tempfile::tempdir().unwrap();
    let mut project = setup(temp.path(), 3);
    if !stop_early {
        let path = temp.path().join("exegenius.toml");
        let toml =
            read(&project, &path).replace("rounds = 3\n", "rounds = 3\nstop_early = false\n");
        std::fs::write(&path, toml).unwrap();
        project = Project::load(temp.path()).unwrap();
    }
    let batch = tasks(next(&project).unwrap());
    answer(&project, &batch[0], &extract_answer("a chirp identity"));
    submit(&project, &batch[0].id).unwrap();
    background(&project);
    argue(&project, &["dft", "convolution"]);
    let write = |t: &Task, first: &str| {
        let body = if page(t) == "chirp-identity" {
            fields(&project, t, json!([]), json!(chirp_relations()));
            format!("{first} It stays the same. {CHIRP_USES}.")
        } else {
            format!("{first} It stays the same.")
        };
        answer(&project, t, &body);
        submit(&project, &t.id).unwrap();
    };
    let audit = |quote: &str| {
        for t in tasks(next(&project).unwrap()) {
            let gaps = json!({"symbols": [], "gaps": [{"term": "step", "quote": quote,
                                                       "kind": "step", "note": "unclear"}]});
            answer(&project, &t, &gaps.to_string());
            submit(&project, &t.id).unwrap();
        }
    };
    for t in tasks(next(&project).unwrap()) {
        write(&t, "Some explanation.");
    }
    audit("Some explanation");
    for t in tasks(next(&project).unwrap()) {
        write(&t, "A better explanation.");
    }
    audit("It stays the same");
    tasks(next(&project).unwrap())
}

#[test]
fn early_stopping_can_be_turned_off() {
    // On, the rounds end after round 2: its notes are dropped and the outline comes next.
    let on = second_round_without_halving(true);
    assert_eq!(on[0].kind, Kind::Outline);
    // Off, round 3 runs: the pages are rewritten for round 2's notes.
    let off = second_round_without_halving(false);
    assert!(
        off.iter().all(|t| matches!(t.kind, Kind::Write { .. })),
        "{off:?}"
    );
    assert_eq!(off.len(), 3);
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
    background(&project);
    argue(&project, &["dft", "convolution"]);
    for t in tasks(next(&project).unwrap()) {
        if page(&t) == "chirp-identity" {
            let relations = json!([
                relation(
                    "dft",
                    "splits using",
                    "The chirp identity splits $nk$ using the [DFT]"
                ),
                relation(
                    "convolution",
                    "is reduced to",
                    "It is reduced to [convolution]"
                ),
            ]);
            fields(&project, &t, json!([]), relations);
        }
        let body = match page(&t) {
            "chirp-identity" => {
                "The chirp identity splits $nk$ using the [DFT](dft.md). It is reduced to \
                 [convolution](convolution.md)."
            }
            "dft" => "The discrete Fourier transform evaluates at roots of unity.",
            _ => "Convolution adds products along diagonals.",
        };
        answer(&project, &t, body);
        submit(&project, &t.id).unwrap();
    }
    for t in tasks(next(&project).unwrap()) {
        answer(&project, &t, CLEAN);
        submit(&project, &t.id).unwrap();
    }
    // No page binds a symbol twice, so the outline comes alone.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch.len(), 1);
    let outline = json!({"overview": "The [chirp identity](chirp-identity.md).",
        "parts": [{"title": "All", "intro": "i",
                   "pages": ["dft", "convolution", "chirp-identity"]}]});
    answer(&project, &batch[0], &outline.to_string());
    submit(&project, &batch[0].id).unwrap();

    // Goals: repair goals and held-out goals come from two tasks in one batch. A key point
    // of a holistic kind copied from a page is rejected.
    let batch = tasks(next(&project).unwrap());
    let kinds: Vec<&Kind> = batch.iter().map(|t| &t.kind).collect();
    assert_eq!(
        kinds,
        vec![
            &Kind::Goals { held_out: false },
            &Kind::Goals { held_out: true }
        ]
    );
    assert!(read(&project, batch[1].file()).contains("## Held-out goals"));
    assert!(read(&project, batch[0].file()).contains("## The argument's steps"));
    let goals = |point: &str| {
        let goal = |question: &str| {
            json!({"question": question, "kind": "transfer", "pages": ["chirp-identity", "dft"],
                "key_points": [{"point": point, "role": "claim"},
                               {"point": "It is about roots of unity", "role": "reason"}]})
        };
        json!({"goals": [goal("How does the chirp identity use the DFT?"),
                         goal("What does the chirp identity need from the DFT?"),
                         goal("Why does the chirp identity need the DFT?")]})
        .to_string()
    };
    answer(
        &project,
        &batch[0],
        &goals("The discrete Fourier transform evaluates at roots"),
    );
    let err = submit(&project, &batch[0].id).unwrap_err().to_string();
    assert!(err.contains("transfer key point"), "{err}");
    assert!(err.contains("appears word for word on `dft`"), "{err}");
    answer(&project, &batch[0], &goals("It rewrites the product nk"));
    submit(&project, &batch[0].id).unwrap();
    let held_out = json!({"goals": [{"question": "What does the chirp identity turn the DFT into?",
        "kind": "meaning", "pages": ["chirp-identity"],
        "key_points": [{"point": "A convolution", "role": "conclusion"}]}]});
    answer(&project, &batch[1], &held_out.to_string());
    submit(&project, &batch[1].id).unwrap();

    // Round 1: for each goal, a reader from the front page, one deep-linked, and the
    // baseline. The held-out goal is the last.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch.len(), 12);
    let ids: Vec<&str> = batch.iter().map(|t| t.id.as_str()).collect();
    assert!(ids[0].ends_with("read-g1-reader-home") && ids[1].ends_with("read-g1-reader-deep"));
    assert!(ids[2].ends_with("baseline-g1-reader") && ids[11].ends_with("baseline-g4-reader"));
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
            "stuck": [{"page": chirp, "quote": "using the", "note": "what the DFT does here",
                       "level": "chain"}],
            "key_idea": {"page": cite_page, "statement": "It splits nk.", "quote": quote},
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
    assert!(home.contains("## The argument"), "{home}");
    assert!(deep.contains("*splits using* ["), "{deep}");
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
    // Goals 2 and 3: the readers give up. The held-out goal: a reader gets stuck.
    for t in &batch[3..] {
        let text = match &t.kind {
            Kind::Baseline { .. } => json!({"answer": "Not covered."}),
            Kind::Read { goal: 3, .. } => {
                read_page(&project, &t.id, &chirp).unwrap();
                json!({"outcome": "gave_up", "answer": "", "citations": [], "reason": "lost",
                    "stuck": [{"page": chirp, "quote": "using the", "level": "step",
                               "note": "held-out confusion"}]})
            }
            _ => {
                read_page(&project, &t.id, "home").unwrap();
                gave_up.clone()
            }
        };
        answer(&project, t, &text.to_string());
        submit(&project, &t.id).unwrap();
    }

    // Grading: one task per goal, blind; a grade for a missing key point, or a key point
    // both covered and contradicted, is rejected.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch.len(), 4);
    for t in &batch[1..] {
        let file = read(&project, t.file());
        let labels: Vec<String> = file
            .lines()
            .filter_map(|l| l.strip_prefix("### Answer "))
            .map(|l| {
                json!({"label": l, "covered": [], "contradicted": [], "wrong": []}).to_string()
            })
            .collect();
        answer(
            &project,
            t,
            &format!("{{\"grades\": [{}]}}", labels.join(",")),
        );
        submit(&project, &t.id).unwrap();
    }
    let batch = &batch[..1];
    let file = read(&project, batch[0].file());
    assert!(file.contains("### Answer A") && file.contains("### Answer B"));
    assert!(file.contains("The chirp identity rewrites nk."));
    let reader_first = file.find("rewrites nk").unwrap() < file.find("does not cover").unwrap();
    let (r, b) = if reader_first { ("A", "B") } else { ("B", "A") };
    let grades = |n: usize, contradicted: usize| {
        json!({"grades": [{"label": r, "covered": [n], "contradicted": [contradicted], "wrong": []},
                          {"label": b, "covered": [], "contradicted": [], "wrong": []}]})
        .to_string()
    };
    answer(&project, &batch[0], &grades(3, 1));
    let err = submit(&project, &batch[0].id).unwrap_err().to_string();
    assert!(err.contains("key point 3 does not exist"), "{err}");
    answer(&project, &batch[0], &grades(1, 1));
    let err = submit(&project, &batch[0].id).unwrap_err().to_string();
    assert!(
        err.contains("key point 1 is both covered and contradicted"),
        "{err}"
    );
    answer(&project, &batch[0], &grades(1, 2));
    submit(&project, &batch[0].id).unwrap();

    // Attribution: the covered key point gets the quotation that supports it; a key point
    // the answer did not cover is rejected.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch.len(), 1);
    assert!(matches!(batch[0].kind, Kind::Attribute { goal: 0, .. }));
    let file = read(&project, batch[0].file());
    assert!(
        file.contains("Covers key points: 1") && file.contains("1. \"The Chirp identity splits"),
        "{file}"
    );
    let support = |point: usize| {
        json!({"answers": [{"answer": 1, "support": [{"point": point, "quotation": 1}]}]})
            .to_string()
    };
    answer(&project, &batch[0], &support(2));
    let err = submit(&project, &batch[0].id).unwrap_err().to_string();
    assert!(
        err.contains("key point 2 is not listed as covered"),
        "{err}"
    );
    answer(&project, &batch[0], &support(1));
    submit(&project, &batch[0].id).unwrap();

    // Repairs: the stuck point, the give-up and the missed key point become rewrite notes.
    let batch = tasks(next(&project).unwrap());
    assert!(batch.iter().all(|t| matches!(t.kind, Kind::Write { .. })));
    let chirp_write = batch.iter().find(|t| page(t) == "chirp-identity").unwrap();
    let file = read(&project, chirp_write.file());
    assert!(
        file.contains(
            "got stuck at \"using the\": what the DFT does here (the reader followed the \
             steps but not why they lead where they do: say what this step is for)"
        ),
        "{file}"
    );
    // Nothing from the held-out goal reaches the writer.
    assert!(!file.contains("held-out confusion") && !file.contains("turn the DFT into"));
    assert!(file.contains("gave up on this page: never found what nk is"));
    assert!(file.contains("could not work out: It is about roots of unity"));
    for t in &batch {
        if page(t) == "chirp-identity" {
            fields(&project, t, json!([]), json!(chirp_relations()));
            answer(&project, t, &format!("A clearer page. {CHIRP_USES}."));
        } else {
            answer(
                &project,
                t,
                "A clearer page, with the [DFT](dft.md) where needed.",
            );
        }
        submit(&project, &t.id).unwrap();
    }

    // Round 2: readers only (the baselines are kept); all give up; then done.
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch.len(), 8);
    assert!(batch.iter().all(|t| matches!(t.kind, Kind::Read { .. })));
    for t in &batch {
        read_page(&project, &t.id, "home").unwrap();
        answer(&project, t, &gave_up.to_string());
        submit(&project, &t.id).unwrap();
    }
    assert!(matches!(next(&project).unwrap(), Next::Done(_)));
    let simulation = read(&project, "exegenius/simulation.json");
    assert_eq!(simulation.matches("\"task\":").count(), 20);
    let report = read(&project, "exegenius/report.md");
    assert!(
        report.contains("| 1 | 8 | 1 | 7 | 8% | 0% | 1 |"),
        "{report}"
    );
    assert!(
        report.contains("| 2 | 8 | 0 | 8 | 0% | 0% | 0 |"),
        "{report}"
    );
    assert!(report.contains("| baseline (no wiki) | 4 | | | 0% | | |"));
    assert!(report.contains("| transfer | 3 | 4% | 0% |"), "{report}");
    assert!(report.contains("| reason | 12 | 0% | 1 |"), "{report}");
    assert!(report.contains("| step | 2 |\n| chain | 1 |"), "{report}");
    assert!(report.contains("Kind: meaning, held out."));
    assert!(report.contains("Path: Front page → Chirp identity → Front page"));
    assert!(
        report
            .contains("- Stuck on Chirp identity [chain]: \"using the\" — what the DFT does here")
    );
    assert!(report.contains("- Contradicts key point 2: It is about roots of unity"));
    assert!(report.contains("| 1 | 0.5 | 75% | 1 of 1 |"), "{report}");
    assert!(
        report.contains("| Round | Words | Pages |\n|---|---|---|\n| 1 | 27 | 3 |\n| 2 | 26 | 3 |")
    );
    // The scripted rewrites are shorter, so no page grew.
    assert!(!report.contains("Pages that grew most"));
    assert!(
        report.contains("| Chirp identity | \"The Chirp identity splits\" | 1 | 1 |"),
        "{report}"
    );
    assert!(report.contains(
        "- **Chirp identity** (no agreement):\n  - It splits nk. (\"The Chirp identity splits\")"
    ));
    assert!(
        report.contains("Possible leaks (real names the reader was not shown): Chirp identity")
    );
    assert!(!project.root.join("docs/report.md").exists());
}

#[test]
fn ablation_hides_a_carrying_sentence_and_a_control() {
    let temp = tempfile::tempdir().unwrap();
    let extra = "[simulation]\nrounds = 1\nbudget = 3\nablations = 1\n";
    let project = setup_with(temp.path(), 1, extra);
    let batch = tasks(next(&project).unwrap());
    answer(&project, &batch[0], &extract_answer("a chirp identity"));
    submit(&project, &batch[0].id).unwrap();
    background(&project);
    argue(&project, &["dft", "convolution"]);
    let chirp_body = "The chirp identity splits the product using the [DFT](dft.md). It is \
                      reduced to [convolution](convolution.md). Nothing here is cited by \
                      anyone at all.";
    for t in tasks(next(&project).unwrap()) {
        if page(&t) == "chirp-identity" {
            let relations = json!([
                relation(
                    "dft",
                    "splits the product using",
                    "splits the product using the [DFT]"
                ),
                relation(
                    "convolution",
                    "is reduced to",
                    "It is reduced to [convolution]"
                ),
            ]);
            fields(&project, &t, json!([]), relations);
            answer(&project, &t, chirp_body);
        } else {
            answer(&project, &t, "A page.");
        }
        submit(&project, &t.id).unwrap();
    }
    for t in tasks(next(&project).unwrap()) {
        answer(&project, &t, CLEAN);
        submit(&project, &t.id).unwrap();
    }
    let batch = tasks(next(&project).unwrap());
    let outline = json!({"overview": "The [chirp identity](chirp-identity.md).", "parts": [{"title": "All", "intro": "i",
                         "pages": ["dft", "convolution", "chirp-identity"]}]});
    answer(&project, &batch[0], &outline.to_string());
    submit(&project, &batch[0].id).unwrap();
    let batch = tasks(next(&project).unwrap());
    let goal = json!({"question": "Why does the chirp identity use the DFT?",
        "kind": "justification", "pages": ["chirp-identity"],
        "key_points": [{"point": "It uses the DFT", "role": "claim"},
                       {"point": "Because the product splits", "role": "reason"}]});
    answer(
        &project,
        &batch[0],
        &json!({"goals": [goal, goal, goal]}).to_string(),
    );
    submit(&project, &batch[0].id).unwrap();
    let mut held_out = goal.clone();
    held_out["kind"] = json!("meaning");
    answer(
        &project,
        &batch[1],
        &json!({"goals": [held_out]}).to_string(),
    );
    submit(&project, &batch[1].id).unwrap();

    // Round 1: on goal 1, the front-page reader cites the chirp page's first sentence.
    let gave_up = json!({"outcome": "gave_up", "answer": "", "citations": [], "stuck": [],
                         "reason": "lost"});
    let batch = tasks(next(&project).unwrap());
    let mut chirp = String::new();
    for t in &batch {
        let text = match &t.kind {
            Kind::Baseline { .. } => json!({"answer": "Not covered."}),
            Kind::Read { goal: 0, start, .. } if start == "home" => {
                let home = read_page(&project, &t.id, "home").unwrap();
                chirp = handles(&home)[0].clone();
                let seen = read_page(&project, &t.id, &chirp).unwrap();
                let quote = seen
                    .lines()
                    .find(|l| l.contains("splits") && !l.contains('*'))
                    .unwrap()
                    .to_string();
                let quote = quote.split(". ").next().unwrap().to_string();
                json!({"outcome": "answered", "answer": "Because the product splits.",
                       "citations": [{"page": chirp, "quote": quote}], "stuck": [],
                       "reason": "sure"})
            }
            _ => {
                read_page(&project, &t.id, "home").unwrap();
                gave_up.clone()
            }
        };
        answer(&project, t, &text.to_string());
        submit(&project, &t.id).unwrap();
    }
    for t in tasks(next(&project).unwrap()) {
        let file = read(&project, t.file());
        let grades: Vec<String> = file
            .lines()
            .filter_map(|l| l.strip_prefix("### Answer "))
            .map(|label| {
                let block = file
                    .split(&format!("### Answer {label}\n"))
                    .nth(1)
                    .and_then(|rest| rest.split("### Answer").next())
                    .unwrap_or_default();
                let covered = if block.contains("product splits.") {
                    json!([2])
                } else {
                    json!([])
                };
                json!({"label": label, "covered": covered, "contradicted": [], "wrong": []})
                    .to_string()
            })
            .collect();
        answer(
            &project,
            &t,
            &format!("{{\"grades\": [{}]}}", grades.join(",")),
        );
        submit(&project, &t.id).unwrap();
    }
    let batch = tasks(next(&project).unwrap());
    assert!(matches!(batch[0].kind, Kind::Attribute { .. }));
    let support = json!({"answers": [{"answer": 1, "support": [{"point": 2, "quotation": 1}]}]});
    answer(&project, &batch[0], &support.to_string());
    submit(&project, &batch[0].id).unwrap();

    // Ablation: goal 1 again, once without the cited sentence and once without a control
    // sentence; each reader sees its page without its sentence.
    let batch = tasks(next(&project).unwrap());
    let ids: Vec<&str> = batch.iter().map(|t| t.id.as_str()).collect();
    assert_eq!(batch.len(), 2, "{ids:?}");
    assert!(
        ids[0].ends_with("read-g1-reader-ablate") && ids[1].ends_with("read-g1-reader-control")
    );
    // The body line: the relation sentence above it uses the same words.
    let body = |page: &str| {
        page.lines()
            .filter(|l| !l.contains('*') && !l.starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let without = read_page(&project, &batch[0].id, &chirp).unwrap();
    assert!(
        !body(&without).contains("splits") && body(&without).contains("reduced to"),
        "{without}"
    );
    let control = read_page(&project, &batch[1].id, &chirp).unwrap();
    assert!(
        body(&control).contains("splits")
            && !(body(&control).contains("reduced to") && body(&control).contains("cited by")),
        "{control}"
    );
    answer(&project, &batch[0], &gave_up.to_string());
    submit(&project, &batch[0].id).unwrap();
    let quote = control
        .lines()
        .find(|l| l.contains("splits") && !l.contains('*'))
        .unwrap();
    let quote = quote.split(". ").next().unwrap();
    let answered = json!({"outcome": "answered", "answer": "Because the product splits.",
                          "citations": [{"page": chirp, "quote": quote}], "stuck": [],
                          "reason": "sure"});
    answer(&project, &batch[1], &answered.to_string());
    submit(&project, &batch[1].id).unwrap();
    let batch = tasks(next(&project).unwrap());
    assert_eq!(batch.len(), 1);
    let grade =
        json!({"grades": [{"label": "A", "covered": [2], "contradicted": [], "wrong": []}]});
    answer(&project, &batch[0], &grade.to_string());
    submit(&project, &batch[0].id).unwrap();
    assert!(matches!(next(&project).unwrap(), Next::Done(_)));
    let report = read(&project, "exegenius/report.md");
    assert!(report.contains("## Ablations"), "{report}");
    assert!(
        report.contains("## Key steps\n\nThe hidden sentences whose loss cost readers more")
            && report.contains("- Chirp identity: \"The chirp identity splits the product using"),
        "{report}"
    );
}

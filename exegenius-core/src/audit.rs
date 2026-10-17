//! Audit tasks: list what the reader cannot follow on one page.
//!
//! The reader's knowledge is a set of concepts that code maintains: the concepts the profile
//! covers plus every page before this one in reading order. The auditor is told the reader
//! knows exactly that set, and that everything else it knows is unknown to the reader. It
//! reports the terms and steps the reader cannot follow, and accounts for every symbol in
//! the page's mathematics: where the reader learns what it denotes. Code then:
//!
//! 1. rejects the answer if a quote is not on the page, if a symbol is not in the page's
//!    mathematics, if a symbol's evidence does not hold (the quote must contain it; the
//!    earlier page or known concept must bind it), or if a symbol the page's writer bound
//!    is not accounted for (the auditor fixes the answer),
//! 2. drops terms the reader already knows,
//! 3. turns a term naming a concept in the graph into a prerequisite edge (or a rewrite
//!    note when the edge exists already or would create a cycle),
//! 4. turns any other term into a candidate concept for an extend task, and steps and
//!    symbols with no evidence into rewrite notes.
//!
//! An auditor that notices nothing therefore no longer passes a page by saying nothing
//! about its symbols.
//!
//! The auditor also checks the page's mathematics against the sources it explains, which it
//! reads: a page can misstate a result consistently, so that nothing on the page shows the
//! error. Those misstatements are reported as steps, and become rewrite notes like any other
//! step. The fidelity, source and error paragraphs of [`audit_instructions`] are the text the
//! calibration programme tested (`calibration/plans/A3.md`, arm R).

use std::collections::{HashMap, HashSet};
use std::path::Path;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::SourceSpec;
use crate::error::{Error, Result, read_json, write_json};
use crate::graph::{Candidate, Graph, TermIndex};
use crate::plan::Plan;
use crate::project::Project;
use crate::symbols::{self, Origin};
use crate::text::{contains_quote, normalise};
use crate::write::{link, names};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum GapKind {
    /// A named concept the reader does not know, which the page neither explains nor links.
    Term,
    /// A symbol nothing tells the reader the meaning of. Auditors report symbols under
    /// `symbols`; this kind records the ones with no evidence.
    #[schemars(skip)]
    Symbol,
    /// An inference, calculation or claim the reader cannot check from what the page gives.
    Step,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Gap {
    /// The term, or a short label for the step.
    pub term: String,
    /// The passage on the page where the gap occurs, copied exactly (a phrase or one sentence).
    pub quote: String,
    pub kind: GapKind,
    /// Why this reader cannot follow it.
    pub note: String,
}

/// Where the reader learns what a symbol denotes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "lowercase", deny_unknown_fields)]
pub enum Anchor {
    /// This page says what the symbol denotes. A use in an example ("take $r = 4$") does not.
    Here {
        /// The sentence that says it, copied exactly from the page.
        quote: String,
    },
    /// An earlier page binds it: one listed under "Symbols bound on earlier pages".
    Page {
        /// Id of that page.
        page: String,
    },
    /// The name of a known concept contains it and binds it, as listed under the known
    /// concepts.
    Concept {
        /// Id of that concept.
        concept: String,
    },
    /// Standard notation the reader profile covers, such as $n$ for an input size.
    Background {
        /// What in the reader profile covers it.
        note: String,
    },
    /// Nothing tells the reader what it denotes.
    Unbound {
        /// Where it is first used, copied exactly from the page.
        quote: String,
        /// What the reader cannot tell.
        note: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SymbolUse {
    /// The symbol in LaTeX, as it appears in the page's mathematics, without dollar signs.
    pub symbol: String,
    pub anchor: Anchor,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AuditAnswer {
    /// Every symbol in the page's mathematics, each once, with where the reader learns what
    /// it denotes. Empty only if the page has no mathematics.
    pub symbols: Vec<SymbolUse>,
    /// Every term or step the reader cannot follow; empty if the reader can follow them all.
    pub gaps: Vec<Gap>,
}

/// What code did with one reported gap.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Outcome {
    pub term: String,
    pub kind: GapKind,
    pub quote: String,
    pub note: String,
    pub action: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageAudit {
    pub page: String,
    /// Gaps that led to a graph edit, a candidate concept or a rewrite note.
    pub gaps: Vec<Outcome>,
    /// Gaps code discarded, with the reason as the action.
    pub dropped: Vec<Outcome>,
    /// The page's symbols and where the reader learns what each denotes, as accepted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub symbols: Vec<SymbolUse>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Round {
    pub round: u32,
    pub pages: Vec<PageAudit>,
}

/// How a rewrite note for a step gap begins.
pub const STEP_NOTE: &str = "The reader cannot follow this step:";

/// The sentences of `body` that are not in `previous`, normalised, as audits see both
/// (citation marks shown as "(cited)"). A sentence that rewords one of `previous`, sharing
/// at least three, and at least two thirds, of its words of four or more letters with that
/// sentence, is not added: a step reported on it is the step reported before, not one inside
/// new reasoning.
pub fn added(previous: &str, body: &str) -> Vec<String> {
    let sentences = |text: &str| -> Vec<String> {
        let text = crate::cite::referenced(text);
        text.split_inclusive(['.', '?', '!', '\n'])
            .map(normalise)
            .filter(|s| s.chars().any(char::is_alphanumeric))
            .collect()
    };
    let words = |sentence: &str| -> HashSet<String> {
        sentence
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.chars().count() >= 4)
            .map(str::to_lowercase)
            .collect()
    };
    let before = sentences(previous);
    let before_words: Vec<HashSet<String>> = before.iter().map(|s| words(s)).collect();
    sentences(body)
        .into_iter()
        .filter(|s| !before.contains(s))
        .filter(|s| {
            let ws = words(s);
            !before_words.iter().any(|b| {
                let shared = ws.intersection(b).count();
                shared >= 3 && 3 * shared >= 2 * ws.len()
            })
        })
        .collect()
}

/// The audit's instructions, which name the sources the auditor checks the page against. With
/// one source they are the text calibrated in A3 (`calibration/plans/A3.md`): against
/// planted errors, the audit then named 13 of 13 errors only the source shows and 25 of 28
/// that the page shows, where the audit that read the page alone named 0 and 14, at about
/// 0.5 false alarms per clean page (an independent blind review). With several sources, the
/// same paragraphs speak of the sources in the plural.
pub fn audit_instructions(sources: &[SourceSpec]) -> String {
    let paths: Vec<&Path> = sources.iter().map(|s| s.path.as_path()).collect();
    let words = Words::of(&paths);
    let mut text = AUDIT_INSTRUCTIONS.replacen(
        AUDIT_RESTRICTION,
        &format!(
            "Use only this task file and {} at {}: do not read other pages or any other file.",
            words.the_sources,
            listed(&paths, |p| format!("`{}`", p.display()))
        ),
        1,
    );
    let checks = format!(
        "{FIDELITY}{}{}",
        source_paragraph(&paths, &words),
        error_paragraph(&words)
    );
    text = text.replacen(QUOTES, &format!("{checks}{QUOTES}"), 1);
    text
}

/// How the instructions name the sources: "the paper" for one, as calibrated, and "the
/// sources" for several.
struct Words {
    the_sources: &'static str,
    sources_possessive: &'static str,
    a_source_possessive: &'static str,
}

impl Words {
    fn of(paths: &[&Path]) -> Self {
        if paths.len() == 1 {
            Words {
                the_sources: "the paper",
                sources_possessive: "the paper's",
                a_source_possessive: "the paper's",
            }
        } else {
            Words {
                the_sources: "the sources",
                sources_possessive: "the sources'",
                a_source_possessive: "a source's",
            }
        }
    }
}

/// `a`, `a and b`, `a, b and c`.
fn listed(paths: &[&Path], show: impl Fn(&Path) -> String) -> String {
    let items: Vec<String> = paths.iter().map(|p| show(p)).collect();
    match items.split_last() {
        None => String::new(),
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
    }
}

/// The format a source is written in, from its extension, as the auditor is told it.
fn format_of(path: &Path) -> Option<&'static str> {
    match path.extension()?.to_str()?.to_ascii_lowercase().as_str() {
        "tex" => Some("LaTeX"),
        "md" | "markdown" => Some("Markdown"),
        "txt" => Some("plain text"),
        _ => None,
    }
}

const AUDIT_RESTRICTION: &str =
    "Use only this task file: do not read the sources, other pages or any other file.";

const QUOTES: &str = "Copy each quote from the page exactly";

const FIDELITY: &str = "\
Fidelity: also check that the page's mathematics is right, and report what is wrong even where \
this reader could follow it. For each claim that states a definition, a hypothesis, a step of a \
proof, a value or a conclusion, work out what it assumes and what it concludes, and check it \
against everything this task file gives: the page's definitions, its other claims and its \
quotations. Report as a `step` any claim that is false, inconsistent with another statement on \
the page, stated more strongly or more widely than what supports it, or missing a hypothesis it \
needs. Quote the claim, and say in the note what is wrong and what the correct statement \
is.\n\n";

fn source_paragraph(paths: &[&Path], words: &Words) -> String {
    let at = listed(paths, |p| match format_of(p) {
        Some(f) => format!("`{}` ({f})", p.display()),
        None => format!("`{}`", p.display()),
    });
    let (head, files, line) = if paths.len() == 1 {
        (
            format!("The source: the paper this page explains is at {at}."),
            "the file",
            "the paper's line number",
        )
    } else {
        (
            format!("The sources: the documents this page explains are at {at}."),
            "the files",
            "the source's path and line number",
        )
    };
    format!(
        "{head} For each such claim, also find the passage of {the} that states, defines or \
         proves it (search {files} for the claim's terms and symbols, then read the passage \
         around each match) and compare: every hypothesis present, the same scope and \
         quantifiers, the same constants, signs, exponents and inequality directions, a \
         conclusion no stronger than {possessive}, and the right result, lemma or author \
         credited. Report any mismatch as a `step` in the same way, and give {line} in the \
         note.\n\n",
        the = words.the_sources,
        possessive = words.sources_possessive,
    )
}

fn error_paragraph(words: &Words) -> String {
    format!(
        "What counts as an error: report a claim as wrong only if, as this page states it, it \
         is false; inconsistent with the page; stated more strongly or more widely than what \
         supports it; missing a hypothesis without which it is false; given a wrong value, \
         sign, exponent, constant, quantifier or credit; or presented as following from \
         something it does not follow from. Do not report a correct statement as wrong because \
         it is informal, abbreviated or simplified, uses a word in {own} own sense, or leaves a \
         technical precondition to another page or to {the}. If {own} own summary of the same \
         step states it the same way, it is not an error.\n\n",
        own = words.a_source_possessive,
        the = words.the_sources,
    )
}

/// The audit's instructions without the source checks; [`audit_instructions`] adds them.
const AUDIT_INSTRUCTIONS: &str = "\
Audit one page of an explanatory wiki on behalf of a specific reader. Use only this task \
file: do not read the sources, other pages or any other file.

The reader knows the concepts listed as known, and everything the reader profile covers: \
what anyone with that background has been taught, whether or not the profile names it (a \
profile of undergraduate computer science covers logarithms and proof by induction without \
naming them). Anything outside that, the reader does not know: treat it as unknown to them, \
however elementary it seems to you. You are simulating that reader's lack of knowledge, so use \
your own knowledge only to recognise what the reader is missing.

Symbols: account for every symbol in the page's mathematics, each once, saying where the \
reader learns what it denotes:
- `here`: the page says what it denotes; quote the sentence that says it. A use in an \
example (\"take $r = 4$\") or a range (\"a power of two $r$\") does not say what $r$ denotes \
or counts. If the page uses the symbol before that sentence, submit records that the meaning \
comes too late.
- `page`: an earlier page binds it, as listed under \"Symbols bound on earlier pages\".
- `concept`: the name of a known concept contains it and binds it, as listed under that \
concept. A symbol in a known concept's name that is listed with no meaning is not known.
- `background`: standard notation the reader profile covers; say what covers it.
- `unbound`: nothing tells the reader what it denotes.
Submit checks that each symbol is in the page's mathematics, that each quote is on the page \
and contains the symbol, and that the named page or concept binds it.

Terms and steps: report what would stop this reader at this point on the page. A term or \
step is followable if the reader knows it, the page explains it, or the page links it to \
another wiki page. A claim marked \"(cited)\" is attributed to the source. A citation lets \
the reader check a result, not follow an argument: a cited result the page uses is followable \
if the page says what it states and why it applies where it is used, and its proof can be \
left to the source. A cited claim that stands in for how the page's own concept works, or \
whose content or role the page does not say, is a step.
- term: a named concept the reader does not know, which the page neither explains nor links. \
References to the documents themselves (the paper, the sources, the target result) are not \
terms.
- step: an inference, calculation or claim that the page presents as following from what it \
gives, which the reader cannot check from what the page gives. A claim about an identity, \
construction or definition that the page never states is a step.

Copy each quote from the page exactly, character for character, and keep it short: submit \
rejects quotes it cannot find on the page. Do not report anything the reader can follow. \
Answer with an empty list of gaps if the reader can follow the whole page.";

pub fn audit_task(project: &Project, graph: &Graph, plan: &Plan, i: usize, body: &str) -> String {
    let c = graph
        .get(&plan.order[i])
        .expect("planned pages are concepts");
    let pages: Vec<String> = plan
        .order
        .iter()
        .filter_map(|id| graph.get(id))
        .map(|p| format!("- {}", link(p)))
        .collect();
    let page = crate::cite::referenced(body);
    // Only the bindings whose symbol is on this page: the rest cannot help the auditor, and
    // leaving them out keeps this task unchanged when unrelated pages change.
    let page_math = symbols::math(&page);
    let bound: Vec<String> = symbols::in_force(graph, plan, i)
        .iter()
        .filter(|b| matches!(b.origin, Origin::Page(_)) && symbols::occurs(b.symbol, &page_math))
        .map(|b| {
            let on = &b.origin.concept().id;
            format!(
                "- ${}$ ({}), bound on `{on}`: {}",
                b.symbol,
                b.name,
                b.meaning.trim()
            )
        })
        .collect();
    let bound = if bound.is_empty() {
        "None of this page's symbols is bound on an earlier page.".to_string()
    } else {
        bound.join("\n")
    };
    format!(
        "## Reader profile\n\n{}\n\n## Known concepts\n\n{}\n\n## Symbols bound on earlier \
         pages\n\n{bound}\n\n## Pages in the wiki\n\n{}\n\n## Page to audit (`{}`)\n\n\
         ````markdown\n# {}\n\n{}\n````\n",
        crate::background::profile(project, graph),
        names(&plan.known_at(graph, i)),
        pages.join("\n"),
        c.id,
        c.title.as_deref().unwrap_or(&c.name),
        page.trim()
    )
}

/// Problems with the symbol accounting for page `plan.order[i]`.
fn check_symbols(
    graph: &Graph,
    plan: &Plan,
    i: usize,
    page: &str,
    uses: &mut [SymbolUse],
) -> Vec<String> {
    let mut errors = Vec::new();
    let page_math = symbols::math(page);
    let in_force = symbols::in_force(graph, plan, i);
    let on_page = |quote: &str, symbol: &str, errors: &mut Vec<String>| {
        errors.extend(symbols::quote_error(page, quote, symbol));
    };
    let mut seen: Vec<String> = Vec::new();
    for u in uses.iter_mut() {
        u.symbol = symbols::clean(&u.symbol);
        let symbol = u.symbol.as_str();
        if !symbols::occurs(symbol, &page_math) {
            errors.push(format!(
                "`{symbol}` does not occur in the page's mathematics; give it as written there"
            ));
            continue;
        }
        if seen.iter().any(|s| symbols::same(s, symbol)) {
            errors.push(format!("`{symbol}` is accounted for twice"));
        }
        seen.push(symbol.to_string());
        let binds = |origin: &dyn Fn(&Origin) -> bool| {
            in_force
                .iter()
                .any(|b| origin(&b.origin) && symbols::same(b.symbol, symbol))
        };
        match &u.anchor {
            Anchor::Here { quote } => on_page(quote, symbol, &mut errors),
            Anchor::Unbound { quote, .. } => on_page(quote, symbol, &mut errors),
            Anchor::Page { page } => {
                if !binds(&|o| matches!(o, Origin::Page(c) if c.id == *page)) {
                    errors.push(format!(
                        "`{symbol}`: `{page}` is not an earlier page that binds it (see \
                         \"Symbols bound on earlier pages\")"
                    ));
                }
            }
            Anchor::Concept { concept } => {
                if !binds(&|o| matches!(o, Origin::Name(c) if c.id == *concept)) {
                    errors.push(format!(
                        "`{symbol}`: `{concept}` is not a known concept whose name binds it"
                    ));
                }
            }
            Anchor::Background { note } => {
                if note.trim().is_empty() {
                    errors.push(format!(
                        "`{symbol}`: say what in the reader profile covers it"
                    ));
                }
            }
        }
    }
    let page_id = &plan.order[i];
    let missing: Vec<&str> = graph
        .bindings
        .get(page_id)
        .into_iter()
        .flatten()
        .map(|b| b.symbol.as_str())
        .filter(|b| !seen.iter().any(|s| symbols::same(s, b)))
        .collect();
    if !missing.is_empty() {
        errors.push(format!(
            "account for every symbol in the page's mathematics; missing: {}",
            missing.join(", ")
        ));
    }
    errors
}

/// Check an audit answer for page `plan.order[i]` and apply its gaps to the graph.
pub fn apply_audit(
    graph: &mut Graph,
    plan: &Plan,
    i: usize,
    body: &str,
    mut answer: AuditAnswer,
    levels: u32,
) -> Result<PageAudit> {
    let page = &plan.order[i];
    let stripped = crate::cite::referenced(body);
    let normalised = normalise(&stripped);
    let mut errors: Vec<String> = answer
        .gaps
        .iter()
        .filter(|g| !contains_quote(&normalised, &g.quote))
        .map(|g| format!("quote not found on the page: \"{}\"", g.quote))
        .collect();
    if answer.gaps.iter().any(|g| g.kind == GapKind::Symbol) {
        errors.push("report symbols under `symbols`, not as gaps".into());
    }
    errors.extend(check_symbols(
        graph,
        plan,
        i,
        &stripped,
        &mut answer.symbols,
    ));
    if !errors.is_empty() {
        return Err(Error::Invalid(format!(
            "answer rejected:\n{}\nCopy quotes exactly from the page, and use only the \
             bindings listed in the task.",
            errors
                .iter()
                .map(|e| format!("- {e}"))
                .collect::<Vec<_>>()
                .join("\n")
        )));
    }
    let known = TermIndex::new(plan.known_at(graph, i));
    let mut audit = apply_gaps(graph, page, &known, answer.gaps, levels);
    for u in &answer.symbols {
        if let Anchor::Here { quote } = &u.anchor
            && symbols::quoted(&stripped, quote, &u.symbol) == symbols::Quoted::Late
        {
            // The page says what the symbol denotes, but only after using it.
            graph.queue_rewrite(
                page,
                Some(format!(
                    "Say what ${}$ denotes where it is first used: the page uses it before \
                     saying so in \"{quote}\"",
                    u.symbol
                )),
            );
            audit.gaps.push(Outcome {
                term: u.symbol.clone(),
                kind: GapKind::Symbol,
                quote: quote.clone(),
                note: "used before the page says what it denotes".into(),
                action: "rewrite: say what the symbol denotes at its first use".into(),
            });
        }
        if let Anchor::Unbound { quote, note } = &u.anchor {
            graph.queue_rewrite(
                page,
                Some(format!(
                    "Say what ${}$ denotes where it is first used (for a size or count, what it \
                     counts): \"{quote}\" ({note})",
                    u.symbol
                )),
            );
            audit.gaps.push(Outcome {
                term: u.symbol.clone(),
                kind: GapKind::Symbol,
                quote: quote.clone(),
                note: note.clone(),
                action: "rewrite: say what the symbol denotes".into(),
            });
        }
    }
    audit.symbols = answer.symbols;
    Ok(audit)
}

/// The action recorded for a step reported inside reasoning a rewrite added for an earlier
/// step: it is sent back to be cited or cut, not explained further.
/// How many levels a step is explained to on a page that presents no step of the argument:
/// a step inside reasoning added for an earlier step is cited or cut.
pub const LEVELS: u32 = 1;

/// How many levels a step is explained to on a page that presents a step of the argument:
/// the page owes how that step's method works, so reasoning added for a step may itself be
/// explained once more.
pub const ARGUMENT_LEVELS: u32 = 2;

/// The levels page `page` explains steps to: [`ARGUMENT_LEVELS`] if the argument assigns it
/// a step, else [`LEVELS`].
pub fn levels(argument: Option<&crate::argument::Argument>, page: &str) -> u32 {
    if argument.is_some_and(|a| a.module(page).is_some()) {
        ARGUMENT_LEVELS
    } else {
        LEVELS
    }
}

/// How a rewrite note for a step inside added reasoning begins.
pub const CITE_NOTE: &str = "Cite or cut this step:";

pub const CITE_OR_CUT: &str = "rewrite: cite or cut a step inside added reasoning";

fn apply_gaps(
    graph: &mut Graph,
    page: &str,
    known: &TermIndex,
    gaps: Vec<Gap>,
    levels: u32,
) -> PageAudit {
    let mut audit = PageAudit {
        page: page.to_string(),
        gaps: Vec::new(),
        dropped: Vec::new(),
        symbols: Vec::new(),
    };
    for gap in gaps {
        let outcome = |action: String| Outcome {
            term: gap.term.clone(),
            kind: gap.kind,
            quote: gap.quote.clone(),
            note: gap.note.clone(),
            action,
        };
        if gap.kind != GapKind::Step && known.lookup(&gap.term).is_some() {
            audit
                .dropped
                .push(outcome("dropped: the reader already knows this".into()));
            continue;
        }
        // A step's depth is that of the added sentence it is in, 0 outside added reasoning.
        // Steps are explained to depth `levels - 1`; deeper, explaining would add claims of
        // its own, and a citation or a cut adds none. A step on a claim that already carries
        // a citation mark cannot be met by citing it, so at the limit it is explained once
        // more; beyond, it is cut.
        let added = graph.added.get(page).and_then(|added| {
            added
                .iter()
                .find(|s| contains_quote(&s.sentence, &gap.quote))
        });
        let depth = added.map_or(0, |s| s.depth);
        let cited = added.is_some_and(|s| s.sentence.contains("(cited)"));
        if gap.kind == GapKind::Step {
            let deepest = graph.note_depth.entry(page.to_string()).or_default();
            *deepest = (*deepest).max(depth);
        }
        if gap.kind == GapKind::Step && (depth > levels || (depth == levels && !cited)) {
            graph.queue_rewrite(
                page,
                Some(format!(
                    "{CITE_NOTE} \"{}\" ({}). It is inside reasoning added for an earlier \
                     step, so do not explain it further: cite the source for it, or remove the \
                     claim.",
                    gap.quote, gap.note
                )),
            );
            audit.gaps.push(outcome(CITE_OR_CUT.into()));
            continue;
        }
        let action = match gap.kind {
            GapKind::Step => {
                // Below the limit on a page presenting a step of the argument, only how that
                // step's method works is developed further; a result it uses is stated.
                let owed = if depth > 0 && levels == ARGUMENT_LEVELS {
                    " It is inside reasoning added for an earlier step. If it is part of how \
                     this page's own step of the argument works (see \"This page in the \
                     argument\"), develop it here; if it concerns a result that step uses, \
                     say what the result states and cite it."
                } else {
                    ""
                };
                graph.queue_rewrite(
                    page,
                    Some(format!(
                        "{STEP_NOTE} \"{}\" ({}). Give the reasoning on this \
                         page. If the step rests on a result from prior work, say what the \
                         result states and why it applies here, and cite the source for its \
                         proof: a citation alone does not explain a step.{owed}",
                        gap.quote, gap.note
                    )),
                );
                "rewrite: explain the step".to_string()
            }
            GapKind::Term => term_action(graph, page, &gap),
            GapKind::Symbol => unreachable!("symbol gaps are rejected before they are applied"),
        };
        audit.gaps.push(outcome(action));
    }
    audit
}

fn term_action(graph: &mut Graph, page: &str, gap: &Gap) -> String {
    let index = TermIndex::new(&graph.concepts);
    let Some(target) = index.lookup(&gap.term).map(str::to_string) else {
        graph.candidates.push(Candidate {
            term: gap.term.clone(),
            page: page.to_string(),
            quote: gap.quote.clone(),
        });
        return "candidate concept".to_string();
    };
    let concept = graph.get(&target).expect("indexed concepts exist").clone();
    if concept.id == page {
        graph.queue_rewrite(
            page,
            Some(format!(
                "The reader cannot follow the page's own subject as used here: \"{}\" ({})",
                gap.quote, gap.note
            )),
        );
        return "rewrite: explain the page's own concept".to_string();
    }
    let already = graph
        .get(page)
        .is_some_and(|p| p.requires.contains(&concept.id));
    if already {
        graph.queue_rewrite(
            page,
            Some(format!(
                "Link {} where \"{}\" is used: \"{}\"",
                link(&concept),
                gap.term,
                gap.quote
            )),
        );
        return format!("rewrite: link prerequisite `{}`", concept.id);
    }
    if graph.add_requires(page, &concept.id) {
        graph.queue_rewrite(
            page,
            Some(format!(
                "{} is now a prerequisite: link it where \"{}\" is used: \"{}\"",
                link(&concept),
                gap.term,
                gap.quote
            )),
        );
        return format!("added prerequisite `{}`", concept.id);
    }
    graph.queue_rewrite(
        page,
        Some(format!(
            "Explain \"{}\" inline where it is used: \"{}\" (its concept depends on this page, \
             so it cannot be a prerequisite)",
            gap.term, gap.quote
        )),
    );
    "rewrite: explain inline (an edge would create a cycle)".to_string()
}

/// Add a page's audit to its round in `audit.json`.
pub fn record(project: &Project, round: u32, page: PageAudit) -> Result<()> {
    let path = project.audit_path();
    let mut rounds: Vec<Round> = if path.exists() {
        read_json(&path)?
    } else {
        Vec::new()
    };
    match rounds.iter_mut().find(|r| r.round == round) {
        Some(r) => {
            r.pages.retain(|p| p.page != page.page);
            r.pages.push(page);
        }
        None => rounds.push(Round {
            round,
            pages: vec![page],
        }),
    }
    write_json(&path, &rounds)
}

/// The symbols later pages take their meaning from `page`, by their latest audits: each
/// symbol with the page that uses it.
pub fn relied_on(latest: &HashMap<String, Vec<SymbolUse>>, page: &str) -> Vec<(String, String)> {
    let mut relied: Vec<(String, String)> = latest
        .iter()
        .flat_map(|(on, uses)| {
            uses.iter()
                .filter(|u| matches!(&u.anchor, Anchor::Page { page: p } if p == page))
                .map(|u| (u.symbol.clone(), on.clone()))
        })
        .collect();
    relied.sort();
    relied
}

/// Each page's symbol accounting from its latest audit.
pub fn latest_symbols(project: &Project) -> Result<HashMap<String, Vec<SymbolUse>>> {
    let mut latest = HashMap::new();
    for round in load_rounds(project)? {
        for page in round.pages {
            latest.insert(page.page, page.symbols);
        }
    }
    Ok(latest)
}

pub fn load_rounds(project: &Project) -> Result<Vec<Round>> {
    let path = project.audit_path();
    if path.exists() {
        read_json(&path)
    } else {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::tests::{concept, graph};
    use crate::symbols::{Binding, PageBinding, Relation};

    fn gap(term: &str, quote: &str, kind: GapKind) -> Gap {
        Gap {
            term: term.into(),
            quote: quote.into(),
            kind,
            note: "n".into(),
        }
    }

    fn uses(symbol: &str, anchor: Anchor) -> SymbolUse {
        SymbolUse {
            symbol: symbol.into(),
            anchor,
        }
    }

    #[test]
    fn gaps_become_edges_candidates_notes_or_drops() {
        let mut g = graph(
            "page",
            vec![
                concept("page", &[], false),
                concept("dft", &[], false),
                concept("later", &["page"], false),
                concept("big-o", &[], true),
            ],
        );
        let plan = Plan {
            order: vec!["page".into()],
        };
        let body = "Uses the DFT, big O, a later idea, dyadic numbers and $\\kappa$.";
        let kappa = || {
            vec![uses(
                "\\kappa",
                Anchor::Unbound {
                    quote: "dyadic numbers and $\\kappa$".into(),
                    note: "never said".into(),
                },
            )]
        };
        let answer = |gaps| AuditAnswer {
            symbols: kappa(),
            gaps,
        };
        let bad = apply_audit(
            &mut g.clone(),
            &plan,
            0,
            body,
            answer(vec![
                gap("x", "not on the page", GapKind::Step),
                gap("κ", "$\\kappa$", GapKind::Symbol),
            ]),
            LEVELS,
        );
        let err = bad.unwrap_err().to_string();
        assert!(
            err.contains("not on the page") && err.contains("under `symbols`"),
            "{err}"
        );
        let audit = apply_audit(
            &mut g,
            &plan,
            0,
            body,
            answer(vec![
                gap("DFT", "Uses the DFT", GapKind::Term),
                gap("big O", "big O", GapKind::Term),
                gap("later", "a later idea", GapKind::Term),
                gap("dyadic", "dyadic numbers", GapKind::Term),
            ]),
            LEVELS,
        )
        .unwrap();
        let actions: Vec<&str> = audit.gaps.iter().map(|o| o.action.as_str()).collect();
        assert_eq!(
            actions,
            vec![
                "added prerequisite `dft`",
                "rewrite: explain inline (an edge would create a cycle)",
                "candidate concept",
                "rewrite: say what the symbol denotes",
            ]
        );
        assert_eq!(audit.dropped.len(), 1);
        assert_eq!(audit.symbols.len(), 1);
        assert_eq!(g.get("page").unwrap().requires, vec!["dft"]);
        assert_eq!(g.candidates[0].term, "dyadic");
        assert_eq!(g.rewrite["page"].len(), 3);
        assert!(g.rewrite["page"][2].starts_with("Say what $\\kappa$ denotes"));
        assert!(g.check().is_empty());
    }

    /// The demo's failure: `r` arrived inside a known concept's name, which bound nothing.
    #[test]
    fn every_symbol_needs_evidence_that_holds() {
        let mut ring = concept("ring", &[], false);
        ring.title = Some("Ring modulo $y^r + 1$".into());
        let mut g = graph(
            "synthetic",
            vec![
                ring,
                concept("dft", &[], false),
                concept("synthetic", &["ring", "dft"], false),
            ],
        );
        let stated = |symbol: &str, quote: &str| PageBinding {
            symbol: symbol.into(),
            name: "root".into(),
            meaning: "A root of unity.".into(),
            relation: Relation::Direct,
            quote: quote.into(),
        };
        g.bindings.insert(
            "dft".into(),
            vec![stated("\\omega", "where $\\omega$ is a root")],
        );
        g.bindings.insert(
            "synthetic".into(),
            vec![stated("T", "Let $T$ be the length.")],
        );
        let plan = Plan {
            order: vec!["ring".into(), "dft".into(), "synthetic".into()],
        };
        let body = "Work modulo $y^r + 1$ with $\\omega$ and $n$. Let $T$ be the length. \
                    Take $r = 4$.";
        let answer = |symbols| AuditAnswer {
            symbols,
            gaps: vec![],
        };
        let concept_r = || {
            uses(
                "r",
                Anchor::Concept {
                    concept: "ring".into(),
                },
            )
        };
        let rest = || {
            vec![
                uses("\\omega", Anchor::Page { page: "dft".into() }),
                uses(
                    "n",
                    Anchor::Background {
                        note: "input size".into(),
                    },
                ),
            ]
        };
        let mut wrong = vec![
            concept_r(),
            uses("q", Anchor::Background { note: "x".into() }),
        ];
        wrong.push(uses(
            "T",
            Anchor::Here {
                quote: "Take $r = 4$.".into(),
            },
        ));
        wrong.push(uses(
            "\\omega",
            Anchor::Page {
                page: "ring".into(),
            },
        ));
        let err = apply_audit(&mut g.clone(), &plan, 2, body, answer(wrong), LEVELS)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("`ring` is not a known concept whose name binds it"),
            "{err}"
        );
        assert!(err.contains("`q` does not occur"), "{err}");
        assert!(
            err.contains("`T`: the quote must cover the symbol"),
            "{err}"
        );
        assert!(
            err.contains("`ring` is not an earlier page that binds it"),
            "{err}"
        );
        let err = apply_audit(&mut g.clone(), &plan, 2, body, answer(rest()), LEVELS)
            .unwrap_err()
            .to_string();
        assert!(err.contains("missing: T"), "{err}");

        // Once the name binds r, the concept is evidence for it.
        g.get_mut("ring").unwrap().symbols = vec![Binding {
            symbol: "r".into(),
            name: "record length".into(),
            meaning: "The number of coefficients in a record.".into(),
            relation: Relation::Count,
        }];
        let mut good = rest();
        good.push(concept_r());
        good.push(uses(
            "T",
            Anchor::Here {
                quote: "Let $T$ be the length.".into(),
            },
        ));
        let audit = apply_audit(&mut g, &plan, 2, body, answer(good), LEVELS).unwrap();
        assert!(audit.gaps.is_empty() && audit.symbols.len() == 4);
        assert!(!g.rewrite.contains_key("synthetic"));
    }

    #[test]
    fn a_meaning_given_after_first_use_is_a_gap() {
        let mut g = graph("page", vec![concept("page", &[], false)]);
        let plan = Plan {
            order: vec!["page".into()],
        };
        let body = "Take $r = 4$. Here $r$ is the record length.";
        let answer = AuditAnswer {
            symbols: vec![uses(
                "r",
                Anchor::Here {
                    quote: "Here $r$ is the record length.".into(),
                },
            )],
            gaps: vec![],
        };
        let audit = apply_audit(&mut g, &plan, 0, body, answer, LEVELS).unwrap();
        assert_eq!(
            audit.gaps[0].action,
            "rewrite: say what the symbol denotes at its first use"
        );
        assert!(g.rewrite["page"][0].contains("uses it before saying so"));
    }

    #[test]
    fn added_sentences_are_those_not_in_the_previous_version() {
        let previous = "It is even, so we halve it. Then we stop.";
        let body = "It is even, so we halve it. One of $n$ and $n-1$ is even. Then we stop.";
        assert_eq!(added(previous, body), vec!["One of $n$ and $n-1$ is even."]);
        // The full run's `gaussian-resampling`: the rewrite split the sentence a step was
        // reported on, and added a sentence declining to give the identity.
        let previous = "Gaussian resampling is the step that makes the transforms of the \
                        multiplier usable: it expresses each source transform through a \
                        power-of-two tensor transform, meaning a transform over several axes \
                        at once in which each axis length is a power of two.";
        let body = "Gaussian resampling is a reduction that replaces a transform of arbitrary \
                    length by transforms whose axis lengths are powers of two. Gaussian \
                    resampling expresses each source transform through a power-of-two tensor \
                    transform, meaning a transform over several axes at once in which each \
                    axis length is a power of two. The construction is the one of Harvey and \
                    van der Hoeven; this page states its effect and does not re-derive the \
                    identity behind it.";
        let added = added(previous, body);
        assert!(
            added.iter().all(|s| !s.contains("expresses each source")),
            "{added:?}"
        );
        assert!(
            added
                .last()
                .unwrap()
                .starts_with("The construction is the one")
        );
        // A claim added inside a reworded sentence leaves it a rewording: a step reported on
        // it gets a step note, not cite or cut, so the claim is still explained.
        let previous = "In general the per-volume cost grows like $f^a$, with exponent below $1$.";
        let body = "In general the per-volume cost grows like $f^a$ as the parameter grows, and \
                    the exponent is below $1$ because $s<m$.";
        assert!(super::added(previous, body).is_empty());
    }

    #[test]
    fn symbols_later_pages_take_from_a_page() {
        let uses = |anchors: Vec<(&str, Anchor)>| {
            anchors
                .into_iter()
                .map(|(s, anchor)| SymbolUse {
                    symbol: s.into(),
                    anchor,
                })
                .collect::<Vec<_>>()
        };
        let from = |page: &str| Anchor::Page { page: page.into() };
        let latest = HashMap::from([
            (
                "butterfly".to_string(),
                uses(vec![
                    ("b", from("kernel")),
                    ("a", from("kernel")),
                    ("V", from("tape")),
                ]),
            ),
            ("tape".to_string(), uses(vec![("a", from("dyadics"))])),
        ]);
        assert_eq!(
            relied_on(&latest, "kernel"),
            vec![
                ("a".to_string(), "butterfly".to_string()),
                ("b".to_string(), "butterfly".to_string())
            ]
        );
        assert!(relied_on(&latest, "butterfly").is_empty());
    }

    #[test]
    fn steps_are_explained_to_the_page_s_levels_then_cited_or_cut() {
        // Sentence n of the page is added reasoning at depth n (1 to 3), uncited and cited;
        // "Plain" is not added (depth 0).
        let added = |depth: u32, cited: bool| crate::graph::Added {
            sentence: format!(
                "Depth {depth} {} claim holds{}.",
                if cited { "cited" } else { "plain" },
                if cited { " (cited)" } else { "" }
            ),
            depth,
        };
        let step = |quote: &str| Gap {
            term: "step".into(),
            quote: quote.into(),
            kind: GapKind::Step,
            note: "why".into(),
        };
        let quotes = [
            ("Plain text", 0, false),
            ("Depth 1 plain claim", 1, false),
            ("Depth 1 cited claim", 1, true),
            ("Depth 2 plain claim", 2, false),
            ("Depth 2 cited claim", 2, true),
            ("Depth 3 plain claim", 3, false),
            ("Depth 3 cited claim", 3, true),
        ];
        // What each step gets: E for a step note, C to be cited or cut.
        for (levels, expected) in [(LEVELS, "ECECCCC"), (ARGUMENT_LEVELS, "EEECECC")] {
            let mut g = graph("p", vec![concept("p", &[], false)]);
            g.added.insert(
                "p".into(),
                quotes
                    .iter()
                    .filter(|(_, depth, _)| *depth > 0)
                    .map(|&(_, depth, cited)| added(depth, cited))
                    .collect(),
            );
            let audit = apply_gaps(
                &mut g,
                "p",
                &TermIndex::default(),
                quotes.iter().map(|(q, _, _)| step(q)).collect(),
                levels,
            );
            let got: String = audit
                .gaps
                .iter()
                .map(|o| if o.action == CITE_OR_CUT { 'C' } else { 'E' })
                .collect();
            assert_eq!(got, expected, "levels {levels}");
            let notes = &g.rewrite["p"];
            assert!(notes[1].starts_with(if levels == LEVELS {
                CITE_NOTE
            } else {
                STEP_NOTE
            }));
            // On a page presenting a step of the argument, a step below the limit inside
            // added reasoning is developed only if it is part of the page's own step.
            assert_eq!(
                notes[1].contains("part of how this page's own step"),
                levels == ARGUMENT_LEVELS
            );
            assert!(!notes[0].contains("part of how this page's own step"));
            // The rewrite answering these notes adds sentences one below the deepest step.
            assert_eq!(g.note_depth["p"], 3);
        }
    }

    fn source(path: &str) -> SourceSpec {
        SourceSpec {
            id: path.into(),
            path: path.into(),
            title: None,
            url: None,
        }
    }

    /// The text calibrated in A3 (`calibration/plans/A3.md`; `calibration/tools/a2setup.py`
    /// and `a3setup.py`): changing it means the audit is no longer the configuration the
    /// calibration report recommends.
    #[test]
    fn one_source_gives_the_calibrated_audit() {
        let text = audit_instructions(&[source("sources/paper.tex")]);
        let restriction = "Use only this task file and the paper at `sources/paper.tex`: do not \
                           read other pages or any other file.";
        let checks = "Fidelity: also check that the page's mathematics is right, and report \
what is wrong even where this reader could follow it. For each claim that states a definition, a \
hypothesis, a step of a proof, a value or a conclusion, work out what it assumes and what it \
concludes, and check it against everything this task file gives: the page's definitions, its \
other claims and its quotations. Report as a `step` any claim that is false, inconsistent with \
another statement on the page, stated more strongly or more widely than what supports it, or \
missing a hypothesis it needs. Quote the claim, and say in the note what is wrong and what the \
correct statement is.

The source: the paper this page explains is at `sources/paper.tex` (LaTeX). For each such claim, \
also find the passage of the paper that states, defines or proves it (search the file for the \
claim's terms and symbols, then read the passage around each match) and compare: every \
hypothesis present, the same scope and quantifiers, the same constants, signs, exponents and \
inequality directions, a conclusion no stronger than the paper's, and the right result, lemma or \
author credited. Report any mismatch as a `step` in the same way, and give the paper's line \
number in the note.

What counts as an error: report a claim as wrong only if, as this page states it, it is false; \
inconsistent with the page; stated more strongly or more widely than what supports it; missing a \
hypothesis without which it is false; given a wrong value, sign, exponent, constant, quantifier \
or credit; or presented as following from something it does not follow from. Do not report a \
correct statement as wrong because it is informal, abbreviated or simplified, uses a word in the \
paper's own sense, or leaves a technical precondition to another page or to the paper. If the \
paper's own summary of the same step states it the same way, it is not an error.

";
        let expected = AUDIT_INSTRUCTIONS
            .replacen(AUDIT_RESTRICTION, restriction, 1)
            .replacen(QUOTES, &format!("{checks}{QUOTES}"), 1);
        assert_eq!(text, expected);
        assert!(!text.contains("do not read the sources"));
        assert_eq!(text.matches(QUOTES).count(), 1);
    }

    #[test]
    fn several_sources_are_named_in_the_plural() {
        let text = audit_instructions(&[
            source("sources/paper.tex"),
            source("sources/notes.md"),
            source("sources/errata"),
        ]);
        assert!(text.contains(
            "Use only this task file and the sources at `sources/paper.tex`, `sources/notes.md` \
             and `sources/errata`: do not read other pages or any other file."
        ));
        assert!(text.contains(
            "The sources: the documents this page explains are at `sources/paper.tex` (LaTeX), \
             `sources/notes.md` (Markdown) and `sources/errata`."
        ));
        assert!(text.contains("give the source's path and line number in the note"));
        assert!(text.contains("uses a word in a source's own sense"));
        for singular in [
            "the paper's",
            "the paper at",
            "the paper this page",
            "the passage of the paper",
        ] {
            assert!(!text.contains(singular), "{singular}");
        }
        let fidelity = text.find("Fidelity:").unwrap();
        let what_counts = text.find("What counts as an error").unwrap();
        assert!(fidelity < what_counts && what_counts < text.find(QUOTES).unwrap());
    }
}

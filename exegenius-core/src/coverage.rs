//! Where a planning answer's quotes fall in the sources: a diagnostic recorded for every
//! extract and argument answer, and, with `[checks] coverage = "reject"`, a reason to send
//! an answer back ([`Coverage::shortfall`], [`rejection`]).
//!
//! The extract and argument tasks quote the sources as evidence. Submit checks that each
//! quote is in its source, and nothing else about where the quotes come from, so a plan
//! drawn from the introduction alone passes (2026-10-13-planner-handoff-run). This module
//! measures, for each source:
//!
//! - how far into the source the quotes reach, in lines, and how many fall in its first
//!   tenth;
//! - which sections hold a quote, and which theorem-like statements (lemmas, theorems,
//!   definitions) lie in a section that holds none;
//! - which concepts or pages share their name with a section heading but quote none of the
//!   sections so named.
//!
//! What it cannot establish: that the plan covers the proof. A quote shows that a passage
//! was read and cited, not that the step it supports is in the plan, and a plan can quote
//! every section and still miss a derivation. A section with no quote may not matter to the
//! target, and a result argued in prose, outside any theorem-like environment, counts in no
//! statement figure. A short quote that occurs more than once is placed at its first occurrence,
//! which can be the introduction's summary rather than the passage the agent read; such
//! quotes are counted as `repeated`. Heading matches compare words without stemming
//! ("butterfly" does not match "butterflies"). Judging coverage of the proof needs a
//! reference of what the plan must contain, which this module does not have.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::argument::Node;
use crate::graph::{Concept, Evidence, Graph};
use crate::project::{Heading, Source};
use crate::text::key;

/// A part of a planning answer that quotes the sources: a concept, or a step of the
/// argument.
#[derive(Debug, Clone)]
pub struct Owner<'a> {
    /// The concept's id, or the step's id.
    pub id: String,
    /// Names to look for in section headings: the concept's name and aliases, or the name
    /// and aliases of the page that presents the step.
    pub terms: Vec<String>,
    pub quotes: Vec<&'a Evidence>,
}

/// Where the quotes of one planning answer fall in one source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coverage {
    pub source: String,
    /// Lines in the source, blank ones included.
    pub lines: usize,
    /// Quotes naming this source.
    pub quotes: usize,
    /// Quotes found in the source word for word.
    pub located: usize,
    /// Located quotes that occur more than once in the source: each is placed at its first
    /// occurrence.
    pub repeated: usize,
    /// The first and last lines a located quote starts on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<(usize, usize)>,
    /// Located quotes starting in the first tenth of the source's lines.
    pub first_tenth: usize,
    /// Whether the source's sections are marked up (LaTeX or markdown). For a source
    /// whose sections are not, `sections`, `unquoted_statements`, `statements` and
    /// `elsewhere` are left empty: headings guessed from numbered lines are unreliable.
    pub structured: bool,
    /// Every section, with the text before the first heading as "(front matter)".
    pub sections: Vec<SectionCoverage>,
    /// Theorem-like statements in a section that holds no quote, as "Lemma `lem:x` (line
    /// 1234)".
    pub unquoted_statements: Vec<String>,
    /// Theorem-like statements in the source.
    pub statements: usize,
    /// Concepts or pages named in a section heading whose quotes lie in none of the sections
    /// so named.
    pub elsewhere: Vec<Elsewhere>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SectionCoverage {
    pub title: String,
    /// 1 for a section, 2 for a subsection; the front matter counts as a section.
    pub level: usize,
    /// First and last line of the section, its subsections included.
    pub lines: (usize, usize),
    /// Located quotes in the section, its subsections included.
    pub quotes: usize,
    /// Theorem-like statements in the section, its subsections included.
    pub statements: usize,
    /// Displayed equations in the section, its subsections included.
    #[serde(default)]
    pub displays: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Elsewhere {
    /// The concept's or step's id.
    pub id: String,
    /// The name found in the headings.
    pub term: String,
    /// The sections whose headings name it.
    pub headings: Vec<String>,
    /// The sections its quotes lie in.
    pub quoted_in: Vec<String>,
}

/// A section's extent in the normalised text.
struct Span {
    title: String,
    level: usize,
    start: usize,
    end: usize,
}

/// Each heading's section runs to the next heading at the same level or above; the text
/// before the first heading is the front matter, at the top level.
fn spans(source: &Source) -> Vec<Span> {
    let headings = source.sections();
    let total = source.normalised.len();
    let top = headings.iter().map(|h| h.level).min().unwrap_or(1);
    let mut spans = Vec::new();
    let first = headings.first().map_or(total, |h| h.offset);
    if first > 0 {
        spans.push(Span {
            title: "(front matter)".into(),
            level: top,
            start: 0,
            end: first,
        });
    }
    for (i, h) in headings.iter().enumerate() {
        let end = headings[i + 1..]
            .iter()
            .find(|n| n.level <= h.level)
            .map_or(total, |n: &Heading| n.offset);
        spans.push(Span {
            title: h.title.clone(),
            level: h.level,
            start: h.offset,
            end,
        });
    }
    spans
}

/// The words of a name or heading, lowercased, for matching a name inside a heading.
fn words(s: &str) -> Vec<String> {
    key(s)
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_string)
        .collect()
}

fn names(heading: &[String], term: &[String]) -> bool {
    !term.is_empty() && heading.windows(term.len()).any(|w| w == term)
}

/// Where the owners' quotes fall in each source. Quotes naming no source are left out.
pub fn coverage(sources: &[Source], owners: &[Owner]) -> Vec<Coverage> {
    sources
        .iter()
        .map(|source| of_source(source, owners))
        .collect()
}

fn of_source(source: &Source, owners: &[Owner]) -> Coverage {
    let structured = source.marked_up();
    let spans = if structured {
        spans(source)
    } else {
        Vec::new()
    };
    let innermost = |at: usize| {
        spans
            .iter()
            .filter(|s| s.start <= at && at < s.end.max(s.start + 1))
            .max_by_key(|s| s.level)
    };
    let tenth = source.line_count().div_ceil(10);
    let mut quotes = 0;
    let mut repeated = 0;
    let mut located = Vec::new();
    // For each owner, the offsets of its located quotes.
    let mut placed: Vec<Vec<usize>> = Vec::new();
    for owner in owners {
        let mut at = Vec::new();
        for q in owner.quotes.iter().filter(|q| q.source == source.id) {
            quotes += 1;
            let found = source.occurrences(&q.quote);
            if found.len() > 1 {
                repeated += 1;
            }
            if let Some(&first) = found.first() {
                located.push(first);
                at.push(first);
            }
        }
        placed.push(at);
    }
    let lines: Vec<usize> = located.iter().map(|&o| source.line_at(o)).collect();
    let span = lines
        .iter()
        .min()
        .zip(lines.iter().max())
        .map(|(&a, &b)| (a, b));
    let first_tenth = lines.iter().filter(|&&l| l <= tenth).count();
    let last_line = |end: usize| {
        if end >= source.normalised.len() {
            source.line_count()
        } else {
            source.line_at(end).saturating_sub(1).max(1)
        }
    };
    let statements = if structured { source.statements() } else { &[] };
    let sections: Vec<SectionCoverage> = spans
        .iter()
        .map(|s| SectionCoverage {
            title: s.title.clone(),
            level: s.level,
            lines: (source.line_at(s.start), last_line(s.end)),
            quotes: located
                .iter()
                .filter(|&&o| s.start <= o && o < s.end)
                .count(),
            statements: statements
                .iter()
                .filter(|t| s.start <= t.offset && t.offset < s.end)
                .count(),
            displays: source
                .displays()
                .iter()
                .filter(|&&d| s.start <= d && d < s.end)
                .count(),
        })
        .collect();
    let quoted: BTreeSet<&str> = located
        .iter()
        .filter_map(|&o| innermost(o))
        .map(|s| s.title.as_str())
        .collect();
    let unquoted_statements = statements
        .iter()
        .filter(|t| innermost(t.offset).is_none_or(|s| !quoted.contains(s.title.as_str())))
        .map(|t| match &t.label {
            Some(label) if label.starts_with(&t.kind) => format!("{label} (line {})", t.line),
            Some(label) => format!("{} `{label}` (line {})", t.kind, t.line),
            None => format!("{} (line {})", t.kind, t.line),
        })
        .collect();
    let mut elsewhere = Vec::new();
    for (owner, at) in owners.iter().zip(&placed) {
        if at.is_empty() {
            continue;
        }
        for term in &owner.terms {
            let term_words = words(term);
            if term_words.concat().chars().count() < 4 {
                continue;
            }
            let named: Vec<&Span> = spans
                .iter()
                .filter(|s| s.title != "(front matter)" && names(&words(&s.title), &term_words))
                .collect();
            if named.is_empty() {
                continue;
            }
            let inside = at
                .iter()
                .any(|&o| named.iter().any(|s| s.start <= o && o < s.end));
            if !inside {
                let mut in_order = at.clone();
                in_order.sort_unstable();
                let mut quoted_in: Vec<String> = in_order
                    .into_iter()
                    .filter_map(innermost)
                    .map(|s| s.title.clone())
                    .collect();
                quoted_in.dedup();
                elsewhere.push(Elsewhere {
                    id: owner.id.clone(),
                    term: term.clone(),
                    headings: named.iter().map(|s| s.title.clone()).collect(),
                    quoted_in,
                });
            }
            // One heading match per owner is enough to report.
            break;
        }
    }
    Coverage {
        source: source.id.clone(),
        lines: source.line_count(),
        quotes,
        located: located.len(),
        repeated,
        span,
        first_tenth,
        sections,
        unquoted_statements,
        structured,
        statements: statements.len(),
        elsewhere,
    }
}

/// The owners in a set of concepts: each concept's evidence, named by its name and aliases.
pub fn concept_owners(concepts: &[Concept]) -> Vec<Owner<'_>> {
    concepts
        .iter()
        .map(|c| Owner {
            id: c.id.clone(),
            terms: std::iter::once(&c.name)
                .chain(&c.aliases)
                .cloned()
                .collect(),
            quotes: c.evidence.iter().collect(),
        })
        .collect()
}

/// The owners in an argument: the root, and each other step a page presents, with the
/// quotes of the steps it presents itself (infer steps' quotes and explicit motivations'
/// quotes, down to steps another page presents), named by its page's name and aliases in
/// `graph`. Each quote belongs to one owner.
pub fn argument_owners<'a>(root: &'a Node, graph: Option<&Graph>) -> Vec<Owner<'a>> {
    let owner = |node: &Node| Owner {
        id: node.id.clone(),
        terms: node
            .page
            .as_deref()
            .and_then(|page| graph?.get(page))
            .map(|c| {
                std::iter::once(&c.name)
                    .chain(&c.aliases)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default(),
        quotes: Vec::new(),
    };
    let mut owners = vec![owner(root)];
    // Depth first, each node with the index of the owner its quotes go to.
    let mut stack = vec![(root, 0)];
    while let Some((node, mut at)) = stack.pop() {
        if node.page.is_some() && !std::ptr::eq(node, root) {
            owners.push(owner(node));
            at = owners.len() - 1;
        }
        let quotes = node
            .quote
            .iter()
            .chain(node.motivation.as_ref().and_then(|m| m.quote.as_ref()));
        owners[at].quotes.extend(quotes);
        stack.extend(node.children.iter().rev().map(|c| (c, at)));
    }
    owners
}

/// Why an answer whose quotes leave most of a source's sections unquoted is sent back, or
/// None if no source has a [`Coverage::shortfall`]. The message names where the quotes come
/// from and the sections left unquoted, and asks for the source to be read in full.
pub fn rejection(coverage: &[Coverage], sources: &[Source]) -> Option<String> {
    let mut parts = Vec::new();
    for c in coverage {
        let Some(unquoted) = c.shortfall() else {
            continue;
        };
        let path = sources
            .iter()
            .find(|s| s.id == c.source)
            .map_or(c.source.clone(), |s| s.path.display().to_string());
        let from = c
            .span
            .map_or("no lines".to_string(), |(a, b)| format!("lines {a}–{b}"));
        let sections = unquoted
            .iter()
            .map(|s| {
                format!(
                    "- {} (lines {}–{}): {} theorem-like statements, {} displayed equations",
                    s.title, s.lines.0, s.lines.1, s.statements, s.displays
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        parts.push(format!(
            "The quotes from `{path}` come from {from} of its {} lines, and none comes from \
             these sections, which develop results in theorems, lemmas, definitions or displayed equations:\n{sections}",
            c.lines
        ));
    }
    if parts.is_empty() {
        return None;
    }
    Some(format!(
        "answer rejected: it is drawn from part of the sources.\n\n{}\n\nAn introduction \
         summarises results; the sections after it develop them, and the target depends on \
         that development. Read each source to its last line, in parts (the Read tool's \
         offset and limit), and revise the answer: keep what holds, add what the target \
         depends on, and quote each concept or step where the sources develop it rather than \
         where the introduction mentions it. A section the target does not depend on may stay \
         unquoted.",
        parts.join("\n\n")
    ))
}

impl Coverage {
    /// The top-level sections that develop mathematics (hold theorem-like statements or
    /// displayed equations) and no quote, if they are more than half of the top-level
    /// sections that develop mathematics: the mark of an answer drawn from part of a source.
    /// None for a source whose sections are not marked up.
    pub fn shortfall(&self) -> Option<Vec<&SectionCoverage>> {
        let top = self.sections.iter().map(|s| s.level).min()?;
        let holding: Vec<&SectionCoverage> = self
            .sections
            .iter()
            .filter(|s| {
                s.level == top && s.statements + s.displays > 0 && s.title != "(front matter)"
            })
            .collect();
        let unquoted: Vec<&SectionCoverage> =
            holding.iter().copied().filter(|s| s.quotes == 0).collect();
        (self.structured && unquoted.len() * 2 > holding.len()).then_some(unquoted)
    }

    /// Lines for `exegenius status`: the reach, the top-level sections quoted, statements in
    /// unquoted sections, and up to `names` concepts or steps quoted away from the sections
    /// their names head.
    pub fn summary(&self, names: usize) -> Vec<String> {
        let mut out = Vec::new();
        let span = self.span.map_or("none located".to_string(), |(a, b)| {
            format!("lines {a}–{b} of {}", self.lines)
        });
        out.push(format!(
            "{}: {} quotes, {} located ({} occur more than once and are placed at the first); \
             {span}; {} in the first tenth",
            self.source, self.quotes, self.located, self.repeated, self.first_tenth
        ));
        if !self.structured {
            out.push(
                "sections and theorem-like statements not reported: the source's headings are \
                 not marked up (LaTeX or markdown)"
                    .to_string(),
            );
            return out;
        }
        let top = self.sections.iter().map(|s| s.level).min().unwrap_or(1);
        let tops: Vec<&SectionCoverage> = self.sections.iter().filter(|s| s.level == top).collect();
        let with: Vec<String> = tops
            .iter()
            .filter(|s| s.quotes > 0)
            .map(|s| format!("{} ({})", s.title, s.quotes))
            .collect();
        if !tops.is_empty() {
            out.push(format!(
                "top-level sections with a quote: {} of {}{}",
                with.len(),
                tops.len(),
                if with.is_empty() {
                    String::new()
                } else {
                    format!(": {}", with.join(", "))
                }
            ));
        }
        if self.statements > 0 {
            out.push(format!(
                "theorem-like statements in sections with no quote: {} of {}",
                self.unquoted_statements.len(),
                self.statements
            ));
        }
        for e in self.elsewhere.iter().take(names) {
            out.push(format!(
                "`{}` is named in the heading of {} but quoted only in {}",
                e.id,
                e.headings.join("; "),
                e.quoted_in.join("; ")
            ));
        }
        if self.elsewhere.len() > names {
            out.push(format!(
                "and {} more named in a heading but quoted elsewhere",
                self.elsewhere.len() - names
            ));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    const PAPER: &str = "\\documentclass{article}
\\newtheorem{theorem}{Theorem}
\\newtheorem{lemma}[theorem]{Lemma}
\\begin{abstract}
We multiply integers quickly.
\\end{abstract}
\\section{Introduction}
We multiply integers quickly by resampling.

\\section{Gaussian resampling}
\\begin{lemma}\\label{lem:resample}
Resampling costs little.
\\end{lemma}
\\subsection{The error bound}
\\begin{lemma}
\\label{lem:error}
The error is small.
\\end{lemma}
\\section{Assembly}
\\begin{theorem}
The algorithm runs in time $O(n)$.
\\end{theorem}
";

    fn source() -> Source {
        Source::new("paper", None, None, Path::new("p.tex"), PAPER)
    }

    fn quote(text: &str) -> Evidence {
        Evidence {
            source: "paper".into(),
            quote: text.into(),
        }
    }

    #[test]
    fn sections_statements_and_lines_are_read_from_latex() {
        let s = source();
        let titles: Vec<(&str, usize, usize)> = s
            .sections()
            .iter()
            .map(|h| (h.title.as_str(), h.level, h.line))
            .collect();
        assert_eq!(
            titles,
            [
                ("§1 Introduction", 1, 7),
                ("§2 Gaussian resampling", 1, 10),
                ("§2.1 The error bound", 2, 14),
                ("§3 Assembly", 1, 19),
            ]
        );
        let labels: Vec<(&str, Option<&str>, usize)> = s
            .statements()
            .iter()
            .map(|t| (t.kind.as_str(), t.label.as_deref(), t.line))
            .collect();
        assert_eq!(
            labels,
            [
                ("Lemma", Some("lem:resample"), 11),
                ("Lemma", Some("lem:error"), 15),
                ("Theorem", None, 20),
            ]
        );
        let at = s.normalised.find("The error is small").unwrap();
        assert_eq!(s.line_at(at), 17);
        assert_eq!(s.line_count(), 22);
    }

    #[test]
    fn a_plan_quoting_the_introduction_reaches_one_section() {
        let s = source();
        let evidence = [
            quote("by resampling"),
            quote("We multiply integers quickly"),
        ];
        let owners = vec![Owner {
            id: "gaussian-resampling".into(),
            terms: vec!["Gaussian resampling".into()],
            quotes: evidence.iter().collect(),
        }];
        let c = &coverage(&[s], &owners)[0];
        assert_eq!((c.quotes, c.located, c.repeated), (2, 2, 1));
        // The repeated quote is placed at its first occurrence, in the abstract.
        assert_eq!(c.span, Some((5, 8)));
        // Lines 1 to 3 are the first tenth of 22.
        assert_eq!(c.first_tenth, 0);
        let quoted: Vec<(&str, usize)> = c
            .sections
            .iter()
            .map(|s| (s.title.as_str(), s.quotes))
            .collect();
        assert_eq!(
            quoted,
            [
                ("(front matter)", 1),
                ("§1 Introduction", 1),
                ("§2 Gaussian resampling", 0),
                ("§2.1 The error bound", 0),
                ("§3 Assembly", 0),
            ]
        );
        assert_eq!(c.sections[2].lines, (10, 18));
        assert_eq!(c.sections[2].statements, 2);
        assert_eq!(
            c.unquoted_statements,
            [
                "Lemma `lem:resample` (line 11)",
                "Lemma `lem:error` (line 15)",
                "Theorem (line 20)"
            ]
        );
        assert_eq!(
            c.elsewhere,
            [Elsewhere {
                id: "gaussian-resampling".into(),
                term: "Gaussian resampling".into(),
                headings: vec!["§2 Gaussian resampling".into()],
                quoted_in: vec!["(front matter)".into(), "§1 Introduction".into()],
            }]
        );
        let summary = c.summary(5);
        assert!(
            summary[1].starts_with("top-level sections with a quote: 2 of 4"),
            "{summary:?}"
        );
        assert_eq!(
            summary[2],
            "theorem-like statements in sections with no quote: 3 of 3"
        );
    }

    #[test]
    fn a_quote_in_a_subsection_covers_its_statements_only() {
        let s = source();
        let evidence = [quote("The error is small."), quote("invented")];
        let owners = vec![Owner {
            id: "error".into(),
            terms: vec!["Gaussian resampling".into()],
            quotes: evidence.iter().collect(),
        }];
        let c = &coverage(&[s], &owners)[0];
        assert_eq!((c.quotes, c.located), (2, 1));
        assert_eq!(c.span, Some((17, 17)));
        assert_eq!(c.sections[2].quotes, 1, "a section counts its subsections");
        assert_eq!(
            c.unquoted_statements,
            ["Lemma `lem:resample` (line 11)", "Theorem (line 20)"]
        );
        // Quoted inside the section its name heads.
        assert!(c.elsewhere.is_empty());
    }

    #[test]
    fn each_argument_quote_belongs_to_the_nearest_presented_step() {
        let node = |id: &str, page: Option<&str>, q: Option<&str>, children: Vec<Node>| Node {
            id: id.into(),
            intention: if children.is_empty() {
                crate::argument::Intention::Infer
            } else {
                crate::argument::Intention::Show
            },
            claim: String::new(),
            from: Vec::new(),
            method: None,
            motivation: None,
            salient: None,
            adds: Vec::new(),
            page: page.map(str::to_string),
            quote: q.map(quote),
            deferred: false,
            children,
        };
        let root = node(
            "S0",
            Some("main"),
            None,
            vec![
                node("S1", None, Some("We begin."), vec![]),
                node(
                    "S2",
                    Some("resampling"),
                    None,
                    vec![node("S2.1", None, Some("The error is small."), vec![])],
                ),
            ],
        );
        let owners: Vec<(String, usize)> = argument_owners(&root, None)
            .iter()
            .map(|o| (o.id.clone(), o.quotes.len()))
            .collect();
        assert_eq!(owners, [("S0".to_string(), 1), ("S2".to_string(), 1)]);
    }

    #[test]
    fn an_answer_missing_most_sections_with_statements_falls_short() {
        let owners_quoting = |texts: &[&str]| -> Vec<Coverage> {
            let evidence: Vec<Evidence> = texts.iter().map(|t| quote(t)).collect();
            let owners = vec![Owner {
                id: "x".into(),
                terms: Vec::new(),
                quotes: evidence.iter().collect(),
            }];
            coverage(&[source()], &owners)
        };
        // Quotes only from the introduction: both sections with statements are unquoted.
        let intro = owners_quoting(&["by resampling"]);
        let short: Vec<&str> = intro[0]
            .shortfall()
            .unwrap()
            .iter()
            .map(|s| s.title.as_str())
            .collect();
        assert_eq!(short, ["§2 Gaussian resampling", "§3 Assembly"]);
        let message = rejection(&intro, &[source()]).unwrap();
        assert!(message.starts_with("answer rejected: it is drawn from part of the sources."));
        assert!(
            message.contains("- §2 Gaussian resampling (lines 10–18): 2 theorem-like statements, 0 displayed equations"),
            "{message}"
        );
        assert!(
            message.contains("come from lines 8–8 of its 22 lines"),
            "{message}"
        );
        // One of the two quoted, in a subsection: not more than half unquoted.
        let half = owners_quoting(&["by resampling", "The error is small."]);
        assert!(half[0].shortfall().is_none());
        assert!(rejection(&half, &[source()]).is_none());
    }

    #[test]
    fn a_section_of_displayed_equations_develops_mathematics() {
        let text = "\\section{Introduction}\nWe bound the time.\n\\section{Cost}\n\
                    The total is\n\\begin{equation*}\nT = O(n)\n\\end{equation*}\n";
        let s = Source::new("paper", None, None, Path::new("p.tex"), text);
        assert_eq!(s.displays().len(), 1);
        let evidence = [quote("We bound the time.")];
        let owners = vec![Owner {
            id: "x".into(),
            terms: Vec::new(),
            quotes: evidence.iter().collect(),
        }];
        let c = &coverage(&[s], &owners)[0];
        assert_eq!((c.sections[1].statements, c.sections[1].displays), (0, 1));
        let short: Vec<&str> = c
            .shortfall()
            .unwrap()
            .iter()
            .map(|s| s.title.as_str())
            .collect();
        assert_eq!(short, ["§2 Cost"]);
    }

    #[test]
    fn numbered_lines_are_not_trusted_as_sections() {
        let text = "Abstract text.\n1 Introduction\nWe begin.\n2 Transforms\nMore.\n";
        let s = Source::new("paper", None, None, Path::new("p.txt"), text);
        assert!(!s.marked_up());
        let evidence = [quote("We begin.")];
        let owners = vec![Owner {
            id: "x".into(),
            terms: vec!["Transforms".into()],
            quotes: evidence.iter().collect(),
        }];
        let c = &coverage(&[s], &owners)[0];
        assert_eq!((c.located, c.span), (1, Some((3, 3))));
        assert!(c.sections.is_empty() && c.unquoted_statements.is_empty());
        assert!(c.elsewhere.is_empty());
        assert!(c.summary(5)[1].starts_with("sections and theorem-like statements not reported"));
        assert!(
            c.shortfall().is_none(),
            "no judgement without marked-up sections"
        );
    }
}

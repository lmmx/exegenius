//! Relations: what a page uses each prerequisite for.
//!
//! A prerequisite edge says only that one page needs another. A relation says how: a
//! natural-language predicate that joins the two pages' concepts into a sentence ("Radix-digit
//! convolution *is reduced to* Cyclic convolution"), as in a knowledge graph whose relations
//! are text rather than types from a fixed schema. There is no list of relation types:
//! coarse labels say less than the predicate, and fine ones are unreliable (eRST's annotators
//! agree at .412 on relations outside its tree).
//!
//! The writer of a page gives one relation per prerequisite page, with the sentence on the
//! page that states it. Following eRST, a relation is accepted only with a textual signal:
//! code checks that the sentence is on the page, that it links or names the prerequisite, and
//! that it contains the predicate's words. That anchors the relation in the text; whether the
//! predicate is apt is for readers to test.

use std::collections::BTreeMap;
use std::ops::Range;

use pulldown_cmark::{Event, Options, Parser, Tag};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::graph::{Concept, Graph};
use crate::text::{key, locate};
use crate::write::link_target;

/// What this page uses one prerequisite page for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Edge {
    /// Id of the prerequisite page.
    pub prerequisite: String,
    /// A short predicate built from the sentence's own words, such that "<this page's
    /// concept> <predicate> <prerequisite>" is a true sentence: "is reduced to", "computes
    /// its transforms by", "is based on".
    pub predicate: String,
    /// The sentence on this page that states it, copied exactly. It must link or name the
    /// prerequisite and contain the predicate's words.
    pub quote: String,
}

/// Words a predicate may have that the quoted sentence need not repeat.
const AUXILIARIES: &[&str] = &[
    "a", "an", "the", "is", "are", "was", "were", "be", "been", "being", "has", "have", "had",
    "it", "its",
];

/// Lowercase words of `text`, with hyphens read as spaces.
fn words(text: &str) -> Vec<String> {
    key(text)
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_string)
        .collect()
}

/// A word without a common inflectional ending, so "reduces", "reduced" and "reducing"
/// compare equal.
fn stem(word: &str) -> &str {
    for ending in ["ing", "es", "ed", "s", "d"] {
        if let Some(root) = word.strip_suffix(ending)
            && root.chars().count() >= 3
        {
            return root;
        }
    }
    word
}

/// The words of a predicate that the quote must contain.
fn content_words(predicate: &str) -> Vec<String> {
    words(predicate)
        .into_iter()
        .filter(|w| !AUXILIARIES.contains(&w.as_str()))
        .collect()
}

/// Whether the word sequence `needle` occurs in `hay`, allowing a plural ending on its last
/// word.
fn has_words(hay: &[String], needle: &[String]) -> bool {
    let Some((last, init)) = needle.split_last() else {
        return false;
    };
    hay.windows(needle.len()).any(|w| {
        w[..init.len()] == *init
            && (w[init.len()] == *last
                || w[init.len()].strip_suffix('s') == Some(last)
                || w[init.len()].strip_suffix("es") == Some(last))
    })
}

/// Whether the quote at `range` of `page` links the page of `c`, or names it by its name or
/// an alias. A link counts if the quote covers any part of it.
fn signals(page: &str, range: &Range<usize>, c: &Concept) -> bool {
    let linked = Parser::new_ext(page, Options::ENABLE_MATH)
        .into_offset_iter()
        .any(|(event, span)| {
            matches!(&event, Event::Start(Tag::Link { dest_url, .. })
                if link_target(dest_url) == Some(c.id.as_str()))
                && span.start < range.end
                && range.start < span.end
        });
    let text = words(&plain(&page[range.clone()]));
    linked
        || std::iter::once(&c.name)
            .chain(&c.aliases)
            .any(|name| has_words(&text, &words(name)))
}

/// The text of a markdown snippet, without link destinations or markup. Brackets in the
/// text are dropped too: a quote that cuts into a link leaves its brackets unmatched.
pub fn plain(markdown: &str) -> String {
    let mut out = String::new();
    for event in Parser::new_ext(markdown, Options::ENABLE_MATH) {
        match event {
            Event::Text(t) => out.extend(t.chars().filter(|c| !matches!(c, '[' | ']'))),
            Event::Code(t) => out.push_str(&t),
            Event::InlineMath(m) => out.push_str(&format!("${m}$")),
            Event::SoftBreak | Event::HardBreak => out.push(' '),
            _ => {}
        }
    }
    out
}

/// Check the relations a writer gives for a page body against the page's prerequisite
/// pages, and tidy them. Returns errors.
pub fn check(body: &str, prerequisites: &[&Concept], edges: &mut [Edge]) -> Vec<String> {
    let page = crate::cite::strip(body);
    let mut errors = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    for e in edges.iter_mut() {
        e.prerequisite = e.prerequisite.trim().to_string();
        e.predicate = e.predicate.split_whitespace().collect::<Vec<_>>().join(" ");
        let id = e.prerequisite.as_str();
        let Some(prerequisite) = prerequisites.iter().find(|p| p.id == id) else {
            errors.push(format!("`{id}` is not a prerequisite page of this page"));
            continue;
        };
        if seen.contains(&id) {
            errors.push(format!("`{id}` has two relations"));
        }
        seen.push(prerequisite.id.as_str());
        let content = content_words(&e.predicate);
        if content.is_empty() || e.predicate.split_whitespace().count() > 6 {
            errors.push(format!(
                "`{id}`: the predicate \"{}\" must be one to six words saying what the page \
                 uses it for",
                e.predicate
            ));
            continue;
        }
        let Some(range) = locate(&page, &e.quote) else {
            errors.push(format!(
                "`{id}`: quote not found on the page: \"{}\"",
                e.quote
            ));
            continue;
        };
        if !signals(&page, &range, prerequisite) {
            errors.push(format!(
                "`{id}`: the quote neither links nor names {}: \"{}\"",
                prerequisite.name, e.quote
            ));
        }
        let quoted = words(&plain(&page[range]));
        let missing: Vec<&String> = content
            .iter()
            .filter(|w| !quoted.iter().any(|q| stem(q) == stem(w)))
            .collect();
        if !missing.is_empty() {
            let missing: Vec<&str> = missing.iter().map(|w| w.as_str()).collect();
            errors.push(format!(
                "`{id}`: the quote does not contain \"{}\" from the predicate \"{}\"; build the \
                 predicate from the sentence's own words",
                missing.join("\", \""),
                e.predicate
            ));
        }
    }
    let missing: Vec<String> = prerequisites
        .iter()
        .filter(|p| !seen.contains(&p.id.as_str()))
        .map(|p| format!("`{}`", p.id))
        .collect();
    if !missing.is_empty() {
        errors.push(format!(
            "give a relation for every prerequisite page; missing: {}",
            missing.join(", ")
        ));
    }
    errors
}

/// The relation page `page` states for its prerequisite `prerequisite`, if any.
pub fn find<'a>(graph: &'a Graph, page: &str, prerequisite: &str) -> Option<&'a Edge> {
    graph
        .relations
        .get(page)?
        .iter()
        .find(|e| e.prerequisite == prerequisite)
}

/// A sentence naming what `subject` builds on: "Subject *is reduced to* A and *uses* B.",
/// followed by "It also builds on C." for prerequisites with no relation. `links` pairs each
/// prerequisite's link with its predicate, if it has one.
pub fn sentence(subject: &str, links: &[(String, Option<&str>)]) -> String {
    let (with, without): (Vec<_>, Vec<_>) = links.iter().partition(|(_, p)| p.is_some());
    let with: Vec<String> = with
        .iter()
        .map(|(link, p)| format!("*{}* {link}", p.unwrap_or_default()))
        .collect();
    let without: Vec<String> = without.iter().map(|(link, _)| link.clone()).collect();
    let mut out = Vec::new();
    if !with.is_empty() {
        out.push(format!("{subject} {}.", crate::build::list(&with)));
    }
    if !without.is_empty() {
        let lead = if with.is_empty() {
            "This page builds on"
        } else {
            "It also builds on"
        };
        out.push(format!("{lead} {}.", crate::build::list(&without)));
    }
    out.join(" ")
}

/// How often each predicate is used across the wiki, most frequent first.
pub fn predicates(graph: &Graph) -> Vec<(String, usize)> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for e in graph.relations.values().flatten() {
        *counts.entry(e.predicate.to_lowercase()).or_default() += 1;
    }
    let mut counts: Vec<(String, usize)> = counts.into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::tests::concept;

    fn edge(prerequisite: &str, predicate: &str, quote: &str) -> Edge {
        Edge {
            prerequisite: prerequisite.into(),
            predicate: predicate.into(),
            quote: quote.into(),
        }
    }

    #[test]
    fn relations_need_a_signal_on_the_page() {
        let mut cyclic = concept("cyclic", &[], false);
        cyclic.name = "Cyclic convolution".into();
        let mut dft = concept("dft", &[], false);
        dft.aliases = vec!["DFT".into()];
        let body = "The product is reduced to [a cyclic convolution](cyclic.md) of length $S$. \
                    Each cyclic convolution is computed by a DFT.";
        let prerequisites = [&cyclic, &dft];
        let mut good = vec![
            edge(
                "cyclic",
                "is reduced to",
                "The product is reduced to [a cyclic",
            ),
            edge(
                "dft",
                "computes  each convolution by",
                "Each cyclic convolution is computed by a DFT.",
            ),
        ];
        assert!(check(body, &prerequisites, &mut good).is_empty());
        assert_eq!(good[1].predicate, "computes each convolution by");
        let mut bad = vec![
            edge("cyclic", "is", "The product is reduced"),
            edge("dft", "is evaluated by", "The product is reduced to"),
            edge("other", "uses", "x"),
        ];
        let errors = check(body, &prerequisites, &mut bad);
        assert!(errors[0].contains("one to six words"), "{errors:?}");
        assert!(errors[1].contains("neither links nor names"), "{errors:?}");
        assert!(errors[2].contains("\"evaluated\", \"by\""), "{errors:?}");
        assert!(
            errors[3].contains("`other` is not a prerequisite"),
            "{errors:?}"
        );
        assert_eq!(errors.len(), 4);
        let errors = check(body, &prerequisites, &mut good[1..]);
        assert_eq!(
            errors,
            vec!["give a relation for every prerequisite page; missing: `cyclic`"]
        );
    }

    #[test]
    fn sentences_put_predicates_first() {
        let links = [
            ("[A](a.md)".to_string(), Some("is reduced to")),
            ("[B](b.md)".to_string(), None),
            ("[C](c.md)".to_string(), Some("uses")),
        ];
        assert_eq!(
            sentence("Radix", &links),
            "Radix *is reduced to* [A](a.md) and *uses* [C](c.md). It also builds on [B](b.md)."
        );
        assert_eq!(
            sentence("Radix", &links[1..2]),
            "This page builds on [B](b.md)."
        );
    }
}

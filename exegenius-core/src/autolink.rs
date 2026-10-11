//! Link the first unlinked mention of each other page in a page body, at build time.
//!
//! Matching is conservative, because a wrong link is worse than a missing one. A name of
//! several words matches whatever its ASCII case ("cyclic convolution" in a sentence
//! matches the page "Cyclic convolution"). A single word matches only if it looks like a
//! name rather than an ordinary word: an acronym or a word with digits or inner capitals
//! ("DFT", "FFT", "L2"), matched with its exact case. Single ordinary words ("Record",
//! "Row") are never linked automatically. A trailing plural "s" or "es" joins the link.
//! Code, mathematics, headings and existing links are left alone.

use std::collections::HashSet;
use std::ops::Range;

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

use crate::graph::Concept;
use crate::write::link_target;

/// A name or alias that links to a page.
#[derive(Debug, Clone, PartialEq)]
pub struct Term {
    pub text: String,
    /// Destination, relative to `docs/`, e.g. `dft.md` or `glossary.md#record`.
    pub dest: String,
    /// Page id, so each page is linked once.
    pub id: String,
    exact: bool,
}

/// The terms for `concepts`, longest first. `dest` gives each concept's destination.
pub fn terms<'a>(
    concepts: impl IntoIterator<Item = &'a Concept>,
    dest: impl Fn(&Concept) -> String,
) -> Vec<Term> {
    let mut terms = Vec::new();
    for c in concepts {
        for text in std::iter::once(&c.name).chain(&c.aliases) {
            let text = text.trim();
            let exact = if text.split_whitespace().count() > 1 {
                false
            } else if is_name_like(text) {
                true
            } else {
                continue;
            };
            terms.push(Term {
                text: text.to_string(),
                dest: dest(c),
                id: c.id.clone(),
                exact,
            });
        }
    }
    terms.sort_by_key(|t| std::cmp::Reverse(t.text.chars().count()));
    terms
}

/// An acronym or a word with digits or inner capitals, at least two characters long.
pub(crate) fn is_name_like(word: &str) -> bool {
    word.chars().count() >= 2
        && word.chars().any(char::is_alphabetic)
        && word
            .chars()
            .skip(1)
            .any(|c| c.is_uppercase() || c.is_ascii_digit())
}

fn boundary(c: Option<char>) -> bool {
    c.is_none_or(|c| !c.is_alphanumeric())
}

/// The byte range of the first whole-word match of `term` in `text` at or after `from`,
/// including a plural ending.
fn find(text: &str, term: &Term, claimed: &[Range<usize>]) -> Option<Range<usize>> {
    let (hay, needle) = if term.exact {
        (text.to_string(), term.text.clone())
    } else {
        (text.to_ascii_lowercase(), term.text.to_ascii_lowercase())
    };
    let mut from = 0;
    while let Some(i) = hay[from..].find(&needle) {
        let start = from + i;
        let mut end = start + needle.len();
        from = start + needle.len().max(1);
        if !boundary(text[..start].chars().next_back()) {
            continue;
        }
        for plural in ["es", "s"] {
            let rest = &text[end..];
            if rest.starts_with(plural) && boundary(rest[plural.len()..].chars().next()) {
                end += plural.len();
                break;
            }
        }
        if !boundary(text[end..].chars().next()) {
            continue;
        }
        if claimed.iter().any(|r| r.start < end && start < r.end) {
            continue;
        }
        return Some(start..end);
    }
    None
}

/// Link the first unlinked mention of each term's page in `body`, skipping the page
/// `own` and pages the body already links.
pub fn autolink(body: &str, own: &str, terms: &[Term]) -> String {
    let parser = || Parser::new_ext(body, Options::ENABLE_MATH | Options::ENABLE_TABLES);
    let mut linked: HashSet<String> = parser()
        .filter_map(|event| match event {
            Event::Start(Tag::Link { dest_url, .. }) => {
                link_target(&dest_url).map(|id| id.to_string())
            }
            _ => None,
        })
        .collect();
    linked.insert(own.to_string());

    let mut edits: Vec<(Range<usize>, String)> = Vec::new();
    let mut skip = 0usize;
    for (event, range) in parser().into_offset_iter() {
        match event {
            Event::Start(Tag::Link { .. } | Tag::Image { .. } | Tag::Heading { .. }) => skip += 1,
            Event::Start(Tag::CodeBlock(_) | Tag::HtmlBlock) => skip += 1,
            // Citation marks hold source text, which must stay as written.
            Event::InlineHtml(html) if html.starts_with("<cite") => skip += 1,
            Event::InlineHtml(html) if html.starts_with("</cite") => skip = skip.saturating_sub(1),
            Event::End(
                TagEnd::Link
                | TagEnd::Image
                | TagEnd::Heading(_)
                | TagEnd::CodeBlock
                | TagEnd::HtmlBlock,
            ) => skip = skip.saturating_sub(1),
            Event::Text(text) if skip == 0 && body.get(range.clone()) == Some(&*text) => {
                let mut claimed: Vec<Range<usize>> = Vec::new();
                for term in terms {
                    if linked.contains(&term.id) {
                        continue;
                    }
                    if let Some(found) = find(&text, term, &claimed) {
                        let original = &text[found.clone()];
                        edits.push((
                            range.start + found.start..range.start + found.end,
                            format!("[{original}]({})", term.dest),
                        ));
                        claimed.push(found);
                        linked.insert(term.id.clone());
                    }
                }
            }
            _ => {}
        }
    }
    edits.sort_by_key(|(r, _)| r.start);
    let mut out = body.to_string();
    for (range, text) in edits.into_iter().rev() {
        out.replace_range(range, &text);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn concept(id: &str, name: &str, aliases: &[&str]) -> Concept {
        Concept {
            id: id.into(),
            name: name.into(),
            aliases: aliases.iter().map(|a| a.to_string()).collect(),
            gloss: String::new(),
            requires: vec![],
            known: false,
            evidence: vec![],
        }
    }

    fn all() -> Vec<Term> {
        let concepts = [
            concept("cyclic-convolution", "Cyclic convolution", &[]),
            concept("convolution", "Linear convolution", &["convolution"]),
            concept("dft", "Discrete Fourier transform", &["DFT"]),
            concept("record", "Record", &[]),
            concept("fft", "Fast Fourier transform", &["FFT"]),
        ];
        terms(&concepts, |c| format!("{}.md", c.id))
    }

    #[test]
    fn links_first_mentions_conservatively() {
        let body = "Take the DFT of each record. Cyclic convolutions wrap; a cyclic convolution \
                    again. The dft and the [FFT](fft.md) and `DFT` and $DFT$ stay.\n\n\
                    ## Discrete Fourier transform\n\nAn ODFT is not a DFT.";
        assert_eq!(
            autolink(body, "fft", &all()),
            "Take the [DFT](dft.md) of each record. [Cyclic convolutions](cyclic-convolution.md) \
             wrap; a cyclic convolution again. The dft and the [FFT](fft.md) and `DFT` and \
             $DFT$ stay.\n\n## Discrete Fourier transform\n\nAn ODFT is not a DFT."
        );
    }

    #[test]
    fn own_page_and_single_ordinary_words_are_not_linked() {
        let terms = all();
        assert!(
            terms
                .iter()
                .all(|t| t.text != "Record" && t.text != "convolution")
        );
        assert_eq!(
            autolink("The cyclic convolution here.", "cyclic-convolution", &terms),
            "The cyclic convolution here."
        );
    }
}

//! Inline citations: a claim on a page can carry the source text it rests on.
//!
//! Writers mark a claim with `<cite data-source="id">text copied from the source</cite>`
//! right after it. Submit checks that the text is in that source, word for word. The build
//! turns each mark into `<cite class="exegenius-ref">(see p. 5)</cite>`, or a section or
//! the source's title when the source has no page breaks, with the source id, locator and
//! quote kept as attributes for whatever later use. The marks are hidden by default (a
//! toggle on the page shows them), and left out of what auditors and simulated readers
//! see.

use std::ops::Range;

use crate::project::{Source, find_source};

/// One citation mark in a page body.
#[derive(Debug, Clone, PartialEq)]
pub struct Mark {
    /// Byte range of the whole `<cite …>…</cite>` element.
    pub range: Range<usize>,
    pub source: String,
    pub quote: String,
}

/// The citation marks in `body`, and descriptions of any that are malformed.
pub fn marks(body: &str) -> (Vec<Mark>, Vec<String>) {
    let mut found = Vec::new();
    let mut errors = Vec::new();
    let mut from = 0;
    while let Some(i) = body[from..].find("<cite") {
        let start = from + i;
        let Some(open_end) = body[start..].find('>').map(|j| start + j + 1) else {
            errors.push("a <cite> tag is not closed with `>`".into());
            break;
        };
        let Some(close) = body[open_end..].find("</cite>").map(|j| open_end + j) else {
            errors.push("a <cite> element has no </cite>".into());
            break;
        };
        let tag = &body[start..open_end];
        let end = close + "</cite>".len();
        from = end;
        let source = tag
            .split_once("data-source=\"")
            .and_then(|(_, rest)| rest.split_once('"'))
            .map(|(id, _)| id.trim().to_string());
        let quote = body[open_end..close].trim().to_string();
        match source {
            Some(source) if !quote.is_empty() => found.push(Mark {
                range: start..end,
                source,
                quote,
            }),
            Some(_) => errors.push("a <cite> element is empty".into()),
            None => errors.push(format!(
                "`{tag}` has no data-source=\"…\" naming the source"
            )),
        }
    }
    (found, errors)
}

/// Problems with the citation marks in `body`: malformed marks, unknown sources, and
/// quotes not found in their source.
pub fn check(body: &str, sources: &[Source]) -> Vec<String> {
    let (found, mut errors) = marks(body);
    for mark in found {
        match find_source(sources, &mark.source) {
            None => errors.push(format!(
                "<cite data-source=\"{}\">: there is no source with that id",
                mark.source
            )),
            Some(s) if s.locate(&mark.quote).is_none() => errors.push(format!(
                "<cite data-source=\"{}\">: \"{}\" is not in that source word for word",
                mark.source, mark.quote
            )),
            Some(_) => {}
        }
    }
    errors
}

/// Replace each mark (with the space before it) by `f(mark)`.
fn replace(body: &str, f: impl Fn(&Mark) -> String) -> String {
    let (found, _) = marks(body);
    let mut out = body.to_string();
    for mark in found.into_iter().rev() {
        let mut start = mark.range.start;
        if out[..start].ends_with(' ') {
            start -= 1;
        }
        out.replace_range(start..mark.range.end, &f(&mark));
    }
    out
}

/// `body` without its citation marks.
pub fn strip(body: &str) -> String {
    replace(body, |_| String::new())
}

fn attribute(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// `body` with its citation marks rendered for the site.
pub fn render(body: &str, sources: &[Source]) -> String {
    replace(body, |mark| {
        let source = find_source(sources, &mark.source);
        let locator = source.and_then(|s| s.locate(&mark.quote)).flatten();
        let mut label = Vec::new();
        if sources.len() > 1 || locator.is_none() {
            label.push(source.map_or(mark.source.clone(), |s| s.title.clone()));
        }
        label.extend(locator.clone());
        format!(
            "<cite class=\"exegenius-ref\" data-source=\"{}\" data-locator=\"{}\" \
             data-quote=\"{}\" title=\"{}\"> (see {})</cite>",
            attribute(&mark.source),
            attribute(locator.as_deref().unwrap_or("")),
            attribute(&mark.quote),
            attribute(&mark.quote),
            attribute(&label.join(", "))
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn sources() -> Vec<Source> {
        let text = "Intro text.\n3.2 Gaussian resampling\nWe resample the grid.\n\u{c}\
                    Page two holds the chirp.";
        vec![Source::new(
            "paper",
            Some("Paper"),
            None,
            Path::new("p.txt"),
            text,
        )]
    }

    #[test]
    fn marks_are_checked_against_their_source() {
        let body = "Resampling helps <cite data-source=\"paper\">We resample the grid.</cite>, \
                    see <cite data-source=\"paper\">a made-up quote</cite> and \
                    <cite data-source=\"other\">x</cite> and <cite>y</cite>.";
        let errors = check(body, &sources());
        assert_eq!(errors.len(), 3, "{errors:?}");
        assert!(errors[0].contains("has no data-source"));
        assert!(errors[1].contains("\"a made-up quote\" is not in that source"));
        assert!(errors[2].contains("no source with that id"));
    }

    #[test]
    fn marks_render_with_a_locator_and_strip_cleanly() {
        let body = "It resamples <cite data-source=\"paper\">We resample the grid.</cite>. \
                    Then <cite data-source=\"paper\">holds the chirp</cite>.";
        assert_eq!(strip(body), "It resamples. Then.");
        let html = render(body, &sources());
        assert!(
            html.contains("data-locator=\"§3.2 Gaussian resampling, p. 1\""),
            "{html}"
        );
        assert!(html.contains("> (see §3.2 Gaussian resampling, p. 1)</cite>."));
        assert!(html.contains("> (see §3.2 Gaussian resampling, p. 2)</cite>."));
        assert!(html.starts_with("It resamples<cite class=\"exegenius-ref\""));
    }
}

//! Text normalisation for quote checks, term matching and ids.

/// Normalise text for verbatim comparison: typographic quotes and dashes become ASCII,
/// soft hyphens are removed, and whitespace runs collapse to one space.
pub fn normalise(s: &str) -> String {
    normalise_mapped(s).0
}

/// [`normalise`], with the byte offset in `s` that each byte of the result comes from.
fn normalise_mapped(s: &str) -> (String, Vec<usize>) {
    let mut out = String::with_capacity(s.len());
    let mut from = Vec::with_capacity(s.len());
    let mut pending_space = None;
    for (i, c) in s.char_indices() {
        let c = match c {
            '\u{2018}' | '\u{2019}' | '\u{201B}' | '\u{2032}' => '\'',
            '\u{201C}' | '\u{201D}' | '\u{201F}' | '\u{2033}' => '"',
            '\u{2010}'..='\u{2015}' | '\u{2212}' => '-',
            '\u{00AD}' => continue,
            c => c,
        };
        if c.is_whitespace() {
            pending_space.get_or_insert(i);
            continue;
        }
        if let Some(space) = pending_space.take()
            && !out.is_empty()
        {
            out.push(' ');
            from.push(space);
        }
        out.push(c);
        from.resize(out.len(), i);
    }
    (out, from)
}

/// The byte range in `haystack` of the first verbatim match of `quote`, compared as by
/// [`contains_quote`].
pub fn locate(haystack: &str, quote: &str) -> Option<std::ops::Range<usize>> {
    let quote = normalise(quote);
    if quote.is_empty() {
        return None;
    }
    let (normalised, from) = normalise_mapped(haystack);
    let at = normalised.find(&quote)?;
    let last = from[at + quote.len() - 1];
    let end = last + haystack[last..].chars().next().map_or(0, char::len_utf8);
    Some(from[at]..end)
}

/// Whether `quote` appears verbatim in `haystack`, which must already be normalised.
pub fn contains_quote(normalised_haystack: &str, quote: &str) -> bool {
    let quote = normalise(quote);
    !quote.is_empty() && normalised_haystack.contains(&quote)
}

/// Matching key for a term: normalised, lowercased, with hyphens and underscores read as
/// spaces and surrounding punctuation removed.
pub fn key(s: &str) -> String {
    let lowered: String = normalise(s)
        .to_lowercase()
        .chars()
        .map(|c| if c == '-' || c == '_' { ' ' } else { c })
        .collect();
    normalise(lowered.trim_matches(|c: char| !c.is_alphanumeric()))
}

/// Lowercase slug for ids and file names.
pub fn slug(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars().flat_map(char::to_lowercase) {
        if c.is_alphanumeric() {
            out.push(c);
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalise_collapses_whitespace_and_typography() {
        assert_eq!(
            normalise("  a\u{2019}s\n\t“chirp”  \u{2013} x\u{00AD}y "),
            "a's \"chirp\" - xy"
        );
    }

    #[test]
    fn quotes_match_across_line_breaks() {
        let source = normalise("converts these through a chirp\nidentity to convolution.");
        assert!(contains_quote(&source, "through a chirp identity"));
        assert!(!contains_quote(&source, "through the chirp identity"));
        assert!(!contains_quote(&source, "   "));
    }

    #[test]
    fn quotes_are_located_in_the_original_text() {
        let text = "One  “two”\nthree four";
        let range = locate(text, "\"two\" three").unwrap();
        assert_eq!(&text[range], "“two”\nthree");
        assert_eq!(locate(text, "five"), None);
    }

    #[test]
    fn keys_and_slugs() {
        assert_eq!(key("Chirp-Identity."), "chirp identity");
        assert_eq!(key("chirp_identity"), "chirp identity");
        assert_eq!(slug("Chirp Z-transform (CZT)"), "chirp-z-transform-czt");
        assert_eq!(slug("--DFT--"), "dft");
    }
}

//! Text normalisation for quote checks, term matching and ids.

/// Normalise text for verbatim comparison: typographic quotes and dashes become ASCII,
/// soft hyphens are removed, and whitespace runs collapse to one space.
pub fn normalise(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut pending_space = false;
    for c in s.chars() {
        let c = match c {
            '\u{2018}' | '\u{2019}' | '\u{201B}' | '\u{2032}' => '\'',
            '\u{201C}' | '\u{201D}' | '\u{201F}' | '\u{2033}' => '"',
            '\u{2010}'..='\u{2015}' | '\u{2212}' => '-',
            '\u{00AD}' => continue,
            c => c,
        };
        if c.is_whitespace() {
            pending_space = true;
            continue;
        }
        if pending_space && !out.is_empty() {
            out.push(' ');
        }
        pending_space = false;
        out.push(c);
    }
    out
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
    fn keys_and_slugs() {
        assert_eq!(key("Chirp-Identity."), "chirp identity");
        assert_eq!(key("chirp_identity"), "chirp identity");
        assert_eq!(slug("Chirp Z-transform (CZT)"), "chirp-z-transform-czt");
        assert_eq!(slug("--DFT--"), "dft");
    }
}

//! Symbol bindings: what each symbol on a page denotes, and where the reader learns it.
//!
//! A binding gives a symbol a display name ("record length"), a meaning (one sentence), and
//! a relation: the symbol denotes something directly, or it counts instances of something
//! (the DIRECT and COUNT relations of Symlink, SemEval-2022 Task 12). Bindings come from
//! two places:
//!
//! - **Concept names.** A name with mathematics in it ("Polynomial ring modulo y^r + 1")
//!   carries a title with that mathematics in LaTeX and a binding for each symbol in it,
//!   proposed at extraction. A reader who knows the concept is then told what its symbols
//!   denote, instead of being assumed to know them.
//! - **Pages.** Each writer lists the symbols its page binds, each with the sentence on the
//!   page that says what it denotes.
//!
//! Code never reads mathematics for meaning. It checks only that a named symbol occurs in a
//! given piece of LaTeX as a run of whole TeX tokens, so `r` occurs in `y^r + 1` but not
//! inside `\frac`, and `y^{r}` matches `y^r`.

use pulldown_cmark::{Event, Options, Parser};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::graph::{Concept, Graph};
use crate::plan::Plan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Relation {
    /// The symbol denotes a thing: "ω, a primitive root of unity".
    Direct,
    /// The symbol is the number of instances of something: "r, the number of complex
    /// coefficients in a record".
    Count,
}

/// What a symbol in a concept's name denotes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Binding {
    /// The symbol in LaTeX, as it appears in the title's mathematics, without dollar signs,
    /// e.g. `r` or `\kappa`.
    pub symbol: String,
    /// Display name of one to four words, e.g. "record length".
    pub name: String,
    /// One sentence saying what the symbol denotes; for a count, what it counts.
    pub meaning: String,
    pub relation: Relation,
}

/// What a symbol a page introduces denotes, and where the page says so.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PageBinding {
    /// The symbol in LaTeX, as it appears in the page's mathematics, without dollar signs,
    /// e.g. `r` or `\kappa`.
    pub symbol: String,
    /// Display name of one to four words, e.g. "record length".
    pub name: String,
    /// One sentence saying what the symbol denotes; for a count, what it counts.
    pub meaning: String,
    pub relation: Relation,
    /// The sentence on the page that says what the symbol denotes, copied exactly. A use
    /// in an example ("take $r = 4$") does not say what a symbol denotes.
    pub quote: String,
}

/// The TeX tokens of `latex`: control words (`\kappa`), control symbols (`\{`) and single
/// characters, leaving out spacing and grouping braces.
fn tokens(latex: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut rest = latex;
    while let Some(c) = rest.chars().next() {
        let len = match c {
            '\\' => match rest[1..].chars().next() {
                Some(a) if a.is_ascii_alphabetic() => {
                    1 + rest[1..]
                        .find(|x: char| !x.is_ascii_alphabetic())
                        .unwrap_or(rest.len() - 1)
                }
                Some(a) => 1 + a.len_utf8(),
                None => 1,
            },
            c => c.len_utf8(),
        };
        let token = &rest[..len];
        let spacing = matches!(token, "\\," | "\\;" | "\\:" | "\\!" | "\\ " | "~");
        if !(c.is_whitespace() || spacing || token == "{" || token == "}") {
            out.push(token);
        }
        rest = &rest[len..];
    }
    out
}

/// A symbol as written in an answer: trimmed, without dollar signs.
pub fn clean(symbol: &str) -> String {
    symbol.trim().trim_matches('$').trim().to_string()
}

/// Whether two symbols are the same TeX tokens.
pub fn same(a: &str, b: &str) -> bool {
    tokens(a) == tokens(b)
}

/// The mathematics of a markdown text, one string per `$...$` or `$$...$$`.
pub fn math(text: &str) -> Vec<String> {
    Parser::new_ext(text, Options::ENABLE_MATH)
        .filter_map(|event| match event {
            Event::InlineMath(m) | Event::DisplayMath(m) => Some(m.to_string()),
            _ => None,
        })
        .collect()
}

/// Where a quote stands, for a symbol it is meant to contain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quoted {
    /// The quote is not on the page.
    Missing,
    /// The quote is on the page, but the symbol is not in the mathematics it covers.
    WithoutSymbol,
    /// The quote covers the symbol, but the page uses the symbol before the quote.
    Late,
    Found,
}

/// Whether `quote` is on `page` and covers an occurrence of `symbol` in the page's
/// mathematics. The quote may start or end inside a formula, or lie wholly inside one.
pub fn quoted(page: &str, quote: &str, symbol: &str) -> Quoted {
    let Some(range) = crate::text::locate(page, quote) else {
        return Quoted::Missing;
    };
    let wanted = tokens(symbol);
    let has = |slice: &str| {
        tokens(slice)
            .windows(wanted.len().max(1))
            .any(|w| w == wanted)
    };
    let spans = math_spans(page);
    let found = spans.iter().any(|span| {
        let (start, end) = (span.start.max(range.start), span.end.min(range.end));
        start < end && has(&page[start..end])
    });
    let earlier = spans
        .iter()
        .any(|span| span.end <= range.start && has(&page[span.clone()]));
    match (found, earlier) {
        (false, _) => Quoted::WithoutSymbol,
        (true, true) => Quoted::Late,
        (true, false) => Quoted::Found,
    }
}

/// Byte ranges of the contents of each `$...$` and `$$...$$` in a markdown text.
fn math_spans(text: &str) -> Vec<std::ops::Range<usize>> {
    Parser::new_ext(text, Options::ENABLE_MATH)
        .into_offset_iter()
        .filter_map(|(event, range)| match event {
            Event::InlineMath(m) | Event::DisplayMath(m) => {
                let start = range.start + text[range.clone()].find(&*m).unwrap_or(0);
                Some(start..(start + m.len()).min(range.end))
            }
            _ => None,
        })
        .collect()
}

/// Whether `symbol` occurs as whole TeX tokens in one of the pieces of mathematics.
pub fn occurs(symbol: &str, math: &[String]) -> bool {
    let wanted = tokens(symbol);
    !wanted.is_empty()
        && math
            .iter()
            .any(|m| tokens(m).windows(wanted.len()).any(|w| w == wanted))
}

/// One line describing a binding, for task files: "$r$ (record length): …".
pub fn describe(symbol: &str, name: &str, meaning: &str) -> String {
    format!("${symbol}$ ({name}): {}", meaning.trim())
}

/// The error for a quote that should be on the page and contain the symbol, if it does not.
pub fn quote_error(page: &str, quote: &str, symbol: &str) -> Option<String> {
    match quoted(page, quote, symbol) {
        Quoted::Found | Quoted::Late => None,
        Quoted::Missing => Some(format!(
            "`{symbol}`: quote not found on the page: \"{quote}\""
        )),
        Quoted::WithoutSymbol => Some(format!(
            "`{symbol}`: the quote must cover the symbol in the page's mathematics: \"{quote}\""
        )),
    }
}

/// Problems with the shared fields of a binding.
fn check_fields(symbol: &str, name: &str, meaning: &str, errors: &mut Vec<String>) {
    let words = name.split_whitespace().count();
    if !(1..=4).contains(&words) {
        errors.push(format!(
            "`{symbol}`: the display name \"{}\" must be one to four words",
            name.trim()
        ));
    }
    if meaning.trim().is_empty() {
        errors.push(format!("`{symbol}` has no meaning"));
    }
}

/// Check the bindings of a concept name against its title, and tidy them. Returns errors.
pub fn check_name(c: &mut Concept) -> Vec<String> {
    let mut errors = Vec::new();
    if c.title.as_deref().is_some_and(|t| t.trim().is_empty()) {
        c.title = None;
    }
    let Some(title) = &c.title else {
        if !c.symbols.is_empty() {
            errors.push(format!(
                "`{}` has symbols but no title: give the name with its mathematics in LaTeX \
                 as `title`",
                c.id
            ));
        }
        return errors;
    };
    let title_math = math(title);
    let mut seen: Vec<String> = Vec::new();
    for b in &mut c.symbols {
        b.symbol = clean(&b.symbol);
        let symbol = b.symbol.clone();
        check_fields(&symbol, &b.name, &b.meaning, &mut errors);
        if !occurs(&symbol, &title_math) {
            errors.push(format!(
                "`{}`: `{symbol}` does not occur in the mathematics of the title \"{title}\"",
                c.id
            ));
        }
        if seen.iter().any(|s| same(s, &symbol)) {
            errors.push(format!("`{}`: `{symbol}` is bound twice", c.id));
        }
        seen.push(symbol);
    }
    errors
}

/// Check the bindings a writer gives for a page body, and tidy them. Returns errors.
pub fn check_page(body: &str, bindings: &mut [PageBinding]) -> Vec<String> {
    let mut errors = Vec::new();
    let page = crate::cite::strip(body);
    let page_math = math(&page);
    let mut seen: Vec<String> = Vec::new();
    for b in bindings.iter_mut() {
        b.symbol = clean(&b.symbol);
        let symbol = b.symbol.clone();
        if symbol.is_empty() {
            errors.push("a binding has an empty symbol".into());
            continue;
        }
        check_fields(&symbol, &b.name, &b.meaning, &mut errors);
        if !occurs(&symbol, &page_math) {
            errors.push(format!(
                "`{symbol}` does not occur in the page's mathematics; give it as written there"
            ));
        } else if quoted(&page, &b.quote, &symbol) == Quoted::Late {
            errors.push(format!(
                "`{symbol}` is used before \"{}\"; say what it denotes where it is first used, \
                 and quote that sentence",
                b.quote
            ));
        } else {
            errors.extend(quote_error(&page, &b.quote, &symbol));
        }
        if seen.iter().any(|s| same(s, &symbol)) {
            errors.push(format!("`{symbol}` is bound twice"));
        }
        seen.push(symbol);
    }
    errors
}

/// Where a binding comes from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Origin<'a> {
    /// The name of this concept.
    Name(&'a Concept),
    /// The body of this page.
    Page(&'a Concept),
}

impl<'a> Origin<'a> {
    pub fn concept(&self) -> &'a Concept {
        match self {
            Origin::Name(c) | Origin::Page(c) => c,
        }
    }
}

/// A binding and where it comes from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bound<'a> {
    pub symbol: &'a str,
    pub name: &'a str,
    pub meaning: &'a str,
    pub relation: Relation,
    pub origin: Origin<'a>,
}

impl Bound<'_> {
    pub fn describe(&self) -> String {
        describe(self.symbol, self.name, self.meaning)
    }
}

/// The bindings in a concept's name.
pub fn of_name(c: &Concept) -> impl Iterator<Item = Bound<'_>> {
    c.symbols.iter().map(move |b| Bound {
        symbol: &b.symbol,
        name: &b.name,
        meaning: &b.meaning,
        relation: b.relation,
        origin: Origin::Name(c),
    })
}

/// The bindings a page's writer stated.
pub fn of_page<'a>(graph: &'a Graph, c: &'a Concept) -> impl Iterator<Item = Bound<'a>> {
    graph
        .bindings
        .get(&c.id)
        .into_iter()
        .flatten()
        .map(move |b| Bound {
            symbol: &b.symbol,
            name: &b.name,
            meaning: &b.meaning,
            relation: b.relation,
            origin: Origin::Page(c),
        })
}

/// The bindings the reader has on reaching page `i`: those in the names of the concepts
/// the reader knows ([`Plan::known_at`]) and those stated on the pages before `i`.
pub fn in_force<'a>(graph: &'a Graph, plan: &Plan, i: usize) -> Vec<Bound<'a>> {
    let known = plan.known_at(graph, i);
    let names = known.iter().flat_map(|c| of_name(c));
    let pages = plan.order[..i]
        .iter()
        .filter_map(|id| graph.get(id))
        .flat_map(|c| of_page(graph, c));
    names.chain(pages).collect()
}

/// Ids of the concepts `id` builds on, directly or through other concepts.
pub fn ancestors<'a>(graph: &'a Graph, id: &str) -> Vec<&'a Concept> {
    let mut found: Vec<&Concept> = Vec::new();
    let mut stack: Vec<&str> = graph
        .get(id)
        .map(|c| c.requires.iter().map(String::as_str).collect())
        .unwrap_or_default();
    while let Some(next) = stack.pop() {
        if let Some(c) = graph.get(next)
            && !found.iter().any(|f| f.id == c.id)
        {
            found.push(c);
            stack.extend(c.requires.iter().map(String::as_str));
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::tests::{concept, graph};

    fn binding(symbol: &str, quote: &str) -> PageBinding {
        PageBinding {
            symbol: symbol.into(),
            name: "record length".into(),
            meaning: "The number of complex coefficients in a record.".into(),
            relation: Relation::Count,
            quote: quote.into(),
        }
    }

    #[test]
    fn symbols_occur_as_whole_tokens() {
        let m = vec!["\\frac{1}{2} y^{r} + 1".to_string(), "T_A(n)".to_string()];
        assert!(occurs("r", &m));
        assert!(occurs("y^r", &m));
        assert!(occurs("T_{A}(n)", &m));
        assert!(!occurs("a", &m), "inside \\frac");
        assert!(!occurs("\\kappa", &m));
        assert!(!occurs("", &m));
        assert!(occurs("\\kappa", &["1 - \\kappa".into()]));
        assert!(same(" r ", "{r}") && !same("r", "q"));
    }

    #[test]
    fn page_bindings_need_the_symbol_in_a_quote_on_the_page() {
        let body = "Records hold $r$ complex coefficients. It is a power of two $r$. \
                    Take $r = 4$ <cite data-source=\"s\">x</cite>.";
        let mut ok = vec![binding("$r$", "Records hold $r$ complex coefficients.")];
        assert!(check_page(body, &mut ok).is_empty());
        assert_eq!(ok[0].symbol, "r");
        let mut bad = vec![
            binding("q", "Records hold"),
            binding("r", "Records hold"),
            binding("r", "not on the page"),
            binding("r", "It is a power of two $r$."),
        ];
        let errors = check_page(body, &mut bad);
        assert!(errors[0].contains("`q` does not occur"), "{errors:?}");
        assert!(errors[1].contains("must cover the symbol"), "{errors:?}");
        assert!(errors[2].contains("quote not found"), "{errors:?}");
        assert!(errors[3].contains("bound twice"), "{errors:?}");
        assert!(errors[4].contains("`r` is used before"), "{errors:?}");
    }

    #[test]
    fn quotes_may_cut_into_formulas() {
        let page = "Since $(s, t) \\mapsto (s + t, s - t)$ undoes it.\n\n$$c_j = a_k b_l$$\n";
        assert_eq!(quoted(page, "Since $(s, t)", "s"), Quoted::Found);
        assert_eq!(quoted(page, "c_j = a_k", "j"), Quoted::Found);
        assert_eq!(quoted(page, "c_j = a_k", "l"), Quoted::WithoutSymbol);
        assert_eq!(quoted(page, "undoes it", "s"), Quoted::WithoutSymbol);
        assert_eq!(quoted(page, "not here", "s"), Quoted::Missing);
        assert_eq!(quoted(page, "c_j = a_k", "a_k"), Quoted::Found);
        let late = "Take $r = 4$. Here $r$ is the record length.";
        assert_eq!(
            quoted(late, "Here $r$ is the record length.", "r"),
            Quoted::Late
        );
    }

    #[test]
    fn name_bindings_need_a_title_with_the_symbol() {
        let mut c = concept("ring", &[], false);
        c.symbols = vec![Binding {
            symbol: "r".into(),
            name: "record length".into(),
            meaning: "The number of coefficients.".into(),
            relation: Relation::Count,
        }];
        assert!(check_name(&mut c)[0].contains("no title"));
        c.title = Some("Polynomial ring modulo $y^q + 1$".into());
        assert!(check_name(&mut c)[0].contains("does not occur"));
        c.title = Some("Polynomial ring modulo $y^r + 1$".into());
        assert!(check_name(&mut c).is_empty());
    }

    #[test]
    fn in_force_has_known_names_and_earlier_pages() {
        let mut ring = concept("ring", &[], false);
        ring.title = Some("Ring mod $y^r + 1$".into());
        ring.symbols = vec![Binding {
            symbol: "r".into(),
            name: "record length".into(),
            meaning: "m".into(),
            relation: Relation::Count,
        }];
        let mut g = graph(
            "t",
            vec![
                ring,
                concept("dft", &[], false),
                concept("t", &["ring", "dft"], false),
            ],
        );
        g.bindings
            .insert("dft".into(), vec![binding("\\omega", "q")]);
        let plan = Plan {
            order: vec!["ring".into(), "dft".into(), "t".into()],
        };
        let symbols =
            |i| -> Vec<&str> { in_force(&g, &plan, i).iter().map(|b| b.symbol).collect() };
        assert!(symbols(0).is_empty());
        assert_eq!(symbols(1), vec!["r"]);
        assert_eq!(symbols(2), vec!["r", "\\omega"]);
        let ids: Vec<&str> = ancestors(&g, "t").iter().map(|c| c.id.as_str()).collect();
        assert_eq!(ids, vec!["dft", "ring"]);
    }
}

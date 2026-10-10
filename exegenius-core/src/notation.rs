//! Notation task: once the pages are final, list the symbols a reader has to keep track of
//! across pages, for the notation page.
//!
//! Code checks each symbol against the page the agent says introduces it: the symbol's
//! LaTeX must appear inside that page's mathematics (ignoring spacing), so every entry
//! points at a page where the reader can see the symbol in use.

use std::collections::{HashMap, HashSet};

use pulldown_cmark::{Event, Options, Parser};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result, read_json, write_json};
use crate::graph::Graph;
use crate::plan::Plan;
use crate::project::Project;

pub const NOTATION_INSTRUCTIONS: &str = "\
List the notation of an explanatory wiki for its notation page: the symbols a reader has to \
keep track of across pages.

- Include each symbol that is used on more than one page, or that stands for a quantity the \
argument depends on: sizes, parameters, exponents, constants, named maps and spaces. Leave \
out variables bound inside a single formula, and standard notation the reader already knows \
(such as $O(\\cdot)$ or $\\sum$).
- `symbol`: the LaTeX exactly as it appears in the mathematics of the page that introduces \
it, without dollar signs, e.g. `\\kappa` or `T_A(n)`. Submit rejects a symbol that does not \
appear inside the mathematics of that page (spacing is ignored).
- `meaning`: what it stands for, as a short phrase, with its value or range if the pages \
give one. LaTeX in $...$ is allowed.
- `page`: the id of the page that introduces the symbol.
- If a symbol means different things in different places, give one entry per meaning, each \
with the page that introduces it.
- If the wiki uses no such notation, answer with an empty list.";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NotationAnswer {
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Symbol {
    /// LaTeX, without dollar signs, as it appears on the page that introduces it.
    pub symbol: String,
    /// What the symbol stands for, as a short phrase.
    pub meaning: String,
    /// Id of the page that introduces the symbol.
    pub page: String,
}

/// The accepted notation, and the pages it was made for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notation {
    /// The planned pages the notation was made for, sorted.
    pub planned: Vec<String>,
    pub symbols: Vec<Symbol>,
}

fn sorted(plan: &Plan) -> Vec<String> {
    let mut pages = plan.order.clone();
    pages.sort();
    pages
}

/// The stored notation, if it was made for the pages of `plan`.
pub fn current(project: &Project, plan: &Plan) -> Result<Option<Notation>> {
    let path = project.notation_path();
    if !path.exists() {
        return Ok(None);
    }
    let notation: Notation = read_json(&path)?;
    Ok((notation.planned == sorted(plan)).then_some(notation))
}

pub fn notation_task(graph: &Graph, plan: &Plan) -> String {
    let mut task = String::from("## Pages, in reading order\n\n");
    for id in &plan.order {
        let c = graph.get(id).expect("planned pages are concepts");
        task.push_str(&format!("- `{}`: {}\n", c.id, c.name));
    }
    task.push_str(
        "\nRead the page bodies in `exegenius/pages/<id>.md`, in reading order, noting where \
         each symbol is first defined.\n",
    );
    task
}

fn compact(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// The mathematics of a page body, without spacing.
fn math(body: &str) -> String {
    Parser::new_ext(body, Options::ENABLE_MATH)
        .filter_map(|event| match event {
            Event::InlineMath(m) | Event::DisplayMath(m) => Some(compact(&m)),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Check a notation answer and store it.
pub fn apply_notation(project: &Project, plan: &Plan, answer: NotationAnswer) -> Result<Notation> {
    let mut maths: HashMap<&str, String> = HashMap::new();
    let mut seen = HashSet::new();
    let mut errors = Vec::new();
    for (i, s) in answer.symbols.iter().enumerate() {
        let n = i + 1;
        let symbol = s.symbol.trim().trim_matches('$').trim();
        if symbol.is_empty() {
            errors.push(format!("symbol {n} is empty"));
            continue;
        }
        if s.meaning.trim().is_empty() {
            errors.push(format!("`{symbol}` has no meaning"));
        }
        if !plan.contains(&s.page) {
            errors.push(format!(
                "`{symbol}`: `{}` is not a page of this wiki",
                s.page
            ));
            continue;
        }
        if !seen.insert((compact(symbol), s.page.clone())) {
            errors.push(format!("`{symbol}` is listed twice for `{}`", s.page));
            continue;
        }
        if !maths.contains_key(s.page.as_str()) {
            let body = project.page_body(&s.page)?.unwrap_or_default();
            maths.insert(&s.page, math(&body));
        }
        if !maths[s.page.as_str()].contains(&compact(symbol)) {
            errors.push(format!(
                "`{symbol}` does not appear in the mathematics of `{}`; give the LaTeX as \
                 written there, or the page that introduces it",
                s.page
            ));
        }
    }
    if !errors.is_empty() {
        return Err(Error::Invalid(format!(
            "answer rejected:\n{}",
            errors
                .iter()
                .map(|e| format!("- {e}"))
                .collect::<Vec<_>>()
                .join("\n")
        )));
    }
    let symbols = answer
        .symbols
        .into_iter()
        .map(|s| Symbol {
            symbol: s.symbol.trim().trim_matches('$').trim().to_string(),
            meaning: s.meaning.trim().to_string(),
            page: s.page,
        })
        .collect();
    let notation = Notation {
        planned: sorted(plan),
        symbols,
    };
    write_json(&project.notation_path(), &notation)?;
    Ok(notation)
}

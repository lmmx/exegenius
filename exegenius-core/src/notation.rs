//! Notation across pages: the symbols a page inherits, the notation page, and letters bound
//! to two meanings.
//!
//! Everything here is derived from the bindings (see [`crate::symbols`]) and from each
//! page's latest audit, which says where the reader learns what each symbol on the page
//! denotes. No agent lists the notation:
//!
//! - **Inherited symbols.** The symbols an audit anchored to an earlier page or to a known
//!   concept's name are shown on the page with their display name and meaning.
//! - **The notation page** lists the bindings that other pages use, each with the page or
//!   concept name that binds it and the pages that use it.
//! - **Overloads.** A page that binds a letter which a page or concept name it builds on
//!   binds too is a candidate for one letter with two meanings. Whether the meanings differ
//!   is a judgement, so the candidates go to an overloads task; a confirmed overload
//!   becomes a rewrite note asking the later page to rename the letter or say that it
//!   changes meaning.

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::audit::{Anchor, SymbolUse};
use crate::error::{Result, read_json, write_json};
use crate::graph::{Concept, Graph};
use crate::plan::Plan;
use crate::project::Project;
use crate::symbols::{self, Bound, Origin};
use crate::text::key;
use crate::write::link;

/// The binding a page's symbol use is anchored to, if it is still there.
fn anchored<'a>(graph: &'a Graph, u: &SymbolUse) -> Option<Bound<'a>> {
    let found = |mut bindings: Box<dyn Iterator<Item = Bound<'a>> + 'a>| {
        bindings.find(|b| symbols::same(b.symbol, &u.symbol))
    };
    match &u.anchor {
        Anchor::Page { page } => {
            let c = graph.get(page)?;
            found(Box::new(symbols::of_page(graph, c)))
        }
        Anchor::Concept { concept } => found(Box::new(symbols::of_name(graph.get(concept)?))),
        _ => None,
    }
}

/// The symbols a page takes from earlier pages and concept names, per its latest audit,
/// leaving out any no longer in the page's mathematics.
pub fn inherited<'a>(graph: &'a Graph, uses: &[SymbolUse], body: &str) -> Vec<Bound<'a>> {
    let page_math = symbols::math(&crate::cite::strip(body));
    uses.iter()
        .filter(|u| symbols::occurs(&u.symbol, &page_math))
        .filter_map(|u| anchored(graph, u))
        .collect()
}

/// One entry of the notation page: a binding and the other pages that use it.
pub struct Entry<'a> {
    pub bound: Bound<'a>,
    pub users: Vec<&'a Concept>,
}

/// The bindings that pages other than their own use, in the order they are bound: known
/// concepts' names first, then the pages in reading order.
pub fn entries<'a>(
    graph: &'a Graph,
    plan: &Plan,
    uses: &HashMap<String, Vec<SymbolUse>>,
) -> Vec<Entry<'a>> {
    let pages: Vec<&Concept> = plan.order.iter().filter_map(|id| graph.get(id)).collect();
    let known = graph.concepts.iter().filter(|c| c.known);
    let mut entries: Vec<Entry> = known
        .flat_map(symbols::of_name)
        .chain(
            pages
                .iter()
                .flat_map(|c| symbols::of_name(c).chain(symbols::of_page(graph, c))),
        )
        .map(|bound| Entry {
            bound,
            users: Vec::new(),
        })
        .collect();
    for page in &pages {
        for u in uses.get(&page.id).into_iter().flatten() {
            let Some(bound) = anchored(graph, u) else {
                continue;
            };
            if let Some(entry) = entries.iter_mut().find(|e| e.bound == bound)
                && !entry.users.iter().any(|p| p.id == page.id)
            {
                entry.users.push(page);
            }
        }
    }
    entries.retain(|e| !e.users.is_empty());
    entries
}

pub const OVERLOADS_INSTRUCTIONS: &str = "\
Check the notation of an explanatory wiki for letters used with two meanings. Each candidate \
below is a symbol that a page binds, and that a page or concept name it builds on also binds. \
A reader arriving from the earlier page brings the earlier meaning with them.

- For each candidate, say whether the two bindings denote the same thing (`same: true`), \
even if worded differently (\"number of digits\" and \"digit count\"), or different things \
(`same: false`, as for a radix and a digit count).
- For different things, say in `note` what the page that changes should do: use another \
letter, or say where the letter is used that it means something else there than on the \
other page. For the same thing, `note` may be empty.
- `change` is the page that changes. Leave it out for the later page. Give the earlier page's \
id when its use of the letter is the odd one out, for instance when the sources use the \
letter as the later page does, and the earlier page binds it on the page, not in a name. A \
page that other pages take the letter from keeps it and says where its meaning changes.
- Read the pages in `exegenius/pages/<id>.md` where the bindings alone do not settle it.
- Give a verdict for every candidate.";

/// A symbol that a page binds and that a page or concept name it builds on binds too.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Candidate {
    pub symbol: String,
    /// The page that binds the symbol again.
    pub page: String,
    pub name: String,
    pub meaning: String,
    /// The concept whose page, or name, binds it first.
    pub earlier: String,
    /// Whether the earlier binding is in the concept's name rather than on its page.
    pub in_name: bool,
    pub earlier_name: String,
    pub earlier_meaning: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Verdict {
    /// Number of the candidate, as listed.
    pub candidate: usize,
    /// True if the two bindings denote the same thing.
    pub same: bool,
    /// For different things: what the page that changes should do.
    pub note: String,
    /// For different things: the page that changes, the later page if left out.
    #[serde(default)]
    pub change: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OverloadsAnswer {
    pub verdicts: Vec<Verdict>,
}

/// A candidate and the verdict on it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Judged {
    pub candidate: Candidate,
    pub same: bool,
    pub note: String,
    /// The page the note was queued on, for different things.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed: Option<String>,
}

/// Every verdict so far, kept so a candidate is judged once.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Overloads {
    pub judged: Vec<Judged>,
    /// Overloads tasks applied so far. A rewrite that rewords a meaning makes a new
    /// candidate, so verdicts could queue rewrites without end; after [`ROUNDS`], candidates
    /// go unjudged.
    #[serde(default)]
    pub rounds: u32,
}

/// Most overloads tasks per wiki, as support checks are limited per page.
pub const ROUNDS: u32 = 2;

/// Overload candidates for the planned pages, in reading order. Pairs whose display names
/// or meanings are the same words are left out.
pub fn candidates(graph: &Graph, plan: &Plan) -> Vec<Candidate> {
    let mut found = Vec::new();
    for page in plan.order.iter().filter_map(|id| graph.get(id)) {
        let ancestors = symbols::ancestors(graph, &page.id);
        let earlier: Vec<Bound> = symbols::of_name(page)
            .chain(ancestors.iter().flat_map(|a| {
                let on_page = plan.contains(&a.id).then(|| symbols::of_page(graph, a));
                symbols::of_name(a).chain(on_page.into_iter().flatten())
            }))
            .collect();
        for b in symbols::of_page(graph, page) {
            for e in earlier.iter().filter(|e| symbols::same(e.symbol, b.symbol)) {
                if key(e.name) == key(b.name) || key(e.meaning) == key(b.meaning) {
                    continue;
                }
                found.push(Candidate {
                    symbol: b.symbol.to_string(),
                    page: page.id.clone(),
                    name: b.name.to_string(),
                    meaning: b.meaning.to_string(),
                    earlier: e.origin.concept().id.clone(),
                    in_name: matches!(e.origin, Origin::Name(_)),
                    earlier_name: e.name.to_string(),
                    earlier_meaning: e.meaning.to_string(),
                });
            }
        }
    }
    found
}

pub fn load(project: &Project) -> Result<Overloads> {
    let path = project.overloads_path();
    if path.exists() {
        read_json(&path)
    } else {
        Ok(Overloads::default())
    }
}

/// The candidates with no verdict yet.
pub fn unjudged(project: &Project, graph: &Graph, plan: &Plan) -> Result<Vec<Candidate>> {
    let overloads = load(project)?;
    Ok(candidates(graph, plan)
        .into_iter()
        .filter(|c| !overloads.judged.iter().any(|j| j.candidate == *c))
        .collect())
}

/// Where a candidate's earlier binding is, as a phrase.
fn where_bound(graph: &Graph, c: &Candidate) -> String {
    let earlier = graph.get(&c.earlier);
    let name = earlier.map_or(c.earlier.clone(), |e| e.name.clone());
    match (c.in_name, c.earlier == c.page) {
        (true, true) => "in this page's heading".to_string(),
        (true, false) => format!("in the name of {name}"),
        (false, _) => format!("on {}", earlier.map_or(name, link)),
    }
}

pub fn overloads_task(graph: &Graph, candidates: &[Candidate]) -> String {
    let mut task = String::from("## Candidates\n\n");
    for (i, c) in candidates.iter().enumerate() {
        task.push_str(&format!(
            "{}. ${}$\n   - on `{}`: {}\n   - {}: {}\n",
            i + 1,
            c.symbol,
            c.page,
            symbols::describe(&c.symbol, &c.name, &c.meaning),
            where_bound(graph, c),
            symbols::describe(&c.symbol, &c.earlier_name, &c.earlier_meaning),
        ));
    }
    task
}

/// Check the verdicts on `candidates`, queue a rewrite for each letter with two meanings,
/// and store the verdicts.
pub fn apply_overloads(
    project: &Project,
    graph: &mut Graph,
    candidates: &[Candidate],
    answer: OverloadsAnswer,
) -> Result<()> {
    let mut errors = Vec::new();
    let mut verdicts: Vec<Option<&Verdict>> = vec![None; candidates.len()];
    for v in &answer.verdicts {
        match verdicts.get_mut(v.candidate.wrapping_sub(1)) {
            None => errors.push(format!("candidate {} does not exist", v.candidate)),
            Some(Some(_)) => errors.push(format!("candidate {} is judged twice", v.candidate)),
            Some(slot) => {
                if !v.same && v.note.trim().is_empty() {
                    errors.push(format!(
                        "candidate {}: say what the page that changes should do",
                        v.candidate
                    ));
                }
                if let (Some(c), Some(change)) =
                    (candidates.get(v.candidate.wrapping_sub(1)), &v.change)
                    && *change != c.page
                    && !(*change == c.earlier && !c.in_name)
                {
                    errors.push(match c.in_name {
                        true => format!(
                            "candidate {}: only `{}` can change, as the earlier binding is in \
                             a name",
                            v.candidate, c.page
                        ),
                        false => format!(
                            "candidate {}: `change` is `{}` or `{}`",
                            v.candidate, c.page, c.earlier
                        ),
                    });
                }
                *slot = Some(v);
            }
        }
    }
    for (i, v) in verdicts.iter().enumerate() {
        if v.is_none() {
            errors.push(format!("no verdict on candidate {}", i + 1));
        }
    }
    if !errors.is_empty() {
        return Err(crate::simulate::rejected(errors));
    }
    let mut overloads = load(project)?;
    for (c, v) in candidates.iter().zip(verdicts) {
        let v = v.expect("every candidate has a verdict");
        let mut changed = None;
        if !v.same {
            let builds_on = if c.earlier == c.page {
                ""
            } else {
                ", which this page builds on"
            };
            let (page, note) = match &v.change {
                Some(earlier) if *earlier != c.page => (
                    earlier.clone(),
                    format!(
                        "${}$ means {} here, but {} on {}, which builds on this page: {}",
                        c.symbol,
                        c.earlier_name,
                        c.name,
                        graph.get(&c.page).map_or(c.page.clone(), link),
                        v.note.trim()
                    ),
                ),
                _ => (
                    c.page.clone(),
                    format!(
                        "${}$ means {} here, but {} {}{builds_on}: {}",
                        c.symbol,
                        c.name,
                        c.earlier_name,
                        where_bound(graph, c),
                        v.note.trim()
                    ),
                ),
            };
            graph.queue_rewrite(&page, Some(note));
            changed = Some(page);
        }
        overloads.judged.push(Judged {
            candidate: c.clone(),
            same: v.same,
            note: v.note.trim().to_string(),
            changed,
        });
    }
    overloads.rounds += 1;
    write_json(&project.overloads_path(), &overloads)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::tests::{concept, graph};
    use crate::symbols::{Binding, PageBinding, Relation};

    fn stated(symbol: &str, name: &str) -> PageBinding {
        PageBinding {
            symbol: symbol.into(),
            name: name.into(),
            meaning: format!("The {name}."),
            relation: Relation::Count,
            quote: String::new(),
        }
    }

    fn fixture() -> (Graph, Plan) {
        let mut ring = concept("ring", &[], false);
        ring.title = Some("Ring modulo $y^r + 1$".into());
        ring.symbols = vec![Binding {
            symbol: "r".into(),
            name: "record length".into(),
            meaning: "The record length.".into(),
            relation: Relation::Count,
        }];
        let mut g = graph(
            "conv",
            vec![
                ring,
                concept("machine", &[], false),
                concept("conv", &["ring", "machine"], false),
                concept("other", &[], false),
            ],
        );
        g.bindings
            .insert("machine".into(), vec![stated("q", "radix")]);
        g.bindings.insert(
            "conv".into(),
            vec![stated("q", "digit count"), stated("r", "Record length")],
        );
        g.bindings
            .insert("other".into(), vec![stated("q", "queue size")]);
        let plan = Plan {
            order: vec![
                "ring".into(),
                "machine".into(),
                "other".into(),
                "conv".into(),
            ],
        };
        (g, plan)
    }

    #[test]
    fn candidates_are_rebindings_along_prerequisites() {
        let (g, plan) = fixture();
        let found = candidates(&g, &plan);
        // `other` binds q too, but `conv` does not build on it; r has the same name.
        assert_eq!(found.len(), 1);
        assert_eq!(
            (found[0].page.as_str(), found[0].earlier.as_str()),
            ("conv", "machine")
        );
        assert!(!found[0].in_name);
        let task = overloads_task(&g, &found);
        assert!(task.contains("$q$ (digit count)") && task.contains("on [machine](machine.md)"));
    }

    #[test]
    fn entries_and_inherited_follow_the_audits() {
        let (g, plan) = fixture();
        let uses: HashMap<String, Vec<SymbolUse>> = [(
            "conv".to_string(),
            vec![
                SymbolUse {
                    symbol: "r".into(),
                    anchor: Anchor::Concept {
                        concept: "ring".into(),
                    },
                },
                SymbolUse {
                    symbol: "q".into(),
                    anchor: Anchor::Page {
                        page: "machine".into(),
                    },
                },
            ],
        )]
        .into();
        let found = entries(&g, &plan, &uses);
        let symbols: Vec<&str> = found.iter().map(|e| e.bound.symbol).collect();
        assert_eq!(symbols, vec!["r", "q"]);
        assert_eq!(found[0].users[0].id, "conv");
        let shown = inherited(&g, &uses["conv"], "Modulo $y^r + 1$ only.");
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0].name, "record length");
    }
}

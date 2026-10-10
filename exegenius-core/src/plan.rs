//! Step 2: the pages needed to reach the target, in reading order.
//!
//! The needed pages are the prerequisite closure of the target, stopping at concepts the
//! reader already knows. The order is a topological sort of that closure; ties go to the
//! concept whose first supporting quote comes earliest in the sources, so the order follows
//! the sources wherever the graph leaves it free.

use std::collections::{BTreeSet, HashMap, HashSet};

use crate::error::{Error, Result};
use crate::graph::{Concept, Graph};
use crate::project::Source;
use crate::text::normalise;

#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    /// Ids of the concepts that get a page, in reading order (the target last).
    pub order: Vec<String>,
}

impl Plan {
    pub fn contains(&self, id: &str) -> bool {
        self.order.iter().any(|o| o == id)
    }

    /// What the reader knows on reaching page `i`: the known concepts and every page
    /// before `i` in reading order.
    pub fn known_at<'a>(&self, graph: &'a Graph, i: usize) -> Vec<&'a Concept> {
        let read: HashSet<&str> = self.order[..i].iter().map(String::as_str).collect();
        graph
            .concepts
            .iter()
            .filter(|c| c.known || read.contains(c.id.as_str()))
            .collect()
    }
}

/// Position of a concept's first supporting quote: (source index, byte offset).
fn first_quote(c: &Concept, sources: &[Source]) -> (usize, usize) {
    c.evidence
        .iter()
        .filter_map(|e| {
            let (index, source) = sources.iter().enumerate().find(|(_, s)| s.id == e.source)?;
            let offset = source.normalised.find(&normalise(&e.quote))?;
            Some((index, offset))
        })
        .min()
        .unwrap_or((usize::MAX, usize::MAX))
}

pub fn plan(graph: &Graph, sources: &[Source]) -> Result<Plan> {
    let target = graph
        .get(&graph.target)
        .ok_or_else(|| Error::Invalid(format!("target `{}` is not a concept id", graph.target)))?;
    if target.known {
        return Err(Error::Invalid(format!(
            "target `{}` is marked as known to the reader, so there is nothing to explain",
            target.id
        )));
    }
    let position: HashMap<&str, usize> = graph
        .concepts
        .iter()
        .enumerate()
        .map(|(i, c)| (c.id.as_str(), i))
        .collect();

    let mut closure: Vec<&Concept> = Vec::new();
    let mut seen = HashSet::new();
    let mut stack = vec![target];
    while let Some(c) = stack.pop() {
        if c.known || !seen.insert(c.id.as_str()) {
            continue;
        }
        closure.push(c);
        stack.extend(c.requires.iter().filter_map(|r| graph.get(r)));
    }
    let members: HashSet<&str> = closure.iter().map(|c| c.id.as_str()).collect();

    let sort_key = |c: &Concept| {
        let (source, offset) = first_quote(c, sources);
        (source, offset, position[c.id.as_str()])
    };
    let mut pending: HashMap<&str, usize> = HashMap::new();
    let mut dependents: HashMap<&str, Vec<&Concept>> = HashMap::new();
    for c in &closure {
        let inside: Vec<&str> = c
            .requires
            .iter()
            .map(String::as_str)
            .filter(|r| members.contains(r))
            .collect();
        pending.insert(c.id.as_str(), inside.len());
        for r in inside {
            dependents.entry(r).or_default().push(c);
        }
    }
    let mut ready: BTreeSet<((usize, usize, usize), &str)> = closure
        .iter()
        .filter(|c| pending[c.id.as_str()] == 0)
        .map(|c| (sort_key(c), c.id.as_str()))
        .collect();
    let mut order = Vec::with_capacity(closure.len());
    while let Some(next) = ready.pop_first() {
        let id = next.1;
        order.push(id.to_string());
        for d in dependents.get(id).into_iter().flatten() {
            let count = pending
                .get_mut(d.id.as_str())
                .expect("dependent is in the closure");
            *count -= 1;
            if *count == 0 {
                ready.insert((sort_key(d), d.id.as_str()));
            }
        }
    }
    if order.len() < closure.len() {
        let mut stuck: Vec<&str> = members
            .iter()
            .copied()
            .filter(|m| !order.iter().any(|o| o == m))
            .collect();
        stuck.sort_unstable();
        return Err(Error::Invalid(format!(
            "prerequisite cycle among: {}",
            stuck.join(", ")
        )));
    }
    Ok(Plan { order })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Evidence;
    use crate::graph::tests::{concept, graph};

    fn with_quote(mut c: Concept, quote: &str) -> Concept {
        c.evidence.push(Evidence {
            source: "s".into(),
            quote: quote.into(),
        });
        c
    }

    #[test]
    fn closure_stops_at_known_and_follows_source_order() {
        let sources = vec![Source::new(
            "s",
            None,
            None,
            "alpha beta gamma delta".into(),
        )];
        let g = graph(
            "t",
            vec![
                with_quote(concept("t", &["late", "early"], false), "delta"),
                with_quote(concept("late", &["k"], false), "gamma"),
                with_quote(concept("early", &["k"], false), "alpha"),
                concept("k", &["unreached"], true),
                concept("unreached", &[], false),
                concept("unrelated", &[], false),
            ],
        );
        let plan = plan(&g, &sources).unwrap();
        assert_eq!(plan.order, vec!["early", "late", "t"]);
        let known: Vec<&str> = plan.known_at(&g, 1).iter().map(|c| c.id.as_str()).collect();
        assert_eq!(known, vec!["early", "k"]);
    }

    #[test]
    fn known_target_is_an_error() {
        let g = graph("t", vec![concept("t", &[], true)]);
        assert!(plan(&g, &[]).is_err());
    }
}

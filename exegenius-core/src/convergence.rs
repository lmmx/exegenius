//! Whether the audit loop is converging.
//!
//! Each audit round reports gaps, and rewrites and extend act on them; the loop is meant to
//! reach a wiki with none. Two figures per round say whether it is getting there: the gaps
//! per audited page, by kind, and, on the pages also audited the round before, how many of
//! this round's gaps are new. A rewrite that fixes what it was told about and adds nothing
//! leaves few new gaps; one that replaces each reported step with another leaves as many
//! new gaps as it removed.

use crate::audit::{GapKind, Outcome, PageAudit, Round};
use crate::text::normalise;

/// Gap counts by kind.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Kinds {
    pub term: usize,
    pub step: usize,
    pub symbol: usize,
}

impl Kinds {
    fn add(&mut self, kind: GapKind) {
        match kind {
            GapKind::Term => self.term += 1,
            GapKind::Step => self.step += 1,
            GapKind::Symbol => self.symbol += 1,
        }
    }

    fn of(gaps: &[Outcome]) -> Kinds {
        let mut kinds = Kinds::default();
        for g in gaps {
            kinds.add(g.kind);
        }
        kinds
    }

    pub fn total(&self) -> usize {
        self.term + self.step + self.symbol
    }
}

/// One audit round, and how it compares with the round before on the pages both audited.
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    pub round: u32,
    pub pages: usize,
    pub gaps: Kinds,
    /// Pages audited in this round and the one before.
    pub repeated: usize,
    /// Gaps on those pages in the round before.
    pub before: Kinds,
    /// Gaps on those pages in this round.
    pub after: Kinds,
    /// Of `after`, the gaps whose passage was not reported on the page the round before.
    pub new: Kinds,
}

/// Whether two reported passages are the same: equal once normalised, or one inside the
/// other when the shorter is long enough not to match by chance.
fn same_passage(a: &str, b: &str) -> bool {
    let (a, b) = (normalise(a), normalise(b));
    let (short, long) = if a.len() <= b.len() {
        (&a, &b)
    } else {
        (&b, &a)
    };
    short == long || (short.len() >= 20 && long.contains(short.as_str()))
}

fn reported_before(gap: &Outcome, before: &PageAudit) -> bool {
    before
        .gaps
        .iter()
        .any(|b| b.kind == gap.kind && same_passage(&b.quote, &gap.quote))
}

/// A summary of every round, in round order.
pub fn summarise(rounds: &[Round]) -> Vec<Summary> {
    let mut rounds: Vec<&Round> = rounds.iter().collect();
    rounds.sort_by_key(|r| r.round);
    let mut out = Vec::new();
    let mut previous: Option<&Round> = None;
    for round in rounds {
        let mut summary = Summary {
            round: round.round,
            pages: round.pages.len(),
            gaps: Kinds::default(),
            repeated: 0,
            before: Kinds::default(),
            after: Kinds::default(),
            new: Kinds::default(),
        };
        for page in &round.pages {
            let gaps = Kinds::of(&page.gaps);
            summary.gaps.term += gaps.term;
            summary.gaps.step += gaps.step;
            summary.gaps.symbol += gaps.symbol;
            let Some(before) = previous.and_then(|p| p.pages.iter().find(|b| b.page == page.page))
            else {
                continue;
            };
            summary.repeated += 1;
            let earlier = Kinds::of(&before.gaps);
            summary.before.term += earlier.term;
            summary.before.step += earlier.step;
            summary.before.symbol += earlier.symbol;
            for gap in &page.gaps {
                summary.after.add(gap.kind);
                if !reported_before(gap, before) {
                    summary.new.add(gap.kind);
                }
            }
        }
        out.push(summary);
        previous = Some(round);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gap(kind: GapKind, quote: &str) -> Outcome {
        Outcome {
            term: String::new(),
            kind,
            quote: quote.to_string(),
            note: String::new(),
            action: String::new(),
        }
    }

    fn page(id: &str, gaps: Vec<Outcome>) -> PageAudit {
        PageAudit {
            page: id.to_string(),
            gaps,
            dropped: Vec::new(),
            symbols: Vec::new(),
        }
    }

    #[test]
    fn repeated_pages_split_gaps_into_old_and_new() {
        let rounds = vec![
            Round {
                round: 2,
                pages: vec![
                    page(
                        "a",
                        vec![
                            gap(GapKind::Step, "so the cost is O(n log n) overall"),
                            gap(GapKind::Term, "the tape"),
                        ],
                    ),
                    page("c", vec![gap(GapKind::Step, "a new page's step")]),
                ],
            },
            Round {
                round: 1,
                pages: vec![
                    page(
                        "a",
                        vec![
                            gap(GapKind::Step, "the cost is O(n log n)"),
                            gap(GapKind::Step, "and the error stays below one"),
                            gap(GapKind::Term, "the tape"),
                        ],
                    ),
                    page("b", vec![gap(GapKind::Symbol, "$r$")]),
                ],
            },
        ];
        let summary = summarise(&rounds);
        assert_eq!(summary.len(), 2);
        assert_eq!(summary[0].round, 1);
        assert_eq!(summary[0].repeated, 0);
        assert_eq!(
            summary[0].gaps,
            Kinds {
                term: 1,
                step: 2,
                symbol: 1
            }
        );
        let second = &summary[1];
        assert_eq!((second.round, second.pages, second.repeated), (2, 2, 1));
        assert_eq!(
            second.before,
            Kinds {
                term: 1,
                step: 2,
                symbol: 0
            }
        );
        assert_eq!(
            second.after,
            Kinds {
                term: 1,
                step: 1,
                symbol: 0
            }
        );
        // The step's passage contains the earlier one, which is long enough to count; the
        // short term matches only exactly, which it does.
        assert_eq!(second.new, Kinds::default());
    }

    #[test]
    fn short_passages_match_only_exactly() {
        assert!(same_passage("$r$", "$r$"));
        assert!(!same_passage("$r$", "take $r = 4$ here"));
        assert!(same_passage(
            "the cost is O(n log n)",
            "so the cost is O(n log n) overall"
        ));
    }
}

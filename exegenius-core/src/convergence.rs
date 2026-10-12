//! Whether the audit loop is converging.
//!
//! Each audit round reports gaps, and rewrites and extend act on them; the loop is meant to
//! reach a wiki with none. Two figures per round say whether it is getting there: the gaps
//! per audited page, by kind, and, on the pages also audited the round before, how many of
//! this round's gaps are new. A rewrite that fixes what it was told about and adds nothing
//! leaves few new gaps; one that replaces each reported step with another leaves as many
//! new gaps as it removed.

use crate::audit::{CITE_OR_CUT, GapKind, Outcome, PageAudit, Round};
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
    /// Of `after`, the gaps not reported on the page the round before: for a symbol, the
    /// same symbol; for a term or step, the same passage.
    pub new: Kinds,
    /// Of `after`, the steps inside reasoning a rewrite added, sent back to be cited or cut.
    pub cite_or_cut: usize,
}

/// Why the audit rounds should end after `summary`'s round, if they should: on the pages
/// audited in this round and the one before, rewrites did not at least halve the step and
/// symbol gaps, the kinds a rewrite is meant to fix. Term gaps are left out, as extend
/// resolves them into pages. The 0.1 run's rounds halved them (28 to 10, 16 to 2); the
/// 0.5.0 run's and the baseline pilot's did not (202 to 282, 34 to 34).
///
/// Steps sent back to be cited or cut are left out too: their fix adds no claim, so they
/// cannot renew, and only a further round fixes them. Counted, they end the rounds just
/// when there are many of them to fix.
pub fn stalled(summary: &Summary) -> Option<String> {
    let before = summary.before.step + summary.before.symbol;
    let after = summary.after.step + summary.after.symbol - summary.cite_or_cut;
    (summary.repeated > 0 && before > 0 && after * 2 > before).then(|| {
        format!(
            "after audit round {}, step and symbol gaps on the {} pages also audited in round {} \
             went from {before} to {after}, not to half or fewer, so the rounds stopped; \
             the gaps may still have been falling{}",
            summary.round,
            summary.repeated,
            summary.round - 1,
            match summary.cite_or_cut {
                0 => String::new(),
                n => format!(" ({n} steps sent back to be cited or cut not counted)"),
            }
        )
    })
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

/// Whether `gap` was reported on the page the round before. A symbol gap is the same gap
/// whatever sentence the auditor quoted, since rewording the sentence does not explain the
/// symbol.
fn reported_before(gap: &Outcome, before: &PageAudit) -> bool {
    before.gaps.iter().any(|b| {
        b.kind == gap.kind
            && match gap.kind {
                GapKind::Symbol => crate::symbols::same(&b.term, &gap.term),
                GapKind::Term | GapKind::Step => same_passage(&b.quote, &gap.quote),
            }
    })
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
            cite_or_cut: 0,
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
                if gap.action == CITE_OR_CUT {
                    summary.cite_or_cut += 1;
                }
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
                    page(
                        "c",
                        vec![Outcome {
                            action: CITE_OR_CUT.into(),
                            ..gap(GapKind::Step, "a new page's step")
                        }],
                    ),
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
        // Page c was not audited before, so its step is not on a repeated page.
        assert_eq!(second.cite_or_cut, 0);
    }

    #[test]
    fn rounds_stall_when_rewrites_do_not_halve_steps_and_symbols() {
        let mut summary = Summary {
            round: 2,
            pages: 7,
            gaps: Kinds::default(),
            repeated: 7,
            before: Kinds {
                term: 12,
                step: 23,
                symbol: 11,
            },
            after: Kinds {
                term: 6,
                step: 21,
                symbol: 13,
            },
            new: Kinds::default(),
            cite_or_cut: 0,
        };
        assert!(stalled(&summary).is_some());
        // The 0.1 run's first comparison: symbols held at 8, but steps fell from 20 to 2.
        summary.before = Kinds {
            term: 16,
            step: 20,
            symbol: 8,
        };
        summary.after = Kinds {
            term: 5,
            step: 2,
            symbol: 8,
        };
        assert!(stalled(&summary).is_none());
        // The 0.6 full run's: 49 to 31, but 21 of the 22 steps were sent back to be cited
        // or cut, leaving 10.
        summary.before = Kinds {
            term: 26,
            step: 35,
            symbol: 14,
        };
        summary.after = Kinds {
            term: 12,
            step: 22,
            symbol: 9,
        };
        assert!(stalled(&summary).is_some());
        summary.cite_or_cut = 21;
        assert!(stalled(&summary).is_none());
        summary.repeated = 0;
        summary.after.step = 40;
        assert!(stalled(&summary).is_none());
    }

    #[test]
    fn symbol_gaps_match_by_symbol() {
        let symbol = |term: &str, quote: &str| Outcome {
            term: term.into(),
            ..gap(GapKind::Symbol, quote)
        };
        let rounds = vec![
            Round {
                round: 1,
                pages: vec![page("a", vec![symbol("s", "where $s$ is the length")])],
            },
            Round {
                round: 2,
                pages: vec![page(
                    "a",
                    vec![
                        symbol("{s}", "with $s$ the number of coordinates"),
                        symbol("t", "where $s$ is the length"),
                    ],
                )],
            },
        ];
        let second = &summarise(&rounds)[1];
        assert_eq!((second.after.symbol, second.new.symbol), (2, 1));
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

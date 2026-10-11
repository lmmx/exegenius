# 2026-10-11: A divergent run

The author's run of 0.5.0 on the n log n paper, on Haiku, did not converge. This entry records its state as `exegenius status` reported it after six hours and 855 tasks issued, compares it with the author's run of 0.1 on the same paper, and records a second loop found while reading the state: overload verdicts have no limit on their rounds. 2026-10-11-graph-growth covers the first two audit rounds and the change to the audit and extend instructions; 2026-10-11-wall-time covers time and tokens.

## The state

`exegenius status` in the 0.5.0 run, with tasks 0815 to 0855 pending:

```
concepts: 157 (5 known to the reader); pages: 142 written of 142 planned
audit round 1: 293 gaps over 32 pages
audit round 2: 698 gaps over 82 pages
audit round 3: 836 gaps over 119 pages
candidate concepts: 0; pages queued for rewriting: 41
outline: 8 parts, 15 pages in the glossary
relations: 381 of 381 prerequisite edges between pages
symbols: 912 bound on pages; letters with two meanings: 1699 confirmed, 661 awaiting a verdict
citation support: 467 marks judged: 202 supported, 185 partial, 79 unsupported, 1 contradicted
  a second judge agreed on 1 of 2 marks
```

Claude Code's session totals for the run were 6h 22m of wall time, 24h 41m of API time, 1.0b cache-read tokens and 20.5m output tokens on Haiku.

## Against the 0.1 run

| | 0.1 run | 0.5.0 run |
|---|---|---|
| target, reader profile, `rounds` | same | same |
| pages at the end | 45 | 142 |
| tasks | 198 | 855 issued |
| round 1 | 44 gaps over 24 pages (1.8 per page) | 293 gaps over 32 pages (9.2 per page) |
| round 2 | 25 gaps over 36 pages (0.7 per page) | 698 gaps over 82 pages (8.5 per page) |
| round 3 | 4 gaps over 43 pages (0.1 per page) | 836 gaps over 119 pages (7.0 per page) |

The figures:
- **Gap rates.** In the 0.1 run, gaps per audited page fell by more than half each round. In the 0.5.0 run they fell by 8% and then 18%.
- **Growth.** In 0.5.0 the pages audited per round grew by more than the rate fell, so total gaps grew.
- **Starting point.** The gap is there from round 1, before any growth: the first 32 pages already drew five times as many gaps per page as the first 24 pages of 0.1.

The rates are not directly comparable, because 0.5 audits count more kinds of problem:
- **Symbols.** Every symbol in a page's mathematics must be accounted for. A symbol whose meaning comes after its first use counts as a gap (2026-10-11-symbol-bindings).
- **Relations.** Writes are rejected without a relation sentence for every prerequisite.

The split of the 0.5.0 gaps by kind, from that run's `exegenius/audit.json`, is not yet known. The split decides between the explanations below.

## Explanations not yet separated

1. **Longer writer obligations.** Since 0.1, writers have been asked for motivations, striking features, relation sentences and symbol meanings. Pages carry more mathematics and more named concepts, so each page offers more to report. This would show as more term and step gaps per page in round 1.
2. **Stricter symbol accounting.** The completeness floor and the late-meaning rule turn symbols an 0.1 auditor let pass into gaps. This would show as symbol gaps dominating, and would raise the rate without growing the graph, since symbol gaps become rewrite notes, not concepts.
3. **The model.** Every task ran on Haiku. A smaller model meeting more simultaneous constraints per page may leave more of them unmet, as a writer, and may report more, as an auditor. Auditing a sample of the same pages with a larger model, and rewriting a sample with one, would separate writer from auditor.
4. **The audit wording.** It counts as unknown what the profile does not name (2026-10-11-graph-growth). The 0.1 run used the same wording and converged, so on its own the wording does not explain the difference.

## Overload verdicts loop without a limit

When the pages are final, the overloads task judges each letter a page binds that an ancestor bound with another meaning. A verdict of "different" queues a rewrite (exegenius-core/src/notation.rs:275-289). Candidates are matched with earlier verdicts on every field, the meaning text included (exegenius-core/src/notation.rs:204-210). The loop therefore runs as follows:
1. A rewrite that rewords a meaning, rather than changing the letter, produces a new candidate.
2. The new candidate gets a new verdict, and a verdict of "different" queues another rewrite.
3. `next` issues overloads tasks whenever unjudged candidates exist (exegenius-core/src/task.rs). Support checks stop at two per page (exegenius-core/src/support.rs:29); overload rounds have no such limit.

In the 0.5.0 run, 1,699 verdicts said "different", more than the 912 page bindings, because each binding is compared with every ancestor's. 661 candidates were waiting for a verdict after the first rewrites. Whether further rounds shrink or cycle has not been observed.

## Current State
- The 0.5.0 run on the n log n paper ended its audit rounds with 142 pages from 157 concepts and 836 gaps over the 119 pages audited in round 3, against 45 pages and 4 gaps in round 3 of the 0.1 run with the same target, reader profile and rounds
- The 0.5.0 run's citation support labels were 202 supported, 185 partial, 79 unsupported and 1 contradicted, of 467 marks
- Overload candidates are matched with earlier verdicts on every field, including the meaning text, so a rewrite that rewords a meaning yields a new candidate (exegenius-core/src/notation.rs:204-210)
- `next` issues overloads tasks while unjudged candidates exist, with no limit on rounds (exegenius-core/src/task.rs)

## Missing
- No split of the 0.5.0 run's gaps by kind
- No audit of the same pages with a larger model
- No limit on overload rounds

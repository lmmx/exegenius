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

## What the run's records show

The author pushed the run's `exegenius/` directory to the `tmp-v0.5-build-record-stash` branch of lmmx/openai-nlogn-wiki (commit fa6c804). The figures below come from its `audit.json`, `graph.json` and pages, set against the same files of the 0.1 run.

**Gaps per audited page, by kind:**

| Round | 0.1 term | 0.1 step | 0.1 symbol | 0.5.0 term | 0.5.0 step | 0.5.0 symbol |
|---|---|---|---|---|---|---|
| 1 | 0.7 | 0.8 | 0.3 | 2.8 | 4.1 | 2.2 |
| 2 | 0.2 | 0.2 | 0.2 | 2.1 | 3.9 | 2.5 |
| 3 | 0.0 | 0 | 0.0 | 1.2 | 3.4 | 2.3 |

Term gaps fell each round. Step and symbol gaps hardly fell.

**Rewrites replace step gaps rather than remove them.** On pages audited in two consecutive rounds:
- **Rounds 1 to 2:** the 32 pages carried 131 step gaps before their rewrites and 171 after. Only 8 of the 171 quote a passage reported before.
- **Rounds 2 to 3:** the 82 pages carried 320 before and 321 after, 19 of them seen before.

The rewrites removed the steps they were told about and added about as many new ones.

**Pages grow with each rewrite round.** Median words per page:
- 589 for the 32 pages first audited in round 1;
- 466 for the 50 first audited in round 2;
- 390 for the 37 first audited in round 3;
- 289 for the 23 pages written after the last round and never audited;
- 221 across the 0.1 run's 45 pages.

The rewrite length budget only warns (exegenius-core/src/write.rs).

**Late symbol meanings grow.** Symbol gaps of the late-meaning kind ("say what the symbol denotes at its first use") went 22, 95, 132 over the three rounds.

**Citations break sentences.** The 0.1 pages carry no citation marks; the 0.5.0 pages carry 771 on 135 pages. Of these, 234 follow "that", "as", "states", "says", "is", "gives", "reads" or a colon, so the sentence needs the quoted words: "the paper states the bound as <cite>…</cite>". Audit tasks remove marks whole (exegenius-core/src/audit.rs:191, exegenius-core/src/cite.rs:97-99), and the site shows each mark as a reference such as "(see p. 5)". Either way, those sentences reach auditors and readers unfinished. Step gaps whose note describes a sentence that breaks off or stops short went 14, 37, 64 over the three rounds.

**Terms.** Round 1 reported 82 distinct terms, round 2 153 and round 3 137:
- **Mostly the paper's own constructions,** such as "bit procedure", "middle transform" and "cost table".
- **Some basics,** such as ring, permutation, eigenvalues, real numbers and natural logarithm.
- **References to the documents,** such as "the paper", "the argument" and "target result", reported as unknown terms.

## Explanations

1. **Longer writer obligations.** Since 0.1, writers have been asked for motivations, striking features, relation sentences, symbol meanings and citations. The never-audited first drafts are 289 words against the 0.1 median of 221, and round 1 already had four to five times the 0.1 rate of term and step gaps.
2. **Rewrites that expand.** A step gap's rewrite note asks the writer to explain the step. The explanation brings new claims, and the audit counts any claim "the reader cannot check from what the page gives". On these figures the rewrite action has a gain near one for step gaps: each round removes the reported steps and adds as many. The step criterion has no stopping point for a research paper, whose pages cannot prove every lemma they use. A step the page attributes to the source with a citation is reported the same as one it presents as following from the page.
3. **Citations used as sentence parts,** as above.
4. **Stricter symbol accounting.** The completeness floor and the late-meaning rule turn symbols an 0.1 auditor let pass into gaps. Symbol gaps are a third of the total and do not fall, but they become rewrite notes, not concepts, so they do not grow the graph.
5. **The model.** Every task ran on Haiku. A smaller model meeting more simultaneous constraints per page may leave more of them unmet, as a writer, and may report more, as an auditor. Auditing a sample of the same pages with a larger model, and rewriting a sample with one, would separate writer from auditor.
6. **The audit wording.** It counts as unknown what the profile does not name (2026-10-11-graph-growth). The 0.1 run used the same wording and converged, so on its own the wording does not explain the difference.

## Overload verdicts loop without a limit

When the pages are final, the overloads task judges each letter a page binds that an ancestor bound with another meaning. A verdict of "different" queues a rewrite (exegenius-core/src/notation.rs:275-289). Candidates are matched with earlier verdicts on every field, the meaning text included (exegenius-core/src/notation.rs:204-210). The loop therefore runs as follows:
1. A rewrite that rewords a meaning, rather than changing the letter, produces a new candidate.
2. The new candidate gets a new verdict, and a verdict of "different" queues another rewrite.
3. `next` issues overloads tasks whenever unjudged candidates exist (exegenius-core/src/task.rs). Support checks stop at two per page (exegenius-core/src/support.rs:29); overload rounds have no such limit.

In the 0.5.0 run, 1,699 verdicts said "different", more than the 912 page bindings, because each binding is compared with every ancestor's. 661 candidates were waiting for a verdict after the first rewrites. Whether further rounds shrink or cycle has not been observed.

Explanations 1 to 3 are supported by the records. Explanation 5 is not separated from them: every task of the run used Haiku.

## Current State
- In the 0.5.0 run, rewrites replaced the step gaps reported on a page with as many new ones: on pages audited in consecutive rounds, step gaps went 131 to 171 and 320 to 321, with 8 and 19 of the later ones quoting a passage reported before (lmmx/openai-nlogn-wiki at fa6c804, exegenius/audit.json)
- 0.5.0 pages first audited in round 1 have a median of 589 words, against 289 for pages never audited and 221 for the 0.1 run
- 234 of the 0.5.0 run's 771 citation marks follow a word or colon that makes the sentence need the quoted text, and audit tasks and the site both remove that text from the sentence (exegenius-core/src/audit.rs:191, exegenius-core/src/cite.rs:97-99)
- The 0.5.0 run on the n log n paper ended its audit rounds with 142 pages from 157 concepts and 836 gaps over the 119 pages audited in round 3, against 45 pages and 4 gaps in round 3 of the 0.1 run with the same target, reader profile and rounds
- The 0.5.0 run's citation support labels were 202 supported, 185 partial, 79 unsupported and 1 contradicted, of 467 marks
- Overload candidates are matched with earlier verdicts on every field, including the meaning text, so a rewrite that rewords a meaning yields a new candidate (exegenius-core/src/notation.rs:204-210)
- `next` issues overloads tasks while unjudged candidates exist, with no limit on rounds (exegenius-core/src/task.rs)

## Missing
- No check that a sentence reads complete without its citation mark
- No acceptance, in the step criterion, of a step the page attributes to a cited source
- No enforced length budget on rewrites
- No audit of the same pages with a larger model
- No limit on overload rounds

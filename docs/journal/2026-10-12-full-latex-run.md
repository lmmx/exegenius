# 2026-10-12: Full run on the LaTeX source

The author ran the experiment of 2026-10-12-stop-early-switch and pushed it to lmmx/openai-nlogn-wiki as `full-v0.6-latex` (e4aff3e).
- **Software:** exegenius 47f9294.
- **Source:** `sources/paper.tex`, with a fresh extraction and the calculus correction at the reader check.
- **Settings:** `rounds = 3`, `stop_early = false`, no simulated readers.
- **The run:** 180 tasks in 32 minutes, all accepted, and 23 pages in 5 parts.

## The run

```
concepts: 48 (25 known to the reader); pages: 23 written of 23 planned
audit round 1: 69 gaps over 22 pages (per page: term 0.5, step 2.0, symbol 0.5)
audit round 2: 61 gaps over 23 pages (per page: term 0.4, step 2.0, symbol 0.3)
  on the 22 pages also audited in round 1: term 12 -> 7 (7 new), step 45 -> 44 (44 new), symbol 12 -> 7 (6 new)
audit round 3: 58 gaps over 23 pages (per page: term 0.5, step 2.0, symbol 0.1)
  on the 23 pages also audited in round 2: term 9 -> 11 (10 new), step 45 -> 45 (45 new), symbol 7 -> 2 (2 new)
steps inside reasoning a rewrite added for an earlier step, sent back to be cited or cut: 54
citation support: 30 marks judged: 13 supported, 14 partial, 2 unsupported, 1 contradicted
```

The stop rule would not have fired at either round, since the steps sent back to be cited or cut are left out of it, so `status` printed no counterfactual. Switching it off changed nothing in this run.

## Step gaps: every one fixed, as many new

Step gaps on repeated pages stayed level, 45, 44, 45, and almost all were new each round. Matching each round's reported passages against the next round's shows:

| Reported in round | Steps | Reported again in the next round |
|---|---|---|
| 1, first level | 45 | 0 |
| 2, first level | 13 | 0 |
| 2, cite or cut | 32 | 0 |

Every step a rewrite was told about was gone from the next audit. The level count is renewal.

**By kind:**
- First-level steps went from 45 in round 1 to 13 in round 2, then rose to 23 in round 3.
- Steps sent back to be cited or cut went 32, then 22. These are the second level, inside reasoning added for an earlier step. They are not reversals: the rule is meant to send them back, and their fix adds no claim.

**Where round 3's 23 first-level steps came from** (against each page's write answers):
- **7 were inside sentences a round 2 rewrite added that were not rewordings, on pages whose round 2 notes were all cite-or-cut notes.**
  - These rewrites were asked to cite or cut and added reasoning anyway, so their pages grew. `synthetic-transform` went from 313 to 387 words, and `address-shear` from 299 to 332.
  - `apply_write` recorded added sentences only for rewrites answering a step note, and replaced the record at each write. These sentences were therefore not counted as added, and steps in them came back as first-level, asking for more reasoning.
- **8 were in sentences the round 2 rewrite reworded.**
- **8 were in text on the page, word for word, at the round 2 audit, which that audit did not report.** An audit reports some of what a reader would stop at, not all of it, and a later audit of the same text reports others.

## The depth limit now remembers

A rewrite answering a note to cite or cut a step now records the sentences it adds, as one answering a step note does. Sentences recorded as added stay recorded while they are on the page, across later rewrites (exegenius-core/src/write.rs; `CITE_NOTE` in exegenius-core/src/audit.rs).

The test `reasoning_added_by_any_rewrite_for_a_step_stays_added` runs three rounds on one page:
- round 1 explains a step;
- round 2's step, inside that explanation, is sent back to be cited or cut;
- the cite-or-cut rewrite adds a sentence;
- round 3's step inside that sentence is sent back to be cited or cut.

Without the change the last came back as "explain the step". On this run's records, 7 of the 23 round 3 first-level steps would have been cite-or-cut instead.

The 8 steps in unchanged text are audit variance, which no change to the writer reaches. They mean that the gaps one round reports undercount what is on the pages. A level count can hide a falling stock of real gaps, or a steady one.

## Citations

The three marks not supported:
- **Contradicted, on `tensor-product`:** the claim gives the identity on the coefficient positions of each record. The quote says the layer acts "coefficientwise", which is to say the same map on each coefficient. The two agree; the judge read "coefficientwise" as acting on the coefficients.
- **Unsupported, on `lower-triangular-factorization`:** "$\Pi$ has exactly $a$ entries equal to one" is cited for $\operatorname{rank}\Pi = a$, which needs the entries in distinct rows and columns.
- **Unsupported, on `chinese-remainder-address-map`:** "by a Chinese-remainder map that preserves convolution" is cited for the coefficient list being placed on distinct prime axes.

The two unsupported marks quote a fragment beside the claim, as in 2026-10-12-latex-pilot.

## Current State
- The full LaTeX run ran 180 tasks over 23 pages in 32 minutes, with step gaps on repeated pages at 45, 44 and 45 in three rounds
- No step gap reported in one round of the full LaTeX run was reported again in the next
- Of its 23 first-level step gaps in round 3, 7 were inside reasoning added by rewrites answering cite-or-cut notes, 8 in reworded sentences, and 8 in text unchanged since the round 2 audit
- A rewrite answering a cite-or-cut note records the reasoning it adds, and added reasoning stays recorded while it is on the page (exegenius-core/src/write.rs)
- Of the run's 30 judged citation marks, the one labelled contradicted agrees with its quote, and the two labelled unsupported quote a fragment beside the claim

## Missing
- No run with added reasoning remembered across rewrites
- No measure of how much of what a reader would stop at one audit reports
- No review of the full LaTeX run's pages against the paper

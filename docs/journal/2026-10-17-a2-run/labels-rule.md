# A2 hand labels: the labelling rule (A1's, adopted unchanged by plans/A2.md, plus one clarification)

**What is labelled:** every gap in `A1-notes-blind.tsv`, a copy of `A1-notes.tsv` without its `asserts` column. Rows are shuffled and carry no configuration or run (the map is in `A1-notes.tsv.unblind.json`, not read until the labels are committed).

**Who labels:** this session, as `plans/A1.md` preregisters ("The session labels every gap"). The session had seen stage 1's preliminary pooled scores, computed with `ASSERTS`, and row 1 of the notes, before labelling. It cannot tell a row's configuration from its text.

**The plan's definition:** a gap names an error when its term or note asserts that something is false. A note that only says the reader cannot check a step does not name an error.

**Applied as follows, before labelling:**

`1` (asserts an error) when the note:
- says something on the page is wrong, false, incorrect, inconsistent or contradictory; or
- says a value, count, sign, exponent, bound, equation or direction on the page differs from what the page's own premises, definitions or other statements give ("that gives X, not Y", "should be Y", "the page says X here but Y there"); or
- says a stated claim fails, with a reason or counterexample.

`0` (does not) when the note only:
- says a step, term or bound is unjustified, unexplained, undefined, unstated, ambiguous or missing;
- says the reader cannot check, verify or follow it;
- says a claim "does not follow" or "is not shown" without saying what does follow or that the claim is false.

**The order of work:** the labels are written to `labels.tsv` (row, label) and committed before agreement with `ASSERTS` is computed and before the unblind map is read.

## For A2: what is labelled, and one clarification (added 07:25 UTC, 2026-10-17, during labelling, before any agreement was computed or the arm map read)

**What is labelled:** `A2-notes-masked.tsv`, which is `A2-notes.tsv` without `asserts`, with paper line references and the words paper and source(s) masked as [ref] (`tools/mask_notes.py`). The map is `A2-notes.tsv.unblind.json`.

**A note that states the page contradicts the paper** ("the paper says X, the page says Y") asserts that something on the page is false: `1`.

**Missing hypotheses:**
- `1` when the note says the page's claim is false or overgeneral as stated because it omits a hypothesis it needs ("stated more widely", "as written … does not hold", "the paper requires X and the page drops it").
- `0` when the note only says the page does not state an assumption the reader needs in order to follow or check a step.

# 2026-10-15 09:30 UTC: the background task, audited from existing outputs (no model calls)

**Question:** is the background task (Sonnet `high`, `master`'s task) stable enough that no model experiment is needed?

**Data:** the eight independent E6 projects F1–F8 (scratchpad `e6/F*/exegenius`). Each has its own extraction and background, from Sonnet `high` subagents.

## Results

| | F1 | F2 | F3 | F4 | F5 | F6 | F7 | F8 |
|---|---|---|---|---|---|---|---|---|
| Known concepts | 17 | 21 | 23 | 17 | 18 | 20 | 17 | 25 |
| Submits (rejected) | 1 (0) | 1 (0) | 1 (0) | 2 (1) | 1 (0) | 1 (0) | 1 (0) | 1 (0) |

- **Acceptance:** 7 of 8 were accepted on the first submit; F4 needed a second submit.
- **Agreement between runs:** a concept counts as matched when its name or one of its aliases matches a name or alias in the other run. The share of one run's concepts matched in another is median 0.67, range 0.36–0.88 over the 56 ordered pairs.
- **Exact names rarely agree** (median Jaccard 0.03): runs name the same idea differently, so name-only comparison is not a usable measure.
- **Downstream:** the backgrounds of F2–F6 and F8 all fed argument plans that reached 19/19 faithful and the central gate under the combined instruction (E9–E11). F1 and F7 were not tested under it.

## Interpretation

- **Planning:** run-to-run variation in the known-concept list is real (about a third of concepts differ between two runs). It did not stop any tested plan from reaching the ceiling.
- **Writing:** the list also tells writers what not to explain. Its effect on pages is not measured. It would show up as over- or under-explanation, which page judging does not score.
- **Decision:** keep Sonnet `high`. A model experiment on background is not justified: the evidence gives no sign that it limits a downstream decision.
- **Smallest remaining check, if writing quality later points here:** compare known-concept lists against the reader profile in `exegenius.toml`. That needs no model.

# 2026-10-12: Early stopping can be turned off

2026-10-12-latex-pilot found that the stop rule ended the rounds after round 2 with 15 gaps of missing reasoning queued for rewriting. A third round would have acted on every one. The next experiment, a full run on the LaTeX source, asks whether the loop closes such gaps in dense material when it gets the chance. With the rule on, it could stop at round 2 again, and the run would test the rule instead of the loop.

The author and ChatGPT chose to switch the rule off for that run, not to change it:
- the threshold stays at a half, on by default;
- `rounds = 3` still bounds the run;
- the run records where the rule would have fired, so that the rule can afterwards be judged against what the next round did.

## The switch

`stop_early`, a top-level key of `exegenius.toml`, defaults to true (exegenius-core/src/config.rs). When it is false, a round that fails to halve the step and symbol gaps on its repeated pages is followed by the next one, up to `rounds` (exegenius-core/src/task.rs). `exegenius status` then prints, for each such round, "`stop_early` is off; with it on, the audit rounds would have stopped after audit round N, …" with the figures (exegenius-cli/src/main.rs).

The test `early_stopping_can_be_turned_off` runs two rounds in which each page's step gap is reported again on a sentence the rewrite kept:
- with the switch on, the outline comes next;
- with it off, the three pages are rewritten for round 3.

## What the full run can then show

At each round where the rule would have fired, the following round's gaps on the same pages say whether stopping would have lost useful work. ChatGPT's caution applies: one run that improves after the rule would have fired shows that stopping cut off useful work in that run. It does not give a general stopping rule. Gaps that persist through round 3 do not by themselves show that the writing cannot fix them.

## Current State
- `stop_early = false` in `exegenius.toml` runs every audit round up to `rounds`; the default is true (exegenius-core/src/config.rs, exegenius-core/src/task.rs)
- With it off, `exegenius status` prints each round after which the stop rule would have ended the rounds (exegenius-cli/src/main.rs)

## Missing
- No run with `stop_early = false`
- No per-gap record of which round 2 gaps a third round closes

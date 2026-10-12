# 2026-10-12: Reader check pilot

The author ran the pilot of 2026-10-11-reader-check and pushed it to lmmx/openai-nlogn-wiki as `pilot-v0.6-reader-check` (ea8751b), at exegenius 366e8ae. The protocol was that of 2026-10-11-depth-limit, interactive, so the reader check was shown to the author. The background could no longer mark planned concepts known. `[pilot] pages = 5` therefore chose `transform-identity`, with the same five pages as the reduction pilot, and the two runs compare directly.

## The run

The session took 16 minutes over 41 tasks and ran to `done`. From `exegenius status`:

```
concepts: 61 (29 known to the reader); pages: 6 written of 6 planned
audit round 1: 12 gaps over 5 pages (per page: term 0.6, step 1.2, symbol 0.6)
audit round 2: 6 gaps over 6 pages (per page: term 0.0, step 0.3, symbol 0.7)
  on the 5 pages also audited in round 1: term 3 -> 0 (0 new), step 6 -> 0 (0 new), symbol 3 -> 1 (1 new)
steps left unexplained because they are inside reasoning a rewrite added for an earlier step: 5
the last audit round's 6 gaps are left on the pages, since no audit would check rewrites for them; raise `rounds` to have them fixed
citation support: 12 marks judged: 4 supported, 5 partial, 3 unsupported, 0 contradicted
```

Against the reduction pilot, same pilot concept and same first five pages:

| | reduction pilot | reader check pilot |
|---|---|---|
| round 1 gaps (term, step, symbol) | 9, 6, 12 | 3, 6, 3 |
| repeated pages, term | 9 → 2 | 3 → 0 |
| repeated pages, step, as reported | 6 → 7 | 6 → 0 |
| repeated pages, step, counting those the depth limit left | 6 → 7 | 6 → 5 |
| repeated pages, symbol | 12 → 19 | 3 → 1 |
| pages planned | 5 → 11 → 15 | 5 → 6 |

The one page extend added is cyclic convolution, a construction of the paper.

## What is fixed

- **Symbols.** Round 1 reported 3 symbol gaps, against 12, and round 2 reported 1 on the repeated pages. The late-meaning rule removed the "where" clauses.
- **Growth.** The graph grew by one page, not ten. 25 known concepts from the background resolved the terms that made pages in the reduction pilot.
- **Accuracy of the final pages.** `gaussian-resampling` now gives the Gaussian maps $S$ and $T$, $N = CTD$, the Neumann series and the contraction bounds, as the paper does. Its final rewrite came from support checks, which still run after the audit rounds.

## What is not fixed: step renewal, now hidden

Step gaps on the repeated pages fell from 6 to 0 only because the depth limit left 5. Counted with those 5, they went 6 → 5, all new: the renewal of 2026-10-11-depth-limit, unchanged. Counting them, step and symbol gaps went 9 → 6, and the stop rule would have ended the run.

The depth limit assumes that a step inside added reasoning is a finer link of the step it explains. Some of the 5 are, and some are not. On `transform-identity`, round 2 left these:
- "$t_i$ is never defined, and $\gcd(s,t)=1$ concerns the whole lengths $s$ and $t$, not the per-axis pair": the rewrite brought in an undefined quantity and a claim that may not hold.
- "The normalization constant of the opposite transform $F_s^+$ is never stated": the final page still asserts $F_s^+F_s = I/M$ without the convention that makes it true.

These are defects a rewrite introduced. The depth limit dropped them with no rewrite note, so the reported figure looks converged while the pages keep them.

2026-10-11-depth-limit planned that such a step "must cite it, or it goes to the report". As built, the limit drops it, and that was the shortcut. A step inside added reasoning could get a rewrite note asking for a citation or for the claim to be cut, with no further reasoning. That fix adds no claim, so it cannot renew.

## The reader check, as the author saw it

The author accepted the background as shown and recorded three observations, for later work:
- **An assumed-background appendix.** The 25 known concepts are standard material, and Wikipedia has pages for each. Instead of only assuming them, the wiki could list them in an appendix that links out, so a reader missing one has somewhere to go. One day.
- **The check is a loaded question.**
  - **What it shows:** an undifferentiated list of 25 concepts, followed by "does this describe the reader you mean?". The conclusion is bundled into the premise, and a careful correction is hard to put into words, so the easy answer is "good enough".
  - **Possible fix:** a check that separated the items, saying that some are more advanced than others or asking about the borderline ones, would invite a precise correction.
  - **Why it matters:** following without checking is the failure the wiki exists to prevent, so the check reproduces that failure. This is a known pain point, not addressed here.
- **Known concepts load the reader.** 29 of 61 concepts are known. The background no longer removes planned pages, but every term it marks known is one the wiki will not explain, and it shifts expectation onto the reader. This is not the adjustment of reading level the author intended, and is left for separate work.

## Current State
- The reader check pilot ran 41 tasks in 16 minutes to `done`, explaining `transform-identity` with the five pages of the reduction pilot and one added page, cyclic convolution
- On the repeated pages, term gaps went 3 → 0 and symbol gaps 3 → 1; step gaps went 6 → 0 as reported and 6 → 5 counting the 5 the depth limit left
- Two of the steps the depth limit left on `transform-identity` are an undefined $t_i$ and an unstated normalisation of $F_s^+$, which the final page still relies on
- The depth limit drops a step inside added reasoning without a rewrite note, and `status` counts it apart from the convergence figures (exegenius-core/src/audit.rs, exegenius-cli/src/main.rs)
- The reader check prints the profile, summary and known concepts as one list, followed by a single question (exegenius-cli/src/main.rs)

## Missing
- No rewrite note for a step inside added reasoning, asking for a citation or for the claim to be cut
- No assumed-background appendix linking the known concepts out
- No reader check that separates borderline known concepts from the rest
- No adjustment of reading level beyond the background task

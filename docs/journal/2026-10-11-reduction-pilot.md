# 2026-10-11: Reduction pilot

The author ran the pilot of 2026-10-11-reduction on 0.6.0 (8cff21f), in a fresh Haiku session in the author's container, and pushed the result to lmmx/openai-nlogn-wiki as `pilot-v0.6-convergence` (4b921b6). The protocol was that of 2026-10-11-baseline-pilot, with `pages = 5` and the subagent definitions. The run did not converge, and the stop rule ended it after round 2. This entry records what changed against the baseline and what did not.

## The run

The session reported 18 minutes 25 seconds; the baseline pilot took 34 minutes. Each task ran in a subagent of the type `next` named. 51 task files were issued.

The binary had been built on a host with glibc 2.38, and the container has 2.36. The session ran it through a newer loader in its scratch directory, leaving the system libc unchanged.

From `exegenius status`:

```
pilot: transform-identity (Transform identity with retained permutations), the part of the target this run explains
concepts: 48 (4 known to the reader); pages: 15 written of 15 planned
audit round 1: 27 gaps over 5 pages (per page: term 1.8, step 1.2, symbol 2.4)
audit round 2: 47 gaps over 11 pages (per page: term 0.6, step 0.8, symbol 2.8)
  on the 5 pages also audited in round 1: term 9 -> 2 (1 new), step 6 -> 7 (7 new), symbol 12 -> 19 (15 new)
audit rounds stopped early: after audit round 2, step and symbol gaps on the 5 pages also audited in round 1 went from 18 to 26, not to half or fewer, so rewrites were not converging
```

## What changed against the baseline

| | baseline pilot (7 pages) | reduction pilot (5 pages) |
|---|---|---|
| first drafts, prose words | mean 469, 5 of 7 over 350 | 153 to 339, median 214 |
| step gaps per page, round 1 | 3.3 | 1.2 |
| term gaps on repeated pages | 12 → 6 | 9 → 2 |
| step gaps on repeated pages | 23 → 21 | 6 → 7 |
| symbol gaps on repeated pages | 11 → 13 | 12 → 19 |
| planned pages | 7 → 14 | 5 → 11 → 15 |

Three of the changes worked:
- **The length limit held.** The first drafts are the length of the 0.1 run's pages (median 180 prose words).
- **Steps by reference cut step gaps.** Round 1 had 1.2 step gaps per page against 3.3. On repeated pages the counts are now small (6 → 7), though still renewed.
- **The stop rule ended the run.** It stopped at round 2, after 51 task files, where the 0.5.0 run had gone on to 855 tasks.

## What did not change

### Symbol gaps

Matched by symbol rather than by passage, the 5 repeated pages kept 6 symbol gaps, fixed 6 and gained 13. Two causes show in the records.

**Late meanings.** The late-meaning rule reports a symbol whose meaning comes after its first use. On `gaussian-resampling`, the same four symbols (s, t, α, p) were reported as late in both rounds: the page writes a formula followed by "where $s$ is the number of coordinates of the source …". That is the usual way mathematics introduces symbols, and the rewrite kept it. Gaps of this kind went 5 in round 1 to 12 in round 2.

**Rewrites add formulas.** Rewrites brought new formulas with new symbols. All six symbol gaps on `transform-identity` in round 2 are symbols absent from its first version:
- F_s, F_t and F_s^+;
- X and Y in a tensor-product identity;
- `*` for convolution.

The step note offers reasoning or a citation, and reasoning was given as formulas whose symbols the page does not explain.

**The figure in `status` overstates renewal for symbols.** It matches gaps by passage, and a reworded sentence about the same symbol counts as new. By symbol, 13 of the 19 are new, not 15.

### Growth

Extend added 12 concepts over the two rounds, none of them marked known:
- axis length, coprime integers, Gaussian width, linear map, modular arithmetic, operator norm, tensor product, the Chinese remainder theorem, the convolution theorem, the Gaussian function, matrix product, and "transform".

Round 1's terms included "linear map", "coprime" (twice), "modular indices", "tensor product" and "norm of a linear map". The reader profile names big-O notation, polynomials, complex numbers and basic Turing machines. Whether it covers linear maps and coprimality is the judgement the new audit and extend wording leaves to the agents, and they judged it does not. The growth factor stays above one page per page audited: 5 pages led to 6 new, and 11 led to 4 more.

## Next changes, from these records

1. **Late meanings.** A meaning given in the sentence or paragraph that follows a formula, as a "where" clause does, counts as given in time.
2. **Symbol renewal by symbol.** `convergence::summarise` matches symbol gaps by their symbol, not their passage.
3. **Growth bounded by construction.** Extend may add at most half as many pages as the round audited. The terms beyond that resolve to known concepts, or to a rewrite note asking for a sentence of explanation on the page. Each round then adds fewer pages than the one before, whatever the agents judge a profile to cover.
4. **No new symbols in rewrites.** A rewrite introduces no symbol the previous version did not use, unless it says what the symbol denotes in the same sentence.

## Current State
- The reduction pilot's first drafts had 153 to 339 words of prose, and round 1 reported 1.2 step gaps per page, against 3.3 in the baseline pilot
- The reduction pilot's stop rule ended the audit rounds after round 2, when step and symbol gaps on the 5 repeated pages went from 18 to 26
- On the reduction pilot's repeated pages, symbol gaps went from 12 to 19, with 13 symbols new and the same four late meanings on `gaussian-resampling` reported in both rounds
- The reduction pilot's extend added 12 concepts, none known to the reader, among them linear map, matrix product, modular arithmetic and coprime integers
- `convergence::summarise` matches symbol gaps by passage, so a reworded sentence about the same symbol counts as a new gap (exegenius-core/src/convergence.rs)

## Missing
- No acceptance of a meaning given in the sentence or paragraph after a formula
- No bound on the pages extend may add per round
- No check on symbols introduced by a rewrite

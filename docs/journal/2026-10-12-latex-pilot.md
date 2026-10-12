# 2026-10-12: LaTeX pilot

The author ran the pilot of 2026-10-12-latex-source unattended and pushed it to lmmx/openai-nlogn-wiki as `pilot-v0.6-latex-source` (42cd64b).
- **Software:** exegenius 7770e5f.
- **Source:** `sources/paper.tex`, with a fresh extraction.
- **Settings:** `rounds = 3` and `[pilot] pages = 5`.
- **The run:** 37 tasks in 19 minutes, ending in `done`, with no answer rejected.
- **The reader check:** the orchestrating agent applied the calculus correction there. Two attempts before `next` had shown the reader were refused, as `redo_background` is meant to refuse them.

## The run

```
target: main-theorem (Deterministic fixed-tape sub-quasilinear multiplication)
pilot: subset-network (Subset-intersection network), the part of the target this run explains
concepts: 53 (24 known to the reader); pages: 5 written of 5 planned
audit round 1: 41 gaps over 5 pages (per page: term 1.6, step 4.6, symbol 2.0)
audit round 2: 29 gaps over 5 pages (per page: term 0.6, step 3.4, symbol 1.8)
  on the 5 pages also audited in round 1: term 8 -> 3 (3 new), step 23 -> 17 (16 new), symbol 10 -> 9 (9 new)
steps inside reasoning a rewrite added for an earlier step, sent back to be cited or cut: 9
audit rounds stopped early: after audit round 2, step and symbol gaps on the 5 pages also audited in round 1 went from 33 to 17, not to half or fewer, so rewrites were not converging (9 steps sent back to be cited or cut not counted)
citation support: 5 marks judged: 0 supported, 4 partial, 1 unsupported, 0 contradicted
```

**A different pilot.** Extraction from the LaTeX gave another target name and another graph. The pilot concept is `subset-network`, the paper's §5 construction, with:
- `fixed-tape-machine`;
- `linear-algebra`;
- `linear-network`;
- `frame`.

None of these pages is in the earlier pilots, so this run tests the source and the loop on new material, not the same pages from a better text.

**The reader.** The background summary now includes calculus, as corrected, and excludes tensor products and bilinear forms. 24 of 53 concepts are known.

## The pages, against the paper

`subset-network` (447 words) states the paper's construction and its count, and each step agrees with §5 of `paper.tex`:
- the form $\langle x,y\rangle = x^{\mathsf T}(I - J/9)y$;
- $\langle t_S,t_T\rangle = |S\cap T|-1$ and $\langle t_T,t_T\rangle = 2$;
- the bank labels $U_a$;
- the signed sum of dimension changes $Wm - 2N$, and $2\lambda$ added for the absolute sum;
- one rank per bank from the rational interface, giving $\sigma = Wm - N + 2\lambda$;
- $\lambda/N = 100/539$, so the bit network's deficit is $\tfrac{339}{539}N$.

No claim found in it contradicts the paper.

**What the remaining gaps say.** The 29 gaps round 2 left, read one by one:

| Kind | Gaps | Examples |
|---|---|---|
| A step or term the page itself leaves unexplained | 15 | which vertices are sources and sinks; why every sink label is the whole space; the movement restriction on the tape; why reducing modulo 2 is valid; what $U_a^\perp$ denotes |
| A concept the five-page pilot has no page for | 7 | tensor products and $\otimes$; a "smaller call"; the rational interface; the central-wire returns; which construction is the bit network |
| Notation | 7 | five pedantic ($x$ and $y$ not said to range over $\mathbb Q^h$; $a_1, a_2, a_3$ as components of the bank index $a$), two real ($u$ reused for a vertex, $S$ and $T$ used before "neighbors" is defined) |

9 of the 15 page-local steps were steps sent back to be cited or cut. A third round would have sent notes for them and the rest. It did not run (below).

The concept is dense, and five pages cannot carry what it rests on. The growth limit allows one new page per round on five pages, and the candidate concept for tensor products came in round 2, whose candidates are dropped. That confounds what the pilot can say:
- **What it shows:**
  - the LaTeX source gives pages whose formulas agree with the paper;
  - a run on it completes without rejections;
  - citations quote fragments rather than the passages that state their claims.
- **What it does not show:** whether the loop closes gaps on dense material. Half of the gaps are page-local and would have had a third round. A quarter needed pages the pilot had no room for.

The pilot has not demonstrated success, and it has not demonstrated failure either.

**Citations.** All five marks quote a fragment beside the claim rather than the passage that states it:
- "both are smaller than $1/2$" cited for $\lambda/N = 100/539$ and $101/539$;
- "to make each edge change inexpensive" cited for the intermediate frames not affecting the route operator.

The judges labelled them partial or unsupported for that reason. The quoted text is in the source, which is all the write check verifies; whether it states the claim is the support check's to judge, and it judged correctly.

## The stop rule

The rounds stopped after round 2 because step and symbol gaps on the repeated pages went from 33 to 17, with the 9 steps sent back to be cited or cut not counted. The rule requires half or fewer, 16. Its message said "so rewrites were not converging", for a cut of 48%.

On reviewing the run, the threshold was changed to a cut of a third (e9dc240), chosen from the spread of all runs so far, and then changed back. The author and ChatGPT both objected:
- the change was made after a run it would have altered;
- a third is no less arbitrary than a half.

ChatGPT also pointed out a deeper problem. The rule takes the rate at which gaps fall as a proxy for whether another round is worth running. They differ:
- a slow round may be facing harder gaps, or an auditor finding new ones;
- a fast round may have cleared only easy gaps.

What the rule should test is open. Its record against outcomes is two diverging runs stopped (202 to 282, 34 to 34) and this pilot stopped while gaps were falling.

The threshold stays at a half. The message now says what was measured and no more: "went from 33 to 17, not to half or fewer, so the rounds stopped; the gaps may still have been falling" (exegenius-core/src/convergence.rs).

## Current State
- The LaTeX pilot ran 37 tasks over 5 pages in 19 minutes, with no rejected answer, and explained `subset-network`, a construction none of the earlier pilots covered
- Its `subset-network` page agrees with §5 of the paper in each formula checked
- Of its 29 remaining gaps, 15 are steps or terms its pages leave unexplained, 7 concern concepts the five-page pilot has no page for, and 7 are notation
- All five of its citation marks quote a fragment beside the claim rather than the passage stating it
- The audit rounds stop when step and symbol gaps on repeated pages are not cut to half or fewer, and the message says the gaps may still have been falling (exegenius-core/src/convergence.rs)

## Missing
- No test of whether the stop rule predicts that a further round would not help
- No pilot of a dense concept with room for the pages it rests on
- No check, at write time, that a citation's quote states its claim
- No full run on the LaTeX source

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

**What the remaining gaps say.** Round 2 left 29 gaps, 21 of them on `subset-network` and `linear-network`. They are about what the page relies on and the pilot has no page for:
- tensor products, outside the reader's background;
- the orthogonal complement $U_a^\perp$;
- what a "smaller call" is;
- the rational interface;
- the central-wire returns;
- which construction is the bit network.

Thirteen pages of the paper's §5 sit behind a five-page pilot. Its growth limit allows one new page per round on five pages, and the candidate concept for tensor products came in round 2, whose candidates are dropped.

**Citations.** All five marks quote a fragment beside the claim rather than the passage that states it:
- "both are smaller than $1/2$" cited for $\lambda/N = 100/539$ and $101/539$;
- "to make each edge change inexpensive" cited for the intermediate frames not affecting the route operator.

The judges labelled them partial or unsupported for that reason. The quoted text is in the source, which is all the write check verifies; whether it states the claim is the support check's to judge, and it judged correctly.

## The stop rule

The rounds stopped after round 2 because step and symbol gaps on the repeated pages went from 33 to 17, with the 9 steps sent back to be cited or cut not counted. The rule required half or fewer, 16.

The threshold of a half was set in 2026-10-11-reduction-pilot from the 0.1 run (28 to 10, 16 to 2), against the diverging 0.5.0 run (202 to 282) and baseline pilot (34 to 34). Since then:
- the full run went from 49 to 10;
- the first-level pilot went from 18 to 2, then 18 to 7;
- this pilot went from 33 to 17.

No run fell between a cut of a half and no cut at all. The rule's message also misstated what it measured: "rewrites were not converging", for a cut of 48%.

The rule now stops the rounds when rewrites cut step and symbol gaps on the repeated pages by less than a third. Its message says "a cut of less than a third, so rewrites were replacing gaps about as fast as they fixed them" (exegenius-core/src/convergence.rs). Every converging comparison above continues under it, and both diverging ones stop. The threshold was moved after a run it would have changed; it is set from the spread of all the runs, and this one passes it with five gaps to spare.

## Current State
- The LaTeX pilot ran 37 tasks over 5 pages in 19 minutes, with no rejected answer, and explained `subset-network`, a construction none of the earlier pilots covered
- Its `subset-network` page agrees with §5 of the paper in each formula checked
- 21 of its 29 remaining gaps are on `subset-network` and `linear-network`, about concepts the five-page pilot has no page for
- All five of its citation marks quote a fragment beside the claim rather than the passage stating it
- The audit rounds stop when step and symbol gaps on repeated pages are cut by less than a third (exegenius-core/src/convergence.rs)

## Missing
- No check, at write time, that a citation's quote states its claim
- No full run on the LaTeX source

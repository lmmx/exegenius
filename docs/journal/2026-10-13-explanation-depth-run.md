# 2026-10-13: The explanation-depth run

The author ran the experiment of 2026-10-12-explanation-depth and pushed it to lmmx/openai-nlogn-wiki as `full-v0.6-latex-depth` (fa33c9d).
- **Software:** exegenius 930d40c.
- **Setup:** Haiku, `paper.tex`, a fresh extraction, the calculus correction, `rounds = 3`, `stop_early = false`. Otherwise as the full LaTeX run.
- **The run:** 234 tasks in 67 minutes, all accepted, and 32 pages in 7 parts.

The run tests two halves of the hypothesis:
- pages explain the method they are responsible for;
- the regress does not return, and fixed gaps stay fixed.

## The run

```
concepts: 67 (35 known to the reader); pages: 32 written of 32 planned
audit round 1: 101 gaps over 27 pages (per page: term 1.3, step 2.1, symbol 0.4)
audit round 2: 102 gaps over 28 pages (per page: term 0.6, step 2.4, symbol 0.6)
audit round 3: 89 gaps over 32 pages (per page: term 0.4, step 2.0, symbol 0.3)
steps inside reasoning a rewrite added for an earlier step, sent back to be cited or cut: 55
citation support: 41 marks judged: 16 supported, 19 partial, 6 unsupported, 0 contradicted
```

**The extraction differed again.** It gave 67 concepts and 32 pages, against 48 and 23 in the full LaTeX run, under a third name for the target. The argument assigns steps to 10 pages:
- `bluestein`;
- `butterfly-layer`;
- `crt-axis-map`;
- `exact-recovery`;
- `fixed-tape-multiplication`;
- `gaussian-resampling`;
- `radix-digit-convolution`;
- `signed-packing`;
- `synthetic-transform`;
- `tape-interchange`.

The totals are not comparable with the earlier run's. Step gaps per page, at 2.1, 2.4 and 2.0, are close to its 2.0.

**The orchestrating agent reported:**
- audits 0052 and 0058 were launched twice, and the second copy rewrote each answer file after the first was accepted;
- three rewrites after the audit rounds (0200, 0205, 0214) were rejected for adding a displayed equation that their notes did not ask for, as 2026-10-12-narrow-rewrites intends, and were redone;
- some subagents lacked `exegenius` on their path.

## Regress and recurrence

**Steps by kind, on pages presenting a step and on the others:**

| Round | explained, presenting a step | cited or cut, presenting a step | explained, other | cited or cut, other |
|---|---|---|---|---|
| 1 | 26 | 0 | 30 | 0 |
| 2 | 29 | 0 | 20 | 18 |
| 3 | 13 | 17 | 15 | 20 |

**The rule acted as built.**
- Round 2's steps inside added reasoning were explained on pages presenting a step, and cited or cut elsewhere.
- In round 3, steps at depth 2 on those pages were cited or cut.
- No page was explained deeper than its limit.

**Gaps stayed fixed:**
- 2 of round 1's 56 step passages were reported again in round 2;
- 3 of round 2's 67 were reported again in round 3.

## Content

**`gaussian-resampling` gives Lemma 19's construction**, within its budget, at 412 words. Against `paper.tex`:
- **Hypotheses:** $p > 100$, $2 \le \sigma < \sqrt p$, $s < t < 2^p$, $\theta > p/\sigma^4$.
- **The maps:** the Gaussian maps $K$ and $M$, with their formulas, and the permutations.
- **The construction:** $\Gamma = \Lambda M \Delta$, and why its $j$-th term is $u_j$.
- **The estimates:** $\|I - \Gamma\| < 2^{-\sigma^2\theta}$, cited to Harvey and van der Hoeven, and $\sigma^2\theta > 1$. Then the Neumann series, with its telescoping partial sums, $\|\Gamma^{-1}\| < 2$, and $\|\Delta\| \le e^{\pi\sigma^2/4} \le 2^{2\sigma^2-2}$.
- **The identity:** the left inverse $\Delta\Gamma^{-1}\Lambda$ gives $P_sF_s = 2^{2\sigma^2}K_2P_tF_tK_1$, with $K_1 = K/2$ and $K_2 = 2^{1-2\sigma^2}\Delta\Gamma^{-1}\Lambda$, and $\|K_2\| \le 2^{1-2\sigma^2}\cdot2^{2\sigma^2-2}\cdot2\cdot1 = 1$.

Each step checks. It is the fullest and most exact version of the page in any run, and it is the outcome the rule was built for.

**`butterfly-layer` cites its bound.**
- Round 2 reported the page's central claim, "the layer costs $O(Vd^{\lambda'})$ (cited)". It was explained, since the claim was cited.
- The rewrite added a one-sentence account of the proof's last step: groups of power-of-base sizes, summed over $O(\log d)$ levels, for any $\lambda'$ above "the swap exponents". It omits the recursion with $s < Wm$, which is where the saving comes from.
- Round 3 reported that account at depth 2, which was then cited or cut.

The page cannot link the recursion. Extraction gave `butterfly-layer` the prerequisites `gaussian-dyadic`, `frame`, `complex-numbers` and `tensor-product`, not `recursive-saving`, `linear-network` or `role-streams`.

**`recursive-saving` cites its decisive fact.** The page holds the paper's recursion. It presents no step of the argument in this run, so it explains to one level, and it cites "$s < Wm$" and "the resulting recurrence saves a power of the chunk width" without deriving the recurrence. In the full LaTeX run, its counterpart `recursive-power-saving` presented a step and derived the geometric sum.

## What this run shows

**The first half holds where responsibility was assigned to the page carrying the mechanism.** `gaussian-resampling` went from hollow to complete.

**Where it does not hold, the cause is the assignment, not the depth rule.** The butterfly layer's mechanism is the recursion. That recursion sits on a page the argument did not assign a step to, and extraction left no prerequisite edge from the butterfly page to it. This is the uncertainty ChatGPT named in 2026-10-12-explanation-depth: a page assigned a step may depend on a construction that belongs on another page.

**The second half holds.** Steps were bounded at the limit and fixed gaps stayed fixed.

**Each run's extraction differs.** Three fresh extractions of `paper.tex` gave three graphs, three target names and three argument plans. Which page carries which mechanism, and which pages present steps, changed with them, so a run cannot be set against the last page for page.

## Current State
- The explanation-depth run explained steps inside added reasoning on its 10 pages presenting a step and cited or cut them elsewhere; at depth 2 on those pages they were cited or cut
- 2 of its round 1 step passages and 3 of its round 2 step passages were reported again in the next round
- Its `gaussian-resampling` gives Lemma 19's construction, hypotheses and constants in agreement with the paper
- Its `butterfly-layer` states the layer bound with a one-sentence account of the proof's last step, and has no prerequisite edge to `recursive-saving`, `linear-network` or `role-streams`
- Its `recursive-saving` presents no step of the argument and cites $s < Wm$ and the power saving without deriving the recurrence
- Three fresh extractions of `paper.tex` gave three different graphs, target names and argument plans

## Missing
- No run that reuses an extraction and an argument, so that runs compare page for page
- No assignment of explanatory responsibility to a page carrying a mechanism that the argument's page relies on
- No review of this run's other 29 pages against the paper

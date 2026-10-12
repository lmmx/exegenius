# 2026-10-12: Depth relative to a page's responsibility, a prototype

2026-10-12-full-latex-run found `gaussian-resampling` and `simultaneous-butterfly-layer` hollow. Both state their result and cite the mechanism. The depth limit had sent the steps of each mechanism back to be cited or cut, as reasoning inside reasoning a rewrite had added.

ChatGPT's view, which the author adopted:
- the depth limit should be relative to a page's explanatory responsibility;
- a page should develop its own construction far enough to show how it works, and elaborate imported results only as far as needed to apply them;
- a flag set by the auditor to bypass the limit risks reopening the regress.

This entry proposes a rule decided from records the code already has, prototypes it on the two pages at no change to exegenius, and sets out how to test it.

## Where responsibility is recorded

The argument task assigns steps of the paper's proof to pages, as `page` on a node of `argument.json`. In the full LaTeX run, 13 of 23 pages present a step:
- `gaussian-resampling` presents S4, a step to show: "A Fourier transform of prime length is a multidimensional transform whose axis lengths are powers of two", by "apply Gaussian resampling and keep the source and target permutations inside the transform identity".
- `simultaneous-butterfly-layer` presents S2.2, a step to infer: the layer "costs $O(Vd^{\lambda'})$".
- Background pages, such as `tensor-product`, `matrix-rank` and `roots-of-unity`, present none.

The assignment is made by an earlier task, from the sources, before any page is written or audited. No auditor sets it.

## The proposed rule

1. **Responsibility.** A page that presents a step of the argument is responsible for how that step's method establishes its claim. Any other page is responsible for what it defines.
2. **Depth.** Each sentence a rewrite adds in answer to a step note has a depth: one more than the step it answers, where text not added for a step has depth 0. A step reported inside an added sentence is sent back to be cited or cut when the sentence's depth reaches the page's limit: 2 on a page presenting a step of the argument, 1 on any other. Explanation stays bounded, at two levels at most.
3. **Notes.** On a page presenting a step, a step note below the limit says the step is part of how the page's own step works and asks for it to be developed there.
4. **Kept.** Three earlier changes stay:
   - added reasoning is tracked across rewrites;
   - a step on an already cited claim is explained, not cited or cut;
   - citations of LaTeX markup are rejected.

**On the run's records:**
- round 2: 23 of the 32 steps sent back to be cited or cut were on pages presenting a step;
- round 3: 18 of 22.

These would have had notes asking for the reasoning instead.

## The prototype

Each page's actual round 2 rewrite task from the full LaTeX run was given to a fresh Haiku subagent. Haiku is the run's model. The task was unchanged except for its notes to cite or cut, which were reworded as the rule would word them:
- 2 on `gaussian-resampling`;
- 3 on `simultaneous-butterfly-layer`.

There was no submit, so the result was checked afterwards against submit's tests (word budget, new letters, citations in the source word for word). Each new page was then audited by a fresh Haiku subagent with the run's round 3 audit task, and read against `paper.tex`.

**`gaussian-resampling`.** 301 words of prose, against a budget of 318. It now gives the construction.
- **The maps:** the Gaussian-weighted maps $\Phi$ and $\Psi$, with their formulas, and the exact identity $\Psi P_qF_q = P_\nu F_\nu\Phi$.
- **The construction:** the selection $C$, the diagonal $\Delta$ and $\Lambda = C\Psi\Delta$, whose $j$-th term is $a_j$. Then the left inverse $\Delta\Lambda^{-1}C$, $L_1 = \Phi/2$ and $L_2 = 2^{-(2\alpha^2-1)}\Delta\Lambda^{-1}C$.
- **Against the paper (Lemma 19's proof, renamed):** $\Phi, \Psi, \Delta, \Lambda$ are its $S, T, D, N$. $L_2$ equals its $D'J'C$, and $\sigma = 2^{2\alpha^2}$ is its constant.
- **An addition of its own:** the writer bounded $\|L_2\| \le 2^{-(2\alpha^2-2)}e^{\pi\alpha^2/4}$, which the paper does not spell out. It equals $2^{2 - 0.87\alpha^2} < 1$ for $\alpha \ge 2$, so it is correct.

No claim found contradicts the paper. Its notation is consistent. One letter, $I$, was introduced unbound, which submit would have rejected.

The audit reported 4 steps and 1 term, each about the construction now on the page:
- the norm of a map, outside the reader's background;
- the sums over all integers, with no convention for indices outside $0..q-1$;
- the estimate $\|\Lambda - I\| < \tfrac12$, asserted without its citation at that sentence;
- invertibility from that bound, the Neumann series;
- the bound on $\|L_2\|$, which needs $\|\Delta\| \le e^{\pi\alpha^2/4}$, unstated.

The run's page drew 2 steps and 2 terms in round 3: fewer gaps, about a page with no construction to have gaps in.

**`simultaneous-butterfly-layer`.** 418 words of prose, against a budget of 423. It now gives the recursion of the paper's proof of the layer bound, and each claim agrees with §5:
- a node of $e$ chunks split into slots;
- $s$ recursive calls;
- rows dealt cyclically to $W$ roles, each call on $1/W$ of the volume, with $s < W\ell$;
- overhead $O(e^\lambda)$ where $e \ge \sqrt d$;
- a geometric series over levels with ratio $(s/W)/\ell^\lambda < 1$;
- leaves costing $O(Ve^\sigma d^{(1-\sigma)/2})$;
- a base-$\ell$ split, with $\lambda' > \max\{\lambda, \sigma + (1-\sigma)/2\}$.

The audit reported 11 steps and 2 terms. The page names, without explaining, "kernel", "phase identities", "residual directions" and "roles", and then states each line of the recursion without its reason. The page holds the proof's skeleton correctly, but at the reader's level and in 418 words it is not followable.

## What the prototype shows

- **The rule works where the mechanism fits the page.** On `gaussian-resampling`, notes asking for the reasoning got the construction, correct and within budget. What the next audit reported is the finer level, where cite-or-cut belongs.
- **Where it does not fit, the rule exposes how much the page owes.**
  - On `simultaneous-butterfly-layer`, the mechanism rests on concepts with no page in this wiki: the kernel $C = aI + bX$, residual bases and phase frames.
  - It also hit the word budget.
  - The result is correct and unfollowable. The rule does not cause that; it shows it. The fix lies elsewhere: pages for those concepts, or room on the page.
- **Gap counts mislead here.** The page that explains more drew more gaps, and the hollow page fewer.

## Testing it against regressions

- **Unit and end-to-end tests:**
  - a depth 1 step on a page presenting a step of the argument gets a step note, and a depth 2 step there gets cite-or-cut;
  - a depth 1 step on any other page gets cite-or-cut;
  - a rewrite answering a depth 1 note records its sentences at depth 2;
  - depths survive later rewrites while the sentences stay on the page.
- **Replay on recorded runs:**
  - how many cite-or-cut steps each run would have reclassified (above);
  - that no step deeper than the limit is ever asked to be explained.
- **In the next run, per page:**
  - depths of reported steps by round;
  - steps reported again;
  - words against the budget on pages presenting a step, since the butterfly page sat 5 words under its limit;
  - for a sample of pages presenting a step, whether the page states its method and whether each claim agrees with the source.
- **The regress bound:** steps at depth 2 or more on pages presenting a step go to cite-or-cut, so no page grows more than two levels of explanation.

## Current State
- In the full LaTeX run, 13 of 23 pages present a step of the argument, recorded in `argument.json` before any page is written
- Rewritten with notes asking for the reasoning in place of notes to cite or cut, `gaussian-resampling` gave the construction of Lemma 19's proof correctly in 301 words, within its budget of 318
- Rewritten the same way, `simultaneous-butterfly-layer` gave the recursion of the paper's proof of the layer bound correctly in 418 words, within its budget of 423, and its audit reported 11 steps and 2 terms
- Under the proposed rule, 23 of round 2's 32 and 18 of round 3's 22 steps sent back to be cited or cut in the full LaTeX run would have asked for the reasoning

## Missing
- No implementation of depth relative to the argument
- No page for the concepts the butterfly layer's mechanism rests on
- No test of the rule across a run

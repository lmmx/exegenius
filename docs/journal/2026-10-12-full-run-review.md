# 2026-10-12: Full run review

The author's full run (2026-10-12-full-run-stop) ran to `done` and was pushed to lmmx/openai-nlogn-wiki as `full-v0.6` (901a730): 183 tasks, 34 pages in 6 parts, with 5 pages folded into the glossary. Its audit rounds stopped after round 2, as that entry records. This entry records what the final pages show, set against the 0.1 run's pages where both have one.

## The final status

```
citation support: 38 marks judged: 19 supported, 14 partial, 5 unsupported, 0 contradicted
symbols: 144 bound on pages; letters with two meanings: 50 confirmed, 8 left unjudged after the last overloads round
```

The 34 pages have 10,410 words with their mathematics and 55 citation marks. The 0.1 run's 45 pages have 10,634 words and no citation marks.

## Steps answered by citing the paper's own constructions

Every step note since the reduction pass (8cff21f) ends: "Either give the reasoning, or attribute the step to the source with a citation; choose whichever needs fewer words" (exegenius-core/src/audit.rs). Audits count a claim marked as cited as attributed to the source. A rewrite may grow by a quarter of the page, or by 50 words (exegenius-core/src/write.rs `budget`). Under those three rules, the cheapest answer to a step note is a citation, and on two pages central to the reduction the writer took it.

**`gaussian-resampling`.** Round 2 reported two steps: how the Gaussian weights turn a length-$q$ transform into power-of-two transforms, and the identity behind the decomposition. Write 0086 answered both with the sentence "The construction is the one of Harvey and van der Hoeven; this page states its effect and does not re-derive the identity behind it." The final page has 204 words, says what the reduction achieves, and gives none of how it works. The 0.1 run's page, at 316 words, gives:
- the coprime lengths $s < t$;
- the permutations $(P_s u)_j = u_{tj}$ and $(P_t v)_k = v_{-sk}$, with a worked case;
- the contractions $A$ and $B_0$;
- the condition $\theta = t/s - 1 > p/\alpha^4$.

**`transform-identity`.** The final page has 158 words. It says the identity keeps both permutations in the formula so that they are never executed as sorts, illustrated with a permutation and its inverse cancelling. It does not state the identity.

The other pages do not show this as plainly; the longest, `packed-bit-addition`, has 633 words. A step explained one level deep (2026-10-11-depth-limit) assumes the first level is explained. As built, a step note can be answered at the first level by a citation, and then nothing is explained at any level.

## A rename after the audits broke a page that relied on it

The overloads task judged $a$ and $b$ on `complex-phase-network`, the scalars of the kernel $aI + bX$, against $a$ and $b$ on `gaussian-dyadics`, the parts of a Gaussian dyadic. The verdict said that `gaussian-dyadics` should change its letters. `apply_overloads` queues every note on the later page, which here was `complex-phase-network` (exegenius-core/src/notation.rs). Write 0174 renamed that page's scalars to $\alpha$ and $\beta$.

`simultaneous-butterfly-layer` still writes $C = aI + bX$, and its round 2 audit had accounted for $a$, $b$ and $X$ as bound on `complex-phase-network`. On the final pages $a$ and $b$ are bound nowhere it links, and $\beta$ is the kernel's second scalar on one page while $\beta_D$ is a different scalar on the next. The mathematics on the page is otherwise right: with $a = (1+i)/2$ and $b = (1-i)/2$, $H_0 = bSCS$, as the paper has it (checked numerically).

After the last audit, 23 rewrites from overload verdicts and support checks were written across 15 pages, none audited.

## What the remaining gaps are

Of the 50 gaps the last round left:
- 21 are steps sent back to be cited or cut;
- 13 are candidate concepts;
- 13 are symbols;
- 2 are steps;
- 1 added a prerequisite.

Nine of the 13 symbols are an index used under another letter than the one the page bound: $x_i$ and $y_l$ where the page bound the digit $x_j$, the coefficients $a_0, \dots, a_3$ of an example record, $e_i \otimes f_j$, $u'$ and $v'$. The steps include an inference the page does not complete ("$y^{2r} = 1$, so $y$ has order $2r$", which needs $y^r = -1$) and an unproved bound in `carry-propagation`.

## The orchestrating agent

The agent's report says:
- it twice said tasks were still running after they had been accepted, and once miscounted what remained;
- it once ran 21 subagents against a limit of 20;
- the session's rate limit stopped several write tasks part way, and it reran them.

## Current State
- The full run's 34 final pages have 10,410 words with mathematics and 55 citation marks, against 10,634 words on 45 pages in the 0.1 run
- A step note offers a citation as an alternative to reasoning, choosing whichever needs fewer words (exegenius-core/src/audit.rs)
- The full run's `gaussian-resampling` page states the reduction's effect and declines to give the identity behind it; the 0.1 run's page gives the permutations, contractions and condition
- The full run's `transform-identity` page does not state the identity
- An overload verdict's note is queued on the later page whichever page the verdict says should change (exegenius-core/src/notation.rs)
- A rename from an overload verdict on `complex-phase-network` left $a$ and $b$ unbound on `simultaneous-butterfly-layer`
- 23 rewrites from overload verdicts and support checks were written after the last audit, unaudited
- 9 of the last round's 13 symbol gaps are indices used under a different letter from the one the page bound

## Missing
- No check that a rewrite keeps the symbols later pages rely on
- No distinction between a step about a page's own concept and a step about a result it uses, in what a step note accepts
- No run with steps sent back to be cited or cut fixed in a third round

# 2026-10-13: Dependency and ownership, a prototype plan

2026-10-13-explanation-depth-run found that the depth rule works where a page carries the mechanism it is responsible for (`gaussian-resampling`). It found that the rule cannot help where the mechanism lies elsewhere in the chain (`butterfly-layer`). ChatGPT read the run's pages and refined that diagnosis, and proposed a targeted prototype on the run's fixed graph in place of another full run. The author approved it. This entry records the diagnosis and the plan, ahead of the results.

## The chain, as the run left it

The butterfly layer's cost bound rests on a chain of results:

| Link | In the run |
|---|---|
| `butterfly-layer` states $O(Vd^{\lambda'})$, with address movement done by fast interchange | links `tape-interchange` in its text, but the graph gives it no prerequisite edge to it. So it has no relation to it, takes no symbols from it, and need not come after it in reading order. |
| `tape-interchange` derives the recurrence | derives $F_d \le (s/W)F_{d-1} + O(1)$, unrolls it, and picks $\tau = 1 - 2^{-50}$ from the paper's deficit $\eta > 20\cdot2^{-50}$ and $\mu < 2^{20}$. Each step agrees with the paper. |
| the bound $s < Wm$ that the recurrence needs | cited on `recursive-saving`, `subspace-labels` and `tape-interchange`, and derived on none. `subspace-labels` ends "The claim is that $s$ is below $Wm$", cited. |

In the paper, $s < Wm$ comes from the subspace labels on the subset-intersection network:
- the bank labels are lines $U_a$;
- the signed sum of dimension changes is $Wm - 2N$;
- absolute changes add $2L$;
- the rational interface adds $N$.

So $s = Wm - N + 2L$, and $L/N = 100/539 < \tfrac12$ gives $s < Wm$. The LaTeX pilot's `subset-network` derived it (2026-10-12-latex-pilot). In this run's graph, `subset-network` holds the gate construction and `subspace-labels` holds the labels, so the derivation belongs on `subspace-labels`.

## Two hypotheses, tested separately

ChatGPT separated two failures that look alike:
- **A missing edge** (dependency placement): a page relies on a result another page derives, and the graph does not connect them.
- **A missing derivation** (explanatory ownership): a result is cited correctly everywhere, and derived nowhere.

Adding a link fixes the first and not the second.

**Hypothesis 1: dependency.** With `tape-interchange` as a prerequisite of `butterfly-layer`, the butterfly page can make its cost argument followable by stating what interchange gives and how the layer uses it, without repeating the interchange proof.

**Hypothesis 2: ownership.** With a note that the count $s < Wm$ is its to derive, `subspace-labels` can derive it correctly from the paper, within its budget.

**Then chain integrity.** With both, a reader can follow from the butterfly operation, through the interchange recurrence, to the saving, with every claim grounded in the source.

## Method

The method is that of 2026-10-12-responsibility-prototype, with no change to exegenius:
- **The task files.** For each page, its last write task from the run, with the final page as the previous version and the run's round 3 notes as its notes. Round 3 notes were dropped when the rounds ended, so these are the notes that page last earned. Then one change each:
  - `butterfly-layer` gets `tape-interchange` as a prerequisite, with a note to link it where the page relies on the cost of moving chunks;
  - `subspace-labels` gets a step note on its cited claim, saying the count is derived nowhere and this page owns it.
- **Writing.** A fresh Haiku subagent writes each page.
- **Checks.** Submit's checks: budget, new letters, citations in `paper.tex` word for word.
- **Audit.** A fresh Haiku subagent audits each page with the run's last audit task for it.
- **Reading.** Each page is read against `paper.tex`.
- **Records.** The run's pages and task records stay as they are, on `full-v0.6-latex-depth`. The prototypes are separate files.

**Questions to answer:**
- Does the butterfly page use the interchange result without duplicating its proof, and is its own recursion still owed?
- Is `subspace-labels`' derivation correct and within budget?
- Can the chain be followed end to end?
- Is anything that remains a missing link, or a missing derivation?

If both hypotheses hold, the next step is to propose the smallest general rule that finds missing dependency edges without giving every prerequisite the responsibility to explain. No full run comes before that. The run after it reuses this run's extraction, background and argument answers, so that pages compare one for one.

## Current State
- In the explanation-depth run, `butterfly-layer` links `tape-interchange` without a prerequisite edge to it
- In the explanation-depth run, `tape-interchange` derives the interchange recurrence and its exponent in agreement with the paper
- In the explanation-depth run, the bound $s < Wm$ is cited on `recursive-saving`, `subspace-labels` and `tape-interchange` and derived on none

## Missing
- No prototype results for the two hypotheses
- No general rule for finding missing dependency edges

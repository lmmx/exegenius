# 2026-10-13: Chain walk: do the existing wikis carry the paper's central saving?

The calibration programme's fidelity track (`calibration/fidelity/README.md`) asks whether a wiki carries a reader along the paper's central chain of reasoning, not only whether the plan states its results. The cheapest first measurement needed no new runs. A blind judge walked the nine steps of the central saving (`calibration/rubrics/nlogn-central-saving.md`) through the pages of the three wikis built from `paper.tex`. Those were the author's full runs: their plans were made from the paper's introduction (2026-10-13-planner-handoff-run), and their effort is unrecorded (2026-10-13-effort-dimension). For each step it recorded whether the wiki reconstructs it, whether a page says what the step is for, and how a reader gets from one step's page to the next.

Instrument: `calibration/fidelity/chain-walk-v1.md` (committed before use). Records: `calibration/fidelity/results/chain-walk-baseline/`.

## Results

Each step's verdict: **R**, reconstructed; **m**, mentioned only; **–**, absent; **✗**, wrong. A `+` marks a reason given.

| Wiki (blind id) | C1 | C2 | C3 | C4 | C5 | C6 | C7 | C8 | C9 | Reconstructed | Breaks | Gate (C4, C5, C6, C9) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| planner and handoff (W1) | m+ | R | m+ | R+ | R+ | R+ | m+ | R+ | R+ | 6 | 5 (second judge: 6) | **passed** |
| explanation depth (W2) | R+ | R+ | ✗+ | R+ | – | m+ | ✗+ | m+ | R+ | 4 | 7 | failed |
| v0.6 LaTeX (W3) | m+ | R+ | – | – | – | m+ | m+ | R+ | ✗+ | 2 | 8 | failed |

**Agreement:** the two independent judges of W1 gave the same verdict on all nine steps and the same answer for all eight links. Their break lists differed by one.

**What the verdicts say:**
- **W1, the planner and handoff run,** reconstructs the counting core on one page, `subset-intersection-construction`:
  - rank equals dimension change, with the source edges' rank-one exception;
  - the budget $s=WD-N+2L$, with $L=3\binom h3^2h^2$;
  - $L/N=100/539$, so $s<WD$.

  `fast-address-interchange` then gives the recurrence to $O(Vu^\tau)$. The chain thins at both ends: the network's actual bank exchange (C1) and the labels' nesting (C3) are only partly given, and the endpoint identity (C7) is only mentioned.
- **W2, the explanation-depth run,** has the ends but not the middle:
  - it explains the network, frames and rank, and reconstructs the recurrence in full;
  - but it describes the labels as nested along every edge, so that every edge drops dimension. The paper has only the central wires decreasing, and the others increasing (C3, wrong);
  - the budget is absent;
  - the endpoint identity is wrong (C7).
- **W3, the v0.6 LaTeX run,** has frames and the recursive calls per edge, but none of the middle: no labels, no rank lemma, no budget, no derivation of $s<Wm$. Its recurrence omits $\tau=1-2^{-50}$ and the check $s/W<m^\tau$ (C9, judged wrong).

## What this shows, and what it does not

**Observed:**
- **The best wiki carries the chain's core, though its plan did not.**
  - **The plan:** the planner and handoff run's plan was made from the introduction, at an unrecorded effort, and assigned none of the $s<Wm$ derivation (2026-10-13-planner-handoff-run).
  - **How the core got onto the pages:** writers' rewrites added that derivation, and the depth rule then sent its steps back to be cited or cut. Round 3's notes were dropped, so they stayed on the page.
  - **What follows:** in this run the chain survived *despite* the pipeline's rules, not because of them.
- **The other two wikis lose the chain in its middle,** where the saving is earned, and one states the labels wrongly.
- **Agreement on the one paired wiki was complete.** That is one pair: the instrument's reliability is not established.

**Interpretation, not shown:**
- The depth rule treats a derivation that enters through a rewrite as elaboration to be cited or cut. When the plan does not own the derivation, that rule works against the central chain. A plan that owns it (high-effort planning, E1) might let writers present it as the page's own content. That is the hypothesis a full run with high-effort planning would test, with this table as its baseline.
- This instrument judges the pages, not a reader. Whether a reader following W1's pages comes away understanding the saving needs the reader-side instruments (simulated readers with held-out goals; people).

## Current State
- `calibration/fidelity/chain-walk-v1.md` walks a document's central chain through a built wiki's pages, step by step, with links and reasons
- On the three LaTeX wikis, the planner and handoff wiki reconstructs 6 of the 9 steps of the central saving and passes the gate; the explanation-depth wiki reconstructs 4, with 2 wrong; the v0.6 LaTeX wiki reconstructs 2
- The verdicts, key and instructions as run are in `calibration/fidelity/results/chain-walk-baseline/`

## Missing
- No chain walk of a wiki whose plan read the whole paper (for example, one planned at high effort)
- One paired judgement only; the instrument's agreement and validity are not established
- No reader-side measure (simulated readers or people) on any of these wikis

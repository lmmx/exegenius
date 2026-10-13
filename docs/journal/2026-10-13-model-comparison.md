# 2026-10-13: Sonnet on planning and auditing, a comparison

2026-10-13-placement-prototype traced the failures of the explanation-depth run to the tasks that decide the wiki's organisation, all done by Haiku:
- **extraction** left out the edge from `butterfly-layer` to `tape-interchange`;
- **the argument task** gave the layer's recursion and the count $s < Wm$ to no page;
- **the audit** passed a cited central bound.

Haiku writers, given an explicit note, derived correctly. The author and ChatGPT asked for a controlled comparison: Sonnet on the planning tasks and on the audit, with inputs fixed, the two judged separately, no code changed, and no full run.

## Method

**Planning.** A new project with the run's `exegenius.toml` and `paper.tex`, driven by the same exegenius binary as the run (no code had changed since 930d40c). Sonnet subagents did, in order:
- the extract task, whose task file was identical to the Haiku run's apart from paths;
- the background task;
- the same calculus correction, word for word, by `background --redo`;
- the background task again;
- the argument task.

Each submitted through the CLI. The Haiku run's answers (tasks 0001, 0003 and 0004 on `full-v0.6-latex-depth`) are the baseline. Neither set of answers was changed.

**Auditing.** The audit task Haiku had been given for the prototype's `butterfly-layer`, identical but for the output path, given to Sonnet twice and to Haiku once more. The question: is the cited central bound, "The layer runs in $O(Vd^{\lambda'})$ tape operations", flagged as standing in for the page's own mechanism?

## Planning

| | Haiku | Sonnet |
|---|---|---|
| concepts extracted | 31 | 49 |
| concepts after the background task | 67, 35 known | 74, 29 known |
| pages presenting a step, first argument | 8 | 30 |
| `butterfly-layer` (Sonnet: `simultaneous-butterfly-layer`) requires | `gaussian-dyadic`, `frame`, `complex-numbers` | `phase-edge-recursion`, `exact-dyadic-guard`, `role-streams` |
| butterfly reaches chunk interchange in the graph | no | yes, through `packed-selected-bit-addition`, which requires `fast-chunk-interchange` |
| the layer's recursion in the argument | not assigned | S3, a step to show, on the layer's page, with sub-steps S3.1 (`phase-edge-recursion`: $H_0^{\otimes D} = b^D\mathcal S_D C^{\otimes D}\mathcal S_D$) and S3.3 ($F(e) \le (s/W)F(e/m) + O((eK)^\tau + 1)$, summed to $O(d^{\lambda'})$) |
| the count $s < Wm$ in the argument | not assigned | S1.4, $s_{\rm b} = W_{\rm b}m - N + 2L_{\rm b} < W_{\rm b}m$, under S1 on `rank-saving`, by the method "sum the dimension changes along the edges" |

**Grounding.**
- Every one of Sonnet's 45 concepts outside standard background carries evidence quotes that submit checked against `paper.tex` word for word.
- The extra concepts are named parts of the paper's proof: the phase network, packed selected-bit addition, the lower-triangular factorization, prime lengths, parameter choices.
- The argument's claims checked here agree with the paper:
  - $H_0 = bSCS$ tensored;
  - the recurrence of §5 and its sum;
  - the rank sum $W_{\rm b}m - N + 2L_{\rm b}$;
  - $(1-2\eta)t_i < s_i \le (1-\eta)t_i$;
  - $\tau = 1 - 2^{-50}$, $c = 2^{-56}$, $\epsilon = 2^{-75}$.

**On these three questions, Sonnet's plan is better, and better where it matters:**
- the layer's mechanism has pages and is the layer page's own step;
- the dependency on chunk interchange is in the graph;
- the derivation of $s < Wm$ has an owner.

**Two consequences to weigh:**
- **A larger run.** 74 concepts against 67, so more pages and tasks.
- **The depth rule loses its selectivity.** 30 of 45 pages present a step, so they explain to two levels. The limit is still a limit, but the distinction between a page's own method and a result it uses would fall almost wholly on the writer's reading of the note.

**Cost, Sonnet only:**
- extract: 199k subagent tokens, 4.2 minutes;
- the two background tasks: 74k and 69k tokens, about a minute each;
- argument: 264k tokens, 8.9 minutes.

The Haiku run's per-task usage was not recorded, so no cost comparison is possible from these records.

## Auditing

| Audit of the same page and task | Flags the cited central bound | Other gaps |
|---|---|---|
| Haiku, prototype run | no | 2 (guard bits, cost of an exact operation) |
| Haiku, repeat | no | 3 (guard bits twice, cost of an exact operation) |
| Sonnet, 1 | no | 1 (guard bits) |
| Sonnet, 2 | yes: "The recursion that produces the exponent is described only vaguely … The cited bound stands in for how the layer's [cost arises]" | 2 (guard bits) |

Sonnet caught it once in two, and Haiku never in two. That is better but not reliable, and two samples each cannot say more. Sonnet's audits took 24 and 29 seconds and about 56k tokens, against 150 and 175 seconds and about 90k tokens for Haiku's.

## Roles by difficulty

The author proposed organising the agents by the difficulty of their tasks, with Opus as a coordinator if Sonnet falls short. ChatGPT set out a hierarchy, as hypotheses to test, not assumptions:
- **Haiku, execution:** bounded writing and edits with ownership, prerequisites and goal explicit.
- **Sonnet, judgement:** extraction, the argument, ownership of derivations, audits of whether a page explains its central claims.
- **Opus, coordination, by escalation only:** failures that span tasks, conflicts between plans, contested ownership, and failures that survive Sonnet.
- **Deterministic checks throughout:** budgets, schema, relations, bindings, completion.

In this design every agent produces an artefact the next can inspect and challenge, and a coordinator works from the graph, the ownership assignments, the pages and the audits.

**What this comparison supports:**
- Sonnet in the planning role, on one sample.
- Sonnet in the auditing role, weakly: 1 of 2.

**What it does not test:**
- the writing role, where Haiku has derived correctly whenever ownership was explicit;
- Opus.

**What would come before routing changes:** ChatGPT asked for a second test on another known failure. For planning, that is a second Sonnet extraction and argument, since three Haiku extractions gave three different graphs. For auditing, it is Sonnet audits of another page whose central claim is cited, such as the run's `recursive-saving`.

## Follow-ups

ChatGPT asked for two follow-ups before any routing decision:
- a second Sonnet planning sample on the identical inputs, checked for the same structure and for regressions;
- two Sonnet audits of another page whose central claim is cited, with success defined before they ran.

It also cautioned that the same paper twice tests repeatability, not generalisation.

**Planning, second sample.** Same procedure, a new project.
- **The size:** 53 concepts extracted, 77 after the background task (28 known), and 34 pages presenting a step. The names differ again: the target is `main-theorem`, against `power-saving-multiplication`.
- **The butterfly layer:** `simultaneous-butterfly-layer` requires `phase-network`, `packed-selected-bit-addition`, `power-saving-recurrence`, `guard-width-exactness` and `fixed-tape-recursion`. It reaches chunk interchange through `packed-selected-bit-addition`, as in the first sample.
- **The argument:**
  - S2, a step to show on the layer's page, by "express the normalized layer through the phase kernel $C$, evaluate $C^{\otimes e}$ recursively";
  - S2.3, $H_0^{\otimes D} = b^D\mathcal S_D C^{\otimes D}\mathcal S_D$;
  - S2.5.2, $F(e) \le (s/W)F(e/m) + O((eK)^\tau + 1)$;
  - S2.5.3, the base-$m$ partition;
  - S1.3.4 on `rank-saving`: "the total loss on decreasing edges is below $N/2$ ($L_{\rm b}/N = 100/539$), so the sum of the edge ranks, $Wm - N + 2L$, is strictly smaller than $Wm$";
  - S1.5.3 on `power-saving-recurrence`: $F_k \le (s/W)F_{k-1} + O(1)$, solved to $O(e^\tau)$.
- **Checked for regressions:**
  - the two interchange concepts, `power-width-interchange` ($e = m^k$) and `address-chunk-interchange` (general $u$), follow the paper's two lemmas and are not duplicates;
  - one edge says little (`rank-saving` requires `big-o`);
  - none of the claims checked disagrees with the paper.
- **The cost:** extract 178k tokens in 3.4 minutes, the two background tasks 69k and 76k, argument 271k in 10.3 minutes.

Both samples give the same structure where it matters, so that much is repeatable. Both also assign steps to most pages (30 of 45, 34 of 49), and both give larger graphs than Haiku's.

**Auditing, second page.** The criteria were written to `proto2/recursive-saving.criteria.md` before the audits ran.
- **The criteria:** a gap succeeds if it targets the page's central claim (the cited $s < Wm$, or the cited use of it, or that the recursion gives interchange its saving), says what the page must establish (how $s/W < m$ becomes a cost below linear, or where $s < Wm$ comes from), and agrees with the paper.
- **Discrimination:** the run's own Haiku audit of the page (task 0162) already met them: "It never says why a bound on the total number of calls yields that saving". So the page discriminates less than intended.
- **The scores:**

| Audit of `recursive-saving` (task 0162) | Meets the criteria |
|---|---|
| Haiku, the run's | yes |
| Sonnet, 1 | no: two real gaps elsewhere, why an edge's rank counts its calls, and $s < Wm$ divided per role |
| Sonnet, 2 | yes, twice: "does not show why three-element subsets with gather and scatter wires give $s < Wm$, nor how the subspace labels yield it", and "does not say how a bound on the number of smaller calls gives the saving" |

Across both pages, Sonnet's audits met the criteria in 2 of 4 and Haiku's in 1 of 3. Sonnet's audits took about 16 to 29 seconds and 53k to 56k tokens each.

## Where the evidence stands

ChatGPT's table of outcomes and decisions:

| Outcome | Decision |
|---|---|
| Planning reproduces the key structure without significant regressions | trial a dedicated Sonnet planner, keeping writers on Haiku |
| Sonnet reliably catches the second page's missing derivation | consider Sonnet for audits |
| Planning succeeds, auditing inconsistent | route planning to Sonnet, and improve audit criteria and validation separately |
| Both inconsistent | make ownership explicit in data and validation before adding model complexity |

The results fall in the third row:
- planning reproduced, on one paper;
- auditing did not separate the models.

**The one open cost of the planner:** with Sonnet's plans, most pages present a step, so the two-level depth limit would apply almost everywhere. The distinction between a page's own method and a result it uses would then rest on the writer's reading of the note.

## Current State
- On `paper.tex` with identical task files, Sonnet's extraction and argument gave the butterfly layer's recursion its own step and pages, connected the layer to chunk interchange through `packed-selected-bit-addition`, and assigned the derivation of $s < Wm$ to `rank-saving`; Haiku's did none of these
- Sonnet's argument assigned steps to 30 of 45 pages, against 8 for Haiku's first argument
- On an identical audit task, Sonnet flagged the cited central bound of `butterfly-layer` in 1 of 2 audits and Haiku in 0 of 2
- A second Sonnet extraction and argument on the same inputs reproduced the butterfly layer's recursion as its page's step, its path to chunk interchange, and ownership of $s < Wm$ on `rank-saving`, under different concept names
- Across `butterfly-layer` and `recursive-saving`, Sonnet audits met the pre-set criteria in 2 of 4 and Haiku audits in 1 of 3
- Every exegenius task goes to the `exegenius-task` or `exegenius-reader` subagent, whose model is the session's

## Missing
- No planning comparison on a second paper
- No routing of task kinds to models
- No Haiku per-task token and time records to compare cost against

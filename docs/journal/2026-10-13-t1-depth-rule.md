# 2026-10-13: T1, would the depth rule have cut the central derivation? No

**The hypothesis.** From the chain-walk baseline: on `subset-intersection-construction`, writers' rewrites added the derivation of the paper's central saving that the plan lacked. The round-3 audit then marked six of its steps "cite or cut a step inside added reasoning". So the depth rule would remove the chain's counting core, and only the end of the run saved it. The author singled this out as the finding to pursue.

**The result.** Applying those notes, as one more round would have, **does not** remove the counting core.
- **Runs:** three independent replays with Haiku writers.
- **What the writers did:** each cited the paper for the marked steps, kept every step, and supplied some of the missing premises.
- **The judging:** a blind chain walk judged C4, C5 and C6 (rank as dimension change, the budget, $L/N<1/2$) reconstructed in all three replays and in the control. One replay improved two further steps.
- **Verdict:** by the preregistered criterion (`calibration/plans/T1.md`, bd104f5), the hypothesis is **not supported**.

## How it was run

**Where:** in the development session, on a copy of the planner-and-handoff project as it ended. The evidence is on wiki branch `131026-exp-t1-depth-rule` (38a6cda, with `RESULTS.md`).

**Raising `rounds` cannot replay a dropped round.** When the last round ends, `submit` clears the rewrite queue. The `status` hint said otherwise; that is corrected in af36d6d.

**So the round-3 notes for the page were restored into the queue** by `tools/restore_notes.py`, in the binary's own wording (`audit.rs`). That gave eight notes:
- six "Cite or cut this step";
- one "The reader cannot follow this step";
- one symbol note.

The round's single "candidate concept" gap was left out, since it would have triggered an extend task first.

**The rewrite and the rebuild:**
- `exegenius next` issued the page's rewrite in each of R1–R3.
- Each went, word for word, to a `general-purpose` subagent on `claude-haiku-5-5` at medium effort. The original run's writers were Haiku `exegenius-task` subagents at an unrecorded effort. These are the deviations stated in the plan.
- All three were accepted.
- **The control (R0)** was rebuilt by the same binary with no rewrite.
- **The comparison:** four wikis, judged blind (seed 20261018) with `calibration/fidelity/chain-walk-v1.md`.

## Results

**The original page is correct where it matters.**
- **The check:** an Opus judge checked its 30 claims against the paper: 21 correct, 9 correct but incomplete, 0 wrong.
- **The derivation holds:** rank as dimension change → $s=Wm-N+2L$ → $L/N=100/539<1/2$.
- **Its six gaps are unstated premises:**
  - $W$ is undefined;
  - the labels' nesting is unstated;
  - only the $N$ sources of $X_a$ have dimension 1;
  - the source of $L$'s count is unstated;
  - why the first $X_a$ edge has no label change is unstated;
  - the rank of a decreasing edge is left implicit.

  These are largely what the round-3 audit flagged. The audit was right that the steps were thin.

**What the rewrites did** (mechanical):

| Copy | Words (original 524) | Marked sentences kept verbatim | `<cite>` marks (original 2) |
|---|---|---|---|
| R1 | 654 | 0 of 6 | 5 |
| R2 | 646 | 0 of 6 | 4 |
| R3 | 643 | 0 of 6 | 5 |

Every marked sentence was rewritten, not deleted. For example, R1 defines the signed change of an edge, then says the changes telescope to $Wm-2N$, with the paper's sentence as a citation. It derives $L=3v^2h^2$ from the invocations and central wires, and concludes $s=(Wm-2N+2L)+N$.

**Chain walk** (blind; R = reconstructed, m = mentioned, ✗ = wrong):

| Wiki | C1 | C2 | C3 | C4 | C5 | C6 | C7 | C8 | C9 | Reconstructed | Gate |
|---|---|---|---|---|---|---|---|---|---|---|---|
| control (R0, no rewrite) | m | R | m | R | R | R | m | R | R | 6 | passed |
| R1 | **R** | R | **R** | R | R | R | m | R | R | 8 | passed |
| R2 | m | R | m | R | R | R | m | R | R | 6 | passed |
| R3 | m | R | m | R | R | R | ✗ | R | R | 6 | failed (C7) |

**R3's C7 "wrong" is not caused by its rewrite.** It rests on `fast-address-interchange.md`, which is byte-identical to the control's. That page says the bit procedure's fields are over $\mathbb F_2$, where the paper has radix-$q$ digits over $\mathbb Z/q^b$ with $q$ an odd prime. The control's judge noted the same error and graded the step "mentioned". The difference is judge variance on a real, pre-existing error that no audit caught.

## What this changes

**The causal story in the baseline entry was wrong.** It read: "the writer reconstructed it, the pipeline then cut it." The accurate story, for this page:
- the plan omitted the counting core;
- the audit's "explain the step" notes led writers to add it in rounds 1 and 2;
- the depth rule's round-3 notes, had they been applied, would have led writers to cite the paper for the deepest steps while keeping and tightening them.

**The audit-and-rewrite loop recovered what the plan lacked.** That is the pipeline working as intended, on this page.

**What does threaten the explanation, on present evidence:**
- **Plans that omit the counting core.** E3: 1–2 plans of 5 reconstruct it even at high effort.
- **Errors no audit catches.** The $\mathbb F_2$ field error above. The explanation-depth wiki's labels shrinking on every edge (chain-walk baseline).
- **Audit variance.** Which gaps a round reports is a sample (2026-10-13-convergence-before-after).

**What was ruled out:** a change to the depth rule. Its stated purpose holds here: it stopped the derivation growing deeper and asked for citations instead.

**Limits:**
- one page, one run's state, three replays;
- general-purpose Haiku writers at medium effort;
- the replay restored the queue by reconstruction, with the candidate-concept gap left out.

The depth rule may still cut elsewhere. This page was the only evidence for the hypothesis, and it does not hold here.

## Current State
- Replaying the planner-and-handoff run's dropped round-3 rewrite of `subset-intersection-construction` three times keeps the central saving's counting core (C4–C6) reconstructed in all three, as in the control; the writers cited the marked steps and kept them
- The original page's derivation of the rank budget is correct (30 claims, 0 wrong), with six unstated premises
- `fast-address-interchange.md` in the planner-and-handoff wiki states the bit procedure's fields are over $\mathbb F_2$, contradicting the paper's $\mathbb Z/q^b$
- `status` no longer tells a user that raising `rounds` after the run fixes the last round's gaps (af36d6d)

## Missing
- No replay of the depth rule on other pages or runs
- No check that catches field and label errors of the kind found in two of the three wikis
- No plan that owns the counting core reliably

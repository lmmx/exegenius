# FB1 (proposed, not launched): where, if anywhere, Fable earns its cost in Exegenius

**Status:** a proposal. **No Fable run is launched until S2 has been judged** and this plan has been completed with S2's results. The author then gets a recommendation and a cost estimate before anything is spent.

## S2's result, and what it changes (added after J9; nothing below was launched)

**What S2 found** (`docs/journal/2026-10-14-s2-model-effort.md`), on E6's extraction F1:
- all five plans pass the central gate by majority;
- Opus `xhigh`, Sonnet `max` and Opus `max` each gain C1 and 3–4 faithful derivations over Sonnet `high`;
- Opus `max` reaches the rubric's ceiling (19 of 19);
- Sonnet `xhigh` is 2 below Sonnet `high`.

**By the table under "How S2 will be used to choose":**
- the second row applies: Opus beats Sonnet at matched effort (`xhigh`: 18 against 13);
- the first applies only in part: Sonnet gains at `max`, not at `xhigh`.

So the argument stage looks capability-limited, and Fable on the argument task is the best-motivated pilot, as this plan said.

**But the pilot as written below cannot work.** It ran Fable on F1 against S2's best configuration. On F1 that configuration is at the ceiling, so a Fable run could at most tie, and would cost a Fable run and a judging batch to learn nothing.

**The revision:**
- The pilot runs on an extraction where the best available configuration is still below the ceiling.
- Which extraction that is, if any, is what `S3.md` (Opus `high` and `xhigh` on F4 and F8, where Sonnet `high` is weakest) decides. S3 runs first.
- S3's table sends FB1 to one of three places:
  - an extraction Opus `xhigh` does not lift;
  - the extraction stage;
  - a second document, if this paper's argument task is solved.
- The comparison, judging, success criterion and stopping rule below stand. The comparison is against Opus `xhigh` (or whatever S3 shows best) on the same extraction, re-judged in the pilot's batch.

**After S3** (`docs/journal/2026-10-14-s3-opus-floor.md`):
- **Opus `xhigh` falls short of the ceiling** on F4 (13) and F8 (16).
- **F4:** the table's extraction-limit row fired there, but the judges' notes show the gaps are depth on the paper's second construction and nested error chains, not the extraction.
- **F8:** the remaining argument-task candidate. Opus `xhigh` misses three results there, two of which Opus `max` derived on F1.
- **So the next step proposed is an Opus `max` run on F8 first** (about 35–40 M cache read with judging).
  - If Opus `max` reaches the ceiling there, the argument task on this paper has no headroom left for Fable. FB1 then moves to a second document, or to auditing the key step.
  - If it does not, the pilot below runs on F8, against Opus `max`.

**Cost estimate for the pilot after S3,** per extraction. These are components, not converted:
- **The Fable run:** unknown until measured. S2's Opus `max` run (15.9 M cache read, 1.1 M cache write, 48 k visible output, 2,021 s) is the nearest known figure, as a placeholder at `max` effort. At `high` it is probably less.
  - Fable's price per token is higher than Opus's.
  - How the usage meter weighs it is not published.
- **The judging:** about 27 M cache read and 2.2 M cache write, on Opus. That is the Fable plan, the best configuration's plan and the reference, at three judges each, plus two anchors: 11 judgements at J9's measured rate.

**The question:** is there one stage of Exegenius where `claude-fable-5-1` is enough better than the strongest existing configuration to justify its much higher usage? Fable is treated as a hypothesis about a specific capability, not a candidate default.

## Stages, and what is known about each

| Stage | Strongest known configuration (evidence) | Where a stronger model could matter | Priority |
|---|---|---|---|
| Argument (key-step development) | Sonnet `high` with the E7 instructions: central gate 8 of 8, C4 8 of 8 (E7, J8, single judges) | C4-type steps, where the plan states the conclusion but not the identity that makes it true. E7's sentence closed most of this gap on one paper; correctness of deep derivations (E4's errors) remains the risk | high, but the bar is now high too |
| Extraction | Sonnet `high` (E1, E3: 19 of 19 core results stated) | stating is saturated on this paper; deriving and faithfulness are not | medium |
| Writing and rewrites | Haiku writers in the only full runs; T1 showed they keep derivations when audits ask | unknown: no full wiki has used the E7 plans | unknown until a full run |
| Auditing | Haiku in full runs; audit variance is large (2026-10-13-convergence-before-after) | catching real errors that audits miss (the F_2 field error, the shrinking labels) | medium; needs an error-detection measure first |

## How S2 will be used to choose

| What S2 shows | Implication for FB1 |
|---|---|
| Higher effort (`xhigh`, `max`) adds fidelity over `high` | test Fable at a lower effort on the same argument task; the question becomes whether a stronger model buys the gain more cheaply |
| Opus beats Sonnet at matched effort | the argument stage is capability-limited; Fable on the argument task is the best-motivated pilot |
| Extra effort adds tokens but not fidelity | be sceptical of an expensive Fable configuration; consider instead its use as an **auditor of the plan's key step** (correctness, not depth), the risk E4 exposed |

These are rules for choosing an experiment, not predictions of Fable's behaviour.

## The pilot, once chosen

- **Inputs:** one run on the same input as S2 (E6's extraction F1, the E7 argument task), or on the stage chosen above.
- **The comparison:** against S2's best configuration, not against Sonnet `high` by default.
- **Judging:** blind, three judges per plan, with each step decided by majority; the reference plan is judged in the same batch.
- **Cost:**
  - recorded through `tools/spend.py`;
  - set against `calibration/budget.md`;
  - downstream effects included where they can be seen, such as fewer submit rejections or retries.
- **The success criterion,** to be fixed before the run, as a minimum gain over the best configuration on central steps by majority and on faithful derivations, with no added major errors.
- **The stopping rule:**
  - One run is a screen.
  - A second run happens only if the first clears the criterion or falls within judge noise of it.
  - Fable is never called better on one run.
- **The budget:** an estimate in tokens for the run and its judging, using S2's measured Opus `max` run as the nearest known cost. Fable's own cost is unknown until its first run is measured.

## The pilot, preregistered for runbook item 5 (14:28 UTC, before S4's result)

**Trigger:** only on S4's outcome "improves, not the ceiling" (incumbent: S4F8OM) or "does not improve" (incumbent: S3F8OX). Not on "reaches the ceiling".

**The run:**
- one run, FB1F8, on a fresh copy of E6's F8 with the E7 instructions (91c4fed);
- `general-purpose`, `model: "fable"`, `effort: "high"`, the prompt word for word;
- **why `high`:** the effort rule above, since S2 showed gains from effort as well as model, and the question is whether the stronger model buys them more cheaply.

**Judging:** the Fable plan and the incumbent's plan, re-judged together. Primary: three judges each, plus anchors (8). Checklist: two judges each (4).

**Fable beats the incumbent** if:
- its checklist total is above the incumbent's beyond judge noise;
- with no loss on the primary measures (the gate by majority, central count, faithful range not entirely below, no major error);
- and no `max_tokens` stop.

**It reaches the ceiling** if, in addition, its faithful median is ≥ 18 and each checklist total is ≥ 24.

| Outcome | Action |
|---|---|
| beats | Fable earns a test on a second document and at the extraction stage. Reported to the author with its measured cost |
| ties or loses | Fable is not worth its cost for argument planning on this paper; recorded |

**Budget:**
- **The run:** unknown. Placeholder 6–16 M cache read, at Fable's rates.
- **Judging:** ~27 M (primary) and ~2.5 M (checklist), on Opus.

**Stopping:** one Fable run, then a report, before any further Fable spend.

### Pilot ledger

| Run | Configuration | Run dir | Status |
|---|---|---|---|
| FB1F8 | Fable, `high` | `e6/FB1F8` | launched 15:01 UTC on S4's branch "improves, not the ceiling" (incumbent S4F8OM); task file identical to S4F8OM's apart from the path |

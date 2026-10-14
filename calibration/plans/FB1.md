# FB1 (proposed, not launched): where, if anywhere, Fable earns its cost in Exegenius

**Status:** a proposal. **No Fable run is launched until S2 has been judged** and this plan has been completed with S2's results. The author then gets a recommendation and a cost estimate before anything is spent.

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

# 2026-10-14: S3, Opus at high and xhigh where Sonnet high is weakest

**The result, by the preregistered table (`calibration/plans/S3.md`, criteria in ba2878b): "`xhigh` lifts both; `high` does not suffice."**

On E6's extractions F4 and F8, where Sonnet at `high` with the E7 instructions was weakest:

| Extraction | Configuration | Central (majority) | Gate | Faithful, median (each judge) | Derived (median) | Major errors |
|---|---|---|---|---|---|---|
| F4 | Sonnet `high` (E7F4W, re-judged) | 8 (C1 mentioned) | pass | 7 (7, 4, 7) | 14 | 0 |
| F4 | Opus `high` | 8 (C1 mentioned) | pass | **14** (13, 14, 16) | 19 | 0 |
| F4 | Opus `xhigh` | **9** | pass | **13** (13, 13, 14) | 19 | 0 |
| F8 | Sonnet `high` (E7F8W, re-judged) | 8 (C1 mentioned) | pass | 7 (7, 7, 6) | 15 | 0 |
| F8 | Opus `high` | 9 | pass | 9 (8, 10, 9) | 16 | 0 |
| F8 | Opus `xhigh` | **9** | pass | **16** (16, 16, 16) | 18 | 0 |

**Applying the reading as fixed** ("beyond judge noise": the two plans' ranges of three judges' faithful counts do not overlap):
- **Opus `xhigh` lifts the floor on both extractions.** It passes the gate, has no major error, and its faithful count is above the reference's beyond noise:
  - F4: 13–14 against 4–7;
  - F8: 16 against 6–7.
- **Opus `high` does not suffice.**
  - On F4 it is within noise of `xhigh` (13–16 against 13–14).
  - On F8 it is far below (8–10 against 16), though still above the reference (8–10 against 6–7).
- **"The extraction is the limit" (both Opus configurations below the ceiling, missing the same core results):**
  - **F4: met as written.** `high` and `xhigh` both fail to derive five core results faithfully, the same five, with `xhigh` missing one more (rank-saving).
  - **F8: not met.** `high` misses 11 and `xhigh` 3.

**What the table then prescribes:**
- **The row:** propose Opus `xhigh` for the argument task.
- **Fable:** gets an argument-task pilot only on an extraction where Opus `xhigh` still falls short of the ceiling. F4 is excluded by the extraction-limit row, which points FB1 there to the extraction stage. F8 remains: Opus `xhigh` is 3 faithful derivations short of 19.

## The judges' notes contradict the causal label on F4

The extraction-limit row was written on the assumption that, if two configurations miss the same results, the input is what limits them. The judges' notes on the five results both Opus plans miss on F4 do not support that.

| Core result | What both plans leave out (judges' notes, F4) |
|---|---|
| bank-exchange-network | the complex network's coefficient check; only the bit case is given |
| label-residual-budget | the gate-label table and the edge-by-edge residual table |
| complex-phase-interface | the endpoint step behind the sign corrections |
| simultaneous-layer | the depth recurrence $A(e)\le sA(e/m)+E$ behind the guard width |
| exact-recovery | the ring and chirp error chain and the disk margins behind $E_s$ |

**Two kinds of gap:**
1. **The paper's second construction:** the complex network, set in parallel with the bit network.
2. **A nested error or guard chain,** which the plan asserts instead of deriving.

**Why the extraction is not the cause:**
- The argument task reads the whole paper. None of these gaps is about something missing from the extraction.
- The same results recur as misses on other extractions and configurations:

  | Result | Missed elsewhere by |
  |---|---|
  | complex-phase-interface | Opus `xhigh` on F8; Sonnet `high` on F1 |
  | label-residual-budget | Opus `xhigh` on F8; Sonnet `high` on F1 |
  | simultaneous-layer | Opus `xhigh` on F1 (2 of 3 judges) |

- Only Opus `max`, on F1, derived all of them.

So the limit on F4 looks like depth on a particular kind of step, not the input. **The prescribed extraction-stage pilot is not supported by the evidence that triggered it.** I report the rule's outcome as written and do not act on it.

## Across S2 and S3: Opus `xhigh` against Sonnet `high`, three extractions

| Extraction | Sonnet `high` faithful (batch) | Opus `xhigh` faithful (batch) | Gain | Central |
|---|---|---|---|---|
| F1 | 15 (J9) | 18 | +3 | 8 → 9 |
| F4 | 7 (J10) | 13 | +6 | 8 → 9 |
| F8 | 7 (J10) | 16 | +9 | 8 → 9 |

**What this supports:**
- **A consistent gain:** three of three extractions, each beyond judge noise within its own batch, with C1 gained every time and no major errors. The exact one-sided sign test on three of three gives only p = 0.125. The support is the size and consistency of the gain, not the test.
- **The gain is largest where Sonnet `high` was weakest.** Opus `xhigh`'s worst faithful count (13) is near Sonnet `high`'s best in E7 (17, J8).

**What it does not support:**
- **A claim about Opus `xhigh`'s own run-to-run variance:** there is one run per extraction.
- **Anything beyond this paper.**

**Opus `high` is uneven:**

| Extraction | Opus `high` | Opus `xhigh` |
|---|---|---|
| F4 | 14 | 13 |
| F8 | 9 | 16 |

On one run per extraction, that unevenness cannot be told apart from run-to-run variation. It is enough that `high` cannot be recommended in `xhigh`'s place.

## Judge agreement and anchors (J10)

- **Central steps:** across the six compared plans, a majority on every step.
- **Splits:** the references and Opus `high` on F4 drew individual central counts of 7–9 (E7F4W 9, 8, 7). The four Opus plans' gate verdicts were unanimous.
- **Faithful ranges:** at most 3, on E7F4W (4–7) and Opus `high` on F4 (13–16).
- **Drift on re-judged plans:**
  - E7F4W was central 7 with C7 mentioned in J8 (one judge) and 8 here by majority, with C7 reconstructed. Its gate passed in both, since C7 is not a gate step.
  - Its faithful count was 7 in J8 and 7 here.
  - E7F8W's faithful count was 5 in J8 and 7 here.
- **The anchors:**
  - E3-M3 held at central 1.
  - E1-P4 moved from 6 (J6–J9) to 7, with faithful 13 (14 in J8 and J9). This is within the drift already recorded for it in `calibration/capability-map.md`.

## Cost, measured

| Run | Calls | Seconds | Cache read | Cache write | Visible output | Thinking signatures |
|---|---|---|---|---|---|---|
| Sonnet `high`, F4 (E7) | 19 | 438 | 2.64 M | 0.24 M | 15 k | 123 k chars |
| Opus `high`, F4 | 13 | 696 | 1.49 M | 0.44 M | 20 k | 165 k chars |
| Opus `xhigh`, F4 | 28 | 962 | 4.02 M | 0.45 M | 23 k | 246 k chars |
| Sonnet `high`, F8 (E7) | 21 | 576 | 2.83 M | 0.25 M | 19 k | 167 k chars |
| Opus `high`, F8 | 16 | 517 | 2.16 M | 0.19 M | 16 k | 113 k chars |
| Opus `xhigh`, F8 | 23 | 1,034 | 2.91 M | 0.44 M | 24 k | 273 k chars |

**Against Sonnet `high` on the same extraction, Opus `xhigh` took:**

| | Seconds | Cache read |
|---|---|---|
| F4 | 2.2× | 1.5× |
| F8 | 1.8× | 1.0× |
| F1 (S2) | 2.0× | 2.3× |

Opus `xhigh`'s S2 run on F1 (6.93 M) was the most expensive of the three.

**Estimates against actuals** (`calibration/plans/S3.md`):

| | Estimated | Measured |
|---|---|---|
| Runs, cache read | ~20–28 M | **10.6 M** |
| Runs, cache write | ~2.7–3.7 M | **1.52 M** |
| Judging, cache read (20 judgements) | ~50 M | **59.6 M** (2.98 M each, against J9's 2.49 M) |
| Judging, cache write | ~4 M | **3.27 M** |
| **S3, cache read** | **~70–78 M** | **70.2 M** |
| S3, cache write | ~7–8 M | 4.79 M |

The run estimate was too high because it scaled from S2's F1 run, which was the most expensive Opus `xhigh` run. **Judging was 85% of S3's cache read.**

**The meter** read 14% at 12:28 UTC, just before launch. No reading has been taken since, so S3's cost on the meter is not yet known.

## The evidence-backed next step

The table's literal prescription is a Fable argument pilot on F8, against Opus `xhigh`. **The evidence points to one cheaper step first.**

**Why:**
- On F8, Opus `xhigh` falls short on complex-phase-interface, label-residual-budget and chirp-permutation-cancellation.
- On F1, Opus `max` derived the first two of these faithfully, and all of S2's configurations except Sonnet `xhigh` derived the third.
- If Opus `max` closes the gap on F8 too, a Fable run there can at most tie, and this paper's argument task needs no stronger model.

**The proposal:** an Opus `max` run on F8, judged in one batch with Opus `xhigh`'s F8 plan (S3F8OX), three judges each, plus the anchors (8 judgements).

| If Opus `max` on F8 | Then |
|---|---|
| reaches the ceiling, beyond noise above Opus `xhigh` | Fable's argument pilot on this paper is not worth its cost; FB1 moves to a second document or to auditing the key step |
| does not | Fable's pilot runs on F8 against Opus `max` |

**The estimate, on S2's and S3's measured components:**

| | Basis | Cache read | Cache write |
|---|---|---|---|
| Run | S2's F1 Opus `max` run: 15.9 M, 1.1 M, 2,021 s | ~10–16 M | ~1 M |
| Judging, 8 judgements | J10's per-judgement figures | ~24 M | ~1.3 M |
| **Total** | | **~35–40 M** | |

That is about half of S3's measured cache read.

**An alternative lever: an instruction.** The shared gaps are of two kinds: the second construction treated in parallel, and nested error chains asserted. That is the same kind of gap E7's sentence closed for counted steps. A sentence asking for "every variant the sources treat in parallel" and for "the chain behind each error or guard bound" could close it on a cheaper model. That would be an E-series experiment on the existing extractions. It is not proposed here, only noted.

**The pipeline default is unchanged.** The table's outcome is a proposal of Opus `xhigh` for the argument task. Whether three extractions on one paper are enough to change a default is the author's call.

## Current State
- Opus `xhigh` with the E7 instructions beats Sonnet `high` beyond judge noise on F1, F4 and F8, with C1 gained each time and no major errors; the gain grows where Sonnet `high` is weakest (+3, +6, +9 faithful)
- Opus `high` is uneven across F4 and F8 and does not suffice
- The extraction-limit row of S3's table fired on F4, but the judges' notes point to depth on second constructions and nested error chains, not to the extraction
- S3 cost 10.6 M cache read in runs and 59.6 M in judging, inside the 70–78 M total estimated

## Missing
- No meter reading after S3, so its cost against the meter is unknown
- No repeat run of any Opus configuration on the same extraction, so its run-to-run variance is unmeasured
- No Opus `max` run on F4 or F8
- No second document

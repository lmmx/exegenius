# 2026-10-14: K1, the mechanism checklist validated

**The result, by the preregistered table (`calibration/plans/K1.md`, committed in 357c335 before any checklist judgement): criteria 1, 2 and 3 are met, so the checklist is adopted as a preregistered secondary measure for the next experiment. Criterion 4 shows resolution at the top, so it may break ties at the ceiling.**

| Criterion | Required | Observed | Met |
|---|---|---|---|
| 1. Validity against the earlier majority faithful verdict (66 pairs of plan and result) | completeness higher for faithful pairs; AUC ≥ 0.80 | 0.85 against 0.33; **AUC 0.957** | yes |
| 2. First judge against the markers (275 pairs) | ≥ 0.80 agreement | **0.836** | yes |
| 3. Second judges on the three top F1 plans (78 sub-steps) | ≥ 0.80 agreement, totals within 3 | **78 of 78**; totals identical | yes |
| 4. Resolution at the top (reported) | any of the F1 top three separated beyond judge disagreement | 24, 21 and 19, against a judge disagreement of 0 | resolves |

**What it costs:**
- **Judging:** 14 checklist judgements, 8.8 M cache read and 1.46 M cache write. The estimate was ~20–35 M and ~2–3 M. A checklist judgement measured 0.63 M cache read, against J10's 2.98 M for a full judgement.
- **Runs:** none.
- **Markers:** free.

## Design

**What was judged:** eleven existing plans, with no new runs. J9's five on F1, and J10's six on F4 and F8.

**How:**
- The plans were judged blind in batch J11 (seed 20261025), one checklist judgement each. The three top F1 plans had a second judge.
- The judges were Opus, as in J9 and J10.
- They were given only:
  - the checklist (`calibration/rubrics/nlogn-mechanism-checklist.md`: 26 sub-steps of six core results, read from the paper's derivation lines);
  - the paper;
  - the plan.
- They saw no reference pack and no earlier verdict.

**Before any verdict:**
- the markers (`tools/markers.py`) were run;
- the analysis script (`tools/checklist.py`) was written and committed (6974564).

## Results (`judge/results.txt`)

| Extraction | Plan | Checklist (of 26) | B L P R M S | Faithful, earlier (of 19) | Markers (of 25) |
|---|---|---|---|---|---|
| F1 | Sonnet `high` (E7F1W) | 13 | 4 3 0 2 3 1 | 15 | 16 |
| F1 | Sonnet `xhigh` | 15 | 4 2 1 2 2 4 | 13 | 18 |
| F1 | Opus `xhigh` | **19** [19] | 4 3 3 4 4 1 | 18 | 19 |
| F1 | Sonnet `max` | **21** [21] | 4 3 4 2 4 4 | 18 | 21 |
| F1 | Opus `max` | **24** [24] | 4 4 4 4 4 4 | 19 | 23 |
| F4 | Sonnet `high` (E7F4W) | 5 | 0 1 1 2 1 0 | 7 | 6 |
| F4 | Opus `high` | 15 | 3 2 2 3 4 1 | 14 | 17 |
| F4 | Opus `xhigh` | 15 | 3 2 2 3 3 2 | 13 | 17 |
| F8 | Sonnet `high` (E7F8W) | 4 | 0 1 0 2 0 1 | 7 | 8 |
| F8 | Opus `high` | 6 | 0 0 2 2 1 1 | 9 | 8 |
| F8 | Opus `xhigh` | **19** | 4 3 2 4 3 3 | 16 | 22 |

Brackets give the second judge's total. B, L, P, R, M and S are the six results: bank-exchange-network, label-residual-budget, complex-phase-interface, rank-saving, matrix-shear and simultaneous-layer.

**The orderings J10 established beyond noise are kept:**
- Opus `xhigh` is above Opus `high` on F8 (19 against 6).
- Opus `xhigh` is above Sonnet `high` on F4 (15 against 5) and on F8 (19 against 4).
- Opus `high` and Opus `xhigh` tie on F4 (15 and 15), as they did within noise in J10.

**Criterion 1, by result:** the AUC is 1.00 for four of the six results, 0.92 for label-residual-budget, and **0.71 for rank-saving**. Rank-saving is the weakest because its sub-steps R2 and R3 are exact constants ($\eta_{\rm b}$, $\eta_{\rm c}$), which the earlier judges did not require for a faithful verdict.

**Criterion 2, by direction:**
- 32 of the 45 disagreements are a marker present where the judge found the sub-step absent: content written without the role it plays.
- They are concentrated in L2 (6 of 11 agree), P2 (4 of 11), P3 (4 of 11) and S4 (5 of 11).
- These four markers over-detect and should be tightened before the markers serve as a cross-check.

## What the checklist adds, and what it does not

**It separates the top of F1, where the faithful count could not.**

| Configuration | Checklist | Faithful |
|---|---|---|
| Opus `max` | 24 | 19 |
| Sonnet `max` | 21 | 18 |
| Opus `xhigh` | 19 | 18 |

**Where the differences lie:**
- **simultaneous-layer:** Opus `xhigh` has 1 of 5 sub-steps. It has the final truncation (S5), but not the constants, the depth recurrence, its unrolling or the bit reservation (S1–S4). Opus `max` has 4 (missing S3) and Sonnet `max` 4 (missing S1).
- **label-residual-budget:** Opus `max` alone derives the orthonormal bases over $\mathbb F_2$ (L4). None of the three has the residual table (L1).
- **complex-phase-interface:** Opus `xhigh` lacks the $k$-column tensoring (P3). Sonnet `max` and Opus `max` have all four.
- **rank-saving:** Sonnet `max` lacks the exact deficits $\eta_{\rm b}$, $\eta_{\rm c}$ (R2, R3).

**This revises one reading of S2, as an exploratory finding:**
- **S2's preregistered reading stands:** Opus `max` was within judge noise of Opus `xhigh` on the faithful count.
- **The checklist is a new measure applied after the fact.** On it, Opus `max` is 5 sub-steps ahead of Opus `xhigh` on F1, with two judges agreeing exactly on each.
- **The cost of that:** 2.3× the cache read on F1.

**Its reliability is not established by three plans:**
- The second judges agreed on every sub-step of the three top plans. Those plans have many sub-steps present, which may be easier to agree on than mid-range plans.
- With a judge disagreement of 0 on three plans, criterion 4 is met easily. The true noise floor is above zero and is not measured.

**It is narrower than the content judge's rubric.** It covers six of the 19 core results, so it can rank plans differently where they lose points elsewhere.
- On F1, Sonnet `xhigh` scores 15 and Sonnet `high` 13 on the checklist, the reverse of the faithful count (13 against 15).
- Sonnet `xhigh`'s losses (common-frame-identity, chirp-permutation-cancellation) are outside the six, and it does derive the simultaneous-layer chain.
- **The checklist is a complement, not a replacement:** the existing rubric stays primary.

## The hold-out (reported only)

On J8's sixteen plans, which K1's judges never saw, the markers alone predict the earlier faithful verdict:

| | AUC |
|---|---|
| All six results | **0.77** |
| bank-exchange-network | 1.00 |
| simultaneous-layer | 0.97 |
| complex-phase-interface | 0.73 |
| rank-saving | 0.68 |
| matrix-shear | 0.68 |

label-residual-budget had no faithful plan in J8, so it has no AUC.

## What it means for the next experiment

The next experiment is the Opus `max` run on F8 (`docs/journal/2026-10-14-s3-opus-floor.md`). It can now be read on a scale with room at the top:
- Opus `xhigh` scores 19 of 26 on F8.
- Opus `max` on F1 scored 24.

**The preregistration for that run should name:**
- **the primary measures,** unchanged: central steps by majority, and faithful derivations;
- **the checklist as the secondary measure,** with one judge per plan and a second judge on the two plans being compared;
- **the gain that counts,** fixed before the run: a checklist total beyond both plans' judge disagreement, with no loss on the primary measures.

**Its cost estimate, revised with J11's measured rate:**

| | Cache read |
|---|---|
| The run | ~10–16 M |
| Primary judging (8 judgements) | ~24 M |
| Checklist judging (4 judgements) | ~2.5 M |
| **Total** | **~37–43 M** |

## Current State
- The mechanism checklist meets K1's three validity criteria (AUC 0.957 against earlier faithful verdicts, 0.836 agreement with markers, 78 of 78 between judges) and resolves the F1 top three (24, 21, 19)
- It is adopted as a preregistered secondary measure; the content judge's rubric and the central gate stay primary
- On it, Opus `max` is 5 sub-steps ahead of Opus `xhigh` on F1, mostly on simultaneous-layer; this is exploratory, since S2 was preregistered on the faithful count
- J11 cost 8.8 M cache read, about a third of the low end of its estimate

## Missing
- The markers L2, P2, P3 and S4 over-detect and need tightening
- The checklist's judge noise on mid-range plans is not measured (only the top three plans were double-judged)
- The checklist covers six results; whether a similar one is needed for others is open

# Calibration programme: current state and next actions (for resuming without this session's context)

**Updated:** 2026-10-15 05:30 UTC. Read this file, then `RUNBOOK.md` §8 ("Resuming"), before acting. Times are UTC and taken from `date -u`.

## Standing instructions from the author (latest, about 01:22–01:45 UTC)

- Proceed autonomously; do not ask the author to reconfirm decisions already made. Escalate only at a real budget or permission boundary.
- **Budget:** decisions use the actual quota meter, never cache-read tokens.
  - Last confirmed reading: **45% used, at about 01:22** (the earlier "40%" is superseded).
  - Runbook §5's ~300 M-since-a-reading pause is a separate operational safeguard, not a substitute for the meter.
  - If planned calls would cross it, or the current budget cannot be established with adequate confidence, pause at a clean checkpoint and ask for a fresh reading.
  - The quota resets at about **04:40 UTC** (the author: "in about 3 hours" at about 01:40). A check-in fires at 04:45.
- **Permissions:** never bypass or work around the permission classifier. If it blocks an action, leave it blocked and report the exact reason.
- **Binaries:** never rebuild or replace a shared binary while runs are active. If a rebuild is needed, use an isolated binary and verify which executable each run invokes.
- **Records:** keep timestamps grounded in the clock. Preserve preregistered criteria. Report results separately from interpretations.
- **When blocked:** do not spin or retry. Wake, verify the quota state, then continue within budget and permissions.

## Experiments in flight

### E11 (`plans/E11.md`): **complete at 04:58 UTC**

- **Result:** "O not shown better". O was favoured on F4 and F8, S on F2 and F5, and F3 and F6 tied (p = 0.69).
- **Control:** two Sonnet runs on F4 are not tied (net +2).
- **Not run:** the cross-check (O was not favoured) and F3's second judges (skipped, since no outcome could change).
- **Recorded:** the result is in E11.md, `policy.md` and the registry.

### A1 (`plans/A1.md`): stage 1 complete at 05:17 UTC; stage 2 triggered and held

- **Result (preregistered):** no configuration is adequate. R_A is AH 0.46, AS 0.29 and AO 0.39, against 0.50; R_S is 0.60 for all three, against 0.70. F_A, F_sham and F_S are within their thresholds.
- **Classifier check:** `ASSERTS` agrees with the blind hand labels on 0.885 of gaps, so it stays. It finds 35 of 78 asserting notes. Under the hand labels (a sensitivity analysis), R_A is 0.64, 0.50 and 0.71, all passing.
- **Recorded:** `plans/A1.md` "Result, stage 1"; `policy.md`; `registry/specs/A1.json`; `docs/journal/2026-10-15-a1-stage1/`.
- **For the author:**
  1. Do the hand labels replace `ASSERTS`? This would change a criterion after results.
  2. Is the inclusive reading of the stage-2 trigger for R_S right?
  3. A fresh meter reading before stage 2.
- **Stage 2, when cleared:**
  - the copies for run 2 are made (05:30): `scratchpad/a1/runs2/`, launch order in `scratchpad/a1/queue2.tsv`. If the author clears AH only, launch only the `A1AH2` rows;
  - launch as stage 1 (up to 20 at a time, `general-purpose`, model and effort from `prompts.tsv`);
  - collect with `a1collect.py`; score pooled; hand-label the new gaps under `labels-rule.md` before unblinding;
  - read by the table.
  - Estimated cost: about 23 M cache read for the tasks if all three run, plus orchestration.

### W3 (`plans/W3.md`): writer validation, **preregistered at 05:31 UTC; not launched**

- **Pages:** key-step 0018, 0021, 0031, 0032, 0033 and 0036; controls 0005, 0006 and 0010.
- **Runs:** Sonnet `high` against Opus `xhigh`, one run per page, judged by J26 (J22's page judge, three judges per page).
- **Estimate:** about 70–140 M cache read.
- **Launch needs:** the author's order (relative to A1 stage 2) and a fresh meter reading.

## Policy state (`policy.md`)

| Stage | State |
|---|---|
| Planning | Sonnet `high` with the combined instruction (E10), for this paper. E11 tests whether Opus is better above the ceiling |
| Writers | Sonnet `high` is the general default. A provisional exception for this wiki covers 0015 and 0016 (W2b). The blind designation of 23 key-step pages is a classification only, and no rollout follows from it (W3 is pending) |
| Judging | the primary rubric below the ceiling; pairwise (K3) above it. The identical-pair control is not informative |
| Extraction | keep Sonnet `high` (EX1 analysis) |
| Auditing | A1 stage 1: no configuration adequate; the audit verdict depends on the classifier; stage 2 held for a meter reading |

## Records still to do (model-free)

- ~~E11's evidence branch, registry spec and branch inventory~~: done at 01:53–01:55 (`141026-exp-e11-ceiling` b02b658; `registry/specs/E11.json`).
- J25's batch entry in `registry/pairwise-judging.json`, and E11's result in `plans/E11.md`, `policy.md` and a journal entry.

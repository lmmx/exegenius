# Calibration programme: current state and next actions (for resuming without this session's context)

**Updated:** 2026-10-15 05:00 UTC. Read this file, then `RUNBOOK.md` §8 ("Resuming"), before acting. Times are UTC and taken from `date -u`.

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

### A1 (`plans/A1.md`): planted-error recall for fidelity auditing

- **State:** preregistered at 01:18. The 132 task copies are made and checked in `scratchpad/a1/runs/A1{AH,AS,AO}1/`. `prompts.tsv` lists description, model, effort, root and prompt for each.
- **Gate:** E11 is complete (04:58) and the reset has passed. Measured since 04:40: 94.2 M, against A1's ~65–80 M and runbook §5's ~300 M safeguard. The session cannot read the meter.
- **Then:**
  - launch from `prompts.tsv`, up to 20 at a time, `general-purpose`, with `model` and `effort` as listed;
  - collect the answers into the plan's layout;
  - score with `plant.py score`;
  - run the classifier check;
  - read the result by A1's table.

### W3 (`plans/W3-draft.md`): writer validation

- A draft proposal on new seeded pages: key-step 0018, 0021, 0031, 0032, 0033 and 0036; controls 0005, 0006 and 0010.
- It is preregistered only after E11 completes and the budget allows, and launched in the order the author sets.

## Policy state (`policy.md`)

| Stage | State |
|---|---|
| Planning | Sonnet `high` with the combined instruction (E10), for this paper. E11 tests whether Opus is better above the ceiling |
| Writers | Sonnet `high` is the general default. A provisional exception for this wiki covers 0015 and 0016 (W2b). The blind designation of 23 key-step pages is a classification only, and no rollout follows from it (W3 is pending) |
| Judging | the primary rubric below the ceiling; pairwise (K3) above it. The identical-pair control is not informative |
| Extraction | keep Sonnet `high` (EX1 analysis) |
| Auditing | unmeasured (A1 pending) |

## Records still to do (model-free)

- ~~E11's evidence branch, registry spec and branch inventory~~: done at 01:53–01:55 (`141026-exp-e11-ceiling` b02b658; `registry/specs/E11.json`).
- J25's batch entry in `registry/pairwise-judging.json`, and E11's result in `plans/E11.md`, `policy.md` and a journal entry.

# Calibration programme: current state and next actions (for resuming without this session's context)

**Updated:** 2026-10-15 09:27 UTC. Read this file, then `RUNBOOK.md` §8 ("Resuming"), before acting. Times are UTC and taken from `date -u`.

## Checkpoint, 09:27 UTC, 2026-10-15 (read first)

**The objective (the author, 09:24):** a complete, defensible configuration policy for every pipeline task: extraction, background, argument planning, writing and fidelity auditing. Finishing the queued experiments is not the goal in itself. The ledger is `policy.md`, "Decision ledger". Run only experiments that can change a configuration decision.

**Quota:**
- The author's reading at about 09:24: **20% remaining** (80% used), with the reset at about 09:44.
- Nothing was launched on it: W3 cannot finish before the reset, and A1 stage 2 would not change a decision.
- A check-in is scheduled for 09:52 (`trig_01SpgYi2xwX2SFduvWJ6jE5h`).
- After the reset, there is no reading until the author gives one. Do not infer the meter from tokens.

### Next actions after reset, in order

1. **Get a fresh meter reading,** or use one the author has posted after 09:44. If none, ask once, and do only model-free work meanwhile.
2. **W3** (`plans/W3.md`, preregistered 05:31): the writer routing question, the one queued experiment that can change a decision.
   - Expected cost: ~70–140 M cache read; about an hour (runs of 15–25 min, 20 at a time, then J26's 54 page judgements).
   - Setup (model-free; not yet done): render the nine write tasks (pages 0018, 0021, 0031, 0032, 0033, 0036, 0005, 0006, 0010) from E9F4Y's project with `exegenius-params` (e8efaa3), as W2 and W2b did (`plans/W2.md`, `plans/W2b.md`). Two copies per page (WS, WO). Diff the task files across arms.
   - Then launch 18 `general-purpose` runs (WS: sonnet/high; WO: opus/xhigh). Judge with J22's page judge (J26) and read by W3's table.
3. **A1b, the support-instruction test:** draft and preregister it before any run.
   - It uses A1's seeded pages and support tasks (20 per configuration), with one instruction sentence added: ask whether the claim is more general than its quotation.
   - Compare with A1 stage 1's support answers.
   - Judge-free; about 6–10 M per configuration.
   - Fix a classifier criterion on recall of hand-labelled positives for any audit measure, prospectively.
4. **Background audit (model-free):** acceptance, redos and stability of the known-concept lists across existing accepted background tasks. It decides whether background needs any model experiment.
5. Update `policy.md`'s ledger after each.

### Decisions only the author can take

- Merge the combined argument instruction (branch `141026-exp-params-cases`) to `master`. This is a default change.
- A1: keep the preregistered `ASSERTS` result as primary (it is), and approve the prospective classifier rule for new experiments.
- Auditing design: whether audit or support tasks get a source check (a task change, not a model change).
- The writer default after W3.

### What is ready

- **A1 stage-2 copies:** `scratchpad/a1/runs2/` and `queue2.tsv`. Deferred, not cancelled.
- **W3 plan:** preregistered.
- **The ledger:** in `policy.md`.

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

- None outstanding. E11's result is in `plans/E11.md`, `policy.md`, `registry/` (J25 in `pairwise-judging.json`) and `docs/journal/2026-10-15-e11-a1-results.md`. A1 stage 1 is recorded in the same places.

## Next actions, in order, when the author replies

1. Record the meter reading in `budget.md`, as a reading, with its time.
2. Record the author's rulings on A1 (classifier, trigger reading, which configurations get stage 2) in `plans/A1.md`'s ledger, before any stage-2 call.
3. Launch whatever the author orders (A1 stage 2, W3), within the reading. Launch nothing without a reading.

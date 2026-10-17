# Calibration programme: current state and next actions (for resuming without this session's context)

**Updated:** 2026-10-17 07:41 UTC. Read this file, then `RUNBOOK.md` §8 ("Resuming"), before acting. Times are UTC and taken from `date -u`.

## Checkpoint, 2026-10-17 07:41 UTC (read first)

**Quota:**
- The last reading is the author's: about 4% used at about 06:44.
- Since then, at 07:37, 89.3 M cache read was measured: A2 runs 18.0 M, seed reviews 2.9 M, this session 68.4 M. These are counts, not a reading.
- **No model call is launched until the author gives a new reading.**
- A2 itself was launched on the 06:44 reading, not a fresh one. That is a recorded deviation (`plans/A2.md`, `budget.md`).

**Done since 07:09:**
- **A2 preregistered (07:14), run (66/66, 0 contaminated), labelled blind (433 rows, committed 07:28), adjudicated blind (07:33) and read (07:39).** Everything is in `plans/A2.md`, Result, and `docs/journal/2026-10-17-a2-run/`.
  - **Outcome row 1.** SI (Sonnet `high`, the FIDELITY and SOURCE text, reading the paper) caught 13/13 source-only and 26/28 page errors. F_sham was 0/18, and adjudicated F_A 0.17.
  - **The false-alarm reading is fragile:** row 2 with borderline adjudications flipped (F_A 0.83) or with raw F_A.
  - **Cost:** 0.42 M cache read per SI audit, against 0.13 M for the current audit.
  - **The stage-2 condition is met by the letter.** The recommendation is an independent blind adjudication first.
- **Registry:** `registry/specs/A2.json`, the record, and INDEX rebuilt.
- **Policy:** the auditing rows updated, plus an update note under the decision ledger. The SI audit text is now a proposed task change, for the author to decide.

**Next, in order (each needs a new meter reading):**
1. **The A2 independent adjudication (~3–6 M, judge-only).**
   - A fresh Opus subagent gets the 15 items in `adjudication-items.json`, with notes masked by `mask_notes.py`.
   - It also gets the six clean pages in `docs/journal/2026-10-17-a2-design/pages/` and `sources/paper.tex`, plus the adjudication rule from `adjudication.json` (`rule`).
   - It is given no arm map and none of this session's verdicts.
   - Report SI's and I's F_A under its verdicts beside the primary. It is a sensitivity analysis, so preregister its reading first: SI passes if at least 6 of its 9 alarms are genuine.
2. **W3, amended (~80–175 M).**
   - 24 runs from `docs/journal/2026-10-15-w3-setup/prompts.tsv` plus the WS2 rows (`scratchpad/w3-prompts.tsv`, 24 rows; copies in `scratchpad/e6/W3{WS,WO,WS2}P*`).
   - Then J26 (72 judgements); read by the primary table and R2.
3. **The author's decisions:**
   - PR #13 (merge);
   - the SI audit change (a CLI change to the audit text and file restriction);
   - the general classifier rule (A2 adds evidence: `ASSERTS` agreement 0.79 overall, 0.75 on SI's notes);
   - the writer default, after W3.

## Checkpoint, 2026-10-17 07:09 UTC (new week)

**Quota:** the author's reading is about 4% used (session), at about 06:44 on 2026-10-17, at the start of a new weekly allowance. Model calls since: two independent seed reviews (Opus subagents). Check the meter again before A2's or W3's runs.

**Done today (model-free unless stated):**
- **Planning instruction:** built and tested in an isolated worktree and target directory (3 + 79 + 15 tests; clippy and fmt clean). It was fast-forward pushed to `wiki-explanation-reader-sim-v8dqtr` and opened as **PR #13**, not merged.
  - The 10-15 denial is explained: that branch existed (PR #7, merged), so `-B` would have force-reset it.
- **W3, amendment 2** (`plans/W3.md`; simulation in `docs/journal/2026-10-17-w3-design/`):
  - The 10-15 fidelity secondary was unsafe. It cleared Sonnet 42–66% of the time when Sonnet fails 20–30% of page-runs.
  - Added: a second Sonnet run on each key-step page (six WS2 copies are made) and rule R2. The primary is unchanged.
  - W3 is now ~80–175 M.
- **A2, prepared** (`plans/A2.md`; `docs/journal/2026-10-17-a2-design/`):
  - 12 source-only seeds and the key (`tools/a2seed.py`).
  - Arms B, I and SI (`tools/a2setup.py`), with the text check passing. 66 copies in `scratchpad/a2/runs/`, launch order in `scratchpad/a2/queue.tsv`.
  - The collector allows `sources/paper.tex`; the masking tool is written.
  - Independent review 1: 9 of 12 accepted. Five seeds were revised.
- **Policy:** gap review added (`policy.md`); 4% reading recorded (`budget.md`).

**Next, in order:**
1. **A2 review 2:** read `scratchpad/a2/review2.json`. If any seed fails again, revise it and repeat; if all pass, record the key's sha256 and **preregister A2** (`plans/A2.md` ledger) before any A2 run.
2. **A2 runs:** check the meter first, then launch the 66 tasks from `scratchpad/a2/queue.tsv`, up to 20 at a time (`general-purpose`, sonnet, high, prompts word for word).
   - Collect: `a1collect.py scratchpad/a2/runs ANSWERS --subagents … --allow sources/paper.tex`.
   - B's A1 variants: copy the AS answers from `docs/journal/2026-10-15-a1-stage1/answers/AS/1`.
   - Score: `plant.py score --key docs/journal/2026-10-17-a2-design/key.json --notes`.
   - Then mask (`mask_notes.py`), hand-label, commit the labels, rescore (`plant_labels.py`), adjudicate clean-page alarms, and read by A2's table.
3. **W3 (amended):** after A2, on a meter reading. 24 runs from `docs/journal/2026-10-15-w3-setup/prompts.tsv`; prepare J26 (54 + 18 judgements); read by the primary table and R2.
4. **The author's decisions:** PR #13; source access (after A2); the writer default (after W3).

## Update, 11:08 UTC (the author's note at about 11:00; no meter reading in it)

**No model calls.** There is no reading since the reset, and the last one (20% remaining at about 09:24) predates it.

**Done, model-free:**
- **W3 check** (`plans/W3.md`, "Can the table decide?"):
  - The primary table guards well against a false "generalises", but returns "page-specific" for most plausible effect sizes.
  - Ceiling ties on 0021 and 0033 make "generalises" unlikely even with a strong advantage.
  - A **secondary fidelity analysis**, added before any run, asks whether Sonnet meets the standard on key-step pages and whether Opus fixes the failures. The primary table is unchanged.
  - **Option for the author:** a second Sonnet run per key-step page, about +15–35 M.
- **A2 drafted** (`plans/A2-draft.md`): a source-aware, structured audit tested against A1's errors plus 12 new source-only seeds; arms B, I and SI on Sonnet `high`; about 25–45 M.
- **The final policy form:** `policy.md`, "Calibration status", with the author's decisions listed separately.
- **The planning-instruction proposal:** `proposals/argument-instruction.md`, the diff and its evidence.

**Blocked:** creating the PR branch for the planning instruction. The permission classifier denied `git worktree add -B wiki-explanation-reader-sim-v8dqtr …` as "Git Destructive". It was not retried.

**Next, in order:**
1. **With a meter reading:** launch W3 (`docs/journal/2026-10-15-w3-setup/prompts.tsv`), with the second Sonnet run only if the author adds it. Prepare J26 while the runs go, and read W3 by both tables.
2. **A2, model-free:** write and check the 12 source-only seeds; build the experiment branch and isolated binary when no runs are active (if the classifier allows the branch); then preregister.
3. **A1b:** preregister, then build its isolated binary.

## Checkpoint, 09:26 UTC, 2026-10-15 (read first)

**The objective (the author, 09:24):** a complete, defensible configuration policy for every pipeline task: extraction, background, argument planning, writing and fidelity auditing. Finishing the queued experiments is not the goal in itself. The ledger is `policy.md`, "Decision ledger". Run only experiments that can change a configuration decision.

**Quota:**
- The author's reading at about 09:24: **20% remaining** (80% used), with the reset at about 09:44.
- Nothing was launched on it: W3 cannot finish before the reset, and A1 stage 2 would not change a decision.
- A check-in is scheduled for 09:52 (`trig_01SpgYi2xwX2SFduvWJ6jE5h`).
- After the reset, there is no reading until the author gives one. Do not infer the meter from tokens.

**09:52 check-in:** the reset has passed and the author has posted no reading since 09:24. Nothing was launched; one request for a reading was sent.

### Next actions after reset, in order

1. **Get a fresh meter reading,** or use one the author has posted after 09:44. If none, ask once, and do only model-free work meanwhile.
   - With a reading after the reset, launch W3's 18 runs at once from `docs/journal/2026-10-15-w3-setup/prompts.tsv`.
   - When the runs are accepted, collect the pages and prepare J26 as J24 was prepared (`scratchpad/prep-j24.py`; J22's page judge, three judges per page, a fresh seed, scored ids from `assigned.json`).
   - Judge, then read by `plans/W3.md`'s table.
2. **W3** (`plans/W3.md`, preregistered 05:31): the writer routing question, the one queued experiment that can change a decision.
   - Expected cost: ~70–140 M cache read; about an hour (runs of 15–25 min, 20 at a time, then J26's 54 page judgements).
   - **Setup done at 09:28–09:29** (model-free): 18 copies in `scratchpad/e6/W3{WS,WO}P{page}`; prompts in `docs/journal/2026-10-15-w3-setup/prompts.tsv`; scored ids fixed in `assigned.json`. **Ready to launch.**
   - Then launch 18 `general-purpose` runs (WS: sonnet/high; WO: opus/xhigh). Judge with J22's page judge (J26) and read by W3's table.
3. **A1b, the support-instruction test:** **drafted at 09:30** (`plans/A1b-draft.md`). It needs preregistration, an isolated binary built from a branch with the one-sentence change, and copies made by `a1setup.py` (support kind) before any run.
   - It uses A1's seeded pages and support tasks (20 per configuration), with one instruction sentence added: ask whether the claim is more general than its quotation.
   - Compare with A1 stage 1's support answers.
   - Judge-free; about 6–10 M per configuration.
   - Fix a classifier criterion on recall of hand-labelled positives for any audit measure, prospectively.
4. ~~Background audit~~: **done at 09:30** (`docs/journal/2026-10-15-background-audit.md`). Keep Sonnet `high`; no model experiment is justified.
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

# Calibration programme: current state and next actions (for resuming without this session's context)

**Updated:** 2026-10-15 04:47 UTC. Read this file, then `RUNBOOK.md` §8 ("Resuming"), before acting. Times are UTC and taken from `date -u`.

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

### E11 (`plans/E11.md`): Opus `xhigh` against Sonnet `high` planners above the ceiling, pairwise

**Runs:** six Opus `xhigh` runs in `scratchpad/e6/E11F{2,3,4,5,6,8}O`.
- **All six are accepted** (01:34–01:53). The run table is in E11.md.
- Check acceptance in each run's `exegenius/log.jsonl`: a `submit` event for `0003-argument` with outcome `accepted`.

**Next:**
1. When all six are accepted (or have ended), record each in E11's ledger: seconds, calls, cache read, submits and `max_tokens` stops (`scratchpad/tools/perrun.py SUBAGENTS '^E11F'`). Then run the judge-free screen: acceptance, node count, characters.
2. If a run ended without an accepted plan, record it. E11's plan has no rerun rule for operational failures, so a rerun must be preregistered in E11.md before it launches.
3. ~~Prepare J25~~ **Done at 01:53.** 14 pair files and prompts are in `docs/journal/2026-10-15-e11-ceiling/judge/prompts/Qnn.txt`, with no leaks.
4. **Budget gate before J25** (14 pairwise judgements, ~85–140 M): a fresh meter reading that leaves room, or the reset having passed (after about 04:40) with spend since the reset measured (`calibration/tools/spend.py SUBAGENTS --since <reset time>`).
5. **J25 launched at 04:47** (all 14), after the reset; see E11.md's ledger for the budget basis. *Was:* **Launch J25:** one Opus judge per file; description `J25 judge Qnn[-b]`; pointer prompt "Read the instructions in <prompt file> and follow them exactly. Read only that file and the files it names."
6. **Score** (checked to run on the empty batch at 01:55). The sign test counts a pair for O when net ≥ 2:

   ```
   cd /home/user/exegenius; J=docs/journal/2026-10-15-e11-ceiling/judge
   python3 -I calibration/tools/pairwise.py score $J --margin 2 \
     --expect E11F2O\>E9F2Y --expect E11F3O\>E10F3Y --expect E11F4O\>E9F4Y \
     --expect E11F5O\>E10F5Y --expect E11F6O\>E10F6Y --expect E11F8O\>E9F8Y \
     --arm O=E11F2O,E11F3O,E11F4O,E11F5O,E11F6O,E11F8O --arm S=E9F2Y,E10F3Y,E9F4Y,E10F5Y,E10F6Y,E9F8Y > $J/results.txt
   python3 -I calibration/tools/pairwise.py score $J --margin 2 --expect E9F4Y\>E10F4Y2 > $J/control.txt
   ```

   **The reading** (E11.md, fixed):
   - Per extraction, O is favoured when net ≥ 2 and O wins at least one verdict in each order. With 2 verdicts that means both; with 4, check "verdicts per order" by hand. S is favoured under the symmetric rule; anything else is tied.
   - "O better" needs O favoured on ≥ 5 extractions and S on none.
   - A proposal also needs the control tied (|net| ≤ 1), then the Sonnet-judge cross-check.

7. **Sequential second judges** (only where a pair's two verdicts disagree in direction) and the **Sonnet-judge cross-check** (only if O is favoured) each need a fresh budget check first.

### A1 (`plans/A1.md`): planted-error recall for fidelity auditing

- **State:** preregistered at 01:18. The 132 task copies are made and checked in `scratchpad/a1/runs/A1{AH,AS,AO}1/`. `prompts.tsv` lists description, model, effort, root and prompt for each.
- **Gate:** no model calls until E11 is complete or the reset has passed, and the budget is checked. E11 has priority.
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

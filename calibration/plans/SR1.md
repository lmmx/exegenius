# SR1: do simulated readers' scores depend on what the wiki says?

**Status:** preregistered on 2026-10-17 at 17:49 UTC, before any SR1 model call. This is the one bounded experiment on simulated readers that the author asked for on 2026-10-17. It decides whether to keep them in the pipeline. No follow-up series is planned.

## Why

- **The pipeline plays a finished wiki against simulated readers.** It writes goal questions with key points, sends readers (`exegenius-reader`, Haiku) through the wiki by `exegenius read` and `exegenius search`, grades their answers against the key points beside a no-wiki baseline, and turns stuck points into rewrite notes.
- **The template's default is `rounds = 2`,** so the simulation is on unless a project turns it off.
- **No simulation has ever been run on these wikis.** No branch of `lmmx/openai-nlogn-wiki` has a `simulation.json`, and every calibration project set `rounds = 0`.
- **The open threat:** the reader is a model that may already know the mathematics. Its answers may not depend on the wiki at all (`docs/journal/2026-10-10-simulation-validity.md`). If they do not, coverage measures nothing about the wiki, and repairs driven by it are noise.

## Design

**The wiki:**
- `full-v0.6-latex-planner-handoff` (lmmx/openai-nlogn-wiki, 72f1de8): 25 pages, all audit rounds done (`state.done`).
- It is extracted to the scratchpad's `sr-probe/`, and run with the current binary (`exegenius` → `target/debug/exegenius`, sha256 c37a2b3d…, 0.6.0; no rebuild).
- `[simulation] rounds = 1`: one round, so no repairs. Budget 15 pages and nonce names on (the defaults); the default reader profile only.
- Checked at preregistration, with no model call: `exegenius next` issues the goals and held-out goals tasks (0213, 0214).

**Phases:**
1. **Goals.** The goals and held-out goals tasks run once, as `general-purpose` subagents on Sonnet `high` (the configuration that `exegenius-task` tasks run on in calibration), with prompts word for word.
2. **The degraded copy** (model-free, before any reader run). The project is copied twice after the goals are accepted: **intact** (unchanged) and **degraded**.
   - In the degraded copy, on the pages each goal lists, every paragraph that states or derives one of that goal's key points is deleted. Headings and all other text stay.
   - Each deletion is recorded with its page, the paragraph and the key point it carries (`docs/journal/2026-10-17-sr1/degradation.json`). The file is committed before any reader runs.
3. **The round.** In each copy, `next` issues the read tasks (one per goal and starting point: the front page and a deep link to the goal's last page) and the baseline tasks; then the grade tasks.
   - Read and baseline tasks run on Haiku `high`, as `exegenius-reader` specifies.
   - Grade tasks run on Sonnet `high`.
   - All prompts are word for word, each in a fresh subagent.
   - The graders of the two copies are separate, and neither sees the other copy.

## Measures

| Measure | Definition |
|---|---|
| **C_int** (primary) | share of key points the grader finds, over all reader runs and goals, intact copy |
| **C_deg** (primary) | the same, degraded copy |
| **C_base** | the same for the baseline answers, averaged over the two copies (their difference is reported as grading noise) |
| Lift | C_int − C_base: what the wiki adds over prior knowledge |
| Sensitivity | C_int − C_deg: whether coverage falls when the wiki stops saying it |
| Localisation (secondary) | of degraded runs that opened a page with a deletion, the share recording a stuck point or a give-up on that page |
| Descriptive | give-ups, pages opened, possible leaks (real names in answers), repair and held-out goals apart, cost per run |

Key points are pooled over goals and runs, with 80% Wilson intervals. The comparisons are differences of shares.

## Reading (fixed)

| # | Condition | Reading |
|---|---|---|
| 1 | Lift ≥ 0.20 **and** sensitivity ≥ 0.15 | **Reader coverage reflects the wiki.** Keep the simulation as configured. Stuck points stay as repair notes if localisation ≥ 0.5; otherwise propose keeping coverage reporting and dropping stuck-point repairs |
| 2 | lift < 0.05 **or** sensitivity < 0.05 | **Reader coverage does not depend on the wiki.** Propose turning the simulation off by default (`rounds = 0`), available on request. A default change, for the author |
| 3 | otherwise | **Inconclusive.** The conservative choice is to propose the simulation off by default, with its cost and unvalidated signal stated. No further reader experiment |

## Cost (estimates)

- Goals: 2 Sonnet tasks, ~1 M.
- Readers: about 6 goals × 2 starting points × 2 copies, about 24 Haiku runs at ~0.5–1.5 M, so ~12–36 M.
- Baselines: ~2–4 M.
- Grading: about 12 Sonnet tasks, ~3–6 M.
- Orchestration: ~10–20 M.
- **Total ~30–65 M.**

## Weaknesses (stated)

- **The wiki** is one run of an older pipeline, one paper. The comparison is within it.
- **The degradation is strong:** it deletes the content, so sensitivity is the minimum a useful reader signal must show. It does not test whether readers find subtler gaps.
- **The deletions are chosen by this session** from the goals, before any reader runs, by the stated rule.
- **One round, one reader profile, one run per goal and starting point.**

## Ledger

| Item | Status |
|---|---|
| Probe | 2026-10-17 17:48 UTC: `next` on the extracted wiki issued the goals tasks (no model call) |
| Preregistration | 2026-10-17 17:49 UTC, before any SR1 model call |

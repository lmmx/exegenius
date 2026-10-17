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

## Result (row 3: inconclusive)

Read by `tools/sr1read.py` (`docs/journal/2026-10-17-sr1/reading.json`) once all 2 + 54 + 18 tasks were accepted: 9 goals, 34 key points, 18 reader runs and 9 baselines per copy.

| Measure | Value | 80% interval |
|---|---|---|
| C_int | 57/68 = **0.838** | 0.773–0.887 |
| C_deg | 47/68 = **0.691** | 0.616–0.758 |
| C_base | 7/68 = 0.103 (intact copy 3/34, degraded 4/34; grading noise −0.03) | 0.065–0.160 |
| Lift | **0.735** (row 1 needs ≥ 0.20) | 0.649–0.797 (Newcombe, descriptive) |
| Sensitivity | **0.147** (row 1 needs ≥ 0.15) | 0.054–0.237 (Newcombe, descriptive) |
| Localisation (secondary) | 18/18 | 0.916–1.000 |

**Reading:**
- **Row 3, inconclusive.** Lift passes row 1 easily. Sensitivity misses it by one key point in 68: row 1 needed C_deg ≤ 46/68. It is above row 2's 0.05, so the wiki is not shown irrelevant either.
- **The row's recommendation stands:** propose the simulation off by default (`rounds = 0`), available on request, with its cost and unvalidated signal stated. That is a default change, for the author. Nothing is changed here, and no further reader experiment follows.

**What the measures do and do not show** (exploratory; none of this changes the row):
- **Lift does not answer the prior-knowledge threat.** The baseline task tells the reader to use only a stated background ("Everything else you know, the reader does not"). 15 of the 18 baseline answers decline ("the background does not cover this"). Lift therefore compares the wiki with an instructed refusal, not with what Haiku knows. In this design only sensitivity speaks to the threat the plan named.
- **Sensitivity sits almost entirely in the held-out goals.**
  - Repair goals: 45/46 → 42/46 (−0.07), near ceiling in both copies.
  - Held-out goals: 12/22 → 5/22 (−0.32).
  - Two explanations fit the repair goals' small drop, and neither was tested: the wiki states the key points more than once (74 of the 111 degraded citations still come from pages that lost paragraphs), or the grading is lenient.
- **Stuck points do not localise deletions.** Localisation is 18/18, but intact runs record stuck points on the same pages at the same rate: 18/18 runs, 148 stuck points intact against 142 degraded. Readers record about eight stuck points a run whether the content is there or not. Row 1's stuck-point clause is moot under row 3, but this is evidence against using stuck points as repair notes.
- **Descriptive (intact / degraded):**
  - give-ups 1 / 3;
  - mean distinct pages opened 8.2 / 9.8;
  - runs whose answers name a real page (leaks) 2 / 3, mostly `nlogn-conjecture`;
  - wrong points 1 / 0.
  - Home and deep starts are within 0.03 of each other in both copies.

**Cost** (`tools/spend.py`, subagents described `SR1 …`):
- 74 subagents (Haiku readers and baselines, Sonnet goals and grades); 661 calls; 37.9 M cache read, 3.07 M cache write, ~120 k visible output.
- One round on this 25-page wiki is about half of that, ~19 M cache read, with orchestration on top.
- Within the 30–65 M estimate. Orchestration is not split by experiment.

**Deviations:**
1. **Readers and baselines ran as `general-purpose` subagents,** which have every tool, not as `exegenius-reader` (Read, Write, Bash). Some Haiku readers called session tools the task forbids. Six sessions were created between 17:55 and 17:59 UTC, plus one `read_documentation` and one `list_sessions` call. All six sessions are archived. One ran a single turn with no task (~61 k tokens, outside the spend counts); five never started. None touched the wiki copies, and the binary accepted every answer. Production's reader agent cannot make these calls.
2. **The deletion rule** was operationalised mechanically before any reader run (recorded above). It removed 31 paragraphs and left `nlogn-conjecture` empty.
3. **Localisation's "give-up on that page"** is read as a give-up whose last page has a deletion. No run gave up on a deleted page, so the choice does not matter.

## Ledger

| Item | Status |
|---|---|
| Probe | 2026-10-17 17:48 UTC: `next` on the extracted wiki issued the goals tasks (no model call) |
| Preregistration | 2026-10-17 17:49 UTC, before any SR1 model call |
| Goals | 0213 and 0214 accepted (9 goals: 6 repair, 3 held out; 34 key points) |
| Degradation | 17:52 UTC, before any reader run: `tools/sr1degrade.py`, 31 paragraphs deleted on 11 pages (`docs/journal/2026-10-17-sr1/degradation.json`). The selection rule is operationalised mechanically (per key point, the top content-word-overlap paragraph on the goal's pages plus any with overlap ≥ 0.35), so that it is blind to reader outcomes. The nlogn-conjecture page is left empty. Both copies issue the same 27 tasks (18 read, 9 baseline), identical apart from the root |
| Round | 17:51–18:06 UTC (tasks issued 17:51:48, last grade accepted 18:05:34, from the copies' run logs): 18 read and 9 baseline tasks per copy on Haiku `high`, then 9 grade tasks per copy on Sonnet `high` (separate graders per copy), all accepted; the stray session calls are deviation 1 |
| Result | ~18:09 UTC (committed 18:10): `tools/sr1read.py` → **row 3, inconclusive** (lift 0.735, sensitivity 0.147 against 0.15). Recommendation: propose `rounds = 0` by default, for the author; not changed here |
| The author's decision | 2026-10-17, about 20:05 UTC: **simulated readers stay on by default** (`rounds = 2`). They are part of exegenius's intended architecture, and this result is inconclusive, not negative, so row 3's conservative proposal is not adopted. The author first approved it at about 19:45, and it was reverted before merging (PR #14). Their effectiveness remains unvalidated |

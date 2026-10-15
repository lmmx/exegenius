# Model and effort policy, per pipeline stage (the calibration programme's deliverable)

**What it is:** the table the calibration programme exists to fill. For each stage of the exegenius pipeline it gives:
- the configuration to use (model, effort, instruction version);
- the evidence for it;
- its cost;
- how far the recommendation generalises.

**Its status:** written on 2026-10-14 at 22:20 UTC, after an outside review of the day's work. That review's point was that the programme had measured one stage deeply and the others not at all. This file makes that gap explicit. Every queued experiment must answer a question in this table, or remove a blocker to answering one (`RUNBOOK.md` §3).

**Its authority:** nothing here changes a pipeline default. A row marked "proposed" goes to the author. Only the author changes defaults.

## The table (one paper, `paper.tex`, unless stated)

| Stage (task) | Current default | Best evidence so far | Status | What would settle it |
|---|---|---|---|---|
| Extraction (`extract`) | Sonnet `high` | **EX1 evidence analysis (judge-free, 34 extractions; `docs/journal/2026-10-14-x1-extraction-evidence/`):** Sonnet `medium` skims: 4 of 5 first attempts were sent back by the coverage check, and the accepted answers quote 11 results against 23.5 and reach 10 core items against 15. Opus `medium` and Haiku `high` are shallower; Opus `high` is as deep as Sonnet `high`, at higher cost. Downstream, the extraction explains no detectable share of plan quality (E6, F = 0.55), and under the combined instruction all six extractions tested give 19/19 plans | **supported: keep Sonnet `high`.** A planner-on-extraction experiment (`plans/EX1-draft.md`, about 71–80 M) is **not run**: the possible saving is about 1 M per wiki |
| Background (`background`) | Sonnet `high` | none | **unmeasured** | low priority: short task, small cost share |
| Argument planning (`argument`) | Sonnet `high`, E6/E7-era instructions on `master` (the key-step instruction is not on `master`) | E4–E10. **Instructions dominate.** With the E8+E9 sentences, Sonnet `high` scores 19/19 faithful and 9/9 central on F2, F4, F8 (E9). Under E7 instructions, Opus `xhigh` beats Sonnet `high` on F1, F4, F8. Effort above `high` is not worth it: Sonnet `xhigh` hit the output limit; Opus `max` ≈ Opus `xhigh` within run variance at 5× cost. Opus `medium` is no better than Sonnet `high` (E5). Fable `high` is below Opus `max` (one run) | **supported for this paper's argument task only (E10, 23:45); not a general pipeline default.** Sonnet `high` + combined instruction. Sonnet `medium` and Haiku `high` fall well below it (S6). Whether a stronger planner is better above the ceiling is open. K3 validated at 00:15, and **E11** (Opus `xhigh` against Sonnet `high`, both with the combined instruction, six extractions, pairwise; `plans/E11.md`) is preregistered because its outcome can change this row. It waits on a meter reading or the next reset | (1) E10; (2) the cheapest configuration that stays at the ceiling (S6); (3) whether a stronger model is better *above* the ceiling, which needs a discriminating measure |
| Exposition (writers) | as `master` | **W2 (screen, one run per page and configuration, two key-step pages, a fixed 19/19 plan):** Sonnet `high` is adequate. Haiku `high` makes a major error and falls below on the hard page. Sonnet `medium` falls below on the hard page. Opus `xhigh` is within noise of Sonnet `high`, at about 2× its cost on the hard page (8/8 against 6–8) | **provisional: Sonnet `high`** (screen) | a stage-isolated writer experiment: fixed plan, a fixed subset of pages, models and efforts compared on page fidelity and simulated readers. **Needs the author's go-ahead** (writer runs) |
| Fidelity auditing (auditors, support checks) | as `master` | **A1 design (23:58 UTC):** neither of the pipeline's checks compares a page with the paper. `audit` asks what a reader cannot follow, from its task file only. `support` judges each claim against its own quotation. An error visible only against the source is a blind spot no model choice fixes. The planted-error test (`plans/A1-draft.md`, `tools/plant.py`, 36 seeded errors over 6 pages) measures what each configuration catches within those limits | **unmeasured; draft ready.** It needs preregistered thresholds, a rebuilt binary, and an orchestration route for 132 tasks | planted-error recall: known errors seeded into fixed pages, with detection rate and false alarms per model and effort. Cheap and largely judge-free |
| Simulated readers | as `master` | none on model choice | **unmeasured** | after exposition |
| Judging (the instrument) | Opus, three judges on primary measures | K2: Sonnet agrees with Opus on the checklist (0.979). The primary rubric saturates (19/19). **K3 (00:15, 2026-10-15): blind pairwise judging, both orders, passes all four preregistered criteria**: 16 of 16 verdicts on known pairs correct, 0 reversals across orders or judges, item position share 0.50, 37 of 37 primary-separated items matched; it detects a single removed sub-step (4 of 4, `slight`) | **primary rubric below the ceiling; pairwise (K3) above it**, one Opus judge per order, about 5.7 M per judgement | whether Sonnet judges can replace Opus on pairwise (K3 (e)1), and the identical-pair control, which the judges see through |

## Cost per argument run, measured (for the table's cost column)

| Configuration | Cache read per run | Time per run |
|---|---|---|
| Sonnet `high`, combined instruction | 3–5 M | 14–19 min |
| Sonnet `high`, E8 | 2.5–7.7 M | 12–22 min |
| Opus `xhigh`, E7 | 1.7–6.9 M | 15–23 min |
| Opus `max`, E7 | 15–16 M | 34–35 min |
| Sonnet `max`, E7 | 25 M | 46 min |

**Judging dominates experiment cost:**
- **Primary judgement:** about 4.8 M each (J18).
- **Checklist judgement:** about 1 M each.
- **This session's orchestration** is a further component: 708 M over 2026-10-14 to 15:51, mostly before a compaction (`budget.md` §4).

## Generality

Every row rests on one paper. No row becomes a general default without the second document (`RUNBOOK.md` item 7, which needs the author to supply a document).

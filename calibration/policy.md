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
| Extraction (`extract`) | Sonnet `high` | Indirect only: S1, E5 and E1 ran full planning with Opus `medium`/`high`, Haiku `high` and Sonnet `high`, judged on the final plan. Haiku `high` failed every calibre (S1). E6's eight Sonnet extractions vary enough to move the downstream plan by 3 faithful | **unmeasured as a stage** | judge-free coverage diagnostics on existing extractions across models, then a planner (fixed configuration) on extractions from each candidate |
| Background (`background`) | Sonnet `high` | none | **unmeasured** | low priority: short task, small cost share |
| Argument planning (`argument`) | Sonnet `high`, E6/E7-era instructions on `master` (the key-step instruction is not on `master`) | E4–E10. **Instructions dominate.** With the E8+E9 sentences, Sonnet `high` scores 19/19 faithful and 9/9 central on F2, F4, F8 (E9). Under E7 instructions, Opus `xhigh` beats Sonnet `high` on F1, F4, F8. Effort above `high` is not worth it: Sonnet `xhigh` hit the output limit; Opus `max` ≈ Opus `xhigh` within run variance at 5× cost. Opus `medium` is no better than Sonnet `high` (E5). Fable `high` is below Opus `max` (one run) | **candidate pending E10:** Sonnet `high` + combined instruction | (1) E10; (2) the cheapest configuration that stays at the ceiling (S6); (3) whether a stronger model is better *above* the ceiling, which needs a discriminating measure |
| Exposition (writers) | as `master` | none. W1 was cancelled; T1 replayed a writer rule, not a model comparison | **unmeasured** | a stage-isolated writer experiment: fixed plan, a fixed subset of pages, models and efforts compared on page fidelity and simulated readers. **Needs the author's go-ahead** (writer runs) |
| Fidelity auditing (auditors, support checks) | as `master` | none | **unmeasured** | planted-error recall: known errors seeded into fixed pages, with detection rate and false alarms per model and effort. Cheap and largely judge-free |
| Simulated readers | as `master` | none on model choice | **unmeasured** | after exposition |
| Judging (the instrument) | Opus, three judges on primary measures | K2: Sonnet agrees with Opus on the checklist (0.979). The primary rubric now saturates (19/19) | **adequate below the ceiling; blind above it** | a discriminating measure: pairwise preference judging, validated like K1, or downstream page and reader outcomes |

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

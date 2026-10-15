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
| Argument planning (`argument`) | Sonnet `high`, E6/E7-era instructions on `master` (the key-step instruction is not on `master`) | E4–E10. **Instructions dominate.** With the E8+E9 sentences, Sonnet `high` scores 19/19 faithful and 9/9 central on F2, F4, F8 (E9). Under E7 instructions, Opus `xhigh` beats Sonnet `high` on F1, F4, F8. Effort above `high` is not worth it: Sonnet `xhigh` hit the output limit; Opus `max` ≈ Opus `xhigh` within run variance at 5× cost. Opus `medium` is no better than Sonnet `high` (E5). Fable `high` is below Opus `max` (one run) | **supported for this paper's argument task only (E10, 23:45); not a general pipeline default.** Sonnet `high` + combined instruction. Sonnet `medium` and Haiku `high` fall well below it (S6). **E11 (04:58 UTC, 2026-10-15): Opus `xhigh` is not shown better above the ceiling.** O was favoured on 2 extractions, S on 2, and 2 tied (sign test p = 0.69). A control of two Sonnet runs on one extraction differed more than any O–S pair, so run variance dominates above the ceiling (`plans/E11.md`) | (1) E10; (2) the cheapest configuration that stays at the ceiling (S6); (3) whether a stronger model is better *above* the ceiling, which needs a discriminating measure |
| Exposition (writers) | as `master` | **W2 (screen) and W2b (repeat, pooled; 00:25 UTC):** on the two hard key-step pages, Opus `xhigh` is better than Sonnet `high` by W2b's preregistered rule. Page 0015: Opus 8, 8, 8 and 8, 7, 8 of 8, no major error; Sonnet 7, 6, 5 and 5, 5, 5, one run with a major error in every judgement. Page 0016: 4, 4, 4 against 3, 3, 3 of 4. On the easier page 0014, both are adequate. Haiku `high` and Sonnet `medium` fall below on the hard page (W2). Opus costs 2.0–3.2× Sonnet's tokens on the hard pages (+1.4–4.2 M each) | **Adopted by the author (01:06 UTC, 2026-10-15) as a provisional, paper-specific exception:** Opus `xhigh` for key-step pages and Sonnet `high` for the others, **in this wiki only**, pending stronger evidence. **Sonnet `high` remains the general writer default.** The key-step rule and this wiki's designation are under "Writer exception" below | more hard pages and a second paper; simulated readers on the same pages |
| Fidelity auditing (auditors, support checks) | as `master` | **A1 stage 1 (05:17 UTC, 2026-10-15; `plans/A1.md`):** planted-error recall on 6 pages, one run each. As preregistered, **no configuration is adequate.** Audit recall R_A: Haiku `high` 0.46, Sonnet `high` 0.29, Opus `xhigh` 0.39, against 0.50. Support recall R_S: 0.60 for all three, against 0.70; all three miss the same errors (generalisation 0 of 2). There were no false alarms beyond the thresholds. **The audit verdict depends on the classifier:** the preregistered `ASSERTS` passed its 80% agreement check (0.885) but finds only 45% of the notes that assert an error. Under blind hand labels (a sensitivity analysis, not preregistered), all three meet R_A (0.64, 0.50, 0.71). The audit and support tasks still never compare a page with the paper (the A1 design) | **unmeasured enough to choose: no proposal.** Stage 2 is triggered (AH on R_A; all three on R_S, reading "within 0.10" inclusively) and waits on a fresh meter reading. Whether the hand labels replace `ASSERTS` is the author's decision (a criterion change after results) | stage 2; the author's ruling on the classifier; a support task that can see generalisation errors |
| Simulated readers | as `master` | none on model choice | **unmeasured** | after exposition |
| Judging (the instrument) | Opus, three judges on primary measures | K2: Sonnet agrees with Opus on the checklist (0.979). The primary rubric saturates (19/19). **K3 (00:15, 2026-10-15): blind pairwise judging, both orders, passes all four preregistered criteria**: 16 of 16 verdicts on known pairs correct, 0 reversals across orders or judges, item position share 0.50, 37 of 37 primary-separated items matched; it detects a single removed sub-step (4 of 4, `slight`) | **primary rubric below the ceiling; pairwise (K3) above it**, one Opus judge per order, about 5.7 M per judgement | whether Sonnet judges can replace Opus on pairwise (K3 (e)1), and the identical-pair control, which the judges see through |

## Writer exception for this wiki (the author's decision, 01:06 UTC, 2026-10-15)

**The scope:**
- **Key-step pages:** Opus `xhigh`.
- **Other pages:** Sonnet `high`.
- **Where it applies:** this wiki (`paper.tex`, E9F4Y's plan) only, pending stronger evidence. It is not a general default; Sonnet `high` remains the general writer default.

**Why:** Opus was consistently more faithful on both hard pages (W2b). Sonnet's major error on page 0015 matters more than the small score differences. Two hard pages from one paper do not establish that the split generalises.

**The key-step rule (the author's wording):** a page is key-step when it introduces or carries a central derivation, proof transition, or result whose omission or distortion would break the explanatory chain. The rule reads the argument's structure, not page numbers.

**The designation is recorded before the writer is chosen,** so the rule cannot be retrofitted to favourable results.

**The designation (blind, 01:25 UTC, 2026-10-15; `docs/journal/2026-10-15-keystep-designation/`).**
- **How it was made:** a fresh Opus reviewer applied the rule to all 35 pages, from the plan alone. It saw neither W2 nor W2b, and its transcript shows it read only its prompt and the plan packet. The author asked for this at about 01:22, so that the classification does not depend on the observed writer scores.
- **The result:** 23 pages are key-step: 0012–0019, 0021–0023 and 0027–0038.
- **Not key-step:** 0004–0011, 0020 and 0024–0026.
- **Borderline:** 0010 (marked no); 0018, 0019, 0021, 0022, 0028, 0034 and 0035 (marked yes).

**This governs the writer choice.** The author's earlier "0014 does not qualify" was made after the writer results were seen, so it is superseded and not counted as independent evidence. 0014 is key-step under the blind designation.

**What it implies (decision-relevant, reported to the author):**
- The rule, applied blind, covers 23 of 35 pages, not the two hard pages the evidence is about.
- On 0014, the one designated page W2 measured besides 0015 and 0016, Opus bought nothing measurable over Sonnet.
- Opus wrote 0014–0016 at 1.4–4.2 M more cache read per page than Sonnet (2–5.6×). Across 23 pages that is roughly 30–100 M more per wiki, a rough range in measured tokens, not converted to quota.
- Whether the rule should be narrower is the author's call. Nothing here runs a writer.

**The author's ruling (about 01:37 UTC):**
- The blind designation stays fixed. It is a *classification* result, not evidence that Opus is better on all 23 pages. W2b establishes the advantage on 0015 and 0016 only.
- **No 23-page Opus rollout.** Sonnet `high` remains the general default.
- After E11, and with budget, a small preregistered writer validation is proposed: `plans/W3.md` (preregistered 05:31 UTC, 2026-10-15; not launched), with 6 seeded key-step pages and 3 non-key-step controls. Its results decide whether the split generalises.

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

# Capability landscape, view B: which configuration is good at which task

**As of 2026-10-17.** Generated from `landscape.json` by `tools/landscape.py`; edit the data, not this page. It is the first deliverable of issue #16, and presents existing evidence only: no experiment was run for it.

**Scope:**
- One paper: `paper.tex` (Integer multiplication below n log n, 5,361 lines). Nothing here is shown to transfer to another document (#19).
- Each task was tested on its own. No complete wiki has been run with the final configuration (#18), so nothing here shows the stages work together.
- Most comparisons are screens: one or a few runs per configuration. The notes give the counts.

**Legend:** ✅ meets the task's standard; ≈ comparable to the chosen configuration, not chosen for cost; ● the reference the row's comparisons are made against; ▲ better than the row's reference; ◐ mixed (better on some inputs, not others) or best tested but not adequate; ✗ below the standard or the row's reference; ? run, but the measure itself is not validated; ★ in production; blank: not tested. The number after a symbol is its note below the table.

| Task | Haiku `high` | Sonnet `medium` | Sonnet `high` | Sonnet `xhigh` | Sonnet `max` | Opus `medium` | Opus `high` | Opus `xhigh` | Opus `max` | Fable `high` |
|---|---|---|---|---|---|---|---|---|---|---|
| **Extraction** | ✗ 1 | ✗ 2 | ✅★ 3 |  |  | ✗ 4 | ≈ 5 |  |  |  |
| **Background** |  |  | ✅★ 6 |  |  |  |  |  |  |  |
| **Argument planning (final: combined instruction)** | ✗ 7 | ✗ 8 | ✅★ 9 |  |  |  |  | ≈ 10 |  |  |
| **Argument planning (earlier key-step instruction, superseded)** |  |  | ● 11 | ✗ 12 | ▲ 13 |  | ◐ 14 | ▲ 15 | ▲ 16 | ✗ 17 |
| **Argument planning (original instruction, superseded)** | ✗ 18 | ✗ 19 | ● 20 |  |  | ◐ 21 | ◐ 22 |  |  |  |
| **Writing** | ✗ 23 | ✗ 24 | ✗★ 25 |  |  |  |  | ◐★ 26 |  |  |
| **Fidelity audit** | ✗ 27 |  | ✅★ 28 |  |  |  |  | ✗ 29 |  |  |
| **Support check** | ✗ 30 |  | ✗★ 31 |  |  |  |  | ✗ 32 |  |  |
| **Simulated readers** | ?★ 33 |  |  |  |  |  |  |  |  |  |

## Notes


**Extraction.** Standard: depth of the extraction (judge-free), and the quality of the plans built on it.

1. Haiku `high`, below the standard or the row's reference: shallower. ([EX1](../docs/journal/2026-10-14-x1-extraction-evidence))
2. Sonnet `medium`, below the standard or the row's reference: skims: 4 of 5 first attempts sent back by the coverage check; quotes 11 results against 23.5. ([EX1](../docs/journal/2026-10-14-x1-extraction-evidence))
3. Sonnet `high`, meets the task's standard: as deep as any tested; every extraction tested fed plans judged 19/19 faithful. In production. ([EX1](../docs/journal/2026-10-14-x1-extraction-evidence), [E9](plans/E9.md), [E10](plans/E10.md))
4. Opus `medium`, below the standard or the row's reference: shallower. ([EX1](../docs/journal/2026-10-14-x1-extraction-evidence))
5. Opus `high`, comparable to the chosen configuration, not chosen for cost: as deep as Sonnet `high`, at higher cost. ([EX1](../docs/journal/2026-10-14-x1-extraction-evidence))

**Background.** Standard: the reader profile and known concepts, judged by the plans built on them.

6. Sonnet `high`, meets the task's standard: for planning only: every run fed ceiling plans; concept lists vary between runs (median overlap 0.67); its effect on writing is unmeasured. In production. ([BG](../docs/journal/2026-10-15-background-audit.md))

**Argument planning (final: combined instruction).** Standard: three blind Opus judges: core results faithful (of 19) and central steps (of 9).

7. Haiku `high`, below the standard or the row's reference: falls below the ceiling. ([S6](plans/S6.md))
8. Sonnet `medium`, below the standard or the row's reference: falls below the ceiling. ([S6](plans/S6.md))
9. Sonnet `high`, meets the task's standard: 19/19 faithful and 9/9 central on every extraction tested. In production. ([E9](plans/E9.md), [E10](plans/E10.md))
10. Opus `xhigh`, comparable to the chosen configuration, not chosen for cost: also at the ceiling; not better above it (pairwise: Opus favoured on 2 extractions, Sonnet on 2, 2 tied; p = 0.69). ([E11](plans/E11.md))

**Argument planning (earlier key-step instruction, superseded).** Standard: faithful core results (of 19), majority of three judges; one run per extraction.

11. Sonnet `high`, the reference the row's comparisons are made against: central gate 8/8; faithful 15, 7 and 7 on extractions F1, F4 and F8. ([E7](plans/E7.md), [S2](plans/S2.md), [S3](plans/S3.md))
12. Sonnet `xhigh`, below the standard or the row's reference: loses 2 faithful on F1. ([S2](plans/S2.md))
13. Sonnet `max`, better than the row's reference: faithful 18 on F1, at about 25 M cache read. ([S2](plans/S2.md))
14. Opus `high`, mixed (better on some inputs, not others) or best tested but not adequate: uneven: faithful 14 and 9 on F4 and F8 (Opus `xhigh`: 13 and 16). ([S3](plans/S3.md))
15. Opus `xhigh`, better than the row's reference: +3, +6 and +9 faithful over Sonnet `high` on F1, F4 and F8; the cheapest configuration that improved. ([S2](plans/S2.md), [S3](plans/S3.md))
16. Opus `max`, better than the row's reference: faithful 19 on F1; +1 over Opus `xhigh` on F8, within Opus `xhigh`'s own run-to-run difference, at 5× the cache read. ([S2](plans/S2.md), [S4](plans/S4.md), [S5](plans/S5.md))
17. Fable `high`, below the standard or the row's reference: below Opus `max` on the checklist (17 and 16 against 20 and 21); one pilot. ([FB1](plans/FB1.md))

**Argument planning (original instruction, superseded).** Standard: calibres: Overview (≥ 17 of 19 stated), Research (the central-derivation gate).

18. Haiku `high`, below the standard or the row's reference: Overview 0/2 (a two-run screen). ([S1](plans/S1.md))
19. Sonnet `medium`, below the standard or the row's reference: Overview 1/26. ([E3](plans/E3.md))
20. Sonnet `high`, the reference the row's comparisons are made against: Overview 10/10; Research 0/5. ([E1](registry/specs/E1.json), [E3](plans/E3.md))
21. Opus `medium`, mixed (better on some inputs, not others) or best tested but not adequate: Overview 5/5, Research 0/5 in five fresh plans (1/2 in an earlier two-run screen); derives less than Sonnet `high`. ([E5](plans/E5.md), [S1](plans/S1.md))
22. Opus `high`, mixed (better on some inputs, not others) or best tested but not adequate: Overview 2/2, Research 1/2 (a two-run screen). ([S1](plans/S1.md))

**Writing.** Standard: page judge, three blind Opus judges: ≥ 90% of the page's steps derived faithfully and no major error.

23. Haiku `high`, below the standard or the row's reference: below on the hard page. ([W2](plans/W2.md))
24. Sonnet `medium`, below the standard or the row's reference: below on the hard page. ([W2](plans/W2.md))
25. Sonnet `high`, below the standard or the row's reference: not adequate on key-step pages: meets the standard on 4 of 12 key-step page-runs; a major error in 1 of 2 runs on page 0015; adequate on the easier page 0014. The gap is underived steps, not wrong mathematics (1 major error in 24 page-runs). In production. ([W2](plans/W2.md), [W2b](plans/W2b.md), [W3](plans/W3.md))
26. Opus `xhigh`, mixed (better on some inputs, not others) or best tested but not adequate: best tested, not adequate: meets the standard on 4 of 6 new key-step pages, and on 0015 and 0016; better than Sonnet on 2 of 6 new key-step pages (large on 0036), tied on 3, worse on 1. In production only on pages 0015 and 0016 of this wiki. In production. ([W2b](plans/W2b.md), [W3](plans/W3.md))

**Fidelity audit.** Standard: planted errors on six pages: page-error recall ≥ 0.50, source-only recall ≥ 0.50, false alarms ≤ 0.5 per clean page, sham alarms ≤ 0.10.

27. Haiku `high`, below the standard or the row's reference: page-only text only: page-error recall 0.46 (0.64 under hand labels, a sensitivity analysis); the source-aware text is untested. ([A1](plans/A1.md))
28. Sonnet `high`, meets the task's standard: with the source-aware text: 13/13 source-only and 25/28 page errors, 0/18 shams, false alarms 0.50 per clean page by an independent review, exactly at the bound. With the old page-only text: 0/13 source-only, 14/28 page errors. In production. ([A2](plans/A2.md), [A3](plans/A3.md))
29. Opus `xhigh`, below the standard or the row's reference: page-only text only: page-error recall 0.39 (0.71 under hand labels, a sensitivity analysis); the source-aware text is untested. ([A1](plans/A1.md))

**Support check.** Standard: planted unsupported citations: support recall ≥ 0.70.

30. Haiku `high`, below the standard or the row's reference: 0.60, the same misses. ([A1](plans/A1.md))
31. Sonnet `high`, below the standard or the row's reference: 0.60; misses both generalisation errors (the source-aware audit catches them: 4/4). In production. ([A1](plans/A1.md), [A3](plans/A3.md))
32. Opus `xhigh`, below the standard or the row's reference: 0.60, the same misses. ([A1](plans/A1.md))

**Simulated readers.** Standard: reader coverage against a no-wiki baseline, and its drop when key paragraphs are deleted.

33. Haiku `high`, run, but the measure itself is not validated: coverage rises with the wiki (lift 0.74), but the baseline is an instructed refusal; deleting key paragraphs lowered coverage by 0.147 against a 0.15 bar (inconclusive); stuck points are not shown to locate missing content. On by default by the author's decision; grading on Sonnet `high`. In production. ([SR1](plans/SR1.md))

## Not yet in this landscape (issue #16)

- View A, model × effort plots: one line per model across effort levels, per task.
- View C, quality against cost: measured cache read per task against each configuration's result.
- View D, pipeline capability map: where in the pipeline each capability comes from.
- View E, evidence explorer: from each cell to its runs, judgements and failures.
- Updating `capability-map.md`'s calibre tables mechanically from the registry, instead of by hand.

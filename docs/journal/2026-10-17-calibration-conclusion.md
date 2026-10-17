# Calibration programme: conclusion and implementation

## Current State

- `calibration/policy.md`, section "Final configuration report", records one row per pipeline stage on `paper.tex`, covering extraction, background, argument planning, writing, fidelity audit, support check and simulated readers. Each row gives the model, effort, prompt, routing, measured cost, evidence strength, mathematical fidelity and limitations.
- `calibration/plans/W3.md` records the writing comparison: 24 page-runs judged by three blind Opus judges each. Opus `xhigh` is better than Sonnet `high` on 2 of 6 new key-step pages, which is the preregistered "page-specific" outcome, and the fidelity rule R2 reads inconclusive.
- The writing stage runs Sonnet `high`, with Opus `xhigh` on pages 0015 and 0016 of the wiki planned from E9F4Y. That is the configuration in place before W3, kept by W3's outcome matrix.
- `calibration/plans/A3.md` records the source-aware audit with a paragraph defining what counts as an error. On A2's planted errors it named 13 of 13 source-only errors and 25 of 28 page errors, with 0 of 18 sham alarms. Two independent blind reviews put its false alarms at 0.50 per clean page, the bound of the standard.
- `exegenius-core/src/audit.rs`, `audit_instructions`, gives the audit task A3's text and lets the auditor read the source files the task names. It was merged in #14 as commit 86937b1.
- `calibration/tools/a3verify.py` checks that the installed binary writes A3's 30 audit tasks byte-identical to the ones A3 ran, apart from the project directory.
- `calibration/plans/SR1.md` records the one test of simulated readers. Coverage rose with the wiki over a no-wiki baseline, and fell by 0.147 against a preregistered bar of 0.15 when the paragraphs carrying the answers were deleted, which is the inconclusive row.
- `exegenius-core/src/config.rs` defaults `[simulation] rounds` to 2, unchanged by the calibration on the author's decision of 2026-10-17.
- `calibration/capability-map.md`, section "All pipeline stages on this paper", summarises each stage's configuration, result, evidence and strength.
- `calibration/landscape.md`, generated from `calibration/landscape.json` by `calibration/tools/landscape.py`, sets every stage against every model and effort tested in one table, with a note and evidence link per cell (issue #16).
- `calibration/registry/specs/` and `calibration/registry/experiments/` hold records for W3, SR1 and A3 beside the earlier experiments, indexed in `calibration/registry/INDEX.md`.
- `calibration/RUNBOOK.md` marks the programme concluded, lists the open issues, and gives a closing checklist under "Stopping conditions".
- Issue #18 tracks a complete wiki run with the final configuration and simulated readers on.
- Issue #19 tracks a second document.

## Missing

- No complete wiki has been run with the final configuration, so no measurement covers the stages working together (issue #18).
- No calibration result has been tested on a document other than `paper.tex` (issue #19).
- No writer configuration meets the page fidelity standard on key-step pages: Sonnet `high` meets it on 4 of 12 key-step page-runs and Opus `xhigh` on 4 of 6.
- No support-check configuration reaches its recall bar of 0.70: every model tested reaches 0.60.
- No measurement relates simulated readers' coverage or stuck points to the quality of a wiki.
- `calibration/landscape.md` has view B only. Views A, C, D and E of issue #16 have no implementation.

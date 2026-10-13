# Calibration

A reusable programme for finding, for a given kind of document, task and required calibre, the least resource-intensive agent configuration that meets the calibre reliably, and for recalibrating that answer as models and software change.

| Part | What it is | Where |
|---|---|---|
| **Calibration Protocol** | the method: terms, configuration dimensions (model, ordinal effort, agent definition, prompt, settings), the four levels of quality instruments, calibres as hard gates, experimental design, sequential allocation and stopping, screening against confirmatory evidence, resource accounting with bases, and recalibration | `PROTOCOL.md` |
| **Evaluation Registry** | the evidence: one record per experiment, built mechanically from git refs (run logs, transcripts, verdicts), with every value's basis (measured, estimated, reported, unavailable) | `registry/` (`schema.md`, `specs/`, `experiments/`, `judging.json`, `INDEX.md`) |
| **Capability Map** | the conclusions: which configurations meet which calibre, for which document class, at what cost, with what uncertainty, at which versions | `capability-map.md` |
| **Explanatory Fidelity Evaluation** | the research track for levels 3 and 4: does the wiki carry a reader along the proof plan's paths, and does the reader come away understanding? | `fidelity/README.md` |
| **Rubrics** | per-document rubrics for the central chain of reasoning, preregistered | `rubrics/` |
| **Plans** | each experiment's preregistered question, arms, measures and decision rules, committed before its outputs exist | `plans/` |
| **Software plan** | execution profiles in exegenius (agent, model, effort per task kind, recorded in the run log): a plan for review, not implemented | `software-plan.md` |
| **Runner template** | the files for a dated experiment branch, and what earlier failures taught | `runner/` |
| **Tools** | `registry.py` (build and index records), `calibre.py` (pass rates with exact intervals), `effort.py` (model and effort per subagent), `usage.py` (tokens), `prepare.py` (blind judge inputs), `score.py` and `depth.py` (quote-location diagnostics) | `tools/` |

The tools in `tools/` are the maintained copies. The copies in `docs/journal/2026-10-13-*/` are the versions each journal entry used, kept as they were.

The journal (`docs/journal/`) remains the narrative record: what was done, why and when. This directory is what is meant to be reused: on another paper, another kind of document, or the next model release.

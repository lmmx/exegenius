# 2026-10-13: Sending back answers that quote sections but not their results

**The gap.** In the recovery experiment (2026-10-13-recovery-experiment), the coverage check judges breadth: an answer passes if it quotes at least half of the top-level sections that develop results. B6's extraction passed with 6 of 10 such sections but reached no core item of the draft reference. Its quotes came from the sections' openings, not from the results they state. B4's extraction (2 core items) and A2's (1) were alike, though the check never fired on A2, a control run.

**The rule.** On branch `131026-depth-rejection`, built on `131026-scratch-dirs` so that its experiment runs with per-task scratch directories:
- **What counts as a result:** a source's results are its theorem-like statements and its labelled displayed equations. A display with a `\label` is one the source refers back to.
- **Where a result is quoted:** each result runs from its start to the next result or heading at any level. It counts as quoted when a quote falls in that stretch.
- **The setting:** `[checks] coverage_depth = "results"` also sends back an answer that quotes fewer than a tenth of the results.
- **The message:** it lists the unquoted results by top-level section, with labels and lines, and asks for each concept or step to be quoted "at the theorem, lemma or equation that states or derives it".
- **The default** stays `"sections"`: the section rule alone, with its rejection text unchanged byte for byte. A replay of the planner-handoff run's extraction against the 1d90c28 binary confirms this.

**The threshold, calibrated on the n log n paper's 26 existing plans.** `paper.tex` states 66 results: 23 theorem-like statements and 43 labelled equations.
- **Plans drawn from the introduction or from section openings quote 0–3 of them:**
  - the extractions of A1, A2, A4, A5, A6, B4 and B6;
  - the arguments of A1, A4, A5 and A6.
- **Plans drawn from reading the paper quote 8–30:**
  - the extractions of B1 (9), B2 (13), B3 (16) and B5 (17);
  - every other argument of the experiment's runs, B6's (8) the lowest;
  - the Sonnet samples (17–30).
- **The best plans quote under half,** so a rule requiring half would send back every plan.
- **A tenth separates the two groups on this data.** Replayed with the rule on, the stored extractions of A2, B4 and B6 are sent back for their results. A1, A4, A5 and A6 are sent back by the section rule. B1, B2, B3, B5 and both Sonnet samples are accepted.
- **That agreement checks the code, not the rule.** The rule was chosen on these plans.

**The experiment** (proposed with this entry, on wiki branch `131026-exp-depth-rejection`):
- **Arms:** five planning-only runs with `coverage = "reject"` alone (C1–C5), and five with `coverage_depth = "results"` added (D1–D5).
- **Binary:** this branch's commit in both arms.
- **Measures:** as in the recovery experiment, plus each attempt's results quoted.
- **The question:** do D's extractions stop skimming, and do they reach more core items than C's, at what cost in attempts and time?

## Current State
- `Source` records each displayed equation with its `\label`, if any (`Display` in exegenius-core/src/project.rs)
- The coverage figures count a source's results (theorem-like statements and labelled equations) and the ones whose stretch holds a quote, and list the unquoted ones by section (`results`, `quoted_results`, `unquoted_results`, `results_shortfall` in exegenius-core/src/coverage.rs)
- `[checks] coverage_depth = "results"` makes the coverage check also send back an answer quoting fewer than a tenth of the results, with a message listing them; `"sections"`, the default, leaves the check and its message as before (exegenius-core/src/config.rs, `rejection` in exegenius-core/src/coverage.rs, `submit` in exegenius-core/src/task.rs)

## Missing
- No planning run with `coverage_depth = "results"`
- No calibration of the tenth on any source but the n log n paper

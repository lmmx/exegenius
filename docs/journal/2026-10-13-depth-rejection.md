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

## The experiment's results

The runs are on wiki branch `131026-exp-depth-rejection`, at commit 0dd87da.
- **Setup:** ten planning-only runs, exegenius f70a919, `exegenius-planner` subagents on Sonnet, orchestrated by a runner session on the development session's model.
- **No contamination:** no planner wrote to a shared place or touched another run's directory, and the highest overlap between two runs' concept sets is 0.30.
- **The scratch directories were used.** C1's planner put its script in `exegenius/tasks/0001-extract/`. It also ran `pkill -f "cat >"`, which could have killed other planners' commands; no other transcript shows an interrupted command.

**From the logs and the stored state** (core items: the draft reference's 19, reached by where quotes fall; results: the paper's 66 theorem-like statements and labelled equations):

| | C (section rule) | D (section and results rules) |
|---|---|---|
| Extraction, core items | 8, 10, 3, 1, 11 | 13, 11, 14, 14, 14 |
| Extraction, results quoted | 9, 16, 2, 2, 16 | 28, 14, 23, 20, 39 |
| Argument, core items | 4, 10, 9, 8, 9 | 11, 14, 11, 11, 14 |
| Saving and layer recursion anchored in the deriving sections (`depth.py`) | 4 of 5 (C3's saving: introduction only) | 5 of 5 |
| Planning time, seconds | 244–358 | 268–345 |

**What happened at each rejection:**
- **The early stop again.** The first extraction of C1, C2, C5, D3 and D4 quoted 2 of 10 result-bearing sections. The section rule sent each back, and each next answer quoted 9–10.
- **Skims passed under C.** The extractions of C3 and C4 were accepted at 7 and 8 sections with 2 results each: the pattern B6 showed.
- **Skims were sent back under D.** The results rule sent back D5's first extraction (6 sections, 5 results) and D3's third (9 sections, 6 results). Their next answers quoted 39 and 23 results.
- **D1** was sent back by the section rule at 5 sections, and its accepted answer quoted 28 results.

**What the comparison can and cannot claim:**
- **The extraction figures are partly circular.** The results rule rewards quotes where results are stated, and the reference scorer counts quotes where its items are stated or derived, which are mostly results.
- **The argument figures are less so.** Four of the five D argument planners were never sent back for coverage (D4's was, twice, by the section rule). Their arguments reach 11–14 core items against C's 4–10, which reflects the better extractions they planned from.
- **No judgement of the plans' content was made.** No reader checked that each step states its derivation correctly.

**Observed:** with the results rule, no accepted extraction quoted fewer than 14 of the 66 results, and no accepted argument reached fewer than 11 core items. Without it, two of five extractions were accepted as skims, and one argument (C1's) reached 4.

**Not shown:**
- that the tenth suits other sources;
- that the plans' steps are correct;
- that the rule holds up under the author's container setup.

## Proposed, not decided

- Make `coverage_depth = "results"` the default once reviewed. It would sit with `coverage = "reject"` on `131026-coverage-default`. This is the author's decision.
- An assessment of plan content that does not rest on where quotes fall: for example, a judge reading each plan against the reference set's statements.

## Current State
- `Source` records each displayed equation with its `\label`, if any (`Display` in exegenius-core/src/project.rs)
- The coverage figures count a source's results (theorem-like statements and labelled equations) and the ones whose stretch holds a quote, and list the unquoted ones by section (`results`, `quoted_results`, `unquoted_results`, `results_shortfall` in exegenius-core/src/coverage.rs)
- Wiki branch `131026-exp-depth-rejection` (0dd87da) holds ten planning runs of f70a919, five per setting, in which every extraction accepted under `coverage_depth = "results"` quotes 14–39 of `paper.tex`'s 66 results and every argument reaches 11–14 core items of the draft reference, against 2–16 and 4–10 under `"sections"`
- `[checks] coverage_depth = "results"` makes the coverage check also send back an answer quoting fewer than a tenth of the results, with a message listing them; `"sections"`, the default, leaves the check and its message as before (exegenius-core/src/config.rs, `rejection` in exegenius-core/src/coverage.rs, `submit` in exegenius-core/src/task.rs)

## Missing
- No calibration of the tenth on any source but the n log n paper
- No assessment of the plans' content beyond where their quotes fall

# 2026-10-13: A run log, and where planning answers' quotes fall

The author approved step 1 of the proposal in 2026-10-13-autonomous-investigation: a run log, and a source-coverage diagnostic for planning answers that only reports. He set these conditions:
- the diagnostic must not equate where quotes fall with coverage of the proof, and must say what it cannot establish;
- no rejection, retries or change to planner behaviour;
- tests for the deterministic checks, and documentation of the log and how to read it;
- the reference set kept as a draft for his review, not used to score runs;
- a539829 and the description of pull request #8 left alone;
- no commit or push until he has seen the diff.

Step 1 is a first increment toward the goal, which remains in-run detection, diagnosis and recovery. Recovery is not part of it.

## What was built

**The run log.**
- **Where:** `exegenius/log.jsonl`, written by `next` and `submit`.
- **An `issued` event** for each task handed out.
- **A `submit` event** for each submit, recording:
  - the attempt number;
  - the outcome: accepted; rejected, meaning the answer was checked and the task stays pending; or error, meaning nothing was checked;
  - the message or warnings;
  - for extract and argument answers, the coverage figures below. These are recorded whether or not the answer was accepted, so a later recovery step can be judged attempt by attempt.
- **Best effort:** a failure to write the log does not change the outcome of a command.

**The coverage diagnostic.** For each source, it records:
- how far the quotes reach in lines, and how many fall in the first tenth;
- every section, with its quotes and its theorem-like statements (environments declared with `\newtheorem`);
- the statements in sections that no quote falls in;
- concepts or presented steps whose name heads a section that none of their quotes falls in.

`exegenius status` recomputes the same figures from `graph.json` and `argument.json`, so they are available for runs made before the log existed. It also shows the log's counts per task kind.

**Nothing the agents see has changed.** On the planner-handoff run's source, the committed head (394b145) and the new build gave byte-identical output for:
- `next`;
- a rejected and an accepted extraction submit;
- the extract and background task files.

The only new file is `log.jsonl`. The skill and the subagent definitions are unchanged.

**What the diagnostic cannot establish** is written into the module and the README:
- a quote shows a passage was cited, not that its step is in the plan;
- a plan can quote every section and miss a derivation;
- a section with no quote may not matter;
- a result argued in prose, outside any theorem-like environment, counts in no statement figure. In this paper that includes the time accounting of §9.8, since §9 has no such environment;
- a short quote that also occurs in the introduction is placed there;
- heading matches do not stem words.

## Decisions made while building

- **Plain-text sources get only the line figures.**
  - `paper.txt` is the `pdftotext` transcription the early runs used. The author retired it for its transcription errors, and every run since uses the original `paper.tex`.
  - Run on `paper.txt`, the section figures were wrong. `pdftotext` puts each body heading's number and title on separate lines, so the numbered-heading heuristic (older code in `project.rs`, also used for citation locators) found only table-of-contents entries and equation lines.
  - For a source with no marked-up headings (LaTeX or markdown), the diagnostic now gives only the line figures, and says why.
  - The heuristic itself is unchanged, so citation locators for plain-text sources are unaffected.
  - The guard matters only for plain-text sources in general, not for this paper.
- **Each argument quote counts once.**
  - The first version gave each presented step all the quotes of its subtree. A step presented inside another presented step was then counted twice, which doubled the planner-handoff argument's 16 quotes to 34.
  - Each quote now belongs to the nearest step that a page presents.
- **LaTeX appendix sections are lettered** (§A), as LaTeX letters them. Before this, the appendix section of `paper.tex` was numbered §12. Citation locators use the same headings, so this also changes a citation of a quote from the appendix, from "§12 …" to "§A …". It is the one change outside the log that a page can show.
- **The diagnostic is not shown in `next` or `submit` output.** Showing it to the agent that wrote the answer, or to the orchestrating agent mid-run, would change behaviour, and the author asked for none in this step. `status`, which the skill tells the agent to run and report at the end, shows it.

## The diagnostic on the earlier runs

These are observations, computed with the new `status` from each wiki branch:
- **Extraction:** the first extract task's answer as submitted.
- **Argument:** the stored `argument.json`.
- **Top-level sections:** 13, counting the front matter.
- **Theorem-like statements:** 23.

| Run | Extraction: quotes, in first tenth, top-level sections quoted, statements in unquoted sections | Argument: same |
|---|---|---|
| `pilot-v0.6-latex-source` (42cd64b) | 29, 28, 4, 22 | 14, 0, 1, 19 |
| `full-v0.6-latex` (e4aff3e) | 23, 21, 3, 20 | 16, 15, 2, 23 |
| `full-v0.6-latex-depth` (fa33c9d) | 27, 24, 3, 22 | 16, 14, 3, 23 |
| `full-v0.6-latex-planner-handoff` (72f1de8) | 27, 27, 2, 22 | 16, 16, 1, 23 |
| Sonnet sample 1 (general-purpose subagent) | 82, 37, 10, 4 | 68, 1, 8, 6 |
| Sonnet sample 2 (general-purpose subagent) | 88, 30, 10, 3 | 72, 2, 9, 4 |

Notes on the table:
- **Agreement with earlier figures.** The extraction figures in the first tenth agree with those in 2026-10-13-reading-failure-reflections (21 of 23, 24 of 27, 27 of 27, 37 of 82), except Sonnet sample 2: 30 here against 29 there. That difference is unexplained, possibly where the tenth's boundary falls.
- **The pilot.** Its argument explains a part of the target by design, so its single quoted section (§3) is expected.
- **The `paper.txt` runs** (`full-v0.6`, `tmp-v0.5-build-record-stash`, on the retired transcription) get line figures only. Their extraction answers give the same figures: 32 quotes, lines 66–4441 of 5,258, 24 in the first tenth.

**What the table shows:** the diagnostic separates the plans read from the introduction from the plans read from the whole paper, on extraction and on argument alike. **What it does not show:** that the Sonnet plans cover the proof.

## The reference set, as a draft

A subagent (general-purpose, on this session's model) read all of `paper.tex` in chunks and drafted the results and derivations a plan of the main theorem must capture. The draft is in `2026-10-13-run-log-and-coverage/reference-draft/`: `reference.json` and `notes.md`.

**What it contains:**
- 31 items: 19 core, 7 supporting and 5 out of scope.
- For each item: its statement, its role, its dependencies, where it is stated and derived, and 1–3 anchor quotes from the body.

**What has been checked:**
- **The anchors, checked separately in this session:** all 73 occur exactly once in the space-joined text of `paper.tex` and start on the line given.
- **Two spot checks against the paper:** τ = σ = 1 − 2⁻⁵⁰ (line 1328), and the minimum margin 2⁻¹⁸¹ = 2κ (line 4854).

**What has not been checked:** the mathematics, the tiers and the granularity. Those are the author's to review.

**The five provisional items** in the proposal all appear, under the paper's names:
- the butterfly layer's recursion is `layer-recurrence`;
- the interchange recurrence is `interchange-recurrence`;
- the saving $s<Wm$ is `rank-saving`;
- Gaussian resampling is `gaussian-resampling`;
- the time accounting is `time-accounting`.

**A finding of the draft:** the introduction summarises every in-scope item but one. A plan drawn from the introduction can therefore name the right results without any of their derivations. That is why the draft takes its anchors from the body. The notes end with five questions for the reviewer.

No code reads the draft.

## Open, not decided

- **What adequate coverage means:** the author's review of the reference draft, including its five questions.
- **Whether `status`'s figures are enough for a person to judge a planning stage,** or whether the diagnostic should also say which reference items lack a body anchor. That would score plans against the reference, which awaits approval.
- **Step 2 of the proposal:** planning-only runs read through the log.
- **Step 3:** recovery through the submit loop. Before step 3 is built, a controlled check is still needed: does a planner that stopped early go back and read when given a coverage rejection? Under the real `exegenius-planner` definition, that may need the author's container.

## Current State
- `exegenius next` appends an `issued` event and `exegenius submit` a `submit` event to `exegenius/log.jsonl`, best effort, with the schema in the README's "Run log and diagnostics" section (exegenius-core/src/runlog.rs, exegenius-core/src/task.rs)
- `exegenius submit` records, for every extract and argument answer that parses, where its quotes fall in each source, and rejects nothing on that account (exegenius-core/src/coverage.rs, `planning_coverage` in exegenius-core/src/task.rs)
- `Source` maps normalised offsets to source lines, gives headings their levels, letters LaTeX appendix sections, reads theorem-like environments declared with `\newtheorem`, and records whether its headings are marked up (exegenius-core/src/project.rs)
- `exegenius status` prints the source coverage of the concept graph's quotes and of the stored argument's quotes, recomputed from `graph.json` and `argument.json`, and the run log's counts per task kind (exegenius-cli/src/main.rs)
- For a source whose headings are not marked up, such as `paper.txt` from `pdftotext`, the coverage diagnostic reports line figures only — the numbered-heading heuristic finds table-of-contents entries and equation lines there
- `next` output, submit messages and task files are byte-identical to 394b145 on the planner-handoff run's source, apart from the new `exegenius/log.jsonl`
- The tests cover sections, statements and lines read from LaTeX, markdown, numbered and plain text; coverage of an introduction-only plan; nested argument quotes counted once; log events round-tripping; and an end-to-end run whose log records a rejected extraction attempt's coverage — 88 tests pass and clippy is clean
- `docs/journal/2026-10-13-run-log-and-coverage/reference-draft/` holds a draft reference of 31 items for the n log n paper, with 73 anchor quotes that each occur once in `paper.tex` at the line given, unreviewed and read by no code

## Missing
- No reviewed reference set, and no scoring of plans against one
- No recovery or rejection on coverage, and no coverage shown to the agent during a run
- No coverage figures for extend answers
- No section structure for plain-text sources whose headings `pdftotext` split over lines
- No planning-only run read through the run log

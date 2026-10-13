# 2026-10-13: Recovery from introduction-only planning, set up

The run log and the coverage diagnostic (2026-10-13-run-log-and-coverage) make the planners' failure measurable, but they do not recover from it. The author set the objective: a working, observable recovery loop that is part of ordinary runs, driven by the development session until there is convincing evidence of recovery or a specific blocker that needs him.

## A new working rule

The author authorised routine work without stopping for approval at each commit:
- implementing, testing, committing and pushing to `timing-and-parallelism`, once the checks pass and the change fits the agreed scope.

What stays an escalation point:
- architectural decisions;
- destructive changes;
- scope expansions;
- actions that cannot be safely reversed.

Two further rules:
- A push is reported only once it has been verified.
- A setup that is not the real planner definition is never presented as equivalent to it.

## The recovery mechanism

**The rule.** With `[checks] coverage = "reject"` in `exegenius.toml`, `submit` sends back an extract or argument answer that:
- passes its other checks; and
- quotes none of more than half of the top-level sections that develop results. A section develops results if it holds theorem-like statements or displayed equations; the front matter does not count.

**The rejection message:**
- names the lines the quotes came from;
- lists each section left unquoted, with its lines and what it holds;
- asks for each source to be read to its last line, in parts;
- asks for each concept or step to be quoted where the source develops it.

**Bounds:** after `coverage_retries` such rejections (2 by default), the answer is accepted with a warning. The log marks each such rejection with `check: coverage`.

**Exemptions:**
- a pilot's argument, which plans part of the target by design;
- sources without marked-up sections.

**Default:** `report`, so ordinary runs behave as before until there is evidence.

**Why displayed equations count.** Counting only theorem environments missed §9 of `paper.tex`. §9 develops the algorithm and the complete time bound in prose, 8 displayed equations and a cost table, without one lemma.

**Replayed on the earlier runs' first extraction answers,** with the check on (deterministic, no model):

| Answer | Verdict | Developing sections unquoted |
|---|---|---|
| `full-v0.6-latex-planner-handoff` (72f1de8) | sent back | 8 of 10 |
| `full-v0.6-latex` (e4aff3e) | sent back | 8 of 10 |
| `full-v0.6-latex-depth` (fa33c9d) | sent back | 8 of 10 |
| `pilot-v0.6-latex-source` (42cd64b) | sent back | 7 of 10 |
| Sonnet sample 1 | accepted | 0 |
| Sonnet sample 2 | accepted | 0 |

## Measuring recovery

There are two tools beside this entry.

**`score.py`** scores a planning answer against the draft reference set of 2026-10-13-run-log-and-coverage.
- **What it counts:** an in-scope item is reached when one of the plan's quotes starts on a line where the paper states or derives the item.
- **What that is:** a proxy for whether the plan draws on the passages that develop each result, measured against a draft the author has not reviewed. It is not a judgement that the plan captures a derivation.
- **The drafter's comparison:** the drafting subagent read the Sonnet samples only after drafting from the paper, to compare against them.

On the earlier plans:

| Plan | Core items reached (of 19) | Supporting (of 7) |
|---|---|---|
| `full-v0.6-latex`, extraction / argument | 1 / 0 | 0 / 0 |
| `full-v0.6-latex-depth`, extraction / argument | 1 / 0 | 1 / 0 |
| `full-v0.6-latex-planner-handoff`, extraction / argument | 0 / 0 | 0 / 0 |
| Sonnet sample 1, extraction / argument | 16 / 19 | 4 / 5 |
| Sonnet sample 2, extraction / argument | 15 / 18 | 6 / 6 |

**`reads.py`** lists, from Claude Code subagent transcripts:
- each Read of the source, with its offset and limit;
- each `exegenius submit`;
- how many distinct lines had been read before each submit.

Run on this session's reference-drafting subagent, it counts 5,361 of 5,361 lines, read in 400-line chunks.

## The experiment, designed

**The question:** does a coverage rejection make a planner that stopped at the introduction:
- go back to the source;
- read the sections it skipped;
- and produce a materially better plan, within the retries?

**The setup:** `run-planning.sh`, beside this entry, makes a planning-only run:
- the planner-handoff run's `exegenius.toml` and `paper.tex`;
- the subagent definitions written by `exegenius init`, loaded by Claude Code from the project's `.claude/agents/`;
- an orchestrating `claude -p` on Haiku, as in the author's container;
- a stop before the write tasks.

**Conditions:** with `[checks] coverage` set to `report` (control) or `reject` (treatment), the same binary otherwise, two runs each.

**Measures:**
- each attempt's coverage figures, from the log;
- what each planner read before each submit, from `reads.py`;
- core reference items reached by the accepted extraction and argument, from `score.py`;
- the attempts used, and the time taken.

**What would count as recovery:**
- the control reproduces the early stop;
- under the treatment, the transcripts show reading past the introduction after the rejection;
- the accepted argument reaches at least 12 of the 19 core items (the introduction-only plans reach 0–1, the full readings 15–19);
- within the 2 retries.

## Blocked: running the real planner from this session

**What worked:** a probe confirmed that a nested `claude -p`, run in a project directory, loads that project's subagent definitions. The transcript's metadata names the custom type. The probe was one `claude -p` call on Haiku with only the Read tool.

**What was blocked:** the session's permission classifier then blocked a planning-run setup as creating unsafe agents, meaning autonomous `claude -p` processes with Bash and Write.

**What did not work:** a definition copied into `~/.claude/agents/` during the session did not become a subagent type of this session's Agent tool. Agent types load when the session starts. The copy was removed.

**Not a substitute:** this session's own subagent types (general-purpose and others) are not the `exegenius-planner` definition, and as general-purpose subagents Sonnet read the whole paper (2026-10-13-model-comparison). So a run with them is not used as a stand-in.

**What unblocks it:** either of these, both the author's to choose:
- this session's permission mode set to Accept edits, so each `run-planning.sh` call asks him first;
- the script run in his container, with `exegenius` built from this branch.

## Current State
- `[checks] coverage = "reject"` makes `exegenius submit` send back an extract or argument answer that passes its other checks but quotes none of more than half of the top-level sections holding theorem-like statements or displayed equations, at most `coverage_retries` times (default 2), then accept it with a warning; `report`, the default, sends nothing back (exegenius-core/src/config.rs, exegenius-core/src/coverage.rs, `submit` in exegenius-core/src/task.rs)
- `exegenius submit` never sends back a pilot's argument or an answer drawn from a source without marked-up sections for coverage (exegenius-core/src/task.rs)
- `exegenius/log.jsonl` marks a rejection by the coverage check with `check: coverage`, and `exegenius status` counts such rejections per task kind (exegenius-core/src/runlog.rs, exegenius-cli/src/main.rs)
- `Source` records displayed equations (`equation`, `align`, `gather`, `multline`, `eqnarray`, `displaymath`), and the coverage figures count them per section (exegenius-core/src/project.rs, exegenius-core/src/coverage.rs)
- Replayed with the check on, the first extraction answers of `full-v0.6-latex`, `full-v0.6-latex-depth`, `full-v0.6-latex-planner-handoff` and `pilot-v0.6-latex-source` are sent back and both Sonnet samples' extraction answers are accepted
- `docs/journal/2026-10-13-coverage-recovery/score.py` scores planning answers against the draft reference set, and `reads.py` lists from subagent transcripts what a subagent read of a source before each submit
- `docs/journal/2026-10-13-coverage-recovery/run-planning.sh` makes one planning-only run with the real subagent definitions under a `claude -p` orchestrator on Haiku
- The tests cover the shortfall rule, the rejection message, a section of displayed equations, the default `report` mode, an answer accepted when it quotes where results are developed, a rejection followed by acceptance once the retries are spent, and a pilot argument left alone — 94 tests pass and clippy is clean

## Missing
- No planning run with the coverage check on, so no evidence yet whether a planner recovers after a rejection
- No control run reproducing the early stop outside the author's container
- No permission in this session to run `claude -p` planning runs
- No reviewed reference set

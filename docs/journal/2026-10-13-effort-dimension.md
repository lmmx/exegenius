# 2026-10-13: Subagent effort, a dimension the earlier comparisons did not control

The author asked for subagent effort to be added as an explicit dimension of the experiments and of the model-allocation investigation:
1. recover from existing artefacts whether earlier runs recorded it, without inferring it from the model or the outcome;
2. from now on, record each subagent's actual model and effort with its other settings and outcomes, and mark any missing or unobservable field;
3. hold model and effort fixed across E1's arms;
4. after the baseline comparison and E1, propose a bounded effort experiment;
5. consider agent definition × effort interactions only if the results warrant them.

Adaptive effort escalation is a planned research direction, not a feature or a finding.

## What Claude Code records

Every record of a Claude Code transcript (2.1.296) carries `effort` and `perTurnEffort`, and each assistant message names its model. A subagent's `.meta.json` gives its agent type and description. When the orchestrator overrides the model, the `.meta.json` also gives that `model`.

`effort.py`, beside this entry, reports these per subagent and prints "unrecorded" where a transcript has none.

## Recovered

**Experiment runs** (from the transcripts on the wiki branches):

| Experiment | Branch (lmmx/openai-nlogn-wiki) | Subagents | Agent type | Model | Effort |
|---|---|---|---|---|---|
| coverage recovery | `131026-exp-coverage-recovery` | 12 | `exegenius-planner` | `claude-sonnet-5-5` | medium, every turn |
| replication | `131026-exp-coverage-recovery-2` | 24 | `exegenius-planner` | `claude-sonnet-5-5` | medium, every turn |
| depth rejection | `131026-exp-depth-rejection` | 30 | `exegenius-planner` | `claude-sonnet-5-5` | medium, every turn |

**The runners** that orchestrated them ran on `claude-opus-5-5` at medium, the default of a session made with `create_session`. The planners took the runner's effort.

**The two general-purpose Sonnet samples** of 2026-10-13-model-comparison ran in the development session, which runs at high. Each of their eight planning subagents (extraction, background, background redo, argument, two samples each) ran on `claude-sonnet-5-5` at **high**, every turn (`sonnet-samples.tsv`).

**The author's full runs** (`full-v0.6`, `full-v0.6-latex`, `full-v0.6-latex-depth`, `full-v0.6-latex-planner-handoff`) and the 0.1 run have no transcripts on their branches. Their effort is **unrecorded**.

## What this changes

**The content judge's comparison** (2026-10-13-content-judge) set 21 `exegenius-planner` plans against 2 general-purpose Sonnet plans. The arms differed in:
- the system prompt (the planner definition's short body, against Claude Code's general-purpose prompt);
- the tools (`Read, Write, Bash`, against all);
- **the effort (medium, against high)**;
- the binary (1d90c28 and f70a919, against 930d40c);
- the orchestrator's prompt (the samples' prompt added "Do not read any other files under exegenius/ and do not run any other exegenius command");
- the orchestrator (a runner, against the development session).

**The interpretation that the gap is the agent configuration's** must therefore include effort as an equally plausible cause.

**E1 holds effort fixed:**
- both arms at high, by the per-call `effort` override, with the model fixed by the per-call `model` override;
- the transcripts are checked afterwards with `effort.py`.

**What E1 then tests:**
- **If the planner at high matches the general-purpose subagent at high,** the earlier gap is not the definition's. Effort becomes the leading candidate, to be tested directly (planner at medium against high).
- **If the general-purpose subagent still wins,** the definition's differences are tested one at a time.

## Recording from now on

**The run log records no model or effort** (2026-10-13-model-allocation). Until it does, every experiment:
- keeps its transcripts on its branch;
- reports each subagent's agent type, model and effort with `effort.py`;
- reports both the orchestrator's and the subagents' effort.

**The model-allocation proposal's profiles** (model plus effort per profile, recorded on `issued` events) are the durable form. They stay unimplemented until reviewed.

## Current State
- Transcripts of Claude Code 2.1.296 record `effort` and `perTurnEffort` on every record, and the model on every assistant message
- All 66 `exegenius-planner` subagents of the three planning experiments ran on Sonnet at medium effort, and their runner sessions on Opus at medium
- The eight subagents of the two general-purpose Sonnet planning samples ran on Sonnet at high effort
- `docs/journal/2026-10-13-effort-dimension/effort.py` reports agent type, model and effort per subagent, and its outputs for these runs are beside it

## Missing
- No record of effort for the author's full runs or the 0.1 run
- No model or effort in exegenius's run log
- No controlled comparison of effort at a fixed model and agent definition

# 2026-10-13: Where a stronger model could change outcomes: investigation and proposal

The author asked for an investigation, run autonomously after the merges and the content judge, of where larger models would add the most value in the pipeline, with these aims:
- to examine the current subagent definitions, and how task instructions are coupled to agent identity and model choice;
- to find where separating task specifications from execution profiles would allow controlled model comparisons, and later adaptive allocation: "low / medium / high" effort, narrowed down to where it pays, as in delta debugging;
- to estimate token and subscription use with stated uncertainty;
- to propose concrete stages and experiments, without refactoring the model-selection architecture before review.

This entry is that investigation and its proposal. **Nothing in it is implemented.**

## How models are chosen now

**The three definitions** `exegenius init` writes (`exegenius-cli/src/agents/`):

| Definition | Model | Used for | Instructions |
|---|---|---|---|
| `exegenius-planner` | `sonnet` | extract, background, argument, extend | do the task file, submit until accepted |
| `exegenius-task` | `inherit` | write, audit, outline, overloads, support, goals, grade, attribute | word for word the same as the planner's |
| `exegenius-reader` | `haiku` | simulated readers | read the wiki only through `exegenius read` and `search` |

**The coupling:**
- **The instructions are already the task file's.** The planner and task definitions differ only in their model line, so each "identity" is already an execution profile in all but name. Only the reader definition carries a role of its own.
- **Which definition a task kind goes to is hard-coded:** `subagent()` in exegenius-cli/src/main.rs, which `next` prints.
- **Each model is fixed in a definition file.** A person can edit it, but `init` rewrites it.
- **`exegenius-task` inherits the orchestrator's model.** Writers, auditors and support judges therefore change model whenever the orchestrating session does: Haiku in the author's container, the development session's model in a runner. This is a hidden variable in any comparison of runs.
- **The run log records no definition, model or effort** for a task, so no outcome can be attributed to one.
- **The decision behind the planner's Sonnet routing (a539829) rests on a confounded comparison** (2026-10-13-model-comparison). Every experiment since has held the planner on Sonnet, so nothing has measured a model's effect on planning.

**What Claude Code supports** (the sub-agents and model-configuration pages, read through a documentation subagent):
- **In a definition:** `model` (an alias, a full model id, or `inherit`) and `effort` (`low`, `medium`, `high`, `xhigh`, `max`, depending on the model). There is no per-subagent thinking setting.
- **Per call:** the orchestrator can override a subagent's `model` and, from 2.1.292, its `effort`.
- **Visibility:** `/tasks` shows each subagent's model and effort. No per-subagent token view is documented, so token use comes from transcripts.

## What the evidence says about where capability matters

**Observed in earlier entries:**
- **The planning failure is the agent configuration's, not the model's.** The `exegenius-planner` definition on Sonnet stopped at the introduction in 7 of 11 clean runs, and 5 of 10 in the depth experiment, and rejections recovered it (2026-10-13-recovery-experiment).
- **The content gap is large at a fixed model** (2026-10-13-content-judge). The two plans by general-purpose Sonnet subagents stated all 19 core results and derived 11–12. Every `exegenius-planner` plan derived 0–4, on the same model family. The differences between them: the system prompt (general-purpose against the planner's short definition), the orchestrator, and the binary (930d40c against 1d90c28 and f70a919). With two samples, the comparison is not controlled.
- **Auditing:** Sonnet caught a cited bound standing in for a page's mechanism in 1 of 2 audits, Haiku in 0 of 2. Sonnet's audits took 24–29 seconds and about 56k tokens, Haiku's 150–175 seconds and about 90k (2026-10-13-model-comparison). The larger model was cheaper in time and tokens on that task.
- **Writing:** Haiku writers given an explicit note derived correctly in both prototypes (2026-10-13-placement-prototype). The failures lay in planning and auditing.

**Interpretations, not shown:**
- **The largest lever measured so far is how the planning agent is configured,** not which model it runs.
- **A model comparison made before the configuration is settled would measure the wrong thing.**

## Cost baseline

**Per planning task** (`usage.py`, beside this entry). Medians over 22 runs of each planning task in the three experiments, all `exegenius-planner` on `claude-sonnet-5-5`:

| Task | Model calls | Fresh input | Cache read | Cache write | Output (estimated) | Seconds |
|---|---|---|---|---|---|---|
| extract | 9 | 18 | 162,544 | 39,847 | 6,647 | 71 |
| background | 4 | 8 | 29,434 | 10,585 | 1,926 | 23 |
| argument | 9 | 19 | 253,145 | 50,204 | 15,601 | 144 |

**The output column is an estimate.** Transcripts record a message's usage when it starts streaming, so their `output_tokens` undercount, sometimes 8 tokens for 30,000 characters of reasoning. `usage.py` takes the input side from the records and estimates output at 4 characters per token. A precise count would need OpenTelemetry's `claude_code.token.usage`, which tags subagent use, or API-side usage.

**Per experiment, as the runner sessions report it.** The cloud session records give each runner's usage, observed after the runners were archived:

| Runner | Planning runs | Cache read | Cache write | Output | Reported cost (USD) |
|---|---|---|---|---|---|
| `131026-exp-coverage-recovery` | 4 | 9.5 M | 0.54 M | 0.15 M | 5.40 |
| `131026-exp-coverage-recovery-2` | 8 | 14.7 M | 0.92 M | 0.23 M | 8.12 |
| `131026-exp-depth-rejection` | 10 | 16.8 M | 1.25 M | 0.33 M | 10.22 |

These totals are three to four times the subagent medians above (about 4.5 M read from cache for ten runs), so they include the runner's own orchestration on Opus: building, copying, collecting and writing its report. Whether they count every subagent's use in full is not documented. They are an upper anchor for an experiment's cost; the subagent medians are the planning itself.

**Volume in a full run.** The planner-handoff run's 212 tasks:
- 95 write and 73 audit tasks;
- 34 citation-support checks;
- 7 planning tasks (extract, background twice, argument twice, extend twice);
- an outline task and two overloads tasks.

Writers and auditors therefore dominate volume and cost. Planning is a few tasks that decide what every page derives.

**Subscription share.** It is not estimated here. A plan's limits are not a published token allowance, so a percentage would need calibration against `/usage` readings taken before and after runs. What can be reported with confidence: tokens by kind and by model, from transcripts or OpenTelemetry; and an API-price-equivalent cost at the current published prices, with the output estimate's uncertainty carried through.

## Proposal: execution profiles, kept apart from task specifications

**Not implemented; for review.**

1. **Profiles in `exegenius.toml`.**
   - **Definition:** for example `[profiles.low] model = "haiku"`, `[profiles.medium] model = "sonnet"`, `[profiles.high] model = "opus", effort = "high"`.
   - **Allocation:** an `[allocation]` table maps each task kind to a profile. Its defaults reproduce today's mapping, except that `inherit` gives way to an explicit profile, ending the hidden dependence on the orchestrator.
   - **`init`:** writes one definition per profile, `exegenius-low`, `-medium` and `-high`, with the shared instructions. The reader definition stays a role of its own.
   - **`next`:** names the profile's definition.
   - **The task file:** unchanged. It remains the specification.
2. **The run log records the profile, model and effort** of every `issued` event. Outcomes, rejections and coverage can then be attributed to a profile. A collector joins transcripts to task ids, from the task id in each subagent's description, so tokens can be attributed too.
3. **Escalation on failure:** the delta-debugging step.
   - **The policy:** every kind starts at its profile. When a check sends a task's answer back k times (the coverage check, or the answer's own checks), `next` reissues the task one profile higher, and the log records the escalation and its outcome.
   - **What it shows:** over runs, the log shows where escalation changes outcomes, task kind by task kind. That narrows the expensive profile to where it pays.
   - **In the first runs:** escalation is recorded but not acted on (shadow mode), until the evidence supports it.
4. **The investigator stays apart from the policy.** Allocation is configuration, pinned per experiment on dated branches. Only a reviewed commit changes the default allocation, never the investigating session on its own evidence.

## Proposal: experiments, in order

Each experiment uses the runner protocol of 2026-10-13-handover-after-merges, five runs per arm, and is judged by the blind content judge (2026-10-13-content-judge) and the run log. Costs are projected from the baseline above.

| | Question | Arms (all with `coverage = "reject"`) | Measures | Projected cost |
|---|---|---|---|---|
| E1 | Is the planning gap the agent configuration? | argument (and extraction) by `exegenius-planner` on Sonnet, against a general-purpose subagent on Sonnet given the same prompt | content judge, coverage, tokens, time | 10 planning runs (extraction, background, argument): about 4.5 M tokens read from cache, 1.0 M written to cache and 0.24 M output (estimated), from the medians; with orchestration, about the depth experiment's runner (10 USD reported); plus 10 judgements |
| E2 | At the better configuration, does the model matter? | that configuration on Haiku, Sonnet, Opus | as E1 | 15 runs |
| E3 | Does effort matter at a fixed model? | the better configuration on Sonnet at `effort` medium against high | as E1 | 10 runs |
| E4 | Do writers need a larger model? | write tasks for a fixed sample of 10 pages from one fixed plan, on Haiku and on Sonnet, audited by one fixed auditor profile | audit gaps, owned-step derivations, a page judge (to be built, like the content judge) | 20 writes and 20 audits |
| E5 | Do auditors need a larger model? | audit tasks for the same pages, on Haiku and on Sonnet | gaps found against a reviewed list of planted and known gaps | 20 audits per arm |

**Ordering:**
- **E1 first:** it tests the largest effect measured, and decides whether planning needs a stronger model at all.
- **E4 and E5 next:** they decide most of a run's cost.

**Profile changes** follow only from these results, through the escalation log.

## Current State
- Task kinds map to the `exegenius-planner` (`model: sonnet`), `exegenius-task` (`model: inherit`) and `exegenius-reader` (`model: haiku`) definitions in `subagent()` of exegenius-cli/src/main.rs, and the planner and task definitions have identical instructions
- The run log records no subagent definition, model or effort for a task
- `docs/journal/2026-10-13-model-allocation/usage.py` reports model calls, input-side tokens and estimated output tokens per subagent from Claude Code transcripts
- Runner sessions of 4, 8 and 10 planning runs reported 5.40, 8.12 and 10.22 USD, including their own orchestration
- The planning stage of a run on `paper.tex` takes medians of about 9 model calls, 163k tokens read from cache and 71 seconds for extraction, and 9 calls, 253k and 144 seconds for the argument, on `exegenius-planner` with Sonnet

## Missing
- No execution profiles: models are fixed per definition, and writers and auditors inherit the orchestrator's model
- No controlled comparison of model, effort or agent configuration on any task
- No page judge for comparing writers
- No calibration of token use against subscription limits

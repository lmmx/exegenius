# Reflections on the reading failure: outline

Edited notes from a recording Louis Maddox made on 2026-10-13, while reading the failure analysis of the planner and handoff run (2026-10-13-planner-handoff-run). The first person is the author's. The notes are not a transcript: they keep the reasoning, reactions, qualifications and open questions, and drop the asides.

Where a point was checked against the repository or the run records after the recording, the note says so under **Checked since**. Each note marks what it holds as one of:
- an observation, from the records;
- an interpretation, which fits the observations but is not shown;
- a proposal, not yet a decision;
- an open question.

The short entry that links these notes is `../2026-10-13-reading-failure-reflections.md`.

## Sections

1. [Context: where this started](01_context.md). The setup, how the project grew from 0.1 to 0.6, and the question of whether Haiku can do it. My view now: the problems are qualitative, and a larger model would amplify them, not fix them.
2. [Reading the diagnosis](02_reading-the-diagnosis.md). My reactions to the analysis as I read it:
   - the planner's token count;
   - one subagent per kind of step, with models matched to difficulty;
   - "laziness" and the system prompt;
   - why I did not see it sooner;
   - the switch to restricted subagents;
   - what "buried the lede".

   With the records checked since.
3. [Verification inside the loop](03_verification-in-the-loop.md). Checking each stage's output before the next stage uses it. Fail fast as something to intensify, not to switch on. Recovery before failure. A preflight check of the setup itself.
4. [Deterministic checks from a single command](04_deterministic-batch-checks.md). An agent states a check once, and a program runs it over every claim. Pipelines and data engineering.
5. [Logging in ordinary runs](05_logging.md). What a run reports about itself, for the agent running it. Tests and one-off experiments as checks after the fact.
6. [Adapting the work to the source](06_adaptive-fan-out.md). One reader, or several, by the size of the source, and what splitting costs.
7. [What the run was meant to test](07_the-handoff-and-what-to-measure.md):
   - the owned-step handoff, barely exercised;
   - gap counts as a measure;
   - the depth rule, and Chesterton's fence;
   - why "better than the status suggests" is not the point.
8. [How I want to work](08_how-i-want-to-work.md):
   - taking myself out of the loop;
   - reading output written for agents;
   - an agent that runs the experiments;
   - recording my reasoning, not my approvals;
   - where these notes go.

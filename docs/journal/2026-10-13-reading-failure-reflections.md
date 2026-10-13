# 2026-10-13: The author's reflections on the reading failure

The author recorded his reactions while reading 2026-10-13-planner-handoff-run, the failure analysis of `full-v0.6-latex-planner-handoff`. The long-form notes, edited from the recording, are in `2026-10-13-reading-failure-reflections/`, starting from [the outline](2026-10-13-reading-failure-reflections/00_outline.md). This entry summarises them and records what is settled, what is proposed and what is open.

## What the recording adds to the analysis

The analysis supplied the diagnosis. The recording supplies a direction for the design, and a view on how the work is done:

- **Not mainly a question of model size** ([01](2026-10-13-reading-failure-reflections/01_context.md)).
  - The failure is qualitative: how work is set up and handed between stages.
  - A hypothesis, untested: a larger model given the same setup could amplify it.
  - Model capability and agent configuration are to be judged separately.
- **A pipeline, checked as one** ([02](2026-10-13-reading-failure-reflections/02_reading-the-diagnosis.md), [03](2026-10-13-reading-failure-reflections/03_verification-in-the-loop.md)).
  - Each stage's output should be checked before the next stage consumes it.
  - Failing fast is to be intensified, not switched on.
  - A failed check should lead to recovery, with failure only when recovery is impossible.
- **Deterministic checks** ([04](2026-10-13-reading-failure-reflections/04_deterministic-batch-checks.md)). An agent states a check once as a command, and a program runs it over every claim. Model judgement is kept for what needs it.
- **Logging in every run** ([05](2026-10-13-reading-failure-reflections/05_logging.md)). Runs report what the agent needs to see a regression as it happens. That complements controlled experiments, which still establish causes.
- **Work that adapts to the source** ([06](2026-10-13-reading-failure-reflections/06_adaptive-fan-out.md)). Fan-out across subagents for large sources, weighed against overlap and merging.
- **The variable under test was lost** ([07](2026-10-13-reading-failure-reflections/07_the-handoff-and-what-to-measure.md)).
  - The run did not test the owned-step handoff.
  - Gap counts do not measure what matters.
  - Improvements are wanted, not reassurance.
- **How the author wants to work** ([08](2026-10-13-reading-failure-reflections/08_how-i-want-to-work.md)).
  - Out of the loop for running experiments, with oversight of direction kept.
  - Reports that put the conclusion first.
  - A record of his reasoning rather than of his approvals.

## Established

- **How the planner read.** In `full-v0.6-latex-planner-handoff`, the extract planner (Sonnet, `exegenius-planner`) listed the outline of `paper.tex`, read lines 1–440 in one Read call with `limit: 440`, and read no more. Its evidence quotes all lie in those lines (2026-10-13-planner-handoff-run).
- **When the restricted definitions came in.** With 0.6.0 (8cff21f, 2026-10-11), for two recorded reasons:
  - three of 38 baseline-pilot subagents called a cloud-session tool that a prompt did not keep them from (2026-10-11-baseline-pilot, 2026-10-11-reduction);
  - the general-purpose subagent's fixed context of 42.5k–46.0k tokens (2026-10-11-wall-time).
- **Writers were made to read less, on purpose.** In the same pass, write tasks began carrying the source paragraphs around evidence quotes. The noted risk was that writers would read less around a concept. Nothing then considered how much the planning tasks read.
- **Introduction-heavy extraction predates the definitions.** The 0.5.0 Haiku extraction, made before the definitions existed, has 22 of its 32 evidence quotes in the first tenth of `paper.txt`. The three extractions under the definitions have 21 of 23, 24 of 27 and 27 of 27. The Sonnet comparison samples, run as general-purpose subagents, have 37 of 82 and 29 of 88 (computed from the extraction answers on the wiki repository's branches).
- **The run did not test the handoff.** The owned-step section reached 13 of the run's 190 write tasks.

## Interpretations, not shown

- **Laziness.** The `exegenius-planner` system prompt may have dropped whatever kept the general-purpose agent reading. Supplementing the system prompt, rather than replacing it, might restore that.
- **Buried lede.** With plans drawn from the introduction, every audit round has to bring in the rest of the paper through rewrites. That could explain rounds that added rather than tightened.
- **The depth rule.** Round 3's cite-or-cut decisions on the $s<Wm$ derivation follow from the plan not owning it. They do not follow from the size of a model.

## Proposals, not decisions

- **Subagents.** One per step where a step needs it, with a model matched to its difficulty.
- **Checks between stages.** In-loop checks after extraction and planning, with diagnosis and recovery before failure. Which conditions are hard failures, warnings or recovery triggers is undecided.
- **Coverage checks.** Judged against what the target depends on and where the source develops it, not against a fixed share of the paper.
- **Batch checks.** A command an agent writes once, run by a program over every claim.
- **Logging.** Structured, emitted in ordinary runs and readable by the running agent, with a flag to quieten it.
- **A preflight check** of the configuration before a costly run, one form of which is a dry run.
- **Fan-out.** Adaptive reading across subagents for large sources, to be tried only if a single planner told to read section by section cannot cover the source.
- **A decision record** for the switch to restricted subagents.

## Open questions

- Why the extract and argument planners each stopped at the introduction (lines 440 and 436):
  - the definition's system prompt;
  - the task's wording, which never says to read the whole source;
  - the model;
  - or a combination.
- How the model and the agent definition each bear on coverage and plan quality. Planning runs that vary one at a time would separate them.
- What a good plan for this paper contains: the results and derivations a plan must capture, against which planning runs can be scored beyond their quote spread.
- Which existing checks a program can establish, and which need judgement.
- How the owned-step handoff performs given a plan that covers the paper.

## Actions

**Done:**
- the transcripts of the run's extract planner and argument planner (task 0004), read and recorded in 2026-10-13-planner-handoff-run: each read only the introduction, and the argument planner's pages also came from the extraction;
- lmmx/openai-nlogn-wiki added to the development session, for reading;
- the history of the restricted definitions and the extraction coverage of earlier runs, checked against the journal, the commits and the wiki repository's branches;
- these notes and this entry.

**Open, not yet approved:**
- optionally, read the transcript of a Haiku extraction under `exegenius-task`;
- write a decision record for the restricted definitions;
- define the results and derivations a plan of this paper should contain;
- design the planning experiment against them, separating model from definition;
- design the in-loop checks and the logging before implementing either.

## Current State
- In `full-v0.6-latex-planner-handoff`, the extract planner read lines 1–440 of the 5,361-line `paper.tex` with one Read call (`limit: 440`), after listing the file's outline, and all its evidence quotes come from those lines
- `exegenius init` has written subagent definitions limited to Read, Write and Bash since 0.6.0 (8cff21f), which introduced them after three of 38 baseline-pilot subagents used a cloud-session tool (exegenius-cli/src/agents/)
- The 0.5.0 Haiku extraction, made without subagent definitions, has 22 of 32 evidence quotes in the first tenth of `paper.txt`
- The long-form notes of the author's recording are in `docs/journal/2026-10-13-reading-failure-reflections/`, with `00_outline.md` and eight numbered sections

## Missing
- No record of why the extract and argument planners in `full-v0.6-latex-planner-handoff` stopped reading at the introduction
- No decision record for the switch to restricted subagent definitions
- No reference set of the results and derivations a plan of the paper should contain
- No check, during a run, of how much of a source a planning task read

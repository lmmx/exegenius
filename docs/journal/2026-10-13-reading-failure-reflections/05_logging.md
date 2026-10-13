# 5. Logging in ordinary runs

## Experiments after the fact

The analysis proposed an experiment to separate the causes:
- **the work:** the planning tasks only;
- **the conditions:** four, Haiku and Sonnet, each under the planner definition and as a general-purpose subagent;
- **the measures:** tokens, the quote spread by tenth of the paper, the size of the argument, and whether the butterfly layer's recursion and the derivation of $s<Wm$ have owners.

**What I would rather have.** The measurements that experiment needs should not be a one-off. They should be part of what the program reports on every run, as logging. Then a regression shows up when it happens, rather than through a test made afterwards.

Unit tests, as I see them here, are a form of experiment after the fact. What is wanted is logging that the tools emit, because the agent running the loop sees it and can act on a regression during the run, rather than someone noticing it during development.

## What kind of logging

**Not a debug mode.** It should be produced in the normal state, marked as a log, with a flag to quieten it. I would hope for something better than the rudimentary configuration of logging levels; that is for whoever designs it.

**For the agent, not only the developer.** ChatGPT put the requirement as making the pipeline's behaviour legible to the agent itself, so that it can decide whether to continue, retry, split a task, change strategy or stop. It suggested questions a run's log should be able to answer:
- which task is running, on which model and definition;
- what parts of the source were read, and what evidence supports each extracted result;
- how much work each stage has used;
- which checks failed, what recovery was tried, and whether it worked;
- whether the output is degrading against a baseline.

Its further points:
- the log should be structured and machine-readable, with a human-readable summary;
- verbosity should change what is shown, not discard what is needed to reconstruct a failure.

**Status:** proposal. No schema is designed.

**What the run records today:**
- `exegenius status`;
- the task and answer files;
- the subagent table, which this run's prompt asked the orchestrating agent to keep in `run-notes.md`.

No run records what a planning task read.

## The planning experiment

Logging complements controlled experiments rather than replacing them: logs make a regression visible while a run is under way, and a controlled experiment is still what establishes its cause. The planning experiment is worth running, and it should use that logging. It should also be run by an agent rather than by me (08_how-i-want-to-work.md).

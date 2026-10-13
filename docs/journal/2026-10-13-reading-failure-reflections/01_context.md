# 1. Context: where this started

## The program and the setup

exegenius reads a source document, here a large LaTeX file describing a result in integer multiplication, and turns it into a wiki. The wiki's structure is not a plain knowledge graph. It follows the paper's proof, as people who study mathematical proofs model one: a plan of steps to show and steps to infer, which a reader can follow to an understanding of the result. The writing works like an FAQ in one respect: each page is written for what a simulated reader of the wiki would next want to know.

**The tool.** exegenius is a compiled Rust command-line tool. It does not call a model itself: it produces task files, checks answers and builds the wiki. A coding agent drives it. `exegenius init` sets up the agent's side, including the subagent definitions, and the agent runs the loop of tasks.

**The runs:**
- Claude Code, on Haiku, runs inside a Podman container that can see one folder: the paper's repository (lmmx/openai-nlogn-wiki).
- I build the tool from the feature branch that a Claude Code cloud session develops, install it with Cargo, and hand it to the container.
- Each experiment is a branch of the wiki repository.
- I set up each run by hand from the same starting state, rather than letting `init` and a fresh extraction decide everything. That way, a difference between two runs is not just the difference between two samples.

**Two assistants.** The development is a two-way partnership:
- the Claude Code session writes the software;
- a ChatGPT chat reads the same journals and results and helps me interpret them, without editing code.

I carry messages between the two.

**The loop.** A run extracts the concepts the target depends on. It filters out those the reader is assumed to know: they get no page, but stay in the graph as grounding taken on trust rather than shown. It then writes a page for each remaining concept. Three rounds follow of auditing the pages, extending to more concepts where needed, and rewriting to close the gaps the audits report.

## From 0.1 to 0.6

Version 0.1 was very simple, and it worked: it converged. By 0.5 it was diverging. I wondered whether the concept had grown too complex for a small model, since I had used Haiku throughout.
- **Why Haiku:** it is good at reading documents and retrieving information from them, which is mostly what it is used for (reading web pages, for instance).
- **What it is weaker at:** sustained agentic work with tools.

The latest run looked like a failure of convergence in the same way. Over three rounds of writing and auditing, the audits did not report fewer gaps.

> **Checked since.** The run's gap totals were 93, 100 and 100 over rounds 1 to 3 (`run-status.txt` on `full-v0.6-latex-planner-handoff`). On the 24 pages audited in all three rounds, step and symbol gaps went 82, 83, 84. So the gaps held level, rather than growing.

## Model size and qualitative problems

My first reading of the analysis was that Haiku was not carrying out the agentic parts well, and that the simplest course might be Sonnet everywhere, dropping to Haiku only where justified. I thought Sonnet had converged before.

> **Checked since.** No full run has used Sonnet throughout. Sonnet has done only the planning tasks: twice in the comparison, as general-purpose subagents, and once in this run, under the planner definition.

By the end of reading it I had come round to a different view. Model size is a quantity, and what is going wrong here is qualitative: how the work is set up and handed on. My hypothesis is that a larger model given the same setup might take longer and produce more output, and could amplify the same failure rather than correct it. If so, it was as well that I did not move to a larger model earlier: the failure might have been more expensive and no clearer. None of this is tested.

**Status:** interpretation. The run's records show that the planner read only the introduction; they do not show what a larger model would do under the same definition. The diagnosis's version of this point:
> A supervisor that read only the introduction would plan the same way.

That holds for any model that reads the same way. It does not settle whether a stronger model would choose to read further, which is part of the planning experiment (02_reading-the-diagnosis.md).

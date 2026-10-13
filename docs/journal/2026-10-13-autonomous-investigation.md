# 2026-10-13: Handing the investigation to the development session

Until now each step of the investigation has gone through the author:
- one Claude Code session developed exegenius;
- runs happened in the author's container;
- transcripts were read by a second, local session;
- a ChatGPT chat reviewed the results;
- the author carried findings between all of them.

The author asked to stop being that coordination layer. From here the development session leads the investigation end to end, with the author keeping architectural oversight (2026-10-13-reading-failure-reflections, note 08).

**The aim is a system, not more analyses after the fact.** exegenius should notice when a stage's work is inadequate, find out why, attempt a recovery, and leave a clear record of what happened, as an ordinary part of a run. The measurements the investigation needs belong in the tool as logging and checks, where the agent running the loop can see and act on them. Controlled experiments still have a place, to settle a design decision, and are run by the development session, not by hand.

## Access, as of this entry

- **exegenius:** this session reads and pushes `timing-and-parallelism`.
- **lmmx/openai-nlogn-wiki:** attached to the session and cloned at `/home/user/openai-nlogn-wiki`, as well as the earlier read-only clone at `/home/user/lmmx/openai-nlogn-wiki`.
  - **Reads work:** the session can read every branch's pages, task files, answers, `argument.json`, `graph.json`, `run-notes.md` and `paper.tex`, alongside the exegenius code.
  - **Pushes are refused**, because the Claude GitHub App has no access to this repository. Installing it on lmmx/openai-nlogn-wiki would allow pushes, if experiment branches are to live there.
- **Not reachable from this session:** the author's container, and the subagent transcripts of runs made there. Runs made in this session leave their own transcripts here.

## Authorship

**Commits:**
- authored and committed as Louis Maddox <louismmx@gmail.com>;
- no Co-Authored-By trailer or other attribution line.

**Pull request descriptions:**
- no "Generated with" line or session link;
- checked after creation, since the harness's instructions add them by default.

**Checked on this entry's date:** all 59 commits on `timing-and-parallelism` since `master` meet the commit rule. The description of pull request #8 has no attribution.

## The proposed approach (for the author's approval, not yet started)

**The question it serves:** why exegenius made a shallow plan when the whole source was available, and what in the architecture would catch and recover from that. 2026-10-13-planner-handoff-run records what is known: both planning tasks chose to read only the introduction, and the argument also inherited the extraction's narrow pages. Why they stopped is not known.

### In the pipeline, on every run

**1. A run log written by exegenius.**
- **What it records:** each task handed out (kind, id, the subagent named), each submit (accepted, or rejected with reasons), and each stage's diagnostics. The log is machine-readable, at `exegenius/log.jsonl`.
- **Where it shows:** summarised in `exegenius status`, and as warnings in `next` output, where the running agent sees them.
- **What exegenius cannot see:** model tokens and tool calls, which only the orchestrating agent and the transcripts hold.

**2. A coverage diagnostic for planning answers, computed at submit.**
- **From the source's structure:** sections and theorem environments, which `project.rs` already reads in LaTeX.
- **From the answer's evidence:** where its quotes fall.
- **What it reports:**
  - which sections hold quoted evidence;
  - which sections hold theorems or lemmas with none;
  - concepts named after a later section but evidenced only in the introduction.

**3. Recovery through the submit loop that already exists.**
- **The mechanism:** a submit rejection already sends the subagent back to fix its answer. A coverage rejection would say:
  - which lines the evidence came from;
  - which sections hold the proof's results;
  - that the answer must quote where each concept is developed, reading the file in pages.
- **Escalation:** after a set number of attempts, the answer is accepted with the shortfall recorded in the log and the status.
- **Undecided:**
  - thresholds;
  - which conditions are hard failures and which are warnings;
  - whether the argument task gets the same check.

**4. The same pattern, later, for other stages.** For example:
- how far the owned-step handoff reached;
- anchored passages that state a step rather than derive it;
- notation drift along a chain.

### In a test and evaluation harness, outside ordinary runs

**5. A reference set for this paper.** A checked list of the results and derivations a plan must capture:
- the butterfly layer's recursion;
- the interchange recurrence;
- $s<Wm$;
- Gaussian resampling;
- the time accounting.

It would be stored in the exegenius repository as an evaluation fixture, and used to score plans beyond where their quotes fall.

**6. Planning-only runs under stated conditions**, scored against the reference set and read through the run log:
- **The questions:** first, whether the coverage check of item 3 recovers a full extraction. Only if it does not: whether the cause is the task's wording, the agent definition or the model.
- **Where they run:** in this session as far as possible. A faithful `exegenius-planner` run needs the definition loaded as a subagent type, which this session may not support.

### The order

1. Build items 1 and 2 as logging only: no rejection, nothing changed in what a run does.
2. Read the log on planning-only runs in this session.
3. Turn on the recovery of item 3 and see whether it recovers.

Each step answers the decision before the next: whether verification and recovery inside the loop is workable for this failure. The full runs and the four-condition model comparison wait on that answer.

## Decisions for the author

- **Approval** of the approach and its order.
- **Where experiment artefacts live:**
  - in lmmx/openai-nlogn-wiki, which needs the Claude GitHub App installed there for pushes;
  - or in the exegenius repository beside the journal, as the pilots' artefacts already are.
- **Who runs the definition conditions:** whether any planning run must use the real `exegenius-planner` definition in the author's container, or whether this session's approximations are enough until a full run.

## Current State
- lmmx/openai-nlogn-wiki is attached to the development session and cloned at `/home/user/openai-nlogn-wiki`; reads work and pushes are refused
- All 59 commits on `timing-and-parallelism` since `master` are authored and committed as Louis Maddox <louismmx@gmail.com> with no co-authorship trailers, and the description of pull request #8 has no attribution line
- `exegenius submit` checks that planning quotes exist in the source and appear in order, and nothing measures which parts of the source a planning answer draws on (exegenius-core/src/extract.rs, exegenius-core/src/argument.rs)

## Missing
- No run log written by exegenius
- No coverage diagnostic or coverage rejection for planning answers
- No reference set of the results and derivations a plan of the n log n paper should contain
- No push access from the development session to lmmx/openai-nlogn-wiki

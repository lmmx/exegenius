# 2026-10-13: Handing the investigation to the development session

Until now each step of the investigation has gone through the author:
- one Claude Code session developed exegenius;
- runs happened in the author's container;
- transcripts were read by a second, local session;
- a ChatGPT chat reviewed the results;
- the author carried findings between all of them.

The author asked to stop being that coordination layer. From here the development session leads the investigation end to end, with the author keeping architectural oversight (2026-10-13-reading-failure-reflections, note 08).

**The aim is a system, not more analyses after the fact.** exegenius should notice when a stage's work is inadequate, find out why, attempt a recovery, and leave a clear record of what happened, as an ordinary part of a run. The measurements the investigation needs belong in the tool as logging and checks, where the agent running the loop can see and act on them. Controlled experiments still have a place, to settle a design decision, and are run by the development session, not by hand.

This entry is also the handover across a compaction of the development session's context. It records the state and decisions that the session's conversation held, and points to the entries that hold the evidence.

## State of the repositories, as of this entry

**lmmx/exegenius**
- **Branch:** `timing-and-parallelism`, pushed, the head of open pull request #8 into `master`. The session reads and pushes it, and the working tree is clean.
- **Code, against 0.6.0:**
  - a539829: planning tasks go to `exegenius-planner` (`model: sonnet`);
  - ec620eb: the owned-step section in write tasks, and `derivations` checked at submit.
- **Everything since** is journal entries and their records.
- **Tests:** 78 pass (2 CLI, 67 unit, 9 end-to-end); clippy is clean.

**lmmx/openai-nlogn-wiki**
- **Clones:**
  - `/home/user/openai-nlogn-wiki`, attached to the session, on `master` (e3d0322);
  - `/home/user/lmmx/openai-nlogn-wiki`, an earlier anonymous clone, holding every branch at depth 1.
- **Push access:**
  - refused at first, because the Claude GitHub App had no access to the repository;
  - after the author changed the App's permissions, a dry-run push of a new branch from the attached clone was accepted;
  - no real push has been made yet.
- **Branches:**

| Branch | Head | What it holds |
|---|---|---|
| `tmp-v0.5-build-record-stash` | fa6c804 | the 0.5.0 Haiku run's `exegenius/` directory (2026-10-11-divergent-run) |
| `pilot-v0.6-convergence` | 4b921b6 | the 0.6.0 reduction pilot |
| `pilot-v0.6-depth`, `pilot-v0.6-first-level`, `pilot-v0.6-reader-check` | 7839ae2, 6698d4c, ea8751b | pilots of 2026-10-11 and 2026-10-12 |
| `pilot-v0.6-latex-source` | 42cd64b | the LaTeX pilot |
| `full-v0.6` | 901a730 | full run on `paper.txt` |
| `full-v0.6-latex` | e4aff3e | full run on `paper.tex` |
| `full-v0.6-latex-depth` | fa33c9d | the explanation-depth run, the planner run's baseline |
| `full-v0.6-latex-planner-handoff` | 72f1de8 | the Sonnet planner and owned-step handoff run |
| `master` | e3d0322 | |

**Records that exist only in the repositories.** The development session's scratch directory goes when its container does. What the investigation needs from it was copied into the journal:
- the pilots' tasks, pages, audits and scripts, including the audit harness that renders and checks audit tasks outside a run: `2026-10-13-brief-pilot/` and `2026-10-13-owned-step-handoff/`;
- the two Sonnet planning samples, the only plans so far made from a reading of the whole paper: `2026-10-13-model-comparison/sonnet-sample-1/` and `sonnet-sample-2/`.

## Authorship

**Commits:**
- authored and committed as Louis Maddox <louismmx@gmail.com>;
- no Co-Authored-By trailer or other attribution line.

**Pull request descriptions:**
- no "Generated with" line or session link.

**Why this needs watching:** the harness's own instructions add those lines by default. Commits therefore set the identity explicitly, and pull request descriptions are checked after creation. If the identity or the attribution cannot be controlled, the session stops and says so.

**Checked:** all commits on `timing-and-parallelism` since `master` meet the commit rule. The description of pull request #8 has no attribution.

## What is established, and what is not

**Established.** From the records and the transcripts (2026-10-13-planner-handoff-run):
- **In `full-v0.6-latex-planner-handoff`, both planning tasks read only the introduction:**
  - the extract planner read lines 1–440 of `paper.tex`, and the argument planner lines 37–436;
  - each after listing the 5,361-line file's outline;
  - each at a read limit it set itself;
  - all their quotes come from those lines.
- **The argument also inherited the extraction's narrow set of pages.**
- **The task wording:** neither task says to read the whole source or each section. Submit checks only that quotes exist and appear in order.
- **The bias predates the restricted definitions:** the 0.5.0 Haiku extraction already drew 22 of its 32 quotes from the first tenth of the paper.
- **Full reading:** the two Sonnet samples, run as general-purpose subagents, read the whole paper and produced 79- and 83-step arguments.
- **The comparison that motivated a539829 is confounded:** it changed the model and the agent setup at once (2026-10-13-model-comparison).
- **The owned-step handoff:** reached 13 of the run's 190 write tasks, so the run did not test it. Its only evidence is the pilot and the replication: 5 of 6 pages with the handoff derived their owned multi-step step, against 0 of 6 without.

**Not established:**
- why either planner stopped reading: the task wording, the definition's system prompt, the model, or a combination;
- whether a planner told that its reading was incomplete would go back and read;
- whether quotes from the right sections amount to coverage of the proof. A plan can quote widely and still miss a derivation. This is why a reference set of required results is part of the proposal below.

## The goal, and the current proposal

**The goal.** The author's direction, recorded in 2026-10-13-reading-failure-reflections: verification, diagnosis and recovery inside the pipeline, with logging the running agent can act on, and checks made by programs where no judgement is needed. Not a larger model.

**The proposal below is not approved.** It is the development session's suggested first step toward that goal, and the author is to review it before any of it is built.

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
- **Its limit:** where quotes fall is a proxy for coverage, not a measure of it.

**3. Recovery through the submit loop that already exists.**
- **The mechanism:** a coverage rejection would say:
  - which lines the evidence came from;
  - which sections hold the proof's results;
  - that the answer must quote where each concept is developed, reading the file in pages.
- **Escalation:** after a set number of attempts, the answer is accepted with the shortfall in the log and the status.
- **Undecided:**
  - thresholds;
  - hard failures against warnings;
  - whether the argument task gets the same check.

**4. The same pattern, later, for other stages.** For example:
- the reach of the owned-step handoff;
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

**6. Planning-only runs under stated conditions**, scored against the reference set and read through the run log. The first question is whether item 3 recovers a full extraction. Only if it does not: the wording, the definition or the model.

### The suggested order

1. Items 1 and 2 as logging only: nothing changed in what a run does.
2. Their output on planning-only runs.
3. Item 3 turned on, to see whether it recovers.

## Decided, and awaiting the author

**Decided by the author:**
- the development session leads the investigation, and the author keeps architectural oversight;
- the direction of the work: integrated verification, diagnosis, recovery and agent-visible logging, with controlled experiments where they settle a decision, not manual experimentation as the product;
- no move to a larger model for now;
- the authorship rule above;
- the depth policy and the 350-word limit unchanged, for the planner and handoff run (2026-10-13-planner-handoff-run-setup).

**Awaiting the author:**
- approval of the proposal and its order, or a revision;
- where experiment artefacts live: branches of lmmx/openai-nlogn-wiki, now that pushes appear to work, or the exegenius journal's directories;
- whether any planning run must use the real `exegenius-planner` definition in the author's container;
- what to do about a539829, which routes planning to Sonnet on the strength of the confounded comparison. It stays in the code until decided.

## What this session can run, and what needs the author's container

**In the development session:**
- **What it can do:**
  - build and run exegenius;
  - read every branch of the wiki repository, including `paper.tex`;
  - start subagents of the built-in types (general-purpose and others) on Haiku, Sonnet or Opus;
  - render and check task answers with the saved audit harness.
- **What it has not done:** used the `exegenius-planner` or `exegenius-task` definitions as subagent types. The session's subagent types do not include them, and whether definitions written mid-session would load is unverified. A run here approximates a definition by its instructions and a request to use only its tools. That is not the same system prompt, so behaviour such as stopping early may not reproduce.

**In the author's container:**
- **The setup:**
  - runs under Podman, on Claude Code with Haiku as the orchestrating session;
  - the definitions written by `exegenius init`;
  - exegenius installed at `/workspace/bin/exegenius`;
  - the wiki project at `/mnt/openai-nlogn`.
- **Transcripts:** under `/home/node/.claude/projects/-workspace/<session>/subagents/agent-*.jsonl`, each beside a `.meta.json` naming its `agentType` and description. The run of 72f1de8 is session `464895b1-7931-4805-8d04-45d99ad5e87a`.
- **What only it can test:** the real definitions. The development session cannot read these transcripts.

## The author's working preferences

- **Ownership:** the development session owns the investigation and the routine coordination, and reports decision points concisely, conclusion first, with terms explained.
- **Status:** observations, hypotheses, proposals and approvals stay distinct in what is reported and recorded.
- **Transcripts:** read only when the answer changes a decision.
- **The author's role:** architectural oversight, not relaying findings between agents.
- **Records of reasoning:** the author's reasoning is to be kept as long-form notes beside the journal (2026-10-13-reading-failure-reflections/), not reduced to approvals.

## Known problems in exegenius, not yet addressed

Each is recorded in the entry named:
- **Audit answers:** 12 of 16 first Haiku audit answers in the pilots were rejected for anchoring symbols to known concepts that do not bind them (2026-10-13-planner-handoff-run-setup).
- **Write rejections,** most often for:
  - symbols used before their binding sentence;
  - relation ids written as `name.md`;
  - word counts (2026-10-13-planner-handoff-run).
- **Wording:** the write task lists earlier pages among the concepts the reader "already understands", which writers misread (2026-10-13-brief-pilot).
- **Notation drift** along chains of pages, from overload renamings that later pages' bindings block (2026-10-13-planner-handoff-run).
- **A content error:** the leaf bound on `recursive-power-saving` in 72f1de8 does not follow (2026-10-13-planner-handoff-run).
- **The 350-word limit** binds on pages whose owned step has many sub-steps (2026-10-13-owned-step-handoff).

## Current State
- `timing-and-parallelism` of lmmx/exegenius holds 0.6.0 with the Sonnet planner routing (a539829) and the owned-step handoff (ec620eb), and journal entries since; pull request #8 is open from it into `master`
- lmmx/openai-nlogn-wiki is attached to the development session and cloned at `/home/user/openai-nlogn-wiki`; a dry-run push of a new branch was accepted after the author changed the Claude GitHub App's permissions
- All commits on `timing-and-parallelism` since `master` are authored and committed as Louis Maddox <louismmx@gmail.com> with no co-authorship trailers, and the description of pull request #8 has no attribution line
- The two Sonnet planning samples of 2026-10-13-model-comparison are stored in `docs/journal/2026-10-13-model-comparison/`
- `exegenius submit` checks that planning quotes exist in the source and appear in order, and nothing measures which parts of the source a planning answer draws on (exegenius-core/src/extract.rs, exegenius-core/src/argument.rs)

## Missing
- No run log written by exegenius
- No coverage diagnostic or coverage rejection for planning answers
- No reference set of the results and derivations a plan of the n log n paper should contain
- No real push from the development session to lmmx/openai-nlogn-wiki yet
- No verified way to run the `exegenius-planner` definition from the development session

# 2026-10-14 (speculative): The discovery exercise, repeated across model generations

**Status: a proposal, derived from strands D and E of the Leviathan conversations (`docs/chats/leviathan/`, files 07–10, 19–21). Not run. It depends on the author's choice of when to spend a long session, and on Fable for the comparison it is designed around.**

## The idea from the archive

The archive ends with two artefacts meant for reuse:
- a prompt (file 19) that takes four strands one at a time, then brings them into contact, then follows whatever discovery emerges into the repository;
- a library of 18 deliberately different intents (file 21) that a simulated author might bring to it.

The author's aim (files 07, 09, 16) is a repeatable exercise in which a model makes its *own* conceptual move about Exegenius — the kind of move the prinz collection and *Import AI 476* report frontier models beginning to make in mathematics — rather than an exercise whose answer is prescribed.

File 10 made the distinction this needs:
- **Discovery taste:** inventing a new way to see the problem.
- **Research taste:** choosing and interpreting experiments, which is what TasteVal measures.

One does not imply the other.

## The proposed exercise

**One run is one model, at one effort, given:**
- file 19's prompt;
- the then-current repository;
- one or two intents sampled from file 21, stated as what this run's author wants.

**Each run's record keeps three things apart:**

| Dimension | What is recorded |
|---|---|
| Discovery | what it saw that was not in the prompt or the repository's existing journals, judged against them |
| Judgement | which direction it chose among the alternatives it named, and why |
| Impact | what changed in the repository, and whether the change survived a test |

**What a run must not see:** the outputs of earlier runs. That way, independent convergence and genuinely different moves can both be seen.

## Why it suits the calibration programme

- **Its unit is the run,** like every other experiment here, and §6a's rules apply:
  - one run per model per intent;
  - a stated decision;
  - a budget;
  - repeats only where they could change a conclusion.
- **It has an obvious first comparison:** today's strongest configuration against Fable (`calibration/plans/FB1.md` proposes testing Fable on the pipeline; this would test it on the development process).
- **It needs the author.** Discovery is the hardest dimension to judge. An LLM judge rating novelty mostly rates persuasiveness, so the discovery column should be judged by the author, or by checking against what the repository already contained.

## What to decide before a first run

- **Strand coverage:** whether to keep file 19 single-turn, or stage it one strand per turn. The companion entry on the sequential, relational method proposes a small test of exactly that, and its answer should come first.
- **The intent library:** which intents from file 21 to sample first. The archive groups them into seven personas: philosopher, mathematical explainer, experimental researcher, software architect, AI futurist, autonomous collaborator, builder of a cumulative practice.
- **Budget:** a long session per run, with repository access, at the model's highest useful effort; S2 is measuring what effort above `high` costs.

## Current State
- File 19's prompt and file 21's intent library exist; no run has used them
- The calibration protocol's sequential rules (§6a) and budget ledger (`calibration/budget.md`) give the exercise a frame for cost and stopping

## Missing
- No run on any model, and so no baseline for a Fable comparison
- No agreed way to judge the discovery dimension beyond the author's own reading

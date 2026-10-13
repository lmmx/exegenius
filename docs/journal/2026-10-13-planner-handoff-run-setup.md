# 2026-10-13: Setup for the planner and handoff run

After 2026-10-13-owned-step-handoff, the author and ChatGPT chose to run the combined system:
- the planner on Sonnet (2026-10-13-planner-agent);
- the owned-step handoff (ec620eb).

## What the run tests

The run tests the combined system, not either change alone. Each change has its own evidence:
- **The planner:** 2026-10-13-model-comparison.
- **The handoff:** 2026-10-13-brief-pilot and 2026-10-13-owned-step-handoff.

The baseline stays `full-v0.6-latex-depth` (fa33c9d).

## Decisions

- **Word limit:** unchanged at 350 words. Budget pressure on pages that own a multi-step derivation is a result to report, not a setting to tune. No limit that scales with ownership yet.
- **Audit:** the task and its model are unchanged, for comparability with the baseline.
  - Anchors show where a writer claims to derive a step, not that it does.
  - The audit judges the reasoning.
  - The audit does not see the anchors in this run.
- **Out of scope until the results are reviewed:**
  - Opus as a supervisor;
  - reusing a writer for repairs.

## A known problem in the audit, recorded and not fixed

In the two pilots, 12 of the 16 first Haiku audit answers were rejected for anchoring a symbol to a known concept that does not bind it:
- `O` to "Big-O notation";
- `\log` to "Logarithms and exponentials";
- `s`, `W` and `m` to "Rank saving $s<Wm$";
- `r` to "Polynomial ring modulo $y^r+1$".

Every one passed after the rejection was sent back, once or twice. In a run each costs a resubmission. A narrow fix will be proposed after the run, from the run's own counts. Changing the audit protocol now would break the comparison.

## Verified setup

**The baseline.** On lmmx/openai-nlogn-wiki, checked on 2026-10-13:
- **Branch:** `full-v0.6-latex-depth` is at fa33c9d, "Full run of exegenius 0.6.0 (LaTeX source, explanation depth)".
- **No clash:** no branch `full-v0.6-latex-planner-handoff` exists.
- **`exegenius.toml` there:**
  - `rounds = 3`, `stop_early = false`, `[simulation] rounds = 0`;
  - the target and reader as in the depth run;
  - one source, `sources/paper.tex`.
- **Other files:** `sources/` also holds `paper.pdf` and the paper's source directory, which no `[[sources]]` entry names.
- **Agent definitions:** `.claude/agents/` holds the depth run's two, `exegenius-task` (`model: inherit`) and `exegenius-reader` (`model: haiku`).

**A dry run of the sequence below** on a copy of that branch, with exegenius built at e8f46a4, whose code is ec620eb:
- `init` kept `exegenius.toml` unchanged and wrote three definitions:
  - `exegenius-planner`, `model: sonnet`;
  - `exegenius-task`, `model: inherit`;
  - `exegenius-reader`, `model: haiku`.
- `next` handed out `0001-extract` to `exegenius-planner`.

**The models in the run:**

| Tasks | Subagent | Model |
|---|---|---|
| extract, background, argument, extend | `exegenius-planner` | Sonnet |
| write, audit, outline, overloads, support, goals, grade, attribute | `exegenius-task` | the session's: Haiku |
| read, baseline (none: simulation is off) | `exegenius-reader` | Haiku |

## The command sequence

```sh
cd ~/exegenius && git pull && git log --oneline -1   # e8f46a4 or later
just install
grep -ac "Steps this page derives" "$(which exegenius)"   # at least 1: the handoff build (0 before it)
cd /mnt/openai-nlogn
git fetch origin && git checkout full-v0.6-latex-depth && git log --oneline -1   # fa33c9d
git status --short                                    # empty
git checkout -b full-v0.6-latex-planner-handoff
git rm -rq exegenius docs zensical.toml run-status.txt run-notes.md
git commit -m "Clear the explanation-depth run's output"
exegenius init
grep -H '^model' .claude/agents/*.md   # planner: sonnet, reader: haiku, task: inherit
git diff --stat HEAD -- exegenius.toml   # empty
git add .claude && git commit -m "exegenius init: planner on Sonnet, owned-step handoff"
claude --model haiku
```

## The prompt

> Run the full exegenius LaTeX-source experiment in this directory on branch `full-v0.6-latex-planner-handoff`, following the exegenius skill, until exegenius next prints done. I'm going to work: don't ask me anything and don't wait for me.
>
> The source is `sources/paper.tex` (the paper's LaTeX, assembled with latexpand). Extraction runs fresh: do task 0001-extract like any other task.
>
> When exegenius next shows the reader, run once:
>   exegenius background --redo "The reader has also taken the calculus such a degree requires: limits, convergence of sequences and series, and basic integrals."
> and then carry on.
>
> This is the full run, not the five-page pilot. Use the existing target and source configuration. Run all 3 audit rounds; `stop_early = false` is intentional, so do not turn it back on. Simulated-reader evaluation remains disabled (`[simulation] rounds = 0`). Use the subagent type exegenius next names for each task: planning tasks go to `exegenius-planner`. Do not change the agent definitions.
>
> Give every subagent the full path to the exegenius binary in its prompt. Start each task exactly once: before starting a subagent, check that exegenius next still lists the task as pending and that no running subagent has it.
>
> To every write task's prompt, add: "After the submit result, on new lines, say how many times submit rejected your answer and why, anything in the task that was missing, unclear or in conflict with the sources, and anything you could not fit in the word limit; or 'none'."
>
> For every subagent, record in `run-notes.md` a table row: task id, subagent type, total tokens and duration as reported when it finished, and for write tasks the subagent's reply after "accepted".
>
> Don't edit anything under `sources/`, `exegenius.toml`, `.claude/`, or anything else by hand except answer files. Don't read git history. If a command fails in a way you can't fix, stop and write what happened to `run-notes.md` at the repo root.
>
> At the end, write `exegenius status` to `run-status.txt`, write a short `run-notes.md` (anything that went wrong, any rejections you had to work around, and the table), and commit everything on this branch as "Full run of exegenius 0.6.0 (LaTeX source, Sonnet planner, owned-step handoff)". Don't push.

**Changes from the depth run's prompt:**
- the planner routing;
- full binary paths and starting each task once (the depth run lost two audit answers to duplicate launches);
- the token table;
- the extra line for write tasks, which reports rejections and budget pressure.

That line also changes the writers' prompts, so it is a third difference from the baseline, though a small one.

## Evaluation, against `full-v0.6-latex-depth`

1. **The chain:** butterfly layer → interchange → $s<Wm$, each link derived where the plan puts it, each step checked against `paper.tex`.
2. **Regressions:** `gaussian-resampling` (Lemma 19's construction, hypotheses and constants) and `tape-interchange` (the recurrence and its exponent), or their counterparts under this run's names.
3. **Owned derivations.** For each page whose owned steps have steps beneath them: whether each owned step is derived completely and correctly, read against the paper. The count reported is of derivations judged complete and correct, not of anchors.
4. **Anchors:** how many anchored passages state a step rather than derive it.
5. **Duplication and unnecessary derivations:** steps owned by other pages derived again, and results a page only uses derived in full.
6. **Page length and budget pressure:**
   - words against the limit;
   - budget rejections on pages owning a multi-step derivation;
   - for each one, whether the reasoning that did not fit was essential (a step of an owned derivation) or verbose (filler, repetition, another page's material). This is judged from the writer's report and the final page.
7. **Process:**
   - tasks handed out, accepted and rejected, with reasons;
   - the audit rejections for symbol anchors;
   - the gap table by round;
   - runtime.
8. **Cost:**
   - tokens and time per task kind;
   - for write tasks, whether the quality gained is worth the 32–70% more writer tokens the replication measured;
   - whether the section could be shorter.

## Current State
- The baseline branch `full-v0.6-latex-depth` is at fa33c9d, and no branch `full-v0.6-latex-planner-handoff` exists
- On a copy of the baseline, the command sequence above leaves `exegenius.toml` unchanged, writes the planner on Sonnet, the task definition on the session's model and the reader on Haiku, and hands out the extract task to the planner

## Missing
- No run with the planner on Sonnet and the owned-step handoff
- No fix for audit answers that anchor symbols to known concepts that do not bind them

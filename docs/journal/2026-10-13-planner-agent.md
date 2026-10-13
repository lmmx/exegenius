# 2026-10-13: A planner subagent on Sonnet

The results of 2026-10-13-model-comparison fall in the third row of ChatGPT's table: planning succeeds and auditing is inconsistent. That row's decision is to route planning to Sonnet, keep writers on Haiku, and leave audits as they are.

The author and ChatGPT approved a narrow change:
- one new subagent definition;
- routing of the four planning task kinds to it;
- two tests;
- nothing else.

The depth policy stays as it is, so that the next run isolates the planner. Audit models are unchanged, and there is no general routing by model.

## The change

**`exegenius-planner`.** `init` now writes a third definition to `.claude/agents/`:
- the same body as `exegenius-task`;
- the same tools: Read, Write and Bash;
- `model: sonnet`.

**Routing.** `next` names the subagent for each task. A new function makes the choice:
- extract, background, argument and extend go to `exegenius-planner`;
- read and baseline go to `exegenius-reader`, as before;
- every other kind goes to `exegenius-task`, as before: write, audit, outline, overloads, support, goals, grade and attribute.

`exegenius-task` keeps `model: inherit`, so writers and auditors run on the session's model. That was Haiku in every run so far.

**The model is a setting in the file.** exegenius names the definition and never reads its `model:` line. Claude Code reads it when the subagent starts. So to change the planner's model, edit that line in `.claude/agents/exegenius-planner.md` after `init`; running `init` again rewrites it.

Checked on a scratch project:
- `next` printed the extract task with `subagent: exegenius-planner`;
- with the line edited to `model: opus`, `next` printed the same.

**Tests.**
- `planning_tasks_go_to_the_planner`: the four planning kinds go to the planner; write, audit, outline, overloads and goals go to `exegenius-task`; baseline goes to `exegenius-reader`.
- `init_writes_the_planner_on_sonnet`: `init` writes the planner with `model: sonnet` and the task definition with `model: inherit`.

**The documents.** The README and the skill name the three definitions.

## The run proposed

**The setup.** A new branch from `full-v0.6-latex-depth`, with the run's output removed:
- the same `paper.tex`;
- the same `exegenius.toml`: `rounds = 3`, `stop_early = false`, no simulated readers;
- the same calculus correction;
- the session on Haiku.

**The difference.** Only the planner: extraction, background and argument run on Sonnet.

**Not comparable page for page.** Extraction runs fresh, as in every run so far. Pages will differ in name and number from the baseline's. The comparison follows the chain and the named mechanisms, not page ids.

## What it is judged on

Against `full-v0.6-latex-depth`.

**The chain.** Does a reader get from the butterfly layer, through interchange, to $s < Wm$, with each link derived where the plan puts it?
- **The butterfly layer:**
  - the layer's page derives its recursion: the phase kernel, the recurrence $F(e) \le (s/W)F(e/m) + O((eK)^\tau + 1)$, and its sum to $O(d^{\lambda'})$;
  - it does not cite that bound in place of its mechanism.
- **Interchange:**
  - the layer's page reaches the interchange page through the graph;
  - it states what interchange gives it, without repeating that proof.
- **The count:**
  - $s < Wm$ is derived once, on the page the argument assigns it to, as $s = Wm - N + 2L$ with $L < N/2$;
  - elsewhere it is cited.
- **Agreement with the paper:** each step of the chain is checked against `paper.tex`.

**Regressions.**
- **`gaussian-resampling`:** still gives Lemma 19's construction, hypotheses and constants.
- **`tape-interchange`:** still derives the interchange recurrence and its exponent.

**Costs of a larger plan.**
- **Duplication:** a derivation given in full on more than one page.
- **Unnecessary derivations:** a result the page only uses, derived there in full.
- **Page length:** words against budget, overall and on pages presenting a step.
- **Size:** concepts, pages, and pages presenting a step.

**Process.**
- tasks handed out, accepted and rejected, with the reasons for rejection;
- the gap table by round;
- runtime;
- model tokens per task kind.

The baseline recorded no per-task tokens, so cost is compared only where both runs have it: tasks and runtime (234 tasks in 67 minutes).

## Later, not now

This change makes the planner stronger. It does not make it a supervisor: the writer still gets only what the write task already carries from the plan. 2026-10-13-planner-handoff separates the two, and inspects that handoff.

Two experiments are left for later, on the evidence of this run:
- an explicit brief for a Haiku writer, on `butterfly-layer`;
- audits that correct rather than reject.

The second depth level is given to every page presenting a step. If the larger plan makes that apply too broadly, restrict it to steps to show.

## Current State
- `exegenius init` writes three subagent definitions: `exegenius-task` (`model: inherit`), `exegenius-planner` (`model: sonnet`) and `exegenius-reader` (`model: haiku`)
- `exegenius next` names `exegenius-planner` for extract, background, argument and extend tasks, `exegenius-reader` for read and baseline tasks, and `exegenius-task` for every other task
- A subagent's model is the `model:` line of its definition, which exegenius does not read, and which `init` rewrites

## Missing
- No run with the planner on Sonnet
- No per-task token records in any run

---
name: exegenius
description: Run the exegenius pipeline in this directory, turning the sources listed in exegenius.toml into an explanatory wiki for the stated reader. Use when asked to run exegenius, to build, continue or finish the wiki, or to run it until complete.
---

# exegenius

`exegenius` is a deterministic CLI that holds all the state (concept graph, reading order,
pages, audit results) and checks every answer. It makes no model calls: you do the reading
and writing it asks for, task by task, until it reports done.

## Loop

1. Run `exegenius next` in this directory. It prints either a batch of tasks or `done`.
2. The tasks in a batch are independent of each other. Hand each one to its own subagent,
   all in parallel (one Agent tool call per task, all in a single message), using the
   prompt `exegenius next` prints for it. Each subagent writes its answer file and runs
   `exegenius submit <id>` until the answer is accepted. Claude Code runs at most 20
   subagents at once by default: for a larger batch, start 20 and start another each time
   one finishes.
3. Use the subagent type `exegenius next` names for each task: `exegenius-planner` for
   the planning tasks (extract, background, argument, extend), `exegenius-reader` for
   simulated readers, and `exegenius-task` for the rest. `exegenius init` writes all three to
   `.claude/agents/`, each with its model and the only tools it may use. Audit tasks must
   each go to a fresh subagent that has seen nothing else: the auditor stands in for a
   reader who has not read the sources or the other pages. Do not do audits yourself.
4. When every task in the batch is accepted, go back to step 1. Keep no tally of your own:
   each time a subagent finishes, run `exegenius next`, which lists the tasks still
   pending, and start a subagent for any pending task none is working on. End your turn
   only while subagents are running, never with a pending task no subagent has.
   Once, early in the run, `exegenius next` prints the reader instead of tasks: the reader
   profile as every later task will be told it, and the concepts counted as known. If the
   user is following the session, show it to them and ask whether it describes the reader
   they mean; if they correct it, run `exegenius background --redo "<their correction>"`.
   If no one can answer (under `claude -p`, or told to run unattended), do not wait: go
   back to step 1.
5. Once the pages are final, `exegenius next` hands out the outline task (group the pages
   into parts, fold minor pages into a glossary, write the front page) and, if a page
   reuses a letter an earlier page bound, the overloads task (judge whether each reused
   letter keeps its meaning), as one batch of independent tasks.
6. Then come the simulated readers: a goals task, then rounds of read and baseline tasks
   (each read task explores the wiki with `exegenius read` and `exegenius search`), grade
   tasks, and rewrites of the pages readers got stuck on. Every read, baseline and grade
   task needs its own fresh subagent, `exegenius-reader` for read and baseline tasks.
   Never do them yourself: you have seen the sources and pages.
7. When `exegenius next` prints `done`, the wiki is in `docs/` with a `zensical.toml`, and
   the simulated readers' results are in `exegenius/report.md`. Run `exegenius status`
   and report it.
8. If the user wants the wiki online, run `exegenius init --vercel`: it adds `vercel.json`
   and `.gitignore` entries, so a GitHub repository of this folder imported into Vercel
   builds and deploys the site on every push.

## Rules

- Run the loop until `exegenius next` prints `done`. exegenius sets no budget of time, tasks
  or tokens, and a batch of a hundred tasks is normal for a large wiki: do not stop to ask
  whether to go on. (`budget` under `[simulation]` in `exegenius.toml` is the most pages a
  simulated reader may open per goal, nothing else.) Stop early only if a command fails in
  a way you cannot fix, and report anything that looks wrong, such as pages on concepts the
  reader already knows, when you report the status at the end.
- Only write answer files (`exegenius/tasks/*.answer.*`) and, for a task, scratch files in
  its own directory (`exegenius/tasks/<task id>/`). Do not edit anything else under
  `exegenius/` or `docs/` by hand.
- Without subagents, do the tasks yourself one at a time, and do each audit task using only
  its task file.
- `exegenius status` shows progress at any point; `exegenius plan` shows the reading order.

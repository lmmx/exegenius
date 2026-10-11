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
   `exegenius submit <id>` until the answer is accepted.
3. Audit tasks must each go to a fresh subagent that has seen nothing else: the auditor
   stands in for a reader who has not read the sources or the other pages. Do not do
   audits yourself.
4. When every task in the batch is accepted, go back to step 1.
5. Once the pages are final, `exegenius next` hands out the outline task (group the pages
   into parts, fold minor pages into a glossary, write the front page) and, if a page
   reuses a letter an earlier page bound, the overloads task (judge whether each reused
   letter keeps its meaning), as one batch of independent tasks.
6. Then come the simulated readers: a goals task, then rounds of read and baseline tasks
   (each read task explores the wiki with `exegenius read` and `exegenius search`), grade
   tasks, and rewrites of the pages readers got stuck on. Every read, baseline and grade
   task needs its own fresh subagent, on the smallest model available (model haiku) for
   read and baseline tasks. Never do them yourself: you have seen the sources and pages.
7. When `exegenius next` prints `done`, the wiki is in `docs/` with a `zensical.toml`, and
   the simulated readers' results are in `exegenius/report.md`. Run `exegenius status`
   and report it.
8. If the user wants the wiki online, run `exegenius init --vercel`: it adds `vercel.json`
   and `.gitignore` entries, so a GitHub repository of this folder imported into Vercel
   builds and deploys the site on every push.

## Rules

- Only write answer files (`exegenius/tasks/*.answer.*`). Do not edit anything else under
  `exegenius/` or `docs/` by hand.
- Without subagents, do the tasks yourself one at a time, and do each audit task using only
  its task file.
- `exegenius status` shows progress at any point; `exegenius plan` shows the reading order.

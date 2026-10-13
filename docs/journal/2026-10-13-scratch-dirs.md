# 2026-10-13: A scratch directory for each task

**The incident.** In the replication of the recovery experiment (2026-10-13-recovery-experiment), planner subagents in different runs wrote their answer-generating scripts to the same file, `/tmp/claude-0/gen.py`:
- one planner submitted another run's draft as its own;
- another planner overwrote a third run's answer file after that answer had been accepted.

**Why it matters beyond the experiment.** Tasks that exegenius hands out together run at the same time on one machine. In a full run that is up to 20 write or audit subagents, so the same sharing can happen there.

**The change.** On branch `131026-scratch-dirs`, kept apart from the feature branch until it is reviewed:
- `next` creates `exegenius/tasks/<task id>/` for each task;
- the task file asks the agent to keep any other files it makes, such as scripts, in that directory, and says why: files elsewhere, in /tmp for example, can be overwritten or run by other tasks.
- The skill's rule on what an agent may write under `exegenius/` now allows these directories.

**Its limit.** This is an instruction, not an enforcement. An agent can still write elsewhere, and nothing checks that it did not.

## Current State
- `exegenius next` creates `exegenius/tasks/<task id>/` for each task it hands out, and every task file asks for scratch files to be kept there (`Task::scratch`, `render` and `issue` in exegenius-core/src/task.rs)
- The skill allows writing scratch files in a task's own directory besides answer files (exegenius-cli/src/skill.md)

## Missing
- No check that an agent kept its scratch files in its task's directory
- No run since the change showing whether agents follow it

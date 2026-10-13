# 2026-10-13: Stopping one task from killing another's commands

**The incident.** In the depth-rejection experiment (2026-10-13-depth-rejection), one extraction planner hung on a `cat >` waiting for standard input. To stop it, the planner ran `pkill -f "cat >"`, which matches every process on the machine whose command line contains `cat >`. Planners of other tasks write files with `cat > file <<'EOF'`, so one task's agent could stop another's command half-way. Nothing else was interrupted that time.

**The change**, on branch `131026-process-safety`, not yet merged:
- `exegenius init` adds `Bash(pkill *)` and `Bash(killall *)` to `permissions.deny` in the project's `.claude/settings.json`. It keeps any settings and deny rules already there, adds only the missing rules, and leaves a file that is not valid JSON untouched, printing what to add by hand.
- Task files add a sentence beside the one about scratch directories: never stop processes by name or pattern, which would stop other tasks' commands too; stop a command of your own by its process id.

**What the Claude Code documentation says about deny rules** (permissions and sub-agents pages, read through a documentation subagent):
- **Syntax:** the rules are written in the space form, which the permission dialog itself writes; `:*` is documented as equivalent.
- **Reach:**
  - deny rules apply to subagents;
  - they apply in every permission mode, `bypassPermissions` included;
  - no allow rule can carve out an exception;
  - they match inside compound commands (`&&`, `;`, `|`, subshells).

**The guard's limit.** A deny rule does not match the same program run another way: through `bash -c '…'`, `env pkill`, or a full path. The documentation says Bash rules are not a security boundary. The instruction in the task file is the second layer. Neither layer is enforcement against an agent determined to stop processes.

## Current State
- `exegenius init` merges `Bash(pkill *)` and `Bash(killall *)` into `permissions.deny` of `.claude/settings.json`, keeping existing settings (`deny_rules` in exegenius-cli/src/main.rs)
- Task files ask agents never to stop processes by name or pattern, and to stop their own commands by process id (`render` in exegenius-core/src/task.rs)
- The test `init_denies_stopping_processes_by_pattern_and_keeps_existing_settings` covers merging into existing settings, running `init` twice, and leaving invalid JSON alone (exegenius-cli/src/main.rs)

## Missing
- No guard against `pkill` run through `bash -c`, `env` or a full path
- No run since the change showing agents keep to it

#!/bin/bash
# One planning-only run of exegenius on the n log n paper, with the real subagent
# definitions that `exegenius init` writes, orchestrated by `claude -p` on Haiku as in the
# author's container. Stops before the write tasks. Collects the run log, the answers and
# the subagent transcripts into the run directory.
#
# usage: run-planning.sh RUN_DIR report|reject WIKI_CHECKOUT
#   RUN_DIR        a new directory for this run
#   report|reject  the [checks] coverage setting
#   WIKI_CHECKOUT  a clone of lmmx/openai-nlogn-wiki with branch
#                  full-v0.6-latex-planner-handoff fetched (for exegenius.toml and paper.tex)
# needs: `exegenius` built from timing-and-parallelism (with [checks]) and `claude` on PATH
set -euo pipefail
run=$1 mode=$2 wiki=$3
mkdir -p "$run/p/sources"
run=$(cd "$run" && pwd)
git -C "$wiki" show origin/full-v0.6-latex-planner-handoff:exegenius.toml > "$run/p/exegenius.toml"
git -C "$wiki" show origin/full-v0.6-latex-planner-handoff:sources/paper.tex > "$run/p/sources/paper.tex"
printf '\n[checks]\ncoverage = "%s"\n' "$mode" >> "$run/p/exegenius.toml"
cd "$run/p"
exegenius init > /dev/null
exegenius --version > "$run/versions.txt"
claude --version >> "$run/versions.txt"
start=$(date +%s)
timeout 3600 claude -p "Use the exegenius skill to run exegenius in this directory, with one \
change: stop as soon as \`exegenius next\` hands out write tasks, and do not start them. No \
one is following this run. When you stop, reply with the output of \`exegenius status\`." \
  --model haiku --allowedTools "Bash Read Write Edit Agent Task Skill" \
  < /dev/null > "$run/orchestrator.txt" 2>&1 || echo "claude exited $?" >> "$run/orchestrator.txt"
echo "seconds: $(( $(date +%s) - start ))" >> "$run/orchestrator.txt"
exegenius status > "$run/status.txt" 2>&1 || true
# Claude Code keeps a project's transcripts under its path with / and . replaced by -.
transcripts="$HOME/.claude/projects/$(pwd | sed 's#[/.]#-#g')"
cp -r "$transcripts" "$run/transcripts" 2>/dev/null || echo "no transcripts at $transcripts" >> "$run/orchestrator.txt"

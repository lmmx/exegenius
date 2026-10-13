#!/bin/bash
# usage: ./collect.sh
# Copies this session's Claude Code transcripts (main and subagents) into results/transcripts,
# each run's `exegenius status` into runs/RUN/status.txt, and versions into results/.
set -euo pipefail
root=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$root/results/transcripts"
for dir in "$HOME"/.claude/projects/*/; do
  cp -r "$dir" "$root/results/transcripts/" 2>/dev/null || true
done
for run in "$root"/runs/*/; do
  (cd "$run" && exegenius status > status.txt 2>&1 || true)
done
{ exegenius --version; claude --version 2>/dev/null || true; git -C "${EXEGENIUS_SRC:-/home/user/exegenius}" rev-parse HEAD 2>/dev/null || echo "exegenius clone not found"; } > "$root/results/versions.txt"
du -sh "$root/results/transcripts"

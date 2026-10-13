#!/bin/bash
# usage: ./setup-run.sh RUN [EXTRA_TOML_FILE]
# Makes runs/RUN from template/: exegenius.toml (plus the lines of EXTRA_TOML_FILE, if given,
# for an arm whose configuration differs) and the sources. The subagent definitions and
# settings are the ones in .claude/ at the repository root, written by `exegenius init` of the
# binary under test.
set -euo pipefail
run=$1
root=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$root/runs/$run"
cp -r "$root/template/sources" "$root/runs/$run/sources"
cp "$root/template/exegenius.toml" "$root/runs/$run/exegenius.toml"
if [ $# -ge 2 ]; then printf '\n' >> "$root/runs/$run/exegenius.toml"; cat "$2" >> "$root/runs/$run/exegenius.toml"; fi
echo "runs/$run ready"

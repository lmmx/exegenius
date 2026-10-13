"""Model and effort of each subagent in Claude Code transcripts.

Usage: python3 effort.py DIR...

Walks DIR for `subagents/agent-*.jsonl`. For each subagent it reports the agent type and
description (from the `.meta.json` beside it), the models of its assistant messages, and the
`effort` and `perTurnEffort` values its records carry. A value missing from every record is
reported as "unrecorded", never inferred.
"""

import collections
import json
import pathlib
import sys


def values(path):
    models, effort, per_turn = collections.Counter(), collections.Counter(), collections.Counter()
    for line in path.open():
        try:
            record = json.loads(line)
        except json.JSONDecodeError:
            continue
        message = record.get("message")
        if record.get("type") == "assistant" and isinstance(message, dict) and message.get("model"):
            models[message["model"]] += 1
        if "effort" in record:
            effort[str(record["effort"])] += 1
        if "perTurnEffort" in record:
            per_turn[str(record["perTurnEffort"])] += 1
    return models, effort, per_turn


def show(counter):
    return ", ".join(f"{k} x{v}" for k, v in sorted(counter.items())) or "unrecorded"


def main(dirs):
    print("\t".join(["transcript", "agent type", "description", "models", "effort", "perTurnEffort"]))
    for d in dirs:
        for path in sorted(pathlib.Path(d).rglob("subagents/agent-*.jsonl")):
            meta_path = path.with_suffix(".meta.json")
            meta = json.loads(meta_path.read_text()) if meta_path.exists() else {}
            models, effort, per_turn = values(path)
            print("\t".join([
                str(path.relative_to(d)),
                meta.get("agentType", "unrecorded"),
                meta.get("description", "unrecorded"),
                show(models),
                show(effort),
                show(per_turn),
            ]))


if __name__ == "__main__":
    main(sys.argv[1:])

"""What each subagent read of a source, attempt by attempt, from Claude Code transcripts.

For each subagent transcript (agent-*.jsonl, with its .meta.json beside it), lists the
Read calls on SOURCE (offset and limit) and the `exegenius submit` commands, in order, and
for each stretch before a submit, how many distinct lines of SOURCE it had read so far.
Reads of SOURCE by other means (sed, grep, cat) are listed as Bash commands that name it.

usage: python3 -I reads.py TRANSCRIPTS_DIR SOURCE_NAME SOURCE_LINES [AGENT_TYPE]
  TRANSCRIPTS_DIR  a Claude Code project directory, or a session directory in it
  SOURCE_NAME      the file name to look for, e.g. paper.tex
  SOURCE_LINES     its length in lines, for Read calls with no limit
  AGENT_TYPE       only subagents of this type (e.g. exegenius-planner)
"""
import json
import pathlib
import sys


def calls(path):
    for line in open(path):
        try:
            event = json.loads(line)
        except ValueError:
            continue
        message = event.get("message") or {}
        content = message.get("content")
        if event.get("type") != "assistant" or not isinstance(content, list):
            continue
        for block in content:
            if block.get("type") == "tool_use":
                yield block["name"], block.get("input", {})


def main():
    root, name, total = pathlib.Path(sys.argv[1]), sys.argv[2], int(sys.argv[3])
    kind = sys.argv[4] if len(sys.argv) > 4 else None
    for transcript in sorted(root.rglob("agent-*.jsonl")):
        meta_path = transcript.with_suffix(".meta.json")
        meta = json.load(open(meta_path)) if meta_path.exists() else {}
        if kind and meta.get("agentType") != kind:
            continue
        print(f"== {transcript.name} {meta.get('agentType', '?')}: {meta.get('description', '')}")
        seen = set()
        for tool, args in calls(transcript):
            if tool == "Read" and args.get("file_path", "").endswith(name):
                offset = int(args.get("offset") or 1)
                limit = int(args.get("limit") or total)
                seen.update(range(offset, min(offset + limit, total + 1)))
                print(f"  Read {name} offset {offset} limit {args.get('limit')}  "
                      f"(lines read so far: {len(seen)} of {total})")
            elif tool == "Bash":
                command = args.get("command", "")
                if "exegenius submit" in command:
                    print(f"  submit  (lines read so far: {len(seen)} of {total})")
                elif name in command:
                    print(f"  Bash: {command[:120]}")


main()

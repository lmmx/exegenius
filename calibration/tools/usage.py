"""Token use per subagent from Claude Code transcripts (agent-*.jsonl with .meta.json).

Each assistant message id is one model call. Its input side (input, cache-read and
cache-write tokens) is taken from its usage record. Its output is estimated from the
characters of its content blocks (thinking, text, tool calls) at 4 characters per token:
transcripts record usage when a message starts streaming, so their output_tokens undercount.

usage: python3 -I usage.py TRANSCRIPT_DIR ...
"""
import collections, json, pathlib, statistics, sys
from datetime import datetime

rows = []
for root in sys.argv[1:]:
    for t in pathlib.Path(root).rglob("agent-*.jsonl"):
        meta = json.load(open(t.with_suffix(".meta.json")))
        calls, chars, model, stamps = {}, collections.Counter(), None, []
        for line in open(t):
            e = json.loads(line)
            if e.get("timestamp"):
                stamps.append(e["timestamp"])
            m = e.get("message") or {}
            if e.get("type") != "assistant":
                continue
            u = m.get("usage") or {}
            calls[m.get("id")] = (u.get("input_tokens", 0), u.get("cache_read_input_tokens", 0),
                                  u.get("cache_creation_input_tokens", 0))
            chars[m.get("id")] += sum(len(json.dumps(b)) for b in m.get("content", []))
            model = m.get("model", model)
        fresh = sum(c[0] for c in calls.values())
        read = sum(c[1] for c in calls.values())
        written = sum(c[2] for c in calls.values())
        out = sum(chars.values()) // 4
        span = lambda s: datetime.fromisoformat(s.replace("Z", "+00:00"))
        secs = (span(max(stamps)) - span(min(stamps))).total_seconds() if stamps else 0
        d = meta.get("description", "")
        kind = d.split()[-1].split("-", 1)[-1] if d else "?"
        rows.append((meta.get("agentType"), kind, model, len(calls), fresh, read, written, out, secs))

groups = collections.defaultdict(list)
for r in rows:
    groups[r[:3]].append(r[3:])
print("agent / task / model / runs: medians of model calls, fresh input, cache read, cache write, "
      "output (estimated), seconds")
for key, v in sorted(groups.items()):
    med = [int(statistics.median(x[i] for x in v)) for i in range(6)]
    print(" / ".join(map(str, key)), f"/ {len(v)}:", *med)

"""Measured token use per experiment, split into producing outputs (runs) and evaluating them
(judging), from the development session's subagent transcripts.

usage: python3 -I spend.py SUBAGENTS_DIR [SUBAGENTS_DIR ...]

Each subagent is assigned to an experiment and a role by its description, which every
protocol fixes ("RUN TASK" for runs, "Jn judge Pnn" for judges). Counts are measured from the
transcripts' usage records: model calls, input, cache-read and cache-write tokens. Output tokens
are estimated from content characters at 4 per token, since transcripts undercount output.
Subagents whose description matches no rule are listed as "other". Experiments run in separate
runner sessions (E1, E3, S1) are not in these transcripts; the registry holds their runs.
"""
import collections
import json
import pathlib
import re
import sys

RULES = [  # (pattern on the description, experiment, role)
    (r"^J(\d) judge P\d+", None, "judging"),
    (r"^(Judge plan|Judge P|Second judge of plan)", "J1-J2", "judging"),
    (r"^([BV])\d \d{4}-argument", "E4", "runs"),
    (r"^N\d \d{4}-", "E5", "runs"),
    (r"^F\d[BV]? \d{4}-", "E6", "runs"),
    (r"^F\dW \d{4}-", "E7", "runs"),
    (r"^F\d(SX|OX|SM|OM) \d{4}-", "S2", "runs"),
    (r"^W1 \d{4}-", "W1 (cancelled)", "runs"),
    (r"^R\d \d{4}-", "T1", "runs"),
    (r"^(T\d chain walk|Chain walk)", "chain walks", "judging"),
]
BATCH = {"3": "E3", "4": "S1", "5": "E4", "6": "E5", "7": "E6", "8": "E7", "9": "S2"}


def classify(desc):
    for pattern, experiment, role in RULES:
        m = re.match(pattern, desc)
        if m:
            if experiment is None:
                return f"{BATCH.get(m.group(1), 'J' + m.group(1))} (J{m.group(1)})", role
            return experiment, role
    return "other", "other"


def main():
    rows = collections.defaultdict(lambda: collections.Counter())
    models = collections.defaultdict(set)
    for root in sys.argv[1:]:
        for meta_path in pathlib.Path(root).glob("*.meta.json"):
            t = meta_path.with_name(meta_path.name[: -len(".meta.json")] + ".jsonl")
            if not t.exists():
                continue
            key = classify(json.load(open(meta_path)).get("description", ""))
            calls = {}
            for line in open(t):
                e = json.loads(line)
                if e.get("type") != "assistant":
                    continue
                m = e.get("message") or {}
                u = m.get("usage") or {}
                calls[m.get("id")] = (u.get("input_tokens", 0), u.get("cache_read_input_tokens", 0),
                                      u.get("cache_creation_input_tokens", 0),
                                      sum(len(json.dumps(b)) for b in m.get("content", [])) // 4)
                if m.get("model") and m["model"] != "<synthetic>":
                    models[key].add(m["model"].replace("claude-", ""))
            r = rows[key]
            r["subagents"] += 1
            r["calls"] += len(calls)
            for c in calls.values():
                r["input"] += c[0]
                r["cache_read"] += c[1]
                r["cache_write"] += c[2]
                r["output_est"] += c[3]
    print("| Experiment | Role | Subagents | Models | Cache read (M) | Cache write (M) | Output, est. (k) |")
    print("|---|---|---|---|---|---|---|")
    for (exp, role), r in sorted(rows.items()):
        print(f"| {exp} | {role} | {r['subagents']} | {', '.join(sorted(models[(exp, role)]))} | "
              f"{r['cache_read'] / 1e6:.1f} | {r['cache_write'] / 1e6:.2f} | {r['output_est'] // 1000} |")


if __name__ == "__main__":
    main()

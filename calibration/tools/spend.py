"""Measured token use per experiment, split into producing outputs (runs) and evaluating them
(judging), from the development session's subagent transcripts.

usage: python3 -I spend.py SUBAGENTS_DIR [SUBAGENTS_DIR ...] [--since 2026-10-14T14:20]

With --since, only model calls at or after that UTC time are counted. When SUBAGENTS_DIR is a
session's `subagents` directory, the session's own transcript (SESSION.jsonl beside SESSION/) is
counted too, as "development session": its long context is re-read on every call, so orchestration
is not free.

Each subagent is assigned to an experiment and a role by its description, which every
protocol fixes ("RUN TASK" for runs, "Jn judge Pnn" for judges). Counts are measured from the
transcripts' usage records: model calls, input, cache-read and cache-write tokens. Visible output
tokens (text and tool calls) are estimated from content characters at 4 per token. Thinking is
not stored in transcripts, only as an encrypted signature: its length is reported as a proxy for
hidden reasoning, not as a token count.
Subagents whose description matches no rule are listed as "other".

The four components are reported separately and never summed: they are priced differently (a
cache read costs a small fraction of uncached input, and output costs most), prices differ by model,
and how a subscription's usage meter weighs them is not published. No conversion to cost or to
quota is made here; a meter reading supplied by the author can be set beside these counts.
Experiments run in separate runner sessions (E1, E3, S1) are not in these transcripts; the
registry holds their runs.
"""
import collections
import json
import pathlib
import re
import sys

RULES = [  # (pattern on the description, experiment, role)
    (r"^J(\d+) judge [PQ]\d+", None, "judging"),
    (r"^(Judge plan|Judge P|Second judge of plan)", "J1-J2", "judging"),
    (r"^([BV])\d \d{4}-argument", "E4", "runs"),
    (r"^N\d \d{4}-", "E5", "runs"),
    (r"^F\d[BV]? \d{4}-", "E6", "runs"),
    (r"^F\dW \d{4}-", "E7", "runs"),
    (r"^F\d(SX|OX|SM|OM) \d{4}-", "S2", "runs"),
    (r"^S3F\d(OH|OX) \d{4}-", "S3", "runs"),
    (r"^S4F\d(OM) \d{4}-", "S4", "runs"),
    (r"^S5F\d(OX) \d{4}-", "S5", "runs"),
    (r"^E8F\dX \d{4}-", "E8", "runs"),
    (r"^E8bF\dX2? \d{4}-", "E8b", "runs"),
    (r"^E9F\dY \d{4}-", "E9", "runs"),
    (r"^E10F\dY2? \d{4}-", "E10", "runs"),
    (r"^S6F\d(SM|HH) \d{4}-", "S6", "runs"),
    (r"^W2W[HMSO]P(15R|16) \d{4}-", "W2b", "runs"),
    (r"^W2W[HMSO]P\d+ \d{4}-", "W2", "runs"),
    (r"^E11F\dO \d{4}-", "E11", "runs"),
    (r"^K3 judge", "K3 (pairwise)", "judging"),
    (r"^KS ", "key-step designation", "design"),
    (r"^(A1|X1|K3) ", "design subagents", "design"),
    (r"^FB1F\d \d{4}-", "FB1", "runs"),
    (r"^W1 \d{4}-", "W1 (cancelled)", "runs"),
    (r"^R\d \d{4}-", "T1", "runs"),
    (r"^(T\d chain walk|Chain walk)", "chain walks", "judging"),
]
BATCH = {"3": "E3", "4": "S1", "5": "E4", "6": "E5", "7": "E6", "8": "E7", "9": "S2", "10": "S3", "11": "K1", "12": "S4", "13": "S4 checklist", "14": "K2", "15": "E8 checklist", "16": "E8", "17": "E8b checklist", "18": "E8b", "19": "FB1 checklist", "20": "E9 checklist", "21": "E10", "22": "W2 pages", "23": "S6 checklist", "24": "W2b pages", "25": "E11 (pairwise)"}


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
    args = sys.argv[1:]
    since = None
    if "--since" in args:
        i = args.index("--since")
        since = args[i + 1]
        args = args[:i] + args[i + 2:]
    for root in args:
        root = pathlib.Path(root)
        found = [(t, classify(json.load(open(t.with_name(t.name[: -len(".jsonl")] + ".meta.json"))).get("description", "")))
                 for t in root.glob("*.jsonl") if t.with_name(t.name[: -len(".jsonl")] + ".meta.json").exists()]
        session = root.parent.parent / (root.parent.name + ".jsonl")
        if root.name == "subagents" and session.exists():
            found.append((session, ("development session", "orchestration")))
        for t, key in found:
            calls, visible, signature = {}, collections.Counter(), collections.Counter()
            for line in open(t):
                e = json.loads(line)
                if e.get("type") != "assistant" or (since and e.get("timestamp", "") < since):
                    continue
                m = e.get("message") or {}
                u = m.get("usage") or {}
                # one call can span several lines, one per content block: usage is per call,
                # content is summed over lines; thinking signatures are not output text
                calls[m.get("id")] = (u.get("input_tokens", 0), u.get("cache_read_input_tokens", 0),
                                      u.get("cache_creation_input_tokens", 0))
                blocks = m.get("content", [])
                visible[m.get("id")] += sum(len(json.dumps({k: v for k, v in b.items() if k != "signature"}))
                                            for b in blocks)
                signature[m.get("id")] += sum(len(b.get("signature", "")) for b in blocks)
                if m.get("model") and m["model"] != "<synthetic>":
                    models[key].add(m["model"].replace("claude-", ""))
            if not calls:
                continue
            r = rows[key]
            r["subagents"] += 1
            r["calls"] += len(calls)
            for c in calls.values():
                r["input"] += c[0]
                r["cache_read"] += c[1]
                r["cache_write"] += c[2]
            r["output_est"] += sum(visible.values()) // 4
            r["signature"] += sum(signature.values())
    print("| Experiment | Role | Subagents | Models | Calls | Uncached input (k) | Cache read (M) | Cache write (M) | Visible output, est. (k) | Thinking signatures (k chars) |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for (exp, role), r in sorted(rows.items()):
        print(f"| {exp} | {role} | {r['subagents']} | {', '.join(sorted(models[(exp, role)]))} | {r['calls']} | "
              f"{r['input'] // 1000} | {r['cache_read'] / 1e6:.1f} | {r['cache_write'] / 1e6:.2f} | {r['output_est'] // 1000} | {r['signature'] // 1000} |")


if __name__ == "__main__":
    main()

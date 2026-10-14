"""Build Evaluation Registry records from experiment specs and the evidence they point to.

usage:
  python3 -I registry.py build SPEC.json [SPEC.json ...] --wiki WIKI_REPO --out DIR
  python3 -I registry.py index RECORD.json [RECORD.json ...] > INDEX.md

`build` reads each hand-written spec (registry/specs/<ID>.json: question, arms and their
intended configurations, evidence refs, judge batches, reported facts) and extracts, from git
refs of the wiki repository and from files in this repository, everything that can be
measured:
- per run, from `runs/RUN/exegenius/log.jsonl` at the evidence commit: each task's issue time,
  its submits with outcome, check and coverage figures, and the planning wall time (first
  issue to the last planning task accepted);
- per subagent, from the transcripts under `results/transcripts/` at that commit: agent type,
  model(s), effort, model calls, input / cache-read / cache-write tokens (measured), output
  tokens (estimated at 4 characters per token, since transcripts undercount output) and
  seconds. Subagents are matched to runs by the description prefix "RUN TASK" that every
  runner protocol requires;
- per run and judge batch, from the batch's verdicts and key: the content judge's counts and,
  where present, the central-derivation rubric's count and gate.

Every value carries a basis: "measured" (extracted here), "estimated" (output tokens),
"reported" (from the spec, with its source) or "unavailable". Nothing is inferred: a run with no
transcript has no model or effort recorded, whatever its arm intended.

`index` writes a human-readable index of records as markdown.
"""
import argparse
import collections
import io
import json
import pathlib
import statistics
import subprocess
import sys
import tarfile
from datetime import datetime, timezone

ROOT = pathlib.Path(__file__).resolve().parents[2]
PLANNING = ("extract", "background", "argument", "extend")
ORDER = {"absent": 0, "mentioned": 1, "stated": 2, "derived": 3}
CENTRAL_GATE = ("C4", "C5", "C6", "C9")


def git(wiki, *args, binary=False):
    out = subprocess.run(["git", "-C", str(wiki), *args], capture_output=True, check=False)
    if out.returncode != 0:
        return None
    return out.stdout if binary else out.stdout.decode()


def measured(value):
    return {"value": value, "basis": "measured"}


def unavailable(why):
    return {"value": None, "basis": "unavailable", "note": why}


def iso(ms):
    return datetime.fromtimestamp(ms / 1000, tz=timezone.utc).isoformat().replace("+00:00", "Z")


def run_log(wiki, commit, run, kinds=PLANNING, since_ms=None):
    text = git(wiki, "show", f"{commit}:runs/{run}/exegenius/log.jsonl")
    if text is None:
        return None
    events = [json.loads(line) for line in text.splitlines() if line.strip()]
    if since_ms is not None:  # a run copied from another: drop the earlier run's events
        events = [e for e in events if e["at"] >= since_ms]
    tasks = collections.OrderedDict()
    for e in events:
        t = tasks.setdefault(e["task"], {"task": e["task"], "kind": e.get("kind"), "submits": []})
        if e["event"] == "issued":
            t.setdefault("issued", iso(e["at"]))
            t["_issued_ms"] = min(t.get("_issued_ms", e["at"]), e["at"])
        elif e["event"] == "submit":
            s = {"attempt": e.get("attempt"), "at": iso(e["at"]), "outcome": e.get("outcome")}
            if e.get("check"):
                s["check"] = e["check"]
            for c in e.get("coverage") or []:
                s.setdefault("coverage", []).append({k: c.get(k) for k in (
                    "source", "lines", "quotes", "located", "first_tenth", "span")}
                    | {"sections_quoted": sum(1 for x in c.get("sections", []) if x.get("quotes")),
                       "sections": len(c.get("sections", []))})
            t["submits"].append(s)
            if e.get("outcome") == "accepted":
                t["_accepted_ms"] = e["at"]
    planning = [t for t in tasks.values() if t["kind"] in kinds]
    start = min((t["_issued_ms"] for t in planning if "_issued_ms" in t), default=None)
    end = max((t["_accepted_ms"] for t in planning if "_accepted_ms" in t), default=None)
    rejected = sum(1 for t in planning for s in t["submits"] if s["outcome"] == "rejected")
    for t in tasks.values():
        t.pop("_issued_ms", None)
        t.pop("_accepted_ms", None)
    return {
        "planning_tasks": planning,
        "other_tasks_issued": sum(1 for t in tasks.values() if t["kind"] not in PLANNING),
        "planning_seconds": measured(round((end - start) / 1000, 1)) if start and end else
        unavailable("no planning task accepted"),
        "rejected_submits": measured(rejected),
        "coverage_rejections": measured(sum(1 for t in planning for s in t["submits"]
                                            if s["outcome"] == "rejected" and s.get("check") == "coverage")),
    }


def transcripts(wiki, commit, path="results/transcripts"):
    """Subagent and main transcripts under `path` (default results/transcripts) at the commit, keyed by path."""
    blob = git(wiki, "archive", commit, path, binary=True)
    if blob is None:
        return {}
    files = {}
    with tarfile.open(fileobj=io.BytesIO(blob)) as tar:
        for m in tar.getmembers():
            if m.isfile() and (m.name.endswith(".jsonl") or m.name.endswith(".meta.json")):
                files[m.name] = tar.extractfile(m).read().decode()
    return files


def subagent(lines, meta):
    calls, chars, models, efforts, stamps = {}, collections.Counter(), collections.Counter(), collections.Counter(), []
    for line in lines.splitlines():
        try:
            e = json.loads(line)
        except json.JSONDecodeError:
            continue
        if e.get("timestamp"):
            stamps.append(e["timestamp"])
        if "effort" in e:
            efforts[str(e["effort"])] += 1
        m = e.get("message") or {}
        if e.get("type") != "assistant" or not isinstance(m, dict):
            continue
        u = m.get("usage") or {}
        calls[m.get("id")] = (u.get("input_tokens", 0), u.get("cache_read_input_tokens", 0),
                              u.get("cache_creation_input_tokens", 0))
        chars[m.get("id")] += sum(len(json.dumps(b)) for b in m.get("content", []))
        if m.get("model") and m["model"] != "<synthetic>":  # an error message, not a model
            models[m["model"]] += 1
    t = lambda s: datetime.fromisoformat(s.replace("Z", "+00:00"))
    return {
        "agent_type": meta.get("agentType"),
        "models": dict(models),
        "effort": dict(efforts) if efforts else None,
        "model_calls": measured(len(calls)),
        "input_tokens": measured(sum(c[0] for c in calls.values())),
        "cache_read_tokens": measured(sum(c[1] for c in calls.values())),
        "cache_write_tokens": measured(sum(c[2] for c in calls.values())),
        "output_tokens": {"value": sum(chars.values()) // 4, "basis": "estimated",
                          "note": "content characters / 4; transcripts undercount output"},
        "seconds": measured(round((t(max(stamps)) - t(min(stamps))).total_seconds(), 1)) if stamps else
        unavailable("no timestamps"),
    }


def subagents_by_run(files, prefix=""):
    out, orchestrators = collections.defaultdict(list), []
    for name, text in files.items():
        if name.endswith(".jsonl") and "/subagents/" in name:
            meta = json.loads(files.get(name[: -len(".jsonl")] + ".meta.json", "{}"))
            desc = meta.get("description", "")
            parts = desc.split()
            if len(parts) >= 2:
                s = subagent(text, meta)
                s["task"] = parts[1]
                s["description"] = desc
                out[prefix + parts[0]].append(s)
        elif name.endswith(".jsonl") and "/subagents/" not in name:
            s = subagent(text, {})
            if s["models"]:
                orchestrators.append({"models": s["models"], "effort": s["effort"]})
    return out, orchestrators


def verdict_summary(verdict, tiers):
    s = collections.Counter()
    for item in verdict["items"]:
        tier = tiers.get(item["id"])
        present = ORDER.get(item.get("presence"), 0) >= 2 and item.get("statement") != "wrong"
        if tier == "core":
            s["core_stated"] += present
            s["core_fully"] += present and item.get("statement") == "correct"
            s["core_derived"] += item.get("presence") == "derived"
            s["core_faithful"] += item.get("derivation") == "faithful"
        elif tier == "supporting":
            s["supporting_stated"] += present
        s["wrong"] += item.get("statement") == "wrong" or item.get("derivation") == "wrong"
    for e in verdict.get("errors", []):
        s["major_errors" if e.get("severity") == "major" else "minor_errors"] += 1
    out = {k: s[k] for k in ("core_stated", "core_fully", "core_derived", "core_faithful",
                              "supporting_stated", "wrong", "major_errors", "minor_errors")}
    central = verdict.get("central")
    if central:
        v = {st["id"]: st.get("verdict") for st in central.get("steps", [])}
        out["central_count"] = sum(1 for x in v.values() if x == "reconstructed")
        out["central_gate"] = all(v.get(i) == "reconstructed" for i in CENTRAL_GATE) and \
            not any(x == "wrong" for x in v.values())
    return out


def judgements(batches):
    """{run name: [{batch, judge: a|b, ...summary}]} over every judge batch."""
    found = collections.defaultdict(list)
    for b in batches:
        d = ROOT / b["dir"]
        key = json.load(open(d / b.get("key", "key.json")))
        tiers = {i["id"]: i["tier"] for i in json.load(open(ROOT / b["reference"]))["items"]}
        for pid, run in key.items():
            for suffix, which in (("", "a"), ("-b", "b")):
                p = d / b.get("verdicts", "verdicts") / f"{pid}{suffix}.json"
                if p.exists():
                    found[run].append({"batch": b["id"], "judge": which, "plan": pid,
                                       **verdict_summary(json.load(open(p)), tiers)})
    return found


def summarise(values):
    v = [x for x in values if x is not None]
    if not v:
        return None
    return {"n": len(v), "median": statistics.median(v), "min": min(v), "max": max(v)}


def build(spec, wiki, batches):
    ev = spec.get("evidence", {})
    commit = ev.get("commit")
    wiki_ev = commit and ev.get("repo") == "lmmx/openai-nlogn-wiki"
    files = transcripts(wiki, commit, ev.get("transcripts", "results/transcripts")) if wiki_ev else {}
    by_run, orchestrators = subagents_by_run(files, ev.get("description_prefix", ""))
    kinds = tuple(ev.get("planning_kinds", PLANNING))
    since = ev.get("log_since")
    since_ms = int(datetime.fromisoformat(since.replace("Z", "+00:00")).timestamp() * 1000) if since else None
    judged = judgements(batches)
    record = {k: spec[k] for k in spec if k != "runs_extra"}
    record["built"] = {"by": "calibration/tools/registry.py", "wiki_commit": commit}
    record["orchestrator_observed"] = orchestrators or unavailable("no main transcript at the evidence commit")
    runs, excluded = [], set(spec.get("excluded", {}))
    for arm_id, arm in spec["arms"].items():
        for run in arm["runs"]:
            r = {"id": run, "arm": arm_id}
            log = run_log(wiki, commit, run, kinds, since_ms) if wiki_ev else None
            r.update(log or {"planning_seconds": unavailable("no run log at the evidence ref")})
            subs = sorted(by_run.get(run, []), key=lambda s: s["task"])
            r["subagents"] = subs
            if subs:
                types = sorted({s["agent_type"] for s in subs})
                models = sorted({m for s in subs for m in s["models"]})
                efforts = sorted({e for s in subs for e in (s["effort"] or {})})
                r["observed_config"] = {"agent_types": types, "models": models, "efforts": efforts or None}
                want = arm.get("intended", {})
                r["matches_arm"] = (
                    (not want.get("agent") or types == [want["agent"]]) and
                    (not want.get("model_id") or models == [want["model_id"]]) and
                    (not want.get("effort") or efforts == [want["effort"]]))
            else:
                r["observed_config"] = spec.get("runs_extra", {}).get(run, {}).get(
                    "observed_config", unavailable("no transcripts for this run in the evidence"))
                r["matches_arm"] = None
            r["quality"] = judged.get(run, [])
            if run in excluded:
                r["excluded"] = spec["excluded"][run]
            runs.append(r)
    record["runs"] = runs
    summary = {}
    for arm_id in spec["arms"]:
        rs = [r for r in runs if r["arm"] == arm_id and "excluded" not in r]
        arm_sum = {"runs": len(rs)}
        for b in sorted({q["batch"] for r in rs for q in r["quality"]}):
            firsts = [next((q for q in r["quality"] if q["batch"] == b and q["judge"] == "a"), None) for r in rs]
            firsts = [q for q in firsts if q]
            arm_sum[f"quality_{b}"] = {k: summarise([q.get(k) for q in firsts]) for k in (
                "core_stated", "core_fully", "core_derived", "core_faithful", "major_errors", "central_count")}
            gates = [q["central_gate"] for q in firsts if "central_gate" in q]
            if gates:
                arm_sum[f"quality_{b}"]["central_gate_passed"] = f"{sum(gates)}/{len(gates)}"
        for task in PLANNING:
            subs = [s for r in rs for s in r["subagents"] if s["task"].endswith(task)]
            if subs:
                arm_sum[f"resources_{task}"] = {k: summarise([s[k]["value"] for s in subs]) for k in (
                    "model_calls", "cache_read_tokens", "cache_write_tokens", "output_tokens", "seconds")}
        arm_sum["planning_seconds"] = summarise([r["planning_seconds"]["value"] for r in rs
                                                 if isinstance(r.get("planning_seconds"), dict)])
        summary[arm_id] = arm_sum
    record["arm_summary"] = summary
    return record


def fmt(x):
    return f"{x:,.0f}" if isinstance(x, (int, float)) and abs(x) >= 1000 else f"{x:g}"


def index(records):
    out = ["# Evaluation Registry index", "",
           "Generated by `calibration/tools/registry.py index`; do not edit by hand.", "",
           "| Id | Kind | Document | Task | Arms (intended) | Runs | Evidence | Journal |",
           "|---|---|---|---|---|---|---|---|"]
    for r in records:
        arms = "; ".join(f"{a}: {v['intended'].get('agent', '?')} / {v['intended'].get('model_id', v['intended'].get('model', '?'))}"
                         f" / {v['intended'].get('effort', '?')}" for a, v in r["arms"].items())
        ev = r.get("evidence", {})
        out.append(f"| {r['id']} | {r.get('kind', '')} | {r.get('document', {}).get('name', '')} | {r.get('task', '')} | "
                   f"{arms} | {len(r.get('runs', []))} | {ev.get('repo', '')} `{ev.get('branch', ev.get('path', ''))}` "
                   f"{ev.get('commit', '')} | {r.get('journal', '')} |")
    out += ["", "## Arm summaries (medians and ranges of first judgements; resources per planning subagent)", ""]
    for r in records:
        out.append(f"### {r['id']}: {r.get('title', '')}")
        out.append("")
        for a, s in r.get("arm_summary", {}).items():
            out.append(f"- **{a}** ({s['runs']} runs)")
            for k, v in s.items():
                if k.startswith("quality_"):
                    parts = [f"{m} {fmt(x['median'])} ({fmt(x['min'])}-{fmt(x['max'])})" for m, x in v.items()
                             if isinstance(x, dict) and x]
                    n = max((x["n"] for x in v.values() if isinstance(x, dict) and x), default=0)
                    if n < s["runs"]:
                        parts.insert(0, f"n={n} of {s['runs']}")
                    if "central_gate_passed" in v:
                        parts.append(f"central gate {v['central_gate_passed']}")
                    out.append(f"  - {k[8:]}: " + "; ".join(parts))
                elif k.startswith("resources_"):
                    parts = [f"{m} {fmt(x['median'])}" for m, x in v.items() if x]
                    out.append(f"  - {k[10:]}: " + "; ".join(parts))
            ps = s.get("planning_seconds")
            if ps:
                out.append(f"  - planning seconds: {fmt(ps['median'])} ({fmt(ps['min'])}-{fmt(ps['max'])})")
        if r.get("limitations"):
            out.append("")
            out.append("Limitations: " + " ".join(r["limitations"]))
        out.append("")
    return "\n".join(out) + "\n"


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    b = sub.add_parser("build")
    b.add_argument("specs", nargs="+")
    b.add_argument("--wiki", required=True)
    b.add_argument("--batches", default=str(ROOT / "calibration/registry/judging.json"))
    b.add_argument("--out", required=True)
    i = sub.add_parser("index")
    i.add_argument("records", nargs="+")
    a = ap.parse_args()
    if a.cmd == "build":
        batches = json.load(open(a.batches))
        for path in a.specs:
            spec = json.load(open(path))
            rec = build(spec, a.wiki, batches)
            pathlib.Path(a.out, f"{spec['id']}.json").write_text(json.dumps(rec, indent=1) + "\n")
            print(f"{spec['id']}: {len(rec['runs'])} runs", file=sys.stderr)
    else:
        sys.stdout.write(index([json.load(open(p)) for p in a.records]))


main()

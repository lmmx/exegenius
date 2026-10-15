"""Collect A1's accepted answers into plant.py's layout, and check each task's isolation from its transcript.

usage: python3 -I a1collect.py RUNS_DIR ANSWERS_DIR [--subagents DIR]

RUNS_DIR is a1setup.py's output: copies under RUNS_DIR/A1<cfg><run>/<code>/<kind>/, and prompts.tsv.
For each task in prompts.tsv, it copies into ANSWERS_DIR/<cfg>/<run>/<code>/ (plans/A1.md, "Collection"):
- `<kind>.answer.json`, only if `exegenius submit` accepted the task (from the copy's log);
- `<kind>.log.jsonl`, the copy's run log;
- `support.task.md` for support tasks.

With --subagents (the session's subagents directory), it also reads each task's transcript, matched by
its description, and lists every file the subagent read or touched besides its own task file and
answer. A task that read anything else is contaminated: excluded and rerun once, in a fresh copy.
Prints, per configuration: tasks, accepted, never accepted, no transcript and contaminated, and
writes ANSWERS_DIR/collect.json.
"""
import json
import pathlib
import re
import shutil
import sys


def accepted(log, tid):
    return any(e.get("event") == "submit" and e.get("task") == tid and e.get("outcome") == "accepted" for e in log)


def transcripts(sub):
    out = {}
    for meta in pathlib.Path(sub).glob("*.meta.json"):
        d = json.loads(meta.read_text()).get("description", "")
        if d.startswith("A1"):
            out[d] = meta.with_suffix("").with_suffix(".jsonl")
    return out


def touched(path):
    """Paths named in the transcript's Read/Write/Edit calls and in its shell commands."""
    seen = set()
    for line in open(path):
        m = json.loads(line).get("message") or {}
        for c in m.get("content", []) if isinstance(m.get("content"), list) else []:
            if c.get("type") != "tool_use":
                continue
            i = c.get("input", {})
            if i.get("file_path"):
                seen.add(i["file_path"])
            if i.get("command"):
                seen.update(re.findall(r"/[\w./-]+", i["command"]))
    return seen


def main():
    runs, out = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
    sub = sys.argv[sys.argv.index("--subagents") + 1] if "--subagents" in sys.argv else None
    tx = transcripts(sub) if sub else {}
    summary, rows = {}, []
    for line in (runs / "prompts.tsv").read_text().splitlines():
        desc, model, effort, root, prompt = line.split("\t")
        root = pathlib.Path(root)
        cfg_run, tid = desc.split(" ")
        cfg, run = cfg_run[2:4], cfg_run[4:]
        code, kind = root.parent.name, root.name
        log_path = root / "exegenius/log.jsonl"
        log = [json.loads(l) for l in log_path.read_text().splitlines() if l.strip()] if log_path.exists() else []
        d = out / cfg / run / code
        d.mkdir(parents=True, exist_ok=True)
        ok = accepted(log, tid)
        if ok:
            shutil.copy(root / "exegenius/tasks" / f"{tid}.answer.json", d / f"{kind}.answer.json")
        if log_path.exists():
            shutil.copy(log_path, d / f"{kind}.log.jsonl")
        if kind == "support":
            shutil.copy(root / "exegenius/tasks" / f"{tid}.md", d / "support.task.md")
        row = {"task": desc, "accepted": ok,
               "submits": sum(1 for e in log if e.get("event") == "submit" and e.get("task") == tid)}
        if sub:
            t = tx.get(desc)
            if t is None:
                row["transcript"] = None
            else:
                allowed = {str(root / "exegenius/tasks" / f"{tid}.md"), str(root / "exegenius/tasks" / f"{tid}.answer.json")}
                extra = sorted(p for p in touched(t) if p not in allowed and not p.startswith(str(root / "exegenius/tasks" / tid))
                               and p != str(root) and not p.startswith("/usr/") and not p.startswith("/dev/"))
                row["contaminated"] = bool(extra)
                row["other_paths"] = extra
        rows.append(row)
        s = summary.setdefault(cfg, {"tasks": 0, "accepted": 0, "never_accepted": 0, "no_transcript": 0, "contaminated": 0})
        s["tasks"] += 1; s["accepted"] += ok; s["never_accepted"] += not ok
        s["no_transcript"] += sub is not None and row.get("transcript", 1) is None
        s["contaminated"] += bool(row.get("contaminated"))
    (out / "collect.json").write_text(json.dumps({"summary": summary, "tasks": rows}, indent=1))
    for cfg, s in summary.items():
        print(cfg, s)


if __name__ == "__main__":
    main()

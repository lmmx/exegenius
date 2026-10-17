"""Collect A1's accepted answers into plant.py's layout, and check each task's isolation from its transcript.

usage: python3 -I a1collect.py RUNS_DIR ANSWERS_DIR [--subagents DIR] [--allow REL ...]

RUNS_DIR is a1setup.py's output: copies under RUNS_DIR/A1<cfg><run>/<code>/<kind>/, and prompts.tsv.
For each task in prompts.tsv, it copies into ANSWERS_DIR/<cfg>/<run>/<code>/ (plans/A1.md, "Collection"):
- `<kind>.answer.json`, only if `exegenius submit` accepted the task (from the copy's log);
- `<kind>.log.jsonl`, the copy's run log;
- `support.task.md` for support tasks.

With --subagents (the session's subagents directory), it also reads each task's transcript, matched by
the copy's root path in its prompt, and lists every file the subagent read or touched besides its own task file and
answer. A task that read anything else is contaminated: excluded and rerun once, in a fresh copy.
Prints, per configuration: tasks, accepted, never accepted, no transcript and contaminated, and
writes ANSWERS_DIR/collect.json.

It also collects A2 (plans/A2.md) and A3 (plans/A3.md, arm R): descriptions `A2<arm><run> <task id>`. `--allow REL` adds a file
that reading does not count as contamination, relative to each copy's root (A2's SI arm reads
`sources/paper.tex`, which its task allows).
"""
import json
import pathlib
import re
import shutil
import sys


def accepted(log, tid):
    return any(e.get("event") == "submit" and e.get("task") == tid and e.get("outcome") == "accepted" for e in log)


def transcripts(sub):
    """{copy root: transcript}. The four variants of a page share a description, so transcripts are
    matched by the copy's root path, which the prompt (the transcript's first user message) contains."""
    out = {}
    for meta in pathlib.Path(sub).glob("*.meta.json"):
        if not json.loads(meta.read_text()).get("description", "").startswith(("A1A", "A2", "A3")):
            continue
        t = meta.with_suffix("").with_suffix(".jsonl")
        first = json.loads(open(t).readline()).get("message", {}).get("content", "")
        m = re.search(r"(/\S*/A(?:1A[HSO]|2(?:B|I|SI)|3R)\d+/a[12]-[0-9a-f]+/(?:audit|support))/exegenius/tasks/", first if isinstance(first, str) else json.dumps(first))
        if m:
            out[m.group(1)] = t
    return out


def touched(path):
    """(paths, other_tools): absolute filesystem paths named in the transcript's Read/Write/Edit/Glob/Grep
    calls and shell commands, and the names of any other tools called. Only tokens that start a word
    and begin with /tmp/, /home/ or /root/ count as paths, so slashes inside formulas or relative paths
    are not mistaken for reads."""
    seen, other = set(), set()
    for line in open(path):
        m = json.loads(line).get("message") or {}
        for c in m.get("content", []) if isinstance(m.get("content"), list) else []:
            if c.get("type") != "tool_use":
                continue
            i, name = c.get("input", {}), c.get("name", "")
            if name not in ("Read", "Write", "Edit", "Bash", "Glob", "Grep", "SubagentHandback"):
                other.add(name)
            for key in ("file_path", "path"):
                if isinstance(i.get(key), str):
                    seen.add(i[key])
            if i.get("command"):
                seen.update(re.findall(r"(?:^|(?<=[\s'\"=(]))(/(?:tmp|home|root)/[\w./-]+)", i["command"]))
    return {p.rstrip("/.") for p in seen if p.startswith(("/tmp/", "/home/", "/root/"))}, sorted(other)


def main():
    runs, out = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
    sub = sys.argv[sys.argv.index("--subagents") + 1] if "--subagents" in sys.argv else None
    allow = [sys.argv[i + 1] for i, a in enumerate(sys.argv) if a == "--allow"]
    tx = transcripts(sub) if sub else {}
    summary, rows = {}, []
    for line in (runs / "prompts.tsv").read_text().splitlines():
        desc, model, effort, root, prompt = line.split("\t")
        root = pathlib.Path(root)
        cfg_run, tid = desc.split(" ")
        cfg, run = re.match(r"^A[123]([A-Z]+?)(\d+)$", cfg_run).groups()
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
            t = tx.get(str(root))
            if t is None:
                row["transcript"] = None
            else:
                paths, other_tools = touched(t)
                own = str(root / "exegenius/tasks")
                allowed = {str(root), str(root / "exegenius")} | {str(root / a) for a in allow}
                extra = sorted(p for p in paths if p not in allowed and not p.startswith(own))
                row["contaminated"] = bool(extra)
                row["other_paths"] = extra
                row["other_tools"] = other_tools
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

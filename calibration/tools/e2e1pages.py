"""E2E1's page judging (P4, P5, S1 and R3; plans/E2E1.md), on J24's page-judge prompt.

usage:
  python3 -I e2e1pages.py prep PROJECT JUDGE_DIR PROMPT_DIR SEED [--pages DIR] [--only ID,ID]
  python3 -I e2e1pages.py read JUDGE_DIR [--json OUT.json]

`prep` judges every page a write task wrote. Each page's text is PROJECT/exegenius/pages/<id>.md
(or DIR/<id>.md with --pages, for the snapshot taken when the audit rounds ended). Its assignment
comes from its latest write task file (the highest-numbered `NNNN-write-<id>.md`), as in J22, J24
and J26:
- a page with steps to derive gets the sections "This page in the argument" and "Steps this page
  derives", and is scored on every step id named in the latter, above "Steps derived on other
  pages" (W2b's and W3's rule);
- a page with none gets "This page" and "Where the argument relies on this page" (or "This page in
  the argument"), headed by the line W3 used for controls, and is scored on errors only.

The pages are copied to JUDGE_DIR/pages/Pnn.md in an order shuffled by SEED, with the key in
JUDGE_DIR/key.json and three prompts per page (Pnn, Pnn-b, Pnn-c) in PROMPT_DIR. The prompt is
J24's, unchanged apart from its paths.

`read` scores the verdicts per page: faithful and derived over the scored ids, and major and
minor errors, per judge and by median. A page meets the page standard when its median major
errors are 0 and, if it has n scored ids, its median faithful count is at least ceil(0.9 n).
"""
import argparse
import json
import math
import pathlib
import random
import re
import statistics
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
J24 = ROOT / "docs/journal/2026-10-14-w2b-writer/judge"
CONTROL_HEAD = ("**This page derives no steps of the argument: there are no items to judge. Give an empty "
                "`items` list, and judge the page's mathematical errors only.**\n\n")


def section(text, title):
    m = re.search(rf"^## {re.escape(title)}\n(.*?)(?=^## |\Z)", text, re.S | re.M)
    return m.group(0).rstrip() + "\n" if m else None


def scored_ids(steps_section):
    own = steps_section.split("### Steps derived on other pages")[0]
    ids = re.findall(r"^(?:\*\*(S[\d.]+)\*\*:|- (S[\d.]+):)", own, re.M)
    out = []
    for a, b in ids:
        i = a or b
        if i not in out:
            out.append(i)
    return out


def latest_write(tasks, page):
    found = sorted(tasks.glob(f"[0-9][0-9][0-9][0-9]-write-{page}.md"))
    assert found, f"no write task for {page}"
    return found[-1]


def assignment(task_text):
    steps = section(task_text, "Steps this page derives")
    if steps:
        arg = section(task_text, "This page in the argument")
        assert arg, "key-step page without 'This page in the argument'"
        ids = scored_ids(steps)
        assert ids, "no step ids in 'Steps this page derives'"
        return arg + steps, ids
    role = section(task_text, "Where the argument relies on this page") or section(task_text, "This page in the argument")
    this = section(task_text, "This page")
    assert this and role, "sections missing"
    return CONTROL_HEAD + this + role, []


def prep(project, jdir, pdir, seed, pages_dir=None, only=None):
    project, jdir, pdir = pathlib.Path(project), pathlib.Path(jdir), pathlib.Path(pdir)
    tasks = project / "exegenius/tasks"
    src = pathlib.Path(pages_dir) if pages_dir else project / "exegenius/pages"
    pages = sorted(p.stem for p in src.glob("*.md"))
    pages = [p for p in pages if list(tasks.glob(f"[0-9][0-9][0-9][0-9]-write-{p}.md"))]
    if only:
        pages = [p for p in pages if p in only]
    for d in (jdir / "pages", jdir / "verdicts", jdir / "assignments", pdir):
        d.mkdir(parents=True, exist_ok=True)
    tpl = (J24 / "judge-prompt.md").read_text().replace(str(J24), str(jdir))
    assert str(jdir) in tpl and "ASSIGNMENT" in tpl
    (jdir / "judge-prompt.md").write_text(tpl)
    random.Random(seed).shuffle(pages)
    key = {}
    for i, page in enumerate(pages, 1):
        pid = f"P{i:02d}"
        task = latest_write(tasks, page)
        body, ids = assignment(task.read_text())
        (jdir / "assignments" / f"{page}.md").write_text(body)
        text = (src / f"{page}.md").read_text()
        assert text.strip(), page
        (jdir / "pages" / f"{pid}.md").write_text(text)
        key[pid] = {"page": page, "task": task.stem, "scored_ids": ids}
        for s in ("", "-b", "-c"):
            txt = (tpl.replace("ASSIGNMENT", f"assignments/{page}.md").replace("pages/P01.md", f"pages/{pid}.md")
                   .replace("verdicts/P01.json", f"verdicts/{pid}{s}.json").replace('"page": "P01"', f'"page": "{pid}"')
                   .replace("`P01:", f"`{pid}:"))
            (pdir / f"{pid}{s}.txt").write_text(txt)
    (jdir / "key.json").write_text(json.dumps(key, indent=1) + "\n")
    print(f"{len(key)} pages ({sum(1 for k in key.values() if k['scored_ids'])} with steps), "
          f"{3 * len(key)} prompts -> {pdir}")


def read(jdir):
    jdir = pathlib.Path(jdir)
    key = json.loads((jdir / "key.json").read_text())
    pages, missing = {}, []
    for pid, k in key.items():
        ids = k["scored_ids"]
        c = {"pid": pid, "task": k["task"], "n": len(ids), "faithful": [], "derived": [], "major": [], "minor": []}
        for s in ("", "-b", "-c"):
            f = jdir / "verdicts" / f"{pid}{s}.json"
            if not f.exists():
                missing.append(f.name)
                continue
            v = json.loads(f.read_text())
            items = {i.get("id"): i for i in v.get("items", [])}
            c["faithful"].append(sum(items.get(i, {}).get("faithful") is True for i in ids))
            c["derived"].append(sum(items.get(i, {}).get("presence") == "derived" for i in ids))
            c["major"].append(sum(e.get("severity") == "major" for e in v.get("errors", [])))
            c["minor"].append(sum(e.get("severity") == "minor" for e in v.get("errors", [])))
        n = c["n"]
        if c["major"]:
            c["median_major"] = statistics.median(c["major"])
            c["median_faithful"] = statistics.median(c["faithful"])
            c["meets_standard"] = c["median_major"] == 0 and (n == 0 or c["median_faithful"] >= math.ceil(0.9 * n))
        pages[k["page"]] = c
    judged = [c for c in pages.values() if c["major"]]
    with_steps = [c for c in judged if c["n"]]
    return {"missing_verdicts": sorted(missing), "pages": dict(sorted(pages.items())),
            "P4_pages_with_major_error": sorted(p for p, c in pages.items() if c.get("median_major", 0) > 0),
            "S1_meet_standard": f"{sum(c['meets_standard'] for c in judged)} of {len(judged)} pages",
            "S1_meet_standard_with_steps": f"{sum(c['meets_standard'] for c in with_steps)} of {len(with_steps)}"}


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("prep")
    p.add_argument("project")
    p.add_argument("jdir")
    p.add_argument("pdir")
    p.add_argument("seed", type=int)
    p.add_argument("--pages")
    p.add_argument("--only")
    r = sub.add_parser("read")
    r.add_argument("jdir")
    r.add_argument("--json")
    a = ap.parse_args()
    if a.cmd == "prep":
        prep(a.project, a.jdir, a.pdir, a.seed, a.pages, set(a.only.split(",")) if a.only else None)
    else:
        out = read(a.jdir)
        text = json.dumps(out, indent=1)
        if a.json:
            pathlib.Path(a.json).write_text(text + "\n")
        print(text)


if __name__ == "__main__":
    sys.exit(main())

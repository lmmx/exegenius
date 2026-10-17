"""W3's page judging (J26) and its preregistered reading (plans/W3.md).

usage:
  python3 -I w3judge.py assignments JUDGE_DIR
  python3 -I w3judge.py prep JUDGE_DIR PROMPT_DIR
  python3 -I w3judge.py read JUDGE_DIR [--json OUT.json]

`assignments` writes JUDGE_DIR/assignment-<page>.md for the nine W3 pages from their task files
(identical across arms apart from the root; the WS copy is used). As in J22 and J24, a key-step
page's assignment is its task file's sections "This page in the argument" and "Steps this page
derives". A control page derives no steps (plans/W3.md: scored on errors only), so its assignment
is its sections "This page" and "Where the argument relies on this page" (or, where the task has
none, "This page in the argument"), headed by a line saying
that there are no items to judge.

`prep`, once every run is accepted, copies the 24 accepted pages to JUDGE_DIR/pages/Pnn.md in an
order shuffled with seed 20261047, writes JUDGE_DIR/key.json (Pnn -> run, page, arm) and three
prompts per page (Pnn, Pnn-b, Pnn-c) to PROMPT_DIR. The prompt is J24's judge prompt, unchanged
apart from its paths.

`read` scores the verdicts: per page-run and judge, faithful and derived over the page's scored ids
(`docs/journal/2026-10-15-w3-setup/assigned.json`), major and minor errors; then the primary table
(W2b's per-page rule on WS against WO, the controls' rule, k and its outcome row) and rule R2
(amendment 2) over WS, WS2 and WO on the key-step pages.
"""
import argparse
import json
import math
import pathlib
import random
import re
import shutil
import statistics
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
SETUP = ROOT / "docs/journal/2026-10-15-w3-setup"
J24 = ROOT / "docs/journal/2026-10-14-w2b-writer/judge"
SCRATCH = pathlib.Path("/tmp/claude-0/-home-user/979e8242-2c77-5242-991d-0bed1baea8dd/scratchpad")
SEED = 20261047
KEY_STEP = ["0018", "0021", "0031", "0032", "0033", "0036"]
CONTROL = ["0005", "0006", "0010"]


def section(text, title):
    m = re.search(rf"^## {re.escape(title)}\n(.*?)(?=^## |\Z)", text, re.S | re.M)
    return m.group(0).rstrip() + "\n" if m else None


def task_file(arm, page, task):
    return SCRATCH / "e6" / f"W3{arm}P{page}" / "exegenius/tasks" / f"{task}.md"


def assignments(jdir):
    jdir = pathlib.Path(jdir)
    jdir.mkdir(parents=True, exist_ok=True)
    assigned = json.loads((SETUP / "assigned.json").read_text())["pages"]
    for page, a in assigned.items():
        text = task_file("WS", page, a["task"]).read_text()
        if page in KEY_STEP:
            parts = [section(text, "This page in the argument"), section(text, "Steps this page derives")]
            assert all(parts), f"{page}: sections missing"
            body = "".join(parts)
            for sid in a["scored_ids"]:
                assert sid in body, f"{page}: scored id {sid} not in the assignment"
        else:
            role = section(text, "Where the argument relies on this page") or section(text, "This page in the argument")
            parts = [section(text, "This page"), role]
            assert all(parts), f"{page}: sections missing"
            body = ("**This page derives no steps of the argument: there are no items to judge. Give an empty "
                    "`items` list, and judge the page's mathematical errors only.**\n\n" + "".join(parts))
        (jdir / f"assignment-{page}.md").write_text(body)
    print(f"{len(assigned)} assignments -> {jdir}")


def runs():
    assigned = json.loads((SETUP / "assigned.json").read_text())["pages"]
    out = []
    for page, a in assigned.items():
        arms = ["WS", "WO"] + (["WS2"] if page in KEY_STEP else [])
        for arm in arms:
            out.append((f"W3{arm}P{page}", arm, page, a["task"]))
    return out


def accepted(run, task):
    log = SCRATCH / "e6" / run / "exegenius/log.jsonl"
    if not log.exists():
        return False
    for line in log.read_text().splitlines():
        e = json.loads(line)
        if e.get("event") == "submit" and e.get("task") == task and e.get("outcome") == "accepted":
            return True
    return False


def prep(jdir, pdir):
    jdir, pdir = pathlib.Path(jdir), pathlib.Path(pdir)
    rs = runs()
    missing = [r for r, _, _, t in rs if not accepted(r, t)]
    if missing:
        sys.exit(f"not accepted: {missing}")
    random.Random(SEED).shuffle(rs)
    (jdir / "pages").mkdir(parents=True, exist_ok=True)
    (jdir / "verdicts").mkdir(exist_ok=True)
    pdir.mkdir(parents=True, exist_ok=True)
    tpl = (J24 / "judge-prompt.md").read_text().replace(str(J24), str(jdir))
    assert str(jdir) in tpl and "ASSIGNMENT" in tpl
    (jdir / "judge-prompt.md").write_text(tpl)
    key = {}
    for i, (run, arm, page, task) in enumerate(rs, 1):
        pid = f"P{i:02d}"
        key[pid] = {"run": run, "arm": arm, "page": page}
        src = SCRATCH / "e6" / run / "exegenius/tasks" / f"{task}.answer.md"
        assert src.stat().st_size > 0, run
        shutil.copy(src, jdir / "pages" / f"{pid}.md")
        for s in ("", "-b", "-c"):
            txt = (tpl.replace("ASSIGNMENT", f"assignment-{page}.md").replace("pages/P01.md", f"pages/{pid}.md")
                   .replace("verdicts/P01.json", f"verdicts/{pid}{s}.json").replace('"page": "P01"', f'"page": "{pid}"')
                   .replace("`P01:", f"`{pid}:"))
            (pdir / f"{pid}{s}.txt").write_text(txt)
    (jdir / "key.json").write_text(json.dumps(key, indent=1) + "\n")
    print(f"{len(key)} pages, {3 * len(key)} prompts -> {pdir}")


def better(a, b):
    """W2b's rule: a is better on a key-step page than b."""
    return min(a["faithful"]) >= statistics.median(b["faithful"]) and \
        statistics.median(a["faithful"]) > statistics.median(b["faithful"]) and \
        statistics.median(a["major"]) <= statistics.median(b["major"])


def better_control(a, b):
    ma, mb = statistics.median(a["major"]), statistics.median(b["major"])
    return ma < mb or (ma == mb and statistics.median(a["minor"]) < statistics.median(b["minor"]))


def read(jdir):
    jdir = pathlib.Path(jdir)
    key = json.loads((jdir / "key.json").read_text())
    assigned = json.loads((SETUP / "assigned.json").read_text())["pages"]
    cell, missing = {}, []
    for pid, k in key.items():
        ids = assigned[k["page"]]["scored_ids"]
        c = {"pid": pid, "faithful": [], "derived": [], "major": [], "minor": [], "n": len(ids)}
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
        c["meets_standard"] = (bool(c["major"]) and statistics.median(c["major"]) == 0 and
                               (n == 0 or statistics.median(c["faithful"]) >= math.ceil(0.9 * n)))
        cell[(k["page"], k["arm"])] = c
    out = {"missing_verdicts": sorted(missing), "cells": {f"{p} {a}": c for (p, a), c in sorted(cell.items())}}
    if missing:
        out["reading"] = "incomplete: rerun the missing judges once (plans/W3.md)"
        return out
    per_page, k_opus, k_sonnet = {}, 0, 0
    for p in KEY_STEP:
        o, s = cell[(p, "WO")], cell[(p, "WS")]
        r = "Opus" if better(o, s) else "Sonnet" if better(s, o) else "tied"
        per_page[p] = r
        k_opus += r == "Opus"
        k_sonnet += r == "Sonnet"
    controls, c_opus = {}, 0
    for p in CONTROL:
        o, s = cell[(p, "WO")], cell[(p, "WS")]
        r = "Opus" if better_control(o, s) else "Sonnet" if better_control(s, o) else "tied"
        controls[p] = r
        c_opus += r == "Opus"
    if k_opus >= 5 and c_opus <= 1:
        row = "generalises to key-step pages: propose the split with its measured cost; the author decides"
    elif k_opus >= 5:
        row = "Opus writes better in general, not only on key-step pages: reported with its cost; the author decides"
    elif k_opus >= 2:
        row = "page-specific: no rollout; the exception stays at 0015 and 0016, or is withdrawn by the author"
    else:
        row = "does not generalise: recommend withdrawing the exception (W2b's direct evidence on 0015/0016 is not retested)"
    s_fail = [(p, a) for p in KEY_STEP for a in ("WS", "WS2") if not cell[(p, a)]["meets_standard"]]
    o_fail = [p for p in KEY_STEP if not cell[(p, "WO")]["meets_standard"]]
    fail_pages_opus_met = {p for p, _ in s_fail if cell[(p, "WO")]["meets_standard"]}
    if len(s_fail) <= 1:
        r2 = "Sonnet high meets the fidelity standard on key-step pages at this sample"
    elif len(s_fail) >= 3 and len(fail_pages_opus_met) >= 2 and not o_fail:
        r2 = "the fidelity case for routing key-step pages to Opus"
    else:
        r2 = "inconclusive on fidelity"
    out.update({"primary": {"per_page": per_page, "k_opus": k_opus, "k_sonnet": k_sonnet, "controls": controls,
                            "controls_opus": c_opus, "row": row},
                "R2": {"sonnet_failures": [f"{p} {a}" for p, a in s_fail], "opus_failures": o_fail,
                       "failure_pages_where_opus_met": sorted(fail_pages_opus_met), "reading": r2}})
    return out


def main():
    p = argparse.ArgumentParser()
    sub = p.add_subparsers(dest="cmd", required=True)
    a = sub.add_parser("assignments"); a.add_argument("jdir")
    b = sub.add_parser("prep"); b.add_argument("jdir"); b.add_argument("pdir")
    c = sub.add_parser("read"); c.add_argument("jdir"); c.add_argument("--json")
    x = p.parse_args()
    if x.cmd == "assignments":
        assignments(x.jdir)
    elif x.cmd == "prep":
        prep(x.jdir, x.pdir)
    else:
        out = read(x.jdir)
        text = json.dumps(out, indent=1)
        if x.json:
            pathlib.Path(x.json).write_text(text + "\n")
        print(text)


if __name__ == "__main__":
    main()

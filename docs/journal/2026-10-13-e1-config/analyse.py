"""Summarise E1's content-judge verdicts by arm, and the agreement of paired judges.

The same summary as 2026-10-13-content-judge/analyse.py, with E1's groups.

usage: python3 -I analyse.py VERDICTS_DIR KEY.json PACK_REFERENCE.json

For each plan, over the 19 core items: stated in substance (stated or derived, not judged
wrong), of those stated fully (judged correct, not imprecise), derived (the plan shows how),
and derivations judged faithful; supporting items stated in substance; statements or
derivations judged wrong; major and minor errors. Groups: E1's two arms, and two plans of
the first batch judged again as anchors of the judges' scale.
"""
import json
import pathlib
import statistics
import sys

GROUPS = {
    "E1 P: exegenius-planner, Sonnet, effort high": ["P1", "P2", "P3", "P4", "P5"],
    "E1 G: general-purpose, Sonnet, effort high": ["G1", "G2", "G3", "G4", "G5"],
    "anchors judged again (first batch: sonnet1 19/14/11/6, C2 11/0/3/0)": ["sonnet1", "C2"],
}
ORDER = {"absent": 0, "mentioned": 1, "stated": 2, "derived": 3}


def summary(verdict, tiers):
    s = {"core_stated": 0, "core_correct": 0, "core_derived": 0, "core_faithful": 0,
         "supp_stated": 0, "major": 0, "minor": 0, "wrong": 0}
    for item in verdict["items"]:
        tier = tiers.get(item["id"])
        # Stated in substance: stated or derived, and not judged wrong. The judges call a
        # statement `imprecise` when it is right but omits a quantity, condition or cost.
        present = ORDER.get(item.get("presence"), 0) >= 2 and item.get("statement") != "wrong"
        if tier == "core":
            s["core_stated"] += present
            s["core_correct"] += present and item.get("statement") == "correct"
            s["core_derived"] += item.get("presence") == "derived"
            s["core_faithful"] += item.get("derivation") == "faithful"
        elif tier == "supporting":
            s["supp_stated"] += present
        s["wrong"] += item.get("statement") == "wrong" or item.get("derivation") == "wrong"
    for e in verdict.get("errors", []):
        s["major" if e.get("severity") == "major" else "minor"] += 1
    return s


def main():
    root, key_path, ref_path = pathlib.Path(sys.argv[1]), sys.argv[2], sys.argv[3]
    key = json.load(open(key_path))
    tiers = {i["id"]: i["tier"] for i in json.load(open(ref_path))["items"]}
    by_run = {}
    for pid, run in key.items():
        path = root / f"{pid}.json"
        if path.exists():
            by_run[run] = summary(json.load(open(path)), tiers)
    cols = ["core_stated", "core_correct", "core_derived", "core_faithful", "supp_stated", "wrong", "major", "minor"]
    print("run      " + " ".join(f"{c:>13s}" for c in cols))
    for group, runs in GROUPS.items():
        print(f"-- {group}")
        for run in runs:
            if run in by_run:
                print(f"{run:8s} " + " ".join(f"{by_run[run][c]:13d}" for c in cols))
        present = [by_run[r] for r in runs if r in by_run]
        if present:
            print("median   " + " ".join(f"{statistics.median(p[c] for p in present):13.1f}" for c in cols))

    print("\nAgreement of paired judges (presence, and statement where both found the item stated):")
    for b in sorted(root.glob("P*-b.json")):
        pid = b.stem[:-2]
        a = root / f"{pid}.json"
        if not a.exists():
            continue
        va = {i["id"]: i for i in json.load(open(a))["items"]}
        vb = {i["id"]: i for i in json.load(open(b))["items"]}
        ids = [i for i in va if i in vb]
        same = sum(va[i].get("presence") == vb[i].get("presence") for i in ids)
        within = sum(abs(ORDER.get(va[i].get("presence"), 0) - ORDER.get(vb[i].get("presence"), 0)) <= 1 for i in ids)
        both = [i for i in ids if ORDER.get(va[i].get("presence"), 0) >= 2 and ORDER.get(vb[i].get("presence"), 0) >= 2]
        st = sum(va[i].get("statement") == vb[i].get("statement") for i in both)
        sa, sb = summary(json.load(open(a)), tiers), summary(json.load(open(b)), tiers)
        print(f"{pid} ({key[pid]}): presence same {same}/{len(ids)}, within one level {within}/{len(ids)}; "
              f"statement same {st}/{len(both)}; core stated {sa['core_stated']} vs {sb['core_stated']}, "
              f"core correct {sa['core_correct']} vs {sb['core_correct']}, major errors {sa['major']} vs {sb['major']}")


main()

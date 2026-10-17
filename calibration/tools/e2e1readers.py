"""E2E1's readers analysis on the page judge (R2 and R3; plans/E2E1.md, 23:18).

usage:
  python3 -I e2e1readers.py SIMULATION.json BEFORE.json FINAL.json [--json OUT.json]

SIMULATION.json is the run's `simulation.json` after the last reader round (every accepted read,
both rounds). BEFORE.json and FINAL.json are `e2e1pages.py read --json` outputs: the pages
rewritten after reader round 1, as they stood when the readers started, and every page as it
stands in the final wiki.

R2 pools both rounds. The unit is a page in a round, classed by the verdict on the version the
readers saw: in round 1 a rewritten page by BEFORE and an unchanged one by FINAL; in round 2 every
page by FINAL. A unit is deficient when its median major count is above 0 or it misses the page
standard, sound otherwise. Rate = stuck points / units; the readers verdict's R2 arm needs a
deficient rate at least twice the sound rate and at least 3 stuck points on deficient units.
Stuck points on pages no write task wrote (home, glossary, notation) have no verdict and are
counted apart.

R3 compares each page in BEFORE with FINAL: it gains (loses) when its median faithful count rises
(falls), and gains a major error when its median major count rises. The R3 arm needs most of the
rewritten pages, all of them counted, to gain and none to gain a major error.
"""
import argparse
import collections
import json
import pathlib
import sys


def deficient(c):
    return not c["meets_standard"]


def analyse(sim, before, final):
    stuck = collections.Counter()
    for run in sim["runs"]:
        for s in run.get("stuck", []):
            stuck[(run["round"], s.get("page"))] += 1
    rounds = sorted({r for r, _ in stuck} | {run["round"] for run in sim["runs"]})
    units, unjudged = [], collections.Counter()
    for r in rounds:
        for page, c in final["pages"].items():
            src = before["pages"][page] if r == 1 and page in before["pages"] else c
            units.append({"round": r, "page": page, "deficient": deficient(src), "stuck": stuck[(r, page)],
                          "judged_as": "before" if src is not c else "final"})
        for (rr, page), n in stuck.items():
            if rr == r and page not in final["pages"]:
                unjudged[f"{r}:{page}"] += n
    d = [u for u in units if u["deficient"]]
    s = [u for u in units if not u["deficient"]]
    rate = lambda us: sum(u["stuck"] for u in us) / len(us) if us else None
    rd, rs = rate(d), rate(s)
    ratio = (rd / rs if rs else (float("inf") if rd else None)) if rd is not None else None
    r2 = {"deficient_units": len(d), "deficient_stuck": sum(u["stuck"] for u in d), "deficient_rate": rd,
          "sound_units": len(s), "sound_stuck": sum(u["stuck"] for u in s), "sound_rate": rs, "ratio": ratio,
          "unjudged_stuck": dict(unjudged), "units": units}
    r2["helped"] = bool(ratio is not None and ratio >= 2 and r2["deficient_stuck"] >= 3)

    rows = []
    for page, b in sorted(before["pages"].items()):
        a = final["pages"][page]
        rows.append({"page": page, "n": b["n"], "faithful_before": b["median_faithful"],
                     "faithful_after": a["median_faithful"], "major_before": b["median_major"],
                     "major_after": a["median_major"], "meets_before": b["meets_standard"],
                     "meets_after": a["meets_standard"],
                     "gains": a["median_faithful"] > b["median_faithful"],
                     "loses": a["median_faithful"] < b["median_faithful"],
                     "gains_major": a["median_major"] > b["median_major"]})
    steps = [x for x in rows if x["n"]]
    r3 = {"pages": len(rows), "gain": sum(x["gains"] for x in rows), "lose": sum(x["loses"] for x in rows),
          "gain_major": sum(x["gains_major"] for x in rows),
          "with_steps": len(steps), "gain_with_steps": sum(x["gains"] for x in steps),
          "meets_before": sum(x["meets_before"] for x in rows), "meets_after": sum(x["meets_after"] for x in rows),
          "rows": rows}
    r3["helped"] = r3["gain"] > len(rows) / 2 and r3["gain_major"] == 0
    r3["hurt"] = r3["lose"] > len(rows) / 2 or r3["gain_major"] > len(rows) / 2
    return {"R2": r2, "R3": r3}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("simulation")
    ap.add_argument("before")
    ap.add_argument("final")
    ap.add_argument("--json")
    a = ap.parse_args()
    load = lambda p: json.loads(pathlib.Path(p).read_text())
    out = analyse(load(a.simulation), load(a.before), load(a.final))
    if a.json:
        pathlib.Path(a.json).write_text(json.dumps(out, indent=1) + "\n")
    r2, r3 = out["R2"], out["R3"]
    print(f"R2: deficient {r2['deficient_stuck']}/{r2['deficient_units']} = {r2['deficient_rate']}, "
          f"sound {r2['sound_stuck']}/{r2['sound_units']} = {r2['sound_rate']}, ratio {r2['ratio']}, "
          f"helped {r2['helped']}; unjudged {r2['unjudged_stuck']}")
    print(f"R3: {r3['gain']} of {r3['pages']} gain ({r3['gain_with_steps']} of {r3['with_steps']} with steps), "
          f"{r3['lose']} lose, {r3['gain_major']} gain a major error; meet the standard {r3['meets_before']} -> "
          f"{r3['meets_after']}; helped {r3['helped']}, hurt {r3['hurt']}")
    for x in r3["rows"]:
        print(f"  {x['page']:28} n={x['n']:2} faithful {x['faithful_before']} -> {x['faithful_after']}, "
              f"major {x['major_before']} -> {x['major_after']}")


if __name__ == "__main__":
    sys.exit(main())

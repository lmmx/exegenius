"""SR1's reading (plans/SR1.md): reader coverage on the intact and degraded copies against the no-wiki baseline.

usage: python3 -I sr1read.py INTACT_PROJECT DEGRADED_PROJECT DEGRADATION.json [--json OUT.json]

From each copy's exegenius/simulation.json (round 1, after every grade task is accepted):
- C_int, C_deg: key points the grader found in the reader answers, pooled over runs and goals;
- C_base: the same for the baseline answers, pooled over both copies (their difference is reported
  as grading noise);
- lift = C_int - C_base, sensitivity = C_int - C_deg, read by the plan's fixed rows;
- localisation (secondary): of degraded runs that opened a page with a deletion, the share that
  recorded a stuck point on such a page or gave up on one (its last page);
- descriptives: give-ups, pages opened, leaks, wrong points, repair and held-out goals apart.
Shares carry 80% Wilson intervals; the differences are differences of shares, given with an 80%
Newcombe (hybrid score) interval for description only. The reading uses the point estimates.
"""
import argparse
import json
import math
import pathlib

Z = 1.2816  # 80% two-sided


def wilson(k, n):
    if n == 0:
        return (None, None)
    p = k / n
    d = 1 + Z * Z / n
    c = (p + Z * Z / (2 * n)) / d
    h = Z * math.sqrt(p * (1 - p) / n + Z * Z / (4 * n * n)) / d
    return (round(c - h, 3), round(c + h, 3))


def share(k, n):
    return {"k": k, "n": n, "share": round(k / n, 3) if n else None, "wilson80": wilson(k, n)}


def newcombe(a, b):
    """80% interval for a.share - b.share (independent samples; descriptive only)."""
    (l1, u1), (l2, u2) = a["wilson80"], b["wilson80"]
    p1, p2 = a["k"] / a["n"], b["k"] / b["n"]
    d = p1 - p2
    lo = d - math.sqrt((p1 - l1) ** 2 + (u2 - p2) ** 2)
    hi = d + math.sqrt((u1 - p1) ** 2 + (p2 - l2) ** 2)
    return (round(lo, 3), round(hi, 3))


def coverage(records, goals, keep=lambda g: True):
    k = n = 0
    for r in records:
        g = goals[r["goal"]]
        if not keep(g):
            continue
        n += len(g["key_points"])
        k += len(set(r.get("covered") or []))
    return share(k, n)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("intact")
    ap.add_argument("degraded")
    ap.add_argument("record")
    ap.add_argument("--json")
    x = ap.parse_args()
    sim = {c: json.loads((pathlib.Path(p) / "exegenius/simulation.json").read_text())
           for c, p in (("intact", x.intact), ("degraded", x.degraded))}
    goals = sim["intact"]["goals"]
    assert sim["degraded"]["goals"] == goals, "the two copies' goals differ"
    for c, s in sim.items():
        assert s["round"] == 1 and s["runs"] and s["baselines"], f"{c}: round 1 not complete"
        ungraded = [r["task"] for r in s["runs"] + s["baselines"] if r.get("covered") is None]
        assert not ungraded, f"{c}: ungraded {ungraded}"
    deleted = {d["page"] for d in json.loads(pathlib.Path(x.record).read_text())["deletions"]}

    out = {"goals": len(goals), "key_points": sum(len(g["key_points"]) for g in goals),
           "held_out_goals": sum(bool(g.get("held_out")) for g in goals)}
    c_int = coverage(sim["intact"]["runs"], goals)
    c_deg = coverage(sim["degraded"]["runs"], goals)
    base = sim["intact"]["baselines"] + sim["degraded"]["baselines"]
    c_base = coverage(base, goals)
    b_int, b_deg = coverage(sim["intact"]["baselines"], goals), coverage(sim["degraded"]["baselines"], goals)
    lift = round(c_int["k"] / c_int["n"] - c_base["k"] / c_base["n"], 3)
    sens = round(c_int["k"] / c_int["n"] - c_deg["k"] / c_deg["n"], 3)
    out["primary"] = {"C_int": c_int, "C_deg": c_deg, "C_base": c_base,
                      "C_base_by_copy": {"intact": b_int, "degraded": b_deg,
                                         "difference (grading noise)": round(b_int["share"] - b_deg["share"], 3)},
                      "lift": lift, "lift_newcombe80": newcombe(c_int, c_base),
                      "sensitivity": sens, "sensitivity_newcombe80": newcombe(c_int, c_deg)}
    if lift >= 0.20 and sens >= 0.15:
        row = 1
        reading = "reader coverage reflects the wiki: keep the simulation as configured"
    elif lift < 0.05 or sens < 0.05:
        row = 2
        reading = "reader coverage does not depend on the wiki: propose rounds = 0 by default (a default change, for the author)"
    else:
        row = 3
        reading = "inconclusive: propose the simulation off by default, with its cost and unvalidated signal stated"

    # localisation: degraded runs that opened a deleted page
    opened, localised, detail = 0, 0, []
    for r in sim["degraded"]["runs"]:
        hit = deleted & set(r.get("path") or [])
        if not hit:
            continue
        opened += 1
        stuck_on = sorted({s["page"] for s in r.get("stuck") or []} & hit)
        gave_up_on = r["outcome"] == "gave_up" and (r.get("path") or [None])[-1] in hit
        localised += bool(stuck_on or gave_up_on)
        detail.append({"task": r["task"], "opened_deleted": sorted(hit), "stuck_on_deleted": stuck_on,
                       "gave_up_on_deleted": gave_up_on})
    loc = share(localised, opened)
    out["localisation"] = {**loc, "runs": detail}
    if row == 1:
        reading += ("; stuck points stay as repair notes" if loc["share"] is not None and loc["share"] >= 0.5
                    else "; keep coverage reporting, drop stuck-point repairs")
    out["row"], out["reading"] = row, reading

    # the same stuck-point rate on the same pages in the intact copy (context for localisation; exploratory)
    i_opened = i_stuck = 0
    for r in sim["intact"]["runs"]:
        hit = deleted & set(r.get("path") or [])
        if hit:
            i_opened += 1
            i_stuck += bool({s["page"] for s in r.get("stuck") or []} & hit or
                            (r["outcome"] == "gave_up" and r["path"][-1] in hit))
    out["exploratory"] = {"intact_runs_stuck_on_the_deleted_pages": share(i_stuck, i_opened)}

    desc = {}
    for c, s in sim.items():
        runs = s["runs"]
        desc[c] = {
            "runs": len(runs),
            "gave_up": sum(r["outcome"] == "gave_up" for r in runs),
            "pages_opened_mean": round(sum(len(set(r.get("path") or [])) for r in runs) / len(runs), 2),
            "stuck_points": sum(len(r.get("stuck") or []) for r in runs),
            "runs_with_leaks": sum(bool(r.get("leaks")) for r in runs),
            "wrong_points": sum(len(r.get("wrong") or []) for r in runs),
            "baseline_wrong_points": sum(len(b.get("wrong") or []) for b in s["baselines"]),
            "repair_goals": coverage(runs, goals, lambda g: not g.get("held_out")),
            "held_out_goals": coverage(runs, goals, lambda g: bool(g.get("held_out"))),
            "by_start": {st: coverage([r for r in runs if (r["start"] == "home") == (st == "home")], goals)
                         for st in ("home", "deep")},
            "by_goal": {f"g{gi + 1}{' (held out)' if g.get('held_out') else ''}":
                        [len(set(r.get("covered") or [])) for r in runs if r["goal"] == gi] +
                        [f"/{len(g['key_points'])}"] for gi, g in enumerate(goals)},
        }
    out["descriptive"] = desc
    text = json.dumps(out, indent=1)
    if x.json:
        pathlib.Path(x.json).write_text(text + "\n")
    print(text)


if __name__ == "__main__":
    main()

"""Compare two arms judged in one batch: per-plan counts, medians, the exact one-sided
Mann-Whitney test, central-rubric results and the rubric's agreement and spread.

usage: python3 -I compare.py VERDICTS_DIR KEY.json REFERENCE.json LOW_ARM=R1,R2,... HIGH_ARM=R1,R2,... [ANCHOR=R,...]

The direction of every test is fixed: HIGH_ARM is expected to exceed LOW_ARM. The p-value is
exact: the rank-sum of HIGH_ARM (mid-ranks for ties) is compared with its value under every one
of the C(n1+n2, n2) relabellings, so ties are handled without approximation. It assumes runs are
exchangeable under the null; see calibration/PROTOCOL.md section 6.
"""
import itertools
import json
import pathlib
import statistics
import sys

ORDER = {"absent": 0, "mentioned": 1, "stated": 2, "derived": 3}
GATE = ("C4", "C5", "C6", "C9")
MEASURES = ("core_stated", "core_fully", "core_derived", "core_faithful", "supporting_stated",
            "wrong", "major_errors", "minor_errors", "central_count")


def summary(v, tiers):
    s = dict.fromkeys(MEASURES, 0)
    for it in v["items"]:
        t = tiers.get(it["id"])
        present = ORDER.get(it.get("presence"), 0) >= 2 and it.get("statement") != "wrong"
        if t == "core":
            s["core_stated"] += present
            s["core_fully"] += present and it.get("statement") == "correct"
            s["core_derived"] += it.get("presence") == "derived"
            s["core_faithful"] += it.get("derivation") == "faithful"
        elif t == "supporting":
            s["supporting_stated"] += present
        s["wrong"] += it.get("statement") == "wrong" or it.get("derivation") == "wrong"
    for e in v.get("errors", []):
        s["major_errors" if e.get("severity") == "major" else "minor_errors"] += 1
    c = {st["id"]: st.get("verdict") for st in (v.get("central") or {}).get("steps", [])}
    s["central_count"] = sum(x == "reconstructed" for x in c.values()) if c else None
    s["central_gate"] = (all(c.get(i) == "reconstructed" for i in GATE) and "wrong" not in c.values()) if c else None
    s["central_steps"] = c
    return s


def ranks(values):
    order = sorted(range(len(values)), key=lambda i: values[i])
    r = [0.0] * len(values)
    i = 0
    while i < len(order):
        j = i
        while j + 1 < len(order) and values[order[j + 1]] == values[order[i]]:
            j += 1
        for k in range(i, j + 1):
            r[order[k]] = (i + j) / 2 + 1
        i = j + 1
    return r


def exact_greater(low, high):
    """P(rank-sum of `high` >= observed) over all relabellings."""
    allv = low + high
    r = ranks(allv)
    n = len(allv)
    observed = sum(r[len(low):])
    combos = list(itertools.combinations(range(n), len(high)))
    hits = sum(1 for c in combos if sum(r[i] for i in c) >= observed - 1e-9)
    return hits / len(combos)


def main():
    vdir, key_path, ref_path = pathlib.Path(sys.argv[1]), sys.argv[2], sys.argv[3]
    arms = [a.split("=", 1) for a in sys.argv[4:]]
    arms = [(name, runs.split(",")) for name, runs in arms]
    key = json.load(open(key_path))
    tiers = {i["id"]: i["tier"] for i in json.load(open(ref_path))["items"]}
    by_run = {run: pid for pid, run in key.items()}
    data = {}
    for name, runs in arms:
        data[name] = {r: summary(json.load(open(vdir / f"{by_run[r]}.json")), tiers) for r in runs}

    print("per plan (first judgements):")
    print("arm     run      " + " ".join(f"{m[:13]:>13s}" for m in MEASURES) + "  gate  steps C1..C9")
    for name, runs in arms:
        for r in runs:
            s = data[name][r]
            steps = "".join({"reconstructed": "R", "mentioned": "m", "absent": "-", "wrong": "X"}.get(
                s["central_steps"].get(f"C{i}"), "?") for i in range(1, 10)) if s["central_steps"] else ""
            print(f"{name:7s} {r:8s} " + " ".join(f"{(s[m] if s[m] is not None else -1):13d}" for m in MEASURES)
                  + f"  {str(s['central_gate'])[:5]:5s} {steps}")
        print(f"{name:7s} {'median':8s} " + " ".join(
            f"{statistics.median([data[name][r][m] for r in runs if data[name][r][m] is not None] or [-1]):13.1f}"
            for m in MEASURES))

    if len(arms) >= 2:
        (low, lr), (high, hr) = arms[0], arms[1]
        print(f"\nexact one-sided Mann-Whitney, {high} > {low} (n = {len(hr)} against {len(lr)}):")
        for m in ("core_derived", "core_faithful", "core_stated", "core_fully", "central_count"):
            lo = [data[low][r][m] for r in lr]
            hi = [data[high][r][m] for r in hr]
            if None in lo or None in hi:
                continue
            print(f"  {m:14s} medians {statistics.median(lo):g} -> {statistics.median(hi):g}, "
                  f"difference {statistics.median(hi) - statistics.median(lo):g}, p = {exact_greater(lo, hi):.4f}")
        for name, runs in arms[:2]:
            g = [data[name][r]["central_gate"] for r in runs]
            if None in g:
                print(f"  central gate, {name}: not measured in this batch")
            else:
                print(f"  central gate passed, {name}: {sum(g)}/{len(g)}")

    print("\npaired judgements (central steps, and counts):")
    for b in sorted(vdir.glob("P*-b.json")):
        pid = b.stem[:-2]
        a = summary(json.load(open(vdir / f"{pid}.json")), tiers)
        c = summary(json.load(open(b)), tiers)
        same = (f"{sum(a['central_steps'].get(f'C{i}') == c['central_steps'].get(f'C{i}') for i in range(1, 10))}/9"
                if a["central_steps"] and c["central_steps"] else "not measured")
        print(f"  {pid} ({key[pid]}): central steps same {same}; central count {a['central_count']} vs "
              f"{c['central_count']}; gate {a['central_gate']} vs {c['central_gate']}; core derived "
              f"{a['core_derived']} vs {c['core_derived']}; faithful {a['core_faithful']} vs {c['core_faithful']}")


main()

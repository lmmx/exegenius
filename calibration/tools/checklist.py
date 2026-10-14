"""Analyse a batch of mechanism-checklist judgements against K1's criteria.

usage: python3 -I checklist.py BATCH_DIR MARKERS.json REFERENCE.json EARLIER_BATCH_DIR [EARLIER_BATCH_DIR ...]

BATCH_DIR holds key.json and verdicts/Pnn.json (and Pnn-b.json for second judgements), in the
shape `docs/journal/2026-10-14-k1-checklist/judge/judge-prompt.md` asks for. MARKERS.json is
`markers.py` output for the same runs. Each EARLIER_BATCH_DIR is a content-judge batch (key.json,
verdicts/) whose majority faithful verdict on each core result is the comparison for criterion 1.
Prints, for `calibration/plans/K1.md`:
1. the AUC of a result's checklist completeness for faithful against non-faithful pairs;
2. agreement of the first judge's `present` with the markers, with disagreements by direction;
3. agreement between first and second judges, and their totals;
4. each plan's total, for the resolution and ordering checks.
"""
import glob
import json
import pathlib
import sys

GROUPS = {"B": "bank-exchange-network", "L": "label-residual-budget", "P": "complex-phase-interface",
          "R": "rank-saving", "M": "matrix-shear", "S": "simultaneous-layer"}


def present(v):
    return {s["id"]: s["verdict"] == "present" for s in v["steps"]}


def main():
    batch, markers_path, reference = map(pathlib.Path, sys.argv[1:4])
    earlier = [pathlib.Path(p) for p in sys.argv[4:]]
    key = json.load(open(batch / "key.json"))
    markers = json.load(open(markers_path))
    first = {key[f.stem]: present(json.load(open(f))) for f in sorted((batch / "verdicts").glob("P??.json"))}
    second = {key[f.stem[:3]]: present(json.load(open(f))) for f in sorted((batch / "verdicts").glob("P??-b.json"))}

    faithful = {}
    for d in earlier:
        k = json.load(open(d / "key.json"))
        for pid, run in k.items():
            if run not in first:
                continue
            vs = [json.load(open(f)) for f in sorted(glob.glob(f"{d}/verdicts/{pid}.json")) +
                  sorted(glob.glob(f"{d}/verdicts/{pid}-*.json"))]
            for g, item in GROUPS.items():
                votes = [next(i for i in v["items"] if i["id"] == item).get("derivation") == "faithful" for v in vs]
                faithful[(run, g)] = sum(votes) * 2 > len(votes)

    print("## Criterion 1: completeness against the earlier majority faithful verdict")
    pairs = []
    for (run, g), f in sorted(faithful.items()):
        ids = [i for i in first[run] if i.startswith(g)]
        pairs.append((f, sum(first[run][i] for i in ids) / len(ids), run, g))
    yes = [c for f, c, *_ in pairs if f]
    no = [c for f, c, *_ in pairs if not f]
    auc = sum((a > b) + 0.5 * (a == b) for a in yes for b in no) / (len(yes) * len(no))
    print(f"pairs {len(pairs)}: faithful {len(yes)} (mean completeness {sum(yes) / len(yes):.2f}), "
          f"not faithful {len(no)} (mean {sum(no) / len(no):.2f}); AUC {auc:.3f}; "
          f"met: {sum(yes) / len(yes) > sum(no) / len(no) and auc >= 0.80}")
    for g, item in GROUPS.items():
        y = [c for f, c, r, gg in pairs if f and gg == g]
        n = [c for f, c, r, gg in pairs if not f and gg == g]
        a = sum((p > q) + 0.5 * (p == q) for p in y for q in n) / (len(y) * len(n)) if y and n else None
        print(f"  {item}: faithful {len(y)}, not {len(n)}, AUC {'n/a' if a is None else f'{a:.2f}'}")

    print("\n## Criterion 2: first judge against markers")
    agree = judge_only = marker_only = total = 0
    by_step = {}
    for run, p in first.items():
        for i, v in p.items():
            m = markers[run].get(i)
            if m is None:
                continue
            total += 1
            agree += v == m
            judge_only += v and not m
            marker_only += m and not v
            by_step.setdefault(i, [0, 0])
            by_step[i][0] += v == m
            by_step[i][1] += 1
    print(f"agreement {agree}/{total} = {agree / total:.3f}; judge present, marker absent {judge_only}; "
          f"marker present, judge absent {marker_only}; met: {agree / total >= 0.80}")
    print("  per sub-step agreement: " + ", ".join(f"{i} {a}/{n}" for i, (a, n) in by_step.items()))

    print("\n## Criterion 3: first against second judge")
    agree = n = 0
    ok = True
    for run, p2 in second.items():
        p1 = first[run]
        a = sum(p1[i] == p2[i] for i in p1)
        agree += a
        n += len(p1)
        t1, t2 = sum(p1.values()), sum(p2.values())
        ok &= abs(t1 - t2) <= 3
        print(f"  {run}: agreement {a}/{len(p1)}, totals {t1} and {t2}")
    if n:
        print(f"agreement {agree}/{n} = {agree / n:.3f}; met: {agree / n >= 0.80 and ok}")

    print("\n## Criterion 4: totals (first judge; second in brackets) and marker hits")
    for run in sorted(first):
        groups = " ".join(f"{g}{sum(first[run][i] for i in first[run] if i.startswith(g))}" for g in GROUPS)
        extra = f" [{sum(second[run].values())}]" if run in second else ""
        hits = sum(1 for v in markers[run].values() if v)
        print(f"  {run}: {sum(first[run].values())}{extra} of 26 ({groups}); markers {hits} of 25")


if __name__ == "__main__":
    main()

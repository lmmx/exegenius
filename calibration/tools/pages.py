"""Score a W2-style page-judging batch: per page and judge, the assigned items derived and faithful, and major errors.

usage: python3 -I pages.py JUDGE_DIR

Only the items the assignment lists are counted: the page's step and its listed sub-steps
(page 0014: S4.2, S4.2.1, S4.2.2; page 0015: S4.3, S4.3.1-S4.3.7). Some judges also itemised
nested sub-steps; those extra items are reported but not scored.
"""
import json, pathlib, statistics, sys

ASSIGNED = {"0014": ["S4.2", "S4.2.1", "S4.2.2"],
            "0015": ["S4.3"] + [f"S4.3.{i}" for i in range(1, 8)],
            # W2b, fixed 23:58 UTC before any J24 verdict: page 0016's assignment lists only S4.4 as
            # its sub-step, with S4.4.1-S4.4.3 beneath it; all four ids named there are scored
            "0016": ["S4.4", "S4.4.1", "S4.4.2", "S4.4.3"]}
d = pathlib.Path(sys.argv[1])
key = json.load(open(d / "key.json"))
rows = []
for pid, k in sorted(key.items(), key=lambda x: (x[1]["page"], x[1]["run"])):
    ids = ASSIGNED[k["page"]]
    per = []
    for s in ("", "-b", "-c"):
        f = d / "verdicts" / f"{pid}{s}.json"
        if not f.exists():
            continue
        v = json.load(open(f))
        items = {i["id"]: i for i in v["items"]}
        der = sum(items.get(i, {}).get("presence") == "derived" for i in ids)
        fai = sum(items.get(i, {}).get("faithful") is True for i in ids)
        maj = sum(e.get("severity") == "major" for e in v.get("errors", []))
        per.append((fai, der, maj, len(items), v.get("motivation_given")))
    if not per:
        continue
    f = [p[0] for p in per]; m = [p[2] for p in per]
    rows.append((k["page"], k["run"], pid, len(ids), f, statistics.median(f), [p[1] for p in per], m, statistics.median(m), [p[3] for p in per], [p[4] for p in per]))
print("| Page | Run | Plan | Assigned | Faithful (each) | Median | Derived (each) | Major errors (each) | Items itemised (each) | Motivation (each) |")
print("|---|---|---|---|---|---|---|---|---|---|")
for r in rows:
    print(f"| {r[0]} | {r[1]} | {r[2]} | {r[3]} | {r[4]} | {r[5]} | {r[6]} | {r[7]} | {r[9]} | {r[10]} |")
print()
for page in ASSIGNED:
    ws = next((r for r in rows if r[0] == page and r[1].startswith("W2WS")), None)  # first WS run per page
    if not ws:
        continue
    for r in rows:
        if r[0] != page or r is ws:
            continue
        below = max(r[4]) < min(ws[4])
        above = min(r[4]) > max(ws[4])
        adequate = (not below) and r[8] <= ws[8]
        print(f"page {page}: {r[1]} {r[4]} vs WS {ws[4]}: {'above' if above else 'below' if below else 'within noise'}; "
              f"major errors {r[8]} vs {ws[8]}; adequate={adequate}")

# W2b's reading (plans/W2b.md, fixed before J24): pool each configuration's runs on a page, and call
# Opus better on that page if all its pooled faithful values are at least Sonnet's pooled median and
# its pooled median exceeds Sonnet's.
if "--pooled" in sys.argv:
    print()
    for page in ASSIGNED:
        pool = {}
        for r in rows:
            if r[0] == page:
                c = pool.setdefault(r[1][3], {"f": [], "m": [], "runs": []})
                c["f"] += r[4]; c["m"] += r[7]; c["runs"].append(r[1])
        if "S" not in pool or "O" not in pool:
            continue
        s, o = pool["S"], pool["O"]
        sm, om = statistics.median(s["f"]), statistics.median(o["f"])
        better = min(o["f"]) >= sm and om > sm
        print(f"page {page} pooled: O {o['runs']} faithful {o['f']} (median {om}), major {o['m']} (median {statistics.median(o['m'])}); "
              f"S {s['runs']} faithful {s['f']} (median {sm}), major {s['m']} (median {statistics.median(s['m'])}); Opus better: {better}")

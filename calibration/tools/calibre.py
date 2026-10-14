"""Pass rates for each calibre, per configuration, from Evaluation Registry records.

usage: python3 -I calibre.py CALIBRES.json RECORD.json [RECORD.json ...]

A configuration is the observed (agent types, models, efforts) of a run, as its record gives
it, with the binary that wrote its task prompts; runs whose configuration is unavailable are reported as such. For each configuration and
judge batch, a run passes a calibre when its first judgement in that batch passes every gate.
Pass counts are reported with exact two-sided 95% Clopper-Pearson intervals. Excluded runs are left out.
"""
import collections
import json
import math
import sys


def binom_cdf(k, n, p):
    return sum(math.comb(n, i) * p**i * (1 - p) ** (n - i) for i in range(k + 1))


def clopper_pearson(k, n, alpha=0.05):
    """Exact two-sided interval for a binomial proportion (each tail alpha/2), by bisection."""
    if n == 0:
        return (0.0, 1.0)

    def solve(f, lo=0.0, hi=1.0):
        for _ in range(100):
            mid = (lo + hi) / 2
            if f(mid):
                hi = mid
            else:
                lo = mid
        return (lo + hi) / 2

    lower = 0.0 if k == 0 else solve(lambda p: 1 - binom_cdf(k - 1, n, p) >= alpha / 2)
    upper = 1.0 if k == n else solve(lambda p: binom_cdf(k, n, p) <= alpha / 2)
    return (lower, upper)


def passes(q, gates):
    if any(q.get(g["measure"]) is None for g in gates):
        return None
    for g in gates:
        v = q[g["measure"]]
        if "min" in g and v < g["min"]:
            return False
        if "max" in g and v > g["max"]:
            return False
        if "equals" in g and v != g["equals"]:
            return False
    return True


def main():
    calibres = json.load(open(sys.argv[1]))
    table = collections.defaultdict(lambda: collections.defaultdict(list))
    for path in sys.argv[2:]:
        rec = json.load(open(path))
        for r in rec["runs"]:
            if "excluded" in r:
                continue
            oc = r.get("observed_config") or {}
            if oc.get("basis") == "unavailable" or not oc.get("models"):
                conf = "unavailable"
            else:
                conf = (f"{'+'.join(oc['agent_types'])} / {'+'.join(oc['models'])} / "
                        f"{'+'.join(oc.get('efforts') or ['?'])} / binary "
                        f"{rec['arms'][r['arm']]['intended'].get('binary', rec['binary']['commit'])}")
            for q in r["quality"]:
                if q["judge"] == "a":
                    table[(conf, q["batch"])][rec["id"]].append((r["id"], q))
    for (conf, batch), by_exp in sorted(table.items()):
        runs = [x for v in by_exp.values() for x in v]
        print(f"{conf} | batch {batch} | {len(runs)} runs from {', '.join(sorted(by_exp))}")
        for name, cal in calibres.items():
            res = [passes(q, cal["gates"]) for _, q in runs]
            known = [x for x in res if x is not None]
            if not known:
                print(f"  {name}: not measurable in this batch")
                continue
            k, n = sum(known), len(known)
            lo, hi = clopper_pearson(k, n)
            print(f"  {name}: {k}/{n} pass (95% exact {lo:.2f}-{hi:.2f})")


main()

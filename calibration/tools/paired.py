"""Compare two arms judged in one batch whose runs are paired: per-pair differences, the exact
one-sided sign test, and the central rubric's steps and gate per arm.

usage: python3 -I paired.py VERDICTS_DIR KEY.json REFERENCE.json BASE1:VARIANT1 BASE2:VARIANT2 ... [--anchors=R,...]

The direction is fixed: each VARIANT is expected to exceed its BASE. The sign test counts the
pairs where the variant is strictly greater, drops ties, and gives P(at least that many) under
Binomial(n, 1/2), exactly. First judgements only: second judgements (`Pnn-b.json`) are read for
agreement, not counted.
"""
import json
import math
import pathlib
import statistics
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))
from compare import MEASURES, summary  # noqa: E402


def sign_test(diffs):
    n = sum(d != 0 for d in diffs)
    k = sum(d > 0 for d in diffs)
    p = sum(math.comb(n, i) for i in range(k, n + 1)) / 2 ** n if n else 1.0
    return k, n, p


def main():
    verdicts, key, reference = sys.argv[1:4]
    pairs = [a.split(":") for a in sys.argv[4:] if not a.startswith("--")]
    anchors = [a.split("=", 1)[1].split(",") for a in sys.argv[4:] if a.startswith("--anchors=")]
    anchors = anchors[0] if anchors else []
    key = json.load(open(key))
    run_of = {v: k for k, v in key.items()}
    tiers = {i["id"]: i["tier"] for i in json.load(open(reference))["items"]}
    s = {run: summary(json.load(open(pathlib.Path(verdicts) / f"{plan}.json")), tiers)
         for plan, run in key.items() if (pathlib.Path(verdicts) / f"{plan}.json").exists()}

    cols = ("core_stated", "core_fully", "core_derived", "core_faithful", "wrong", "major_errors",
            "central_count")
    print("run   plan " + " ".join(f"{c:>14}" for c in cols) + "  gate  C1-C9")
    for run in [r for p in pairs for r in p] + anchors:
        if run not in s:
            print(f"{run:5} {run_of.get(run, '?'):4} (no verdict)")
            continue
        x = s[run]
        steps = " ".join({"reconstructed": "R", "mentioned": "m", "absent": "-", "wrong": "X"}
                         .get(x["central_steps"].get(f"C{i}"), "?") for i in range(1, 10))
        print(f"{run:5} {run_of[run]:4} " + " ".join(f"{x[c]:>14}" for c in cols)
              + f"  {'pass' if x['central_gate'] else 'fail':4}  {steps}")

    done = [(b, v) for b, v in pairs if b in s and v in s]
    print(f"\npairs judged: {len(done)} of {len(pairs)}")
    for m in MEASURES:
        diffs = [s[v][m] - s[b][m] for b, v in done]
        k, n, p = sign_test(diffs)
        print(f"{m:18} differences {diffs}  median {statistics.median(diffs) if diffs else '-'}  "
              f"variant greater in {k} of {n} untied, one-sided p = {p:.3f}")

    for name, arm in (("base", [b for b, _ in done]), ("variant", [v for _, v in done])):
        per = [sum(s[r]["central_steps"].get(f"C{i}") == "reconstructed" for r in arm) for i in range(1, 10)]
        print(f"{name:8} reconstructed per step C1-C9: {per}; gate passes {sum(bool(s[r]['central_gate']) for r in arm)} of {len(arm)}; "
              f"totals: wrong {sum(s[r]['wrong'] for r in arm)}, major errors {sum(s[r]['major_errors'] for r in arm)}")

    seconds = sorted(p.stem for p in pathlib.Path(verdicts).glob("*-b.json"))
    for sec in seconds:
        first = sec[:-2]
        a = json.load(open(pathlib.Path(verdicts) / f"{first}.json"))
        b = json.load(open(pathlib.Path(verdicts) / f"{sec}.json"))
        ca = {x["id"]: x["verdict"] for x in a["central"]["steps"]}
        cb = {x["id"]: x["verdict"] for x in b["central"]["steps"]}
        agree = sum(ca.get(i) == cb.get(i) for i in ca)
        sa, sb = summary(a, tiers), summary(b, tiers)
        print(f"second judgement {first} ({key[first]}): central steps agree {agree} of {len(ca)}; "
              f"central count {sa['central_count']} against {sb['central_count']}; "
              f"derived {sa['core_derived']} against {sb['core_derived']}; "
              f"gate {sa['central_gate']} against {sb['central_gate']}")


if __name__ == "__main__":
    main()

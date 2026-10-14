"""Summarise a judging batch in which some plans have several independent judgements
(`Pnn.json`, `Pnn-b.json`, `Pnn-c.json`, ...), deciding each central step by majority.

usage: python3 -I majority.py VERDICTS_DIR KEY.json REFERENCE.json

For each plan, per central step C1-C9: the verdict a strict majority of its judges gave, or
"split" when none did. The central count is the number of steps whose majority verdict is
`reconstructed`; the gate passes when C4, C5, C6 and C9 are reconstructed by majority and no step
is `wrong` by majority. Content counts are given per judge and as the median. Each judge's own
central count is listed too, so the spread between judges stays visible.
"""
import collections
import json
import pathlib
import statistics
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))
from compare import GATE, summary  # noqa: E402

ABBR = {"reconstructed": "R", "mentioned": "m", "absent": "-", "wrong": "X", "split": "?"}


def main():
    vdir, key_path, reference = map(pathlib.Path, sys.argv[1:4])
    key = json.load(open(key_path))
    tiers = {i["id"]: i["tier"] for i in json.load(open(reference))["items"]}
    rows = []
    for plan, run in sorted(key.items(), key=lambda kv: kv[1]):
        files = sorted(vdir.glob(f"{plan}.json")) + sorted(vdir.glob(f"{plan}-*.json"))
        if not files:
            continue
        verdicts = [json.load(open(f)) for f in files]
        sums = [summary(v, tiers) for v in verdicts]
        steps = {}
        for i in range(1, 10):
            c = collections.Counter(s["central_steps"].get(f"C{i}") for s in sums)
            top, n = c.most_common(1)[0]
            steps[f"C{i}"] = top if n * 2 > len(sums) else "split"
        count = sum(v == "reconstructed" for v in steps.values())
        gate = all(steps[g] == "reconstructed" for g in GATE) and "wrong" not in steps.values()
        med = {m: statistics.median(s[m] for s in sums) for m in
               ("core_stated", "core_derived", "core_faithful", "major_errors")}
        rows.append((run, plan, len(sums), count, gate, "".join(ABBR[steps[f"C{i}"]] for i in range(1, 10)),
                     [s["central_count"] for s in sums], med, [s["major_errors"] for s in sums],
                     [s["core_derived"] for s in sums], [s["core_faithful"] for s in sums]))
    print("| Run | Plan | Judges | Central (majority) | Gate | C1-C9 | Each judge's central | "
          "Derived (median; each) | Faithful (median; each) | Major errors (each) |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for run, plan, n, count, gate, steps, each, med, majors, der, fai in rows:
        print(f"| {run} | {plan} | {n} | {count} | {'pass' if gate else 'fail'} | {steps} | {each} | "
              f"{med['core_derived']:g}; {der} | {med['core_faithful']:g}; {fai} | {majors} |")


if __name__ == "__main__":
    main()

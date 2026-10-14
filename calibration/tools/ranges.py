"""Checklist totals per plan, with their range across judges, for comparisons that are decided by
non-overlapping judge ranges (`plans/S4.md`, `S5.md`, `E8.md`).

usage: python3 -I ranges.py BATCH_DIR [RUN_A:RUN_B ...]

BATCH_DIR holds key.json and verdicts/Pnn.json, Pnn-b.json, ... (checklist verdicts). Prints each
run's totals per judge and, for each pair RUN_A:RUN_B, whether RUN_A's range lies entirely above
RUN_B's ("above"), entirely below ("below"), or overlaps ("within noise").
"""
import json
import pathlib
import sys


def totals(batch):
    key = json.load(open(batch / "key.json"))
    out = {}
    for pid, run in key.items():
        files = sorted(batch.glob(f"verdicts/{pid}.json")) + sorted(batch.glob(f"verdicts/{pid}-*.json"))
        out[run] = [sum(s["verdict"] == "present" for s in json.load(open(f))["steps"]) for f in files]
    return out


def main():
    batch = pathlib.Path(sys.argv[1])
    t = totals(batch)
    for run in sorted(t):
        print(f"{run}: {t[run]}")
    for pair in sys.argv[2:]:
        a, b = pair.split(":")
        ra, rb = t[a], t[b]
        verdict = "above" if min(ra) > max(rb) else "below" if max(ra) < min(rb) else "within noise"
        print(f"{a} {ra} vs {b} {rb}: {verdict}")


if __name__ == "__main__":
    main()

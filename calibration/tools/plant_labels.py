"""Rescore A1 with the classifier check's hand labels in place of plant.py's ASSERTS pattern.

usage: python3 -I plant_labels.py --key KEY.json --answers DIR --notes NOTES.tsv --labels LABELS.tsv
                                 [--flip-borderline] [--json OUT.json]

NOTES.tsv is `plant.py score --notes` output, with its `.unblind.json` beside it. LABELS.tsv has
columns row, label (1 = the note asserts an error) and borderline ("b" or empty). Each gap is matched
to its row by configuration, run, code, quote and note, and `plant.asserts` is replaced by the row's
label, so every measure that uses ASSERTS (R_A, F_A, F_sham, U, the clean-site baseline) is recomputed
by plant.py's own scoring. --flip-borderline inverts the borderline labels, as a sensitivity check.

It also prints the agreement of ASSERTS with the labels, overall and per configuration.
"""
import argparse
import csv
import importlib.util
import json
import pathlib


def load_plant():
    spec = importlib.util.spec_from_file_location("plant", pathlib.Path(__file__).with_name("plant.py"))
    plant = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(plant)
    return plant


def clean(s):
    return s.replace("\t", " ").replace("\n", " ")


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--key", required=True)
    p.add_argument("--answers", required=True)
    p.add_argument("--notes", required=True)
    p.add_argument("--labels", required=True)
    p.add_argument("--flip-borderline", action="store_true")
    p.add_argument("--json")
    args = p.parse_args()
    plant = load_plant()
    # QUOTE_NONE: plant.py writes the notes unquoted, and a quote or note may begin with '"'.
    notes = list(csv.DictReader(open(args.notes), delimiter="\t", quoting=csv.QUOTE_NONE))
    blind = json.loads(pathlib.Path(args.notes + ".unblind.json").read_text())
    labels = {r["row"]: r for r in csv.DictReader(open(args.labels), delimiter="\t", quoting=csv.QUOTE_NONE)}
    hand, agree = {}, {}
    for n in notes:
        config, run = blind[n["row"]]
        lab = labels[n["row"]]
        y = int(lab["label"])
        if args.flip_borderline and lab["borderline"] == "b":
            y = 1 - y
        k = (config, run, n["code"], n["quote"], n["note"])
        if k in hand and hand[k] != y:
            raise SystemExit(f"row {n['row']}: duplicate gap with a different label")
        hand[k] = y
        a = agree.setdefault(config, {"rows": 0, "agree": 0, "hand_1": 0, "asserts_1": 0, "both_1": 0})
        x = int(n["asserts"])
        a["rows"] += 1; a["agree"] += x == y; a["hand_1"] += y; a["asserts_1"] += x; a["both_1"] += x and y

    data = plant.load_answers(args.answers)
    # Tag each gap with its key, so the replacement `asserts` can look its label up.
    tagged = 0
    for config in data:
        for run in data[config]:
            for code, rec in data[config][run].items():
                for g in (rec.get("audit") or {}).get("gaps", []):
                    g["_key"] = (config, run, code, clean(g.get("quote", "")), clean(g.get("note", "")))
                    if g["_key"] not in hand:
                        raise SystemExit(f"gap without a label: {g['_key'][:3]}")
                    tagged += 1
    if tagged != len(hand):
        raise SystemExit(f"{tagged} gaps against {len(hand)} labels")
    plant.asserts = lambda g: bool(hand[g["_key"]])
    report = plant.cmd_score(argparse.Namespace(key=args.key, answers=None, json=None, notes=None), data=data)
    total = {k: sum(a[k] for a in agree.values()) for k in ("rows", "agree", "hand_1", "asserts_1", "both_1")}
    agreement = {c: dict(a, share=round(a["agree"] / a["rows"], 3),
                         asserts_recall=round(a["both_1"] / a["hand_1"], 3) if a["hand_1"] else None)
                 for c, a in sorted(agree.items()) + [("all", total)]}
    for c, a in agreement.items():
        print(c, a)
    if args.json:
        pathlib.Path(args.json).write_text(json.dumps({"labels": "hand, borderline flipped" if args.flip_borderline
                                                       else "hand", "agreement": agreement,
                                                       "report": report}, indent=1) + "\n")


if __name__ == "__main__":
    main()

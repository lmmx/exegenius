"""Read A2 by its preregistered table (plans/A2.md, "Reading"), with the adjudicated F_A.

usage: python3 -I a2read.py --key KEY.json --answers DIR --notes NOTES.tsv --labels LABELS.tsv
                            --adjudication ADJ.json [--json OUT.json]

Recall and F_sham come from `plant.py`'s own scoring, with `plant.asserts` set to one of three
classifiers: ASSERTS, the hand labels (primary), and the hand labels with borderlines flipped.
F_A needs the clean-page asserting gaps outside native defects one by one, so each gap of a clean
variant is scored alone with `plant.score_audit` (a gap's site and cues do not depend on the
others). Each such gap is looked up in ADJ.json (verdicts written blind to arm): a genuine find is
excluded from F_A. A gap with no verdict (one that only another classifier asserts) counts as a
false alarm and is listed. F_A is given raw, adjudicated, and adjudicated with borderline verdicts
flipped.

The outcome rows and the stage-2 condition are applied as written; the stage-2 condition is
reported under both the strict ("less than 0.10 from the threshold") and the inclusive ("at most
0.10") reading, since a point estimate of 0 on F_sham sits exactly 0.10 from its threshold.
"""
import argparse
import csv
import importlib.util
import json
import math
import pathlib

Z80 = 1.2816


def load(name):
    spec = importlib.util.spec_from_file_location(name, pathlib.Path(__file__).with_name(f"{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def wilson(k, n, z=Z80):
    if n == 0:
        return None
    p = k / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return [round(max(0.0, c - h), 3), round(min(1.0, c + h), 3)]


def clean(s):
    return s.replace("\t", " ").replace("\n", " ")


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--key", required=True)
    p.add_argument("--answers", required=True)
    p.add_argument("--notes", required=True)
    p.add_argument("--labels", required=True)
    p.add_argument("--adjudication", required=True)
    p.add_argument("--json")
    args = p.parse_args()
    plant = load("plant")
    key = json.loads(pathlib.Path(args.key).read_text())
    pages_dir = pathlib.Path(args.key).parent
    notes = list(csv.DictReader(open(args.notes), delimiter="\t", quoting=csv.QUOTE_NONE))
    blind = json.loads(pathlib.Path(args.notes + ".unblind.json").read_text())
    labels = {r["row"]: r for r in csv.DictReader(open(args.labels), delimiter="\t", quoting=csv.QUOTE_NONE)}
    adj = {i["row"]: i for i in json.loads(pathlib.Path(args.adjudication).read_text())["items"]}
    gap_row = {}
    for n in notes:
        config, run = blind[n["row"]]
        gap_row[(config, run, n["code"], n["quote"], n["note"])] = n["row"]

    def classifier(mode):
        def lab(row):
            y = int(labels[row]["label"])
            return 1 - y if mode == "hand-flip" and labels[row]["borderline"] == "b" else y
        if mode == "asserts":
            return lambda g: plant_asserts(g)
        return lambda g: bool(lab(g["_row"]))

    plant_asserts = plant.asserts
    data = plant.load_answers(args.answers)
    for config in data:
        for run in data[config]:
            for code, rec in data[config][run].items():
                for g in (rec.get("audit") or {}).get("gaps", []):
                    g["_row"] = gap_row[(config, run, code, clean(g.get("quote", "")), clean(g.get("note", "")))]

    variants = {v["code"]: v for v in key["variants"]}
    out = {}
    for mode in ("hand", "asserts", "hand-flip"):
        plant.asserts = classifier(mode)
        report = plant.cmd_score(argparse.Namespace(key=args.key, answers=None, json=None, notes=None),
                                 data=data, quiet=True)
        arms = {}
        for config in sorted(data):
            r = report[config]
            fa_rows = []
            for run in data[config]:
                for code, rec in data[config][run].items():
                    if variants[code]["kind"] != "clean":
                        continue
                    for g in (rec.get("audit") or {}).get("gaps", []):
                        if plant.score_audit(key, pages_dir, variants[code], {"gaps": [g]})["spurious_asserted"]:
                            fa_rows.append(g["_row"])
            pages = r["fa_clean_asserted"]["n"]
            unadj = [x for x in fa_rows if x not in adj]
            false = [x for x in fa_rows if x not in adj or not adj[x]["genuine"]]
            false_flip = [x for x in fa_rows if x not in adj or adj[x]["genuine"] == adj[x]["borderline"]]
            assert len(fa_rows) == r["fa_clean_asserted"]["k"], (mode, config, fa_rows, r["fa_clean_asserted"])
            sq_k = r["audit_named_source"]["k"] + r["audit_named_quote"]["k"]
            sq_n = r["audit_named_source"]["n"] + r["audit_named_quote"]["n"]
            m = {
                "R_src": r["audit_named_source"], "R_page": r["audit_named_page"], "R_quote": r["audit_named_quote"],
                "F_sham": r["fa_sham_asserted"],
                "F_A": {"k": len(false), "pages": pages, "rate": round(len(false) / pages, 3)},
                "F_A_raw": {"k": len(fa_rows), "pages": pages, "rate": round(len(fa_rows) / pages, 3)},
                "F_A_adj_flipped": {"k": len(false_flip), "pages": pages, "rate": round(len(false_flip) / pages, 3)},
                "src_quote": {"k": sq_k, "n": sq_n, "rate": round(sq_k / sq_n, 3), "ci80": wilson(sq_k, sq_n)},
                "clean_rows": fa_rows, "genuine_rows": [x for x in fa_rows if x in adj and adj[x]["genuine"]],
                "unadjudicated_rows": unadj,
            }
            arms[config] = m
        out[mode] = {"arms": arms, "reading": read(arms, "F_A"), "reading_adj_flipped": read(arms, "F_A_adj_flipped"),
                     "reading_raw": read(arms, "F_A_raw")}
    plant.asserts = plant_asserts
    for mode, o in out.items():
        print(f"== {mode}")
        for arm, m in o["arms"].items():
            print(f"  {arm:3} R_src {fmt(m['R_src'])}  R_page {fmt(m['R_page'])}  R_quote {fmt(m['R_quote'])}  "
                  f"F_sham {fmt(m['F_sham'])}  src+quote {fmt(m['src_quote'])}")
            print(f"      F_A {m['F_A']['k']}/{m['F_A']['pages']} = {m['F_A']['rate']}   raw {m['F_A_raw']['rate']}   "
                  f"adj-flipped {m['F_A_adj_flipped']['rate']}   unadjudicated {m['unadjudicated_rows'] or '-'}")
        for which in ("reading", "reading_adj_flipped", "reading_raw"):
            print(f"  {which}: {o[which]}")
    if args.json:
        pathlib.Path(args.json).write_text(json.dumps(out, indent=1) + "\n")


def fmt(x):
    return f"{x['k']}/{x['n']} {x['rate']} {x.get('ci80')}"


def meets(m, fa, src):
    r = m["R_page"]
    ok = {"R_page>=0.50": r["rate"] >= 0.5, "R_page_lb>=0.35": r["ci80"][0] >= 0.35,
          "F_A<=0.5": m[fa]["rate"] <= 0.5, "F_sham<=0.10": m["F_sham"]["rate"] <= 0.10}
    if src:
        ok["R_src>=0.50"] = m["R_src"]["rate"] >= 0.5
    return all(ok.values()), ok


def beyond(a, b):
    """a's 80% interval lies wholly above b's."""
    return a["ci80"][0] > b["ci80"][1]


def read(arms, fa):
    B, I, SI = arms["B"], arms["I"], arms["SI"]
    si_ok, si_detail = meets(SI, fa, True)
    i_ok, i_detail = meets(I, fa, False)
    si_b = beyond(SI["src_quote"], B["src_quote"])
    si_i = beyond(SI["src_quote"], I["src_quote"])
    fa_fail = not (si_detail["F_A<=0.5"] and si_detail["F_sham<=0.10"])
    if si_ok and si_b:
        row = 1
    elif si_b and fa_fail:
        row = 2
    elif i_ok and not si_i:
        row = 3
    elif not i_ok and not si_ok:
        row = 4
    else:
        row = 5
    return {"row": row, "SI_meets": si_ok, "SI_detail": si_detail, "I_meets": i_ok, "I_detail": i_detail,
            "SI_srcquote_beyond_B": si_b, "SI_srcquote_beyond_I": si_i,
            "SI_Rsrc_beyond_I": beyond(SI["R_src"], I["R_src"])}


if __name__ == "__main__":
    main()

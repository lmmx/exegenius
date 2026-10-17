"""A3's clean-page alarms, the independent reviewer's packet, and the preregistered reading (plans/A3.md).

usage:
  python3 -I a3read.py items  --run RUN_DIR
  python3 -I a3read.py build  --run RUN_DIR PACKET_DIR
  python3 -I a3read.py read   --run RUN_DIR VERDICTS.json [--transcript T.jsonl] [--json OUT.json]

RUN_DIR is docs/journal/2026-10-17-a3-run, holding answers/, A3-notes.tsv (+ .unblind.json) and
labels.tsv. The key is A2's (docs/journal/2026-10-17-a2-design/key.json).

`items` lists arm R's asserting gaps on clean pages outside native defects (plant.py's
`spurious_asserted`), under the hand labels (primary), the hand labels with borderlines flipped,
and ASSERTS, and writes RUN_DIR/alarms.json: the union, so that every classifier's F_A can be
decided by the reviewer.

`build` (with --record NAME for a rerun's record directory, default `review`) writes the reviewer's packet exactly as A2's review did (`a2review.py`: the same task text,
note masking, the four decoys and the seed), with A3's alarms in place of A2's, and RUN_DIR/review/
key.json (id -> alarm row or decoy). The reviewer sees no arm, no label and no key.

`read` scores R by plant.py under each classifier (R_src, R_page, R_quote, F_sham), sets F_A to the
alarms the reviewer did not find genuine, per clean page (6), and applies A3's rows: a decoy
accepted means R cannot pass on F_A. Row 2's choice compares R's F_A with SI's under A2's
independent review (1.00).
"""
import argparse
import csv
import importlib.util
import json
import math
import pathlib
import random
import re
import shutil

ROOT = pathlib.Path(__file__).resolve().parents[2]
KEY = ROOT / "docs/journal/2026-10-17-a2-design/key.json"
SI_FA_REVIEW = 1.00  # A2's SI under the independent review (plans/A2.md, R3)
CLEAN_PAGES = 6
Z80 = 1.2816


def load(name):
    spec = importlib.util.spec_from_file_location(name, pathlib.Path(__file__).with_name(f"{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def wilson(k, n, z=Z80):
    p = k / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return [round(max(0.0, c - h), 3), round(min(1.0, c + h), 3)]


def clean(s):
    return s.replace("\t", " ").replace("\n", " ")


def setup(run):
    plant = load("plant")
    run = pathlib.Path(run)
    key = json.loads(KEY.read_text())
    notes = list(csv.DictReader(open(run / "A3-notes.tsv"), delimiter="\t", quoting=csv.QUOTE_NONE))
    blind = json.loads((run / "A3-notes.tsv.unblind.json").read_text())
    labels = {r["row"]: r for r in csv.DictReader(open(run / "labels.tsv"), delimiter="\t", quoting=csv.QUOTE_NONE)}
    gap_row = {(*blind[n["row"]], n["code"], n["quote"], n["note"]): n["row"] for n in notes}
    data = plant.load_answers(str(run / "answers"))
    for config in data:
        for r in data[config]:
            for code, rec in data[config][r].items():
                for g in (rec.get("audit") or {}).get("gaps", []):
                    g["_row"] = gap_row[(config, r, code, clean(g.get("quote", "")), clean(g.get("note", "")))]
    asserts = plant.asserts

    def classifier(mode):
        if mode == "asserts":
            return asserts

        def f(g):
            lab = labels[g["_row"]]
            y = int(lab["label"])
            return bool(1 - y if mode == "hand-flip" and lab["borderline"] == "b" else y)
        return f
    return plant, key, data, notes, classifier


def clean_alarms(plant, key, data):
    variants = {v["code"]: v for v in key["variants"]}
    rows = []
    for config in data:
        for r in data[config]:
            for code, rec in data[config][r].items():
                if variants[code]["kind"] != "clean":
                    continue
                for g in (rec.get("audit") or {}).get("gaps", []):
                    if plant.score_audit(key, KEY.parent, variants[code], {"gaps": [g]})["spurious_asserted"]:
                        rows.append(g["_row"])
    return rows


def cmd_items(run):
    plant, key, data, notes, classifier = setup(run)
    by_mode = {}
    for mode in ("hand", "hand-flip", "asserts"):
        plant.asserts = classifier(mode)
        by_mode[mode] = sorted(clean_alarms(plant, key, data), key=int)
    union = sorted(set().union(*by_mode.values()), key=int)
    note = {n["row"]: n for n in notes}
    variants = {v["code"]: v for v in key["variants"]}
    items = [{"row": r, "code": note[r]["code"], "page": variants[note[r]["code"]]["page"],
              "quote": note[r]["quote"], "note": note[r]["note"],
              "classifiers": [m for m in by_mode if r in by_mode[m]]} for r in union]
    pathlib.Path(run, "alarms.json").write_text(json.dumps({"by_classifier": by_mode, "items": items}, indent=1,
                                                           ensure_ascii=False) + "\n")
    print(json.dumps(by_mode))
    print(f"{len(items)} clean-page alarms (union) -> {pathlib.Path(run, 'alarms.json')}")


def cmd_build(run, packet, record_name="review"):
    a2r = load("a2review")
    run, packet = pathlib.Path(run), pathlib.Path(packet)
    record = run / record_name
    if packet.exists():
        shutil.rmtree(packet)
    (packet / "pages").mkdir(parents=True)
    record.mkdir(parents=True, exist_ok=True)
    key = json.loads(KEY.read_text())
    clean_pages = {v["page"]: KEY.parent / v["file"] for v in key["variants"] if v["kind"] == "clean"}
    for page, f in clean_pages.items():
        shutil.copy(f, packet / "pages" / f"{page}.md")
    paper = pathlib.Path(key["paper"]["path"])
    assert a2r.sha(paper) == key["paper"]["sha256"], "paper differs from the key's"
    shutil.copy(paper, packet / "paper.tex")
    alarms = json.loads((run / "alarms.json").read_text())["items"]
    entries = [{"source": {"row": a["row"]}, "page": a["page"], "quote": a["quote"], "note": a2r.mask(a["note"])}
               for a in alarms]
    for d in a2r.DECOYS:
        body = (packet / "pages" / f"{d['page']}.md").read_text()
        assert body.count(d["quote"]) == 1, f"{d['decoy']}: quote not once on its page"
        entries.append({"source": {"decoy": d["decoy"], "why": d["why"]}, "page": d["page"], "quote": d["quote"],
                        "note": a2r.mask(d["note"])})
    leaks = [e["note"] for e in entries
             if re.search(r"\b1\d{3}\b|\blines?\s*\d|paper|Lemma\s+\w+-|Proposition\s+\w+-", e["note"])]
    assert not leaks, leaks
    random.Random(a2r.SEED).shuffle(entries)
    items, ids = [], {}
    for n, e in enumerate(entries, 1):
        rid = f"R{n:02d}"
        ids[rid] = e["source"]
        items.append({"id": rid, "page": e["page"], "quote": e["quote"], "note": e["note"]})
    (packet / "items.json").write_text(json.dumps({"items": items}, indent=1, ensure_ascii=False) + "\n")
    (packet / "task.md").write_text(a2r.TASK.format(packet=packet))
    shutil.copy(packet / "items.json", record / "items.json")
    shutil.copy(packet / "task.md", record / "task.md")
    (record / "key.json").write_text(json.dumps({
        "seed": a2r.SEED, "packet": str(packet), "ids": ids,
        "sha256": {"alarms.json": a2r.sha(run / "alarms.json"), "labels.tsv": a2r.sha(run / "labels.tsv"),
                   "items.json": a2r.sha(packet / "items.json"), "task.md": a2r.sha(packet / "task.md"),
                   "paper.tex": a2r.sha(packet / "paper.tex")}}, indent=1) + "\n")
    print(f"{len(items)} items ({len(alarms)} alarms, {len(a2r.DECOYS)} decoys) -> {packet}; key -> {record / 'key.json'}")


def cmd_read(run, verdicts, transcript=None, record_name="review"):
    a2r = load("a2review")
    run = pathlib.Path(run)
    rkey = json.loads((run / record_name / "key.json").read_text())
    assert a2r.sha(run / "alarms.json") == rkey["sha256"]["alarms.json"], "alarms.json changed after the packet"
    alarms = json.loads((run / "alarms.json").read_text())
    got = {v["id"]: v for v in json.loads(pathlib.Path(verdicts).read_text())["items"]}
    missing = sorted(set(rkey["ids"]) - set(got))
    verdict, decoys = {}, {}
    for rid, src in rkey["ids"].items():
        if rid not in got:
            continue
        g = got[rid]["verdict"] == "genuine"
        b = got[rid].get("confidence") == "borderline"
        if "decoy" in src:
            decoys[src["decoy"]] = {"id": rid, "genuine": g, "borderline": b}
        else:
            verdict[src["row"]] = {"id": rid, "genuine": g, "borderline": b, "reason": got[rid].get("reason", "")}
    decoys_accepted = sum(d["genuine"] for d in decoys.values())

    plant, key, data, notes, classifier = setup(run)
    out = {"missing": missing, "decoys": decoys, "decoys_accepted": decoys_accepted, "verdicts": verdict,
           "classifiers": {}}
    if transcript:
        out["outside_packet"] = a2r.touched(transcript, rkey["packet"])
    for mode in ("hand", "asserts", "hand-flip"):
        plant.asserts = classifier(mode)
        rep = plant.cmd_score(argparse.Namespace(key=str(KEY), answers=None, json=None, notes=None),
                              data=data, quiet=True)["R"]
        rows = alarms["by_classifier"][mode]
        false = [r for r in rows if not verdict.get(r, {}).get("genuine")]
        false_flip = [r for r in rows if not (verdict.get(r, {}).get("genuine", False) ^ verdict.get(r, {}).get("borderline", False))]
        m = {"R_src": rep["audit_named_source"], "R_page": rep["audit_named_page"], "R_quote": rep["audit_named_quote"],
             "F_sham": rep["fa_sham_asserted"],
             "clean_alarms": rows, "genuine": [r for r in rows if verdict.get(r, {}).get("genuine")],
             "F_A": {"k": len(false), "pages": CLEAN_PAGES, "rate": round(len(false) / CLEAN_PAGES, 3)},
             "F_A_reviewer_borderline_flipped": {"k": len(false_flip), "rate": round(len(false_flip) / CLEAN_PAGES, 3)},
             "F_A_raw": {"k": len(rows), "rate": round(len(rows) / CLEAN_PAGES, 3)}}
        m["reading"] = reading(m, decoys_accepted)
        out["classifiers"][mode] = m
    plant.asserts = classifier("asserts")
    if missing:
        out["status"] = "incomplete: rerun the reviewer once (plans/A3.md, via A2's protocol)"
    elif out.get("outside_packet"):
        out["status"] = "contaminated: rerun the reviewer once in a fresh packet"
    else:
        out["status"] = "complete"
    return out


def reading(m, decoys_accepted):
    rp, rs = m["R_page"], m["R_src"]
    page_ok = rp["rate"] >= 0.5 and rp["ci80"][0] >= 0.35
    src_ok = rs["rate"] >= 0.5
    fa_ok = m["F_A"]["rate"] <= 0.5 and not decoys_accepted
    sham_ok = m["F_sham"]["rate"] <= 0.10
    detail = {"R_page>=0.50 & lb>=0.35": page_ok, "R_src>=0.50": src_ok, "F_A<=0.5 (no decoy accepted)": fa_ok,
              "F_sham<=0.10": sham_ok}
    if page_ok and src_ok and fa_ok and sham_ok:
        row, rec = 1, "R is the recommended audit configuration (a proposed default change, for the author)"
    elif page_ok and src_ok:
        pick = "R" if m["F_A"]["rate"] <= SI_FA_REVIEW else "SI"
        row, rec = 2, (f"a source-aware audit is recommended provisionally, not adequate on false alarms: {pick} "
                       f"(R's F_A {m['F_A']['rate']} against SI's {SI_FA_REVIEW})")
    else:
        row, rec = 3, "the refinement cost recall: SI is recommended provisionally"
    return {"row": row, "recommendation": rec, "detail": detail}


def main():
    p = argparse.ArgumentParser()
    sub = p.add_subparsers(dest="cmd", required=True)
    a = sub.add_parser("items"); a.add_argument("--run", required=True)
    b = sub.add_parser("build"); b.add_argument("--run", required=True); b.add_argument("packet")
    b.add_argument("--record", default="review")
    c = sub.add_parser("read"); c.add_argument("--run", required=True); c.add_argument("verdicts")
    c.add_argument("--transcript"); c.add_argument("--json"); c.add_argument("--record", default="review")
    x = p.parse_args()
    if x.cmd == "items":
        cmd_items(x.run)
    elif x.cmd == "build":
        cmd_build(x.run, x.packet, x.record)
    else:
        out = cmd_read(x.run, x.verdicts, x.transcript, x.record)
        text = json.dumps(out, indent=1, ensure_ascii=False)
        if x.json:
            pathlib.Path(x.json).write_text(text + "\n")
        for mode, m in out["classifiers"].items():
            f = lambda d: f"{d['k']}/{d['n']} {d['rate']} {d.get('ci80')}"
            print(f"{mode:9} R_src {f(m['R_src'])}  R_page {f(m['R_page'])}  R_quote {f(m['R_quote'])}  "
                  f"F_sham {f(m['F_sham'])}  F_A {m['F_A']['k']}/6={m['F_A']['rate']} (raw {m['F_A_raw']['rate']})  "
                  f"row {m['reading']['row']}")
        print("decoys accepted:", out["decoys_accepted"], "| status:", out["status"])
        print("primary (hand):", out["classifiers"]["hand"]["reading"]["recommendation"])


if __name__ == "__main__":
    main()

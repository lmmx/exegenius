"""A2's independent review of the clean-page alarms (plans/A2.md, "Independent review").

usage:
  python3 -I a2review.py build PACKET_DIR RECORD_DIR
  python3 -I a2review.py score RECORD_DIR VERDICTS.json [--transcript T.jsonl] [--json OUT.json]

`build` writes the reviewer's packet to PACKET_DIR (outside the repository):
- items.json: the 15 alarms of `docs/journal/2026-10-17-a2-run/adjudication-items.json` and the
  constructed decoys below, shuffled with a fixed seed, each as an id, its page, the quoted text and
  the note. Notes are masked by `mask()`: paper line numbers and ranges, `paper.tex` and the names
  of the paper's labelled results are removed, and "the paper" becomes "the source", so that a note
  does not cite where its claim is checked. What a note asserts is left as written;
- pages/<page>.md: the six clean pages, as A2's auditors saw them;
- paper.tex: the paper (sha256 checked against the key);
- task.md: the reviewer's instructions (the adjudication rule, the output format).
It writes RECORD_DIR/key.json (each id's source: an A2 row, or a decoy) and copies of the packet's
items.json and task.md, for the record. The reviewer sees neither the key nor any verdict.

Decoys are alarms written for this review, each objecting to a statement that is correct (the
`why` field says why). They measure leniency: a reviewer that calls one genuine is not reliable on
this item set. They are excluded from every A2 measure.

`score` reads the reviewer's verdicts and reports, against the original adjudication (which it
does not change): agreement on the 15 alarms (share and Cohen's kappa), the disagreements, decoys
accepted, and SI's and I's genuine counts and F_A under the original adjudication, the reviewer, the
reviewer with its borderline verdicts flipped, both agreeing (intersection) and either (union).
With --transcript, it lists any path the reviewer touched outside PACKET_DIR.
"""
import argparse
import hashlib
import json
import pathlib
import random
import re
import shutil
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
RUN = ROOT / "docs/journal/2026-10-17-a2-run"
DESIGN = ROOT / "docs/journal/2026-10-17-a2-design"
SEED = 20261017
CLEAN_PAGES = 6

DECOYS = [
    {"decoy": "D1", "page": "recursive-power-saving",
     "quote": r"For $\lambda=0.8$ the ratio is $\tfrac{3/2}{2^{0.8}}\approx0.86$.",
     "note": "Arithmetic error: 2^{0.8} is about 1.74, so (3/2)/2^{0.8} is about 1.16, not 0.86. The example's "
             "ratio is then above 1, which contradicts the sentence before it: with log_2(3/2) about 0.585 < 0.8, "
             "the choice lambda = 0.8 should make the ratio below 1, so the numbers as given are inconsistent.",
     "why": "1.5 / 2^0.8 = 1.5 / 1.741 = 0.862; the note inverts the ratio"},
    {"decoy": "D2", "page": "linear-network-roles",
     "quote": r"$(D_{\alpha_k}D_{\alpha_{k-1}}^{-1})\cdots(D_{\alpha_1}D_{\alpha_0}^{-1})=D_{\alpha_k}D_{\alpha_0}^{-1}$",
     "note": "The product is composed in the wrong order. The first edge's change D_{alpha_1}D_{alpha_0}^{-1} acts "
             "first, so the route applies (D_{alpha_1}D_{alpha_0}^{-1})...(D_{alpha_k}D_{alpha_{k-1}}^{-1}), and in "
             "that order adjacent factors do not cancel. The telescoping to the endpoint operator then holds only "
             "if the frames commute, a hypothesis the page does not state.",
     "why": "operators compose right to left: the first edge's factor is rightmost, as the page writes it, and "
            "adjacent factors cancel with no commutation"},
    {"decoy": "D3", "page": "subset-intersection-construction",
     "quote": r"Let $N=\binom h3^3$, the number of indices $\iota$",
     "note": "Miscount. An index iota = (T_1,T_2,T_3) is a choice of three triples, which the source treats as a "
             "set, so the number of indices is binom(binom(h,3),3), not binom(h,3)^3. The signed total WD-2N and "
             "the deficit computed from it change accordingly.",
     "why": "the page defines iota = (T_1,T_2,T_3) in T^3, an ordered triple, so there are binom(h,3)^3 indices"},
    {"decoy": "D4", "page": "fast-address-interchange",
     "quote": r"since $H+(D-H)=D$ and $D-(D-H)=H$",
     "note": "The identity D-(D-H)=H is used for a general residue ring, but over Z/q^bZ the third update maps "
             "(D, D-H) to (D, D-(D-H)), which equals H only when 2H = 0, that is in characteristic 2. The source "
             "states this step for F_2 only; the page drops that hypothesis.",
     "why": "D - (D - H) = H in every ring; the note's characteristic condition is wrong"},
]

TASK = """# Review of alarms raised against explanatory pages

You are reviewing alarms that automated auditors raised against six pages of an explanatory wiki about a mathematics paper. Each alarm quotes a statement from a page and says, in a note, what is wrong with it. Some alarms identify real misstatements and some do not. Decide each one yourself, from the page and the paper.

**Files (use only these; do not read any other file or directory):**
- `{packet}/items.json`: the alarms, each with an `id`, the `page`, the `quote` and the `note`.
- `{packet}/pages/<page>.md`: the six pages, in Markdown with LaTeX mathematics. `<cite data-source="paper">…</cite>` marks a verbatim quotation of the paper.
- `{packet}/paper.tex`: the paper (LaTeX, about 5,400 lines). Search it for the terms and symbols of each statement, and read the passages around each match.

**The notes have had their citations removed,** and what a note says the source states is an unverified claim. Check every claim, in the page and in the paper. A note can be confident and wrong.

**The rule.** An alarm is **genuine** if the quoted statement, read in the context of its page, misstates the mathematics:
(a) it is false as stated, or inconsistent with the rest of its page;
(b) it is presented as following from what precedes it, but does not follow; or
(c) it omits a hypothesis that the paper requires for that claim, so that as stated it is false or wider than what the paper establishes.

An alarm is **not genuine** if the quoted statement is correct (the objection's mathematics is wrong, or it misreads the page), or if the objection is only that the page does not explain, justify or motivate a correct statement. Judge the statement, not the note: a note can be partly wrong and still identify a genuine misstatement; then say which part holds. Mark a verdict **borderline** when competent mathematicians could reasonably disagree, for example when the statement is an informal simplification that an explanatory page might legitimately make.

**Output.** Write `{packet}/verdicts.json`, one entry per alarm, in this form, and nothing else:

```json
{{"items": [
  {{"id": "R01", "verdict": "genuine", "confidence": "clear",
    "page_evidence": "what the page says that bears on it",
    "paper_evidence": "the paper's line numbers and what they say",
    "reason": "two to four sentences: why the statement is or is not a misstatement"}}
]}}
```

`verdict` is `genuine` or `not_genuine`; `confidence` is `clear` or `borderline`. Give a verdict for every id. Give paper line numbers wherever the paper bears on the decision. Then reply with one line: the number of verdicts written.
"""


def mask(s):
    s = re.sub(r"\bpaper\.tex\b", "the source", s)
    s = re.sub(r"\b(Lemma|Proposition|Theorem|Corollary|Definition|Section)\s+[A-Za-z]+(?:[-:][A-Za-z]+)+(\s+proof)?",
               lambda m: f"a {m.group(1).lower()}" + ("'s proof" if m.group(2) else ""), s)
    s = re.sub(r"\((?:the\s+)?paper\s+", "(", s)
    s = re.sub(r"\b(?:paper\s+)?(?:lines?|ll?\.|L)\s*\d{2,4}(?:\s*[-–]\s*\d{2,4})?", "", s, flags=re.I)
    s = re.sub(r"\b(?:at\s+)?\d{3,4}\s*[-–]\s*\d{3,4}\b", "", s)
    s = re.sub(r"\bThe paper's\b", "The source's", s)
    s = re.sub(r"\bthe paper's\b", "the source's", s)
    s = re.sub(r"\bThe paper\b", "The source", s)
    s = re.sub(r"\bthe paper\b", "the source", s)
    s = re.sub(r"\bpaper\b", "source", s)
    # tidy what the removals leave behind
    for _ in range(3):
        s = re.sub(r"\(\s*(?:,|and|at)?\s*\)", "", s)
        s = re.sub(r"\(\s*(?:the source|a (?:lemma|proposition|theorem|corollary|definition|section)(?:'s proof)?)\s*\)", "", s)
        s = re.sub(r"\(\s*,\s*", "(", s)
        s = re.sub(r",\s*(?:and\s+)?(?:at\s*)?\)", ")", s)
        s = re.sub(r"\s+(?:and|at)\s*\)", ")", s)
        s = re.sub(r"([.;])\s*(?:and|at)\s*([.;]|$)", r"\1", s)
        s = re.sub(r"\s+([.,;)])", r"\1", s)
        s = re.sub(r"\s{2,}", " ", s)
    return s.strip()


def sha(p):
    return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()


def build(packet, record):
    packet, record = pathlib.Path(packet), pathlib.Path(record)
    if packet.exists():
        shutil.rmtree(packet)
    (packet / "pages").mkdir(parents=True)
    record.mkdir(parents=True, exist_ok=True)
    key = json.loads((DESIGN / "key.json").read_text())
    clean = {v["page"]: DESIGN / v["file"] for v in key["variants"] if v["kind"] == "clean"}
    for page, f in clean.items():
        shutil.copy(f, packet / "pages" / f"{page}.md")
    paper = pathlib.Path(key["paper"]["path"])
    assert sha(paper) == key["paper"]["sha256"], "paper differs from the key's"
    shutil.copy(paper, packet / "paper.tex")
    alarms = json.loads((RUN / "adjudication-items.json").read_text())
    entries = [{"source": {"row": a["row"]}, "page": a["page"], "quote": a["quote"], "note": mask(a["note"])}
               for a in alarms]
    for d in DECOYS:
        body = (packet / "pages" / f"{d['page']}.md").read_text()
        assert body.count(d["quote"]) == 1, f"{d['decoy']}: quote not once on its page"
        entries.append({"source": {"decoy": d["decoy"], "why": d["why"]}, "page": d["page"], "quote": d["quote"],
                        "note": mask(d["note"])})
    for e in entries:
        assert re.search(r"\b1\d{3}\b|\blines?\s*\d|paper|Lemma\s+\w+-|Proposition\s+\w+-", e["note"]) is None, e["note"]
    random.Random(SEED).shuffle(entries)
    items, ids = [], {}
    for n, e in enumerate(entries, 1):
        rid = f"R{n:02d}"
        ids[rid] = e["source"]
        items.append({"id": rid, "page": e["page"], "quote": e["quote"], "note": e["note"]})
    (packet / "items.json").write_text(json.dumps({"items": items}, indent=1, ensure_ascii=False) + "\n")
    (packet / "task.md").write_text(TASK.format(packet=packet))
    shutil.copy(packet / "items.json", record / "items.json")
    shutil.copy(packet / "task.md", record / "task.md")
    (record / "key.json").write_text(json.dumps({
        "seed": SEED, "packet": str(packet), "ids": ids,
        "sha256": {"adjudication.json": sha(RUN / "adjudication.json"),
                   "adjudication-items.unblind.json": sha(RUN / "adjudication-items.unblind.json"),
                   "items.json": sha(packet / "items.json"), "task.md": sha(packet / "task.md"),
                   "paper.tex": sha(packet / "paper.tex"),
                   **{f"pages/{p}.md": sha(packet / "pages" / f"{p}.md") for p in sorted(clean)}}},
        indent=1) + "\n")
    print(f"{len(items)} items ({len(alarms)} alarms, {len(DECOYS)} decoys) -> {packet}; key -> {record / 'key.json'}")


def kappa(pairs):
    n = len(pairs)
    po = sum(a == b for a, b in pairs) / n
    pa, pb = sum(a for a, _ in pairs) / n, sum(b for _, b in pairs) / n
    pe = pa * pb + (1 - pa) * (1 - pb)
    return round((po - pe) / (1 - pe), 3) if pe < 1 else None


def touched(transcript, packet):
    paths = set()
    for line in open(transcript):
        m = json.loads(line).get("message") or {}
        for c in m.get("content", []) if isinstance(m.get("content"), list) else []:
            if c.get("type") != "tool_use":
                continue
            i = c.get("input", {})
            for k in ("file_path", "path"):
                if isinstance(i.get(k), str):
                    paths.add(i[k])
            if i.get("command"):
                paths.update(re.findall(r"(?:^|(?<=[\s'\"=(]))(/(?:tmp|home|root)/[\w./-]+)", i["command"]))
    return sorted(p for p in paths if p.startswith(("/tmp/", "/home/", "/root/")) and not p.startswith(str(packet)))


def score(record, verdicts, transcript=None):
    record = pathlib.Path(record)
    key = json.loads((record / "key.json").read_text())
    assert sha(RUN / "adjudication.json") == key["sha256"]["adjudication.json"], "the original adjudication changed"
    orig = {i["row"]: i for i in json.loads((RUN / "adjudication.json").read_text())["items"]}
    arm = json.loads((RUN / "adjudication-items.unblind.json").read_text())
    got = {v["id"]: v for v in json.loads(pathlib.Path(verdicts).read_text())["items"]}
    missing = sorted(set(key["ids"]) - set(got))
    rev, decoys = {}, {}
    for rid, src in key["ids"].items():
        if rid not in got:
            continue
        g = int(got[rid]["verdict"] == "genuine")
        b = got[rid].get("confidence") == "borderline"
        if "decoy" in src:
            decoys[src["decoy"]] = {"id": rid, "genuine": g, "borderline": b}
        else:
            rev[src["row"]] = {"id": rid, "genuine": g, "borderline": b, "reason": got[rid].get("reason", ""),
                               "paper_evidence": got[rid].get("paper_evidence", "")}
    rows = sorted(rev, key=int)
    pairs = [(orig[r]["genuine"], rev[r]["genuine"]) for r in rows]
    schemes = {
        "original": lambda r: orig[r]["genuine"],
        "original, borderline flipped": lambda r: orig[r]["genuine"] ^ orig[r]["borderline"],
        "reviewer": lambda r: rev[r]["genuine"],
        "reviewer, borderline flipped": lambda r: rev[r]["genuine"] ^ rev[r]["borderline"],
        "both genuine (intersection)": lambda r: orig[r]["genuine"] and rev[r]["genuine"],
        "either genuine (union)": lambda r: orig[r]["genuine"] or rev[r]["genuine"],
    }
    by = {}
    for name, f in schemes.items():
        by[name] = {}
        for a in ("I", "SI"):
            rs = [r for r in rows if arm[r] == a]
            genuine = sum(bool(f(r)) for r in rs)
            false = len(rs) - genuine
            by[name][a] = {"alarms": len(rs), "genuine": genuine, "false": false,
                           "F_A": round(false / CLEAN_PAGES, 3), "meets_F_A": false / CLEAN_PAGES <= 0.5}
    out = {
        "missing": missing,
        "agreement": {"items": len(pairs), "agree": sum(a == b for a, b in pairs),
                      "share": round(sum(a == b for a, b in pairs) / len(pairs), 3) if pairs else None,
                      "kappa": kappa(pairs) if pairs else None},
        "disagreements": [{"row": r, "arm": arm[r], "original": orig[r]["genuine"], "original_borderline": orig[r]["borderline"],
                           "reviewer": rev[r]["genuine"], "reviewer_borderline": rev[r]["borderline"],
                           "original_reason": orig[r]["reason"], "reviewer_reason": rev[r]["reason"]}
                          for r in rows if orig[r]["genuine"] != rev[r]["genuine"]],
        "decoys": decoys, "decoys_accepted": sum(d["genuine"] for d in decoys.values()),
        "by_scheme": by,
    }
    if transcript:
        out["outside_packet"] = touched(transcript, key["packet"])
    sig = by["reviewer"]["SI"]["genuine"]
    inter = by["both genuine (intersection)"]["SI"]["genuine"]
    if missing:
        reading = "incomplete: rerun once (plans/A2.md)"
    elif out.get("outside_packet"):
        reading = "contaminated: rerun once in a fresh packet (plans/A2.md)"
    elif out["decoys_accepted"]:
        reading = "R4: the reviewer accepted a decoy; its verdicts are reported but cannot by themselves settle F_A"
    elif sig >= 6 and inter >= 6:
        reading = "R1: SI's false-alarm criterion is met under the independent review and under both-agree"
    elif sig >= 6:
        reading = "R2: met under the independent review, not under both-agree"
    else:
        reading = "R3: not met under the independent review; the recall/false-alarm trade-off is documented"
    out["reading"] = reading
    return out


def main():
    p = argparse.ArgumentParser()
    sub = p.add_subparsers(dest="cmd", required=True)
    b = sub.add_parser("build")
    b.add_argument("packet")
    b.add_argument("record")
    s = sub.add_parser("score")
    s.add_argument("record")
    s.add_argument("verdicts")
    s.add_argument("--transcript")
    s.add_argument("--json")
    a = p.parse_args()
    if a.cmd == "build":
        build(a.packet, a.record)
    else:
        out = score(a.record, a.verdicts, a.transcript)
        text = json.dumps(out, indent=1, ensure_ascii=False)
        if a.json:
            pathlib.Path(a.json).write_text(text + "\n")
        print(text)


if __name__ == "__main__":
    main()

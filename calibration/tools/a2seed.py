"""A2's error set: A1's 24 page variants plus 12 new source-only errors in 6 new variants.

usage: python3 -I a2seed.py OUT_DIR

Writes OUT_DIR/key.json and OUT_DIR/pages/: A1's 24 variants unchanged (copied from
docs/journal/2026-10-14-a1-audit-design) and one new variant per page carrying two source-only
errors (plans/A2-draft.md). A source-only error is a statement about the paper's construction,
values, hypotheses or attributions that the page neither derives nor contradicts, outside any
quotation that would reveal it: only the paper shows it is wrong.

The key is in plant.py's format, so `plant.py score` scores A2 answers unchanged. The checks are
plant.py's own (each replaced text occurs once on its clean page; the error's site is visible
to the auditor and unique; no new symbols reach the auditor; bound symbols still occur; every
quotation is verbatim in the paper), plus: the two errors of a variant are in different
sentences, and no new error's text is inside a citation mark.
"""
import hashlib
import importlib.util
import json
import pathlib
import shutil
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
A1 = ROOT / "docs/journal/2026-10-14-a1-audit-design"


def load_plant():
    spec = importlib.util.spec_from_file_location("plant", pathlib.Path(__file__).with_name("plant.py"))
    plant = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(plant)
    return plant


P = load_plant()
E = P.E

# Written 2026-10-17 from paper.tex (sha256 6d2b6d00...); `why` gives the paper's lines.
SEEDS = [
    E("ASF-S1", "address-shear-factorization", "hypothesis", ["source"],
      r"Here $q$ is a fixed odd prime,", r"Here $q$ is a fixed prime, which may be taken to be $2$,",
      "the paper requires a fixed odd prime, chosen to avoid every denominator in the table of factors "
      "and every numerator of a diagonal entry of an invertible factor; q = 2 is excluded. (A first "
      "draft only dropped 'odd'; rejected by the independent review because 'a prime' is still true "
      "and the paper says 'a prime' at 1478 and 1651)", "1397, 1415-1417",
      ["odd prime", "q=2", "q = 2", "taken to be 2", "taken to be $2$", "must be odd"]),
    E("ASF-S2", "address-shear-factorization", "attribution", ["source"],
      r"Elimination proves this:",
      r"The paper cites this factorization from earlier work without proof; elimination proves it:",
      "the paper proves the factorization itself (Lemma lower-lower, 'We give its elementary "
      "elimination proof'); it does not cite it without proof", "1362-1392",
      ["without proof", "earlier work", "own proof", "proves the factorization", "proves it itself"]),
    E("FAI-S1", "fast-address-interchange", "constant", ["source"],
      r"computes $s$ as $Wm$ minus a positive constant",
      r"computes $s$ as $Wm/2$ minus a positive constant",
      "s_b = W_b m - (N - 2L_b): the deficit is tiny (eta_b = 339/22587335000000), so s/W is just "
      "below m, not below m/2", "1003-1010, 1319-1323", ["Wm/2", "m/2"]),
    E("FAI-S2", "fast-address-interchange", "hypothesis", ["source"],
      r"each combining $D$ with the earlier field $H$ in linear time",
      r"each combining $D$ with $H$, which lies after $D$ on the tape, in linear time",
      "the matrix-shear lemma requires every H field before every D field; that order is what makes "
      "the D updates linear-cost. (Reworded after the independent review: 'the later field H' could "
      "be read in time)", "1400-1401", ["after $D$ on the tape", "after D on the tape", "before every",
                                        "H before D", "H fields before", "precede"]),
    E("LNR-S1", "linear-network-roles", "conclusion", ["source"],
      r"The intermediate frames cancel and the route applies only its endpoint operator.",
      r"The intermediate frames cancel and the route applies only its endpoint operator, so the paper "
      r"fixes every intermediate frame to the identity.",
      "the paper chooses the intermediate frames so that every edge change is inexpensive (implemented "
      "by smaller calls); they are not the identity. (Replaces a first seed, 'scratch contents start "
      "at zero', rejected by the independent review because the page's cyclic row distribution puts "
      "array data on the scratch roles)", "149-150, 822-823, 1143-1146",
      ["identity", "intermediate frames are chosen", "chosen to make", "inexpensive", "smaller calls"]),
    E("LNR-S2", "linear-network-roles", "generalisation", ["source"],
      r"Each fixed-tape procedure of the argument comes from one such network",
      r"Every fixed-tape procedure of the argument comes from one and the same network",
      "each procedure arises from its own network: the bit construction (F_2, neighbours meet in "
      "one element) and the complex construction (Z[i,1/2], even intersection) differ",
      "120-121, 685-703", ["same network", "its own network", "own network", "two constructions",
                           "complex construction", "different network"]),
    E("SIC-S1", "subset-intersection-construction", "conclusion", ["source"],
      r"injected back into $S$, adds",
      r"injected back into $S$ and never removed from $A_{ST}$ afterwards (no undo-copy step), adds",
      "the schedule undoes the copy (row 7) and restores every auxiliary wire to its starting value. "
      "(Reworded after the independent review: 'never cleared' could be read as 'never reset to zero', "
      "which is true)", "705-722, 744-750",
      ["never removed", "undo copy", "undo-copy", "undoes the copy", "restores the auxiliary",
       "restores every auxiliary"]),
    E("SIC-S2", "subset-intersection-construction", "attribution", ["source"],
      r"with $h=100$ (the letter",
      r"with $h=100$, as in Alon's construction that the paper specializes (the letter",
      "the paper specializes Alon's construction to the prime 2 'with the ground-set size changed "
      "to h=100': the value is the paper's, not Alon's", "872-874",
      ["Alon", "changed to h=100", "ground-set size", "changed the ground"]),
    E("RPS-S1", "recursive-power-saving", "constant", ["source"],
      r"Here $m$ is the factor by which the parameter grows",
      r"Here $m=2$ is the factor by which the parameter grows",
      "the growth factor is the fixed space's dimension m = h^3 = 1000000 (the example's m = 2 is "
      "only an illustration)", "655, 162-174", ["m=2", "m = 2", "10^6", "10^{6}", "1000000", "h^3"]),
    E("RPS-S2", "recursive-power-saving", "generalisation", ["source"],
      r"which the main theorem needs",
      r"which only the paper's later division algorithm needs, not its main multiplication theorem",
      "the power saving feeds the main multiplication theorem (Theorem thm:main) through the fast "
      "interchange and the numerical layers; division is a later consequence. (Replaces a first seed "
      "on the parameter's meaning, sent back by the independent review because the page's later "
      "chunk counting exposed it)", "57-66, 103-117, 168-180",
      ["division", "main theorem", "multiplication theorem", "main multiplication"]),
    E("TI-S1", "tape-interchange", "constant", ["source"],
      r"where $\mu$ is its number of rows", r"where $\mu$ is the number of central wires in one invocation",
      "the vertex matrices are fixed rational m x m matrices with m = h^3 = 1000000 (the fixed "
      "space's dimension); an invocation has c_b = 100 central wires. (A first draft said 'the "
      "number of wires'; dropped because the page's 'W wire streams' makes that detectable "
      "without the paper)", "655, 760, 1119",
      ["central wires", "m\\times m", "10^6", "1000000", "fixed space"]),
    E("TI-S2", "tape-interchange", "attribution", ["source"],
      r"exchanges the two banks and returns every auxiliary wire to its starting value",
      r"exchanges the two banks and, by a restoration device the paper introduces as new, "
      r"returns every auxiliary wire to its starting value",
      "the paper credits the cancellation of arbitrary scratch values to the restoration "
      "construction of Buhrman, Cleve, Koucky, Loff and Speelman (Section 3)", "709-711",
      ["Buhrman", "introduces as new", "transparent computation", "not new", "credits"]),
]


def main():
    out = pathlib.Path(sys.argv[1])
    (out / "pages").mkdir(parents=True, exist_ok=True)
    key = json.loads((A1 / "key.json").read_text())
    paper = pathlib.Path(key["paper"]["path"]).read_text()
    assert P.sha(paper) == key["paper"]["sha256"], "paper.tex differs from A1's"
    paper_norm = P.normalise(paper)
    for v in key["variants"]:
        shutil.copy(A1 / v["file"], out / v["file"])
    key["schema"] = f"{key['schema']}+a2"
    key["a2"] = {"source_only_errors": [s["id"] for s in SEEDS], "written": "2026-10-17"}
    problems = []
    clean = {v["page"]: (A1 / v["file"]).read_text() for v in key["variants"] if v["kind"] == "clean"}
    pages_meta = {p["page"]: p for p in key["pages"]}
    for page in sorted(clean):
        body0 = clean[page]
        errs = [s for s in SEEDS if s["page"] == page]
        sents = P.sentence_spans(body0)
        spans = []
        for e in errs:
            n = body0.count(e["find"])
            if n != 1:
                problems.append(f"{e['id']}: text to replace occurs {n} times")
                continue
            at = body0.index(e["find"])
            spans.append(next((sp for sp in sents if sp[0] <= at < sp[1]), None))
            if any(a <= at < b for a, b, *_ in P.cite_marks(body0)):
                problems.append(f"{e['id']}: inside a citation mark")
        if len(set(map(tuple, filter(None, spans)))) != len(errs):
            problems.append(f"{page}: two errors share a sentence")
        if problems:
            continue
        body = P.apply(body0, sorted(errs, key=lambda e: body0.index(e["find"])))
        code = "a2-" + hashlib.sha256(f"A2:{page}:source".encode()).hexdigest()[:6]
        view, clean_view = P.auditor_view(body), P.auditor_view(body0)
        for sym in pages_meta[page]["bound_symbols"]:
            if P.occurs(sym, clean_view) and not P.occurs(sym, view):
                problems.append(f"{code}: bound symbol {sym} no longer occurs")
        new = P.math_tokens(view) - P.math_tokens(clean_view)
        if new:
            problems.append(f"{code}: new symbols reach the auditor: {sorted(new)}")
        for _, _, _, q in P.cite_marks(body):
            if P.normalise(q) not in paper_norm:
                problems.append(f"{code}: quotation not in the paper: {q!r}")
        entry = {"code": code, "page": page, "variant": "S", "kind": "seeded", "file": f"pages/{code}.md",
                 "sha256": P.sha(body), "errors": [f"{e['id']}@{code}" for e in errs], "shams": [],
                 "support_marks": [{"claim": m["claim"], "quote": m["quote"]} for m in P.support_marks(body)]}
        (out / entry["file"]).write_text(body)
        key["variants"].append(entry)
        for e in errs:
            at, clean_at = body.index(e["replace"]), body0.index(e["find"])
            try:
                site = P.site_span(view, e.get("site") or P.visible(e["replace"]))
                site_clean = P.site_span(clean_view, e.get("site") or P.visible(e["find"]))
            except ValueError as err:
                problems.append(f"{e['id']}: {err}")
                continue
            key["errors"].append({
                "id": f"{e['id']}@{code}", "catalogue_id": e["id"], "code": code, "page": page,
                "variant": "S", "original": e["find"], "seeded": e["replace"],
                "location": {"offset": at, "line": body[:at].count("\n") + 1, "audit_site": site,
                             "audit_site_clean": site_clean, "linked": [],
                             "support_mark": P.mark_for(body, at), "support_mark_clean": P.mark_for(body0, clean_at)},
                "type": e["type"], "detectable_by": e["detectable_by"], "why": e["why"],
                "paper_lines": e["paper_lines"], "cues": e["cues"]})
    if problems:
        print("seeding failed:\n- " + "\n- ".join(problems), file=sys.stderr)
        sys.exit(1)
    (out / "key.json").write_text(json.dumps(key, indent=1, ensure_ascii=False) + "\n")
    print(f"{len(key['variants'])} variants, {len(key['errors'])} errors "
          f"({sum(e['detectable_by'] == ['source'] for e in key['errors'])} source-only), "
          f"{len(key['shams'])} shams -> {out}")


if __name__ == "__main__":
    main()

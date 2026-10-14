"""Judge-free markers for the mechanism checklist (`calibration/rubrics/nlogn-mechanism-checklist.md`).

usage: python3 -I markers.py NAME=ARGUMENT.json [NAME=ARGUMENT.json ...] > markers.json

For each plan, and each checklist sub-step that has a distinctive written form, report whether
its marker occurs in the plan's text (claims, methods, motivations, quotes). The text is
normalized first: whitespace, braces, `\\left`/`\\right` and thin spaces are removed, so `B^{k+1}`
and `B^{k + 1}` match alike. A marker can only suggest presence. It cannot see the role a
sub-step plays or whether it is stated correctly, so it is a cross-check on judges, not a score.
Sub-steps without a distinctive form (L1) have no marker and are reported as null.
"""
import json
import re
import sys

MARKERS = {
    "B1": [r"\\mathcalJ\\mathcalV\+\\mathcalR\\mathcalG|JV\+RG|independent(ly)?of(the)?(initial|scratch|auxiliary)"],
    "B2": [r"\|S\\capT\|(\\bmod|\\mod|\\pmod|mod)\(?2"],
    "B3": [r"\(\|S\\capT\|-1\)/2|\\frac\|S\\capT\|-12"],
    "B4": [r"\(-y,y\+x\)|\(x,y\+x\)"],
    "L1": None,
    "L2": [r"t_[A-Z]\\perpt_[A-Z]|\\langlet_[A-Z],t_[A-Z]\\rangle\^\\perp"],
    "L3": [r"W(_\\rm[bc]|_[bc])?m-2N\+2L"],
    "L4": [r"orthonormalbas[ie]s", r"alternating"],
    "L5": [r"100/539|\\frac100539"],
    "P1": [r"additiv", r"(mod|\\bmod|\\pmod)\(?4"],
    "P2": [r"aI\+bX_?v|diag\(1,-1\)H=X"],
    "P3": [r"\\otimesk"],
    "P4": [r"i\^27k|Z_a,k|27coordinates"],
    "R1": [r"L/N<1/2|2L<N|N-2L>0|2N-2L"],
    "R2": [r"339/22587335000000|\\frac33922587335000000"],
    "R3": [r"73/19906842167500|\\frac7319906842167500"],
    "R4": [r"20\\cdot2\^-50|\\logm<20"],
    "M1": [r"pivot"],
    "M2": [r"prime.{0,160}denominator|denominator.{0,160}prime"],
    "M3": [r"descending"],
    "M4": [r"H_i-D_j"],
    "S1": [r"128mB\^2|\\nu=11"],
    "S2": [r"A\(e\)(\\le|\\leq|≤)|sA\(e/m\)"],
    "S3": [r"B\^k\+1(\\le|\\leq|≤)B\^2d|B\^2d\^\\nu|8B\^2d"],
    "S4": [r"(\\epsilon|\\varepsilon)C_1<1|\\Delta\+3|p\+\\Delta"],
    "S5": [r"rowsum"],
}


def text(node):
    parts = [node.get("claim", ""), node.get("method") or ""]
    m = node.get("motivation") or {}
    parts.append(m.get("text", "") if isinstance(m, dict) else str(m))
    q = node.get("quote") or {}
    parts.append(q.get("quote", "") if isinstance(q, dict) else str(q))
    return " ".join(parts) + " " + " ".join(text(c) for c in node.get("children", []))


def normalize(s):
    s = re.sub(r"\\(left|right|big|Big|bigl|bigr|Bigl|Bigr)\b", "", s)
    s = re.sub(r"\\[,;:! ]", "", s)
    s = s.replace("\\mathrm", "\\rm").replace("{\\rm ", "\\rm")
    return re.sub(r"[\s{}]", "", s)


def main():
    out = {}
    for arg in sys.argv[1:]:
        name, path = arg.split("=", 1)
        a = json.load(open(path))
        t = normalize(text(a.get("root", a)))
        out[name] = {k: (None if pats is None else all(re.search(p, t, re.I) for p in pats))
                     for k, pats in MARKERS.items()}
    json.dump(out, sys.stdout, indent=1)
    print()


if __name__ == "__main__":
    main()

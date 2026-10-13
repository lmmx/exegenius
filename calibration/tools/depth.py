"""Where an argument anchors two central derivations of the n log n paper.

For the steps whose claim or method states the saving s<Wm (or tau = 1 - 2^-50) and the
steps that state the butterfly-layer recursion, collects the source lines of the quotes in
their subtrees and reports whether any lies in the section that derives the result
(section 3, lines 640-1346, for the saving; section 5, lines 1763-2758, for the layer
recursion), only elsewhere in the body, or only in the introduction (lines 1-431). Stating a
result is not enough: the introduction states nearly every result of the paper.

usage: python3 -I depth.py PAPER.tex RUN_NAME ARGUMENT.json [RUN_NAME ARGUMENT.json ...]
"""
import bisect
import json
import re
import sys

PATTERNS = {
    "saving": (r"s\s*/\s*W\s*<\s*m|s\s*<\s*W\s*m|s_\{?\\?r?m?\{?[bc]\}?\}?\s*<\s*W|2\^\{-50\}", (640, 1346)),
    "layer": (r"F\(e\)|F\(e/m\)|lambda'|d\^\{?\\lambda|C\^\{?\\otimes\s*e/m"
              r"|layer[^.]{0,80}recurs|recurs[^.]{0,80}layer|unroll", (1763, 2758)),
}


def main():
    paper, pairs = sys.argv[1], sys.argv[2:]
    numbers, starts, pieces, offset = [], [], [], 0
    for number, line in enumerate(open(paper).read().split("\n"), 1):
        piece = re.sub(r"\s+", " ", line).strip()
        if piece:
            numbers.append(number)
            starts.append(offset)
            pieces.append(piece)
            offset += len(piece) + 1
    text = " ".join(pieces)

    def line_of(quote):
        at = text.find(re.sub(r"\s+", " ", quote).strip())
        return None if at < 0 else numbers[bisect.bisect_right(starts, at) - 1]

    def quotes_below(node):
        out, stack = [], [node]
        while stack:
            n = stack.pop()
            stack.extend(n.get("children", []))
            for q in [(n.get("quote") or {}).get("quote"),
                      ((n.get("motivation") or {}).get("quote") or {}).get("quote")]:
                if q and line_of(q):
                    out.append(line_of(q))
        return out

    for run, path in zip(pairs[::2], pairs[1::2]):
        root = json.load(open(path))["root"]
        nodes, stack = [], [root]
        while stack:
            n = stack.pop()
            stack.extend(n.get("children", []))
            nodes.append(n)
        found = []
        for name, (pattern, (lo, hi)) in PATTERNS.items():
            lines = sorted({x for n in nodes
                            if re.search(pattern, n.get("claim", "") + " " + (n.get("method") or ""), re.I)
                            for x in quotes_below(n)})
            where = ("deriving section" if any(lo <= x <= hi for x in lines)
                     else "body" if any(x > 431 for x in lines) else "introduction only")
            found.append(f"{name}: {where}")
        print(f"{run}: " + "; ".join(found))


main()

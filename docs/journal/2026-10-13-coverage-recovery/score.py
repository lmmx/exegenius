"""Score planning answers against the draft reference set (unreviewed).

For each in-scope reference item, a plan "reaches" it when one of the plan's quotes starts on
a line inside the item's stated or derived line ranges. This counts where quotes land, against
an unreviewed draft: it is a proxy for whether the plan draws on the passages that develop
each result, not a judgement that the plan captures the derivation.

usage: python3 -I score.py REFERENCE.json PAPER.tex ANSWER.json [ANSWER.json ...]
Each ANSWER is an extract answer ({"concepts": [...]}) or an argument answer ({"root": ...}).
"""
import bisect
import json
import re
import sys


def lines_of(paper):
    """Normalised text (whitespace collapsed, lines joined by spaces) and line starts."""
    pieces, starts, offset = [], [], 0
    for number, line in enumerate(open(paper).read().split("\n"), 1):
        piece = re.sub(r"\s+", " ", line).strip()
        if piece:
            starts.append((offset, number))
            pieces.append(piece)
            offset += len(piece) + 1
    return " ".join(pieces), starts


def quotes(answer):
    if "concepts" in answer:
        return [e["quote"] for c in answer["concepts"] for e in c.get("evidence", [])]
    out, stack = [], [answer["root"]]
    while stack:
        node = stack.pop()
        if node.get("quote"):
            out.append(node["quote"]["quote"])
        m = node.get("motivation") or {}
        if m.get("quote"):
            out.append(m["quote"]["quote"])
        stack.extend(node.get("children", []))
    return out


def ranges(text):
    return [(int(a), int(b or a)) for a, b in re.findall(r"(\d+)(?:\s*[-–]\s*(\d+))?", text)]


def main():
    reference, paper, answers = sys.argv[1], sys.argv[2], sys.argv[3:]
    text, starts = lines_of(paper)
    offsets = [o for o, _ in starts]
    items = [i for i in json.load(open(reference))["items"] if i["tier"] != "out-of-scope"]
    for path in answers:
        lines = []
        for q in quotes(json.load(open(path))):
            at = text.find(re.sub(r"\s+", " ", q).strip())
            if at >= 0:
                lines.append(starts[bisect.bisect_right(offsets, at) - 1][1])
        hit = {"core": [], "supporting": []}
        missed = []
        for item in items:
            spans = ranges(item["where"].get("stated_lines", "") + " " + item["where"].get("derived_lines", ""))
            if any(a <= line <= b for line in lines for a, b in spans):
                hit[item["tier"]].append(item["id"])
            else:
                missed.append(item["id"])
        core = sum(i["tier"] == "core" for i in items)
        supporting = len(items) - core
        print(f"{path}: {len(lines)} quotes located; core reached {len(hit['core'])}/{core}, "
              f"supporting {len(hit['supporting'])}/{supporting}; missed: {', '.join(missed)}")


main()

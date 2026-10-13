"""Prepare blind content-judge inputs: the reference pack and one text file per plan.

usage: python3 -I prepare.py REFERENCE.json PAPER.tex OUT_DIR SEED NAME=ARGUMENT.json ...

Writes OUT_DIR/pack.md (each in-scope reference item: its draft statement and role, the
paper's text at the lines where it is stated, with line numbers, and the line ranges where it
is derived), OUT_DIR/plans/Pnn.md (each argument as an indented tree, with each quote's
line in the paper), and OUT_DIR/key.json (which plan is which run). Plans are numbered in an
order shuffled by SEED, so a judge given a plan file cannot tell which run or arm made it.
"""
import bisect
import json
import pathlib
import random
import re
import sys


def spans(text):
    return [(int(a), int(b or a)) for a, b in re.findall(r"(\d+)(?:\s*[-–]\s*(\d+))?", text or "")]


def main():
    reference, paper, out, seed = sys.argv[1:5]
    plans = [a.split("=", 1) for a in sys.argv[5:]]
    out = pathlib.Path(out)
    (out / "plans").mkdir(parents=True, exist_ok=True)
    raw = open(paper).read().split("\n")
    ref = json.load(open(reference))

    pack = ["# Reference pack for judging plans of `paper.tex`\n",
            "Each item is a result the main theorem's proof depends on, from a draft list that has "
            "not been reviewed: treat its statement as a guide and the paper's text as the "
            "authority. Line numbers are those of `paper.tex`.\n"]
    for item in ref["items"]:
        if item["tier"] == "out-of-scope":
            continue
        where = item["where"]
        pack.append(f"\n## {item['id']} ({item['tier']}): {item['name']}\n")
        pack.append(f"**Draft statement.** {item['statement']}\n")
        pack.append(f"**Role.** {item['role']}\n")
        pack.append(f"**Derived at lines:** {where.get('derived_lines', '')}\n")
        pack.append("**The paper where it is stated:**\n")
        for a, b in sorted(set(spans(where.get("stated_lines")))):
            if b - a > 80:
                b = a + 80
            pack.append("```\n" + "\n".join(f"{n:5d}  {raw[n - 1]}" for n in range(a, b + 1)) + "\n```\n")
    (out / "pack.md").write_text("\n".join(pack))

    numbers, starts, pieces, offset = [], [], [], 0
    for n, line in enumerate(raw, 1):
        piece = re.sub(r"\s+", " ", line).strip()
        if piece:
            numbers.append(n)
            starts.append(offset)
            pieces.append(piece)
            offset += len(piece) + 1
    text = " ".join(pieces)

    def line_of(q):
        at = text.find(re.sub(r"\s+", " ", q).strip())
        return None if at < 0 else numbers[bisect.bisect_right(starts, at) - 1]

    def render(node, depth, out_lines):
        pad = "  " * depth
        head = f"{pad}- **{node['id']}** ({node['intention']}"
        head += f", page `{node['page']}`" if node.get("page") else ""
        head += ", reported out of order" if node.get("deferred") else ""
        out_lines.append(head + f"): {node['claim']}")
        if node.get("method"):
            out_lines.append(f"{pad}  - method: {node['method']}")
        m = node.get("motivation")
        if m:
            out_lines.append(f"{pad}  - motivation ({m.get('source')}): {m.get('text')}")
        if node.get("from"):
            out_lines.append(f"{pad}  - relies on: {', '.join(node['from'])}")
        if node.get("quote"):
            q = node["quote"]["quote"]
            out_lines.append(f"{pad}  - quotes line {line_of(q)}: \"{q}\"")
        for child in node.get("children", []):
            render(child, depth + 1, out_lines)

    order = list(range(len(plans)))
    random.Random(int(seed)).shuffle(order)
    key = {}
    for number, i in enumerate(order, 1):
        name, path = plans[i]
        pid = f"P{number:02d}"
        key[pid] = name
        lines = [f"# Plan {pid}\n", "The plan of the argument for the main theorem, as a tree of steps.\n"]
        render(json.load(open(path))["root"], 0, lines)
        (out / "plans" / f"{pid}.md").write_text("\n".join(lines) + "\n")
    (out / "key.json").write_text(json.dumps(key, indent=1))
    print(f"{len(plans)} plans; pack {len((out / 'pack.md').read_text())} chars")


main()

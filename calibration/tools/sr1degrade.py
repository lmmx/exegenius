"""SR1's two copies (plans/SR1.md): intact, and degraded by deleting the paragraphs that carry the goals' key points.

usage: python3 -I sr1degrade.py BASE_PROJECT OUT_DIR RECORD.json

BASE_PROJECT is the wiki project after its goals tasks are accepted. OUT_DIR/intact is a copy of it;
OUT_DIR/degraded is a copy in which, for every key point of every goal, paragraphs on the goal's
listed pages are deleted: the paragraph whose content words overlap most with the key point's, and
every other paragraph with overlap at least 0.35 (overlap = shared content words / the key point's
content words; headings are never deleted). The rule is mechanical so that the selection is blind
to any reader outcome. RECORD.json lists each deletion (page, paragraph index, text, key points)
and the sha256 of every page in both copies.
"""
import collections
import hashlib
import json
import pathlib
import re
import shutil
import sys

THRESHOLD = 0.35
STOP = set(("the a an of to and or in on for is are be by with that this as it its at from which each one two their "
            "than into then so not only but if when we can any all such these those there more less same also both "
            "use used make makes made over per under what how why").split())


def words(s):
    return {w for w in re.findall(r"[a-z][a-z0-9]+", s.lower()) if w not in STOP and len(w) > 2}


def blocks(text):
    return re.split(r"(\n\s*\n)", text)  # paragraphs at even indices, separators at odd


def paras(text):
    b = blocks(text)
    return [(i // 2, b[i]) for i in range(0, len(b), 2) if b[i].strip() and not b[i].lstrip().startswith("#")]


def main():
    base, out, record = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2]), pathlib.Path(sys.argv[3])
    sim = json.loads((base / "exegenius/simulation.json").read_text())
    assert sim["goals"] and not sim.get("runs"), "goals must be accepted and no reader run made"
    for name in ("intact", "degraded"):
        if (out / name).exists():
            shutil.rmtree(out / name)
        shutil.copytree(base, out / name, symlinks=True)
    pages_dir = out / "degraded/exegenius/pages"
    text = {p.stem: p.read_text() for p in pages_dir.glob("*.md")}
    dele = collections.defaultdict(dict)
    for gi, g in enumerate(sim["goals"]):
        for ki, k in enumerate(g["key_points"]):
            kw = words(k["point"])
            sc = sorted(((len(kw & words(p)) / max(1, len(kw)), page, i) for page in g["pages"]
                         for i, p in paras(text[page])), key=lambda x: (-x[0], x[1], x[2]))
            for ov, page, i in [sc[0]] + [x for x in sc[1:] if x[0] >= THRESHOLD]:
                dele[page].setdefault(i, {"overlap": [], "key_points": []})
                dele[page][i]["key_points"].append(f"G{gi}K{ki}")
                dele[page][i]["overlap"].append(round(ov, 2))
    rows = []
    for page, idx in sorted(dele.items()):
        b = blocks(text[page])
        for i in sorted(idx):
            rows.append({"page": page, "paragraph": i, "text": b[2 * i], **idx[i]})
            b[2 * i] = ""
        new = re.sub(r"\n{3,}", "\n\n", "".join(b)).strip() + "\n"
        (pages_dir / f"{page}.md").write_text(new)
    sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
    record.write_text(json.dumps({
        "rule": f"per key point: the top-overlap paragraph on the goal's pages, plus any with overlap >= {THRESHOLD}",
        "goals": len(sim["goals"]), "key_points": sum(len(g["key_points"]) for g in sim["goals"]),
        "deletions": rows,
        "sha256": {n: {p.stem: sha(p) for p in sorted((out / n / "exegenius/pages").glob("*.md"))} for n in ("intact", "degraded")},
    }, indent=1, ensure_ascii=False) + "\n")
    print(f"{len(rows)} paragraphs deleted on {len(dele)} pages -> {out}/degraded; record {record}")


if __name__ == "__main__":
    main()

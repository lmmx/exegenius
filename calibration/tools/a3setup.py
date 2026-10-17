"""Make A3's project copies: arm R, A2's SI audit plus one paragraph on what counts as an error (plans/A3.md).

usage: python3 -I a3setup.py WIKI_REPO OUT_DIR RUN

Copies are made exactly as A2's (`a2setup.py`, on A2's key: the 30 variants of
docs/journal/2026-10-17-a2-design/). Each task is A2's SI task with REFINE inserted after the
SOURCE paragraph, and nothing else. The script checks that each task equals the rendered task
plus exactly FIDELITY, SOURCE and REFINE, with SI's widened restriction, so R differs from A2's SI
only by REFINE. Descriptions are `A3R<run> <task id>`; it writes OUT_DIR/prompts.tsv and
OUT_DIR/binary.txt.
"""
import hashlib
import importlib.util
import json
import pathlib
import shutil
import sys


def load(name):
    spec = importlib.util.spec_from_file_location(name, pathlib.Path(__file__).with_name(f"{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


A2S = load("a2setup")
A1S = A2S.A1S

REFINE = (
    "What counts as an error: report a claim as wrong only if, as this page states it, it is false; "
    "inconsistent with the page; stated more strongly or more widely than what supports it; missing a "
    "hypothesis without which it is false; given a wrong value, sign, exponent, constant, quantifier or "
    "credit; or presented as following from something it does not follow from. Do not report a correct "
    "statement as wrong because it is informal, abbreviated or simplified, uses a word in the paper's own "
    "sense, or leaves a technical precondition to another page or to the paper. If the paper's own "
    "summary of the same step states it the same way, it is not an error.\n\n")


def transform(task):
    out = A2S.transform(task, "SI")
    anchor = A2S.SOURCE + A2S.ANCHOR
    assert out.count(anchor) == 1, "SI text not found"
    return out.replace(anchor, A2S.SOURCE + REFINE + A2S.ANCHOR)


def check(rendered, task):
    expect = rendered.replace(A2S.ANCHOR, A2S.FIDELITY + A2S.SOURCE + REFINE + A2S.ANCHOR)
    expect = expect.replace(A2S.RESTRICT, A2S.RESTRICT_SI)
    return task == expect


def main():
    wiki, out, run = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2]), sys.argv[3]
    key = json.loads((A2S.DESIGN / "key.json").read_text())
    pages = {p["page"]: p for p in key["pages"]}
    exe = shutil.which("exegenius")
    real = pathlib.Path(exe).resolve()
    out.mkdir(parents=True, exist_ok=True)
    (out / "binary.txt").write_text(f"{exe} -> {real}\nsha256 {hashlib.sha256(real.read_bytes()).hexdigest()}\n"
                                    f"{A1S.sh(['exegenius', '--version'], out).strip()}\n")
    rows, problems = [], []
    for v in key["variants"]:
        pg = pages[v["page"]]
        src = A2S.DESIGN / v["file"]
        assert hashlib.sha256(src.read_bytes()).hexdigest() == v["sha256"], v["code"]
        root = out / f"A3R{run}" / v["code"] / "audit"
        if root.exists():
            shutil.rmtree(root)
        shutil.copytree(A1S.base(wiki, pg["commit"], out / ".base"), root)
        if hashlib.sha256((root / "sources/paper.tex").read_bytes()).hexdigest() != key["paper"]["sha256"]:
            problems.append(f"{root}: sources/paper.tex differs from the key's paper")
        shutil.copy(src, root / "exegenius/pages" / f"{v['page']}.md")
        printed = A1S.audit_copy(root, v["page"])
        pending = A1S.state(root)["pending"]
        assert len(pending) == 1 and pending[0]["kind"] == "audit" and pending[0]["page"] == v["page"], (root, pending)
        tid = pending[0]["id"]
        path = root / "exegenius/tasks" / f"{tid}.md"
        rendered = path.read_text()
        task = transform(rendered)
        path.write_text(task)
        if not check(rendered, task):
            problems.append(f"{root}: arm text check failed")
        rows.append((f"A3R{run} {tid}", A2S.MODEL, A2S.EFFORT, str(root), A1S.prompt_of(printed)))
    with open(out / "prompts.tsv", "w") as f:
        for r in rows:
            f.write("\t".join(r) + "\n")
    print(f"{len(rows)} copies (R); problems: {problems or 'none'}")
    if problems:
        sys.exit(1)


if __name__ == "__main__":
    main()

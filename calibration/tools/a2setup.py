"""Make A2's project copies: audit tasks for arms B, I and SI on A2's key (plans/A2.md).

usage: python3 -I a2setup.py WIKI_REPO OUT_DIR RUN [ARM ...]

Copies are made exactly as A1's (a1setup.py: the wiki at the page's commit, the variant page in
place, the state edited so that `exegenius next` issues one audit of that page). Then each arm's
task file is transformed, and nothing else:
- B (baseline): the task as rendered, unchanged. Only A2's six new variants: A1 stage 1's AS
  answers serve as B on A1's 24 variants (same binary and configuration).
- I (instruction): FIDELITY inserted before the paragraph on copying quotes. All 30 variants.
- SI (source and instruction): as I, with SOURCE inserted after FIDELITY, and the opening
  restriction widened to let the auditor read `sources/paper.tex`, which every copy holds.

All arms run Sonnet `high`. The script checks: one pending audit per copy; each transformed
task equals the rendered task plus exactly the arm's insertions (so I differs from B only by
FIDELITY, and SI from I only by SOURCE and the widened restriction); the paper in every copy is
the paper the key was checked against; and each rendered task is identical across arms apart
from its root. It writes OUT_DIR/prompts.tsv (description, model, effort, root, prompt) and
OUT_DIR/binary.txt.
"""
import hashlib
import importlib.util
import json
import pathlib
import shutil
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
DESIGN = ROOT / "docs/journal/2026-10-17-a2-design"
MODEL, EFFORT = "sonnet", "high"

RESTRICT = "Use only this task file: do not read the sources, other pages or any other file."
RESTRICT_SI = ("Use only this task file and the paper at `sources/paper.tex`: do not read other "
               "pages or any other file.")
ANCHOR = "Copy each quote from the page exactly"

FIDELITY = (
    "Fidelity: also check that the page's mathematics is right, and report what is wrong even "
    "where this reader could follow it. For each claim that states a definition, a hypothesis, a "
    "step of a proof, a value or a conclusion, work out what it assumes and what it concludes, and "
    "check it against everything this task file gives: the page's definitions, its other claims "
    "and its quotations. Report as a `step` any claim that is false, inconsistent with another "
    "statement on the page, stated more strongly or more widely than what supports it, or missing "
    "a hypothesis it needs. Quote the claim, and say in the note what is wrong and what the "
    "correct statement is.\n\n")

SOURCE = (
    "The source: the paper this page explains is at `sources/paper.tex` (LaTeX). For each such "
    "claim, also find the passage of the paper that states, defines or proves it (search the file "
    "for the claim's terms and symbols, then read the passage around each match) and compare: "
    "every hypothesis present, the same scope and quantifiers, the same constants, signs, "
    "exponents and inequality directions, a conclusion no stronger than the paper's, and the "
    "right result, lemma or author credited. Report any mismatch as a `step` in the same way, and "
    "give the paper's line number in the note.\n\n")


def load(name):
    spec = importlib.util.spec_from_file_location(name, pathlib.Path(__file__).with_name(f"{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


A1S = load("a1setup")


def transform(task, arm):
    if arm == "B":
        return task
    assert task.count(ANCHOR) == 1 and task.count(RESTRICT) == 1, "task text changed"
    ins = FIDELITY + (SOURCE if arm == "SI" else "")
    out = task.replace(ANCHOR, ins + ANCHOR)
    if arm == "SI":
        out = out.replace(RESTRICT, RESTRICT_SI)
    return out


def check(rendered, task, arm):
    """The transformed task equals the rendered one plus exactly this arm's insertions."""
    if arm == "B":
        return task == rendered
    expect = rendered.replace(ANCHOR, FIDELITY + (SOURCE if arm == "SI" else "") + ANCHOR)
    if arm == "SI":
        expect = expect.replace(RESTRICT, RESTRICT_SI)
    return task == expect


def main():
    wiki, out, run = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2]), sys.argv[3]
    arms = sys.argv[4:] or ["B", "I", "SI"]
    key = json.loads((DESIGN / "key.json").read_text())
    pages = {p["page"]: p for p in key["pages"]}
    exe = shutil.which("exegenius")
    real = pathlib.Path(exe).resolve()
    out.mkdir(parents=True, exist_ok=True)
    (out / "binary.txt").write_text(f"{exe} -> {real}\nsha256 {hashlib.sha256(real.read_bytes()).hexdigest()}\n"
                                    f"{A1S.sh(['exegenius', '--version'], out).strip()}\n")
    rows, rendered_by, problems = [], {}, []
    for arm in arms:
        for v in key["variants"]:
            if arm == "B" and not v["code"].startswith("a2-"):
                continue
            pg = pages[v["page"]]
            src = DESIGN / v["file"]
            assert hashlib.sha256(src.read_bytes()).hexdigest() == v["sha256"], v["code"]
            root = out / f"A2{arm}{run}" / v["code"] / "audit"
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
            task = transform(rendered, arm)
            path.write_text(task)
            if not check(rendered, task, arm):
                problems.append(f"{root}: arm text check failed")
            rendered_by.setdefault(v["code"], {})[arm] = rendered.replace(str(root), "ROOT")
            rows.append((f"A2{arm}{run} {tid}", MODEL, EFFORT, str(root), A1S.prompt_of(printed)))
    differ = [c for c, by in rendered_by.items() if len(set(by.values())) > 1]
    if differ:
        problems.append(f"rendered tasks differ across arms apart from the root: {differ}")
    with open(out / "prompts.tsv", "a") as f:
        for r in rows:
            f.write("\t".join(r) + "\n")
    print(f"{len(rows)} copies ({', '.join(arms)}); problems: {problems or 'none'}")
    print(f"binary: {(out / 'binary.txt').read_text().strip()}")
    if problems:
        sys.exit(1)


if __name__ == "__main__":
    main()

"""Make A1's project copies: one per configuration, run, seeded variant and task kind (plans/A1.md, "Setup").

usage: python3 -I a1setup.py WIKI_REPO OUT_DIR RUN [CFG ...]

For each configuration (AH, AS, AO by default) and each of the key's 24 variants, it builds:
- an audit copy: the wiki at the page's commit, without `exegenius/tasks/`, with the seeded page in
  place. The state is edited so that `exegenius next` issues exactly one audit, of the target page;
- a support copy (20 variants; the four with no checked mark have none). The target's support
  judgements and checks are dropped, so `next` issues exactly one support task, of the target page.

No model is called. Every copy's task is rendered by the `exegenius` on PATH, whose path and
sha256 are written to OUT_DIR/binary.txt. OUT_DIR/prompts.tsv gets one row per task:
description, model, effort, root and the prompt `next` printed, word for word. The script checks:
the seeded page's sha256 against the key, one pending task of the right kind and page per copy,
and (when given more than one configuration) that each task file is identical across
configurations apart from its root path.
"""
import hashlib
import io
import json
import pathlib
import shutil
import subprocess
import sys
import tarfile

ROOT = pathlib.Path(__file__).resolve().parents[2]
DESIGN = ROOT / "docs/journal/2026-10-14-a1-audit-design"
CONFIGS = {"AH": ("haiku", "high"), "AS": ("sonnet", "high"), "AO": ("opus", "xhigh")}


def sh(args, cwd):
    out = subprocess.run(args, cwd=cwd, capture_output=True, text=True)
    if out.returncode != 0:
        sys.exit(f"{' '.join(args)} in {cwd}: {out.stderr.strip()}")
    return out.stdout


def base(wiki, commit, cache):
    """The wiki at COMMIT (exegenius.toml, sources/paper.tex, exegenius/) without exegenius/tasks."""
    d = cache / commit
    if d.exists():
        return d
    tar = subprocess.run(["git", "-C", str(wiki), "archive", commit, "exegenius.toml", "sources/paper.tex", "exegenius"],
                         capture_output=True, check=True).stdout
    d.mkdir(parents=True)
    tarfile.open(fileobj=io.BytesIO(tar)).extractall(d)
    shutil.rmtree(d / "exegenius/tasks", ignore_errors=True)
    return d


def state(root):
    return json.loads((root / "exegenius/state.json").read_text())


def save(root, s):
    (root / "exegenius/state.json").write_text(json.dumps(s, indent=1))


def prompt_of(out):
    lines = [l.strip()[len("prompt: "):] for l in out.splitlines() if l.strip().startswith("prompt: ")]
    if len(lines) != 1:
        sys.exit(f"expected one prompt, got {len(lines)}:\n{out}")
    return lines[0]


def audit_copy(root, page):
    s = state(root)
    next_id = s["next_id"]
    s["pending"], s["done"] = [], False
    save(root, s)
    sh(["exegenius", "next"], root)
    s = state(root)
    for t in s["pending"]:
        if t["kind"] == "audit" and t["page"] != page:
            s["audited"][t["page"]] = t["input"]
    s["pending"], s["next_id"] = [], next_id
    save(root, s)
    shutil.rmtree(root / "exegenius/tasks", ignore_errors=True)
    (root / "exegenius/log.jsonl").unlink(missing_ok=True)
    return sh(["exegenius", "next"], root)


def support_copy(root, page):
    s = state(root)
    s["pending"], s["done"] = [], True
    save(root, s)
    p = root / "exegenius/support.json"
    sp = json.loads(p.read_text())
    sp["judged"] = [j for j in sp.get("judged", []) if j["mark"]["page"] != page]
    sp["second"] = [j for j in sp.get("second", []) if j["mark"]["page"] != page]
    sp.get("checks", {}).pop(page, None)
    p.write_text(json.dumps(sp, indent=1))
    return sh(["exegenius", "next"], root)


def main():
    wiki, out, run = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2]), sys.argv[3]
    cfgs = sys.argv[4:] or list(CONFIGS)
    key = json.loads((DESIGN / "key.json").read_text())
    pages = {p["page"]: p for p in key["pages"]}
    exe = shutil.which("exegenius")
    real = pathlib.Path(exe).resolve()
    out.mkdir(parents=True, exist_ok=True)
    (out / "binary.txt").write_text(f"{exe} -> {real}\nsha256 {hashlib.sha256(real.read_bytes()).hexdigest()}\n"
                                    f"{sh(['exegenius', '--version'], out).strip()}\n")
    rows, files = [], {}
    for cfg in cfgs:
        model, effort = CONFIGS[cfg]
        for v in key["variants"]:
            pg = pages[v["page"]]
            src = DESIGN / v["file"]
            assert hashlib.sha256(src.read_bytes()).hexdigest() == v["sha256"], v["code"]
            kinds = ["audit"] + (["support"] if v.get("support_marks") else [])
            for kind in kinds:
                root = out / f"A1{cfg}{run}" / v["code"] / kind
                if root.exists():
                    shutil.rmtree(root)
                shutil.copytree(base(wiki, pg["commit"], out / ".base"), root)
                shutil.copy(src, root / "exegenius/pages" / f"{v['page']}.md")
                printed = (audit_copy if kind == "audit" else support_copy)(root, v["page"])
                pending = state(root)["pending"]
                assert len(pending) == 1 and pending[0]["kind"] == kind and pending[0]["page"] == v["page"], (root, pending)
                tid = pending[0]["id"]
                rows.append((f"A1{cfg}{run} {tid}", model, effort, str(root), prompt_of(printed)))
                task = (root / "exegenius/tasks" / f"{tid}.md").read_text().replace(str(root), "ROOT")
                files.setdefault((v["code"], kind), {})[cfg] = task
    with open(out / "prompts.tsv", "a") as f:
        for r in rows:
            f.write("\t".join(r) + "\n")
    differ = [k for k, by in files.items() if len(set(by.values())) > 1]
    print(f"{len(rows)} copies; task files differing across configurations apart from the root: {differ or 'none'}")
    print(f"binary: {(out / 'binary.txt').read_text().strip()}")


if __name__ == "__main__":
    main()

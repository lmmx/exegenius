"""Check that the installed exegenius writes the audit task A3 tested (plans/A3.md, arm R).

usage: python3 -I a3verify.py WIKI_REPO A3_RUN_DIR OUT_DIR

For each of A3's 30 page variants, it makes a fresh project copy exactly as `a3setup.py` did
(A2's key, the same base commit and page variant), has the installed `exegenius` render the
audit task with no transformation, and compares the task file with the one A3's run used
(A3_RUN_DIR/A3R1/<code>/audit/exegenius/tasks/<task>.md, written by the 0.6.0 binary and
`a3setup.py`'s insertions). The two may differ only in the project directory. It prints the
number identical and the first difference of any that is not, and exits 1 if any differs.
"""
import difflib
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


def main():
    wiki, a3, out = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2]), pathlib.Path(sys.argv[3])
    key = json.loads((A2S.DESIGN / "key.json").read_text())
    pages = {p["page"]: p for p in key["pages"]}
    out.mkdir(parents=True, exist_ok=True)
    print(A1S.sh(["exegenius", "--version"], out).strip())
    same, differ = 0, []
    for v in key["variants"]:
        pg = pages[v["page"]]
        root = out / v["code"] / "audit"
        if root.exists():
            shutil.rmtree(root)
        shutil.copytree(A1S.base(wiki, pg["commit"], out / ".base"), root)
        shutil.copy(A2S.DESIGN / v["file"], root / "exegenius/pages" / f"{v['page']}.md")
        A1S.audit_copy(root, v["page"])
        (task,) = A1S.state(root)["pending"]
        new = (root / "exegenius/tasks" / f"{task['id']}.md").read_text()
        old_root = a3 / "A3R1" / v["code"] / "audit"
        old = (old_root / "exegenius/tasks" / f"{task['id']}.md").read_text()
        if new.replace(str(root), "ROOT") == old.replace(str(old_root), "ROOT"):
            same += 1
        else:
            diff = difflib.unified_diff(old.replace(str(old_root), "ROOT").splitlines(),
                                        new.replace(str(root), "ROOT").splitlines(), "A3", "now", lineterm="", n=0)
            differ.append((v["code"], "\n".join(list(diff)[:12])))
    print(f"{same} of {len(key['variants'])} audit tasks identical to A3's arm R (project directory aside)")
    for code, d in differ:
        print(f"--- {code}\n{d}")
    sys.exit(1 if differ else 0)


if __name__ == "__main__":
    main()

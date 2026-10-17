"""Render the capability landscape (issue #16) from calibration/landscape.json.

usage: python3 -I landscape.py [--check]

Writes calibration/landscape.md: one row per pipeline task, one column per model and effort,
each cell the configuration's result against that task's standard, with numbered notes and
links to the experiments behind it. Every evidence id must resolve to a file or directory
under calibration/ (or the journal), and every status to the legend. With --check, it exits 1
if landscape.md differs from what the data gives, so the page cannot drift from its data.
"""
import json
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parents[1]
DATA = HERE / "landscape.json"
PAGE = HERE / "landscape.md"


def render(d):
    configs = d["configs"]
    statuses = d["statuses"]
    for e, path in d["evidence"].items():
        assert (HERE / path).exists(), f"evidence {e}: {path} not found"
    out = [f"# {d['title']}", "",
           f"**As of {d['as_of']}.** Generated from `landscape.json` by `tools/landscape.py`; edit the data, not "
           "this page. It is the first deliverable of issue #16, and presents existing evidence only: no "
           "experiment was run for it.", "", "**Scope:**"]
    out += [f"- {s}" for s in d["scope"]]
    out += ["", "**Legend:** " + "; ".join(f"{sym} {text}" for sym, text in statuses.values()) +
            "; ★ in production; blank: not tested. The number after a symbol is its note below the table.", ""]
    out.append("| Task | " + " | ".join(c["label"] for c in configs) + " |")
    out.append("|---|" + "---|" * len(configs))
    notes = []
    for t in d["tasks"]:
        row = [f"**{t['label']}**"]
        for c in configs:
            cell = t["cells"].get(c["id"])
            if cell is None:
                row.append("")
                continue
            assert cell["status"] in statuses, f"{t['id']} {c['id']}: unknown status {cell['status']}"
            for e in cell["evidence"]:
                assert e in d["evidence"], f"{t['id']} {c['id']}: unknown evidence {e}"
            notes.append((t, c, cell))
            row.append(f"{statuses[cell['status']][0]}{'★' if cell.get('production') else ''} {len(notes)}")
        out.append("| " + " | ".join(row) + " |")
    out += ["", "## Notes", ""]
    task = None
    for n, (t, c, cell) in enumerate(notes, 1):
        if t is not task:
            task = t
            out += ["", f"**{t['label']}.** Standard: {t['standard']}.", ""]
        links = ", ".join(f"[{e}]({d['evidence'][e]})" for e in cell["evidence"])
        prod = " In production." if cell.get("production") else ""
        out.append(f"{n}. {c['label']}, {statuses[cell['status']][1]}: {cell['note']}.{prod} ({links})")
    out += ["", "## Not yet in this landscape (issue #16)", ""]
    out += [f"- {s}" for s in d["not_yet"]]
    return "\n".join(out) + "\n"


def main():
    page = render(json.loads(DATA.read_text()))
    if "--check" in sys.argv:
        if not PAGE.exists() or PAGE.read_text() != page:
            sys.exit("landscape.md is out of date: run python3 -I calibration/tools/landscape.py")
        print("landscape.md matches landscape.json")
        return
    PAGE.write_text(page)
    print(f"wrote {PAGE}")


if __name__ == "__main__":
    main()

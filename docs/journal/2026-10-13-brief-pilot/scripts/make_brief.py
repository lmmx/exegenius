"""Insert a brief, built mechanically from argument.json, into a write task.

usage: make_brief.py <project dir> <page id> <task file>
"""
import json
import re
import sys

root, page, task_path = sys.argv[1:4]
arg = json.load(open(f"{root}/exegenius/argument.json"))
graph = json.load(open(f"{root}/exegenius/graph.json"))
paper = open(f"{root}/sources/paper.tex").read()
task = open(task_path).read()

links = dict(
    (m.group(2), m.group(0))
    for m in re.finditer(r"\[([^\]]+)\]\(([a-z0-9-]+)\.md\)", task)
)
prereq_section = task.split("## Prerequisite pages", 1)[1].split("\n## ", 1)[0]
prereqs = set(re.findall(r"\]\(([a-z0-9-]+)\.md\)", prereq_section))
concepts = {c["id"]: c for c in graph["concepts"]} if isinstance(graph.get("concepts"), list) else graph["concepts"]


def walk(n):
    yield n
    for c in n.get("children", []):
        yield from walk(c)


node = next(n for n in walk(arg["root"]) if n.get("page") == page and n["intention"] == "show")


def paragraph(quote):
    i = paper.find(quote)
    if i < 0:
        return None
    start = paper.rfind("\n\n", 0, i)
    end = paper.find("\n\n", i)
    return paper[start + 2 if start >= 0 else 0 : end if end >= 0 else len(paper)].strip()


def evidence(n, indent):
    q = n.get("quote")
    if not q:
        return ""
    para = paragraph(q["quote"])
    out = f'{indent}Source: "{q["quote"]}" ({q["source"]})\n'
    if para:
        body = "\n".join(f"{indent}> {line}" if line else f"{indent}>" for line in para.splitlines())
        out += f"{indent}The paragraph around it:\n\n{body}\n\n"
    return out


owned = [c for c in node.get("children", []) if not c.get("page")]
elsewhere = [c for c in node.get("children", []) if c.get("page")]

out = ["## Brief: what this page must establish\n"]
out.append(
    "The plan of the paper's proof gives this page the step above. This brief passes on what the "
    "plan says about it: which parts this page derives, which other pages derive, and where in "
    "the paper each part is.\n"
)
out.append("### Steps this page derives\n")
for c in owned:
    out.append(f"**{c['id']}.** {c['claim']}\n")
    if c.get("method"):
        out.append(f"How: {c['method']}\n")
    if c["intention"] == "infer":
        out.append(evidence(c, ""))
    for g in c.get("children", []):
        out.append(f"- **{g['id']}.** {g['claim']}\n")
        out.append(evidence(g, "  "))
out.append("### Steps derived on other pages\n")
for c in elsewhere:
    out.append(f"- **{c['id']}**, on {links.get(c['page'], c['page'])}: {c['claim']}\n")
out.append("")

deps = []
for n in [node] + owned + [g for c in owned for g in c.get("children", [])]:
    for f in n.get("from", []):
        if f not in deps and f not in prereqs and f != page:
            deps.append(f)
pages = [links[f] for f in deps if f in links]
known = [concepts[f]["name"] for f in deps if f not in links and f in concepts]
out.append("### What these steps rely on, besides the prerequisite pages\n")
if pages:
    out.append("Pages (link each where the derivation uses it; do not derive their results again):\n")
    out.extend(f"- {p}" for p in pages)
    out.append("")
if known:
    out.append("Concepts the reader already knows: " + "; ".join(known) + "\n")

out.append("### Acceptance criteria\n")
ids = ", ".join(c["id"] for c in owned)
out.append(
    f"- Derive each step this page derives ({ids}): show how each follows from the ones before, "
    "so the reader can check it. Stating its result, or citing the paper for it, is not enough.\n"
    "- For a step derived on another page, state its result in a sentence where this page uses "
    "it, and link that page. Do not derive it again.\n"
    "- The quotes and paragraphs above ground the derivation. Do not copy them onto the page; "
    "cite only a result or constant a reader may want to check.\n"
    "- The word limit below still holds. If the steps cannot all be derived within it, derive "
    "the ones the page's claim most depends on, and say so in your reply.\n"
)
brief = "\n".join(out) + "\n"
task = task.replace("## Prerequisite pages", brief + "## Prerequisite pages", 1)
open(task_path, "w").write(task)
print(f"{page}: {len(owned)} owned, {len(elsewhere)} elsewhere, {len(pages)} dependency pages, {len(known)} known; brief {len(brief.split())} words")

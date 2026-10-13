"""Mechanical checks of the brief's criteria on a written page.

usage: check.py <project dir> <page id>

For each step of the page's subtree in argument.json:
- owned steps (no page of their own): how many of the claim's math segments appear on the
  page, and whether the step's quote is cited on the page;
- steps owned by other pages: how many of their inferred sub-steps' math segments appear on
  the page (a sign of re-derivation).
"""
import json
import re
import sys

root, page = sys.argv[1:3]
arg = json.load(open(f"{root}/exegenius/argument.json"))
body = open(f"{root}/docs/{page}.md").read() if False else None
import glob

cands = glob.glob(f"{root}/exegenius/pages/{page}.md") + glob.glob(f"{root}/**/{page}.md", recursive=True)
body = open(cands[0]).read()


def norm(s):
    s = re.sub(r"\\(left|right|bigl|bigr|Bigl|Bigr|!|,|;|quad|qquad)", "", s)
    s = s.replace("\\le", "\\leq").replace("\\leqq", "\\leq")
    s = re.sub(r"\\rm\s*", "", s)
    s = re.sub(r"\\mathrm\{([^}]*)\}", r"\1", s)
    return re.sub(r"[\s{}]", "", s)


def maths(s):
    return re.findall(r"\$\$(.+?)\$\$|\$(.+?)\$", s, re.S)


page_math = norm(" ".join(a or b for a, b in maths(body)))
cites = [norm(c) for c in re.findall(r"<cite[^>]*>(.*?)</cite>", body, re.S)]


def segs(claim):
    # Only segments long enough to be a relation, not a lone letter.
    return [norm(a or b) for a, b in maths(claim) if len(norm(a or b)) >= 4]


def walk(n):
    yield n
    for c in n.get("children", []):
        yield from walk(c)


node = next(n for n in walk(arg["root"]) if n.get("page") == page and n["intention"] == "show")
words = len(re.sub(r"<cite.*?</cite>|\$\$.+?\$\$|\$.+?\$", " ", body, flags=re.S).split())
print(f"{root.split('/')[-1]}: {words} words of prose (approx.), {len(cites)} citations")
for c in node.get("children", []):
    owned = not c.get("page")
    steps = [c] + c.get("children", [])
    if owned:
        for s in steps:
            ss = segs(s["claim"])
            hit = sum(1 for x in ss if x in page_math)
            q = s.get("quote", {}).get("quote") if s.get("quote") else None
            cited = bool(q) and any(norm(q)[:40] in x or x[:40] in norm(q) for x in cites if x)
            print(f"  owned {s['id']}: math {hit}/{len(ss)} on page; quote cited: {cited}")
    else:
        ss = [x for s in c.get("children", []) for x in segs(s["claim"])]
        hit = sum(1 for x in ss if x in page_math)
        print(f"  elsewhere {c['id']} ({c['page']}): sub-step math {hit}/{len(ss)} on page")

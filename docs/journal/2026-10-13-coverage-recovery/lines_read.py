"""Usage: python3 -I lines_read.py TRANSCRIPTS_DIR PAPER.tex

Lines of paper.tex a planner put in front of itself, per attempt (between submits),
from its transcript: Read offset/limit, `sed -n A,Bp`, `for r in A,B ...` loops over
`sed -n "${r}p"`, `head -N`/`cat | head -N`, `head -c N`. grep output (outline) is not counted."""
import json, pathlib, re, sys
root, paper = pathlib.Path(sys.argv[1]), sys.argv[2]
text = open(paper, 'rb').read()
total = text.count(b'\n')
def bytes_to_lines(n): return text[:n].count(b'\n')
for t in sorted(root.rglob("agent-*.jsonl"), key=lambda p: json.load(open(p.with_suffix(".meta.json"))).get("description","")):
    meta = json.load(open(t.with_suffix(".meta.json")))
    d = meta.get("description", "")
    if meta.get("agentType") != "exegenius-planner" or "background" in d: continue
    seen, attempts, cur = set(), [], set()
    for line in open(t):
        e = json.loads(line)
        if e.get("type") != "assistant": continue
        for b in (e.get("message") or {}).get("content") or []:
            if b.get("type") != "tool_use": continue
            a = b.get("input", {})
            got = set()
            if b["name"] == "Read" and a.get("file_path","").endswith("paper.tex"):
                o = int(a.get("offset") or 1); l = int(a.get("limit") or total)
                got = set(range(o, min(o + l, total + 1)))
            elif b["name"] == "Bash":
                c = a.get("command", "")
                if "paper.tex" in c:
                    for x, y in re.findall(r"sed -n '?\"?(\d+),(\d+)p", c):
                        got |= set(range(int(x), int(y) + 1))
                    m = re.search(r"for r in ([\d, ]+);", c)
                    if m and "${r}" in c:
                        for x, y in re.findall(r"(\d+),(\d+)", m.group(1)):
                            got |= set(range(int(x), int(y) + 1))
                    for n in re.findall(r"head -c (\d+)", c):
                        got |= set(range(1, bytes_to_lines(int(n)) + 1))
                    for n in re.findall(r"head -(\d+)\b", c):
                        got |= set(range(1, int(n) + 1))
                if "exegenius submit" in c:
                    seen |= got; got = set()
                    attempts.append(len(seen))
            seen |= got
    beyond = len([x for x in seen if x > 431])
    print(f"{d:18s} lines read before each submit: {attempts}; total {len(seen)} of {total}; past the introduction (line 432+): {beyond}")

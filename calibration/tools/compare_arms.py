"""List every material difference between two runs' setups, from git refs of the wiki repository.

usage: python3 -I compare_arms.py WIKI_REPO REF_A:RUN_A REF_B:RUN_B

Compares, and prints as identical or different:
- the source and template configuration (sha256 of every file under template/);
- the subagent definitions, skill and settings (sha256 of every file under .claude/);
- each run's exegenius.toml;
- each planning task file both runs have (exegenius/tasks/000N-*.md), with the run directory
  path normalised;
- the exegenius build each runner recorded (results/versions.txt);
- the runner protocol (EXPERIMENT.md), as a unified diff;
- the prompt each planning subagent received (its first message), path-normalised;
- each planning subagent's recorded agent type, models and effort, and the orchestrator's.
Differences are facts for the report; deciding which matter is the analyst's job.
"""
import difflib
import hashlib
import io
import json
import re
import subprocess
import sys
import tarfile


def git(repo, *args):
    r = subprocess.run(["git", "-C", repo, *args], capture_output=True)
    return r.stdout if r.returncode == 0 else None


def tree(repo, ref, path):
    out = git(repo, "ls-tree", "-r", ref, path)
    if not out:
        return {}
    files = {}
    for line in out.decode().splitlines():
        meta, name = line.split("\t", 1)
        files[name] = meta.split()[2]
    return files


def show(repo, ref, path):
    out = git(repo, "show", f"{ref}:{path}")
    return None if out is None else out.decode(errors="replace")


def norm(text, run):
    text = re.sub(rf"runs/{re.escape(run)}\b", "runs/RUN", text)
    return re.sub(r"\b[A-Z][0-9]+ (000\d-)", r"RUN \1", text)


def sha(text):
    return hashlib.sha256(text.encode()).hexdigest()[:16] if text is not None else None


def subagents(repo, ref, run):
    blob = git(repo, "archive", ref, "results/transcripts")
    found = {}
    if not blob:
        return found, []
    orch = []
    with tarfile.open(fileobj=io.BytesIO(blob)) as tar:
        members = {m.name: m for m in tar.getmembers() if m.isfile()}
        for name, m in members.items():
            if not name.endswith(".jsonl"):
                continue
            lines = tar.extractfile(m).read().decode().splitlines()
            if "/subagents/" not in name:
                eff = {json.loads(l).get("effort") for l in lines if '"effort"' in l}
                mods = {json.loads(l).get("message", {}).get("model") for l in lines if '"assistant"' in l}
                orch.append((sorted(x for x in eff if x), sorted(x for x in mods if x)))
                continue
            meta_name = name[: -len(".jsonl")] + ".meta.json"
            meta = json.loads(tar.extractfile(members[meta_name]).read()) if meta_name in members else {}
            desc = meta.get("description", "").split()
            if len(desc) < 2 or desc[0] != run:
                continue
            first = json.loads(lines[0]).get("message", {}).get("content", "")
            first = first if isinstance(first, str) else json.dumps(first)
            eff = sorted({json.loads(l).get("effort") for l in lines if '"effort"' in l} - {None})
            mods = sorted({json.loads(l).get("message", {}).get("model") for l in lines if '"type":"assistant"' in l} - {None})
            found[desc[1]] = {"prompt_sha": sha(norm(first, run)), "agent": meta.get("agentType"),
                              "models": mods, "effort": eff}
    return found, orch


def main():
    repo, a, b = sys.argv[1:4]
    (ra, run_a), (rb, run_b) = a.split(":"), b.split(":")
    rows = []

    def same(label, x, y):
        rows.append((label, "identical" if x == y else "DIFFERENT", "" if x == y else f"{x} | {y}"))

    for prefix in ("template", ".claude"):
        ta, tb = tree(repo, ra, prefix), tree(repo, rb, prefix)
        for name in sorted(set(ta) | set(tb)):
            same(name, ta.get(name), tb.get(name))
    same("run exegenius.toml", sha(show(repo, ra, f"runs/{run_a}/exegenius.toml")),
         sha(show(repo, rb, f"runs/{run_b}/exegenius.toml")))
    for task in ("0001-extract", "0002-background", "0003-argument"):
        label = f"task file {task}" + ("" if task == "0001-extract" else
                                         " (embeds the run's own earlier answers: differs by construction)")
        same(label, sha(norm(show(repo, ra, f"runs/{run_a}/exegenius/tasks/{task}.md") or "", run_a)),
             sha(norm(show(repo, rb, f"runs/{run_b}/exegenius/tasks/{task}.md") or "", run_b)))
    va, vb = show(repo, ra, "results/versions.txt"), show(repo, rb, "results/versions.txt")
    same("results/versions.txt", va and va.strip(), vb and vb.strip())
    sa, oa = subagents(repo, ra, run_a)
    sb, ob = subagents(repo, rb, run_b)
    for task in sorted(set(sa) | set(sb)):
        for k in ("prompt_sha", "agent", "models", "effort"):
            same(f"subagent {task} {k}", sa.get(task, {}).get(k), sb.get(task, {}).get(k))
    same("orchestrator (effort, models)", oa, ob)
    for label, status, detail in rows:
        print(f"{status:10s} {label}" + (f"  [{detail}]" if detail else ""))
    ea, eb = show(repo, ra, "EXPERIMENT.md") or "", show(repo, rb, "EXPERIMENT.md") or ""
    diff = list(difflib.unified_diff(ea.splitlines(), eb.splitlines(), "EXPERIMENT.md A", "EXPERIMENT.md B", n=0, lineterm=""))
    print(f"\nEXPERIMENT.md: {'identical' if not diff else f'{len(diff)} diff lines'}")
    print("\n".join(diff[:200]))


main()

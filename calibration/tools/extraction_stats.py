"""Judge-free measures of existing extractions of the n log n paper, and their relation to the
downstream argument plans judged on them (policy.md, row "Extraction").

usage: python3 -I calibration/tools/extraction_stats.py [--wiki PATH] [--repo PATH] [--out DIR]

Reads, without changing anything:
- the extraction answers (`exegenius/tasks/0001-extract.answer.json`) and run logs
  (`exegenius/log.jsonl`) of 34 planning projects on lmmx/openai-nlogn-wiki, at the evidence
  commits the registry names (E1 85611ce, E3 5ced29e, S1 7335096, E5 904a3c6, E6 834fb5a), with
  `git show` only;
- `paper.tex` from the same commits (one blob, 60ba998);
- the draft reference (docs/journal/2026-10-13-run-log-and-coverage/reference-draft/reference.json),
  the mechanism checklist's sub-step lines (calibration/rubrics/nlogn-mechanism-checklist.md) and
  its markers (calibration/tools/markers.py);
- the registry records (calibration/registry/experiments/*.json), the primary verdicts of the
  batches in calibration/registry/judging.json and the checklist verdicts of
  calibration/registry/checklist-judging.json.

Writes DIR/table.md (every table) and DIR/extractions.tsv (one row per extraction).

What the measures are. Every figure is where an extraction's quotes fall, or what its text
contains. A quote shows that a passage was cited, not that the concept graph captures it; a
reference item "reached" means a quote starts in the lines the unreviewed draft gives for it, not
that the extraction states it. Downstream figures are judge counts taken from the registry and
the verdicts; nothing here judges anything.
"""
import argparse
import bisect
import collections
import importlib.util
import itertools
import json
import math
import pathlib
import random
import re
import statistics
import subprocess

PAPER_COMMIT = "834fb5a"
PAPER_PATH = "template/sources/paper.tex"
INTRO_END = 431  # last line of section 1

# (extraction id, experiment, evidence commit, run dir, model, effort, agent)
EXTRACTIONS = (
    [(f"E1-P{k}", "E1", "85611ce", f"runs/P{k}", "Sonnet", "high", "planner") for k in range(1, 6)]
    + [(f"E1-G{k}", "E1", "85611ce", f"runs/G{k}", "Sonnet", "high", "general") for k in range(1, 6)]
    + [(f"E6-F{k}", "E6", "834fb5a", f"runs/E6F{k}", "Sonnet", "high", "general") for k in range(1, 9)]
    + [(f"E3-M{k}", "E3", "5ced29e", f"runs/M{k}", "Sonnet", "medium", "planner") for k in range(1, 6)]
    + [(f"S1-O{k}", "S1", "7335096", f"runs/O{k}", "Opus", "medium", "planner") for k in range(1, 3)]
    + [(f"E5-N{k}", "E5", "904a3c6", f"runs/N{k}", "Opus", "medium", "general") for k in range(1, 6)]
    + [(f"S1-Q{k}", "S1", "7335096", f"runs/Q{k}", "Opus", "high", "planner") for k in range(1, 3)]
    + [(f"S1-K{k}", "S1", "7335096", f"runs/K{k}", "Haiku", "high", "planner") for k in range(1, 3)]
)

# The batch each plan's primary figures are read from: the batch of its own experiment, or, for
# full-planning runs, the batch that judged two configurations together (J3: P with M; J6: G
# with N; J4: O, Q, K with anchors).
PRIMARY_BATCH = {"E1:P": "J3", "E1:G": "J6", "E3:M": "J3", "S1:O": "J4", "S1:Q": "J4", "S1:K": "J4",
                 "E5:N": "J6", "E4:B": "J5", "E4:V": "J5", "E6:B": "J7", "E6:V": "J7", "E7:W": "J8",
                 "E8:X": "J16", "E8b:X": "J18", "E9:Y": "J18", "S2:SX": "J9", "S2:OX": "J9",
                 "S2:SM": "J9", "S2:OM": "J9", "S3:OH": "J10", "S3:OX": "J10", "S4:OM": "J12",
                 "S5:OX": "J12"}


# ---------------------------------------------------------------- reading


def git(wiki, *args):
    return subprocess.run(["git", "-C", str(wiki), *args], check=True, capture_output=True,
                          text=True).stdout


def norm(s):
    s = s.replace("‘", "'").replace("’", "'").replace("“", '"').replace("”", '"')
    s = s.replace("–", "-").replace("—", "-")
    return re.sub(r"\s+", " ", s).strip()


class Paper:
    def __init__(self, text):
        pieces, self.starts, self.numbers, offset = [], [], [], 0
        lines = text.split("\n")
        self.n_lines = len(lines) - (1 if lines and lines[-1] == "" else 0)
        for number, line in enumerate(lines, 1):
            piece = norm(line)
            if piece:
                self.starts.append(offset)
                self.numbers.append(number)
                pieces.append(piece)
                offset += len(piece) + 1
        self.text = " ".join(pieces)

    def line_of(self, quote):
        q = norm(quote)
        at = self.text.find(q) if q else -1
        return None if at < 0 else self.numbers[bisect.bisect_right(self.starts, at) - 1]


def line_ranges(text):
    """Line ranges in a reference or rubric cell; numbers under 100 (such as 1/2) are not lines."""
    out = []
    for a, b in re.findall(r"(\d{3,})(?:\s*[-–]\s*(\d{3,}))?", text or ""):
        out.append((int(a), int(b or a)))
    return out


def load_markers(repo):
    spec = importlib.util.spec_from_file_location("markers", repo / "calibration/tools/markers.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def checklist_substeps(repo):
    rows = {}
    for line in open(repo / "calibration/rubrics/nlogn-mechanism-checklist.md"):
        m = re.match(r"\|\s*([BLPRMS]\d)\s*\|\s*([\w-]+)\s*\|.*\|\s*([\d–\-, ]+)\|\s*$", line)
        if m:
            rows[m.group(1)] = (m.group(2), line_ranges(m.group(3)))
    return rows


# ---------------------------------------------------------------- per extraction


def extract_events(log_text):
    events = [json.loads(x) for x in log_text.splitlines() if x.strip()]
    return [e for e in events if e.get("event") == "submit" and e.get("kind") == "extract"
            and e.get("coverage")]


def measure(paper, answer, cov, reference, substeps, markers, result_labels):
    concepts = answer["concepts"]
    # exegenius counts a quote given under two concepts once; so does this
    quotes = list(dict.fromkeys(e["quote"] for c in concepts for e in c.get("evidence", [])))
    lines = [paper.line_of(q) for q in quotes]
    located = sorted(x for x in lines if x)
    m = collections.OrderedDict()
    m["concepts"] = len(concepts)
    m["unknown"] = sum(not c.get("known") for c in concepts)
    m["gloss_chars"] = sum(len(c.get("gloss", "")) for c in concepts)
    m["quotes"] = cov["quotes"]
    m["located"] = cov["located"]
    m["located_here"] = len(located)
    m["first_tenth"] = cov["first_tenth"]
    m["body_quotes"] = sum(x > INTRO_END for x in located)
    m["body_share"] = round(m["body_quotes"] / len(located), 2) if located else 0
    m["last_line"] = cov["span"][1] if cov.get("span") else 0
    m["blocks"] = len({(x - 1) // 100 for x in located})
    secs = cov["sections"]
    top = [s for s in secs if s["level"] == 1 and s["title"] != "(front matter)"]
    developing = [s for s in top if s["statements"] or s["displays"]]
    m["top_quoted"] = sum(s["quotes"] > 0 for s in top)
    m["dev_quoted"] = sum(s["quotes"] > 0 for s in developing)
    m["dev_total"] = len(developing)
    body_sub = [s for s in secs if s["level"] >= 2 and not s["title"].startswith("§1.")]
    m["subsections_quoted"] = sum(s["quotes"] > 0 for s in body_sub)
    m["subsections_total"] = len(body_sub)
    m["results_quoted"] = cov["quoted_results"]
    m["results_total"] = cov["results"]
    unquoted = {u["what"].split("`")[1] for u in cov.get("unquoted_results", []) if "`" in u["what"]}
    reached = {"core": set(), "supporting": set()}
    derived = set()
    by_label, labelled = set(), set()
    for item in reference:
        spans = line_ranges(item["where"].get("stated_lines", "")) + line_ranges(item["where"].get("derived_lines", ""))
        dspans = line_ranges(item["where"].get("derived_lines", ""))
        if any(a <= x <= b for x in located for a, b in spans):
            reached[item["tier"]].add(item["id"])
        if any(a <= x <= b for x in located for a, b in dspans):
            derived.add(item["id"])
        labels = [lab for lab in re.findall(r"\b(?:lem|prop|eq|thm|cor|def):[\w-]+", item["where"].get("label", ""))
                  if lab in result_labels]
        if labels and item["tier"] == "core":
            labelled.add(item["id"])
            if any(lab not in unquoted for lab in labels):
                by_label.add(item["id"])
    core_ids = {i["id"] for i in reference if i["tier"] == "core"}
    m["core_lines"] = len(reached["core"])
    m["core_derived_lines"] = len(derived & core_ids)
    m["core_labels"] = len(by_label)
    m["core_labelled_total"] = len(labelled)
    m["core_either"] = len(reached["core"] | by_label)
    m["supporting_lines"] = len(reached["supporting"])
    m["checklist_lines"] = sum(any(a <= x <= b for x in located for a, b in spans)
                               for _, spans in substeps.values())
    text = " ".join(" ".join([c.get("name", ""), c.get("title") or "", c.get("gloss", ""),
                              " ".join(c.get("aliases", [])),
                              " ".join(e["quote"] for e in c.get("evidence", []))]) for c in concepts)
    t = markers.normalize(text)
    m["markers"] = sum(all(re.search(p, t, re.I) for p in pats)
                       for pats in markers.MARKERS.values() if pats)
    return m, reached["core"] | by_label, reached["core"]


def extraction_cost(repo, wiki, exp, commit, run):
    """Cache-read tokens and seconds of the accepted extraction's subagent(s)."""
    rec = repo / f"calibration/registry/experiments/{exp}.json"
    name = run.split("/")[-1]
    if exp != "E6":
        d = json.load(open(rec))
        for r in d["runs"]:
            if r["id"] == name:
                subs = [s for s in r.get("subagents", []) if s.get("task") == "0001-extract"]
                if subs:
                    return (sum(s["cache_read_tokens"]["value"] for s in subs),
                            sum(s["seconds"]["value"] for s in subs), "registry")
        return None, None, "unavailable"
    # E6's records cover the argument task only; read the extraction transcripts directly.
    base = "results/transcripts/e6/subagents"
    want = name.replace("E6", "") + " 0001-extract"
    for f in git(wiki, "ls-tree", "--name-only", f"{commit}:{base}").split():
        if not f.endswith(".meta.json"):
            continue
        meta = json.loads(git(wiki, "show", f"{commit}:{base}/{f}"))
        if meta.get("description") != want:
            continue
        calls, stamps = {}, []
        for line in git(wiki, "show", f"{commit}:{base}/{f[:-10]}.jsonl").splitlines():
            e = json.loads(line)
            if e.get("timestamp"):
                stamps.append(e["timestamp"])
            msg = e.get("message") or {}
            if e.get("type") == "assistant":
                calls[msg.get("id")] = (msg.get("usage") or {}).get("cache_read_input_tokens", 0)
        from datetime import datetime
        p = lambda s: datetime.fromisoformat(s.replace("Z", "+00:00"))
        secs = (p(max(stamps)) - p(min(stamps))).total_seconds() if stamps else None
        return sum(calls.values()), secs, "transcript"
    return None, None, "unavailable"


# ---------------------------------------------------------------- downstream


def extraction_of(exp, run):
    if exp in ("E1", "E3", "S1", "E5"):
        return f"{exp}-{run}"
    if exp == "E4":
        return "E1-P" + run[-1]
    m = re.match(r"^(?:E6|E7|E8|E9|E10|S\d|FB1)F(\d)", run)
    return f"E6-F{m.group(1)}" if m else None


# Primary batches judged after the registry was last built, read from their verdicts directly
# (E10's J21). Each is skipped once judging.json lists it, so a rebuilt registry takes over.
EXTRA_BATCHES = [{"id": "J21", "experiment": "E10", "arm": "Y",
                  "dir": "docs/journal/2026-10-14-e10-combined/judge", "key": "key.json",
                  "verdicts": "verdicts",
                  "reference": "docs/journal/2026-10-13-run-log-and-coverage/reference-draft/reference.json"}]
PRIMARY_BATCH["E10:Y"] = "J21"


def extra_batches(repo):
    listed = {b["id"] for b in json.load(open(repo / "calibration/registry/judging.json"))}
    return [b for b in EXTRA_BATCHES if b["id"] not in listed and (repo / b["dir"] / b["key"]).exists()]


def summarise_verdict(v, tiers):
    """The registry's counts (registry.py `verdict_summary`) that this script uses."""
    core = [i for i in v.get("items", []) if tiers.get(i["id"]) == "core"]
    out = {"core_faithful": sum(i.get("derivation") == "faithful" for i in core),
           "core_derived": sum(i.get("presence") == "derived" for i in core),
           "major_errors": sum(e.get("severity") == "major" for e in v.get("errors", []))}
    central = v.get("central")
    if central:
        out["central_count"] = sum(st.get("verdict") == "reconstructed" for st in central.get("steps", []))
    return out


def primary_plans(repo):
    """Every judged plan on one of the 34 extractions, with its primary-batch medians."""
    plans = []
    for b in extra_batches(repo):
        root = repo / b["dir"]
        tiers = {i["id"]: i["tier"] for i in json.load(open(repo / b["reference"]))["items"]}
        for pid, run in json.load(open(root / b["key"])).items():
            ext = extraction_of(b["experiment"], run)
            if not ext or not run.startswith(b["experiment"]):
                continue  # re-judged plans and anchors are read from their own batches
            files = sorted((root / b["verdicts"]).glob(f"{pid}.json")) + sorted((root / b["verdicts"]).glob(f"{pid}-*.json"))
            q = [summarise_verdict(json.load(open(x)), tiers) for x in files]
            if not q:
                continue
            med = lambda k: statistics.median(x[k] for x in q if x.get(k) is not None) \
                if any(x.get(k) is not None for x in q) else None
            plans.append({"run": run, "arm": f"{b['experiment']}:{b['arm']}", "extraction": ext,
                          "batch": b["id"], "judges": len(q), "faithful": med("core_faithful"),
                          "faithful_each": [x["core_faithful"] for x in q], "derived": med("core_derived"),
                          "central": med("central_count"), "major": med("major_errors")})
    for f in sorted((repo / "calibration/registry/experiments").glob("*.json")):
        d = json.load(open(f))
        for r in d["runs"]:
            ext = extraction_of(d["id"], r["id"])
            key = f"{d['id']}:{r['arm']}"
            if not ext or key not in PRIMARY_BATCH or r.get("excluded"):
                continue
            batch = PRIMARY_BATCH[key]
            q = [x for x in r.get("quality", []) if x["batch"] == batch]
            if not q:
                continue
            med = lambda k: statistics.median(x[k] for x in q if x.get(k) is not None) \
                if any(x.get(k) is not None for x in q) else None
            plans.append({"run": r["id"], "arm": key, "extraction": ext, "batch": batch,
                          "judges": len(q), "faithful": med("core_faithful"),
                          "faithful_each": [x["core_faithful"] for x in q],
                          "derived": med("core_derived"), "central": med("central_count"),
                          "major": med("major_errors")})
    return plans


def checklist_totals(repo):
    out = collections.defaultdict(dict)  # run -> batch -> totals
    for b in json.load(open(repo / "calibration/registry/checklist-judging.json")):
        if b["id"] == "J14":  # K2's re-judgement of J11's plans by Sonnet; J11 is the first
            continue
        root = repo / b["dir"]
        key = json.load(open(root / "key.json"))
        for pid, run in key.items():
            files = sorted((root / b["verdicts"]).glob(f"{pid}.json")) + sorted((root / b["verdicts"]).glob(f"{pid}-*.json"))
            tot = [sum(s["verdict"] == "present" for s in json.load(open(x))["steps"]) for x in files]
            if tot:
                out[run][b["id"]] = tot
    return out


def item_verdicts(repo):
    """run -> batch -> item -> list of derivation verdicts (one per judge)."""
    out = collections.defaultdict(lambda: collections.defaultdict(lambda: collections.defaultdict(list)))
    for b in json.load(open(repo / "calibration/registry/judging.json")) + extra_batches(repo):
        root = repo / b["dir"]
        key = json.load(open(root / b["key"]))
        for pid, run in key.items():
            for x in sorted((root / b["verdicts"]).glob(f"{pid}.json")) + sorted((root / b["verdicts"]).glob(f"{pid}-*.json")):
                v = json.load(open(x))
                for item in v.get("items", []):
                    out[run][b["id"]][item["id"]].append(item.get("derivation"))
    return out


# ---------------------------------------------------------------- statistics


def ranks(xs):
    order = sorted(range(len(xs)), key=lambda i: xs[i])
    r = [0.0] * len(xs)
    i = 0
    while i < len(xs):
        j = i
        while j + 1 < len(xs) and xs[order[j + 1]] == xs[order[i]]:
            j += 1
        for k in range(i, j + 1):
            r[order[k]] = (i + j) / 2 + 1
        i = j + 1
    return r


def pearson(a, b):
    ma, mb = statistics.fmean(a), statistics.fmean(b)
    sa = math.sqrt(sum((x - ma) ** 2 for x in a))
    sb = math.sqrt(sum((y - mb) ** 2 for y in b))
    if sa == 0 or sb == 0:
        return None
    return sum((x - ma) * (y - mb) for x, y in zip(a, b)) / (sa * sb)


def spearman(x, y, seed=1):
    """Spearman's rho and its two-sided permutation p (exact for n <= 8)."""
    rx, ry = ranks(x), ranks(y)
    rho = pearson(rx, ry)
    if rho is None:
        return None, None
    n = len(x)
    if n <= 8:
        perms = itertools.permutations(ry)
        total = math.factorial(n)
    else:
        rng = random.Random(seed)
        total = 20000
        perms = (rng.sample(ry, n) for _ in range(total))
    hits = 0
    for p in perms:
        r = pearson(rx, list(p))
        if r is not None and abs(r) >= abs(rho) - 1e-12:
            hits += 1
    return rho, hits / total


def two_way(matrix, perms=20000, seed=7):
    """matrix[arm][extraction]: values, one run per cell. Returns the share of the non-arm
    variance that lies between extractions, the F statistic for extractions, and its permutation
    p (extraction labels shuffled within each arm)."""
    arms = list(matrix)
    exts = list(next(iter(matrix.values())))
    a, e = len(arms), len(exts)

    def f_stat(m):
        grand = statistics.fmean(v for arm in arms for v in m[arm])
        arm_mean = [statistics.fmean(m[arm]) for arm in arms]
        ext_mean = [statistics.fmean(m[arm][j] for arm in arms) for j in range(e)]
        ss_e = a * sum((x - grand) ** 2 for x in ext_mean)
        ss_r = sum((m[arms[i]][j] - arm_mean[i] - ext_mean[j] + grand) ** 2 for i in range(a) for j in range(e))
        df_e, df_r = e - 1, (a - 1) * (e - 1)
        return (ss_e / df_e) / (ss_r / df_r) if ss_r else float("inf"), ss_e, ss_r

    m0 = {arm: [matrix[arm][x] for x in exts] for arm in arms}
    f0, ss_e, ss_r = f_stat(m0)
    rng = random.Random(seed)
    hits = 0
    for _ in range(perms):
        mp = {arm: rng.sample(m0[arm], e) for arm in arms}
        if f_stat(mp)[0] >= f0 - 1e-12:
            hits += 1
    return f0, hits / perms, ss_e / (ss_e + ss_r)


def mantel_haenszel(strata):
    """strata: lists of (exposed, outcome) pairs. Returns the MH odds ratio and the table totals."""
    num = den = 0.0
    tot = collections.Counter()
    for s in strata:
        n = len(s)
        if n < 2:
            continue
        a = sum(1 for x, y in s if x and y)
        b = sum(1 for x, y in s if x and not y)
        c = sum(1 for x, y in s if not x and y)
        d = sum(1 for x, y in s if not x and not y)
        tot.update(a=a, b=b, c=c, d=d)
        num += a * d / n
        den += b * c / n
    return (num / den if den else float("inf")), tot


def superiority(xs, ys, seed=3, limit=50000):
    """P(x > y) + P(x = y)/2 over pairs, and its two-sided permutation p (exact when the
    relabellings number at most `limit`, otherwise 20,000 random relabellings)."""
    def a_stat(a, b):
        return sum((x > y) + 0.5 * (x == y) for x in a for y in b) / (len(a) * len(b))

    obs = a_stat(xs, ys)
    pooled, k = xs + ys, len(xs)
    if math.comb(len(pooled), k) <= limit:
        splits = itertools.combinations(range(len(pooled)), k)
        total = math.comb(len(pooled), k)
    else:
        rng = random.Random(seed)
        total = 20000
        splits = (rng.sample(range(len(pooled)), k) for _ in range(total))
    hits = 0
    for idx in splits:
        s = set(idx)
        a = [pooled[i] for i in s]
        b = [pooled[i] for i in range(len(pooled)) if i not in s]
        if abs(a_stat(a, b) - 0.5) >= abs(obs - 0.5) - 1e-12:
            hits += 1
    return obs, hits / total


# ---------------------------------------------------------------- output


def fmt(x):
    if x is None:
        return "-"
    if isinstance(x, float):
        return f"{x:.2f}" if abs(x) < 10 and x != int(x) else f"{x:g}"
    return str(x)


def md_table(header, rows):
    out = ["| " + " | ".join(header) + " |", "|" + "---|" * len(header)]
    out += ["| " + " | ".join(fmt(c) for c in r) + " |" for r in rows]
    return "\n".join(out)


def med_range(xs):
    xs = [x for x in xs if x is not None]
    if not xs:
        return "-"
    m = statistics.median(xs)
    return f"{fmt(m)} ({fmt(min(xs))}-{fmt(max(xs))})"


MEASURES = [
    ("concepts", "concepts"), ("unknown", "unknown concepts"), ("gloss_chars", "gloss characters"),
    ("quotes", "quotes"),
    ("located", "located (exegenius)"), ("first_tenth", "quotes in first tenth"),
    ("body_share", "share of quotes past the introduction"), ("last_line", "last line quoted"),
    ("blocks", "100-line blocks quoted (of 54)"), ("dev_quoted", "developing top-level sections quoted"),
    ("subsections_quoted", "body subsections quoted"), ("results_quoted", "results quoted (of 66)"),
    ("core_lines", "core items reached by lines (of 19)"),
    ("core_derived_lines", "core items reached in derivation lines"),
    ("core_labels", "core items reached by label"), ("core_either", "core items reached (lines or label)"),
    ("supporting_lines", "supporting items reached by lines (of 7)"),
    ("checklist_lines", "checklist sub-steps whose lines are quoted (of 26)"),
    ("markers", "checklist markers in the extraction's text (of 25)"),
    ("cache_read_m", "extraction cache read (M)"), ("seconds", "extraction seconds"),
]


def main():
    here = pathlib.Path(__file__).resolve()
    ap = argparse.ArgumentParser()
    ap.add_argument("--wiki", default="/home/user/openai-nlogn-wiki")
    ap.add_argument("--repo", default=str(here.parents[2]))
    ap.add_argument("--out", default=str(here.parents[2] / "docs/journal/2026-10-14-x1-extraction-evidence"))
    args = ap.parse_args()
    wiki, repo, out = pathlib.Path(args.wiki), pathlib.Path(args.repo), pathlib.Path(args.out)

    paper = Paper(git(wiki, "show", f"{PAPER_COMMIT}:{PAPER_PATH}"))
    ref = json.load(open(repo / "docs/journal/2026-10-13-run-log-and-coverage/reference-draft/reference.json"))
    reference = [i for i in ref["items"] if i["tier"] != "out-of-scope"]
    core_ids = [i["id"] for i in reference if i["tier"] == "core"]
    substeps = checklist_substeps(repo)
    markers = load_markers(repo)

    # The paper's 66 results, as the logs name them: union of every attempt's unquoted results
    # and of every result labelled in the logs.
    logs, result_labels = {}, set()
    for ext, exp, commit, run, *_ in EXTRACTIONS:
        logs[ext] = extract_events(git(wiki, "show", f"{commit}:{run}/exegenius/log.jsonl"))
        for e in logs[ext]:
            for c in e["coverage"]:
                for u in c.get("unquoted_results", []):
                    if "`" in u["what"]:
                        result_labels.add(u["what"].split("`")[1])

    rows, reach, reach_lines = [], {}, {}
    for ext, exp, commit, run, model, effort, agent in EXTRACTIONS:
        answer = json.loads(git(wiki, "show", f"{commit}:{run}/exegenius/tasks/0001-extract.answer.json"))
        evs = logs[ext]
        accepted = [e for e in evs if e["outcome"] == "accepted"][-1]
        cov = [c for c in accepted["coverage"] if c["source"] == "paper"][0]
        m, reached, reached_lines = measure(paper, answer, cov, reference, substeps, markers, result_labels)
        first = [c for c in evs[0]["coverage"] if c["source"] == "paper"][0]
        cr, secs, basis = extraction_cost(repo, wiki, exp, commit, run)
        m["cache_read_m"] = round(cr / 1e6, 2) if cr else None
        m["seconds"] = round(secs) if secs else None
        row = {"extraction": ext, "experiment": exp, "commit": commit, "model": model, "effort": effort,
               "agent": agent, "submits": len(evs),
               "coverage_rejections": sum(e.get("check") == "coverage" for e in evs),
               "first_results_quoted": first["quoted_results"], "first_dev_quoted": None, **m,
               "cost_basis": basis}
        top = [s for s in first["sections"] if s["level"] == 1 and s["title"] != "(front matter)"
               and (s["statements"] or s["displays"])]
        row["first_dev_quoted"] = sum(s["quotes"] > 0 for s in top)
        rows.append(row)
        reach[ext], reach_lines[ext] = reached, reached_lines

    plans = primary_plans(repo)
    checks = checklist_totals(repo)
    items = item_verdicts(repo)
    by_ext = {r["extraction"]: r for r in rows}

    out.mkdir(parents=True, exist_ok=True)
    md = ["# Extraction evidence: generated tables",
          "",
          "Generated by `calibration/tools/extraction_stats.py`; do not edit by hand. Read the README "
          "beside this file for what the figures can and cannot show.",
          ""]

    # 1. per extraction
    md.append("## 1. Judge-free measures, per extraction (accepted answer)\n")
    md.append(f"Paper: {paper.n_lines} lines; the introduction ends at line {INTRO_END}. "
              f"{rows[0]['dev_total']} top-level sections develop results (hold theorem-like statements or "
              f"displays); {rows[0]['subsections_total']} subsections lie outside the introduction. "
              f"Reference: 19 core and 7 supporting items; "
              f"{rows[0]['core_labelled_total']} core items carry a label that is one of the paper's "
              f"{rows[0]['results_total']} results. Cost basis: the registry's measured subagent figures, "
              "or (E6) the extraction transcripts on the evidence branch.\n")
    hdr = ["Extraction", "Config", "Submits", "Concepts (unknown)", "Gloss chars", "Quotes", "1st tenth", "Past intro",
           "Last line", "Blocks", "Dev. sections (1st try)", "Subsections", "Results (1st try)",
           "Core: lines / deriv. / label / either", "Supp.", "Checklist lines", "Markers", "Cache read M", "s"]
    body = []
    for r in rows:
        body.append([r["extraction"], f"{r['model']} {r['effort']}, {r['agent']}", r["submits"],
                     f"{r['concepts']} ({r['unknown']})", r["gloss_chars"], r["quotes"], r["first_tenth"], r["body_share"],
                     r["last_line"], r["blocks"], f"{r['dev_quoted']}/{r['dev_total']} ({r['first_dev_quoted']})",
                     f"{r['subsections_quoted']}/{r['subsections_total']}",
                     f"{r['results_quoted']} ({r['first_results_quoted']})",
                     f"{r['core_lines']} / {r['core_derived_lines']} / {r['core_labels']} / {r['core_either']}",
                     r["supporting_lines"], r["checklist_lines"], r["markers"], r["cache_read_m"], r["seconds"]])
    md.append(md_table(hdr, body))
    differ = [f"{r['extraction']} ({r['located_here']} of {r['located']})" for r in rows
              if r["located_here"] != r["located"]]
    md.append(f"\nexegenius located every quote in {sum(r['located'] == r['quotes'] for r in rows)} of "
              f"{len(rows)} accepted extractions. This script's locator (used for reference items, blocks and "
              f"checklist lines) places the same number in all but: {', '.join(differ) or 'none'}. "
              "Core items reached by label never add an item that lines miss, so the 'either' count equals "
              "the 'lines' count throughout.\n")

    # 2. by group
    md.append("## 2. By model and effort (median and range)\n")
    groups = collections.OrderedDict()
    for r in rows:
        groups.setdefault((r["model"], r["effort"]), []).append(r)
    sub = collections.OrderedDict()
    for r in rows:
        sub.setdefault((r["model"], r["effort"], r["agent"]), []).append(r)
    keys = ["concepts", "unknown", "gloss_chars", "quotes", "first_tenth", "body_share", "blocks", "dev_quoted",
            "subsections_quoted", "results_quoted", "core_lines", "core_derived_lines", "core_either",
            "supporting_lines", "checklist_lines", "markers", "cache_read_m", "seconds"]
    md.append(md_table(["Measure"] + [f"{m} {e} (n={len(v)})" for (m, e), v in groups.items()],
                       [[label] + [med_range([r[k] for r in v]) for v in groups.values()]
                        for k, label in MEASURES if k in keys]))
    md.append("\nBy agent type within a configuration:\n")
    md.append(md_table(["Measure"] + [f"{m} {e}, {a} (n={len(v)})" for (m, e, a), v in sub.items()],
                       [[label] + [med_range([r[k] for r in v]) for v in sub.values()]
                        for k, label in MEASURES if k in ("results_quoted", "core_lines", "core_either",
                                                          "checklist_lines", "blocks", "unknown", "cache_read_m")]))
    md.append("\nFirst-attempt figures (before any coverage rejection): developing sections quoted and "
              "results quoted.\n")
    md.append(md_table(["Configuration", "Coverage rejections", "Dev. sections, 1st try", "Results, 1st try"],
                       [[f"{m} {e}", sum(r["coverage_rejections"] for r in v),
                         med_range([r["first_dev_quoted"] for r in v]),
                         med_range([r["first_results_quoted"] for r in v])] for (m, e), v in groups.items()]))

    md.append("\n### Which differences from Sonnet `high` are beyond the spread within configurations\n")
    md.append("A = probability that a Sonnet `high` extraction exceeds one of the other configuration (ties "
              "count half; 0.5 = no difference); p = two-sided permutation p of A (exact where feasible). "
              "Sonnet `high` has 18 extractions; the others 2-7, so a two-extraction group can reach p < 0.05 "
              "only if both lie outside the whole Sonnet `high` range.\n")
    base = groups[("Sonnet", "high")]
    test_keys = ["results_quoted", "core_lines", "core_derived_lines", "subsections_quoted", "blocks",
                 "first_tenth", "unknown", "gloss_chars"]
    body = []
    for (mo, ef), v in groups.items():
        if (mo, ef) == ("Sonnet", "high"):
            continue
        row = [f"{mo} {ef} (n={len(v)})"]
        for k in test_keys:
            a, p = superiority([r[k] for r in base], [r[k] for r in v])
            row.append(f"{a:.2f} (p {p:.3f})")
        body.append(row)
    md.append(md_table(["Against Sonnet high"] + [dict(MEASURES)[k] for k in test_keys], body))

    # Reference items reached, per configuration
    md.append("\n### Core reference items reached (lines or label), share of extractions per configuration\n")
    hdr = ["Core item"] + [f"{m} {e}" for (m, e) in groups]
    body = []
    for cid in core_ids:
        body.append([cid] + [f"{sum(cid in reach[r['extraction']] for r in v)}/{len(v)}" for v in groups.values()])
    md.append(md_table(hdr, body))

    # 3. downstream on each extraction
    md.append("\n## 3. Plans judged on each extraction (primary batch, median of its judges)\n")
    md.append("Faithful = core results whose derivation was judged faithful (of 19); central = steps of the "
              "central saving reconstructed (of 9); checklist = mechanism sub-steps present (of 26), one "
              "total per judge. Arms: the experiment and its arm (P/G/M/O/Q/K/N: full planning in one "
              "configuration, the argument by the same model as the extraction; E4 B/V, E6 B/V, E7 W, "
              "E8/E8b X, E9 Y: Sonnet `high` arguments on fixed extractions, under successive "
              "instructions).\n")
    body = []
    for p in sorted(plans, key=lambda p: (p["extraction"], p["arm"])):
        ch = checks.get(p["run"], {})
        body.append([p["extraction"], p["run"], p["arm"], p["batch"], p["judges"], p["faithful"],
                     ",".join(map(str, p["faithful_each"])), p["central"], p["major"],
                     "; ".join(f"{b} {t}" for b, t in sorted(ch.items())) or "-"])
    md.append(md_table(["Extraction", "Plan", "Arm", "Batch", "Judges", "Faithful", "Each", "Central",
                        "Major", "Checklist"], body))

    # 4. E6's eight extractions
    md.append("\n## 4. E6's eight Sonnet `high` extractions: how much of the plan is the extraction's\n")
    fs = [f"E6-F{k}" for k in range(1, 9)]
    arm_of = {"E6:B": "B", "E6:V": "V", "E7:W": "W", "E8:X": "X", "E8b:X": "X", "E9:Y": "Y", "E10:Y": "Y"}
    repeats = ("E8F4X2", "E10F4Y2")
    mat = collections.defaultdict(dict)
    central = collections.defaultdict(dict)
    for p in plans:
        a = arm_of.get(p["arm"])
        if a and p["extraction"] in fs and p["run"] not in repeats:
            mat[a][p["extraction"]] = p["faithful"]
            central[a][p["extraction"]] = p["central"]
    ckl = collections.defaultdict(dict)
    for run, bt in checks.items():
        m = re.match(r"^E([78])F(\d)([WX])$", run)
        if m:  # the batch that judged W and X on the extraction together
            tots = [t for b, t in sorted(bt.items()) if b in ("J15", "J17")]
            if tots:
                ckl[m.group(3)][f"E6-F{m.group(2)}"] = statistics.fmean(tots[0])
    body = []
    for f in fs:
        body.append([f] + [mat[a].get(f) for a in "BVWXY"] + [central[a].get(f) for a in "BVW"] +
                    [ckl[a].get(f) for a in "WX"])
    md.append(md_table(["Extraction", "B faithful", "V", "W", "X", "Y", "B central", "V central",
                        "W central", "W checklist (mean of 2)", "X checklist"], body))
    full = {a: mat[a] for a in "BVWX" if all(f in mat[a] for f in fs)}
    f0, p0, share = two_way({a: {f: full[a][f] for f in fs} for a in full})
    md.append(f"\n**Two-way decomposition of faithful (arms {', '.join(full)} x 8 extractions, one run per cell):** "
              f"F = {f0:.2f} for extractions, permutation p = {p0:.3f}; extractions carry {share:.0%} of the "
              "variance left after the arm means. The residual includes run-to-run variance, which the "
              "design cannot separate from an extraction x instruction interaction.\n")
    full_c = {a: {f: ckl[a][f] for f in fs} for a in "WX" if all(f in ckl[a] for f in fs)}
    if len(full_c) == 2:
        f1, p1, share1 = two_way(full_c)
        md.append(f"**The same on the checklist (W, X):** F = {f1:.2f}, permutation p = {p1:.3f}, "
                  f"extraction share {share1:.0%}.\n")
    md.append("**Do the arms rank the extractions alike?** Spearman's rho between arms' faithful counts "
              "over F1-F8 (exact permutation p):\n")
    body = []
    arms4 = list(full)
    for a, b in itertools.combinations(arms4, 2):
        rho, p = spearman([full[a][f] for f in fs], [full[b][f] for f in fs])
        body.append([f"{a} vs {b}", f"{rho:+.2f}", f"{p:.4f}"])
    if len(full_c) == 2:
        rho, p = spearman([full_c["W"][f] for f in fs], [full_c["X"][f] for f in fs])
        body.append(["W vs X checklist", f"{rho:+.2f}", f"{p:.4f}"])
    md.append(md_table(["Pair", "rho", "p"], body))
    md.append("\n**Run-to-run variance on one extraction (same planner, same instruction, same extraction):**\n")
    body = []
    for a, b in (("E8F4X", "E8F4X2"), ("E9F4Y", "E10F4Y2"), ("S3F8OX", "S5F8OX")):
        if not [p for p in plans if p["run"] == b]:
            continue
        pa = [p for p in plans if p["run"] == a][0]
        pb = [p for p in plans if p["run"] == b][0]
        shared = sorted(set(checks.get(a, {})) & set(checks.get(b, {})))
        ch = "; ".join(f"{bt}: {checks[a][bt]} vs {checks[b][bt]}" for bt in shared) or "-"
        body.append([f"{a} vs {b}", f"{pa['faithful']} ({pa['batch']}) vs {pb['faithful']} ({pb['batch']})",
                     abs(pa["faithful"] - pb["faithful"]), ch])
    md.append(md_table(["Repeat", "Faithful", "Difference", "Checklist, same batch"], body))
    rejudged = items.get("E9F4Y", {}).get("J21", {})
    if rejudged:
        n = max(len(v) for v in rejudged.values())
        each = [sum(1 for cid in core_ids if len(rejudged.get(cid, [])) > j and rejudged[cid][j] == "faithful")
                for j in range(n)]
        md.append(f"\nE9F4Y and E10F4Y2 were first judged in different batches (J18, J21); J21 also re-judged "
                  f"E9F4Y, faithful {each} against E10F4Y2's {[p for p in plans if p['run'] == 'E10F4Y2'][0]['faithful_each']}.\n")

    # 5. correlations
    md.append("## 5. Do the judge-free measures predict the plan?\n")
    outcomes = [(f"{a} faithful", {f: full[a][f] for f in fs}) for a in arms4]
    outcomes += [(f"{a} checklist", full_c[a]) for a in full_c]
    n_tests = len([k for k, _ in MEASURES if k not in ("cache_read_m", "seconds")]) * (len(outcomes) + 1)
    md.append("Spearman's rho of each extraction measure against each arm's faithful count (and the "
              "checklist), over E6's eight extractions; exact permutation p, two-sided. With n = 8, "
              f"|rho| >= 0.74 is needed for p < 0.05, and with {n_tests} tests about "
              f"{round(n_tests * 0.05)} would cross it by chance alone. The last column is the mean of the "
              "arms' within-arm ranks.\n")
    mean_rank = {f: statistics.fmean(ranks([o[g] for g in fs])[fs.index(f)] for _, o in outcomes) for f in fs}
    body = []
    for k, label in MEASURES:
        if k in ("cache_read_m", "seconds"):
            continue
        xs = [by_ext[f][k] for f in fs]
        row = [label, " ".join(fmt(x) for x in xs)]
        for name, o in outcomes + [("mean rank", mean_rank)]:
            rho, p = spearman(xs, [o[f] for f in fs])
            row.append("const" if rho is None else f"{rho:+.2f} (p {p:.2f})")
        body.append(row)
    md.append(md_table(["Measure", "F1..F8"] + [n for n, _ in outcomes] + ["mean rank"], body))

    # E1's P extractions, three Sonnet-high planners each
    md.append("\n**E1's five planner extractions (P1-P5), each planned three times by Sonnet `high`** "
              "(P: its own run, J3; E4 B and V, J5):\n")
    ps = [f"E1-P{k}" for k in range(1, 6)]
    pm = collections.defaultdict(dict)
    for p in plans:
        if p["extraction"] in ps and p["arm"] in ("E1:P", "E4:B", "E4:V"):
            pm[p["arm"]][p["extraction"]] = p["faithful"]
    body = [[e] + [pm[a].get(e) for a in ("E1:P", "E4:B", "E4:V")] +
            [by_ext[e][k] for k in ("results_quoted", "core_either", "checklist_lines", "blocks")] for e in ps]
    md.append(md_table(["Extraction", "P faithful", "E4 B", "E4 V", "Results quoted", "Core reached",
                        "Checklist lines", "Blocks"], body))
    full_p = {a: pm[a] for a in pm if all(e in pm[a] for e in ps)}
    if len(full_p) == 3:
        f2, p2, share2 = two_way(full_p)
        md.append(f"\nTwo-way decomposition (3 arms x 5 extractions): F = {f2:.2f}, permutation p = {p2:.3f}, "
                  f"extraction share {share2:.0%}.\n")
        body = []
        for k in ("results_quoted", "core_either", "checklist_lines", "blocks", "unknown"):
            xs = [by_ext[e][k] for e in ps]
            row = [k]
            for a in ("E1:P", "E4:B", "E4:V"):
                rho, p = spearman(xs, [pm[a][e] for e in ps])
                row.append("const" if rho is None else f"{rho:+.2f} (p {p:.2f})")
            body.append(row)
        md.append(md_table(["Measure", "P", "E4 B", "E4 V"], body))

    # full-planning runs across configurations (extraction and argument by the same model)
    md.append("\n**Full-planning runs across configurations** (the argument was planned by the same "
              "model and effort as the extraction, so a measure that tracks the plan here may track the "
              "planner, not the extraction):\n")
    fp = [p for p in plans if p["arm"] in ("E1:P", "E1:G", "E3:M", "S1:O", "S1:Q", "S1:K", "E5:N")]
    body = []
    for k in ("results_quoted", "core_either", "checklist_lines", "blocks", "unknown", "dev_quoted"):
        xs = [by_ext[p["extraction"]][k] for p in fp]
        rho, pv = spearman(xs, [p["faithful"] for p in fp])
        rho2, pv2 = spearman(xs, [p["central"] for p in fp])
        sh = [p for p in fp if by_ext[p["extraction"]]["model"] == "Sonnet" and by_ext[p["extraction"]]["effort"] == "high"]
        rho3, pv3 = spearman([by_ext[p["extraction"]][k] for p in sh], [p["faithful"] for p in sh])
        body.append([k, f"{rho:+.2f} (p {pv:.3f})", f"{rho2:+.2f} (p {pv2:.3f})",
                     "const" if rho3 is None else f"{rho3:+.2f} (p {pv3:.2f})"])
    md.append(md_table(["Measure", f"faithful, all {len(fp)} runs", "central, all runs",
                        f"faithful, Sonnet high only (n={sum(1 for p in fp if p['arm'] in ('E1:P', 'E1:G'))})"], body))

    # 6. item level
    md.append("\n## 6. Item level: is a core result faithful more often when the extraction reached it?\n")
    md.append("For each plan on a fixed extraction (E6 family: B, V, W, X, one plan per arm and extraction, "
              "E8F4X2 left out; E1-P family: E4 B and V) and each core result: was it faithful by majority "
              "of the plan's primary-batch judges, and did the extraction reach it (a quote in its stated "
              "or derived lines)? Strata: arm x core result, so an arm's general level and a result's "
              "difficulty are held fixed; strata where every extraction reaches the result, or none does, "
              "carry no information. Mantel-Haenszel odds ratio. The permutation p reassigns the eight "
              "(five) extractions' reach patterns to one another, every permutation, one-sided (OR at "
              "least as large).\n")
    merged = {"E8:X": "X", "E8b:X": "X"}

    def outcomes_for(family, arms):
        out = {}
        for p in plans:
            a = merged.get(p["arm"], p["arm"])
            if a not in arms or p["extraction"] not in family or p["run"] in ("E8F4X2", "E10F4Y2"):
                continue
            for cid in core_ids:
                v = items[p["run"]][p["batch"]].get(cid, [])
                if v:
                    out[(a, cid, p["extraction"])] = sum(x == "faithful" for x in v) * 2 > len(v)
        return out

    def mh_for(family, arms, outc, reach_map):
        strata = [[(cid in reach_map[f], outc[(a, cid, f)]) for f in family if (a, cid, f) in outc]
                  for a in arms for cid in core_ids]
        return mantel_haenszel(strata)

    def exact_p(family, arms, outc, reach_map, observed):
        hits = total = 0
        for perm in itertools.permutations(family):
            o, _ = mh_for(family, arms, outc, {f: reach_map[g] for f, g in zip(family, perm)})
            hits += o >= observed - 1e-12
            total += 1
        return hits / total

    rr = lambda a, b: f"{a}/{a + b} = {a / (a + b):.2f}" if a + b else "-"
    body = []
    e6_arms = ("E6:B", "E6:V", "E7:W", "X")
    for name, family, arms in [("E6 F1-F8, arms B V W X", fs, e6_arms)] + \
                              [(f"E6 F1-F8, arm {a} alone", fs, (a,)) for a in e6_arms] + \
                              [("E1 P1-P5, E4 arms B V", ps, ("E4:B", "E4:V"))]:
        outc = outcomes_for(family, arms)
        orr, tot = mh_for(family, arms, outc, reach)
        body.append([name, rr(tot["a"], tot["b"]), rr(tot["c"], tot["d"]), orr,
                     f"{exact_p(family, arms, outc, reach, orr):.4f}"])
    md.append(md_table(["Plans", "Faithful when reached", "Faithful when not", "MH OR", "exact p"], body))
    md.append("\n**Robustness (E6 family, arms B V W X):** leaving out one extraction, or the two weakest "
              "plans on low-reach extractions (V on F6, faithful 0; W on F8, faithful 5).\n")
    body = []
    outc_all = outcomes_for(fs, e6_arms)
    for drop in fs:
        fam = [f for f in fs if f != drop]
        orr, _ = mh_for(fam, e6_arms, outc_all, reach)
        body.append([f"without {drop}", orr, f"{exact_p(fam, e6_arms, outc_all, reach, orr):.4f}"])
    trimmed = {k: v for k, v in outc_all.items() if (k[0], k[2]) not in (("E6:V", "E6-F6"), ("E7:W", "E6-F8"))}
    orr, _ = mh_for(fs, e6_arms, trimmed, reach)
    body.append(["without plans E6F6V and E7F8W", orr, f"{exact_p(fs, e6_arms, trimmed, reach, orr):.4f}"])
    md.append(md_table(["Variant", "MH OR", "exact p"], body))
    merged.update({"E9:Y": "Y", "E10:Y": "Y"})
    y = outcomes_for(fs, ("Y",))
    y_exts = sorted({k[2][-2:] for k in y})
    unreached = sum(1 for (a, cid, f), v in y.items() if cid not in reach[f])
    md.append(f"\n**Under the combined instruction (Y: E9 and E10, on {' '.join(y_exts)}):** {sum(y.values())} "
              f"of {len(y)} core results faithful by majority, {sum(v for (a, cid, f), v in y.items() if cid not in reach[f])} "
              f"of the {unreached} that the extraction does not reach among them. No stratum varies, so Y "
              "carries no information on reach: under Y the planner supplies what these extractions miss.\n")
    md.append("**What X (E8/E8b) did not get faithful, and whether the extraction reached it:**\n")
    body = []
    for f in fs:
        miss = [cid for cid in core_ids if ("X", cid, f) in outc_all and not outc_all[("X", cid, f)]]
        body.append([f, mat["X"].get(f), ", ".join(f"{c} ({'reached' if c in reach[f] else 'not reached'})"
                                                   for c in miss) or "none"])
    md.append(md_table(["Extraction", "X faithful", "Core results not faithful by majority"], body))
    md.append("\nPer core result, the E6 extractions that reach it:\n")
    md.append(md_table(["Core result", "Reached by"],
                       [[cid, " ".join(f[-2:] for f in fs if cid in reach[f]) or "none"] for cid in core_ids]))

    (out / "table.md").write_text("\n".join(md) + "\n")
    cols = ["extraction", "experiment", "commit", "model", "effort", "agent", "submits",
            "coverage_rejections"] + [k for k, _ in MEASURES] + ["first_dev_quoted", "first_results_quoted",
                                                                  "dev_total", "located_here", "cost_basis"]
    with open(out / "extractions.tsv", "w") as f:
        f.write("\t".join(cols) + "\n")
        for r in rows:
            f.write("\t".join(fmt(r.get(c)) for c in cols) + "\n")
    print(f"wrote {out / 'table.md'} and {out / 'extractions.tsv'}")


if __name__ == "__main__":
    main()

"""Blind pairwise preference judging of argument plans (K3, draft): prepare pair files, score verdicts.

usage:
  python3 -I pairwise.py prepare PAPER.tex OUT_DIR SEED NAME=ARGUMENT.json ... --pair X:Y[:J] ...
         [--drop NEW=BASE:STEP[,STEP...]] [--orders both|one] [--reference REFERENCE.json]
         [--prompt TEMPLATE.md] [--checklist CHECKLIST.md]
  python3 -I pairwise.py score OUT_DIR [--expect X>Y ...] [--margin M] [--arm ARM=NAME,NAME ...]
         [--primary BATCH_DIR ...] [--boot B] [--seed S]

prepare: renders each named plan as prepare.py does (an indented tree, each quote with its line in the
paper) and writes, for each pair X:Y, one pair file per order: OUT_DIR/pairs/Qnn-A.md and Qnn-B.md.
With --orders both (the default) X:Y gives two pair files, X as A and Y as A; X:X (the same plan twice)
gives one. With --orders one, each pair gets one order, drawn from SEED. J is the number of judges per
pair file (default 1). All pair files are numbered in an order shuffled by SEED, so a judge given one
cannot tell which runs it holds or which order it is. OUT_DIR/key.json records, per pair file, the plan
in position A and in position B, and the judges; it also records each plan's source and rendered size.
--drop defines a plan NEW as plan BASE with the named steps (and their subtrees) removed: a planted
omission whose ordering is known by construction. --reference writes OUT_DIR/pack.md (byte for byte as
prepare.py writes it). --prompt renders TEMPLATE (a leading <!-- --> comment is stripped) once per
judge to OUT_DIR/prompts/Qnn[-b|-c...].txt, filling {{PAIR}}, {{PLAN_A}}, {{PLAN_B}}, {{VERDICT}},
{{PACK}}, {{CHECKLIST}} and {{PAPER}} with absolute paths. It prints a cache-read estimate per
judgement from J18's measured fit (see calibration/plans/K3-draft.md, "Cost").

score: reads OUT_DIR/verdicts/Qnn[-b...].json and maps A and B back to plan names through the key.
It reports:
  - the preference matrix (overall verdicts; wins of the row over the column, and ties);
  - each --expect X>Y pair: verdicts for X, for Y, tied, the net (X minus Y), reversals across orders,
    and the exact one-sided sign test over expected pairs (a pair counts for X when net >= M, for Y when
    net <= -M; otherwise it is tied and dropped; M defaults to 1);
  - position bias: on pairs of different plans, the share of non-tie verdicts (overall and per item)
    given to position A, with an exact two-sided binomial p; on identical pairs, every non-tie verdict
    is a false preference, reported with its position;
  - swap consistency: across the two orders of a pair, every couple of verdicts (one from each order)
    is compared after mapping to plan names: agree, partial (one tie) or reversal. Judge-to-judge
    agreement is the same on couples within one pair file, with Cohen's kappa; the gap between the
    two isolates the effect of order;
  - with --primary (a primary judging batch: BATCH_DIR/key.json and BATCH_DIR/verdicts/): for each pair
    whose two plans were judged in that batch, the items whose primary verdicts differ beyond noise
    (the judges' ranges do not overlap, on absent 0 < mentioned 1 < stated 2 < derived 3 < derived
    faithfully 4, wrong 0), and whether the pairwise majority on each item agrees, ties or reverses;
    also the items tied in the primary verdicts on which the pairwise verdicts agree in direction;
  - Bradley-Terry strengths (ties as half a win, one pseudo-tie per compared couple) over plans, or
    over arms with --arm, with a bootstrap over pairs (the unordered pairs are the resampled units).

Verdicts are data from judges: every field is checked, and a malformed file is reported and skipped.
"""
import argparse
import bisect
import collections
import copy
import json
import math
import pathlib
import random
import re
import statistics
import sys

PLACEHOLDERS = ("PAIR", "PLAN_A", "PLAN_B", "VERDICT", "PACK", "CHECKLIST", "PAPER")
SUFFIXES = ["", "-b", "-c", "-d", "-e", "-f"]
LABELS = ("A", "B", "tie")
# J18 (50 Opus primary judgements, 11-280 KB plans): cache read = 3.28 M + 1.53 M per 100 KB of plan.
FIT_INTERCEPT, FIT_SLOPE = 3.28e6, 1.53e6 / 1e5


# ---------------------------------------------------------------- rendering (as prepare.py)

def spans(text):
    return [(int(a), int(b or a)) for a, b in re.findall(r"(\d+)(?:\s*[-–]\s*(\d+))?", text or "")]


def write_pack(reference, raw, out):
    ref = json.load(open(reference))
    pack = ["# Reference pack for judging plans of `paper.tex`\n",
            "Each item is a result the main theorem's proof depends on, from a draft list that has "
            "not been reviewed: treat its statement as a guide and the paper's text as the "
            "authority. Line numbers are those of `paper.tex`.\n"]
    for item in ref["items"]:
        if item["tier"] == "out-of-scope":
            continue
        where = item["where"]
        pack.append(f"\n## {item['id']} ({item['tier']}): {item['name']}\n")
        pack.append(f"**Draft statement.** {item['statement']}\n")
        pack.append(f"**Role.** {item['role']}\n")
        pack.append(f"**Derived at lines:** {where.get('derived_lines', '')}\n")
        pack.append("**The paper where it is stated:**\n")
        for a, b in sorted(set(spans(where.get("stated_lines")))):
            if b - a > 80:
                b = a + 80
            pack.append("```\n" + "\n".join(f"{n:5d}  {raw[n - 1]}" for n in range(a, b + 1)) + "\n```\n")
    (out / "pack.md").write_text("\n".join(pack))


def quote_locator(raw):
    numbers, starts, pieces, offset = [], [], [], 0
    for n, line in enumerate(raw, 1):
        piece = re.sub(r"\s+", " ", line).strip()
        if piece:
            numbers.append(n)
            starts.append(offset)
            pieces.append(piece)
            offset += len(piece) + 1
    text = " ".join(pieces)

    def line_of(q):
        at = text.find(re.sub(r"\s+", " ", q).strip())
        return None if at < 0 else numbers[bisect.bisect_right(starts, at) - 1]
    return line_of


def render(node, depth, out_lines, line_of):
    pad = "  " * depth
    head = f"{pad}- **{node['id']}** ({node['intention']}"
    head += f", page `{node['page']}`" if node.get("page") else ""
    head += ", reported out of order" if node.get("deferred") else ""
    out_lines.append(head + f"): {node['claim']}")
    if node.get("method"):
        out_lines.append(f"{pad}  - method: {node['method']}")
    m = node.get("motivation")
    if m:
        out_lines.append(f"{pad}  - motivation ({m.get('source')}): {m.get('text')}")
    if node.get("from"):
        out_lines.append(f"{pad}  - relies on: {', '.join(node['from'])}")
    if node.get("quote"):
        q = node["quote"]["quote"]
        out_lines.append(f"{pad}  - quotes line {line_of(q)}: \"{q}\"")
    for child in node.get("children", []):
        render(child, depth + 1, out_lines, line_of)


def drop_steps(root, steps):
    """A copy of the tree without the named steps (and their subtrees); fails if a step is missing."""
    root, found = copy.deepcopy(root), set()

    def prune(node):
        kept = []
        for child in node.get("children", []):
            if child["id"] in steps:
                found.add(child["id"])
            else:
                prune(child)
                kept.append(child)
        if "children" in node:
            node["children"] = kept
        if node.get("from"):
            node["from"] = [f for f in node["from"] if f not in steps]
    if root["id"] in steps:
        sys.exit("cannot drop the root step")
    prune(root)
    if found != set(steps):
        sys.exit(f"steps not found: {sorted(set(steps) - found)}")
    return root


def prepare(a):
    out = pathlib.Path(a.out).resolve()
    paper = pathlib.Path(a.paper).resolve()
    raw = paper.read_text().split("\n")
    line_of = quote_locator(raw)
    roots, sources = {}, {}
    for spec in a.plans:
        if "=" not in spec:
            sys.exit(f"not NAME=path: {spec}")
        name, path = spec.split("=", 1)
        if name in roots:
            sys.exit(f"duplicate plan name: {name}")
        roots[name] = json.load(open(path))["root"]
        sources[name] = str(pathlib.Path(path).resolve())
    derived = {}
    for spec in a.drop or []:
        new, rest = spec.split("=", 1)
        base, steps = rest.split(":", 1)
        if base not in roots or new in roots:
            sys.exit(f"bad --drop: {spec}")
        steps = [s for s in steps.split(",") if s]
        roots[new] = drop_steps(roots[base], set(steps))
        derived[new] = {"base": base, "dropped": steps}
        sources[new] = sources[base]
    rendered = {}
    for name, root in roots.items():
        lines = []
        render(root, 0, lines, line_of)
        rendered[name] = "\n".join(lines) + "\n"

    rng = random.Random(int(a.seed))
    entries, seen = [], set()
    for spec in a.pair:
        parts = spec.split(":")
        if len(parts) not in (2, 3) or any(p not in roots for p in parts[:2]):
            sys.exit(f"bad --pair (names must be given as NAME=path or --drop): {spec}")
        x, y = parts[:2]
        judges = int(parts[2]) if len(parts) == 3 else 1
        if not 1 <= judges <= len(SUFFIXES):
            sys.exit(f"judges per pair file must be 1-{len(SUFFIXES)}: {spec}")
        if frozenset((x, y)) in seen:
            sys.exit(f"pair given twice: {spec}")
        seen.add(frozenset((x, y)))
        if x == y:
            entries.append((x, x, judges))
        elif a.orders == "both":
            entries += [(x, y, judges), (y, x, judges)]
        else:
            entries.append((x, y, judges) if rng.random() < 0.5 else (y, x, judges))
    rng.shuffle(entries)

    (out / "pairs").mkdir(parents=True, exist_ok=True)
    (out / "verdicts").mkdir(exist_ok=True)
    if a.reference:
        write_pack(a.reference, raw, out)
    pack = out / "pack.md"
    key = {"seed": int(a.seed), "orders": a.orders, "pairs": {}, "plans": sources, "derived": derived,
           "rendered_bytes": {n: len(t.encode()) for n, t in rendered.items()}}
    for number, (pa, pb, judges) in enumerate(entries, 1):
        qid = f"Q{number:02d}"
        key["pairs"][qid] = {"A": pa, "B": pb, "judges": judges}
        for side, name in (("A", pa), ("B", pb)):
            head = (f"# Plan {side} of pair {qid}\n\nThe plan of the argument for the main theorem, as a "
                    f"tree of steps. Its step ids are this plan's own.\n\n")
            (out / "pairs" / f"{qid}-{side}.md").write_text(head + rendered[name])
    (out / "key.json").write_text(json.dumps(key, indent=1) + "\n")

    if a.prompt:
        text = pathlib.Path(a.prompt).read_text()
        text = re.sub(r"\A\s*<!--.*?-->\s*", "", text, flags=re.S)
        checklist = pathlib.Path(a.checklist or pathlib.Path(__file__).resolve().parent.parent
                                 / "rubrics" / "nlogn-mechanism-checklist.md").resolve()
        if not pack.exists():
            print(f"warning: {pack} does not exist; pass --reference")
        (out / "prompts").mkdir(exist_ok=True)
        for qid, e in key["pairs"].items():
            for suffix in SUFFIXES[:e["judges"]]:
                fill = {"PAIR": qid, "PLAN_A": out / "pairs" / f"{qid}-A.md", "PLAN_B": out / "pairs" / f"{qid}-B.md",
                        "VERDICT": out / "verdicts" / f"{qid}{suffix}.json", "PACK": pack,
                        "CHECKLIST": checklist, "PAPER": paper}
                t = text
                for k in PLACEHOLDERS:
                    t = t.replace("{{" + k + "}}", str(fill[k]))
                if "{{" in t:
                    sys.exit(f"unfilled placeholder in the template: {re.findall(r'{{[^}]*}}', t)}")
                (out / "prompts" / f"{qid}{suffix}.txt").write_text(t)

    print(f"{len(entries)} pair files, {sum(e[2] for e in entries)} judgements, seed {a.seed}")
    total = 0.0
    for qid, e in key["pairs"].items():
        size = key["rendered_bytes"][e["A"]] + key["rendered_bytes"][e["B"]]
        est = FIT_INTERCEPT + FIT_SLOPE * size
        total += est * e["judges"]
        print(f"  {qid}: A {e['A']:10} B {e['B']:10} judges {e['judges']}  plans {size / 1e3:6.1f} KB  "
              f"~{est / 1e6:4.1f} M per judgement (J18 fit, before any pairwise overhead)")
    print(f"estimated cache read: ~{total / 1e6:.0f} M at the fit; ~{total * 1.3 / 1e6:.0f} M with 30% overhead")


# ---------------------------------------------------------------- scoring

def binom_tail(k, n):
    """P(X >= k) for X ~ Binomial(n, 1/2)."""
    return sum(math.comb(n, i) for i in range(k, n + 1)) / 2 ** n if n else 1.0


def two_sided(k, n):
    return min(1.0, 2 * min(binom_tail(k, n), binom_tail(n - k, n))) if n else 1.0


def kappa(couples):
    """Cohen's kappa over (label, label) couples, with pooled marginals."""
    if not couples:
        return None
    po = sum(x == y for x, y in couples) / len(couples)
    counts = collections.Counter(v for c in couples for v in c)
    pe = sum((c / (2 * len(couples))) ** 2 for c in counts.values())
    return None if pe >= 1 else (po - pe) / (1 - pe)


def load_verdicts(out, key, expected_items):
    good, problems = [], []
    for f in sorted((out / "verdicts").glob("Q*.json")):
        qid, _, judge = f.stem.partition("-")
        entry = key["pairs"].get(qid)
        try:
            v = json.load(open(f))
            if entry is None:
                raise ValueError("not in the key")
            if v.get("pair") != qid:
                raise ValueError(f"pair field is {v.get('pair')!r}")
            ov = v["overall"]["preference"]
            if ov not in LABELS:
                raise ValueError(f"overall preference {ov!r}")
            items = {}
            for it in v["items"]:
                if it.get("preference") not in LABELS:
                    raise ValueError(f"item {it.get('id')} preference {it.get('preference')!r}")
                items[it["id"]] = it["preference"]
            if v.get("chain", {}).get("preference") in LABELS:
                items["chain"] = v["chain"]["preference"]
            missing = [i for i in expected_items if i not in items]
            if missing:
                problems.append(f"{f.name}: items missing {missing}")
            errors = collections.Counter((e.get("plan"), e.get("severity")) for e in v.get("errors", []))
        except (ValueError, KeyError, TypeError, json.JSONDecodeError) as exc:
            problems.append(f"{f.name}: skipped ({exc})")
            continue
        good.append({"qid": qid, "judge": judge or "a", "A": entry["A"], "B": entry["B"],
                     "overall": ov, "strength": v["overall"].get("strength"), "items": items,
                     "errors": errors})
    return good, problems


def name_of(v, label):
    return None if label == "tie" else v[label]


def outcome(n1, n2):
    """Two verdicts mapped to plan names (None for a tie): agree, partial (one tie) or reversal."""
    return "agree" if n1 == n2 else ("partial" if None in (n1, n2) else "reversal")


def compare_couples(couples):
    """Outcome counts over couples of verdicts: overall, and per item the two share."""
    c, ci = collections.Counter(), collections.Counter()
    for v1, v2 in couples:
        c[outcome(name_of(v1, v1["overall"]), name_of(v2, v2["overall"]))] += 1
        for i in set(v1["items"]) & set(v2["items"]):
            ci[outcome(name_of(v1, v1["items"][i]), name_of(v2, v2["items"][i]))] += 1
    return c, ci


def split_orders(u, vs):
    """The verdicts on unordered pair u, split by which plan held position A."""
    return [v for v in vs if v["A"] == u[0]], [v for v in vs if v["A"] == u[1]]


def share(c):
    n = sum(c.values())
    return f"agree {c['agree']}, partial {c['partial']}, reversal {c['reversal']} of {n}" + (
        f" ({c['agree'] / n:.2f} agree)" if n else "")


def bt_fit(units, arms):
    """Bradley-Terry strengths by MM; units is a list of lists of (i, j, score of i); ties count 1/2."""
    w, n = collections.defaultdict(float), collections.defaultdict(float)
    for unit in units:
        for i, j, s in unit:
            w[i] += s
            w[j] += 1 - s
            n[frozenset((i, j))] += 1
    for couple in list(n):  # one pseudo-tie per compared couple keeps every estimate finite
        for i in couple:
            w[i] += 0.5
        n[couple] += 1
    p = {i: 1.0 for i in arms}
    for _ in range(5000):
        new = {}
        for i in arms:
            den = sum(n[frozenset((i, j))] / (p[i] + p[j]) for j in arms if frozenset((i, j)) in n)
            new[i] = w[i] / den if den else p[i]
        g = math.exp(statistics.mean(math.log(x) for x in new.values()))
        new = {i: x / g for i, x in new.items()}
        done = max(abs(math.log(new[i] / p[i])) for i in arms) < 1e-12
        p = new
        if done:
            break
    return p


def primary_scores(batch):
    """{run: {item: [score per judge]}} from a primary judging batch."""
    batch = pathlib.Path(batch)
    key = json.load(open(batch / "key.json"))
    out = {}
    for pid, run in key.items():
        files = sorted((batch / "verdicts").glob(f"{pid}.json")) + sorted((batch / "verdicts").glob(f"{pid}-*.json"))
        per = collections.defaultdict(list)
        for f in files:
            for it in json.load(open(f))["items"]:
                if it.get("statement") == "wrong" or it.get("derivation") == "wrong":
                    s = 0
                else:
                    s = {"absent": 0, "mentioned": 1, "stated": 2, "derived": 3}.get(it.get("presence"), 0)
                    s += s == 3 and it.get("derivation") == "faithful"
                per[it["id"]].append(s)
        if per:
            out[run] = dict(per)
    return out


def score(a):
    out = pathlib.Path(a.out)
    key = json.load(open(out / "key.json"))
    pack = out / "pack.md"
    expected_items = (re.findall(r"^## (\S+) \(", pack.read_text(), flags=re.M) if pack.exists() else []) + ["chain"]
    verdicts, problems = load_verdicts(out, key, expected_items)
    print(f"verdicts read: {len(verdicts)}; files expected: {sum(e['judges'] for e in key['pairs'].values())}")
    for p in problems:
        print(f"  problem: {p}")
    if not verdicts:
        return

    def unit(v):
        return tuple(sorted((v["A"], v["B"])))
    identical = [v for v in verdicts if v["A"] == v["B"]]
    different = [v for v in verdicts if v["A"] != v["B"]]
    by_unit = collections.defaultdict(list)
    for v in different:
        by_unit[unit(v)].append(v)

    # the preference matrix
    names = sorted({n for v in different for n in (v["A"], v["B"])})
    wins, ties = collections.Counter(), collections.Counter()
    for v in different:
        w = name_of(v, v["overall"])
        if w is None:
            ties[unit(v)] += 1
        else:
            wins[(w, v["B"] if w == v["A"] else v["A"])] += 1
    if names:
        width = max(10, max(map(len, names)) + 1)
        print("\npreference matrix (overall verdicts): row's wins over column / ties")
        print(" " * width + "".join(f"{n:>{width}}" for n in names))
        for r in names:
            cells = []
            for c in names:
                u = tuple(sorted((r, c)))
                cells.append(f"{'':>{width}}" if r == c or u not in by_unit else f"{f'{wins[(r, c)]}/{ties[u]}':>{width}}")
            print(f"{r:<{width}}" + "".join(cells))

    # expected orderings and the sign test
    if a.expect:
        print(f"\nexpected orderings (a pair counts when |net| >= {a.margin}):")
        counted = []
        for spec in a.expect:
            x, y = spec.split(">")
            vs = by_unit.get(tuple(sorted((x, y))), [])
            if not vs:
                print(f"  {x} > {y}: no verdicts")
                continue
            fx = sum(name_of(v, v["overall"]) == x for v in vs)
            fy = sum(name_of(v, v["overall"]) == y for v in vs)
            clear = sum(name_of(v, v["overall"]) == x and v["strength"] == "clear" for v in vs)
            o1, o2 = split_orders(tuple(sorted((x, y))), vs)
            reversals = compare_couples([(v1, v2) for v1 in o1 for v2 in o2])[0]["reversal"]
            items_x = statistics.mean(sum(name_of(v, l) == x for l in v["items"].values()) for v in vs)
            items_y = statistics.mean(sum(name_of(v, l) == y for l in v["items"].values()) for v in vs)
            net = fx - fy
            counted.append(net)
            print(f"  {x} > {y}: verdicts for {x} {fx} ({clear} clear), for {y} {fy}, tied {len(vs) - fx - fy} "
                  f"of {len(vs)}; net {net:+d}; verdicts per order {len(o1)}, {len(o2)}; "
                  f"reversals across orders {reversals}; items per verdict: {x} {items_x:.1f}, {y} {items_y:.1f}")
        k = sum(n >= a.margin for n in counted)
        m = sum(n <= -a.margin for n in counted)
        print(f"  sign test over pairs: expected direction in {k} of {k + m} untied ({len(counted) - k - m} tied), "
              f"one-sided p = {binom_tail(k, k + m):.3f}")
        allx = sum(name_of(v, v["overall"]) == spec.split(">")[0] for spec in a.expect
                   for v in by_unit.get(tuple(sorted(spec.split(">"))), []))
        ally = sum(name_of(v, v["overall"]) == spec.split(">")[1] for spec in a.expect
                   for v in by_unit.get(tuple(sorted(spec.split(">"))), []))
        print(f"  pooled verdicts (not independent; reported only): expected {allx}, reversed {ally}")

    # position bias
    print("\nposition bias:")
    nt = [v["overall"] for v in different if v["overall"] != "tie"]
    ka = nt.count("A")
    print(f"  different plans, overall: position A in {ka} of {len(nt)} non-tie verdicts "
          f"({ka / len(nt) if nt else float('nan'):.2f}); two-sided p = {two_sided(ka, len(nt)):.3f}")
    ni = [l for v in different for l in v["items"].values() if l != "tie"]
    kia = ni.count("A")
    print(f"  different plans, items: position A in {kia} of {len(ni)} non-tie item verdicts "
          f"({kia / len(ni) if ni else float('nan'):.2f}); two-sided p = {two_sided(kia, len(ni)):.3f} (items are not independent)")
    if identical:
        fo = [v for v in identical if v["overall"] != "tie"]
        fi = [l for v in identical for l in v["items"].values() if l != "tie"]
        ni_all = sum(len(v["items"]) for v in identical)
        print(f"  identical plans, overall: {len(fo)} false preferences of {len(identical)} verdicts "
              f"(A {sum(v['overall'] == 'A' for v in fo)}, B {sum(v['overall'] == 'B' for v in fo)}; "
              f"clear {sum(v['strength'] == 'clear' for v in fo)}); files {sorted({v['qid'] for v in identical})}")
        print(f"  identical plans, items: {len(fi)} false preferences of {ni_all} item verdicts "
              f"(A {fi.count('A')}, B {fi.count('B')})")

    # swap consistency and judge-to-judge agreement
    cross, within, cross_i, within_i = (collections.Counter() for _ in range(4))
    kap, kap_i, per_unit = [], [], []
    for u, vs in sorted(by_unit.items()):
        o1, o2 = split_orders(u, vs)
        c, ci = compare_couples([(v1, v2) for v1 in o1 for v2 in o2])
        cross += c
        cross_i += ci
        same = [(g[i1], g[i2]) for g in (o1, o2) for i1 in range(len(g)) for i2 in range(i1 + 1, len(g))]
        c, ci = compare_couples(same)
        within += c
        within_i += ci
        for v1, v2 in same:
            kap.append((v1["overall"], v2["overall"]))
            kap_i += [(v1["items"][i], v2["items"][i]) for i in set(v1["items"]) & set(v2["items"])]
        if o1 and o2:
            per_unit.append(f"{u[0]}:{u[1]} overall {share(compare_couples([(v1, v2) for v1 in o1 for v2 in o2])[0])}")

    print("\nswap consistency (couples across the two orders of a pair, mapped to plan names):")
    print(f"  overall: {share(cross)}")
    print(f"  items:   {share(cross_i)}")
    for line in per_unit:
        print(f"    {line}")
    print("judge-to-judge agreement (couples within one pair file, different plans):")
    print(f"  overall: {share(within)}")
    print(f"  items:   {share(within_i)}")
    k1, k2 = kappa(kap), kappa(kap_i)
    print(f"  Cohen's kappa on A/B/tie: overall {'-' if k1 is None else f'{k1:.2f}'}, "
          f"items {'-' if k2 is None else f'{k2:.2f}'}")
    groups = collections.defaultdict(list)
    for v in identical:
        groups[v["qid"]].append(v)
    same = [(g[i1], g[i2]) for g in groups.values() for i1 in range(len(g)) for i2 in range(i1 + 1, len(g))]
    if same:
        print(f"  identical-pair files: overall labels equal in {sum(x['overall'] == y['overall'] for x, y in same)} "
              f"of {len(same)} couples")

    # agreement with the primary verdicts, item by item
    if a.primary:
        batches = [(b, primary_scores(b)) for b in a.primary]
        print("\nitems against the primary verdicts (same primary batch for both plans):")
        for u, vs in sorted(by_unit.items()):
            src = next(((b, s) for b, s in batches if u[0] in s and u[1] in s), None)
            if src is None:
                print(f"  {u[0]}:{u[1]}: no primary batch holds both")
                continue
            b, s = src
            tally, beyond = collections.Counter(), []
            for item in sorted(set(s[u[0]]) & set(s[u[1]])):
                p, q = s[u[0]][item], s[u[1]][item]
                labels = [name_of(v, v["items"][item]) for v in vs if item in v["items"]]
                net = sum(n == u[0] for n in labels) - sum(n == u[1] for n in labels)
                if min(p) > max(q) or min(q) > max(p):
                    better = u[0] if min(p) > max(q) else u[1]
                    sign = net if better == u[0] else -net
                    tally["agree" if sign > 0 else "tie" if sign == 0 else "reverse"] += 1
                else:
                    pref = [n for n in labels if n is not None]
                    if len(pref) >= 2 and len(set(pref)) == 1:
                        beyond.append(f"{item}->{pref[0]} ({len(pref)} of {len(labels)})")
            n = sum(tally.values())
            print(f"  {u[0]}:{u[1]} ({pathlib.Path(b).name if pathlib.Path(b).name != 'judge' else pathlib.Path(b).parent.name}): "
                  f"{n} items differ beyond noise; pairwise majority agrees {tally['agree']}, ties {tally['tie']}, "
                  f"reverses {tally['reverse']}" + (f" ({tally['agree'] / n:.2f})" if n else ""))
            if beyond:
                print(f"    tied in the primary, consistent pairwise preference: {', '.join(beyond)}")

    # Bradley-Terry
    arm_of = {}
    for spec in a.arm or []:
        arm, members = spec.split("=", 1)
        for m in members.split(","):
            arm_of[m] = arm
    units, within_arm = [], 0
    for u, vs in sorted(by_unit.items()):
        comps = []
        for v in vs:
            i, j = arm_of.get(v["A"], v["A"]), arm_of.get(v["B"], v["B"])
            if i == j:
                within_arm += 1
                continue
            s = {"A": 1.0, "B": 0.0, "tie": 0.5}[v["overall"]]
            comps.append((i, j, s))
        if comps:
            units.append(comps)
    arms = sorted({x for unit in units for i, j, _ in unit for x in (i, j)})
    if len(arms) >= 2:
        p = bt_fit(units, arms)
        rng = random.Random(a.seed)
        boots = [bt_fit([rng.choice(units) for _ in units], arms) for _ in range(a.boot)] if len(units) > 1 else []
        print(f"\nBradley-Terry over {'arms' if arm_of else 'plans'} ({len(units)} pairs as units; "
              f"{within_arm} within-arm verdicts left out; bootstrap {len(boots)}):")
        for i in sorted(arms, key=lambda x: -p[x]):
            print(f"  {i:12} log-strength {math.log(p[i]):+.2f}")
        if len(arms) <= 6:
            for x in range(len(arms)):
                for y in range(x + 1, len(arms)):
                    i, j = arms[x], arms[y]
                    lr = math.log(p[i] / p[j])
                    line = f"  {i} vs {j}: log ratio {lr:+.2f}, P({i} preferred) {p[i] / (p[i] + p[j]):.2f}"
                    if boots:
                        rs = sorted(math.log(b[i] / b[j]) for b in boots if i in b and j in b)
                        lo, hi = rs[int(0.025 * (len(rs) - 1))], rs[int(0.975 * (len(rs) - 1))]
                        line += f"; bootstrap 95% [{lo:+.2f}, {hi:+.2f}]"
                    print(line)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    sub = ap.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("prepare")
    p.add_argument("paper")
    p.add_argument("out")
    p.add_argument("seed")
    p.add_argument("plans", nargs="+", metavar="NAME=ARGUMENT.json")
    p.add_argument("--pair", action="append", required=True, metavar="X:Y[:J]")
    p.add_argument("--drop", action="append", metavar="NEW=BASE:STEP[,STEP]")
    p.add_argument("--orders", choices=("both", "one"), default="both")
    p.add_argument("--reference")
    p.add_argument("--prompt")
    p.add_argument("--checklist")
    s = sub.add_parser("score")
    s.add_argument("out")
    s.add_argument("--expect", action="append", metavar="X>Y")
    s.add_argument("--margin", type=int, default=1)
    s.add_argument("--arm", action="append", metavar="ARM=NAME,NAME")
    s.add_argument("--primary", action="append", metavar="BATCH_DIR")
    s.add_argument("--boot", type=int, default=2000)
    s.add_argument("--seed", type=int, default=0)
    a = ap.parse_args()
    (prepare if a.cmd == "prepare" else score)(a)


if __name__ == "__main__":
    main()

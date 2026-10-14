"""Planted-error recall for exegenius's fidelity auditing (calibration item A1, RUNBOOK §9 P2).

Seeds known errors into fixed wiki pages, writes the seeded pages and a JSON answer key, and
scores the pipeline's own auditors against the key. The auditors are the two task kinds that
`policy.md` calls fidelity auditing:

- `audit` (exegenius-core/src/audit.rs): a reader stand-in that reads only its task file and
  reports the terms and steps the reader cannot follow, each with a quote copied from the page.
  It sees citation marks as "(cited)", never the quotation.
- `support` (exegenius-core/src/support.rs): labels each cited claim `supported`, `partial`,
  `unsupported` or `contradicted` from its quotation alone.

Neither task compares a page with the source, so each planted error is tagged with what could
reveal it (`detectable_by`): `page` (the page itself, with what the reader profile covers),
`quote` (the claim's own quotation, so only `support`), or `source` (only the paper: a blind
spot of both tasks, reported, never scored against a configuration).

usage:
  python3 -I plant.py fetch --repo WIKI_REPO --out DIR
      read the chosen pages, their bound symbols and their support labels from the wiki
      repository's commits (read-only `git show`), into DIR
  python3 -I plant.py seed --clean DIR --out DIR --seed S [--errors N] [--variants V]
                           [--paper PAPER.tex]
      write DIR/pages/<code>.md (one clean control and V seeded variants per page, each with
      N errors and one sham edit) and DIR/key.json
  python3 -I plant.py score --key KEY.json --answers DIR [--json OUT.json] [--notes OUT.tsv]
      score collected auditor answers: DIR/<config>/<run>/<code>/ holding audit.answer.json,
      support.answer.json and, if kept, support.task.md and log.jsonl
  python3 -I plant.py selftest --key KEY.json
      check the scorer with synthetic auditors (oracle, silent, alarmist) and the seeding's
      determinism

Selection is deterministic: the same clean pages, seed, N and V give byte-identical output.
"""

import argparse
import collections
import hashlib
import json
import math
import pathlib
import random
import re
import subprocess
import sys

VERSION = "a1-plant/1"

# ---------------------------------------------------------------------------------------------
# The error taxonomy (fixed before any run).

TYPES = {
    "constant": "a wrong number, coefficient, exponent, index or computed value",
    "inequality": "an inequality reversed, or its strictness changed, so the stated bound is false",
    "hypothesis": "a condition the claim or step needs is removed from its statement",
    "cases": "a case analysis or enumeration with a case dropped, or a wrong count of its "
             "cases, steps or terms",
    "generalisation": "a claim widened beyond what holds (a quantifier or class enlarged)",
    "sign": "a sign or an operation reversed (plus and minus, add and subtract)",
    "attribution": "a citation whose quotation is a real sentence of the paper that does not "
                   "support the claim it is attached to",
}
TYPE_ORDER = list(TYPES)

# ---------------------------------------------------------------------------------------------
# The pages: four to six pages that carry derivations of the paper's key step, the central
# saving s<Wm (calibration/rubrics/nlogn-central-saving.md, steps C1-C9). W1 and W2 are the
# blinded names of journal 2026-10-13-chain-walk-baseline.

WIKIS = {
    "W1": {"commit": "72f1de8", "branch": "full-v0.6-latex-planner-handoff"},
    "W2": {"commit": "fa33c9d", "branch": "full-v0.6-latex-depth (no ref left; commit only)"},
}

PAGES = {
    "subset-intersection-construction": {
        "wiki": "W1", "plan": ["S1.4"], "chain": ["C1", "C3", "C4", "C5", "C6"],
        "native": [("Sources have dimension $1$, summing to $N$",
                    "only the X_a sources have dimension 1; the sum N is right (chain walk W1, C5)")],
    },
    "address-shear-factorization": {
        "wiki": "W1", "plan": ["S1.2.1"], "chain": ["C8", "C5", "C6"],
        "native": [],
    },
    "fast-address-interchange": {
        "wiki": "W1", "plan": ["S1.2"], "chain": ["C9", "C6"],
        "native": [("for the bit procedure this ring is $\\mathbb F_2$",
                    "fields are residues mod q^b, q an odd prime (paper 1397-1404); chain walk W1-b, C7"),
                   ("Over $\\mathbb F_2$ each update is a pointwise XOR",
                    "same native error: only the data gates are XOR")],
    },
    "linear-network-roles": {
        "wiki": "W1", "plan": ["S1.1"], "chain": ["C2", "C7"],
        "native": [],
    },
    "recursive-power-saving": {
        "wiki": "W1", "plan": ["S1.5"], "chain": ["C9"],
        "native": [("the leaves cost $O(f^\\sigma d^{\\vartheta(1-\\sigma)})\\le O(f^\\lambda)$",
                    "does not follow: with d<=f^(1/theta) it gives only O(f) (journal "
                    "2026-10-13-planner-handoff-run)")],
    },
    "tape-interchange": {
        "wiki": "W2", "plan": ["S7.1"], "chain": ["C9", "C6", "C8"],
        "native": [("The shear is such an invocation, with $D$ as $X$ and $H$ as $Y$",
                    "the shear comes from the endpoint frames, not from treating the fields as "
                    "the banks (chain walk W2, C7)")],
    },
}

# ---------------------------------------------------------------------------------------------
# The catalogue of candidate errors. Each is a single replacement of a string that occurs once
# on its clean page. `why` says why the seeded text is wrong, with the paper's lines; `cues` are
# strings a note that names the error is likely to contain; `site` (default: the seeded text)
# is text visible to the auditor that locates the error; `linked` are other sentences where the
# inconsistency shows; `conflicts` are errors never seeded on the same variant.

def E(id, page, type, detect, find, replace, why, lines, cues, site=None, linked=(),
      conflicts=()):
    return {"id": id, "page": page, "type": type, "detectable_by": list(detect), "find": find,
            "replace": replace, "why": why, "paper_lines": lines, "cues": list(cues),
            "site": site, "linked": list(linked), "conflicts": list(conflicts)}


CATALOGUE = [
    # subset-intersection-construction (W1): C1, C3-C6
    E("SIC-1", "subset-intersection-construction", "constant", ["page"],
      r"the bilinear form with matrix $I-J/9$", r"the bilinear form with matrix $I-J/3$",
      "t_T^T J t_S = 9 for triples, so I-J/3 gives |T∩S|-3, not the |T∩S|-1 the page states",
      "858-864", ["J/3", "J/9", "-3"]),
    E("SIC-2", "subset-intersection-construction", "sign", ["page"],
      r"$\langle t_T,t_S\rangle=|T\cap S|-1$; neighbors",
      r"$\langle t_T,t_S\rangle=|T\cap S|+1$; neighbors",
      "with +1, neighbours (|T∩S|=1) have product 2, so they are not orthogonal; I-J/9 gives -1",
      "862-866", ["+1", "-1", "= 2", "=2"]),
    E("SIC-3", "subset-intersection-construction", "generalisation", ["page"],
      r"neighbors are orthogonal.", r"distinct triples are orthogonal.",
      "by |T∩S|-1, only triples meeting in one element are orthogonal; disjoint ones give -1",
      "862-868", ["distinct", "disjoint", "-1", "only neighbors", "only neighbours"]),
    E("SIC-4", "subset-intersection-construction", "hypothesis", ["page"],
      r"If $U\subseteq V$, then $P_V-P_U$ is", r"For any labels $U,V$, $P_V-P_U$ is",
      "the decomposition and rank dim V - dim U need nested labels U⊆V; for labels that are not "
      "nested, P_V-P_U is not a projection and its rank is not the dimension change",
      "1157-1160", ["nested", "subset", "subseteq", "contain", "comparable"]),
    E("SIC-5", "subset-intersection-construction", "constant", ["page"],
      r"Each of the $3\binom h3^2$ invocations", r"Each of the $2\binom h3^2$ invocations",
      "the paper has I=3v^2 invocations; the page's own L=3 binom(h,3)^2 h^2 then no longer "
      "follows from the count it gives", "672-676, 1000-1006",
      ["2\\binom", "3\\binom", "2 binom", "3 binom", "invocations"],
      linked=[r"so $L=3\binom h3^2h^2$"]),
    E("SIC-6", "subset-intersection-construction", "cases", ["page"],
      r"is $0$ for sizes $0,1,2$ and $1$ for $T=S$",
      r"is $0$ for sizes $0$ and $2$ and $1$ for $T=S$",
      "intersection size 1 is dropped from the case list; the conclusion that only g_S remains "
      "needs sizes 0, 1 and 2 all to vanish (size 1 vanishes only through the auxiliary wire)",
      "700-783", ["size 1", "sizes 0", "|S\\cap T|=1", "intersection 1", "case"],
      linked=[r"so only $g_S$ remains"]),
    E("SIC-7", "subset-intersection-construction", "inequality", ["page"],
      r"so $2L<N$ and $s<WD$", r"so $2L>N$ and $s<WD$",
      "L/N=100/539 < 1/2 gives 2L<N, and s=WD-N+2L<WD needs it", "1105-1110, 1166",
      ["2L>N", "2L<N", "1/2"], conflicts=["SIC-8"]),
    E("SIC-8", "subset-intersection-construction", "constant", ["page"],
      r"\binom h3=100/539$", r"\binom h3=110/539$",
      "3h^2/binom(h,3) = 30000/161700 = 100/539 with h=100", "1105-1108",
      ["110/539", "100/539", "30000", "161700"], conflicts=["SIC-7"]),
    E("SIC-9", "subset-intersection-construction", "sign", ["page"],
      r"the matrix $-P_{U_\iota}$", r"the matrix $P_{U_\iota}$",
      "the first edge's matrix 2P_{U_ι} is P_{U_ι}-(-P_{U_ι}); with source matrix +P_{U_ι} it "
      "would be 0, and the endpoint difference would not be the identity",
      "1115, 1146, 1169", ["-P", "2P", "+P", "minus", "negative"],
      linked=[r"The first edge out of each source has matrix $2P_{U_\iota}$"]),
    E("SIC-10", "subset-intersection-construction", "attribution", ["quote"],
      r'<cite data-source="paper">smaller calls satisfies $s<Wm$</cite>',
      r'<cite data-source="paper">Every displayed summand is nondegenerate.</cite>',
      "the quotation (paper 990) is about the label sums' nondegeneracy, not the number of "
      "smaller calls", "168 (true), 990 (seeded)", ["nondegenerate", "summand"],
      site="The paper's decisive property is a bound on the number of smaller calls"),
    # address-shear-factorization (W1): C8, with C5-C6 restated
    E("ASF-1", "address-shear-factorization", "hypothesis", ["page"],
      r"where $F_1$ and $F_2$ are invertible lower-triangular rational matrices",
      r"where $F_1$ and $F_2$ are lower-triangular rational matrices",
      "the procedure applies F_1^{-1} and F_2^{-1}; a lower-triangular matrix need not be "
      "invertible", "1366-1373", ["invertible", "inverse", "singular", "F_1^{-1}"],
      linked=[r"replace $D$ by $F_2D$ and $H$ by $F_1^{-1}H$"]),
    E("ASF-2", "address-shear-factorization", "cases", ["page"],
      r"takes one interchange of two equal fields and two additions",
      r"takes two interchanges of two equal fields and one addition",
      "the sequence the page gives has one swap and two additive updates; the count of "
      "interchanges also feeds rank_Q M interchanges for the shear", "1436-1446",
      ["two interchanges", "one interchange", "one swap", "two swaps"],
      linked=[r"So the shear needs $\operatorname{rank}_{\mathbb Q}M$ interchanges"]),
    E("ASF-3", "address-shear-factorization", "sign", ["page"],
      r"then set $D_\tau\leftarrow H_\sigma-D_\tau$", r"then set $D_\tau\leftarrow D_\tau-H_\sigma$",
      "with D_τ ← D_τ - H_σ the pair (η,ζ) ends as (η+ζ,-ζ), not the (η+ζ,ζ) the page states",
      "1436-1440", ["-\\zeta", "-ζ", "D_\\tau-H_\\sigma", "H_\\sigma-D_\\tau", "sign"],
      linked=[r"becomes $(\eta+\zeta,\zeta)$"]),
    E("ASF-4", "address-shear-factorization", "constant", ["page"],
      r"$F_2=\begin{pmatrix}1&0\\1&1\end{pmatrix}$", r"$F_2=\begin{pmatrix}1&0\\2&1\end{pmatrix}$",
      "with this F_2, F_1 Π F_2 = [[2,1],[0,0]], not M = [[1,1],[0,0]]", "1366-1373 (example)",
      ["2&1", "[[2", "(2,1)", "2,1", "not equal", "≠"]),
    E("ASF-5", "address-shear-factorization", "inequality", ["page"],
      r"The paper shows $N>2L_{\rm b}$, so", r"The paper shows $N<2L_{\rm b}$, so",
      "with s_b = W_bR - N + 2L_b, N<2L_b gives s_b > W_bR; the paper's deficit is N-2L_b>0",
      "1166-1167", ["N<2L", "N>2L", "2L_b", "deficit"]),
    E("ASF-6", "address-shear-factorization", "generalisation", ["page", "quote"],
      r"A lower-triangular transformation costs $O(V)$ <cite",
      r"Any invertible linear transformation of $H$ or $D$ costs $O(V)$ <cite",
      "only triangular transformations run in place in linear time (each coordinate combines "
      "earlier, unchanged ones); if every invertible linear map of the fields cost O(V), the "
      "shear itself would, and the recursion would be pointless", "1427-1433",
      ["triangular", "any invertible", "any linear", "every linear"], conflicts=["ASF-7"]),
    E("ASF-7", "address-shear-factorization", "attribution", ["quote"],
      r'<cite data-source="paper">A lower triangular transformation of $H$, or of $D$, costs $O(V)$</cite>',
      r'<cite data-source="paper">It uses just one interchange.</cite>',
      "the quotation (paper 1443) is about one update's single interchange, not the cost of a "
      "triangular transformation", "1427 (true), 1443 (seeded)", ["interchange"],
      site="A lower-triangular transformation costs $O(V)$", conflicts=["ASF-6"]),
    E("ASF-8", "address-shear-factorization", "constant", ["page"],
      r"So the shear needs $\operatorname{rank}_{\mathbb Q}M$ interchanges",
      r"So the shear needs $2\operatorname{rank}_{\mathbb Q}M$ interchanges",
      "each 1 of Π takes one interchange and Π has rank_Q M ones, so rank_Q M interchanges",
      "1443-1446", ["2\\operatorname", "2 rank", "twice", "one interchange"],
      conflicts=["ASF-2"]),
    # fast-address-interchange (W1): C9, with C6 restated
    E("FAI-1", "fast-address-interchange", "cases", ["page"],
      r"Swapping takes three updates, applied componentwise:",
      r"Swapping takes two updates, applied componentwise:",
      "the page then displays three updates, as the paper does", "1530-1536",
      ["three", "two updates", "3 updates"]),
    E("FAI-2", "fast-address-interchange", "constant", ["page"],
      r"the states are $(1,1)$, $(0,1)$, $(0,1)$, which is $(D,H)$",
      r"the states are $(1,1)$, $(0,1)$, $(1,0)$, which is $(D,H)$",
      "from (0,1), D←H-D gives (0,1); and (D,H) for (H,D)=(1,0) is (0,1), not (1,0)",
      "1530-1536 (example)", ["(1,0)", "(0,1)"]),
    E("FAI-3", "fast-address-interchange", "inequality", ["page"],
      r"fix $\tau<1$ with $s/W<m^\tau$", r"fix $\tau<1$ with $s/W>m^\tau$",
      "the bound needs s/W<m^τ, which the next paragraph uses", "1482, 1554",
      ["s/W>m", "s/W<m", "m^\\tau"], linked=[r"because $s/W<m^\tau$"]),
    E("FAI-4", "fast-address-interchange", "cases", ["page"],
      r"The geometric sum is also $O(m^{k\tau})$: if $s/W\le1$ it is at most $k$, and if $s/W>1$ it is $O((s/W)^k)$.",
      r"The geometric sum is also $O(m^{k\tau})$, since it is $O((s/W)^k)$.",
      "the case s/W≤1 is dropped: then (s/W)^k→0 while the sum is at least 1, so the sum is not "
      "O((s/W)^k); the paper bounds it by k+1 in that case", "1554-1556",
      ["s/W\\le1", "s/W<1", "s/W ≤ 1", "at most k", "case"]),
    E("FAI-5", "fast-address-interchange", "constant", ["quote"],
      r"For the bit network, $s<Wm$ <cite", r"For the bit network, $s<W$ <cite",
      "the bound is s<Wm (the quotation); s_b is W_b m(1-η) with η≈1.5e-11, far above W_b",
      "168, 1150", ["s<W", "Wm"], conflicts=["FAI-6"]),
    E("FAI-6", "fast-address-interchange", "generalisation", ["quote"],
      r"For the bit network, $s<Wm$ <cite", r"For every linear network, $s<Wm$ <cite",
      "s<Wm is a property of the paper's construction, not of every network",
      "168, 1150", ["every", "construction", "this network"], conflicts=["FAI-5"]),
    E("FAI-7", "fast-address-interchange", "attribution", ["quote"],
      r'<cite data-source="paper">for a fixed $\tau<1$</cite>',
      r'<cite data-source="paper">contains $V/W$ bits</cite>',
      "the quotation (paper 128) is about role-stream volume, not the cost exponent",
      "97 (true), 128 (seeded)", ["V/W", "volume"],
      site="Here $\\tau$, the fixed exponent below 1 in the cost"),
    E("FAI-8", "fast-address-interchange", "sign", ["page"],
      r"Unrolling gives $Q_k\le (s/W)^k Q_0+C\sum_{j<k}(s/W)^j$",
      r"Unrolling gives $Q_k\le (s/W)^k Q_0-C\sum_{j<k}(s/W)^j$",
      "unrolling Q_k ≤ (s/W)Q_{k-1}+C adds the outside work at every level", "1549-1556",
      ["-C", "+C", "minus", "sign"]),
    # linear-network-roles (W1): C2, C7
    E("LNR-1", "linear-network-roles", "constant", ["page", "quote"],
      r"Each role stream then holds $V/W$ bits <cite", r"Each role stream then holds $V/(2W)$ bits <cite",
      "V padded bits split cyclically over W roles give V/W per role (the quotation)", "126-128",
      ["V/(2W)", "V/W", "2W", "half"]),
    E("LNR-2", "linear-network-roles", "sign", ["page", "quote"],
      r"has $\rho(w)=w$ with sign $+1$", r"has $\rho(w)=w$ with sign $-1$",
      "scratch inputs go to their own outputs with sign +1 (the quotation); with -1 the undoing "
      "would need a correction, which the page says it does not", "1275-1277",
      ["-1", "+1", "sign"], linked=[r"so the undoing needs no correction there"],
      conflicts=["LNR-3"]),
    E("LNR-3", "linear-network-roles", "generalisation", ["page", "quote"],
      r"For a scratch role the construction has", r"For every role the construction has",
      "the network exchanges the two banks (ρ(X_a)=Y_a), as the page's first sentence says; only "
      "scratch roles are fixed", "1115-1117, 1275-1277",
      ["every role", "scratch", "exchange", "bank"], conflicts=["LNR-2"]),
    E("LNR-4", "linear-network-roles", "generalisation", ["page"],
      r"Frames therefore pass through gates.", r"Frames therefore pass through every gate, linear or not.",
      "the commutation G(D f_1,...)=D G(f_1,...) uses the gate's linearity and pointwise action",
      "136-139", ["linear", "nonlinear", "not linear", "or not"]),
    E("LNR-5", "linear-network-roles", "hypothesis", ["page"],
      r"carries an invertible linear operator $D_\nu$", r"carries a linear operator $D_\nu$",
      "edges apply D_β D_α^{-1}, so frames must be invertible", "130-134",
      ["invertible", "inverse", "D_\\alpha^{-1}"],
      linked=[r"the stored array changes by $D_\beta D_\alpha^{-1}$"]),
    E("LNR-6", "linear-network-roles", "constant", ["page"],
      r"=D_{\alpha_k}D_{\alpha_0}^{-1}$", r"=D_{\alpha_k}D_{\alpha_1}^{-1}$",
      "the product telescopes to D_{α_k} D_{α_0}^{-1}; the last factor is D_{α_1}D_{α_0}^{-1}",
      "140-145", ["alpha_1", "alpha_0", "α_1", "α_0", "telescop"]),
    E("LNR-7", "linear-network-roles", "sign", ["page"],
      r"and multiplying by $\varepsilon_w$ again removes the sign",
      r"and multiplying by $-\varepsilon_w$ again removes the sign",
      "ε_w^2=1 removes the sign; -ε_w·ε_w=-1 flips it", "143-147",
      ["-\\varepsilon", "-ε", "varepsilon_w^2", "ε_w^2", "sign"]),
    E("LNR-8", "linear-network-roles", "attribution", ["quote"],
      r'<cite data-source="paper">leaves the endpoint operator on every role array</cite>',
      r'<cite data-source="paper">Every displayed summand is nondegenerate.</cite>',
      "the quotation (paper 990) is about label sums, not the endpoint operator",
      "147 (true), 990 (seeded)", ["nondegenerate", "summand"],
      site="Every role keeps the endpoint operator"),
    # recursive-power-saving (W1): C9 in general form; no mark here is support-checked
    E("RPS-1", "recursive-power-saving", "sign", ["page"],
      r"\frac{s}{W}\Phi(f)+C(mf)^\lambda.$$", r"\frac{s}{W}\Phi(f)-C(mf)^\lambda.$$",
      "the call's own outside work adds to the subcalls' cost", "1549-1553",
      ["-C(mf)", "+C(mf)", "minus", "sign", "subtract"]),
    E("RPS-2", "recursive-power-saving", "inequality", ["page"],
      r"Taking $\lambda$ with $\log_m(s/W)<\lambda<1$", r"Taking $\lambda$ with $\lambda<\log_m(s/W)<1$",
      "the ratio (s/W)/m^λ is below 1 only when λ > log_m(s/W)", "1482, 1554",
      ["\\lambda<\\log", "λ<log", "\\log_m(s/W)<\\lambda", "greater than", "above 1"],
      conflicts=["RPS-5"]),
    E("RPS-3", "recursive-power-saving", "hypothesis", ["page"],
      r"Since $\vartheta=\tfrac12$ is a fixed exponent and $f\ge d^\vartheta$ at an internal node, $d\le f^{1/\vartheta}$",
      r"Since $\vartheta=\tfrac12$ is a fixed exponent, $d\le f^{1/\vartheta}$",
      "d ≤ f^{1/ϑ} follows from f ≥ d^ϑ at an internal node, not from ϑ being fixed",
      "(the page's own parameters)", ["internal node", "f\\ge d", "f ≥ d", "d^\\vartheta"]),
    E("RPS-4", "recursive-power-saving", "constant", ["page"],
      r"\approx0.86$", r"\approx1.16$",
      "2^0.8 ≈ 1.741, so (3/2)/2^0.8 ≈ 0.862; with λ=0.8 > log_2(3/2) the ratio is below 1",
      "(the page's example)", ["1.16", "0.86", "below 1", "1.74"]),
    E("RPS-5", "recursive-power-saving", "generalisation", ["page"],
      r"so the internal sum is at most a constant times $f^\lambda$.",
      r"so the internal sum is at most a constant times $f^\lambda$ for every $\lambda>0$.",
      "the geometric sum is bounded only when the ratio is below 1, that is λ > log_m(s/W)",
      "1554", ["every", "\\lambda>0", "λ>0", "any"], conflicts=["RPS-2"]),
    E("RPS-6", "recursive-power-saving", "constant", ["page"],
      r"$\log_2(3/2)\approx0.585$", r"$\log_2(3/2)\approx0.415$",
      "log_2(1.5) ≈ 0.585 (0.415 is log_2(4/3))", "(the page's example)",
      ["0.415", "0.585", "4/3"]),
    E("RPS-7", "recursive-power-saving", "inequality", ["page"],
      r"The saving exists because $s/W<m$.", r"The saving exists because $s/W>m$.",
      "the construction gives s<Wm, which the page states above; s/W>m gives no saving",
      "168, 1166", ["s/W>m", "s/W<m", "s<Wm"]),
    # tape-interchange (W2): C9 with the explicit exponent, C6, C8
    E("TI-1", "tape-interchange", "constant", ["page"],
      r"Take $\tau=1-2^{-50}$.", r"Take $\tau=1-2^{-40}$.",
      "the chain on the page shows λ < μ^{1-2^{-50}}; λ < μ^{1-2^{-40}} would need "
      "η > 2^{-40} ln μ, which η > 20·2^{-50} does not give", "1327-1340",
      ["2^{-40}", "2^{-50}", "1-2^"], conflicts=["TI-7"]),
    E("TI-2", "tape-interchange", "inequality", ["page"],
      r"=\exp(-2^{-50}\ln\mu)>1-20\cdot2^{-50}", r"=\exp(-2^{-50}\ln\mu)<1-20\cdot2^{-50}",
      "e^{-x} > 1-x, and x = 2^{-50} ln μ < 20·2^{-50}; reversed, the chain to λ<μ^τ breaks",
      "1333-1336", ["e^{-x}", "1-x", "\\exp", "reversed", "<1-20"], conflicts=["TI-3"]),
    E("TI-3", "tape-interchange", "constant", ["source"],
      r"Since $\mu<2^{20}$, $\ln\mu<20$", r"Since $\mu<2^{19}$, $\ln\mu<20$",
      "μ is the paper's m = h^3 = 10^6 > 2^19 = 524288; the page never states μ's value, so "
      "only the source reveals it", "655, 1331", ["2^{19}", "2^{20}", "10^6", "524288"],
      conflicts=["TI-2"]),
    E("TI-4", "tape-interchange", "cases", ["page"],
      r"If $\lambda\le1$ the sum is at most $d+1$.", r"If $\lambda\le1$ the sum is at most $d$.",
      "1+λ+...+λ^d has d+1 terms, and equals d+1 at λ=1", "1555",
      ["d+1", "terms", "at most d"]),
    E("TI-5", "tape-interchange", "hypothesis", ["page"],
      r"The bound needs $\lambda<\mu^{\tau}$ for one fixed $\tau<1$, not merely $\lambda<\mu$.",
      r"The bound needs only $\lambda<\mu$.",
      "with only λ<μ, F_d = O(λ^d) is O(μ^d) = O(u): no saving; the page's conclusion uses "
      "λ^d < μ^{dτ}", "1482, 1554", ["\\mu^{\\tau}", "μ^τ", "only", "merely", "not enough"],
      linked=[r"and $\lambda^d<\mu^{d\tau}$"]),
    E("TI-6", "tape-interchange", "inequality", ["page", "quote"],
      r"The bound needs $s<W\mu$ <cite", r"The bound needs $s>W\mu$ <cite",
      "the quotation is s<Wm; the page's λ=μ(1-η) with η>0 also gives s<Wμ", "168",
      ["s>W", "s<W", "1-\\eta"]),
    E("TI-7", "tape-interchange", "constant", ["quote"],
      r"the paper computes $\eta>20\cdot2^{-50}$", r"the paper computes $\eta>20\cdot2^{-40}$",
      "η_b ≈ 1.5e-11 and η_c ≈ 3.7e-12 are below 20·2^{-40} ≈ 1.8e-11; the quotation says only "
      "greater than 20·2^{-50}", "1320-1329", ["2^{-40}", "2^{-50}"], conflicts=["TI-1"]),
    E("TI-8", "tape-interchange", "sign", ["page", "quote"],
      r"adds $x_S$ into $y_S$", r"subtracts $x_S$ from $y_S$",
      "a forward invocation changes y_S to y_S+x_S (the quotation)", "678-690",
      ["subtract", "y_S+x_S", "adds", "minus"]),
    E("TI-9", "tape-interchange", "generalisation", ["quote"],
      r"The interchange costs $O(Vu^{\tau})$ <cite", r"Any permutation of address fields costs $O(Vu^{\tau})$ <cite",
      "the bound is for interchanging two equal disjoint chunks (the quotation's 'Its cost' is "
      "the interchange's)", "97, 1484-1488", ["any permutation", "interchange", "two fields"],
      conflicts=["TI-10"]),
    E("TI-10", "tape-interchange", "attribution", ["quote"],
      r'<cite data-source="paper">Its cost is $O(Vu^\tau)$ for a fixed $\tau<1$</cite>',
      r'<cite data-source="paper">contains $V/W$ bits</cite>',
      "the quotation (paper 128) is about role-stream volume, not the interchange's cost",
      "97 (true), 128 (seeded)", ["V/W", "volume"],
      site="The interchange costs $O(Vu^{\\tau})$", conflicts=["TI-9"]),
]

# Sham edits: meaning-preserving rewordings, one per seeded variant. A note that calls a sham
# site wrong is a false alarm that an edit, not an error, provoked.
SHAMS = [
    ("SIC-s1", "subset-intersection-construction",
     r"Call two triples $T,S$ *neighbors* when $|T\cap S|=1$.",
     r"Two triples $T,S$ are called *neighbors* when $|T\cap S|=1$."),
    ("SIC-s2", "subset-intersection-construction", r"Absolute values add $2L$.",
     r"Taking absolute values adds $2L$."),
    ("ASF-s1", "address-shear-factorization", r"Elimination proves this:", r"Elimination shows this:"),
    ("ASF-s2", "address-shear-factorization", r"Several letters here change meaning from earlier pages.",
     r"Several letters on this page change meaning from earlier pages."),
    ("FAI-s1", "fast-address-interchange", r"The network performs only the middle update, which is the costly one.",
     r"The network performs only the middle update, the costly one."),
    ("FAI-s2", "fast-address-interchange", r"Two further linear-cost address updates complete the interchange.",
     r"Two more linear-cost address updates complete the interchange."),
    ("LNR-s1", "linear-network-roles", r"A wire's fixed place in the network is its *role*.",
     r"The fixed place of a wire in the network is its *role*."),
    ("LNR-s2", "linear-network-roles", r"The intermediate frames cancel and the route applies only its endpoint operator.",
     r"The intermediate frames cancel, and the route applies only its endpoint operator."),
    ("RPS-s1", "recursive-power-saving", r"The outside work is a fixed number of address-field row additions.",
     r"The outside work consists of a fixed number of address-field row additions."),
    ("RPS-s2", "recursive-power-saving", r"For example, with $m=2$", r"For instance, with $m=2$"),
    ("TI-s1", "tape-interchange", r"The outer two cost linear time.", r"The outer two take linear time."),
    ("TI-s2", "tape-interchange", r"Width one ($d=0$) is a single-digit operation",
     r"At width one ($d=0$) it is a single-digit operation"),
]

# ---------------------------------------------------------------------------------------------
# Ports of the pipeline's text handling (exegenius-core/src/text.rs, cite.rs, support.rs), so
# that sites and marks are found as the tasks see them.

def normalise_mapped(s):
    """text::normalise, with the offset in `s` each character of the result comes from."""
    out, frm, pending = [], [], None
    for i, c in enumerate(s):
        if c in "\u2018\u2019\u201B\u2032":
            c = "'"
        elif c in "\u201C\u201D\u201F\u2033":
            c = '"'
        elif "\u2010" <= c <= "\u2015" or c == "\u2212":
            c = "-"
        elif c == "\u00AD":
            continue
        if c.isspace():
            if pending is None:
                pending = i
            continue
        if pending is not None and out:
            out.append(" ")
            frm.append(pending)
        pending = None
        out.append(c)
        frm.append(i)
    return "".join(out), frm


def normalise(s):
    return normalise_mapped(s)[0]


def locate(haystack, quote):
    """text::locate: the span in `haystack` of the first verbatim match of `quote`."""
    q = normalise(quote)
    if not q:
        return None
    norm, frm = normalise_mapped(haystack)
    at = norm.find(q)
    if at < 0:
        return None
    return frm[at], frm[at + len(q) - 1] + 1


def cite_marks(body):
    """cite::marks: (start, end, source, quote) for each well-formed mark."""
    found, frm = [], 0
    while True:
        i = body.find("<cite", frm)
        if i < 0:
            break
        oe = body.find(">", i)
        close = body.find("</cite>", oe + 1) if oe >= 0 else -1
        if oe < 0 or close < 0:
            raise ValueError("malformed <cite> mark")
        oe += 1
        end = close + len("</cite>")
        m = re.search(r'data-source="([^"]*)"', body[i:oe])
        quote = body[oe:close].strip()
        if not m or not quote:
            raise ValueError("malformed <cite> mark")
        found.append((i, end, m.group(1).strip(), quote))
        frm = end
    return found


def replace_marks(body, text):
    """cite::replace: each mark, with the space before it, becomes `text`."""
    out = body
    for start, end, _, _ in reversed(cite_marks(body)):
        if out[:start].endswith(" "):
            start -= 1
        out = out[:start] + text + out[end:]
    return out


def auditor_view(body):
    """cite::referenced: the page as an audit task shows it."""
    return replace_marks(body, " (cited)")


def support_marks(body):
    """support::marks: the marks a support task lists, in order, with claim and quote."""
    out = []
    for start, end, source, quote in cite_marks(body):
        before = body[:start]
        ps = before.rfind("\n\n")
        ps = 0 if ps < 0 else ps + 2
        stops = [(before[ps:].rfind(s), s) for s in (". ", "? ", "! ", "</cite>")]
        starts = [ps + k + len(s) for k, s in stops if k >= 0]
        cs = max(starts) if starts else ps
        claim = replace_marks(before[cs:], "").strip()
        if not claim or normalise(claim) == normalise(quote):
            continue
        out.append({"claim": claim, "quote": quote, "claim_start": cs, "end": end})
    return out


def math_segments(text):
    """The $...$ and $$...$$ segments (an approximation of symbols::math)."""
    return re.findall(r"\$\$(.+?)\$\$|\$(.+?)\$", text, flags=re.S)


def tex_tokens(segment):
    return re.findall(r"\\[A-Za-z]+|\\.|[A-Za-z]", segment)


def math_tokens(text):
    toks = set()
    for a, b in math_segments(text):
        toks.update(tex_tokens(a or b))
    return toks


def occurs(symbol, text):
    """symbols::occurs, approximately: the symbol's tokens appear in order in one segment."""
    want = re.findall(r"\\[A-Za-z]+|\\.|[^\s{}]", symbol.strip().strip("$"))
    for a, b in math_segments(text):
        toks = re.findall(r"\\[A-Za-z]+|\\.|[^\s{}]", a or b)
        for i in range(len(toks) - len(want) + 1):
            if toks[i:i + len(want)] == want:
                return True
    return False


def sentence_spans(text):
    """Sentence spans outside mathematics: a sentence ends at . ? ! before a space or line end,
    or at a line end. Display formulas on their own line are their own sentence."""
    spans, start, i, inline, display, n = [], 0, 0, False, False, len(text)
    while i < n:
        if text.startswith("$$", i):
            display = not display
            i += 2
            continue
        c = text[i]
        if c == "\\":
            i += 2
            continue
        if c == "$" and not display:
            inline = not inline
        elif c == "\n" and not display:
            spans.append((start, i))
            start = i + 1
        elif c in ".?!" and not inline and not display and (i + 1 == n or text[i + 1] in " \n"):
            spans.append((start, i + 1))
            start = i + 1
        i += 1
    spans.append((start, n))
    out = []
    for a, b in spans:
        while a < b and text[a].isspace():
            a += 1
        while b > a and text[b - 1].isspace():
            b -= 1
        if b > a:
            out.append((a, b))
    return out


def paragraph_spans(text):
    spans, start = [], 0
    for m in re.finditer(r"\n\s*\n", text):
        spans.append((start, m.start()))
        start = m.end()
    spans.append((start, len(text)))
    return [(a, b) for a, b in spans if b > a]


def enclosing(spans, a, b):
    """The spans that overlap [a, b)."""
    return [s for s in spans if s[0] < b and a < s[1]]


def sha(text):
    return hashlib.sha256(text.encode()).hexdigest()


# ---------------------------------------------------------------------------------------------
# fetch

def cmd_fetch(args):
    out = pathlib.Path(args.out)
    out.mkdir(parents=True, exist_ok=True)

    def show(commit, path):
        return subprocess.run(["git", "-C", args.repo, "show", f"{commit}:{path}"],
                              check=True, capture_output=True, text=True).stdout

    bindings, native = {}, {}
    graphs, supports = {}, {}
    for page, meta in PAGES.items():
        commit = WIKIS[meta["wiki"]]["commit"]
        (out / f"{page}.md").write_text(show(commit, f"exegenius/pages/{page}.md"))
        if commit not in graphs:
            graphs[commit] = json.loads(show(commit, "exegenius/graph.json"))
            supports[commit] = json.loads(show(commit, "exegenius/support.json"))
        bindings[page] = [b["symbol"] for b in graphs[commit]["bindings"].get(page, [])]
        native[page] = [{"claim": j["mark"]["claim"], "quote": j["mark"]["quote"],
                         "label": j["label"]}
                        for j in supports[commit]["judged"] if j["mark"]["page"] == page]
    (out / "bindings.json").write_text(json.dumps(bindings, indent=1, ensure_ascii=False) + "\n")
    (out / "native-support.json").write_text(json.dumps(native, indent=1, ensure_ascii=False) + "\n")
    print(f"{len(PAGES)} pages, their bound symbols and native support labels in {out}")


# ---------------------------------------------------------------------------------------------
# seed

def site_span(view, text):
    span = locate(view, text)
    if span is None:
        raise ValueError(f"site not visible to the auditor: {text!r}")
    sents = enclosing(sentence_spans(view), *span)
    paras = enclosing(paragraph_spans(view), *span)
    return {"text": text, "span": list(span),
            "sentence": view[sents[0][0]:sents[-1][1]],
            "sentence_span": [sents[0][0], sents[-1][1]],
            "paragraph_span": [paras[0][0], paras[-1][1]]}


def mark_for(body, at):
    """The support-checked mark whose claim or quotation covers offset `at`, if any."""
    for i, m in enumerate(support_marks(body)):
        if m["claim_start"] <= at < m["end"]:
            return {"index": i + 1, "claim": m["claim"], "quote": m["quote"]}
    return None


def choose(page, rng, n_err, n_var, by_id):
    """N errors for each of V variants: one quote-detectable error first when the page has
    one, then types in a rotating order, at most one source-only error per page, no two errors
    of a variant in one sentence or in conflict."""
    pool = [e for e in CATALOGUE if e["page"] == page]
    rng.shuffle(pool)
    want = n_err * n_var
    picked = []
    quote = [e for e in pool if "quote" in e["detectable_by"]]
    if quote:
        picked.append(quote[0])
    order = TYPE_ORDER[:]
    rng.shuffle(order)
    while len(picked) < want:
        added = False
        for t in order:
            if len(picked) >= want:
                break
            for e in pool:
                if e in picked or e["type"] != t:
                    continue
                if e["detectable_by"] == ["source"] and any(p["detectable_by"] == ["source"] for p in picked):
                    continue
                picked.append(e)
                added = True
                break
        if not added:
            break
    # Distribute into variants, keeping conflicts and shared sentences apart.
    variants = [[] for _ in range(n_var)]
    for e in picked:
        slots = sorted(range(n_var), key=lambda v: (len(variants[v]), v))
        for v in slots:
            if len(variants[v]) >= n_err:
                continue
            if any(e["id"] in o["conflicts"] or o["id"] in e["conflicts"] or
                   e["_sentence"] == o["_sentence"] for o in variants[v]):
                continue
            variants[v].append(e)
            break
    return variants


def apply(body, edits):
    for ed in edits:
        if body.count(ed["find"]) != 1:
            raise ValueError(f"{ed['id']}: text to replace occurs {body.count(ed['find'])} times")
        body = body.replace(ed["find"], ed["replace"])
    return body


def cmd_seed(args):
    clean_dir, out = pathlib.Path(args.clean), pathlib.Path(args.out)
    bindings = json.loads((clean_dir / "bindings.json").read_text())
    paper = pathlib.Path(args.paper).read_text() if args.paper else None
    paper_norm = normalise(paper) if paper else None
    (out / "pages").mkdir(parents=True, exist_ok=True)
    key = {"schema": VERSION, "seed": args.seed, "errors_per_variant": args.errors,
           "variants_per_page": args.variants, "types": TYPES,
           "paper": {"path": args.paper, "sha256": sha(paper) if paper else None},
           "wikis": WIKIS, "pages": [], "variants": [], "errors": [], "shams": []}
    problems = []
    by_id = {e["id"]: e for e in CATALOGUE}
    for page in sorted(PAGES):
        meta = PAGES[page]
        clean = (clean_dir / f"{page}.md").read_text()
        clean_view = auditor_view(clean)
        for e in CATALOGUE + [{"id": s[0], "page": s[1], "find": s[2], "replace": s[3]} for s in SHAMS]:
            if e["page"] != page:
                continue
            if clean.count(e["find"]) != 1:
                problems.append(f"{e['id']}: text to replace occurs {clean.count(e['find'])} times")
                continue
            at = clean.index(e["find"])
            sents = sentence_spans(clean)
            e["_sentence"] = next((s for s in sents if s[0] <= at < s[1]), None)
        if problems:
            continue
        key["pages"].append({
            "page": page, "wiki": meta["wiki"], "commit": WIKIS[meta["wiki"]]["commit"],
            "plan_steps": meta["plan"], "chain_steps": meta["chain"], "clean_sha256": sha(clean),
            "bound_symbols": bindings.get(page, []),
            "native": [{"quote": q, "note": n,
                        "span": list(locate(clean_view, q) or [])} for q, n in meta["native"]]})
        for q, _ in meta["native"]:
            if locate(clean_view, q) is None:
                problems.append(f"{page}: native defect quote not on the page: {q!r}")
        rng = random.Random(f"A1:{args.seed}:{page}")
        variants = choose(page, rng, args.errors, args.variants, by_id)
        shams = [s for s in SHAMS if s[1] == page]
        rng.shuffle(shams)
        bodies = [(0, "clean", [], None, clean)]
        for v, errs in enumerate(variants, start=1):
            used = {e["_sentence"] for e in errs}
            sham = next((s for s in shams
                         if [x for x in CATALOGUE + [{"id": s[0], "page": s[1], "find": s[2]}]
                             if x["id"] == s[0]][0].get("_sentence", None) not in used), None)
            sham_e = None
            if sham:
                sham_e = {"id": sham[0], "find": sham[2], "replace": sham[3]}
                shams.remove(sham)
                shams.append(sham)
            edits = sorted(errs, key=lambda e: clean.index(e["find"]))
            body = apply(clean, edits + ([sham_e] if sham_e else []))
            bodies.append((v, "seeded", errs, sham_e, body))
        for v, kind, errs, sham_e, body in bodies:
            code = "a1-" + hashlib.sha256(f"{args.seed}:{page}:{v}".encode()).hexdigest()[:6]
            view = auditor_view(body)
            entry = {"code": code, "page": page, "variant": v, "kind": kind,
                     "file": f"pages/{code}.md", "sha256": sha(body),
                     "errors": [f"{e['id']}@{code}" for e in errs],
                     "shams": [f"{sham_e['id']}@{code}"] if sham_e else [],
                     "support_marks": [{"claim": m["claim"], "quote": m["quote"]}
                                       for m in support_marks(body)]}
            # Checks: the writer's bound symbols still occur; no new letters reach the auditor;
            # attribution quotes are verbatim in the paper.
            for s in bindings.get(page, []):
                if occurs(s, auditor_view(clean)) and not occurs(s, view):
                    problems.append(f"{code}: bound symbol {s} no longer occurs")
            new = math_tokens(view) - math_tokens(clean_view)
            if new:
                problems.append(f"{code}: new symbols reach the auditor: {sorted(new)}")
            if paper_norm is not None:
                for _, _, _, q in cite_marks(body):
                    if normalise(q) not in paper_norm:
                        problems.append(f"{code}: quotation not in the paper: {q!r}")
            (out / entry["file"]).write_text(body)
            key["variants"].append(entry)
            for e in errs + ([dict(sham_e, sham=True)] if sham_e else []):
                at = body.index(e["replace"]) if e["replace"] else None
                clean_at = clean.index(e["find"])
                site = e.get("site") or e["replace"]
                site_clean = e.get("site") or e["find"]
                rec = {"id": f"{e['id']}@{code}", "catalogue_id": e["id"], "code": code,
                       "page": page, "variant": v,
                       "original": e["find"], "seeded": e["replace"],
                       "location": {
                           "offset": at, "line": body[:at].count("\n") + 1,
                           "audit_site": site_span(view, site),
                           "audit_site_clean": site_span(clean_view, site_clean),
                           "linked": [site_span(view, t) for t in e.get("linked", [])],
                           "support_mark": mark_for(body, at),
                           "support_mark_clean": mark_for(clean, clean_at)}}
                if e.get("sham"):
                    key["shams"].append(rec)
                    continue
                rec.update({"type": e["type"], "detectable_by": e["detectable_by"],
                            "why": e["why"], "paper_lines": e["paper_lines"], "cues": e["cues"]})
                if "quote" in e["detectable_by"] and rec["location"]["support_mark"] is None:
                    problems.append(f"{rec['id']}: tagged quote-detectable but no support-checked mark covers it")
                key["errors"].append(rec)
    if problems:
        print("seeding failed:\n- " + "\n- ".join(problems), file=sys.stderr)
        sys.exit(1)
    (out / "key.json").write_text(json.dumps(key, indent=1, ensure_ascii=False) + "\n")
    counts = collections.Counter(e["type"] for e in key["errors"])
    detect = collections.Counter("+".join(e["detectable_by"]) for e in key["errors"])
    print(f"{len(key['variants'])} page variants ({len(key['pages'])} clean controls), "
          f"{len(key['errors'])} errors, {len(key['shams'])} shams in {out}")
    print("types:", dict(sorted(counts.items())))
    print("detectable by:", dict(sorted(detect.items())))


# ---------------------------------------------------------------------------------------------
# score

# A note names an error, rather than a gap in the explanation, when it asserts that something
# on the page is false. Fixed before any run; its precision is checked by hand (notes.tsv).
ASSERTS = re.compile(
    r"\b(wrong|incorrect\w*|errors?|erroneous|mistakes?|mistaken|typo\w*|inconsisten\w*|"
    r"contradict\w*|conflicts? with|false|untrue|not true|not correct|"
    r"does not (equal|match|hold|agree)|doesn't (equal|match|hold|agree)|"
    r"do not (match|agree)|should (be|read|have)|instead of|sign (error|flip)|reversed|"
    r"backwards|cannot (hold|both)|impossible|actually (is|equals|gives))\b", re.I)
FLAGGED = {"unsupported", "contradicted"}


def asserts(gap):
    return bool(ASSERTS.search(f"{gap.get('term', '')} {gap.get('note', '')}"))


def squash(s):
    return re.sub(r"[\s${}]", "", s).lower()


def mentions(gap, cues):
    note = squash(f"{gap.get('term', '')} {gap.get('note', '')}")
    return any(squash(c) and squash(c) in note for c in cues)


def overlaps(span, other):
    return span is not None and span[0] < other[1] and other[0] < span[1]


def parse_support_task(text):
    """The quotations a support task lists, as (claim, quote), in order."""
    marks = []
    for m in re.finditer(r"^### (\d+)\n\nClaim: (.*?)\n\nQuotation, from .*?: \"(.*)\"\n\nParagraph:",
                         text, flags=re.M | re.S):
        marks.append({"claim": m.group(2), "quote": m.group(3)})
    return marks


def load_answers(root):
    root = pathlib.Path(root)
    data = {}
    for d in sorted(p for p in root.glob("*/*/*") if p.is_dir()):
        config, run, code = d.parts[-3:]
        rec = {}
        for name in ("audit", "support"):
            f = d / f"{name}.answer.json"
            if f.exists():
                rec[name] = json.loads(f.read_text())
        if (d / "support.task.md").exists():
            rec["support_task"] = (d / "support.task.md").read_text()
        if (d / "log.jsonl").exists():
            rec["log"] = [json.loads(l) for l in (d / "log.jsonl").read_text().splitlines() if l.strip()]
        data.setdefault(config, {}).setdefault(run, {})[code] = rec
    return data


def wilson(k, n, z=1.2816):
    """Two-sided 80% Wilson interval."""
    if n == 0:
        return [None, None]
    p = k / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return [round(c - h, 3), round(c + h, 3)]


def rate(k, n):
    return {"k": k, "n": n, "rate": round(k / n, 3) if n else None, "ci80": wilson(k, n)}


def score_audit(key, pages_dir, variant, answer):
    """Per error and sham of one variant: how the audit answer treats its site."""
    body = (pages_dir / variant["file"]).read_text()
    view = auditor_view(body)
    gaps = [dict(g, _span=locate(view, g.get("quote", ""))) for g in (answer or {}).get("gaps", [])]
    errs = [e for e in key["errors"] if e["code"] == variant["code"]]
    shams = [s for s in key["shams"] if s["code"] == variant["code"]]
    page = next(p for p in key["pages"] if p["page"] == variant["page"])
    native_spans = [tuple(enclosing(sentence_spans(view), *locate(view, n["quote"]))[0])
                    for n in page["native"] if locate(view, n["quote"])]
    used, out = set(), {"errors": {}, "shams": {}, "gaps": len(gaps)}
    for e in errs:
        loc = e["location"]
        site = loc["audit_site"]["sentence_span"]
        near = [loc["audit_site"]["paragraph_span"]] + [l["sentence_span"] for l in loc["linked"]]
        best = "missed"
        for i, g in enumerate(gaps):
            exact = overlaps(g["_span"], site)
            close = exact or any(overlaps(g["_span"], n) for n in near)
            named = asserts(g) or mentions(g, e["cues"])
            level = None
            if exact and named:
                level = "named-exact"
            elif close and named:
                level = "named-near"
            elif asserts(g) and mentions(g, e["cues"]):
                level = "named-elsewhere"
            elif exact:
                level = "located"
            if level:
                rank = ["missed", "located", "named-elsewhere", "named-near", "named-exact"]
                if rank.index(level) > rank.index(best):
                    best = level
                if level.startswith("named") or level == "located":
                    used.add(i)
        out["errors"][e["id"]] = best
    for s in shams:
        site = s["location"]["audit_site"]["sentence_span"]
        hits = [g for g in gaps if overlaps(g["_span"], site)]
        out["shams"][s["id"]] = ("asserted" if any(asserts(g) for g in hits)
                                 else "located" if hits else "quiet")
        used.update(i for i, g in enumerate(gaps) if overlaps(g["_span"], site))
    # Assertions that match no planted error or sham: native defect, or a false alarm.
    out["native_asserted"] = 0
    out["spurious_asserted"] = 0
    out["unlocated_quotes"] = sum(1 for g in gaps if g["_span"] is None)
    for i, g in enumerate(gaps):
        if i in used or not asserts(g):
            continue
        if any(overlaps(g["_span"], n) for n in native_spans):
            out["native_asserted"] += 1
        else:
            out["spurious_asserted"] += 1
    return out


def score_support(key, pages_dir, variant, rec):
    """Per error of one variant: the label its mark got, and the labels on the clean marks."""
    answer = rec.get("support")
    if answer is None:
        return None
    marks = parse_support_task(rec["support_task"]) if rec.get("support_task") else None
    if not marks:
        marks = variant["support_marks"]
    labels = {l["mark"]: l["label"] for l in answer.get("labels", [])}

    def label_of(mark):
        if mark is None:
            return None
        for i, m in enumerate(marks, start=1):
            if normalise(m["claim"]) == normalise(mark["claim"]) and normalise(m["quote"]) == normalise(mark["quote"]):
                return labels.get(i)
        return "unmatched"

    out = {"errors": {}, "labels": [labels.get(i) for i in range(1, len(marks) + 1)]}
    for e in key["errors"]:
        if e["code"] == variant["code"]:
            out["errors"][e["id"]] = label_of(e["location"]["support_mark"])
    return out


def cmd_score(args, data=None, quiet=False):
    key = json.loads(pathlib.Path(args.key).read_text())
    pages_dir = pathlib.Path(args.key).parent
    data = data if data is not None else load_answers(args.answers)
    variants = {v["code"]: v for v in key["variants"]}
    errors = {e["id"]: e for e in key["errors"]}
    report, notes = {}, []
    for config in sorted(data):
        per_run, agg = {}, collections.defaultdict(lambda: [0, 0])
        missing = 0
        for run in sorted(data[config]):
            got = data[config][run]
            for code, v in variants.items():
                rec = got.get(code, {})
                if "audit" not in rec:
                    missing += 1
                if "support" not in rec:
                    missing += 1
                a = score_audit(key, pages_dir, v, rec.get("audit")) if "audit" in rec else None
                s = score_support(key, pages_dir, v, rec)
                clean_code = next(c for c, x in variants.items() if x["page"] == v["page"] and x["kind"] == "clean")
                clean_rec = got.get(clean_code, {})
                if a is not None:
                    agg["audit_gaps"][0] += a["gaps"]
                    agg["audit_gaps"][1] += 1
                    agg["unlocated_quotes"][0] += a["unlocated_quotes"]
                    if v["kind"] == "clean":
                        agg["fa_clean_asserted"][0] += a["spurious_asserted"]
                        agg["fa_clean_asserted"][1] += 1
                        agg["native_asserted"][0] += a["native_asserted"]
                        agg["native_asserted"][1] += 1
                    else:
                        agg["fa_seeded_spurious"][0] += a["spurious_asserted"]
                        agg["fa_seeded_spurious"][1] += 1
                    for sid, how in a["shams"].items():
                        agg["fa_sham_asserted"][0] += how == "asserted"
                        agg["fa_sham_asserted"][1] += 1
                        agg["sham_located"][0] += how != "quiet"
                        agg["sham_located"][1] += 1
                    for eid, how in a["errors"].items():
                        e = errors[eid]
                        page_class = "page" in e["detectable_by"]
                        named = how.startswith("named")
                        for bucket in (["audit_named_page", f"audit_named_type:{e['type']}"] if page_class
                                       else [f"audit_named_{'+'.join(e['detectable_by'])}"]):
                            agg[bucket][0] += named
                            agg[bucket][1] += 1
                        if page_class:
                            agg["audit_located_page"][0] += how != "missed"
                            agg["audit_located_page"][1] += 1
                            agg["audit_exact_of_named"][0] += how == "named-exact"
                            agg["audit_exact_of_named"][1] += named
                            # Baseline: the same sentence on the clean page, in this run.
                            if "audit" in clean_rec:
                                cview = auditor_view((pages_dir / variants[clean_code]["file"]).read_text())
                                span = e["location"]["audit_site_clean"]["sentence_span"]
                                cg = [g for g in clean_rec["audit"].get("gaps", [])
                                      if overlaps(locate(cview, g.get("quote", "")), span)]
                                agg["clean_site_located"][0] += bool(cg)
                                agg["clean_site_located"][1] += 1
                                agg["clean_site_asserted"][0] += any(asserts(g) for g in cg)
                                agg["clean_site_asserted"][1] += 1
                        notes.append((config, run, eid, how))
                if s is not None:
                    for eid, label in s["errors"].items():
                        e = errors[eid]
                        if "quote" in e["detectable_by"]:
                            agg["support_flagged_quote"][0] += label in FLAGGED
                            agg["support_flagged_quote"][1] += 1
                            agg[f"support_flagged_type:{e['type']}"][0] += label in FLAGGED
                            agg[f"support_flagged_type:{e['type']}"][1] += 1
                            agg["support_partial_quote"][0] += label == "partial"
                            agg["support_partial_quote"][1] += 1
                    if v["kind"] == "clean":
                        for label in s["labels"]:
                            agg["support_fa_clean_marks"][0] += label in FLAGGED
                            agg["support_fa_clean_marks"][1] += 1
                # The pipeline's union: an error is caught if the audit names it or the support
                # check flags its mark.
                if v["kind"] == "seeded":
                    for e in (x for x in key["errors"] if x["code"] == code):
                        if e["detectable_by"] == ["source"]:
                            continue
                        hit = bool(a and a["errors"][e["id"]].startswith("named")) or \
                            bool(s and s["errors"].get(e["id"]) in FLAGGED)
                        agg["union_caught"][0] += hit
                        agg["union_caught"][1] += 1
                        per_run.setdefault(run, [0, 0])
                        per_run[run][0] += hit
                        per_run[run][1] += 1
        report[config] = {k: rate(*v) for k, v in sorted(agg.items())}
        report[config]["audit_gaps_per_page"] = (round(agg["audit_gaps"][0] / agg["audit_gaps"][1], 2)
                                                 if agg["audit_gaps"][1] else None)
        report[config]["union_caught_per_run"] = {r: rate(*v) for r, v in per_run.items()}
        report[config]["missing_answers"] = missing
        report[config]["submits"] = submit_summary(data[config])
    if args.json:
        pathlib.Path(args.json).write_text(json.dumps(report, indent=1) + "\n")
    if args.notes:
        write_notes(args.notes, data, key, pages_dir)
    if not quiet:
        print_report(report)
    return report


def submit_summary(runs):
    attempts, rejected, unaccepted = 0, 0, 0
    for run in runs.values():
        for rec in run.values():
            log = rec.get("log") or []
            subs = [e for e in log if e.get("event") == "submit"]
            attempts += len(subs)
            rejected += sum(1 for e in subs if e.get("outcome") == "rejected")
            if subs and subs[-1].get("outcome") != "accepted":
                unaccepted += 1
    return {"submits": attempts, "rejected": rejected, "never_accepted": unaccepted}


def write_notes(path, data, key, pages_dir):
    """Every gap at a planted or sham site, and every asserting gap, blinded to configuration,
    for the hand check of the ASSERTS classifier."""
    rows = []
    for config in data:
        for run in data[config]:
            for code, rec in data[config][run].items():
                for g in (rec.get("audit") or {}).get("gaps", []):
                    rows.append((config, run, code, g.get("quote", ""), g.get("note", ""), asserts(g)))
    rng = random.Random(f"A1-notes:{key['seed']}")
    rng.shuffle(rows)
    with open(path, "w") as f:
        f.write("row\tcode\tquote\tnote\tasserts\n")
        blind = {}
        for i, (config, run, code, q, n, a) in enumerate(rows, start=1):
            blind[i] = (config, run)
            clean = lambda s: s.replace("\t", " ").replace("\n", " ")
            f.write(f"{i}\t{code}\t{clean(q)}\t{clean(n)}\t{int(a)}\n")
    pathlib.Path(str(path) + ".unblind.json").write_text(json.dumps(blind) + "\n")


def print_report(report):
    keys = ["audit_named_page", "audit_located_page", "clean_site_located", "clean_site_asserted",
            "audit_exact_of_named", "fa_clean_asserted", "fa_sham_asserted", "fa_seeded_spurious",
            "native_asserted", "support_flagged_quote", "support_partial_quote",
            "support_fa_clean_marks", "union_caught"]
    for config, r in report.items():
        print(f"== {config}  (gaps per page {r['audit_gaps_per_page']}, missing answers "
              f"{r['missing_answers']}, submits {r['submits']})")
        for k in keys + sorted(k for k in r if ":" in k or k.startswith("audit_named_q")
                               or k.startswith("audit_named_s")):
            if k in r and isinstance(r[k], dict) and "k" in r[k]:
                x = r[k]
                print(f"  {k:34s} {x['k']:>4}/{x['n']:<4} {x['rate'] if x['rate'] is not None else '-':>6}  80% {x['ci80']}")


# ---------------------------------------------------------------------------------------------
# selftest

def synthetic(key, pages_dir, kind):
    """Answers from an oracle (names every error the task can see), a silent auditor and an
    alarmist (asserts an error in every sentence, contradicts every mark)."""
    answers = {}
    for v in key["variants"]:
        body = (pages_dir / v["file"]).read_text()
        view = auditor_view(body)
        marks = v["support_marks"]
        gaps, labels = [], []
        if kind == "oracle":
            for e in key["errors"]:
                if e["code"] == v["code"] and "page" in e["detectable_by"]:
                    gaps.append({"term": e["type"], "quote": e["location"]["audit_site"]["text"],
                                 "kind": "step", "note": f"This is wrong: {e['why']}"})
            gaps.append({"term": "x", "quote": view[sentence_spans(view)[0][0]:][:20],
                         "kind": "term", "note": "The reader does not know this term."})
            flagged = {e["location"]["support_mark"]["index"] for e in key["errors"]
                       if e["code"] == v["code"] and "quote" in e["detectable_by"]}
            labels = [{"mark": i, "label": "contradicted" if i in flagged else "supported", "reason": "r"}
                      for i in range(1, len(marks) + 1)]
        elif kind == "silent":
            labels = [{"mark": i, "label": "supported", "reason": ""} for i in range(1, len(marks) + 1)]
        elif kind == "alarmist":
            for a, b in sentence_spans(view):
                gaps.append({"term": "t", "quote": view[a:b], "kind": "step", "note": "This is wrong."})
            labels = [{"mark": i, "label": "contradicted", "reason": "r"} for i in range(1, len(marks) + 1)]
        answers[v["code"]] = {"audit": {"symbols": [], "gaps": gaps}, "support": {"labels": labels}}
    return {kind: {"r1": answers}}


def cmd_selftest(args):
    key_path = pathlib.Path(args.key)
    key = json.loads(key_path.read_text())
    pages_dir = key_path.parent
    ns = argparse.Namespace(key=args.key, answers=None, json=None, notes=None)
    failures = []

    def check(cond, what):
        print(("ok   " if cond else "FAIL ") + what)
        if not cond:
            failures.append(what)

    for v in key["variants"]:
        check(sha((pages_dir / v["file"]).read_text()) == v["sha256"], f"{v['code']} matches the key")
    data = {}
    for kind in ("oracle", "silent", "alarmist"):
        data.update(synthetic(key, pages_dir, kind))
    r = cmd_score(ns, data=data, quiet=True)
    o, s, a = r["oracle"], r["silent"], r["alarmist"]
    check(o["audit_named_page"]["rate"] == 1.0, "oracle names every page-detectable error")
    check(o["audit_exact_of_named"]["rate"] == 1.0, "oracle's detections are all at the exact site")
    check(o["support_flagged_quote"]["rate"] == 1.0, "oracle's support flags every quote-detectable error")
    check(o["fa_clean_asserted"]["k"] == 0 and o["support_fa_clean_marks"]["k"] == 0,
          "oracle raises no false alarm on clean controls")
    check(o["fa_sham_asserted"]["k"] == 0, "oracle raises no false alarm at sham sites")
    check(o["union_caught"]["rate"] == 1.0, "oracle's union catches every page- or quote-detectable error")
    check(s["audit_named_page"]["rate"] == 0.0 and s["support_flagged_quote"]["rate"] == 0.0,
          "silent auditor detects nothing")
    check(s["fa_clean_asserted"]["k"] == 0, "silent auditor raises no false alarm")
    check(a["audit_named_page"]["rate"] == 1.0, "alarmist 'names' everything at its site")
    check(a["fa_clean_asserted"]["rate"] is not None and a["fa_clean_asserted"]["rate"] > 1,
          "alarmist's false alarms on clean pages are counted")
    check(a["fa_sham_asserted"]["rate"] == 1.0, "alarmist is caught at every sham site")
    check(a["support_fa_clean_marks"]["rate"] == 1.0, "alarmist's support labels are all false alarms on clean marks")
    check(a["clean_site_asserted"]["rate"] == 1.0, "the clean-site baseline sees the alarmist")
    # Determinism: choices depend only on the seed.
    by_id = {e["id"]: e for e in CATALOGUE}
    for page in PAGES:
        clean = next(p for p in key["pages"] if p["page"] == page)
        picks = [sorted(e.split("@")[0] for e in v["errors"]) for v in key["variants"] if v["page"] == page]
        check(all(len(p) <= key["errors_per_variant"] for p in picks), f"{page}: at most N errors per variant")
    types = collections.Counter(e["type"] for e in key["errors"])
    check(set(types) == set(TYPES), f"every type is seeded at least once: {dict(types)}")
    if failures:
        sys.exit(1)
    print("selftest passed")


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = p.add_subparsers(dest="cmd", required=True)
    f = sub.add_parser("fetch")
    f.add_argument("--repo", required=True)
    f.add_argument("--out", required=True)
    s = sub.add_parser("seed")
    s.add_argument("--clean", required=True)
    s.add_argument("--out", required=True)
    s.add_argument("--seed", required=True)
    s.add_argument("--errors", type=int, default=2)
    s.add_argument("--variants", type=int, default=3)
    s.add_argument("--paper")
    c = sub.add_parser("score")
    c.add_argument("--key", required=True)
    c.add_argument("--answers", required=True)
    c.add_argument("--json")
    c.add_argument("--notes")
    t = sub.add_parser("selftest")
    t.add_argument("--key", required=True)
    args = p.parse_args()
    {"fetch": cmd_fetch, "seed": cmd_seed, "score": cmd_score, "selftest": cmd_selftest}[args.cmd](args)


if __name__ == "__main__":
    main()

# A1 design: a seeded page set for planted-error recall in fidelity auditing

**Status:** design only. No model or subagent has run on these pages. The plan is `calibration/plans/A1-draft.md` (DRAFT, not preregistered).

**What this directory holds:**
- 24 page bodies in `pages/`: 6 clean controls and 18 seeded variants;
- `key.json`, the answer key.

They were made by `calibration/tools/plant.py`, which also scores auditor answers against the key.

**Keep this directory away from auditors.** An auditor runs in a project copy and is told to read only its task file. The key must never be inside a run directory.

## How it was made

```
python3 -I calibration/tools/plant.py fetch --repo /home/user/openai-nlogn-wiki --out SCRATCH/a1/fetched
python3 -I calibration/tools/plant.py seed --clean SCRATCH/a1/fetched --out docs/journal/2026-10-14-a1-audit-design \
    --seed 20261014 --errors 2 --variants 3 --paper SCRATCH/paper.tex
python3 -I calibration/tools/plant.py selftest --key docs/journal/2026-10-14-a1-audit-design/key.json
```

- **`fetch`** reads the pages, their bound symbols and their support labels with `git show COMMIT:PATH`. It is read-only.
- **`paper.tex`** has sha256 `6d2b6d00dc34…`, the same file as `sources/paper.tex` at both wiki commits.
- **Determinism:** seeding twice with the same seed gives byte-identical output (checked by `diff -r`). Seed 7 gives a different draw.
- **The clean controls** are byte-identical to the wiki pages (`clean_sha256` in the key).

## The pages

The six pages below carry derivations of the paper's key step: the central saving $s<Wm$, steps C1–C9 of `calibration/rubrics/nlogn-central-saving.md`. W1 and W2 are the blinded names used in journal 2026-10-13-chain-walk-baseline.

| Page | Wiki (commit) | Plan step | Chain steps it derives | Native defect recorded in the key |
|---|---|---|---|---|
| `subset-intersection-construction` | W1, planner-handoff (72f1de8) | S1.4 | C1 coefficients, C3 labels, C4 rank = dimension change, C5 budget, C6 $L/N=100/539$ | "Sources have dimension 1" (only the $X_a$ sources do) |
| `address-shear-factorization` | W1 | S1.2.1 | C8 $M=F_1\Pi F_2$, one interchange per 1, restating C5–C6 | none |
| `fast-address-interchange` | W1 | S1.2 | C9 recurrence $Q_k\le(s/W)Q_{k-1}+C$ with its case split; C6 | fields "over $\mathbb F_2$" (two sentences) |
| `linear-network-roles` | W1 | S1.1 | C2 frames pass through pointwise gates, telescoping; C7 (in part) | none |
| `recursive-power-saving` | W1 | S1.5 | C9 in general form: choosing $\lambda$, the leaf bound | the leaf bound that does not follow (journal 2026-10-13-planner-handoff-run) |
| `tape-interchange` | W2, explanation-depth (fa33c9d) | S7.1 | C9 with the explicit $\tau=1-2^{-50}$, $\eta>20\cdot2^{-50}$, $\mu<2^{20}$; C6; C8 | "the shear is such an invocation" (chain walk W2, C7) |

**Native defects** are errors already on the clean pages. An auditor that calls one wrong is credited with a native detection, not charged with a false alarm.

**Why W3 is not used:** the W3 wiki (e4aff3e) reconstructs too little of the chain (C3–C5 absent).

## The error taxonomy

Seven types, fixed before any run:

| Type | Definition | Seeded |
|---|---|---|
| constant | a wrong number, coefficient, exponent, index or computed value | 8 |
| inequality | an inequality reversed, or its strictness changed | 6 |
| hypothesis | a condition the claim or step needs is removed | 5 |
| cases | a case dropped from a case analysis, or a wrong count of its cases, steps or terms | 3 |
| generalisation | a claim widened beyond what holds | 5 |
| sign | a sign or an operation reversed | 5 |
| attribution | a citation whose quotation is a real sentence of the paper (so it passes the writer's verbatim check) but does not support the claim | 4 |

**What can reveal each error (`detectable_by`).** Neither of the pipeline's auditors compares a page with the paper:
- the audit task sees each citation as "(cited)";
- the support task sees only a claim and its quotation.

So each error is tagged with what could reveal it:

| Tag | Meaning | Errors |
|---|---|---|
| `page` | the page itself, with what the reader profile covers | 25 |
| `page+quote` | the page, or the claim's own quotation | 3 |
| `quote` | only the claim's own quotation, so only the support task | 7 |
| `source` | only the paper: a blind spot of both tasks, reported and never scored | 1 (TI-3: $\mu<2^{19}$; the page never states $\mu=10^6$) |

## The seeded variants

Each page has a clean control and three seeded variants. A seeded variant carries two errors and one sham edit.

**How errors are chosen:**
- from a hand-written catalogue of 51 errors and 12 shams, in `plant.py`;
- one error that its quotation would reveal goes first, where the page has one;
- then types are taken in a seeded rotation;
- no two errors of a variant share a sentence or are marked as conflicting;
- at most one source-only error per page.

**Every error comes with:**
- the original and seeded text;
- its location: an offset and line in the stored body, and the sentence and paragraph the auditor sees, in both the seeded and the clean version;
- the support-checked mark it touches, if any;
- the reason it is wrong, with the paper's lines;
- cues: specific strings a note naming the error would contain.

**The 36 errors:**

| Error | Variant | Type | Detectable by | Original → seeded |
|---|---|---|---|---|
| ASF-1 | a1-34e565 | hypothesis | page | "$F_1$ and $F_2$ are invertible lower-triangular" → "lower-triangular" |
| ASF-7 | a1-34e565 | attribution | quote | quotation "A lower triangular transformation … costs $O(V)$" → "It uses just one interchange." |
| ASF-5 | a1-25155c | inequality | page | $N>2L_{\rm b}$ → $N<2L_{\rm b}$ |
| ASF-6 | a1-25155c | generalisation | page+quote | "A lower-triangular transformation" → "Any invertible linear transformation of $H$ or $D$" |
| ASF-3 | a1-334ea2 | sign | page | $D_\tau\leftarrow H_\sigma-D_\tau$ → $D_\tau\leftarrow D_\tau-H_\sigma$ |
| ASF-8 | a1-334ea2 | constant | page | $\operatorname{rank}M$ interchanges → $2\operatorname{rank}M$ |
| FAI-5 | a1-79746e | constant | quote | $s<Wm$ → $s<W$ |
| FAI-7 | a1-79746e | attribution | quote | quotation "for a fixed $\tau<1$" → "contains $V/W$ bits" |
| FAI-4 | a1-fe95b8 | cases | page | the $s/W\le1$ case of the geometric sum dropped |
| FAI-8 | a1-fe95b8 | sign | page | unrolled $+C\sum$ → $-C\sum$ |
| FAI-3 | a1-c3b1a7 | inequality | page | $s/W<m^\tau$ → $s/W>m^\tau$ |
| FAI-6 | a1-c3b1a7 | generalisation | quote | "For the bit network" → "For every linear network" |
| LNR-7 | a1-a8a305 | sign | page | multiplying by $\varepsilon_w$ → by $-\varepsilon_w$ |
| LNR-8 | a1-a8a305 | attribution | quote | quotation "leaves the endpoint operator on every role array" → "Every displayed summand is nondegenerate." |
| LNR-5 | a1-aef5e8 | hypothesis | page | "invertible linear operator" → "linear operator" (frames) |
| LNR-6 | a1-aef5e8 | constant | page | telescopes to $D_{\alpha_k}D_{\alpha_0}^{-1}$ → $D_{\alpha_k}D_{\alpha_1}^{-1}$ |
| LNR-1 | a1-edc768 | constant | page+quote | $V/W$ bits → $V/(2W)$ |
| LNR-4 | a1-edc768 | generalisation | page | "pass through gates" → "every gate, linear or not" |
| RPS-1 | a1-53c428 | sign | page | $+C(mf)^\lambda$ → $-C(mf)^\lambda$ |
| RPS-2 | a1-53c428 | inequality | page | $\log_m(s/W)<\lambda<1$ → $\lambda<\log_m(s/W)<1$ |
| RPS-3 | a1-06c1ac | hypothesis | page | "$f\ge d^\vartheta$ at an internal node" dropped |
| RPS-4 | a1-06c1ac | constant | page | ratio $\approx0.86$ → $\approx1.16$ |
| RPS-5 | a1-6969c7 | generalisation | page | the bound on the internal sum "for every $\lambda>0$" |
| RPS-7 | a1-6969c7 | inequality | page | $s/W<m$ → $s/W>m$ |
| SIC-10 | a1-8c4d7c | attribution | quote | quotation "smaller calls satisfies $s<Wm$" → "Every displayed summand is nondegenerate." |
| SIC-5 | a1-8c4d7c | constant | page | $3\binom h3^2$ invocations → $2\binom h3^2$ (with $L=3\binom h3^2h^2$ kept) |
| SIC-4 | a1-a9453b | hypothesis | page | "If $U\subseteq V$" → "For any labels $U,V$" |
| SIC-6 | a1-a9453b | cases | page | sizes $0,1,2$ → sizes $0$ and $2$ |
| SIC-3 | a1-492e03 | generalisation | page | "neighbors are orthogonal" → "distinct triples are orthogonal" |
| SIC-7 | a1-492e03 | inequality | page | $2L<N$ → $2L>N$ |
| TI-2 | a1-3d211b | inequality | page | $\exp(-2^{-50}\ln\mu)>1-20\cdot2^{-50}$ → $<$ |
| TI-7 | a1-3d211b | constant | quote | $\eta>20\cdot2^{-50}$ → $\eta>20\cdot2^{-40}$ (false: $\eta_{\rm b}\approx1.5\cdot10^{-11}$) |
| TI-3 | a1-d043fc | constant | source | $\mu<2^{20}$ → $\mu<2^{19}$ |
| TI-8 | a1-d043fc | sign | page+quote | "adds $x_S$ into $y_S$" → "subtracts $x_S$ from $y_S$" |
| TI-4 | a1-2c1af2 | cases | page | the sum is at most $d+1$ → at most $d$ |
| TI-5 | a1-2c1af2 | hypothesis | page | "needs $\lambda<\mu^\tau$ …, not merely $\lambda<\mu$" → "needs only $\lambda<\mu$" |

## The controls

**Clean controls (6 pages).** These are the original pages. They give:
- the false-alarm rate: notes asserting an error on a clean page, outside the native defects;
- the per-site baseline: how often a configuration flags the same sentence when it is correct. The audit task reports followability gaps on correct text routinely, about two to three per page in the full runs. So a gap at a site is evidence of detection only beyond that baseline;
- the support labels on 14 correct citation marks.

**Sham edits (18).** These are meaning-preserving rewordings, for example "Elimination proves this:" → "Elimination shows this:". A note calling a sham site wrong is a false alarm provoked by an edit, not by an error.

## Checks done here

All of these ran with no model:

- **`plant.py seed`.** Each replaced text occurs once on its clean page. The writer's bound symbols still occur in each variant: submit rejects an audit that cannot account for one. No new letter reaches the auditor. Every quotation is verbatim in the paper.
- **`plant.py selftest`.** It scores three synthetic auditors:
  - an oracle: recall 1, no false alarms, every detection at its site;
  - a silent auditor: recall 0, no false alarms;
  - an alarmist: every sham and clean site caught as a false alarm.
  
  It also checks that a gap at one error is never credited to a neighbouring error that the task cannot see.
- **`plant.py score` from disk.** A synthetic answer tree in the collector's layout was scored, with support task files in the pipeline's format.
- **A dry run of the pipeline with no model.** Project copies of W1 and W2 were rebuilt with `git show` in the scratchpad, and a seeded page was put in place. The state edits in the plan's setup were applied, and `exegenius next` (0.6.0, `target/release`, built 2026-10-14 06:30) was run:
  - it issued an audit task whose "Page to audit" block equals the seeded page as the auditor sees it;
  - it issued a support task whose listed quotations equal the key's marks, for `subset-intersection-construction` (a1-492e03) and `tape-interchange` (a1-3d211b);
  - the two-step setup in the plan leaves a single audit task (`0213-audit-…`), identical to the one-step render apart from path and id.

## Key schema (`key.json`, `a1-plant/1`)

- **`pages[]`:** page, wiki, commit, plan and chain steps, `clean_sha256`, bound symbols, native defects, and the native support labels from the original run.
- **`variants[]`:** opaque `code` (`a1-` plus six hex digits), page, variant number (0 is the clean control), file, sha256, error and sham ids, and the support marks in task order.
- **`errors[]`:** `id` (`CATALOGUE-ID@code`), type, `detectable_by`, `original`, `seeded`, and `location`:
  - offset and line;
  - `audit_site` and `audit_site_clean`, each with the sentence and its spans in the auditor's view;
  - `linked` sentences;
  - `support_mark` and `support_mark_clean`.
  
  Also `why`, `paper_lines` and `cues`.
- **`shams[]`:** the same location fields.

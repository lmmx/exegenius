# 2026-10-12: Full run on the LaTeX source

The author ran the experiment of 2026-10-12-stop-early-switch and pushed it to lmmx/openai-nlogn-wiki as `full-v0.6-latex` (e4aff3e).
- **Software:** exegenius 47f9294.
- **Source:** `sources/paper.tex`, with a fresh extraction and the calculus correction at the reader check.
- **Settings:** `rounds = 3`, `stop_early = false`, no simulated readers.
- **The run:** 180 tasks in 32 minutes, all accepted, and 23 pages in 5 parts.

## The run

```
concepts: 48 (25 known to the reader); pages: 23 written of 23 planned
audit round 1: 69 gaps over 22 pages (per page: term 0.5, step 2.0, symbol 0.5)
audit round 2: 61 gaps over 23 pages (per page: term 0.4, step 2.0, symbol 0.3)
  on the 22 pages also audited in round 1: term 12 -> 7 (7 new), step 45 -> 44 (44 new), symbol 12 -> 7 (6 new)
audit round 3: 58 gaps over 23 pages (per page: term 0.5, step 2.0, symbol 0.1)
  on the 23 pages also audited in round 2: term 9 -> 11 (10 new), step 45 -> 45 (45 new), symbol 7 -> 2 (2 new)
steps inside reasoning a rewrite added for an earlier step, sent back to be cited or cut: 54
citation support: 30 marks judged: 13 supported, 14 partial, 2 unsupported, 1 contradicted
```

The stop rule would not have fired at either round, since the steps sent back to be cited or cut are left out of it, so `status` printed no counterfactual. Switching it off changed nothing in this run.

## Step gaps: every one fixed, as many new

Step gaps on repeated pages stayed level, 45, 44, 45, and almost all were new each round. Matching each round's reported passages against the next round's shows:

| Reported in round | Steps | Reported again in the next round |
|---|---|---|
| 1, first level | 45 | 0 |
| 2, first level | 13 | 0 |
| 2, cite or cut | 32 | 0 |

Every step a rewrite was told about was gone from the next audit. The level count is renewal.

**By kind:**
- First-level steps went from 45 in round 1 to 13 in round 2, then rose to 23 in round 3.
- Steps sent back to be cited or cut went 32, then 22. These are the second level, inside reasoning added for an earlier step. They are not reversals: the rule is meant to send them back, and their fix adds no claim.

**Where round 3's 23 first-level steps came from** (against each page's write answers):
- **7 were inside sentences a round 2 rewrite added that were not rewordings, on pages whose round 2 notes were all cite-or-cut notes.**
  - These rewrites were asked to cite or cut and added reasoning anyway, so their pages grew. `synthetic-transform` went from 313 to 387 words, and `address-shear` from 299 to 332.
  - `apply_write` recorded added sentences only for rewrites answering a step note, and replaced the record at each write. These sentences were therefore not counted as added, and steps in them came back as first-level, asking for more reasoning.
- **8 were in sentences the round 2 rewrite reworded.**
- **8 were in text on the page, word for word, at the round 2 audit, which that audit did not report.** An audit reports some of what a reader would stop at, not all of it, and a later audit of the same text reports others.

## The depth limit now remembers

A rewrite answering a note to cite or cut a step now records the sentences it adds, as one answering a step note does. Sentences recorded as added stay recorded while they are on the page, across later rewrites (exegenius-core/src/write.rs; `CITE_NOTE` in exegenius-core/src/audit.rs).

The test `reasoning_added_by_any_rewrite_for_a_step_stays_added` runs three rounds on one page:
- round 1 explains a step;
- round 2's step, inside that explanation, is sent back to be cited or cut;
- the cite-or-cut rewrite adds a sentence;
- round 3's step inside that sentence is sent back to be cited or cut.

Without the change the last came back as "explain the step". On this run's records, 7 of the 23 round 3 first-level steps would have been cite-or-cut instead.

The 8 steps in unchanged text are audit variance, which no change to the writer reaches. They mean that the gaps one round reports undercount what is on the pages. A level count can hide a falling stock of real gaps, or a steady one.

## Citations

The three marks not supported:
- **Contradicted, on `tensor-product`:** the claim gives the identity on the coefficient positions of each record. The quote says the layer acts "coefficientwise", which is to say the same map on each coefficient. The two agree; the judge read "coefficientwise" as acting on the coefficients.
- **Unsupported, on `lower-triangular-factorization`:** "$\Pi$ has exactly $a$ entries equal to one" is cited for $\operatorname{rank}\Pi = a$, which needs the entries in distinct rows and columns.
- **Unsupported, on `chinese-remainder-address-map`:** "by a Chinese-remainder map that preserves convolution" is cited for the coefficient list being placed on distinct prime axes.

The two unsupported marks quote a fragment beside the claim, as in 2026-10-12-latex-pilot.

## The pages, against the paper

Five central pages were read against `paper.tex`.

**Correct, and they explain:**
- **`integer-multiplication-target`:**
  - $\lg n = \max\{\lceil\log_2 n\rceil, 1\}$ (the paper's line 55);
  - $\kappa = 2^{-182}$;
  - the two tape procedures with their costs;
  - the recursion multiplying cost by $s/W < m$ per level, "the source of the power saving".
- **`recursive-power-saving`:** derives the cost. Level $\nu$ has $s^\nu$ calls on volume $V/W^\nu$, so it costs $V(s/W)^\nu$. The geometric sum is dominated by its last term, $O(V(m^\ell)^e)$ with $e = \log_m(s/W) < 1$ exactly when $s < Wm$.
- **`rounding-recovery`:**
  - derives that the retained permutation cancels, since $QF_K^+Q = F_K^+$ by substituting $\eta k$, and $Q$ commutes with pointwise products;
  - shows why rounding is exact within $1/2$;
  - works the carry propagation in base 2, with $3 + 2\cdot5 = 13$ to $1101_2$.
  
  Each step checks.

**Correct as far as they go, but hollow:**
- **`gaussian-resampling`** states the identity $P_sF_{n_0} = K M_2 P_t F_{n_1} M_1$ and cites the existence of the contractions ("Such contractions exist", cited). None of its five versions gave the construction (the Gaussian maps $S$ and $T$, $N = CTD$, the inverse estimate) that the first-level pilot's page gave.
  - Round 1 asked for the identity, and the rewrite stated it.
  - Round 2 reported that the page "gives no construction". That sentence was inside the reasoning the rewrite had added, so it was sent back to be cited or cut, "do not explain it further", and the writer cited.
  - Round 3 reported the cited existence claim, again inside added reasoning, again cite-or-cut.
  
  The page's own mechanism was filed as a second-level step.
  
  Its notation is broken too. Rewrites after the audits, answering overload notes, renamed $s$ and $t$ to $n_0$ and $n_1$, and $A$ and $B_0$ to $M_1$ and $M_2$. They kept $P_s$ and $P_t$, whose subscripts the page no longer defines. A prime $q$ is introduced and never tied to $n_0$.
- **`simultaneous-butterfly-layer`** cites the $O(Vd^{\lambda'})$ bound and the sections that prove it, and gives none of the mechanism the earlier full run's page had: the kernel $C = aI + bX$ and $H_0 = bSCS$. One citation quotes the source's markup, "Section~\ref{sec:fast-butterfly-layers} gives the numerical layout…", which readers see as is in the popup.

## Steps on cited claims

Seven of the run's cite-or-cut steps were reported on claims that already carried a citation mark, among them "Such contractions exist (cited)". A citation cannot answer them. Under the audit rule of 2026-10-12-explain-first-level, a step reported on a cited claim means the citation did not make the step followable.

A step reported inside added reasoning now gets a note asking for the reasoning, not one to cite or cut, when its sentence already carries a citation mark (exegenius-core/src/audit.rs).

## Citations of markup

A citation whose quote contains `\ref{`, `\eqref{`, `\cite` or `\label{` is rejected at write time. The message says readers would see the markup as is, and asks for the passage that states the claim (exegenius-core/src/cite.rs).

## Current State
- The full LaTeX run ran 180 tasks over 23 pages in 32 minutes, with step gaps on repeated pages at 45, 44 and 45 in three rounds
- No step gap reported in one round of the full LaTeX run was reported again in the next
- Of its 23 first-level step gaps in round 3, 7 were inside reasoning added by rewrites answering cite-or-cut notes, 8 in reworded sentences, and 8 in text unchanged since the round 2 audit
- A rewrite answering a cite-or-cut note records the reasoning it adds, and added reasoning stays recorded while it is on the page (exegenius-core/src/write.rs)
- Of five central pages checked against the paper, `integer-multiplication-target`, `recursive-power-saving` and `rounding-recovery` are correct and derive their claims, while `gaussian-resampling` and `simultaneous-butterfly-layer` state their results and cite the mechanism
- A step reported inside added reasoning on a claim that already carries a citation mark is asked to be explained, not cited or cut (exegenius-core/src/audit.rs)
- A citation quoting `\ref{`, `\eqref{`, `\cite` or `\label{` markup is rejected (exegenius-core/src/cite.rs)
- Of the run's 30 judged citation marks, the one labelled contradicted agrees with its quote, and the two labelled unsupported quote a fragment beside the claim

## Missing
- No run with added reasoning remembered across rewrites
- No measure of how much of what a reader would stop at one audit reports
- No review of the full LaTeX run's other 18 pages against the paper
- No distinction, in the depth limit, between reasoning about a page's own mechanism and reasoning about what it uses

# W2b (preregistered): does Opus `xhigh` write the hard key-step pages better than Sonnet `high`?

**Status:** written and committed before any W2b run, at 23:56 UTC. It follows W2, whose table found Opus `xhigh` within noise of Sonnet `high`, and whose plan required a repeat before any writer proposal.

**The decision it informs:** whether the writer policy should use Opus `xhigh` for the hard key-step pages and Sonnet `high` elsewhere. On the hard page W2 found 8, 8, 8 for Opus against 8, 6, 7 for Sonnet, at about twice Sonnet's cost.

## Design

**Held fixed:** the plan (E9F4Y), the sources, and the task files rendered by `exegenius-params`. They are identical apart from the path, checked by diff.

**The runs:**

| Run | Configuration | Page |
|---|---|---|
| W2WSP15R | Sonnet `high` | 0015 (repeat) |
| W2WOP15R | Opus `xhigh` | 0015 (repeat) |
| W2WSP16 | Sonnet `high` | 0016 `rank-saving-recurrence` (a second hard page) |
| W2WOP16 | Opus `xhigh` | 0016 |

**Judging, J24:**
- **The pages:** the four new pages, with W2WSP15 and W2WOP15 re-judged in the same batch: six pages, three Opus judges each, 18 judgements.
- **The judge:** J22's page judge, unchanged. Page 0016's assignment is its task file's "This page in the argument" and "Steps this page derives".
- **Scoring:** the listed items only (`tools/pages.py`). For page 0016 these are S4.4, S4.4.1, S4.4.2 and S4.4.3: the assignment names S4.4 as the sub-step, with S4.4.1–S4.4.3 beneath it. The ids were fixed at 23:58 UTC, before any J24 verdict.

## Reading (fixed now)

**For each configuration:**
- on page 0015, pool its two runs: six judgements per configuration;
- on page 0016, its three judgements.

**Opus is better on a page** if its pooled faithful values are all at least Sonnet's median, and its median exceeds Sonnet's.

| Outcome | Action |
|---|---|
| Opus better on both hard pages, with no more major errors | propose a split writer policy (Opus `xhigh` for key-step pages, Sonnet `high` elsewhere) to the author, with its cost |
| Opus better on one page or neither | Sonnet `high` stays the writer for every page |

## Budget

| | Basis | Cache read |
|---|---|---|
| 4 writer runs | W2: Sonnet 4.1 M, Opus 8.1 M on the hard page | ~25 M |
| J24, 18 judgements | J22: about 2 M each | ~35–45 M |

## Ledger

| Item | Status |
|---|---|
| W2WSP16, W2WOP16, W2WSP15R | accepted; subagents ended at 00:00, 00:04 and 00:06 UTC |
| W2WOP15R | accepted; subagent ended at 00:18 UTC (1,187 s, 50 calls) |
| J24 | prepared at 00:18 (`scratchpad/prep-j24.py`, seed 20261037; key P01 W2WOP16, P02 W2WOP15, P03 W2WSP16, P04 W2WSP15R, P05 W2WSP15, P06 W2WOP15R); all 18 judges launched at 00:19 |
| J24 | complete at 00:25 UTC, all 18 verdicts (`docs/journal/2026-10-14-w2b-writer/judge/results.txt`) |

## Result (J24, 00:25 UTC; read by the pooled rule above, `tools/pages.py --pooled`)

| Page | Opus `xhigh`, faithful of assigned (each judge) | Sonnet `high` | Opus major errors | Sonnet major errors | Opus better (rule) |
|---|---|---|---|---|---|
| 0015 (8 items) | W2WOP15 8, 8, 8; W2WOP15R 8, 7, 8. Pooled median 8, minimum 7 | W2WSP15 7, 6, 5; W2WSP15R 5, 5, 5. Pooled median 5 | 0 in all six | W2WSP15 0; W2WSP15R 2, 1, 1 | **yes** (7 ≥ 5, and 8 > 5) |
| 0016 (4 items) | W2WOP16 4, 4, 4 | W2WSP16 3, 3, 3 | 0 | 0 | **yes** (4 ≥ 3, and 4 > 3) |

**By the table: Opus is better on both hard pages, with no more major errors. The action is to propose a split writer policy to the author:** Opus `xhigh` for key-step pages, Sonnet `high` elsewhere, with its cost.

**What the differences are** (from the verdicts, read after the result):
- **Page 0015:** W2WSP15R makes the same major error in all three judgements. It states the inner products of the triple indicators, $\langle t_S,t_T\rangle=|S\cap T|-1$ and $\langle t_T,t_T\rangle=2$, right after defining both forms. Those values hold only for the rational form; under the phase network's $\mathbb F_2$ dot product (paper lines 867–870) they are false, and the line they span would be degenerate. One judge also finds "a nonzero residual contains a coordinate unit vector" false. W2WSP15 has no major error but leaves 1–3 sub-steps stated rather than derived.
- **Page 0016:** every judge finds the same gap in W2WSP16. S4.4.1, the size of the saving $\eta=(Wm-s)/(Wm)$, is stated: the page copies the two exact values and asserts that both exceed $20\cdot2^{-50}$, without computing them. W2WOP16 derives it.

**Judge drift between batches.** W2WSP15 was judged 8, 6, 7 in J22 and 7, 6, 5 here; W2WOP15 was 8, 8, 8 in both. Pooling both runs and re-judging the W2 pages in the same batch is what the design was for: W2's one-run reading ("within noise") did not hold once Sonnet's second run and the shared batch were in.

**The limits:**
- one paper and two hard pages;
- two runs per configuration on page 0015, and one on page 0016, where the gap is a single sub-step (consistent across the three judges);
- on the easier page 0014 (W2), Opus and Sonnet were both adequate, so Opus there buys nothing measured.

**Measured cost per page, cache read** (`perrun.py`):

| Page | Opus `xhigh` | Sonnet `high` | Ratio |
|---|---|---|---|
| 0015 | 8.13 M, 8.06 M (1,187–1,226 s) | 4.06 M, 3.92 M (509–608 s) | ~2.0× |
| 0016 | 2.09 M (348 s) | 0.66 M (116 s) | ~3.2× |
| 0014 (W2) | 1.92 M | 0.34 M | ~5.6× |

- The added cost is about 1.4–4.2 M cache read per hard page, in measured tokens. Opus's higher price per token is not converted here (runbook §5).
- How many pages count as "key-step" is part of the proposal. In E9F4Y's wiki, 0014–0016 carry the central saving's step S4; the wiki has 36 write pages.

**J24 measured 9.67 M cache read** for 18 judgements (0.54 M each). The four W2b runs measured 14.7 M.

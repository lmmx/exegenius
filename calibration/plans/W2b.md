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
| W2WSP16, W2WOP16, W2WSP15R | accepted (00:0x UTC) |
| W2WOP15R | accepted at 00:17 UTC (1,187 s, 58 tool uses) |
| J24 | prepared at 00:18 (`scratchpad/prep-j24.py`, seed 20261037; key P01 W2WOP16, P02 W2WOP15, P03 W2WSP16, P04 W2WSP15R, P05 W2WSP15, P06 W2WOP15R); all 18 judges launched at 00:18 |

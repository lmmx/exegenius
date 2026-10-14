# E8b (preregistered): confirming the variant sentence on the other five extractions, and its run variance

**Status:** written and committed before any E8b run (runbook item 4b). It follows E8's supported result. The pipeline default is unchanged.

## The decision it informs

**Whether to propose the variant sentence for the argument task.**
- E8 supported it on F1, F4 and F8 (faithful 18, 18, 19 against 14, 3, 4).
- A proposal resting on three extractions is weaker than one on eight.
- E6 and E7 used eight; this brings E8 to the same eight.

**Whether one run per extraction is a fair basis:** a repeat on F4 measures X's own run-to-run variance.

## Design

**The runs:** six fresh projects (copies of E6's Fk, background accepted), rendered by `exegenius-chains` (4f5ee26). Each task file differs from E7FkW's only in the sentence and the path (checked by diff). `general-purpose`, `model: "sonnet"`, `effort: "high"`, the prompt word for word.

| Run | Extraction |
|---|---|
| E8F2X, E8F3X, E8F5X, E8F6X, E8F7X | F2, F3, F5, F6, F7 |
| E8F4X2 | F4, a repeat |

**Stage 1, the checklist screen (J17):**
- **The plans:** the six new X plans, E8F4X (the first run), and E7F2W, E7F3W, E7F5W, E7F6W and E7F7W, re-judged. That is 12 plans.
- **The judges:** two each, the first Opus and the second Sonnet (allowed by K2): 24 judgements.

**Stage 2, primary confirmation (J18), only if stage 1 passes:**
- the five new X plans with their W plans, plus E8F4X and E8F4X2;
- three judges each, plus anchors: 38 judgements;
- J3's instructions, unchanged.

**Stage 2's timing:** it is expected to fall after the 18:30 UTC reset, under runbook §5's threshold.

## Reading (fixed now)

**Beyond judge noise:** the plans' judge ranges do not overlap.

**Stage 1 passes** if X's checklist is above W's beyond noise on at least 4 of the 5 new extractions.

**Stage 2 supports the sentence on fresh extractions** if, on at least 4 of 5:
- X's faithful range is above W's;
- the gate passes by majority;
- and X has no more major errors than W.

**Across all eight extractions (E8 and E8b), reported:** the exact one-sided sign test on faithful differences.

**Run variance:** E8F4X against E8F4X2 on both measures. The variance condition (runbook §6) is met if the difference is smaller than the smallest X-over-W gain.

| Outcome | Action |
|---|---|
| supported on ≥ 4 of 5, and the variance condition met | the proposal of the sentence to the author is complete for this paper. Its generality rests on the second document |
| not supported | E8's three-extraction result stands as a screen. The proposal is withdrawn until it is understood why it fails on the others |

## Budget

These are estimates, not converted.

| | Basis | Cache read |
|---|---|---|
| Runs | E8's runs: 16.8 M for 3, by `spend.py` | ~34 M |
| Stage 1, 24 checklist judgements (J15: 16.0 M for 18) | | ~21 M |
| Stage 2, if reached: 38 primary judgements (~3 M each) | | ~115 M, after the reset |

**Stopping:** one run per new extraction and one repeat.

**Amendment, 15:47 UTC, before any J18 judgement:**
- J18 also contains E9's three Y plans (E9F2Y, E9F4Y, E9F8Y) and E8F8X, three judges each (`plans/E9.md`). That makes 38 + 12 = 50 judgements.
- E8b's criteria, plans and judges are unchanged. The added plans only share the batch, so that E9's comparisons are within it.
- J18 launches when E9's runs are accepted.

## Ledger

| Run | Extraction | Window (UTC) | Seconds | Calls | Cache read | Cache write | Outcome |
|---|---|---|---|---|---|---|---|
| E8F2X | F2 | 15:09–15:25 | 914 | 19 | 3.08 M | 0.47 M | accepted, first submit |
| E8F3X | F3 | 15:09–15:25 | 952 | 19 | 3.40 M | 0.48 M | accepted, first submit |
| E8F5X | F5 | 15:09–15:21 | 691 | 18 | 2.51 M | 0.46 M | accepted, first submit |
| E8F6X | F6 | 15:09–15:26 | 993 | 33 | 7.15 M | 0.47 M | accepted, first submit (a second `submit` after acceptance was refused as not pending) |
| E8F7X | F7 | 15:09–15:32 | 1,329 | 31 | 7.72 M | 0.54 M | accepted, first submit |
| E8F4X2 | F4 (repeat) | 15:09–15:23 | 805 | 21 | 3.48 M | 0.44 M | accepted, first submit |

- No `max_tokens` stop in any run.
- The six ran concurrently with FB1's Fable run and judging (J19), and with J15/J16's tail.

## Stage 1 result (J17, 15:39 UTC; `docs/journal/2026-10-14-e8b-confirm/checklist/`)

Seed 20261031. Each plan was judged by Opus first, then Sonnet. The analysis was run with `tools/ranges.py`.

| Extraction | X (sentence) | W (E7) | Reading |
|---|---|---|---|
| F2 | 21, 19 | 15, 14 | above |
| F3 | 18, 18 | 14, 16 | above |
| F5 | 20, 19 | 16, 16 | above |
| F6 | 18, 17 | 19, 17 | within noise |
| F7 | 23, 22 | 11, 10 | above |

**By the table: stage 1 passes** (above beyond noise on 4 of 5). Stage 2 (J18) follows.

**Run variance on the checklist:** E8F4X 23, 21 against E8F4X2 22, 21, which is within noise. The difference of at most 1 is smaller than the smallest X-over-W gain beyond noise (F3: 18 against 16, a margin of 2). The primary measure's variance is read in J18.

**F6:** E7F6W was already the strongest W plan of all eight (19, 17; the next is 16, 16). It had L2, L4, M1, M2, S1 and S3, which most W plans lack. X matched it rather than exceeding it, so this is a tie at a high W level, not a loss.

**Measured cost against the estimate:**

| | Estimated | Measured (`spend.py`) |
|---|---|---|
| Runs, cache read | ~34 M | 27.3 M |
| Runs, cache write | | 2.85 M |
| J17 (24 judgements), cache read | ~21 M | 23.8 M |
| J17, cache write | | 2.19 M |

**Stage 2's timing:**
- At 15:39 the usage measured since the 14:20 reset was about 361 M cache read: 260 M by subagents and 101 M by the development session itself.
- That is past runbook §5's 300 M pause threshold, so J18 waits for a meter reading or for the 18:30 reset.

## Stage 2 result (J18, complete at 19:24 UTC; `docs/journal/2026-10-14-e8b-confirm/judge/results.txt`)

Seed 20261032. Opus judges, three per plan, by majority (`tools/majority.py`). J3's instructions.

| Extraction | X faithful (median; each) | W faithful (median; each) | X above W? | Gate X, W | Major errors X, W |
|---|---|---|---|---|---|
| F2 | 16; 16, 16, 16 | 13; 14, 13, 13 | **above** | pass, pass | 0, 0 |
| F3 | 16; 16, 16, 16 | 13; 14, 11, 13 | **above** | pass, pass | 0, 0 |
| F5 | 16; 16, 16, 15 | 16; 16, 16, 12 | no (overlap) | pass, pass | 0, 0 |
| F6 | 16; 16, 16, 16 | 16; 16, 16, 16 | no (tie) | pass, pass | 0, 0 |
| F7 | 17; 17, 17, 17 | 11; 11, 11, 10 | **above** | pass, pass | 0, 0 |

**By the table: not supported.**
- X is above W on 3 of 5, and the rule needs 4.
- The table's action: E8's three-extraction result stands as a screen, and the proposal of the E8 sentence alone is withdrawn until it is understood why it does not hold on the others.

**X is never below W.**
- On F5 and F6, X ties W at 16.
- These are the two extractions where W was already strongest in J17 and J18.

**Across all eight extractions (reported):**
- **The data:** E8, J16 (F1, F4, F8), and E8b, J18 (F2, F3, F5, F6, F7). Each pair is compared within its own batch.
- **Faithful differences X − W, by median:** +4, +3, +3, +15, 0, 0, +6, +15.
- **The sign test:** 6 positive, 0 negative, 2 ties. The exact one-sided sign test, ties excluded, gives p = 1/64 ≈ 0.016.

**Run variance:**
- E8F4X: 18 (18, 18, 17). E8F4X2: 17 (17, 16, 18). The ranges overlap, and the medians differ by 1.
- That is smaller than the smallest X-over-W gain beyond noise (3, on F2 and F3), so the variance condition is met.

**Why it does not hold on F5 and F6** (analysis of the J18 verdicts, after the result):

| Plan | Core results not faithful by majority |
|---|---|
| E7F5W | `label-residual-budget`, `matrix-shear`, `packed-selected-bit-addition` |
| E8F5X | `label-residual-budget`, `common-frame-identity`, `time-accounting` |
| E7F6W | `label-residual-budget`, `complex-phase-interface`, `phase-frame-recursion` |
| E8F6X | `label-residual-budget`, `common-frame-identity`, `matrix-shear` |

- The sentence changes which results are missed, not how many, on these two.
- **One result is missed throughout:** `label-residual-budget` is missed by every W plan in J18, and by every X plan except E8F8X (E8F4X and E8F4X2 included). This is the result whose sub-steps are the per-edge residual table (checklist L1–L5).
- X plateaus at 16–18 largely because of it. E9's sentence targets it: every E9 plan is faithful on all 19 (`plans/E9.md`).
- The follow-up that tests this reading against W is E10 (`plans/E10.md`).

**Measured cost:**
- J18: 50 judgements, 241.0 M cache read and 11.8 M cache write (`spend.py`), about 4.8 M per judgement.
- The estimate was ~3 M per judgement, from J16. The plans are longer now.
- E8b's and E9's estimates together were ~175 M, so J18 came in at 1.38×.

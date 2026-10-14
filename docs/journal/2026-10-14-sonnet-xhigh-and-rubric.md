# 2026-10-14: why Sonnet xhigh regressed, and whether the rubric still discriminates

Two questions the author asked after S2. Both are answered from evidence already collected: transcripts, plans, and the 53 judgements of compared plans in J8, J9 and J10. **No model run and no judging was paid for here.**

## 1. Sonnet `xhigh` (S2F1SX) against Sonnet `high` (E7F1W), same task file

### Evidence (transcripts, registry)

**The `xhigh` run's main planning call was cut off at the output limit.**
- At 08:29:03 the call ended with `stop_reason: max_tokens` and `output_tokens: 128000`, after 14.5 minutes. It contained only thinking: no plan text and no tool call.
- The next call's context was 64 tokens larger than the truncated call's. So the truncated thinking was not carried forward, and that work was lost.
- This is the only `max_tokens` stop among all subagents of this session (E4–E7, S2, S3, all judges) and all registry records with transcripts. The registry now records `max_tokens_stops` per subagent.
- It is also the source of the "~128 k output" figure in S2's ledger: the one recorded usage value was this truncated call.

**After the truncation, the run re-planned in less time than Sonnet `high` took:**

| | Sonnet `high` (E7F1W) | Sonnet `xhigh` (S2F1SX) |
|---|---|---|
| Reading the paper | 06:32:03–06:32:30, the whole paper in 9 reads | 08:12:26–08:13:42, the whole paper in 12 reads |
| Main planning call | one, 6.5 min (182 k signature chars) | one of 14.5 min, **truncated and lost** (408 k); then one of 5.6 min (154 k) |
| Writing the plan | one build script in 4 appended chunks; small thinking between (1–3 k) | five separate part files, each preceded by fresh deliberation (13–34 k) |
| Submits | 1, accepted | 1, accepted |
| Plan | 205 nodes, 168 quotes; top-level sections of 42, 31, 25, 27, 18, 44 and 16 nodes | 176 nodes, 143 quotes; sections of 44, 25, 15, 11, 13, **1**, **4** and **62** nodes, with S8 missing from the numbering |

**The four results it lost** (all three J9 judges agree; their notes):

| Result | What the plan has |
|---|---|
| common-frame-identity | a single `infer` step with no sub-steps, so stated, not derived |
| complex-phase-interface | stated in S2.1, without the endpoint corrections and the undoing of the signed exchange |
| matrix-shear | the factorisation $A=E_1\Pi E_2$ asserted, without its elimination proof (lines 1375–1391) |
| chirp-permutation-cancellation | the chirp identity as a single `infer` step (S6), without the coefficient computation |

### Interpretation (not established)

**The likeliest reading is that the regression is mechanical, not a sign that "more reasoning hurts".**
- The run spent its longest deliberation on a call that produced nothing.
- It then rebuilt the plan from a shorter deliberation, writing it in pieces.
- The pieces written last are the thinnest: S6 and S7 hold 1 and 4 nodes.
- The lost results are exactly of the kind a compressed re-plan would leave as one-line `infer` steps.

**What cannot be checked:**
- The thinking itself is encrypted, so what was lost cannot be read.
- One run cannot separate this mechanism from ordinary run-to-run variation.
- Sonnet `max` also wrote in pieces (9 part files) and scored 18. So writing in pieces is not harmful in itself; losing the plan before writing is the difference.

### Implication (a risk, estimated)

**How close the other runs came to the cap.** The only fixed point is S2F1SX's call: 408 k signature chars at 128 k output tokens. Taking that, very roughly, as about 3.2 signature chars per token (a single point, and not known to hold across models):
- **Sonnet `max`'s longest call** (397 k chars) came within a few percent of the cap.
- **Opus `max`'s** (379 k) was near it too.
- **Opus `xhigh`'s longest calls** (116–261 k) and **Sonnet `high`'s** (≤ 182 k) were well below.

**The risk of adopting `max`, or Sonnet `xhigh`:**
- Effort above `high` lets a single planning call run into the output limit. When it does, the call's work is silently discarded.
- The submit still succeeds, so nothing in the pipeline flags it.
- Any adoption of `max`, or of Sonnet `xhigh`, should come with either a check for `max_tokens` stops, or a task design that writes the plan incrementally, so that a truncated call loses less.

## 2. Does the faithful-derivation measure still discriminate, or does it reward detail?

### Evidence

**The central gate no longer discriminates among configurations using the E7 instructions:**
- every compared plan in J9 and J10 passes it by majority (11 of 11);
- in J8, 8 of 8 E7 plans passed.

**Core derived is near its ceiling:**
- 14–19 across J9 and J10;
- 19 of 19 in 6 of the 11.

**Core faithful still spreads widely:** 7–19 in J9 and J10.

**It correlates with plan length.** The Spearman correlation of the median faithful count with the characters of claims and methods:

| Batch | ρ (plans) |
|---|---|
| J8 | 0.94 (16) |
| J9 | 0.41 (5) |
| J10 | 0.84 (6) |
| Pooled | **0.92** (27) |

The correlation with node count is weaker: 0.61 pooled, and −0.06 in J10.

**But length does not decide the score:**

| Pair or plan | Length | Faithful |
|---|---|---|
| S2F1SX against S2F1OX, same extraction | 43 k against 41 k chars | 13 against 18 |
| Sonnet `max` (the longest plan) | 67 k | 18, equal to Opus `xhigh` at 41 k |
| Opus `max` | 56 k | 19, above Sonnet `max` |
| S3F8OX against E7F8W | 38 k against 30 k | 16 against 7 |

**The verdicts are item-specific:**
- 72% of the 258 `partial` verdicts name a specific missing step with its paper line numbers (e.g. "the elimination proof 1375–1391 is missing").
- **A free check against a mechanical marker.** For simultaneous-layer, whether the plan's text contains the depth recurrence $A(e)\le sA(e/m)+E$ predicts the judges' majority verdict in **26 of 27 plans**.
- **For complex-phase-interface,** the marker "weight additivity mod 4" appears in 12 plans, but the judges call only 4 of them faithful. They also require the phase frames and the endpoint corrections, a conjunctive standard. Only one plan was judged faithful without the marker.
- **Longer plans do not collect more minor errors:** ρ = −0.53 between minor errors and length.

**The information sits in a few items.** Faithful rates across the 53 judgements:

| Rate | Core results |
|---|---|
| ≥ 0.89 | 4 (multiplication-algorithm, synthetic-convolution, synthetic-transform, interchange-recurrence): they carry almost no information |
| 0.17–0.55 | 6 (label-residual-budget, complex-phase-interface, simultaneous-layer, bank-exchange-network, rank-saving, matrix-shear) |

The six low-rate items are where the configurations separate. Recurring themes in the partial notes:

| Theme | Notes |
|---|---|
| error or guard chain omitted | 71 |
| table omitted | 52 |
| asserted, not derived | 43 |
| the second (complex) construction | 30 |

### Reading

**The measure is not merely rewarding detail.**
- Its per-item verdicts track specific mechanisms.
- Plans of similar length differ by up to 5 points.
- The length correlation is largely because a faithful derivation needs its content written down.

**It is losing resolution in three places:**
1. **The gate:** saturated with the E7 instructions.
2. **The top of faithful:** binary per item, with only about six informative items. Opus `xhigh` (18), Sonnet `max` (18) and Opus `max` (19) cannot be separated, and a stronger configuration has at most one point to gain on F1.
3. **Within the hard items:** `partial` lumps a plan with three of four sub-steps together with one that has one. On the hardest items, that is exactly where the stronger configurations differ.

### Proposal (not applied; S3's rubric and every preregistered criterion are unchanged)

**A secondary, item-level mechanism checklist for the six discriminating results.** It is added beside the existing rubric, which stays the primary measure for continuity.

**For each of the six items:**
- 3–5 named sub-derivations, taken from the paper's derivation lines and from what the judges' partial notes repeatedly cite;
- each judged present or absent, with the plan step and paper line as evidence.

**Examples:**
- **simultaneous-layer:**
  - the statement;
  - the guard width;
  - the depth recurrence $A(e)\le sA(e/m)+E$;
  - the bound $B^{k+1}\le B^2d^\nu$.
- **complex-phase-interface:**
  - the phase frames $C_U$;
  - weight additivity mod 4;
  - the Hadamard conjugation to $aI+bX_v$;
  - the endpoint corrections and the undoing of the signed exchange.
- **bank-exchange-network:**
  - the bit coefficient check;
  - the complex coefficient check;
  - the parameters $h$, $N$, $m$.

**What it buys:**
- **Graded scores where the top configurations differ:** about 20–25 checks instead of 6 binary items.
- **Less room for holistic judgement,** so less judge variance.
- **Some checks scorable mechanically, for free,** like the $A(e)$ marker. That gives a judge-free cross-check on every plan.

**A length control in reporting.** A configuration's gain is read beside its plans' length, and claimed only when it also beats a comparator of similar length. S2F1SX against S2F1OX is such a pair.

**Validation before use, cheap:**
1. Write the checklist and its mechanical markers. This is analysis only.
2. Score the 11 existing J9 and J10 plans mechanically, where markers exist.
3. Score them with one judge per plan on the checklist only (about 11 short judgements), to check:
   - agreement with the markers;
   - that it separates Opus `xhigh`, Sonnet `max` and Opus `max` on F1, where faithful ties.

Only if it does would it become a measure for later experiments, preregistered as such.

## What existing evidence already answered (and was not paid for again)

| Question | Answered by |
|---|---|
| why Sonnet `xhigh` lost fidelity | the transcripts and J9's notes |
| whether faithful rewards length | the 27 plans judged in J8–J10 and their verdict notes |
| whether the judges' item verdicts are content-specific | the same, with a regex marker |

**Not answerable without new runs:**
- whether a non-truncated Sonnet `xhigh` run would match or beat `high`;
- whether the truncation risk at `max` is real in practice.

## Current State
- Sonnet `xhigh`'s S2 run lost its main planning call to the output-token limit (`max_tokens` at 128,000), re-planned in less time than Sonnet `high`, and wrote its plan in five pieces; its four lost results are the kind a compressed re-plan leaves as one-line steps
- It is the only `max_tokens` stop in the programme; the registry now records `max_tokens_stops` per subagent
- The faithful measure correlates with plan length (ρ = 0.92 pooled), but its item verdicts track specific mechanisms (a single marker predicts one item's verdict in 26 of 27 plans), and plans of similar length differ by up to 5 points
- The gate is saturated and the top of the faithful scale is near its ceiling; a secondary mechanism checklist on the six discriminating items is proposed, not applied

## Missing
- A repeat of Sonnet `xhigh` without truncation
- The checklist itself and its validation

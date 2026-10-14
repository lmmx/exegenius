# 2026-10-14: S2, model × effort above high with the E7 instructions (a one-run screen)

**The result.** On E6's extraction F1, with the E7 argument instructions, all five plans pass the central gate under majority judging. So the gate cannot separate them: it is at its ceiling. Faithful derivations and cost separate them.

| Configuration | Central (majority) | Faithful (median; 3 judges) | Seconds | Cache read |
|---|---|---|---|---|
| Sonnet `high`, the reference (E7F1W) | 8 (C1 mentioned) | 15 (15, 15, 16) | 693 | 3.06 M |
| Sonnet `xhigh` | 8 (C1 mentioned) | **13** (13, 13, 14) | 1,897 | 5.45 M |
| **Opus `xhigh`** | **9** | **18** (19, 18, 17) | 1,408 | 6.93 M |
| Sonnet `max` | 9 | 18 (19, 18, 17) | 2,785 | 24.83 M |
| Opus `max` | 9 | **19** (19, 19, 19), the rubric's ceiling | 2,021 | 15.93 M |

**The reading, by the preregistered rule (`calibration/plans/S2.md`).** A configuration is worth a repeat if it differs from the reference by more than judge noise on central steps or faithful derivations.
- **Opus `xhigh`, Sonnet `max` and Opus `max` qualify.** Each gains C1 and 3–4 faithful derivations, and no judge of the reference scored as high as any judge of these plans.
- **Sonnet `xhigh` qualifies too, in the wrong direction.** It is 2 faithful derivations below the reference, and it costs more. So by the rule it may be dropped on this run.

**My reading beyond the rule, labelled as such.** Opus `xhigh` gets the same result as Sonnet `max` and Opus `max` (within judge noise) for less, and it is the cheapest configuration that improved on the reference:

| | Seconds | Cache read |
|---|---|---|
| Sonnet `max` against Opus `xhigh` | 2.0× | 3.6× |
| Opus `max` against Opus `xhigh` | 1.4× | 2.3× |

**What one run on F1 cannot show:**
- **Whether Opus `xhigh` is better in general.** Run-to-run and extraction-to-extraction variation is large: E7's eight Sonnet-`high` plans ranged from 5 to 17 faithful derivations across extractions (J8). A 3–4 point gap on one extraction is inside that.
- **Anything about configurations stronger than these.** F1 is the extraction on which the reference already did best in E7, and three of four configurations reach or nearly reach the rubric's ceiling. A further run on F1, including a Fable run, has almost no room to show a gain.

## Design (preregistered before any result: `calibration/plans/S2.md`, 9fdd9da)

- **The task:**
  - E6's project F1 (background accepted), with the argument task rendered by exegenius branch `141026-exp-key-step-identity` (91c4fed);
  - byte-identical to E7F1W's task apart from the project path.
- **The subagents:**
  - a fresh `general-purpose` subagent per configuration, with the prompt `exegenius next` printed, word for word;
  - every turn on the intended model at the intended effort (transcripts);
  - every run accepted on its first submit, with no restarts.
- **The schedule:**
  - row 1 (Sonnet `xhigh`) ran alone;
  - rows 2–4 ran in parallel from 08:48 UTC, after the usage window reset (recorded in the ledger as a change of plan).
- **Judging, batch J9:**
  - the four plans and the reference E7F1W, blind, three independent judges each;
  - the anchors E1-P4 and E3-M3, one judge each;
  - J3's instructions, unchanged;
  - each central step decided by majority (`calibration/tools/majority.py`; output in `judge/results.txt`). Key: P01 Sonnet `max`, P02 Opus `max`, P03 reference, P04 anchor P4, P05 Sonnet `xhigh`, P06 anchor M3, P07 Opus `xhigh`.
- **The evidence:** wiki branch `141026-exp-s2-model-effort` (74b4094), runs `S2F1SX`, `S2F1OX`, `S2F1SM`, `S2F1OM`, with transcripts. The runs' own helper files are kept.

## Results (J9; `judge/results.txt`)

| Run | Central, majority (each judge) | C1–C9 | Gate | Derived (each) | Faithful (each) | Major errors |
|---|---|---|---|---|---|---|
| E7F1W, Sonnet `high` | 8 (8, 8, 9) | m R R R R R R R R | pass | 19 (19, 19, 19) | 15 (15, 15, 16) | 0, 0, 0 |
| S2F1SX, Sonnet `xhigh` | 8 (8, 8, 8) | m R R R R R R R R | pass | 17 (17, 17, 17) | 13 (13, 13, 14) | 0, 0, 0 |
| S2F1OX, Opus `xhigh` | 9 (9, 9, 9) | all R | pass | 19 (19, 19, 19) | 18 (19, 18, 17) | 0, 0, 0 |
| S2F1SM, Sonnet `max` | 9 (9, 9, 9) | all R | pass | 19 (19, 19, 19) | 18 (19, 18, 17) | 0, 0, 0 |
| S2F1OM, Opus `max` | 9 (9, 9, 9) | all R | pass | 19 (19, 19, 19) | 19 (19, 19, 19) | 0, 0, 0 |
| *anchor E1-P4* | 6 | m R R m m R R R R | fail | 17 | 14 | 0 |
| *anchor E3-M3* | 1 | | fail | 1 | 0 | 0 |

**Every plan states all 19 core results,** and none has a major error or a `wrong` step.

**What the stronger plans add.** The core results whose derivation differs between plans, by each judge's verdict (F faithful, d derived but not faithful, s stated only):

| Core result | Sonnet `high` | Sonnet `xhigh` | Opus `xhigh` | Sonnet `max` | Opus `max` |
|---|---|---|---|---|---|
| complex-phase-interface | ddd | **sss** | FFF | FFF | FFF |
| label-residual-budget | ddd | ddF | FFd | Fdd | FFF |
| simultaneous-layer | ddd | FFF | Fdd | FFF | FFF |
| common-frame-identity | FFF | **sss** | FFF | FFF | FFF |
| matrix-shear | FFF | ddd | FFF | FFF | FFF |
| chirp-permutation-cancellation | FFF | ddd | FFF | FFF | FFF |
| rank-saving, phase-frame-recursion, layer-recurrence, gaussian-resampling, time-accounting | one judge apart at most | | | | |

**What the judges' notes say the reference lacks** (all three agree):
- **C1:** the exchange $(X_a,Y_a)\mapsto(-Y_a,X_a)$ is reconstructed, but $h=100$, the banks indexed by triples, $N=\binom h3^3$ and $m=h^3$ never appear, so later steps use $N$ and $m$ undefined.
- **complex-phase-interface:** the phase frames $C_U=H^{\otimes m}\operatorname{diag}(i^{q_U})H^{\otimes m}$ are never defined. Nor is the mechanism (lines 1232–1257): weight additivity mod 4 for orthogonal vectors, and the Hadamard conjugation.
- **simultaneous-layer:** the guard-width bound is asserted. The arithmetic-depth recurrence $A(e)\le sA(e/m)+E$ that gives it (lines 2479–2560) is absent.

The Opus `max` plan has all three, with the judges' checks noted (e.g. "$C_0$ checked at 2674").

**So the gain is one level further down.** E7's sentence asks each counted or bounded sub-step for the identity that makes it hold. Sonnet at `high` answers it at the step the plan names, and stops. The stronger configurations follow it into the mechanism inside a cited proposition, and define the parameters a step's count rests on.

**Sonnet `xhigh` is worse than Sonnet `high` in different places.** It states two core results without deriving them (common-frame-identity, complex-phase-interface). It derives two others less faithfully. On one run this cannot be told apart from run-to-run variation. Effort above `high` did not, on this run, buy Sonnet fidelity until `max`.

## Judge agreement in J9

- **Central steps:** unanimous on 44 of 45 step verdicts across the five screen plans. The one split is the reference's C1 (two judges said mentioned, one reconstructed).
- **The gate:** unanimous in all 15 judgements.
- **Core derived:** identical across judges on every plan.
- **Faithful:** within-plan range at most 2.
- **Why agreement is so much higher than J8's** (where one re-judged plan in three changed gate verdict): plans near the ceiling are easy to agree on. This does not show that judges have become more reliable.
- **Batch drift on the same plan:**
  - E7F1W was judged in J8 at 17 faithful (first judgement) and here at 15, 15 and 16;
  - its C1 has now been called reconstructed by 2 of 5 judges across the two batches.
  - So comparisons across batches carry about two faithful derivations of drift. This is why the reference was re-judged here, not reused.
- **The anchors held:** E1-P4 at central 6, faithful 14; E3-M3 at central 1, faithful 0. Both are the same as in J8.

## Cost, measured (registry, `calibration/registry/experiments/S2.json`)

| Run | Calls | Seconds | Cache read | Cache write | Visible output, est. | Thinking signatures (proxy) | Quotes in the answer |
|---|---|---|---|---|---|---|---|
| Sonnet `high` (E7F1W) | 20 | 693 | 3.06 M | 0.42 M | 25 k | 199 k chars | 195 |
| Sonnet `xhigh` | 32 | 1,897 | 5.45 M | 0.72 M | 27 k | 713 k chars | 164 |
| Opus `xhigh` | 33 | 1,409 | 6.93 M | 0.93 M | 29 k | 413 k chars | 138 |
| Sonnet `max` | 63 | 2,785 | 24.83 M | 1.15 M | 61 k | 1,047 k chars | 209 |
| Opus `max` | 60 | 2,021 | 15.93 M | 1.12 M | 48 k | 538 k chars | 185 |

**Judging J9:** 17 judges (Opus, `high`), 309 calls, 42.4 M cache read, 3.42 M cache write, 110 k visible output (`tools/spend.py`).

**What the columns can and cannot carry:**
- **Cache read, cache write and seconds are measured.** They are kept separate and not converted to cost or quota (`calibration/budget.md`).
- **Visible output is an estimate:** text and tool calls at 4 characters per token.
- **Thinking is not stored in transcripts, only as an encrypted signature.** Its length is given as a proxy for hidden reasoning. It is not a token count, and it is comparable only within one model.
  - **Within Sonnet:** the signatures grow 3.6× from `high` to `xhigh` and 5.3× to `max`. Sonnet at `xhigh` reasoned far more than at `high` and wrote no more, and its plan was not better.
  - **Within Opus:** they grow 1.3× from `xhigh` to `max`, while calls nearly double (60 against 33).
- **Opus at `max` wrote and compiled a Rust checker of its own.** It parses each claim's LaTeX and confirms that every symbol a node `adds` is used below it (`runs/S2F1OM/exegenius/tasks/0003-argument/mathcheck/`). This is a notation check, not a check of the mathematics.

**A correction, found while writing this up.** The ledger's output figures for S2 mixed two bases:
- Sonnet `xhigh`'s "~128 k" came from the transcript's `usage.output_tokens`, which is recorded for that run only;
- the others were visible content.

The claim in `calibration/budget.md` §4 that Sonnet `xhigh` "wrote about 5× the output of `high`" was therefore wrong: visible output was about equal (27 k against 25 k).

The registry's output estimate also counted thinking signatures as output, which inflated every subagent's output figure. `tools/registry.py` and `tools/spend.py` now count visible content only, and report the signatures separately. `spend.py` previously kept only the last line of a multi-line message: close in effect, since the earlier lines are mostly thinking blocks.

**The usage meter** (`calibration/budget.md` §1):
- it read 0% at 08:45 (row 1 already done) and 5% at 09:25, with rows 3 and 4 still running and no judging yet;
- no reading has been taken since judging, so S2's whole cost against the meter is not yet known.

## What this decides

**Repeats, as screened (§6a: only where a repeat could change a decision):**

| Configuration | Repeat? | Why |
|---|---|---|
| **Opus `xhigh`** | **yes, but not on F1** | F1 has no headroom left. The repeat that could change a decision is on extractions where the reference is weak: F4 (central 7, faithful 7 in J8) and F8 (faithful 5). |
| Sonnet `xhigh` | no | Worse than the reference on one run, at 2.7× the time. Even a tie on repeat would not make it worth adopting. |
| Sonnet `max` | no, for now | The same result as Opus `xhigh` at 2.0× the time and 3.6× the cache read. It becomes relevant only if Opus `xhigh` fails on the harder extractions. |
| Opus `max` | no, for now | Within judge noise of Opus `xhigh` at 1.4× the time and 2.3× the cache read. Held as the fallback if Opus `xhigh` fails on F4 or F8. |

**The cell S2 did not test: Opus at `high` with the E7 instructions.** If it matches Opus `xhigh`, it would be the cheaper default, and it separates the model's effect from the effort's. It was screened in S1 only with older instructions.

**For the Fable pilot (`calibration/plans/FB1.md`):** FB1 as written would run Fable on F1, where S2's best configuration is at the ceiling, so it could show nothing. The proposal now in `calibration/plans/S3.md` runs first: Opus `high` and `xhigh` on F4 and F8. Its result decides where, if anywhere, Fable has room to show a gain.

## Limits

- **Scope:** one paper, one extraction, one run per configuration: a screen, not a ranking.
- **The reference set is unreviewed.**
- **The judges are all Opus at `high`.** Judging Opus plans with Opus judges could favour them, and nothing here tests that.

## Addendum: why Sonnet `xhigh` regressed (from the transcripts; no new runs)

Its main planning call hit the output-token limit (`max_tokens`, 128,000 tokens, all thinking). Nothing from that call was carried forward. The run then re-planned in 5.6 minutes and wrote the plan in five pieces.

This is the only such stop in the programme. Analysis, and the evidence kept apart from interpretation: `2026-10-14-sonnet-xhigh-and-rubric.md`.

## Current State
- All four S2 configurations and the reference pass the central gate on F1 under majority judging; the gate is saturated with the E7 instructions on this extraction
- Opus `xhigh`, Sonnet `max` and Opus `max` each gain C1 and 3–4 faithful derivations over Sonnet `high`, beyond J9's judge noise; Sonnet `xhigh` loses 2 at 2.7× the time
- Opus `xhigh` is the cheapest configuration that improved (2.0× the reference's seconds, 2.3× its cache read)
- The output estimators are corrected: thinking signatures are no longer counted as output
- The registry reads third judgements (`-c`), and J9 is recorded in `judging.json`

## Missing
- Opus `high` with the E7 instructions has not been run
- No configuration has been tested on the extractions where Sonnet `high` was weakest (F4, F8)
- There is no meter reading after J9
- Same-model judging bias is untested

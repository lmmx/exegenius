# 2026-10-14: E6, the key-step instruction confirmed on fresh extractions

**The result, by the preregistered rule: two of three criteria met. The instruction is not supported as a whole, because of the gate.**
- **Depth: met.** On eight fresh extractions, Sonnet-high plans given the key-step instruction reconstructed more of the central saving than their paired baselines in **8 of 8** pairs. The median rise was +3 steps (exact one-sided sign test, p = 0.004).
- **Errors: met.** The instructed plans made **no** major errors. One baseline plan made one.
- **The gate: not met.** Two of 8 instructed plans passed the central gate, against 0 of 8 baselines. The rule asked for at least 4 of 8.
- **Also measured:**
  - core derivations rose in 8 of 8 pairs (median +6.5, p = 0.004);
  - faithful derivations rose in 7 of 8 (median +5, p = 0.035).

**E4's failure did not recur.** E4 had failed on errors, not depth. Here, on new extractions, the instruction deepened every plan without adding errors.

**What still blocks the gate is one step: C4, rank as dimension change.** It has two parts:
1. for nested labels $U\subseteq V$, $P_V-P_U$ has rank $\dim V-\dim U$;
2. the $N$ source edges are rank-one exceptions.

**How the instructed plans fell short:**
- **C4:** reconstructed by only 3 of the 8 instructed plans. Of the six that failed the gate, five fell short on C4. In those, the judges found one part of C4 and not the other (judges' notes, below).
- **C5 (the budget) and C6 (the strict saving):** reconstructed by 7 of 8 instructed plans, against 0 and 1 of 8 baselines.

**The decision.** The instruction is a consistent, error-neutral improvement in plan depth across E4 and E6. It does not by itself make plans reach the gate.

Putting it on `master` is a pipeline change for the author. It would change the argument task for every document. It goes to the author with this evidence, as a proposal; this session does not merge it.

The gate's remaining gap is a single, two-part step. A targeted follow-up is designed below, not run.

## Design (preregistered: `calibration/plans/E6.md`, f07da93)

**The projects:**
- eight fresh planning projects, F1–F8, each with E1's configuration and paper, byte for byte;
- extract and background ran with the baseline binary, master af36d6d. The variant changes only the argument instructions, and its extract task file was checked to be identical;
- after the background task was accepted, each project was copied:
  - **FkB:** its argument task rendered by the baseline;
  - **FkV:** its argument task rendered by the variant (`131026-exp-key-step`, 0c29556);
- in every pair the task files differ only in the key-step paragraph (checked by diff).

**The subagents:** every task went word for word to a fresh `general-purpose` subagent, `model: "sonnet"`, `effort: "high"`. All 32 accepted subagents ran on `claude-sonnet-5-5` at `high` on every turn (transcripts).

**The evidence:** wiki branch `141026-exp-e6-key-step-confirm` (834fb5a).

**Judging, batch J7** (`2026-10-14-e6-key-step-confirm/judge/`):
- **Plans:** 18, blind, seed 20261021. The anchors are E1-P4 and E3-M3.
- **Instructions:** J3's, unchanged.
- **Second judgements:** four, drawn by seed. They fell on F3B, F5V, F3V and M3.

**Deviations:**
- **The usage limit.** The account's usage limit (HTTP 429) killed 10 of the 16 first argument attempts before they submitted: F1V, F2V, F3V, F4V, F5B, F5V, F7B, F7V, F8B and F8V.
  - After the reset, each was retried with the same prompt and parameters.
  - Before every retry, the task's scratch directory and any answer file were moved aside, as the plan required after E4's B4 deviation. Three killed attempts had written drafts (F1V, F2V, F4V).
  - So no retry saw a killed draft. The killed transcripts and drafts are kept apart on the evidence branch.
- **The orchestrator:** the development session (Opus, high), with its own permissions. It only passed prompts.

## Results (J7, first judgements; `judge/results.txt`)

| Pair | Central, B → V | Derived, B → V | Faithful, B → V | Gate, B / V | Major errors, B / V |
|---|---|---|---|---|---|
| F1 | 4 → 6 | 9 → 13 | 1 → 5 | fail / fail | 0 / 0 |
| F2 | 4 → 7 | 9 → 16 | 4 → 9 | fail / fail | 0 / 0 |
| F3 | 4 → 9 | 12 → 18 | 6 → 11 | fail / **pass** | 0 / 0 |
| F4 | 4 → 8 | 12 → 15 | 2 → 6 | fail / fail | 0 / 0 |
| F5 | 6 → 7 | 3 → 18 | 0 → 10 | fail / fail | 1 / 0 |
| F6 | 2 → 5 | 6 → 8 | 2 → 0 | fail / fail | 0 / 0 |
| F7 | 6 → 8 | 7 → 18 | 3 → 13 | fail / **pass** | 0 / 0 |
| F8 | 3 → 7 | 7 → 17 | 2 → 11 | fail / fail | 0 / 0 |
| *anchor E1-P4* | 6 | 19 | 14 | fail | 0 |
| *anchor E3-M3* | 1 | 1 | 0 | fail | 0 |

- **Core results stated:** 17–19 of 19 in every plan. No item was judged wrong in any plan.

**Steps reconstructed,** out of 8 plans per arm:

| Step | C1 | C2 | C3 | C4 | C5 | C6 | C7 | C8 | C9 |
|---|---|---|---|---|---|---|---|---|---|
| baseline | 2 | 8 | 4 | **0** | **0** | 1 | 5 | 8 | 5 |
| variant | 2 | 8 | 8 | **3** | **7** | 7 | 8 | 8 | 6 |

**The criteria** (`calibration/plans/E6.md`):

| Criterion | Required | Observed | Met |
|---|---|---|---|
| 1. Depth | V above B in ≥ 6 of 8 pairs, median difference ≥ +2 | 8 of 8 (p = 0.004), median +3 | **yes** |
| 2. The gate | V passes in ≥ 4 of 8, and more than B | 2 of 8, against 0 of 8 | **no** |
| 3. Errors | ≤ 2 of 8 V plans with a major error, and V ≤ B + 1 | 0 of 8, against 1 of 8 | **yes** |

**Calibres** (`calibration/registry/calibre-pass-rates.txt`, exact 95% intervals):

| Arm | Overview | Technical | Research |
|---|---|---|---|
| baseline | 7/8 (0.47–1.00) | 2/8 (0.03–0.65) | 0/8 (0.00–0.37) |
| variant | 8/8 (0.63–1.00) | **7/8** (0.47–1.00) | 2/8 (0.03–0.65) |

The Technical calibre (Overview plus ≥ 10 core results derived) is where the instruction changes the most: 7 of 8 against 2 of 8.

**Where the instructed plans fail the gate:**

| Plan | Failed on |
|---|---|
| F1V | C4, C9 |
| F2V | C4 |
| F4V | C4 |
| F5V | C9 (C4 reconstructed) |
| F6V | C4, C5, C6 |
| F8V | C4 |

**What the judges found missing in C4** (first judgements, variant plans marked "mentioned"):
- **F1V:** records the $N$ rank-one source exceptions, but never states that $\operatorname{rank}(P_V-P_U)=\dim V-\dim U$.
- **F2V:** asserts that an edge costs its dimension gain and counts the $N$ source ranks; it never gives the projection identity that makes this true.
- **F4V:** asserts that rank equals dimension change and refers to the $N$ source ranks; the projections never appear.
- **F6V:** states the identity but omits the source exceptions.
- **F8V:** alludes to both parts and states neither.

In every case the plan has one half of the step, or its conclusion without its mechanism.

**Resources,** argument subagent, medians (ranges) (registry; output estimated):

| Arm | Cache read | Output | Seconds |
|---|---|---|---|
| baseline | 2.12 M (1.76–2.93) | 43 k (38–62) | 398 (342–551) |
| variant | 2.64 M (2.04–5.75) | 67 k (36–99) | 619 (330–901) |

- **The ratio:** as in E4, about 1.6 times the output and the seconds.
- **Killed first attempts:** not counted.

**Judge stability (second judgements):**
- **M3:** 9 of 9 steps agree, 1 against 1.
- **F3V:** 8 of 9 agree, 9 against 8, gate passed both times.
- **F5V:** 8 of 9 agree, 7 against 8. The second judge passes its gate, which raises the variant's gate count to 3 of 8, still short of 4.
- **F3B:** 5 of 9 agree, 4 against 6. Even so, pair F3's difference stays positive (+3 instead of +5).
- **The anchor E1-P4:** 6, as in J6.

None of the second judgements changes a criterion's outcome.

## Across E4 and E6 (interpretation, not a preregistered pooled test)

| | E4 (5 pairs) | E6 (8 pairs) | Together (13 pairs) |
|---|---|---|---|
| Central count higher with the instruction | 4 (1 tie) | 8 | 12, 1 tie |
| Core derived higher | 5 | 8 | 13 |
| Gate passes, V / B | 3 / 0 | 2 / 0 | 5 / 0 |
| Plans with a major error, V / B | 1 / 0 | 0 / 1 | 1 / 1 |

- **Depth:** the effect is the most consistent seen in this programme, across two sets of extractions.
- **Errors:** across 13 instructed plans, one plan erred (E4's V1); across 13 baselines, one erred. E4's error criterion failed on that single plan, and E6 did not reproduce it.
- **The gate:** it passes in 5 of 13 instructed plans. That is the only configuration so far to pass it more than once, but it is not a reliable route to the Research calibre.

## Next (designed, not run)

**C4 is a two-part, short step.** Its half-statement is the most common reason an instructed plan fails the gate. Two smallest tests would tell which lever closes it:
1. **An instruction-side test.** It is general, and names no paper. Add to the key-step paragraph: *"State the identity or lemma that makes each step of the key step's count true, and every exception to it."* Paired on the same eight extractions (F1–F8), V against V′.
   - **Primary:** C4 reconstructed.
   - **Secondary:** gate passes and major errors.
   - **Cost:** 8 argument tasks and one judging batch, since the V arm exists.
2. **A check-side test.** The pipeline would audit the plan's key step for an unsupported claim before writing, with a check task, not a prompt change.
   - **What it involves:** a software change for the author's review (`calibration/software-plan.md`).

Test 1 is cheaper and reversible. It changes the same paragraph that E4 and E6 tested.

## Current State
- On eight fresh extractions, a one-paragraph key-step instruction raised the central count of Sonnet-high plans in 8 of 8 pairs (median +3, exact one-sided p = 0.004) and core derivations in 8 of 8 (median +6.5), with no major errors against one in the baselines, in batch J7
- Instructed plans reach the Technical calibre in 7 of 8 against 2 of 8
- Instructed plans passed the central gate in 2 of 8 against 0 of 8, short of the preregistered 4 of 8; five of the six that failed fell short on C4, stating either the projection-rank identity or the $N$ rank-one source exceptions but not both
- Across E4 and E6 the instruction deepened every one of 13 paired plans on core derivations and passed the gate in 5 of 13 against 0 of 13; it stays on exegenius branch `131026-exp-key-step` pending the author's decision
- E6's runs and transcripts are on wiki branch `141026-exp-e6-key-step-confirm` (834fb5a)

## Missing
- No plan configuration passes the central gate reliably
- No test of an instruction that asks for the identity and exceptions behind the key step's count
- No second document
- No measure of whether instructed plans lead to wikis that carry the central chain

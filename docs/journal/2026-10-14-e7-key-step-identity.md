# 2026-10-14: E7, the key step asks for its identities and exceptions

**The result, by the preregistered rule: supported, all four criteria met.** I added one general sentence to the key-step instruction, asking each counted, bounded or equated sub-step for the identity behind it and its exceptions. On the same eight extractions:
- **C4 (rank as dimension change):** reconstructed in **8 of 8** plans with the sentence (W), against 5 of 8 without it (V, E6's instructed plans), judged in the same batch. The three discordant pairs all favour W (exact one-sided p = 0.125).
- **The gate:** passed in **8 of 8** W plans, against 5 of 8 V plans.
- **Depth and errors:** the central count did not fall (median W − V = +0.5); faithful derivations rose in 7 of 8 pairs (p = 0.035); W made no major errors, V one.

**W is the first configuration to reach the Research calibre in every run:** 8 of 8, exact 95% interval 0.63–1.00.

**How strong this is.** The margin over V is three plans, and the same batch measured how far a single judge moves:
- **The same eight V plans, judged blind in J7 and again in J8:**
  - agreed on 64 of 72 central steps;
  - but changed gate verdict in 3 of 8 plans (all fail → pass);
  - and changed C4 in 2 of 8.
- **The consequence:** the gate is a conjunction of four steps, so one judge's view of it is noisy.
- **Two kinds of evidence:**
  - W's 8 of 8 within one batch is strong evidence that the sentence does not hurt, and good evidence that it helps;
  - the size of the help over V is within single-judge noise.

**The decision.** The evidence across E4, E6 and E7 supports a proposal to the author to put on `master` the key-step paragraph with this sentence, for the argument task. It is a pipeline change, so it is the author's to make. This session does not merge it.

**A methodological change follows, and it is within this programme:** the central gate should be judged by the majority of three independent judges per plan. That is proposed below for every future comparison.

## Design (preregistered: `calibration/plans/E7.md`, 8fe040c)

- **W:** for each of E6's projects F1–F8 (background accepted), a copy whose argument task was rendered by exegenius branch `141026-exp-key-step-identity` (91c4fed). It carries E6's key-step paragraph plus:

  > Where a sub-step concludes from a count, a bound or an equality, state the identity, lemma or definition that makes it hold, and every case the sources treat as an exception to it.

- **Checked by diff:** the task files of FkV and FkW differ only in that sentence and the project path.
- **The subagents:** `general-purpose`, `model: "sonnet"`, `effort: "high"`, with the prompt word for word. All eight ran on `claude-sonnet-5-5` at `high` every turn. No restarts.
- **The comparison:** W against V (E6F1V–E6F8V), both judged blind in batch J8 (seed 20261022), with the anchors E1-P4 and E3-M3 and J3's instructions unchanged.
- **Second judgements:** four, drawn by seed. They fell on E6F3V, E7F1W, E6F8V and E7F6W (P01, P06, P10, P12).
- **The evidence:** wiki branch `141026-exp-e7-key-step-identity` (c89a3fd), whose base is E6's branch.

## Results (J8, first judgements; `judge/results.txt`)

| Pair | Central, V → W | C4, V → W | Gate, V / W | Faithful, V → W | Major errors, V / W |
|---|---|---|---|---|---|
| F1 | 9 → 9 | R → R | pass / pass | 4 → 17 | 0 / 0 |
| F2 | 8 → 8 | R → R | pass / pass | 8 → 14 | 0 / 0 |
| F3 | 9 → 9 | R → R | pass / pass | 12 → 14 | 0 / 0 |
| F4 | 6 → 7 | m → **R** | fail / **pass** | 5 → 7 | 0 / 0 |
| F5 | 8 → 9 | R → R | pass / pass | 9 → 14 | 0 / 0 |
| F6 | 5 → 9 | m → **R** | fail / **pass** | 0 → 17 | 1 / 0 |
| F7 | 9 → 9 | R → R | pass / pass | 11 → 12 | 0 / 0 |
| F8 | 7 → 8 | m → **R** | fail / **pass** | 10 → 5 | 0 / 0 |
| *anchor E1-P4* | 6 | | fail | 14 | 0 |
| *anchor E3-M3* | 1 | | fail | 0 | 0 |

**Steps reconstructed,** out of 8:

| Step | C1 | C2 | C3 | C4 | C5 | C6 | C7 | C8 | C9 |
|---|---|---|---|---|---|---|---|---|---|
| V (J8) | 3 | 8 | 8 | 5 | 7 | 7 | 7 | 8 | 8 |
| W (J8) | 5 | 8 | 8 | **8** | 8 | 8 | 7 | 8 | 8 |

**The criteria** (`calibration/plans/E7.md`):

| Criterion | Required | Observed | Met |
|---|---|---|---|
| 1. C4 | W ≥ 5 of 8, discordant pairs favour W by ≥ 3 | 8 of 8; discordant 3 to 0 | yes |
| 2. The gate | W ≥ 4 of 8 | 8 of 8 | yes |
| 3. Depth kept | median central W − V ≥ 0 | +0.5 | yes |
| 4. Errors | ≤ 2 of 8 W plans with a major error, W ≤ V + 1 | 0, against 1 | yes |

**Calibres** (`calibration/registry/calibre-pass-rates.txt`, exact 95% intervals):

| Arm | Overview | Technical | Research |
|---|---|---|---|
| V, in J8 | 7/8 (0.47–1.00) | 7/8 (0.47–1.00) | 5/8 (0.24–0.91) |
| W, in J8 | **8/8** (0.63–1.00) | **8/8** (0.63–1.00) | **8/8** (0.63–1.00) |

**Resources,** W's argument subagent, median (range):
- **Cache read:** 2.79 M (2.26–4.45);
- **Output:** 72 k (47–92);
- **Seconds:** 645 (438–812).

That is close to V's in E6 (2.64 M, 67 k, 619 s).

## Judge reliability: the same plans judged twice

E6's eight V plans were judged blind in J7 and again in J8, by different judges with the same instructions.

| Plan | Central steps agreeing | Central count | C4 | Gate |
|---|---|---|---|---|
| F1V | 6 of 9 | 6 → 9 | m → R | fail → pass |
| F2V | 8 | 7 → 8 | m → R | fail → pass |
| F3V | 9 | 9 → 9 | R | pass |
| F4V | 7 | 8 → 6 | m | fail |
| F5V | 8 | 7 → 8 | R | fail → pass |
| F6V | 9 | 5 → 5 | m | fail |
| F7V | 8 | 8 → 9 | R | pass |
| F8V | 9 | 7 → 7 | m | fail |

**Agreement:**
- **Steps:** 64 of 72 (89%).
- **The gate:** agreed in 5 of 8; every change was fail → pass.
- **Core derived:** within 2 in every plan.
- **The anchors:** E1-P4 and E3-M3 scored 6 and 1 in both batches.

**What this means for the programme:**
- **The gate:** a single judge's verdict is unreliable. Four steps must all be "reconstructed", so an 89% per-step agreement compounds.
- **Earlier gate rates are noisy measurements.** This applies to E3's 0 of 5, E4's 3 of 5, E6's 2 of 8 and this 8 of 8. Within-batch comparisons are fairer, since the judges are drawn alike, but differences of one to three plans are within this noise.
- **The central count and the content counts are steadier** and remain the primary measures.

**The second judgements in J8:**
- **E6F3V:** 9 of 9 steps agree, 9 against 9.
- **E7F1W:** 9 against 8, gate passed both times.
- **E6F8V and E7F6W:** `judge/results.txt` (appended when complete).

## Next (proposed)

1. **For the author: the argument task on `master`.** Put the key-step paragraph and this sentence on `master`.
   - **The change:** two general instructions in `ARGUMENT_INSTRUCTIONS`, 7 lines, on branch `141026-exp-key-step-identity` (91c4fed, based on `131026-exp-key-step`).
   - **The evidence:** across 21 paired comparisons in three experiments, the change made plans deeper and introduced no more errors than the baseline.
   - **The limits:** it is untested on any other document. The cost is about 1.6 times the argument task's output and time.
2. **The protocol: majority-of-three judges for the central gate.** In every future batch, each plan gets three independent first judgements, and the gate counts only by majority.
   - **Cost:** about three times the judging.
   - **Where it goes:** `calibration/PROTOCOL.md`, as a change to the judging procedure, versioned.
3. **The second document.** The instruction claims generality, and only a second document class can test that. It needs:
   - a reference set and a central rubric for, say, an expository paper or a blog post;
   - the protocol's calibres for that class.

## Current State
- With one further sentence asking the key step for the identity behind each counted, bounded or equated sub-step and its exceptions, Sonnet-high plans reconstructed the counting core's step C4 in 8 of 8 against 5 of 8 for the key-step instruction alone, and passed the central gate in 8 of 8 against 5 of 8, on the same eight extractions in batch J8; the preregistered rule is met
- That configuration reaches the Research calibre in 8 of 8 plans (exact 95% interval 0.63–1.00), the first to do so in every run
- Eight plans judged blind twice agree on 89% of central steps but on the gate in only 5 of 8, so single-judge gate verdicts are noisy; every change was from fail to pass
- The instruction and sentence are on exegenius branch `141026-exp-key-step-identity` (91c4fed), not on `master`; E7's runs and transcripts are on wiki branch `141026-exp-e7-key-step-identity` (c89a3fd)

## Missing
- The author's decision on the argument instruction
- No majority-of-three judging, so gate rates carry single-judge noise
- No second document
- No full wiki run with the instructed planning, so no measure of whether the plans' depth reaches the pages

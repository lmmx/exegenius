# Capability Map

**What it answers:** which configurations meet which calibre, for which document class and task, at what observed resource cost, and how sure we are.

**Where its figures come from:** every entry comes from records in `registry/experiments/` (pass rates computed by `tools/calibre.py`, in `registry/calibre-pass-rates.txt`).

**Scope:** each entry holds only for the versions it names. It is flagged for recalibration when a model, effort semantics, definition, task prompt, reference, rubric or judge changes (`PROTOCOL.md` §8).

**Status (2026-10-17): every pipeline stage has been calibrated on one paper; the map is still preliminary in scope.**
- **Stages:** one document, with every pipeline stage tested on its own. "All pipeline stages on this paper" below summarises them; `landscape.md` sets them out as one table (#16).
- **Not shown:**
  - whether the stages work together (no complete wiki run with the final configuration, #18);
  - whether any result transfers to another document (#19).
- **The planning section below** is the detailed calibre record for one stage. Its reference set and calibre thresholds are unvalidated drafts, and the thresholds were written after E1's results had been seen.

## Planning on a frontier mathematics paper

- **Document:** "Integer multiplication below n log n" (`paper.tex`, 5,361 lines).
- **Task:** planning (extract, background, argument).
- **Calibres:** those of `PROTOCOL.md` §4. Overview: ≥ 17 of 19 core results stated, no major error and nothing wrong. Technical: also ≥ 10 core results derived. Research: also the central-derivation gate.
- **Judges:** blind Opus judges, batches J1–J6. J3 onwards also apply the central-derivation rubric.

| Configuration (agent / model / effort / binary) | Runs | Overview | Technical | Research | Argument task, median: cache read · output (est.) · seconds | Evidence |
|---|---|---|---|---|---|---|
| `exegenius-planner` / `claude-sonnet-5-5` / medium / 1d90c28 | 11 | 1/11 (0.00–0.41) | 0/11 (0.00–0.28) | not measured | ~0.08–0.42 M · ~7–17 k · 73–163 s (by arm) | X1, X2 |
| `exegenius-planner` / `claude-sonnet-5-5` / medium / f70a919 | 10 | 0/10 (0.00–0.31) | 0/10 (0.00–0.31) | not measured | 0.26–0.47 M · ~16 k · 145–150 s | X3 |
| `exegenius-planner` / `claude-sonnet-5-5` / medium / **cc70355** (batch J3) | 5 | 0/5 (0.00–0.52) | 0/5 (0.00–0.52) | 0/5 (0.00–0.52) | 0.29 M · ~12 k · 108 s | E3 |
| `exegenius-planner` / `claude-sonnet-5-5` / **high** / cc70355 (J2; J3 re-judged) | 5 | **5/5** (0.48–1.00) | 2/5 (0.05–0.85) | 0/5 (0.00–0.52), in J3 | 1.50 M · ~56 k · 480 s | E1, E3 |
| `general-purpose` / `claude-sonnet-5-5` / **high** / cc70355 | 5 | **5/5** (0.48–1.00) | 3/5 (0.15–0.95) | not measured | 2.39 M · ~58 k · 502 s | E1 |
| `exegenius-planner` / `claude-opus-5-5` / medium / cc70355 (J4) | 2 | 2/2 (0.16–1.00) | 1/2 (0.01–0.99) | 1/2 (0.01–0.99) | 1.04 M · ~25 k · 281 s | S1 (screen) |
| `exegenius-planner` / `claude-opus-5-5` / high / cc70355 (J4) | 2 | 2/2 (0.16–1.00) | 1/2 (0.01–0.99) | 1/2 (0.01–0.99) | 1.44 M · ~35 k · 395 s | S1 (screen) |
| `general-purpose` / `claude-opus-5-5` / medium / af36d6d (J6) | 5 | 5/5 (0.48–1.00) | 0/5 (0.00–0.52) | 0/5 (0.00–0.52) | 1.98 M · ~21 k · 239 s | E5 |
| `exegenius-planner` / `claude-haiku-5-5` / high / cc70355 (J4) | 2 | 0/2 (0.00–0.84) | 0/2 (0.00–0.84) | 0/2 (0.00–0.84) | 2.41 M · ~111 k · 621 s | S1 (screen) |
| `general-purpose` / `claude-sonnet-5-5` / high / af36d6d, argument only on E1's extractions (J5) | 5 | 5/5 (0.48–1.00) | 2/5 (0.05–0.85) | 0/5 (0.00–0.52) | 3.06 M · ~40 k · 383 s | E4 baseline |
| `general-purpose` / `claude-sonnet-5-5` / high / **key-step instruction** (0c29556), argument only on E1's extractions (J5) | 5 | 4/5 (0.28–0.99) | 4/5 (0.28–0.99) | **3/5** (0.15–0.95) | 2.22 M · ~66 k · 607 s | E4 variant |
| `general-purpose` / `claude-sonnet-5-5` / high / af36d6d, argument on 8 fresh extractions (J7) | 8 | 7/8 (0.47–1.00) | 2/8 (0.03–0.65) | 0/8 (0.00–0.37) | 2.12 M · ~43 k · 398 s | E6 baseline |
| `general-purpose` / `claude-sonnet-5-5` / high / **key-step instruction** (0c29556), argument on 8 fresh extractions (J7) | 8 | **8/8** (0.63–1.00) | **7/8** (0.47–1.00) | 2/8 (0.03–0.65) | 2.64 M · ~67 k · 619 s | E6 variant |
| `general-purpose` / `claude-sonnet-5-5` / high / key-step instruction (0c29556), re-judged (J8) | 8 | 7/8 (0.47–1.00) | 7/8 (0.47–1.00) | 5/8 (0.24–0.91) | as E6 | E6 variant, in E7's batch |
| `general-purpose` / `claude-sonnet-5-5` / high / **key step with identities and exceptions** (91c4fed), on E6's extractions (J8) | 8 | **8/8** (0.63–1.00) | **8/8** (0.63–1.00) | **8/8** (0.63–1.00) | 2.79 M · ~72 k · 645 s | E7 |
| `general-purpose` / `claude-sonnet-5-5` / high / 91c4fed, re-judged on F1 only (J9, majority of 3) | 1 | 1/1 (0.02–1.00) | 1/1 (0.02–1.00) | 1/1 (0.02–1.00); central 8, faithful 15 | 3.06 M · 25 k · 693 s | S2 reference (E7F1W) |
| `general-purpose` / `claude-sonnet-5-5` / **xhigh** / 91c4fed, on F1 (J9, majority of 3) | 1 | 1/1 (0.02–1.00) | 1/1 (0.02–1.00) | 1/1 (0.02–1.00); central 8, faithful 13 | 5.45 M · 27 k · 1,897 s | S2 (screen) |
| `general-purpose` / `claude-opus-5-5` / **xhigh** / 91c4fed, on F1 (J9, majority of 3) | 1 | 1/1 (0.02–1.00) | 1/1 (0.02–1.00) | 1/1 (0.02–1.00); central **9**, faithful **18** | 6.93 M · 29 k · 1,409 s | S2 (screen) |
| `general-purpose` / `claude-sonnet-5-5` / **max** / 91c4fed, on F1 (J9, majority of 3) | 1 | 1/1 (0.02–1.00) | 1/1 (0.02–1.00) | 1/1 (0.02–1.00); central **9**, faithful **18** | 24.83 M · 61 k · 2,785 s | S2 (screen) |
| `general-purpose` / `claude-opus-5-5` / **max** / 91c4fed, on F1 (J9, majority of 3) | 1 | 1/1 (0.02–1.00) | 1/1 (0.02–1.00) | 1/1 (0.02–1.00); central **9**, faithful **19** (ceiling) | 15.93 M · 48 k · 2,021 s | S2 (screen) |
| `general-purpose` / `claude-opus-5-5` / **high** / 91c4fed, on F4 and F8 (J10, majority of 3) | 2 | 2/2 (0.16–1.00) | 2/2 (0.16–1.00) | 2/2 (0.16–1.00); faithful 14 and 9 | 1.49–2.16 M · 16–20 k · 517–696 s | S3 (screen) |
| `general-purpose` / `claude-opus-5-5` / **xhigh** / 91c4fed, on F4 and F8 (J10, majority of 3) | 2 | 2/2 (0.16–1.00) | 2/2 (0.16–1.00) | 2/2 (0.16–1.00); central 9 and 9, faithful **13** and **16** | 2.91–4.02 M · 23–24 k · 962–1,034 s | S3 (screen) |
| `general-purpose` / `claude-sonnet-5-5` / high / 91c4fed, F4 and F8 re-judged (J10, majority of 3) | 2 | 2/2 (0.16–1.00) | 2/2 (0.16–1.00) | 2/2 (0.16–1.00); faithful 7 and 7 | as E7 | S3 comparison (E7F4W, E7F8W) |
| `general-purpose` / `claude-sonnet-5-5` / high / 930d40c | 2 | 2/2 (0.16–1.00) | 2/2 (0.16–1.00) | not measured | 264 k and 271 k subagent tokens as Claude Code's task notification counts them (not comparable with the cache-read figures), 8.9 and 10.3 min | S0 |

**S2's rows** give visible output only (09:47 UTC basis). Hidden reasoning is not in transcripts. The `~k` output figures in earlier rows came from the registry before that time and include thinking signatures, so they overstate visible output; the rebuilt registry holds both. S2's single-run 95% intervals are 0.02–1.00, so S2 shows nothing about pass rates: its differences are in faithful derivations, within one batch.

Pass counts are given with exact two-sided 95% (Clopper–Pearson) intervals. These are screening-stage and provisional evidence (`PROTOCOL.md` §6), not guarantees of reliability. Medium-effort runs come from batch J1, high-effort runs from batch J2. Anchors re-judged in J2 suggest J2 is at least as strict as J1 (journal 2026-10-13-e1-config). **The anchors' central counts, first judgements:**

| Anchor | J3 | J4 | J5 | J6 | J7 | J8 | J9 |
|---|---|---|---|---|---|---|---|
| E3-M3 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| E1-P4 | 8 | 8 | 7 | 6 | 6 | 6 | 6 |

E3-M3 has held throughout.

E1-P4 has held at 6 from J6 on, but J6's second judge gave it 8. *Corrected 2026-10-14:* this note said earlier that both anchors held within one step, which E1-P4 has not done since J6. Whether batches from J6 on are stricter on a mid-depth plan, or the J3–J5 judges happened to be lenient, is not settled. Central counts from J3–J5 and from J6–J9 should not be compared directly, which the design already avoids: each comparison re-judges its arms in one batch.

### What this supports, so far

- **Overview calibre:**
  - **Sonnet at medium does not reach it:** it passed in 1 of 26 runs (1 of 21 earlier, 0 of 5 in E3).
  - **Sonnet at high reaches it** in 10 of 10 runs, with either agent definition.
- **Effort, not the definition, explains the gap.** E1 found no difference between definitions at high effort. E3 compared effort within one batch, on one binary.
- **Technical calibre:** without the key-step instruction no tested configuration reaches it reliably; at high effort it is 2 to 3 of 5 per arm.
- **Research calibre:** no configuration reaches it reliably.
  - **Sonnet:** 0 of 5 plans at high effort, and 0 of 5 at medium, pass the central-derivation gate (E3, batch J3). At high effort the gap is the counting core (labels, rank as dimension change, the budget, $L/N<1/2$), reconstructed in only 1–2 plans of 5.
  - **Opus:** passes it in 1 of 2 plans at medium and 1 of 2 at high effort (S1, a screen), but in 0 of 5 fresh general-purpose plans at medium (E5), which reconstruct rank as dimension change (C4) in none. Against Sonnet at high in the same batch: central count median 6 against 5 (p = 0.33), core derived 6 against 11. Opus at medium is not preferred for planning on this paper.
  - **Sonnet at high with the key-step instruction:** passes it in 3 of 5 plans, against 0 of 5 without, on the same extractions (E4). One of the five made two major errors in the derivation, so the preregistered criterion was not met. On eight fresh extractions (E6), it passes in 2 of 8 against 0 of 8, with no major errors; the central count rose in 8 of 8 pairs (p = 0.004). The gate's remaining blocker is C4, stated by halves. The instruction is not on `master`.
- **Research calibre with the key step asking for identities and exceptions (E7):** 8 of 8 plans pass the central gate, against 5 of 8 for the key-step instruction alone in the same batch; C4 in 8 of 8 against 5 of 8. This is the first configuration to reach the Research calibre in every run.
- **Judge reliability for the gate:** the same eight plans judged in two batches (J7, J8) agree on 89% of central steps but on the gate in 5 of 8 (all changes fail → pass). Single-judge gate rates, here and above, carry that noise; a majority of three judges is proposed.
- **Technical calibre with the key-step instruction:** 7 of 8 against 2 of 8 (E6), and 4 of 5 against 2 of 5 (E4). This is the first configuration to reach the Technical calibre in most runs.
- **E3, a one-batch comparison on one binary,** confirms the effort finding at stage 2. Medians: stated 7 → 19, derived 1 → 7, faithful 0 → 3; exact one-sided p = 0.004 and 0.024 on the primary measures. This is at about 5× the tokens read from cache and 3–4× the seconds.
- **The agent definition** has shown no measured advantage at high effort: E1, 5 against 5. A general-purpose agent given the task prompt is an equal candidate.

### Open, in order of what would change a recommendation

1. **E3: Sonnet planner at medium against high,** one batch, with the central-derivation rubric. *Done: high preferred (R1), and neither reaches Research (R2).*
2. **Whether a different model reaches the same calibre at lower cost or effort.** *Screened in S1: Opus at medium and high advance, Haiku at high passes nothing. E5: Opus at medium is not better than Sonnet at high on the central derivation and derives less; it is cheaper in output and time.* The screen proposed by screening at 2 runs per arm (`PROTOCOL.md` §6). The arms would be:
   - Opus at medium and at low;
   - Haiku at high and at xhigh;
   - Sonnet at xhigh, which may matter for the Research calibre.
3. **Whether any configuration reaches the Research calibre** (the central derivation). *Partly answered: the gap moves with what the task asks for (E4), not with the model (E5).* Open: a confirmation of the key-step instruction on fresh runs, with the rate of major errors in the derivation as a criterion, and the instruction on Opus.
4. **Model × effort with the E7 instructions:**
   - **Opus at `high` with the E7 instructions:** run in S3 on F4 and F8, one run each. It was uneven: 14 faithful on F4, 9 on F8.
   - **Screened in S2** (one run each, on E6's extraction F1, majority of three judges):
     - every configuration passes the gate, so the gate is saturated there;
     - Opus `xhigh`, Sonnet `max` and Opus `max` gain C1 and 3–4 faithful derivations over Sonnet `high`;
     - Sonnet `xhigh` loses 2;
     - Opus `xhigh` is the cheapest that improved.
   - **S3** (Opus `high` and `xhigh` on F4 and F8, where Sonnet `high` is weakest; J10, majority of three):
     - Opus `xhigh` lifts both (faithful 7 → 13 on F4, 7 → 16 on F8), with C1 gained and no major errors;
     - Opus `high` does not suffice.
   - **Across F1, F4 and F8,** Opus `xhigh` beats Sonnet `high` beyond judge noise every time (+3, +6, +9 faithful). This is one run per extraction, on one paper.
   - **S4 and S5 (F8):**
     - Opus `max` is +1 faithful and +1 checklist sub-step over Opus `xhigh`, at 5× the cache read.
     - That gain is within Opus `xhigh`'s own run-to-run difference (2 faithful).
   - **E8: the instruction outweighs the model.**
     - With one more sentence (every variant, and the chain behind each bound), Sonnet `high` reaches faithful 18, 18 and 19 on F1, F4 and F8 (J16), against 14, 3 and 4 without it.
     - It is not below Opus `xhigh` on the checklist anywhere, and at the ceiling on F8.
     - **E8b (J17 screen, J18 primary):** on the primary measure, X is above W on F2, F3 and F7 and ties on F5 and F6. That is 3 of 5, so not supported by its table. X is never below W; the sign test over eight extractions gives p ≈ 0.016.
     - **The ties trace to `label-residual-budget`,** which nearly every X and W plan misses.
   - **E9: a second sentence** (each fixed parameter's value, and every case of a case-by-case step) closes C1. On F2, F4 and F8, every judgement finds 9/9 central steps and 19/19 core results faithful, with plans no longer (`docs/journal/2026-10-14-e8b-e9.md`).
   - **The candidate is now the combined instruction.** E10 tests it against E7 where E8 alone tied.
   - **Fable at `high` (FB1 pilot, F8, E7 instructions):** below Opus `max` on the checklist (17, 16 against 20, 21). No further Fable spend on this task.
5. **Writers, auditors and support checks:** *answered since,* in "All pipeline stages on this paper" below. On planning itself:
   - the combined instruction (E9, E10) brings Sonnet `high` to 19/19 faithful and 9/9 central;
   - Opus `xhigh` is not better above that ceiling (E11);
   - Sonnet `medium` and Haiku `high` fall below it (S6).

## All pipeline stages on this paper (2026-10-17)

These are per-stage results on `paper.tex`. Each stage was tested on its own, against its own standard; no result here comes from a complete wiki run (#18). The model × effort detail behind each row is in `landscape.md`, and the decisions in `policy.md`.

| Stage | Configuration in production | Meets its standard? | Evidence | Strength |
|---|---|---|---|---|
| Extraction | Sonnet `high` | **yes, indirectly:** every extraction tested fed plans judged 19/19 faithful. Opus `high` is as deep at higher cost; Sonnet `medium`, Opus `medium` and Haiku `high` are shallower | EX1 (34 extractions, judge-free); E9–E11 | moderate |
| Background | Sonnet `high` | **for planning only, indirectly;** its effect on writing is unmeasured | the 2026-10-15 background audit (8 runs) | weak to moderate |
| Argument planning | Sonnet `high`, combined instruction | **yes:** 19/19 faithful and 9/9 central on every extraction tested | E9, E10, E11, S6 | strong for this paper |
| Writing | Sonnet `high`; Opus `xhigh` on pages 0015 and 0016 of this wiki | **no, on key-step pages:** Sonnet meets the page standard on 4 of 12 key-step page-runs, Opus on 4 of 6. The failures are underived steps, not wrong mathematics (1 major error in 24 page-runs). Opus is better on 2 of 6 new key-step pages | W2, W2b, W3 (24 page-runs, 72 judgements) | moderate |
| Fidelity audit | Sonnet `high`, source-aware text (#14) | **yes, at the bound:** 13/13 source-only and 25/28 page errors, 0/18 shams, 0.50 false alarms per clean page (independent review). The page-only audit catches no source-only error | A1, A2, A3 (planted errors on six pages, one run per arm) | moderate; fragile on false alarms |
| Support check | Sonnet `high` | **no:** recall 0.60 against 0.70 for every model tested. The source-aware audit catches the generalisation errors it misses (4/4) | A1, A2, A3 | moderate |
| Simulated readers | Haiku `high` readers, Sonnet `high` grading; on by default (the author's decision) | **unvalidated:** coverage drop 0.147 against a 0.15 bar when key paragraphs were deleted (inconclusive); the baseline is an instructed refusal; stuck points are not shown to locate missing content | SR1 (one bounded experiment) | weak |
| Judging (the instrument) | Opus, three judges; pairwise above the planning ceiling | **validated as an instrument** (K3); Sonnet agrees with Opus on the checklist (K2, 0.979) | K2, K3 | strong for its purpose |

## Other document classes

None tested. The protocol is meant to be rerun on a different class, such as an expository blog post or a survey article. That needs:
- that class's calibres;
- a reference set;
- a central-argument rubric, if the class has one.

No result above should be assumed to transfer.

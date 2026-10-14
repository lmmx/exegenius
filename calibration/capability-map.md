# Capability Map

**What it answers:** which configurations meet which calibre, for which document class and task, at what observed resource cost, and how sure we are.

**Where its figures come from:** every entry comes from records in `registry/experiments/` (pass rates computed by `tools/calibre.py`, in `registry/calibre-pass-rates.txt`).

**Scope:** each entry holds only for the versions it names. It is flagged for recalibration when a model, effort semantics, definition, task prompt, reference, rubric or judge changes (`PROTOCOL.md` §8).

**Status: preliminary.**
- one document;
- one task stage;
- one model family;
- the reference set and calibre thresholds are unvalidated drafts;
- the thresholds were written after E1's results had been seen.

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
| `general-purpose` / `claude-sonnet-5-5` / high / 930d40c | 2 | 2/2 (0.16–1.00) | 2/2 (0.16–1.00) | not measured | 264 k and 271 k subagent tokens as Claude Code's task notification counts them (not comparable with the cache-read figures), 8.9 and 10.3 min | S0 |

Pass counts are given with exact two-sided 95% (Clopper–Pearson) intervals. These are screening-stage and provisional evidence (`PROTOCOL.md` §6), not guarantees of reliability. Medium-effort runs come from batch J1, high-effort runs from batch J2. Anchors re-judged in J2 suggest J2 is at least as strict as J1 (journal 2026-10-13-e1-config). From J3 on, the central counts of the anchors E1-P4 and E3-M3 have held within one step across batches (8, 8, 7 and 1, 1, 1).

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
4. **Writers, auditors and support checks:** no evidence yet. They dominate a full run's cost.

## Other document classes

None tested. The protocol is meant to be rerun on a different class, such as an expository blog post or a survey article. That needs:
- that class's calibres;
- a reference set;
- a central-argument rubric, if the class has one.

No result above should be assumed to transfer.

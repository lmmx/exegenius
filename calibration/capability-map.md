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
- **Judges:** blind Opus judges, batches J1 and J2.

| Configuration (agent / model / effort / binary) | Runs | Overview | Technical | Research | Argument task, median: cache read · output (est.) · seconds | Evidence |
|---|---|---|---|---|---|---|
| `exegenius-planner` / `claude-sonnet-5-5` / medium / 1d90c28 | 11 | 1/11 (0.00–0.41) | 0/11 (0.00–0.28) | not measured | ~0.08–0.42 M · ~7–17 k · 73–163 s (by arm) | X1, X2 |
| `exegenius-planner` / `claude-sonnet-5-5` / medium / f70a919 | 10 | 0/10 (0.00–0.31) | 0/10 (0.00–0.31) | not measured | 0.26–0.47 M · ~16 k · 145–150 s | X3 |
| `exegenius-planner` / `claude-sonnet-5-5` / medium / **cc70355** (batch J3) | 5 | 0/5 (0.00–0.52) | 0/5 (0.00–0.52) | 0/5 (0.00–0.52) | 0.29 M · ~12 k · 108 s | E3 |
| `exegenius-planner` / `claude-sonnet-5-5` / **high** / cc70355 (J2; J3 re-judged) | 5 | **5/5** (0.48–1.00) | 2/5 (0.05–0.85) | 0/5 (0.00–0.52), in J3 | 1.50 M · ~56 k · 480 s | E1, E3 |
| `general-purpose` / `claude-sonnet-5-5` / **high** / cc70355 | 5 | **5/5** (0.48–1.00) | 3/5 (0.15–0.95) | not measured | 2.39 M · ~58 k · 502 s | E1 |
| `general-purpose` / `claude-sonnet-5-5` / high / 930d40c | 2 | 2/2 (0.16–1.00) | 2/2 (0.16–1.00) | not measured | 264 k and 271 k subagent tokens as Claude Code's task notification counts them (not comparable with the cache-read figures), 8.9 and 10.3 min | S0 |

Pass counts are given with exact two-sided 95% (Clopper–Pearson) intervals. These are screening-stage and provisional evidence (`PROTOCOL.md` §6), not guarantees of reliability. Medium-effort runs come from batch J1, high-effort runs from batch J2. Anchors re-judged in J2 suggest J2 is at least as strict as J1 (journal 2026-10-13-e1-config).

### What this supports, so far

- **Overview calibre:**
  - **Sonnet at medium does not reach it:** it passed in 1 of 26 runs (1 of 21 earlier, 0 of 5 in E3).
  - **Sonnet at high reaches it** in 10 of 10 runs, with either agent definition.
- **Effort, not the definition, explains the gap.** E1 found no difference between definitions at high effort. E3 compared effort within one batch, on one binary.
- **Technical calibre:** no tested configuration reaches it reliably. At high effort it is 2 to 3 of 5 per arm.
- **Research calibre:** no configuration reaches it. 0 of 5 plans at high effort, and 0 of 5 at medium, pass the central-derivation gate (E3, batch J3). At high effort the gap is the counting core (labels, rank as dimension change, the budget, $L/N<1/2$), reconstructed in only 1–2 plans of 5.
- **E3, a one-batch comparison on one binary,** confirms the effort finding at stage 2. Medians: stated 7 → 19, derived 1 → 7, faithful 0 → 3; exact one-sided p = 0.004 and 0.024 on the primary measures. This is at about 5× the tokens read from cache and 3–4× the seconds.
- **The agent definition** has shown no measured advantage at high effort: E1, 5 against 5. A general-purpose agent given the task prompt is an equal candidate.

### Open, in order of what would change a recommendation

1. **E3: Sonnet planner at medium against high,** one batch, with the central-derivation rubric. *Done: high preferred (R1), and neither reaches Research (R2).*
2. **Whether a different model reaches the same calibre at lower cost or effort,** by screening at 2 runs per arm (`PROTOCOL.md` §6). The arms would be:
   - Opus at medium and at low;
   - Haiku at high and at xhigh;
   - Sonnet at xhigh, which may matter for the Research calibre.
3. **Whether any configuration reaches the Research calibre** (the central derivation). If none does, the gap is in what the task asks for, not in how much compute it gets.
4. **Writers, auditors and support checks:** no evidence yet. They dominate a full run's cost.

## Other document classes

None tested. The protocol is meant to be rerun on a different class, such as an expository blog post or a survey article. That needs:
- that class's calibres;
- a reference set;
- a central-argument rubric, if the class has one.

No result above should be assumed to transfer.

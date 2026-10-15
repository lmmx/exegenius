# A2 (draft, a design; not preregistered, not launched): does a source-aware, structured audit catch the errors the current tasks cannot?

**Status:**
- Drafted on 2026-10-15 at 11:07 UTC, at the author's request (about 11:00): develop the source-aware auditor experiment, without launching it until the design is ready.
- No model has run for it. It changes nothing on `master`.

**It is kept apart from two other decisions:**
- **The general classifier rule** (`A1b-draft.md`): how a note is scored. A2 makes its own measurement choice, below, and that does not settle the general rule.
- **The author's design decision on source access:** A2 tests whether source access helps; adopting it remains the author's call.

## Why

- **The current tasks never compare a page with the paper.** `audit` sees the page only. `support` sees one claim and its quotation. Their blind spot is structural (`plans/A1.md`).
- **A1 stage 1 shows the limit:**
  - support recall is 0.60 for every model, with the same misses (both generalisation errors);
  - audit recall rests mostly on errors visible on the page;
  - A1's seeded set has only one error detectable against the source alone, so it cannot measure that blind spot.
- **Source access alone may not be enough** (the author's adviser, about 11:00). An auditor also needs a structured way to check each claim against the relevant definitions, hypotheses, proof steps and conclusions. A2 separates the two effects.

## Design

### Error set: A1's, plus new source-only errors

- **A1's 24 variants** on its six pages are kept unchanged: 6 clean and 18 seeded, with 36 errors and 18 shams.
- **New: 12 source-only errors, 2 per page,** in 6 new seeded variants. Each one is consistent with everything else on the page and with its own quotation, and wrong only against the paper.
  - **Types,** at most 3 of each:
    - a dropped hypothesis;
    - a widened scope or quantifier;
    - a constant, exponent or threshold changed consistently throughout the page;
    - a conclusion stated more strongly than the source proves;
    - a result attributed to the wrong lemma or source.
  - **Seeding:** with `plant.py`'s seeding format, in a new key (`a2/key.json`). A1's key stays untouched.
  - **Checks:** each seed is checked by hand against `paper.tex` and the page before any run, with no model.
- **Total:** 30 variants and 48 planted errors (28 page-, 7 quote- and 13 source-detectable) per run.

### Arms: one model, Sonnet `high`, which the audit task runs on in production

| Arm | Audit task | What it isolates |
|---|---|---|
| **B** (baseline) | the current audit: page only, current instruction | A1 stage 1's AS answers serve for A1's 24 variants (same binary and configuration); B runs only the 6 new variants |
| **I** (instruction) | the page only, with a structured fidelity check: for each claim stating a definition, hypothesis, proof step or conclusion, list what it assumes and concludes, and name any claim that is wrong, unsupported, or stronger or wider than its own justification | the effect of structure without the source |
| **SI** (source and instruction) | as I, plus the source passages the page's plan steps cite, located in `paper.tex` with their enclosing LaTeX section. The check is done against them: hypotheses all present, scope and quantifiers equal, constants, signs, exponents and inequality directions equal, conclusion no stronger, attribution right. Each mismatch is named with the source line | the effect of the source, given structure |

**Arms not run:** a source-only arm without the structured instruction. The adviser's hypothesis is that source access alone is not enough. If SI wins, an S arm can be added to attribute the gain.

### Implementation (model-free, before preregistration)

1. **Build the audit variants** on an experiment branch, with an isolated binary (its own `CARGO_TARGET_DIR`, installed under its own name). The shared `target/debug/exegenius` is never rebuilt while A1 or W3 copies may use it. Each copy records the binary's path and sha256.
2. **Source passages:** from each page's plan steps (`argument.json`, the `quote` evidence), located in `paper.tex` and widened to the enclosing section. They are rendered into the task file, so the auditor needs no tool access to the paper and the same text reaches every run.
3. **Copies:** made with `a1setup.py`, extended to take the binary and the key.
4. **Collection and the isolation check:** `a1collect.py` unchanged.

### Measures and scoring

- **Recall per detectability class:** page (28), quote (7) and source (13).
- **Audit false alarms:** per clean page, and on shams.
- **Localisation and gaps per page:** descriptive.
- **Whether a note asserts an error (A2's measurement choice, fixed now):** every gap is labelled blind to arm by hand under A1's labelling rule (`docs/journal/2026-10-15-a1-stage1/labels-rule.md`). The hand labels are primary. `ASSERTS` is reported beside them for comparison.
  - **Why:** A1 measured `ASSERTS` finding 45% of asserting notes.
  - **Scope:** this choice applies to A2 only. The general rule for future experiments stays a separate decision for the author.

### Reading (proposed; fixed at preregistration)

**The fidelity standard for auditing** carries over A1's thresholds:
- R (page) ≥ 0.50, with its 80% lower bound ≥ 0.35;
- F_A ≤ 0.5 per clean page;
- F_sham ≤ 0.10.

It adds, for SI only, **R (source) ≥ 0.50**.

| Outcome | Reading |
|---|---|
| SI meets the standard, including source recall, and beats B beyond noise on source plus quote recall | a source-aware structured audit closes the blind spot at this scale. Propose it as a task change, with cost; the author decides (a CLI change and a default) |
| I meets the standard and SI adds nothing beyond noise | structure, not source, is what helps. Propose the instruction alone |
| SI raises recall but fails a false-alarm bound | report the trade-off; no proposal |
| Neither meets the standard | auditing stays unresolved at Sonnet `high`. The next step is SI on Opus `xhigh`, preregistered separately |

**Beyond noise:** as in A1, 80% Wilson intervals that do not overlap.

### Sample and cost (estimates, not converted)

- **Tasks:** one run per arm: I 30 audit tasks, SI 30, B 6, so 66 tasks.
- **Cache read:**
  - I and B: about 0.15 M per task, A1's AS figure;
  - SI: larger task files, an estimated 0.3–0.6 M per task.
  - Runs: about 15–25 M. Orchestration: 10–20 M. **Total: ~25–45 M.**
- **A second run of I and SI** (about +15–25 M) is preregistered only if it could change the row.
- **Hand labelling:** about 300–500 gaps; no model.

## Open before preregistration

1. **The author's agreement to test source access in this form:** passages from the plan's quotes, not the whole paper.
2. **The 12 source-only seeds,** written and checked by hand.
3. **The experiment branch and isolated binary,** built when no runs are active.

**Relation to A1b:** A1b tests a one-sentence change to the support task. A2 tests the audit task. They share A1's harness and can run in either order.

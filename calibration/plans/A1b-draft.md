# A1b (draft, a proposal; not preregistered): can the support check be taught to see generalisation?

**Status:** drafted on 2026-10-15 at 09:30 UTC, after A1 stage 1, on the author's direction at 09:24: pursue the auditing gap as a task problem, not a model choice. No model has run for it, and it changes nothing on `master`.

## Why

A1 stage 1 (`plans/A1.md`) found:
- **Support recall** is 0.60 for Haiku `high`, Sonnet `high` and Opus `xhigh` alike, against a bar of 0.70.
- **The same misses everywhere:** all three miss both generalisation errors and 2 of the 3 constant errors.

A claim that generalises its quotation reads as supported when only the quotation is checked. Changing the model did not help, so the instruction is the candidate.

## Design (proposed)

**The change:** one sentence added to the support task's instructions (the support template in `exegenius-cli/src/main.rs`):

> Mark a claim `unsupported` when it states more than its quotation does: a wider scope, a stronger quantifier, a different constant or an unstated hypothesis dropped.

- **Binary:** built on a branch into an isolated binary, never the shared `target/debug/exegenius`. The runs' executable is verified by path and sha256 before launch (the author's rule).
- **Tasks:** A1's 20 support tasks on the same seeded pages, rendered by that binary in fresh copies (`a1setup.py`, support kind only). The audit side is unchanged.
- **Configuration:** Sonnet `high` (AS), the configuration the support task runs on in production, since it shares `exegenius-task` with writers.
- **Runs:** two, giving 40 tasks, 20 quote-detectable error instances and 28 clean marks.
- **Comparison:** AS's stage-1 support answers (6 of 10; F_S 1 of 14).
- **Scoring:** `plant.py score`, judge-free. Support labels need no classifier.

## Reading (proposed; fixed at preregistration)

| Outcome | Action |
|---|---|
| Pooled R_S ≥ 0.70, with at least one generalisation error caught, and F_S ≤ 0.20 | propose the sentence for the support task; the author decides the default |
| R_S ≥ 0.70 but F_S > 0.20 | the sentence trades false alarms for recall. Report it; no proposal |
| R_S < 0.70 | the support check cannot see these errors from the quotation alone. Whether to give it the source is the author's design decision |

## The classifier rule for future audit measures (proposed, prospective only)

- **Agreement is replaced as the test of `ASSERTS`:** it may decide whether a note names an error only if, on blind hand labels of every gap, its recall on hand-labelled positives is at least 0.80 and its precision at least 0.80. Otherwise the hand labels decide.
- **Prospective only:** this applies to experiments preregistered after it is adopted. A1 stage 1's preregistered result stays primary as recorded.

## Budget (estimate, not converted)

- 40 Sonnet `high` support tasks at the measured 0.14 M each: about 6 M cache read.
- Orchestration: 5–10 M.
- **Total: ~11–16 M.**
- Building the isolated binary costs no model quota.

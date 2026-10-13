# Content judge: inputs and verdicts

- `prepare.py` builds `pack.md` (the reference items, with the paper's text where each is stated) and `plans/Pnn.md` (each argument as a tree, blind to its run) from the stored `argument.json` of each run.
- `judge-prompt.md` is the judge's instructions, with placeholders for the four paths.
- `verdicts/Pnn.json` is one verdict per plan; `verdicts/Pnn-b.json` is a second, independent verdict for agreement.
- `key.json` (added once the judging is done) maps plans to runs.

**Reproducing the blinding.** `prepare.py` was run with seed 20261013 and these plans, in this order:
- A1, A2, B1, B2 from wiki branch `131026-exp-coverage-recovery`;
- A4, A5, A6, B3, B4, B5, B6 from `131026-exp-coverage-recovery-2` (A3 excluded);
- C1–C5 and D1–D5 from `131026-exp-depth-rejection`;
- `sonnet1`, `sonnet2` from `docs/journal/2026-10-13-model-comparison/sonnet-sample-{1,2}/argument.json`.

**Each judge** was a general-purpose subagent on Opus, told to read only the instructions, the pack, `paper.tex` and its one plan.

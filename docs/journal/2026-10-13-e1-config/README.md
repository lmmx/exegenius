# E1: judge inputs, verdicts and measures

- `judge/pack.md`: byte-identical to `../2026-10-13-content-judge/pack.md`.
- `judge/judge-prompt.md`: the judges' instructions, identical to the first batch's.
- `judge/plans/Pnn.md`: the twelve plans as judged, blind.
- `judge/verdicts/Pnn.json`: one verdict per plan; `Pnn-b.json` is an independent second verdict (P03, P09, P12).
- `judge/key.json`: which plan is which run, held outside `judge/` until every verdict was in.
- `judge/results.txt`: the output of `analyse.py judge/verdicts judge/key.json ../2026-10-13-run-log-and-coverage/reference-draft/reference.json`.
- `measures.txt`: `usage.py`, `score.py` (arguments, then extractions) and `depth.py` on the runs.
- `effort.tsv`: `../2026-10-13-effort-dimension/effort.py` on the runner's transcripts.

**Reproducing the blinding.** `../2026-10-13-content-judge/prepare.py` was run with seed 20261014 and these plans, in this order:
- P1–P5, then G1–G5, from `runs/RUN/exegenius/argument.json` on wiki branch `131026-exp-e1-config` (85611ce);
- `sonnet1`, from `../2026-10-13-model-comparison/sonnet-sample-1/argument.json`;
- `C2`, from `runs/C2/exegenius/argument.json` on wiki branch `131026-exp-depth-rejection`.

**The paper:** `template/sources/paper.tex` on that branch.

**Each judge** was a general-purpose subagent on Opus, at the development session's effort (high), told to read only its instructions and the files they name.

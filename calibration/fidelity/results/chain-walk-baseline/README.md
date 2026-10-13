# Chain walk v1: baseline on the three existing LaTeX wikis

- **Instrument:** `../../chain-walk-v1.md` (committed at d66d542, before first use).
- **Wikis:** each wiki's `docs/`, blinded as W1–W3 in an order shuffled with `random.Random(20261016)`. The key is `key.txt`, held outside the judges' reach until all four results were in.
  - W1: `full-v0.6-latex-planner-handoff` (72f1de8)
  - W2: `full-v0.6-latex-depth` (fa33c9d)
  - W3: `full-v0.6-latex` (e4aff3e)
- **Judges:** general-purpose Opus subagents at the development session's effort (high). Each read only the instructions, its wiki's pages and `paper.tex` (sha256 6d2b6d00dc3422dd…). W1 was judged twice, independently (`W1-b.json`), with W1 drawn by `random.Random(20261016).choice`.
- **Verdicts:** `verdicts/*.json`, unedited. `prompt-as-run-W1.txt` is the filled-in instructions exactly as one judge received them.

# P3: chain walk on the built wiki

- **Instrument:** `calibration/fidelity/chain-walk-v1.md`, unchanged apart from its placeholders.
- **Wiki:** the built site `docs/` of `lmmx/openai-nlogn-wiki` branch `171026-e2e-final-config` at 42727e0: 49 markdown files, copied with their paths and nothing else to a scratch directory. The wiki id is W1; there is one wiki, so there is nothing to blind.
- **Paper:** `paper.tex`, sha256 6d2b6d00dc3422dd….
- **Judges:** three independent general-purpose Opus subagents, as in the baseline (2026-10-13). Each was launched with "Read the instructions in … and follow them exactly. Read only that file and the files it names."
- **Verdicts:** `verdicts/W1.json`, `W1-b.json`, `W1-c.json`, unedited. `prompt-as-run-W1.txt` holds the filled-in instructions as one judge received them.
- **P3 passes** when C4, C5, C6 and C9 are `reconstructed` by majority and no step is `wrong` by majority.

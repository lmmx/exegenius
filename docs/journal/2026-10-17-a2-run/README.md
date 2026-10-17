# A2 runs, 2026-10-17 (07:17–07:39 UTC): source-aware structured audit

The plan, the result and the deviations are in `calibration/plans/A2.md`; the design files are in `../2026-10-17-a2-design/`. This directory holds the evidence.

**Files, in the order they were made:**

| File | What it is |
|---|---|
| `answers/` | the audit answers per arm (`A2B1`, `A2I1`, `A2SI1`). `A2B1` also holds A1 stage 1's AS answers for A1's 24 variants, as the plan specifies. `collect.json` records isolation (paper reads, contamination, stray tools) |
| `A2-score.json`, `score.txt` | `plant.py score` as preregistered, with the `ASSERTS` classifier |
| `A2-notes.tsv` (+ `.unblind.json`) | every gap from every arm, shuffled, blind to arm |
| `A2-notes-masked.tsv` | the same notes with paper line references and the words paper and source(s) masked (`tools/mask_notes.py`): what was labelled |
| `labels-rule.md`, `labels.tsv` | A1's labelling rule plus A2's clarifications, and 433 hand labels (157 ones, 96 borderline), committed before agreement or unblinding |
| `A2-score-hand.json`, `hand.txt`; `…-flip` | `tools/plant_labels.py` rescoring with the hand labels, and with borderlines flipped; agreement of `ASSERTS` with the labels |
| `adjudication-items.json` (+ `.unblind.json`) | the 15 clean-page asserting gaps outside native defects, blind to arm |
| `adjudication.json` | verdicts against the paper (genuine, borderline, reason), committed before the arm map was read |
| `A2-reading.json` | `tools/a2read.py`: per arm and classifier, recall, F_A raw / adjudicated / with borderline verdicts flipped, source + quote recall, and the outcome row |

**To reproduce the reading:**
```
python3 -I calibration/tools/a2read.py --key docs/journal/2026-10-17-a2-design/key.json \
  --answers docs/journal/2026-10-17-a2-run/answers --notes docs/journal/2026-10-17-a2-run/A2-notes.tsv \
  --labels docs/journal/2026-10-17-a2-run/labels.tsv --adjudication docs/journal/2026-10-17-a2-run/adjudication.json
```

**The result in one paragraph** (results first, then interpretation):
- **Results:** SI caught all 13 source-only errors, against none for B and 2 for I. On page errors it caught 26/28, against 14/28 for B. Every arm raised no sham alarm. On clean pages, SI raised 9 alarms outside native defects; 8 were adjudicated genuine, and 5 of those 8 verdicts are borderline. By the preregistered table this is outcome row 1.
- **Interpretation:** giving the auditor the paper and a structured fidelity check closes the source-only blind spot at this scale, at about 3.2× B's cache read per audit.
- **What is uncertain:** whether SI's clean-page alarms are acceptable. If the borderline verdicts are flipped, SI fails the false-alarm bound (row 2).
- **A finding on its own:** the six "clean" production pages carry at least 8 distinct genuine misstatements (SI found all 8, I found 3 of them) that A1's native-defect list missed.

# A1 stage 1: planted-error recall for fidelity auditing (2026-10-15)

**What ran:** the preregistered A1 screen (`calibration/plans/A1.md`), one run each of Haiku `high` (AH), Sonnet `high` (AS) and Opus `xhigh` (AO). Each run is 24 audit tasks and 20 support tasks on six pages with 36 planted errors and 18 shams. All 132 tasks were launched at 05:02–05:15 UTC and accepted by 05:17. None was contaminated.

**The result and its reading are in `calibration/plans/A1.md`, "Result, stage 1".** In brief:
- **Preregistered result:** no configuration is adequate. All three fail audit recall (R_A: AH 0.46, AS 0.29, AO 0.39, against 0.50) and support recall (R_S: 0.60 for all three, against 0.70). There are no false alarms beyond the thresholds.
- **Classifier check:** `ASSERTS` agrees with the blind hand labels on 0.885 of gaps (bar 0.80), so it stays. It finds only 45% of the notes that assert an error.
- **Sensitivity analysis, not preregistered:** under the hand labels, all three meet R_A (0.64, 0.50, 0.71). None reaches R_S under either labelling.
- **Stage 2:** triggered. It is not launched until a fresh meter reading is taken.

## Files

| File | What it is |
|---|---|
| `answers/` | each task's accepted answer, run log and (for support) task file, in `plant.py`'s layout; `collect.json` holds acceptance, isolation and other tools per task |
| `A1-score.json` | `plant.py score` output: the preregistered scoring |
| `A1-notes.tsv`, `A1-notes.tsv.unblind.json` | every audit gap, shuffled and blinded, with `ASSERTS`'s verdict, and the map from row to configuration and run |
| `A1-notes-blind.tsv` | the same rows without the `asserts` column: what was labelled |
| `labels-rule.md` | the labelling rule, fixed before any row was labelled |
| `labels.tsv` | the hand labels (row, label, borderline), committed in f9937e7 before agreement was computed or the map read |
| `A1-score-hand.json`, `A1-score-hand-flip.json`, `hand.txt`, `hand-flip.txt` | the sensitivity rescoring with the hand labels, and with the borderline labels inverted (`calibration/tools/plant_labels.py`) |

## Reproduce

```
K=docs/journal/2026-10-14-a1-audit-design/key.json; J=docs/journal/2026-10-15-a1-stage1
python3 -I calibration/tools/plant.py score --key $K --answers $J/answers --json $J/A1-score.json --notes $J/A1-notes.tsv
python3 -I calibration/tools/plant_labels.py --key $K --answers $J/answers --notes $J/A1-notes.tsv --labels $J/labels.tsv --json $J/A1-score-hand.json
```

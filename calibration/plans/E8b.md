# E8b (preregistered): confirming the variant sentence on the other five extractions, and its run variance

**Status:** written and committed before any E8b run (runbook item 4b). It follows E8's supported result. The pipeline default is unchanged.

## The decision it informs

**Whether to propose the variant sentence for the argument task.**
- E8 supported it on F1, F4 and F8 (faithful 18, 18, 19 against 14, 3, 4).
- A proposal resting on three extractions is weaker than one on eight.
- E6 and E7 used eight; this brings E8 to the same eight.

**Whether one run per extraction is a fair basis:** a repeat on F4 measures X's own run-to-run variance.

## Design

**The runs:** six fresh projects (copies of E6's Fk, background accepted), rendered by `exegenius-chains` (4f5ee26). Each task file differs from E7FkW's only in the sentence and the path (checked by diff). `general-purpose`, `model: "sonnet"`, `effort: "high"`, the prompt word for word.

| Run | Extraction |
|---|---|
| E8F2X, E8F3X, E8F5X, E8F6X, E8F7X | F2, F3, F5, F6, F7 |
| E8F4X2 | F4, a repeat |

**Stage 1, the checklist screen (J17):**
- **The plans:** the six new X plans, E8F4X (the first run), and E7F2W, E7F3W, E7F5W, E7F6W and E7F7W, re-judged. That is 12 plans.
- **The judges:** two each, the first Opus and the second Sonnet (allowed by K2): 24 judgements.

**Stage 2, primary confirmation (J18), only if stage 1 passes:**
- the five new X plans with their W plans, plus E8F4X and E8F4X2;
- three judges each, plus anchors: 38 judgements;
- J3's instructions, unchanged.

**Stage 2's timing:** it is expected to fall after the 18:30 UTC reset, under runbook §5's threshold.

## Reading (fixed now)

**Beyond judge noise:** the plans' judge ranges do not overlap.

**Stage 1 passes** if X's checklist is above W's beyond noise on at least 4 of the 5 new extractions.

**Stage 2 supports the sentence on fresh extractions** if, on at least 4 of 5:
- X's faithful range is above W's;
- the gate passes by majority;
- and X has no more major errors than W.

**Across all eight extractions (E8 and E8b), reported:** the exact one-sided sign test on faithful differences.

**Run variance:** E8F4X against E8F4X2 on both measures. The variance condition (runbook §6) is met if the difference is smaller than the smallest X-over-W gain.

| Outcome | Action |
|---|---|
| supported on ≥ 4 of 5, and the variance condition met | the proposal of the sentence to the author is complete for this paper. Its generality rests on the second document |
| not supported | E8's three-extraction result stands as a screen. The proposal is withdrawn until it is understood why it fails on the others |

## Budget

These are estimates, not converted.

| | Basis | Cache read |
|---|---|---|
| Runs | E8's runs: 16.8 M for 3, by `spend.py` | ~34 M |
| Stage 1, 24 checklist judgements (J15: 16.0 M for 18) | | ~21 M |
| Stage 2, if reached: 38 primary judgements (~3 M each) | | ~115 M, after the reset |

**Stopping:** one run per new extraction and one repeat.

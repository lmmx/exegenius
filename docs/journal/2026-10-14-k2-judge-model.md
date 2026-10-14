# 2026-10-14: K2, Sonnet judges on the mechanism checklist

**The result, by the preregistered table (`calibration/plans/K2.md`):** criterion 1 is met and criterion 2 is not flagged. On the checklist, **no same-model bias is evident**, and checklist screens may use Sonnet judges, keeping one Opus judge per compared pair.

**What was done:**
- K1's eleven blind plans were judged once each by Sonnet (batch J14), with K1's prompt and only the output path changed.
- The Sonnet verdicts were compared with the first Opus judge's (J11).
- No runs were needed.

## Results (`2026-10-14-k1-checklist/k2-results.txt`)

| Run | Producer | Opus judge | Sonnet judge | Gap | Sub-step agreement |
|---|---|---|---|---|---|
| E7F1W | Sonnet | 13 | 13 | 0 | 26/26 |
| E7F4W | Sonnet | 5 | 6 | −1 | 25/26 |
| E7F8W | Sonnet | 4 | 6 | −2 | 24/26 |
| S2F1SX | Sonnet | 15 | 15 | 0 | 26/26 |
| S2F1SM | Sonnet | 21 | 21 | 0 | 26/26 |
| S2F1OX | Opus | 19 | 18 | +1 | 25/26 |
| S2F1OM | Opus | 24 | 24 | 0 | 26/26 |
| S3F4OH | Opus | 15 | 15 | 0 | 26/26 |
| S3F4OX | Opus | 15 | 15 | 0 | 26/26 |
| S3F8OH | Opus | 6 | 8 | −2 | 24/26 |
| S3F8OX | Opus | 19 | 19 | 0 | 26/26 |

**The two criteria:**
1. **Agreement:**
   - 280 of 286 sub-steps (0.979);
   - Spearman's ρ between totals is 0.995.
   - Met (≥ 0.80 and ≥ 0.90).
2. **Same-model bias:**
   - The mean gap (Opus judge minus Sonnet judge) is −0.17 on Opus-made plans and −0.60 on Sonnet-made plans.
   - Opus judges credit Opus plans **+0.43** sub-steps relative to Sonnet plans.
   - Not flagged (the threshold was 2).

**Where they disagree:** all six disagreements are on low-scoring plans (4–8 sub-steps), where Sonnet judges were slightly more generous. That holds whoever made the plan.

**This also extends K1's reliability check to mid- and low-range plans.** K1 double-judged only the three top plans, and a cross-model judge agreeing at 0.979 across the whole range bounds the within-model noise from above.

## What it changes

- **The S2 and S3 model effects:** on the checklist, they are not an artefact of Opus judging Opus plans. The primary rubric was not cross-checked, and a cross-check there is not queued, since K2's bias is far below the threshold that would have triggered one.
- **Cheaper checklist screens:** they may use a Sonnet judge for the second judgement, keeping one Opus judge per compared plan. E8's stage-1 screen (J15) is amended accordingly before any E8 result (`plans/E8.md`).

## Limits

- **The checklist only:** whether Sonnet judges agree on the full content rubric, which needs more judgement, is untested.
- **One paper, eleven plans.**

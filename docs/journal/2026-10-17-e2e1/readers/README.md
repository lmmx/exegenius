# The simulated readers against the page judge (R1–R4)

- `simulation-round1.json`: the run's `simulation.json` after reader round 1 (every round-1 grade and attribution).
- `simulation-round2.json`: after reader round 2; holds the accepted runs of both rounds.
- `analysis.json` and `analysis-first.json`: R2 and R3 from `python3 -I calibration/tools/e2e1readers.py simulation-round2.json ../page-judge-before-r1/results.json ../page-judge-final/results.json`, on the completed and the first-written scorings.

## Result

**HELPED, on the R2 arm** (plans/E2E1.md, Results). 17 deficient page-rounds carry 7.65 stuck points each, against 2.52 for 61 sound ones (ratio 3.03). R3 is not met: 6 of the 23 rewritten pages gain faithful steps, 1 loses, none gains a major error. R4 shows no change (held out 20/24 in both rounds). R1 coverage, unvalidated, is 18/18 on the chain goals, against a baseline of 3/9.

No reader opened packed-bit-addition, the page with the major error, in either round.

# 2026-10-13: A judge of what the plans say

The coverage measures of the earlier experiments say where a plan's quotes fall, not whether its steps state the paper's mathematics. The author asked for a judge of the plans' content:
- checked against the reference and the source;
- kept apart from the coverage heuristics;
- with its raw evidence preserved and its limits documented;
- claiming no mathematical correctness on proxy metrics.

## Method

**Plans.** 23 arguments, from the stored `argument.json` of:
- the 21 clean experiment runs: A1, A2, A4–A6 and B1–B6 of the recovery experiment and its replication, and C1–C5 and D1–D5 of the depth experiment;
- the two Sonnet samples of 2026-10-13-model-comparison, made by general-purpose subagents on Sonnet from the same task files at an earlier binary (930d40c).

**Blinding.** `prepare.py` rendered each plan as an indented tree, with each quote's line in the paper, numbered P01–P23 in an order shuffled with seed 20261013. The judges saw no run names, arms or coverage figures. `key.json` was added only after every verdict was in.

**The pack** (`pack.md`) holds the 26 in-scope items of the draft reference set (19 core, 7 supporting). Each item has its draft statement and role, the paper's own text where the result is stated, with line numbers, and the line ranges where it is derived. The reference is unreviewed, so the judges were told to treat the draft statement as a guide and the paper as the authority.

**Judges.** Each was a general-purpose subagent on Opus, told to read only the instructions (`judge-prompt.md`), the pack, `paper.tex` and its one plan. For each item it gave:
- `presence`: absent, mentioned, stated, or derived;
- `statement`: correct; imprecise (right in substance, missing a quantity, condition or cost); or wrong;
- `derivation`: faithful, partial, or wrong;
- the paper lines it checked.

Each judge then listed errors (major or minor) and described the plan's chain to the main bound. Five plans, drawn at random, were judged twice by independent judges.

**Records** in `2026-10-13-content-judge/`: the inputs, the instructions, every verdict (`verdicts/Pnn.json`, and `Pnn-b.json` for the second judgements), `key.json`, `analyse.py` and its output `results.txt`.

## Results

Core items are of 19:
- **stated:** stated or derived in substance, not wrong;
- **fully:** stated and judged correct, not imprecise;
- **derived:** the plan shows how;
- **faithful:** the derivation follows the paper with no essential step missing.

| Group (runs) | Stated | Fully | Derived | Faithful | Major errors |
|---|---|---|---|---|---|
| Stopped at the introduction, not sent back (A1, A4, A5, A6) | 8, 4, 10, 4 | 0 each | 0, 1, 1, 0 | 0 | 0, 0, 1, 1 |
| Stopped there, sent back by the section rule (B1, B2, B3) | 15, 9, 12 | 2, 0, 0 | 3, 2, 1 | 0 | 1, 0, 0 |
| Read broadly unprompted (A2, B4, B5, B6) | 10, 9, 18, 8 | 0, 0, 3, 0 | 2, 3, 4, 0 | 0 | 0 |
| Depth experiment, section rule (C1–C5) | 3, 11, 12, 12, 12 | 0, 0, 2, 0, 0 | 0, 3, 3, 2, 0 | 0 | 0, 1, 0, 0, 1 |
| Depth experiment, results rule (D1–D5) | 13, 16, 9, 8, 12 | 0, 4, 0, 0, 0 | 2, 2, 0, 2, 4 | 0 | 1, 0, 0, 0, 0 |
| Sonnet, general-purpose subagent (2 samples) | 19, 19 | 14, 11 | 11, 12 | 6, 5 | 0, 0 |

**Size of the plans:** the `exegenius-planner` plans have 17–44 steps; the Sonnet samples 83 and 79.

**Statements judged wrong:** 2 of all the items judged.

**The major errors repeat across independent runs:**
- **Exactness:** the ring convolution "computed exactly" (B1, A6). The paper approximates it with an explicit error bound, and exactness comes from rounding afterwards.
- **The shear's cost:** the address shear "in linear time" or "by rank" (C2, D1). It costs exactly rank-many recursive interchanges plus $O(V)$.
- **The swap:** C5's account of the swap's three updates.
- **Cancellation:** "both permutations cancel" (A5). Only the source permutation does.

**Agreement of the paired judges,** on P02 (B1), P05 (A4), P11 (A1), P13 (C1) and P21 (A6):
- presence agreed on 22–25 of 26 items, and never differed by more than one level;
- statement verdicts agreed on 41 of 44 items;
- "stated" counts differed by at most 2.

## What this shows

**Observed:**
- **The section rule helps, modestly.** The runs sent back by it state about twice as many core results in substance as the runs that stopped early and were not (medians 12 against 6), and derive slightly more (2 against 0.5).
- **The results rule adds nothing the judge can see.** The depth experiment's arms do not differ: medians of 12 stated and 2 derived in both. Its gains on the reference scorer were where quotes fall, the circularity noted in 2026-10-13-depth-rejection.
- **No `exegenius-planner` plan states a core result fully more than 4 times, or derives one faithfully.** The judges' notes describe statements right in substance but missing the conditions, costs and exponents that make them results.
- **The two general-purpose Sonnet plans** state all 19 core results, 11–14 fully, and derive 11–12, 5–6 faithfully.

**Interpretations, not shown:**
- **The checks change what the planners read more than how they plan.** The planners summarise results rather than reconstruct their derivations, wherever they read.
- **The repeated errors** are the introduction's simplified wording carried into the plan. The introduction describes the convolution step and the shear's cost loosely.
- **The largest gap measured is between agent configurations, not models.** The general-purpose and `exegenius-planner` plans both come from Sonnet. They differ in:
  - the agent's system prompt (Claude Code's general-purpose prompt, against the planner definition's short one);
  - the orchestrator;
  - the binary (930d40c against 1d90c28 and f70a919, whose task text differs by the coverage check's rejection messages).

  With two samples, the comparison is suggestive, not controlled.

**Limits of the judge:**
- it judges against an unreviewed draft reference;
- it is one model judging another's plan;
- "imprecise" covers a range from a missing exponent to a missing mechanism;
- it judges the argument only, not the extraction;
- agreement was measured on five plans.

## Current State
- `docs/journal/2026-10-13-content-judge/` holds the blinded inputs, instructions, 28 verdicts, the key and the analysis of a judge of 23 plans' mathematical content, on branch `131026-content-judge`
- Over 21 `exegenius-planner` plans, the judge finds 3–18 of 19 core results stated in substance, 0–4 stated fully, 0–4 derived and none derived faithfully; the two general-purpose Sonnet plans state 19, 11–14 fully, and derive 11–12, 5–6 faithfully
- Planners sent back by the section rule state a median of 12 core results against 6 for those that stopped at the introduction and were not; the results rule's arm and the section rule's arm both have medians of 12 stated and 2 derived

## Missing
- No controlled comparison of agent configurations (definition, system prompt, effort) at a fixed model on the argument task
- No content judgement of extractions or pages
- No review of the reference set the judge uses

# EX1 (DRAFT, not preregistered): does the extraction configuration reach the plan under the combined instruction?

**Status: DRAFT.**
- **Written:** 2026-10-14, after E10 (supported, 23:45 UTC) and S6 stage 1 (neither cheap planner holds) were read. It is not committed as a preregistration, and no run has been made.
- **Its evidence:** `docs/journal/2026-10-14-x1-extraction-evidence/`.
- **The pipeline default is unchanged.**

**The id clashes.** The registry already has EX1: "Coverage recovery: report against reject", 2026-10-13. This plan needs a new id before preregistration, for example EEX1. Run names below use `EX1` as a placeholder.

## The decision it informs

It informs the extraction row of `policy.md`: which model and effort to use for `extract`. Today that row is "unmeasured as a stage".

**What the judge-free evidence settles:**
- **Opus `medium` is dominated.** Its extractions are shallower than Sonnet `high`'s beyond the spread within configurations: 9 against 23.5 results quoted (p = 0.003), and 8 against 15 core items reached (p = 0.002). Its model is priced higher, and it uses no fewer tokens.
- **Opus `high` cannot win on this paper.** Its extractions are as deep as Sonnet `high`'s, with more concepts. Under the combined instruction (Y), Sonnet `high` plans already score 19/19 faithful and 9/9 central on all six Sonnet `high` extractions tried, so a gain from Opus extraction could not register. RUNBOOK §3.5 rules out comparisons without headroom; this belongs to the second document.
- **No measure predicts a plan's total.** Among Sonnet `high` extractions, nothing judge-free predicts it. Under Y the planner supplies every core result the extraction misses: 34 of 34.

**What it does not settle:** whether a cheaper extraction still reaches the ceiling under Y. No plan under Y has been made on an extraction from any other configuration.

**The cheaper candidate is Sonnet `medium`:**
- **Cost:** about 0.18 M cache read and 1 minute, against about 1.2 M and 3 minutes for Sonnet `high` general-purpose.
- **Depth:** shallower. 11 results, 10 core items and 2 in derivation lines, after a coverage rejection in 4 of 5 runs.
- **Granularity:** half as many concepts (21 against about 40), so half the pages.

**Haiku `high` is not a token-cost candidate.** Its extractions used 0.47 and 2.06 M. It is used here only as a probe: S1-K1 is the shallowest accepted extraction in the set, with 3 results, 4 core items and 7 of 10 sections.

**The stakes are small in tokens.** Switching to Sonnet `medium` saves about 1 M cache read per wiki on this paper. Stage 1 below costs about as much as 23–44 wikis' saving.

The experiment's larger value is the second document. It shows whether the argument planner's compensation for weak extractions holds beyond Sonnet `high`'s gaps, before a paper where extraction may matter more.

**Option 0 (no runs, no cost):** record the judge-free result in the policy row and keep Sonnet `high`. This is the right choice if the author would not switch the extractor for about 1 M per wiki.

## Design

**The planner is held fixed** at the argument configuration E10 supports:
- `general-purpose`, `model: "sonnet"`, `effort: "high"`;
- the combined instruction, binary `/usr/local/bin/exegenius-params` (e8efaa3, branch `141026-exp-params-cases`);
- one run per extraction;
- `coverage = "reject"`, `coverage_depth = "sections"`, as in E6.

S6 showed that cheaper planners fall well below the ceiling, so the planner stays Sonnet `high`.

**The background is held fixed too** (RUNBOOK §3.3, one factor at a time). On each copy it is re-run with Sonnet `high`, general-purpose, so that only the extraction differs from the anchors.

| Run | Extraction (existing, accepted) | Configuration | Results quoted | Core items reached | Unknown concepts | Why |
|---|---|---|---|---|---|---|
| EX1M4Y | E3-M4 (5ced29e) | Sonnet `medium`, planner | 11 | 10 | 21 | the cost candidate; closest to its configuration's median (rule: core reached, then results) |
| EX1M5Y | E3-M5 (5ced29e) | Sonnet `medium`, planner | 11 | 10 | 25 | the same, a second extraction, so that a pass does not rest on one |
| EX1K1Y | S1-K1 (7335096) | Haiku `high`, planner | 3 | 4 | 36 | the floor probe: the shallowest accepted extraction, with Sonnet-like granularity |
| E9F8Y (anchor, re-judged) | E6-F8 | Sonnet `high`, general-purpose | 11 | 11 | 37 | the lowest-reach Sonnet `high` extraction under Y: 19/19, checklist 26, 25 (J20) |
| E9F4Y (anchor, re-judged) | E6-F4 | Sonnet `high`, general-purpose | 24 | 15 | 35 | a typical Sonnet `high` extraction under Y: 19/19 (twice), checklist 25, 24 |

**Why these three runs discriminate.** The plans separate two causes that the judge-free figures cannot:
- **M4 and M5** match F8's reach (10 against 11 core items, 11 results each) with about half its concepts;
- **K1** keeps F8-like granularity (36 unknown concepts) with a third of its reach.

So:
- if the M plans fall and K1's holds, the graph's granularity (pages) is the cause;
- if K1's falls and the M plans hold, reach is the cause;
- if all hold, the extraction configuration does not reach the plan under Y on this paper.

**Mechanics:**
- **Projects:** copies of `runs/M4` and `runs/M5` (5ced29e) and `runs/K1` (7335096). Each is reset to just after its extraction was accepted: the reset E4 made before the argument task, made one task earlier.
- **Contamination guard.** Before any run, these are moved out of each copy, because they embed the original configuration's plans:
  - the original background answer;
  - the argument answer, its task directory and `argument.json`;
  - every write task.
- **Task-file checks, by diff, recorded before launch:**
  - each background task file is identical to the original `0002-background.md` apart from the path, or every difference is listed;
  - each argument task file's instruction text, everything before `## Target`, is identical to E9F8Y's.
- **Prompts:** the prompt `next` printed, word for word, to a fresh subagent per task.
- **Launch:** at most three subagents at once, on a meter reading that leaves room for the whole of stage 1 (RUNBOOK §5).

## Stage 1: the checklist screen

**The batch:** the three new plans and the two anchors, rendered blind with a fresh seed.
- Two judges each, Opus then Sonnet, as in J17 and J20 (K2: Sonnet agrees with Opus on the checklist at 0.979).
- That makes 10 judgements.
- Analysis with `tools/ranges.py`.

**Recorded before judging, judge-free and free:**
- each plan's node count and `max_tokens` stops;
- `markers.py`;
- the argument's core items reached (`tools/score.py`).

**Reading, fixed now:**
- **"Below":** a plan's two totals both lie below the lower total of the lower-scoring anchor (`ranges.py` "below").
- **Noise reference:** repeats on one extraction differ by at most 2 on the checklist (E8F4X against E8F4X2: 23, 21 and 22, 21; S3F8OX against S5F8OX: 19, 19 and 19, 19).
- **Sonnet `medium` holds stage 1** if neither EX1M4Y nor EX1M5Y is below.
- **K1 holds** if EX1K1Y is not below.

## Stage 2: primary judging (only if Sonnet `medium` holds stage 1 and the author would act on it)

**The batch:**
- EX1M4Y and EX1M5Y, three Opus judges each;
- E9F8Y re-judged, three judges;
- the scale anchor E1-P4, one judge.
- That makes 10 judgements, with J3's instructions and the central rubric (`calibration/rubrics/nlogn-central-saving.md`).

**Sonnet `medium` stays at the ceiling** if, on both of its extractions:
- the gate passes by majority;
- C1 is reconstructed by majority;
- its faithful range is not below E9F8Y's;
- it has no more major errors than E9F8Y (by median).

## Decision table

| Outcome | Action |
|---|---|
| Sonnet `medium` stays at the ceiling, and K1 holds stage 1 | the extraction configuration does not reach the plan under Y on this paper. **Propose Sonnet `medium` for `extract`** (about 1 M cache read and 2 minutes saved per wiki). State that it halves the pages, which no plan judgement measures; the author decides whether that granularity is acceptable. The default changes only after the second document |
| Sonnet `medium` stays at the ceiling, and K1 is below | there is a floor, and Sonnet `medium` is above it. Propose as above, and record K1's figures as the first point below the floor (3 results, 4 core items, 7 of 10 sections) |
| Sonnet `medium` holds stage 1 but not stage 2 | Sonnet `high` stays. Report which central steps or results the M plans lose, against their extractions' reach |
| EX1M4Y or EX1M5Y is below at stage 1 | Sonnet `high` stays. The policy row records that Y does not compensate for Sonnet `medium`'s extractions. If K1 holds while the M plans fall, record granularity, not reach, as the likely cause. Nothing further unless the author asks |
| one M plan below, the other not | ambiguous. Report it, and run one fresh Sonnet `medium` extraction (about 0.2 M), with its background and argument, before any proposal |
| the author declines stage 2 | the stage 1 result is reported as a screen; no default changes |

**Reported, not criteria:**
- **Whether reach predicts the plan under Y:** Spearman's ρ between core items reached and the checklist mean, over the five plans (exact p; n = 5 limits it). The prediction from reach alone puts K1 lowest.
- **Per new plan:** the checklist sub-steps missed (and, at stage 2, the core results not faithful), each against whether the extraction reached it.
- **The background answers:** known concepts added, and whether they differ from the originals.

## Not tested, and why

- **Opus `high` extraction:** the same depth as Sonnet `high` without judging, and the planner under Y is already at 19/19 and checklist 23–26 on Sonnet `high` extractions. No headroom (RUNBOOK §3.5). It belongs to the second document, or to a measure that discriminates above the ceiling.
- **Opus `medium` extraction:** shallower than Sonnet `high` beyond the spread, at a higher price per token. It is dominated.
- **Sonnet above `high` for extraction:** there is no hypothesis about what it would improve (RUNBOOK §3.4), and the same ceiling applies.
- **Haiku `high` as a configuration:** K1 is a probe of one extraction, a screen. Haiku's extraction token use is not lower than Sonnet `high`'s.

## Budget (cache read, measured components; orchestration not included)

| | Basis | Cache read |
|---|---|---|
| 3 backgrounds (Sonnet `high`) | E1 G backgrounds, median 0.29 M; E5 N 0.38 M | ~1 M |
| 3 argument runs (Y) | 4–7 M per Sonnet `high` argument run; E10's runs were 4.5–11.0 M | ~12–21 M (up to ~33 M) |
| Stage 1 checklist, 10 judgements | ~1 M each | ~10 M |
| **Stage 1** | | **~23–32 M (up to ~44 M)** |
| Stage 2, 10 primary judgements (conditional) | ~4.8 M each (J18) | ~48 M |
| **Whole experiment, if stage 2 runs** | | **~71–80 M (up to ~92 M)** |

**A cheaper minimum, if the author wants only the floor question:** EX1M4Y and EX1K1Y with one anchor (E9F8Y), at stage 1 only. That is 2 runs and 6 checklist judgements, about 15–20 M. It gives up the second Sonnet `medium` extraction, so a pass would not support a proposal.

**Stopping:** one run per extraction. No repeats, except the one fresh Sonnet `medium` extraction named in the decision table.


**Decision (2026-10-15, 00:00 UTC):** not run. The evidence analysis answers the policy question (keep Sonnet `high`), and the experiment's cost (~71–80 M) exceeds what it could save (~1 M per wiki). Renamed from X1, which the registry already uses for coverage recovery.

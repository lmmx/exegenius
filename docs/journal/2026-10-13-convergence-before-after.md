# 2026-10-13: Do the audit rounds drive the gaps out? Before and after the fixes

The author asked whether the fixes since the divergent 0.5.0 run made the audit rounds converge: whether the gaps the audits report get driven out round by round. The fixes include the reduction pass, narrow rewrites, the depth limit, the background task, the LaTeX source, the planner and the owned-step handoff. He asked for the original measurements beside the later ones:
- **for each metric:** the baseline, the later values, the change, the sample size, and the source;
- **comparability:** comparable measurements kept apart from those taken under changed prompts, models, references or scoring;
- **missing values:** marked as unavailable, not estimated;
- **no new experiments** until this comparison is assembled.

This entry is that comparison. Every figure comes from `exegenius status` as recorded at the end of each run, or from the journal entry named, and nothing is re-run. The recorded status files are kept in `2026-10-13-convergence-before-after/status/`.

## The answer

**No. In no full run since 0.1 do the audit rounds drive the gaps out.**

**What changed:** the divergence is gone.
- In 0.5.0, the gaps grew from 293 to 836 over three rounds, and the wiki from 32 to 142 pages.
- In the four full runs since, total gaps stay roughly level from first round to last: 0.67 to 1.08 times. The wiki grows by 1 to 5 pages between rounds.

**What did not change:**
- **Step gaps renew.** On pages audited in consecutive rounds, step gaps stay level, and 90–100% of each round's step gaps are in passages not reported before.
- **Fixes hold.** A gap a rewrite was told about is almost never reported again.
- **New gaps take their place.** They arise in text the rewrites add or reword, and in unchanged text an earlier audit did not report.
- **Gaps remain at the end.** Every full run ends with 50–100 gaps on its pages, because the last round's notes are dropped.

**Convergence of the kind the 0.1 run showed** has been seen only in pilots of 5–7 pages. In 0.1, gaps per page more than halved each round, from 44 gaps to 4.

**Unmeasured: whether the planning fixes of 2026-10-13 change this.** The fixes in question are the coverage rejection, its default, and the results rule. Every run since them stopped after planning, so no audit rounds have been run with them.

## Metrics, by run

**The runs,** all the author's. From 0.5.0 on they were orchestrated on Haiku in his container, and writers and auditors inherited Haiku:

| Run | Software | Source | Rounds | Branch (lmmx/openai-nlogn-wiki) | Entry |
|---|---|---|---|---|---|
| 0.1 | 0.1.0, the Claude Code task loop, with the skill and no agent definitions | `paper.txt` | 3 | `master` (e3d0322) | 2026-10-11-divergent-run |
| 0.5.0 (baseline: the divergence) | 0.5.0 | `paper.txt` | 3, stopped at task 855 | `tmp-v0.5-build-record-stash` (fa6c804) | 2026-10-11-divergent-run |
| v0.6 | 7cc4d73 | `paper.txt` | stopped after 2 by the stop rule | `full-v0.6` (901a730) | 2026-10-12-full-run-stop |
| v0.6 LaTeX | 47f9294 | `paper.tex` | 3, `stop_early = false` | `full-v0.6-latex` (e4aff3e) | 2026-10-12-full-latex-run |
| explanation depth | 930d40c | `paper.tex` | 3, `stop_early = false` | `full-v0.6-latex-depth` (fa33c9d) | 2026-10-13-explanation-depth-run |
| planner and handoff | 0.6.0 with the owned-step handoff (ec620eb) and the planner on Sonnet | `paper.tex` | 3, `stop_early = false` | `full-v0.6-latex-planner-handoff` (72f1de8) | 2026-10-13-planner-handoff-run |

**Total gaps and gaps per audited page, by round:**

| Run | Round 1 | Round 2 | Round 3 | Last ÷ first, total | Pages audited, first → last | Pages at the end |
|---|---|---|---|---|---|---|
| 0.1 | 44 over 24 (1.8) | 25 over 36 (0.7) | 4 over 43 (0.1) | 0.09 | 24 → 43 | 45 |
| 0.5.0 | 293 over 32 (9.2) | 698 over 82 (8.5) | 836 over 119 (7.0) | 2.85 | 32 → 119 | 142 |
| v0.6 | 75 over 32 (2.3) | 50 over 34 (1.5) | not run (stop rule) | 0.67 | 32 → 34 | 34 |
| v0.6 LaTeX | 69 over 22 (3.1) | 61 over 23 (2.7) | 58 over 23 (2.5) | 0.84 | 22 → 23 | 23 |
| explanation depth | 101 over 27 (3.7) | 102 over 28 (3.6) | 89 over 32 (2.8) | 0.88 | 27 → 32 | 32 |
| planner and handoff | 93 over 24 (3.9) | 100 over 24 (4.2) | 100 over 25 (4.0) | 1.08 | 24 → 25 | 25 |

**Gaps on pages audited in consecutive rounds,** before the rewrites → after (new: the passage was not reported before):

| Run | Rounds | Term | Step | Symbol |
|---|---|---|---|---|
| 0.1 | 1 → 2 | 16 → 5 (5 new) | 20 → 2 (2 new) | 8 → 8 (8 new) |
| 0.5.0 | 1 → 2 | 91 → 46 (45 new) | 131 → 171 (164 new) | 71 → 111 (110 new) |
| 0.5.0 | 2 → 3 | unavailable | 320 → 321 (306 new) | unavailable |
| v0.6 | 1 → 2 | 26 → 12 (10 new) | 35 → 22 (22 new) | 14 → 9 (7 new) |
| v0.6 LaTeX | 1 → 2 | 12 → 7 (7 new) | 45 → 44 (44 new) | 12 → 7 (6 new) |
| v0.6 LaTeX | 2 → 3 | 9 → 11 (10 new) | 45 → 45 (45 new) | 7 → 2 (2 new) |
| explanation depth | 1 → 2 | 34 → 16 (14 new) | 56 → 61 (59 new) | 11 → 16 (14 new) |
| explanation depth | 2 → 3 | 17 → 9 (8 new) | 67 → 58 (55 new) | 18 → 10 (10 new) |
| planner and handoff | 1 → 2 | 11 → 17 (16 new) | 64 → 73 (69 new) | 18 → 10 (9 new) |
| planner and handoff | 2 → 3 | 17 → 12 (10 new) | 73 → 73 (70 new) | 10 → 11 (11 new) |

The 0.1 and 0.5.0 rows for rounds 1 → 2 are from 2026-10-11-baseline-pilot, computed from those runs' records. The 0.5.0 row for rounds 2 → 3 is from 2026-10-11-divergent-run, which gives step gaps only.

**Steps by the action the audit asks for,** in the runs that record it:

| Run | Round 1: explain, cite or cut | Round 2 | Round 3 |
|---|---|---|---|
| v0.6 LaTeX, first level against cite or cut | 45, 0 | 13, 32 | 23, 22 |
| explanation depth | 56, 0 | 49, 18 | 28, 37 |
| planner and handoff | 64, 0 | 63, 10 | 35, 40 |

**Gaps left on the pages when the rounds end:**

| Run | Left | Of which cite or cut | Citation support at the end: supported, partial, unsupported, contradicted |
|---|---|---|---|
| v0.6 | 50 | unavailable | not run |
| v0.6 LaTeX | 58 | 22 | 13, 14, 2, 1 of 30 |
| explanation depth | 89 | 37 | 16, 19, 6, 0 of 41 |
| planner and handoff | 100 | 40 | 23, 32, 10, 0 of 65 |

**Pilots,** the part of the paper one page depends on, 5–7 pages:

| Pilot | Software entry | Round 1 | Round 2 | Round 3 |
|---|---|---|---|---|
| baseline | 2026-10-11-baseline-pilot | 46 over 7 | 40 over 7 | not run |
| reduction | 2026-10-11-reduction-pilot | 27 over 5 | 47 over 11 (stop rule) | not run |
| depth | 2026-10-11-depth-pilot | 10 over 5 | 3 over 5 | not run |
| reader check | 2026-10-12-reader-check-pilot | 12 over 5 | 6 over 6; step 6 → 0 | not run |
| first level | 2026-10-12-first-level-pilot | 24 over 5 | 23 over 7 | 8 over 5 |
| LaTeX source | 2026-10-12-latex-pilot | 41 over 5 | 29 over 5 (stop rule) | not run |

## What is comparable, and what is not

**Comparable between the four v0.6 runs:**
- the audit's kinds of gap;
- the "new" rule (2026-10-11-divergent-run: same kind, and one passage equal to or containing the other, at least 20 characters);
- the reader configuration, though the background task's answer differs per run;
- the orchestrating model, Haiku.

**Not comparable, between these runs:**
- **The extraction differs in every run:** 23–34 pages from 42–67 concepts, with the target under different names. Totals are therefore not comparable; ratios within a run and gaps per page are the closer comparisons.
- **The source changed** from `paper.txt` to `paper.tex` after the v0.6 run.
- **The depth rule changed** between the v0.6 LaTeX run and the explanation-depth run, so which steps are first-level changed.
- **The planner and handoff run** planned on Sonnet and owned sub-steps. Its rewrite check, by the run's notes, rejected many requested derivations as new claims.

**Not comparable with 0.1 and 0.5.0:**
- the audits now count symbols (from 0.5) and cite-or-cut steps (from 0.6);
- relation sentences are checked;
- the per-page rates of 0.1 and 0.5.0 therefore measure a different audit;
- **the 0.1 run's model is not recorded.** It ran with the skill and no agent definitions (`.claude/` on the wiki's `master` holds only the skill), so its tasks went to general-purpose subagents, or to the orchestrator itself, on the session's model. The only run that converged therefore also differs from every later run in agent configuration, and possibly in model. That is the variable E1 isolates.

**Unavailable:**
- term and symbol gaps on 0.5.0's repeated pages for rounds 2 → 3;
- cite-or-cut counts for the v0.6 run's last round;
- any audit-round figure under the 2026-10-13 planning fixes;
- wall time and tokens per round, for every run.

## Which concerns are closed

| Concern (where first recorded) | Status | Evidence |
|---|---|---|
| Gaps growing round on round (2026-10-11-divergent-run) | **Closed** in the four v0.6 runs | last ÷ first 0.67–1.08, against 2.85 |
| The graph growing without bound; overload rounds without limit (2026-10-11-divergent-run, 2026-10-11-graph-growth) | **Closed** | 1–5 pages added between rounds, against 50 and 37; overload rounds are bounded |
| Rewrites removing the steps they are told about | **Holds** | of the steps rewrites were told about, 0 of 45 (round 1) and 0 of 45 (round 2) were reported again (2026-10-12-full-latex-run); of all step passages, 2 of 56 and 3 of 67 (2026-10-13-explanation-depth-run); in 0.5.0, 7 of 171 were seen before |
| Rewrites adding as many new step gaps as they fix (2026-10-11-divergent-run) | **Open** | step gaps on repeated pages level in every v0.6 run with three rounds; 90–100% new |
| The depth regress: steps inside added reasoning asking for more (2026-10-11-depth-limit) | **Bounded, not removed** | first-level steps fall (45 → 23, 56 → 28, 64 → 35); cite-or-cut steps rise to 22–40 and are left unfixed at the end |
| Term gaps | **Fall** in 5 of 7 consecutive-round pairs | the exceptions: v0.6 LaTeX rounds 2 → 3 (9 → 11) and planner and handoff rounds 1 → 2 (11 → 17) |
| Symbol gaps | **Fall** in 5 of 7 pairs | the exceptions: explanation depth rounds 1 → 2 (11 → 16) and planner and handoff rounds 2 → 3 (10 → 11) |
| Gaps left on the final pages | **Open** | 50–100 per run, since the last round's notes are dropped |
| Audit variance: gaps in unchanged text reported only in a later round (2026-10-12-full-latex-run) | **Open, unmeasured as a rate** | 8 of round 3's 23 first-level steps in the v0.6 LaTeX run |
| Whether the 2026-10-13 planning fixes change the rounds | **Unmeasured** | no full run since them |

## Interpretation, not shown

- **Reported gaps are not a stock.** An audit reports a sample of what a reader would stop at. A level count across rounds is therefore consistent with a falling stock of real gaps as well as a steady one. The re-report rates near zero show fixes hold. They do not show how many unreported gaps remain.
- **Renewal is about rewrites, not the plan.** In all three runs with three rounds, step gaps renew about one for one. That held whether the plan came from the introduction (v0.6 LaTeX, explanation depth) or was owned-step but shallow (planner and handoff). This points at what rewrites add, and at audit sampling, rather than at the plan's coverage. A run with the planning fixes would test whether a plan that owns the derivations reduces what rewrites have to add.

## Proposed, not run

These follow E1 in the order the author set:
1. **A full run on `master`** with the planning fixes, `rounds = 3`, `stop_early = false`, and the same reader and source as the planner and handoff run. Measures: this entry's tables, and the content judge on its plan. This is the before/after measurement of the planning fixes on the rounds, which no run so far provides.
2. **Audit variance on fixed pages.** Audit 10 unchanged final pages of one run three times each, with the same auditor profile. That measures how much of each round's "new" gaps is sampling. It is cheap (30 audits), and it decides whether reported gaps can measure convergence at all, or whether a fixed checklist per page is needed.

## Current State
- In the four v0.6 full runs, total gaps from the first audit round to the last change by 0.67–1.08 times, against 2.85 in 0.5.0 and 0.09 in 0.1
- Step gaps on pages audited in consecutive rounds stay level in each v0.6 run with three rounds, and 90–100% of them are in passages not reported before
- Each v0.6 run ends with 50–100 gaps left on its pages
- The only full run that converged (0.1) ran without agent definitions on an unrecorded model
- The recorded `exegenius status` of each run compared is in `docs/journal/2026-10-13-convergence-before-after/status/`

## Missing
- No full run with the 2026-10-13 planning fixes, so their effect on the audit rounds is unmeasured
- No measure of audit variance on unchanged pages
- No fixed per-page checklist that would make reported gaps a stock measure
- No wall time or tokens per audit round for any run

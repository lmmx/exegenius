# Budget: what experiments cost, what they bought, and how that steers the next one

**What it answers:** how much each experiment cost, what it settled, and what that implies for choosing and sizing the next experiment (`PROTOCOL.md` §6a).

**What it holds, kept separate:**
- **Measured figures:** token counts from transcripts, via `tools/spend.py` for this development session's subagents, and via the registry for runner sessions.
- **Reported figures:** currency costs a runner reported, and usage-meter readings the author supplied. The development session cannot see the meter.
- **Estimates:** labelled as estimates, never merged into the measured columns.

## 1. Usage-meter readings (supplied by the author; times are when the reading reached this session)

| Time (UTC) | Reading | Window resets | Note |
|---|---|---|---|
| 2026-10-14 ~01:53 | 97% | 05:50 | E6's ten argument retries in flight; most were then killed by the limit |
| 2026-10-14 ~08:00 | 91% | 11:00 | after the overnight run; Max 5× plan |
| 2026-10-14 ~08:45 | 0% | 13:40 | plan upgraded; S2 row 1 (Sonnet `xhigh`) had just finished |
| 2026-10-14 ~09:00 | 2% | 13:40 | S2 rows 2–4 (Opus `xhigh`, Sonnet `max`, Opus `max`) running in parallel since 08:48 |

**These are window readings, not the weekly allowance,** and the two should not be read as interchangeable.

**What they allow:** setting a reading against the tokens measured in the same interval, and so estimating what a run costs as a share of a window.

**What they do not allow yet:** a calibrated rate. Two readings within one window are too few.

## 2. Measured spend, development-session subagents (`tools/spend.py`, 2026-10-14 09:04 UTC)

Tokens read from cache dominate. Output is estimated at 4 characters per token.

| Experiment | Runs: subagents · model · cache read · output | Judging: subagents · cache read · output |
|---|---|---|
| E3 (judging only; runs in a runner session) | — | 15 · 18.3 M · 74 k |
| S1 (judging only; runs in a runner session) | — | 11 · 14.3 M · 58 k |
| T1 | 3 · Haiku · 5.5 M · 19 k | 4 chain walks (shared, below) |
| E4 | 18 · Sonnet · 38.2 M · 166 k | 15 · 29.3 M · 85 k |
| E5 | 15 · Opus · 16.3 M · 98 k | 15 · 24.3 M · 81 k |
| E6 | 42 · Sonnet · 66.9 M · 379 k | 22 · 44.4 M · 127 k |
| E7 | 8 · Sonnet · 23.8 M · 181 k | 22 · 51.9 M · 134 k |
| S2 (in progress) | 4 · Sonnet, Opus · 16.0 M · 136 k so far | not yet |
| W1 (cancelled) | 1 · Sonnet · 0.1 M · 0 | — |
| J1–J2 (content-judge development, E1) | — | 43 · 54.8 M · 192 k |
| Chain walks (baseline and T1) | — | 8 · 6.4 M · 30 k |

**All judging is Opus.** Over E4–E7, judging read 150 M tokens from cache against the runs' 145 M, and at Opus prices it probably cost more.

**Wasted spend, measured:** subagents killed by the usage limit before submitting were rerun.
- **E4:** 8 of 10 first attempts were killed.
- **E6:** 10 of 16 were killed.

Their tokens are inside the run totals above.

**Runner sessions, reported in currency by the runners** (includes orchestration):

| Experiment | Reported cost |
|---|---|
| E1 | 28.27 USD, for 10 Sonnet-high planning runs |
| E3 | 5.46 USD (attempt 1: 0.34 USD more), for 5 Sonnet-medium runs |
| S1 | 21.27 USD, for 6 runs (Opus medium, Opus high, Haiku high) |

## 3. What each experiment bought

| Experiment | Uncertainty it resolved | Preregistered criterion | Decision it supports | Could something cheaper have answered it? |
|---|---|---|---|---|
| E1 | is the planner definition the cause of shallow plans? | met (no difference) | definition is not the lever; effort is | yes, in part: a 2+2 screen would have shown no large difference |
| E3 | does effort explain the depth gap? | met (R1) | plan at high effort | no, a clear effect needed 5 + 5 to separate |
| T1 | does the depth rule cut the central derivation? | not supported | leave the depth rule alone | yes: one replay and one judge would likely have refuted it |
| S1 | does another model reach the calibres? | screen | Opus advances; Haiku does not | about the minimum |
| E4 | does a key-step instruction deepen plans? | not met (errors) | worth confirming | the paired design was efficient; 8 of 10 runs were paid twice through the limit |
| E5 | is Opus at medium better than Sonnet at high? | not met | keep Sonnet high | yes: a 2-run screen with the gap's size in view, before 5 runs |
| E6 | does the key-step instruction hold on fresh extractions? | depth and errors met, gate not | instruction deepens plans safely; the gate needs more | 8 pairs gave a clear depth result; fewer would have sufficed for depth (8 of 8 is p = 0.004) |
| E7 | does the identity sentence close C4? | met | propose for `master` | 8 runs, plus re-judging V, which also measured judge reliability; that was the most informative judging spend so far |
| S2 | does effort above high, or Opus, buy fidelity? | screen, in progress | which configuration, if any, merits a repeat | one run per configuration, by design |

## 4. Lessons the record supports so far (each with its evidence; none is a universal rule)

- **Judging is a cost on the scale of the experiment.** Evidence: E4–E7 (§2).
  - Plan for it in every budget.
  - Measure a cheaper judge's agreement with Opus before using it.
- **Parallel launches near a limit waste work; at the start of a window they cost nothing extra.** Evidence: the E4 and E6 kills, and S2's parallel rows 2–4 at 0–2%.
- **Effort above high makes runs much longer.** Evidence: one run (S2 row 1).
  - Sonnet `xhigh` took 2.7× the time and wrote about 5× the output of `high` on the same task, for a plan of similar size.
  - Whether that bought fidelity awaits judging.
- **Re-judging the same plans is worth its cost once per instrument.** It is what showed single-judge gate verdicts to be noisy (E7).

## 5. Planned use

- **Before an experiment:** the plan gives an estimate for runs and judging, with this file's measured figures as the basis.
- **After it:** `tools/spend.py` is rerun, and §2 and §3 are updated.
- **Meter readings** go into §1 as the author supplies them.

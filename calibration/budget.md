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
| 2026-10-14 ~09:15 | 4% | 13:40 | the author confirmed it on the meter (the autofill had suggested 4%); row 2 (Opus `xhigh`) finished at 09:11; rows 3 and 4 still running |
| 2026-10-14 ~09:25 | 5% | 13:40 | rows 3 and 4 still running; no judging yet |

**These are window readings, not the weekly allowance,** and the two should not be read as interchangeable.

**What they allow:** setting a reading against the tokens measured in the same interval, and so estimating what a run costs as a share of a window.

**What they do not allow yet:** a calibrated rate. Two readings within one window are too few.

**Readings are only as clean as the work running between them.**
- **When several jobs overlap,** the change in the meter is their joint cost, not the cost of any one of them.
- **The ledgers record what ran when,** so an interval can be read as such.
- **Isolated runs** give a per-run figure, at the price of serial time.

**Three quantities are kept apart:** money spent, quota consumed and research value. None is converted into another.

## 2. Measured spend, development-session subagents (`tools/spend.py`, 2026-10-14 09:04 UTC)

**The four token components are kept separate and never summed.**
- **What they are:**
  - uncached input (near zero here);
  - tokens read from cache, which every model call re-reads for the whole context so far;
  - tokens written to cache;
  - output, estimated at 4 characters per token.
- **Why they are not summed:** they are priced very differently. A cache read costs a small fraction of uncached input, and output costs most. Prices also differ by model, and how the subscription's meter weighs each is not published.
- **So the counts below are not a cost or a share of quota.** A ranking by any one column can differ from a ranking by cost.

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

**All judging is Opus.** Over E4–E7:

| | Read from cache | Written to cache | Output | Model |
|---|---|---|---|---|
| judging | 150 M | 11.0 M | 0.43 M | Opus |
| runs | 145 M | 16.2 M | 0.82 M | mostly Sonnet |

So judging re-reads about as much context as the runs, but writes about half the output. Which cost more in quota is not established. Judging uses the pricier model, and the runs produce more output, which is the costliest component per token.

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

| Experiment | Uncertainty it resolved | Preregistered criterion | Decision it supports | Could something cheaper have answered it? (a hypothesis in hindsight, not a measured saving) |
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

- **Making judging cheaper must not make the evidence weaker.** E7 showed that single-judge gate verdicts are noisy. The remedy may be better-targeted judging or a firmer adjudication rule (majority of three on the gate only), not fewer judges.
- **Judging is a cost on the scale of the experiment, not an afterthought.** Evidence: E4–E7 (§2). Judging re-reads about as much context as the runs, on the pricier model, with about half their output. Its exact share of quota is unknown, since the components are priced differently and the meter's weighting is not published.
  - Plan for it in every budget.
  - Measure a cheaper judge's agreement with Opus before using it.
- **Parallel launches near a limit waste work; at the start of a window they cost nothing extra.** Evidence: the E4 and E6 kills, and S2's parallel rows 2–4 at 0–2%.
- **Effort above high makes runs much longer.** Evidence: one run (S2 row 1).
  - Sonnet `xhigh` took 2.7× the time and wrote about 5× the output of `high` on the same task, for a plan of similar size.
  - Whether that bought fidelity awaits judging.
- **Re-judging the same plans is worth its cost once per instrument.** It is what showed single-judge gate verdicts to be noisy (E7).

## 5. Planned use

- **Where the files live:**
  - Shared research infrastructure goes on `master`, where every session inherits it: the protocol, the tools, this file, the plans and the journals.
  - Experimental changes to the pipeline stay on experiment branches until the author decides: for example, the E7 argument instructions on `141026-exp-key-step-identity`.
  - Results are recorded on `master`; the runs that produced them are kept on wiki branches.

- **Before an experiment:** the plan gives an estimate for runs and judging, with this file's measured figures as the basis.
- **After it:** `tools/spend.py` is rerun, and §2 and §3 are updated.
- **Meter readings** go into §1 as the author supplies them.

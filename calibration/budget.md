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
| 2026-10-14 ~12:28 | 14% | ~13:28 ("in an hour", as the author reported it; 13:40 was recorded earlier) | taken just before S3 launched (12:29). Since 09:25: S2 rows 3–4 finished, J9's 17 judges, and this session's write-up. The meter is account-wide, so any other use of the account in that interval is included too |

| 2026-10-14 ~14:20 | ~0% ("basically empty", the author's observation) | ~18:30 (19:30 local, as the author reported it) | the window had reset since 12:28. S3, K1 and S4's launch fell partly in the previous window and partly in this one; no reading separates them |
| 2026-10-15 ~01:06 | 40% used — **confirmed by the author at ~01:25** ("The meter is at 40% used. That is my confirmed reading.") | not stated | the author's message opened "meter says 40% used, launch E11 now", but the decision pasted below it said "I don't have your current meter reading" and "hold until the quota meter confirms sufficient headroom". So it may have been an autofill line. It is kept here, flagged, until the author confirms it on the meter. Measured since the 23:37 reset at 01:07: 384 M cache read (`spend.py --since 2026-10-14T23:37`). Three E11 runs (F4, F5, F6) were launched on it at 01:07; the other three were denied by the session's permission classifier. Nothing more launched until the author confirmed the reading at ~01:25; the other three were then launched at 01:26. Budget gate for E11's later calls (the author's rule): the actual meter, not cache-read tokens; runbook §5's ~300 M-since-the-last-reading pause still applies |
| 2026-10-14 ~22:17 | 65% | ~23:37 ("reset in 1h20m") | the window that began at the ~18:30 reset. Measured in it by `spend.py --since 18:30` at 22:17: J18 241.0 M, E9's runs 12.1 M, J20 12.2 M, and the development session 52.7 M cache read, **318 M together**. Little else ran on the account, as far as this session knows. This is the first reading whose interval matches a whole measured window |

**The 22:17 reading against the measured window (an estimate, not a conversion):**
- The window 18:30–22:17 is 318 M cache read for 65%, about 4.9 M per point.
- That is one interval, and cache read stands in for every component the meter weighs. It is used only to judge whether a launch fits before a reset, and is recorded here so the next reading can test it.
- On it, E10's runs (~16 M, about 3 points) went at 22:17. J21 (~125 M, about 25 points) is held until the ~23:37 reset.

**Autofill predictions, kept apart from readings:**
- The prompt autofill sometimes proposes a meter value.
- A proposal is recorded here as a prediction, with the observation it was checked against. It never goes into the readings above unless the author confirms it on the meter.

| Time (UTC) | Autofill predicted | Observed | Matched |
|---|---|---|---|
| 2026-10-14 ~09:15 | 4% | 4% (author checked the meter) | yes |
| 2026-10-14 ~14:20 | 20% | ~0%: the window had reset | **no**. The prediction ignored the reset |

**Measured since the 18:30 reset, with no reading** (`spend.py --since 2026-10-14T18:30`, 19:26 UTC). These are counts, not a reading:

| | Cache read |
|---|---|
| J18 | 241.0 M |
| E9's runs | 12.1 M |
| J20 | 12.2 M |
| the development session | 45.1 M |
| **together** | **310 M** |

That is past runbook §5's threshold, so E10 waits for a reading or the ~23:30 reset.

**Measured since the ~23:37 reset, with no reading yet** (`spend.py --since 2026-10-14T23:37`, 00:17 UTC on 2026-10-15). These are counts, not a reading:

| | Cache read |
|---|---|
| K3 (pairwise validation, 20 judgements) | 116.5 M |
| J21 (E10, the part after 23:37) | 49.7 M |
| design subagents (K3, A1, EX1 drafts) | 27.2 M |
| W2 and W2b writer runs | 15.9 M + 14.7 M |
| J22 (W2 pages) | 10.4 M |
| S6 runs and J23 | 5.8 M + 9.3 M |
| other | 0 (W2b runs reattributed) |
| the development session | 115.1 M |
| **together** | **~364 M** |

That is past runbook §5's ~300 M threshold. J24 (W2b's page judging, ~8–10 M by J22's measured 0.43 M per judgement; measured 9.67 M) was launched at 00:19 because it completes an experiment whose runs were already spent. E11 (~125–250 M) waits for a reading or the next reset (about 04:40 UTC, assuming a five-hour window; a check-in is scheduled at 04:42).

**Measured since the 14:20 reset, with no reading yet** (`spend.py --since 2026-10-14T14:20`, 15:51 UTC). These are counts, not a reading:

| | Cache read | Cache write |
|---|---|---|
| subagents (S4's run tail, S5, K2, J12, J13, E8, J15, J16, E8b, J17, FB1, J19) | 259.9 M | 18.9 M |
| the development session itself | 110.4 M | 0.45 M |
| **together** | **370 M** | **19.3 M** |

That is past runbook §5's 300 M pause threshold. New launches wait for a reading or for the ~18:30 reset.

**These are window readings, not the weekly allowance,** and the two should not be read as interchangeable.

**What they allow:** setting a reading against the tokens measured in the same interval, and so estimating what a run costs as a share of a window.

**What they do not allow yet:** a calibrated rate. Two readings within one window are too few.

**What the readings say, and do not say, about S2 (an interpretation, not a measurement):**
- **S2's spend from 08:45 to 12:28:** rows 2–4, plus J9, plus the development session's own work, moved the meter from 0% to 14%.
- **The tokens measured in that interval:**

  | | Cache read |
  |---|---|
  | rows 2–4 | 47.7 M |
  | J9 | 42.4 M |
  | orchestration and write-up | not counted by `spend.py` |

- **Why no per-run quota figure:** 14 points over about 90 M cache read, plus unmeasured orchestration, would be one interval of overlapping jobs, with an unknown share for other use of the account. No per-run or per-token quota rate is derived from it.

**Readings are only as clean as the work running between them.**
- **When several jobs overlap,** the change in the meter is their joint cost, not the cost of any one of them.
- **The ledgers record what ran when,** so an interval can be read as such.
- **Isolated runs** give a per-run figure, at the price of serial time.

**Three quantities are kept apart:** money spent, quota consumed and research value. None is converted into another.

## 2. Measured spend, development-session subagents (`tools/spend.py`, 2026-10-14 12:54 UTC for S3; 09:47 UTC for the rest)

**The four token components are kept separate and never summed.**
- **What they are:**
  - uncached input (near zero here);
  - tokens read from cache, which every model call re-reads for the whole context so far;
  - tokens written to cache;
  - visible output (text and tool calls), estimated at 4 characters per token.
- **Hidden reasoning is not measured.** Transcripts keep thinking only as an encrypted signature. `spend.py` reports its length as a proxy, comparable only within one model, and never as tokens.
  - Before 09:47 UTC the registry counted those signatures as output, which inflated its output figures.
  - `spend.py`'s figures were close to visible output all along, since it kept only the last line of each message.
- **Why they are not summed:** they are priced very differently. A cache read costs a small fraction of uncached input, and output costs most. Prices also differ by model, and how the subscription's meter weighs each is not published.
- **So the counts below are not a cost or a share of quota.** A ranking by any one column can differ from a ranking by cost.

| Experiment | Runs: subagents · model · cache read · output | Judging: subagents · cache read · output |
|---|---|---|
| E3 (judging only; runs in a runner session) | — | 15 · 18.3 M · 77 k |
| S1 (judging only; runs in a runner session) | — | 11 · 14.3 M · 61 k |
| T1 | 3 · Haiku · 5.5 M · 19 k | 4 chain walks (shared, below) |
| E4 | 18 · Sonnet · 38.2 M · 169 k | 15 · 29.3 M · 90 k |
| E5 | 15 · Opus · 16.3 M · 101 k | 15 · 24.3 M · 84 k |
| E6 | 42 · Sonnet · 66.9 M · 388 k | 22 · 44.4 M · 132 k |
| E7 | 8 · Sonnet · 23.8 M · 183 k | 22 · 51.9 M · 140 k |
| S2 | 4 · Sonnet, Opus · 53.1 M · 166 k (visible) | 17 · 42.4 M · 110 k |
| S3 | 4 · Opus · 10.6 M · 84 k (visible) | 20 · 59.6 M · 129 k |
| K1 (checklist validation, no runs) | — | 14 · 8.8 M · 39 k |
| K2 (Sonnet checklist judges, no runs) | — | 11 · Sonnet · 8.9 M · 32 k |
| S4 | 1 · Opus `max` · 15.0 M · 43 k | J12 (shared with S5): 11 · 43.1 M · 78 k; J13 checklist: 6 · 4.4 M · 19 k |
| S5 | 1 · Opus `xhigh` · 2.7 M · 26 k | in S4's batches |
| E8 | 3 · Sonnet · 16.8 M · 115 k | J15 checklist: 18 · Opus and Sonnet · 16.0 M · 57 k; J16: 20 · 98.3 M · 164 k |
| E8b | 6 · Sonnet · 27.3 M · 178 k | J17 checklist: 24 · Opus and Sonnet · 23.8 M · 76 k |
| FB1 (pilot) | 1 · Fable · 4.0 M · 31 k | J19 checklist: 4 · Opus and Sonnet · 3.4 M · 12 k |
| E8b stage 2 and E9 (shared batch J18) | E9: 3 · Sonnet · 12.1 M · 106 k | J18: 50 · Opus · 241.0 M · 401 k (4.8 M per judgement, against ~3 M estimated); J20 checklist: 12 · Opus and Sonnet · 12.2 M · 39 k |
| **The development session (orchestration, analysis, write-up), all of 2026-10-14** | 1 · Opus · **707.9 M** · 654 k | — |
| W1 (cancelled) | 1 · Sonnet · 0.1 M · 0 | — |
| J1–J2 (content-judge development, E1) | — | 43 · 54.8 M · 200 k |
| Chain walks (baseline and T1) | — | 8 · 6.4 M · 32 k |

**All judging is Opus.** Over E4–E7:

| | Read from cache | Written to cache | Output | Model |
|---|---|---|---|---|
| judging | 150 M | 11.0 M | 0.45 M | Opus |
| runs | 145 M | 16.2 M | 0.84 M | mostly Sonnet |

So judging re-reads about as much context as the runs, but writes about half the visible output.

**S2 on its own:**

| | Read from cache | Written to cache | Visible output | Model |
|---|---|---|---|---|
| runs | 53.1 M | 3.92 M | 0.17 M | Sonnet and Opus |
| judging | 42.4 M | 3.42 M | 0.11 M | Opus |

Most of the run figure is Sonnet `max` (24.8 M) and Opus `max` (15.9 M). Which cost more in quota is not established. Judging uses the pricier model, and the runs produce more output, which is the costliest component per token.

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
| S2 | does effort above high, or Opus, buy fidelity? | screen: Opus `xhigh`, Sonnet `max` and Opus `max` beyond judge noise (+C1, +3–4 faithful); Sonnet `xhigh` below | repeat Opus `xhigh`, but on extractions with headroom (S3); drop Sonnet `xhigh`; hold the `max` configurations | one run per configuration was right. The input could have been chosen better: F1 was the reference's best extraction, which left little headroom. Choosing the input where the default is weakest would have made the same four runs more informative |
| S3 | does Opus lift the floor where Sonnet `high` is weakest, and is `high` enough? | met: Opus `xhigh` lifts F4 (+6 faithful) and F8 (+9); Opus `high` does not suffice (F8: 9 against 16) | proposal of Opus `xhigh` for the argument task (the author's call); Fable's argument pilot narrowed to F8, after an Opus `max` check | the runs cost under half the estimate. Judging was 85% of the cache read: 20 judgements for 6 plans, as fixed. A cheaper judging design would need agreement measured first (§4) |
| K1, K2 | can a mechanism checklist resolve the top of the scale, and may Sonnet judge it? | adopted (AUC 0.957, judges 78/78, resolves 24, 21, 19); Sonnet agrees with Opus (0.979, no same-model bias) | the checklist as a secondary measure and a cheap screen (about a third of a primary judgement's cache read) | 17.7 M bought a screen that let E8 and E8b, and the Fable pilot, be screened before primary judging |
| S4, S5 | does Opus `max` improve on Opus `xhigh` on F8, and how much does `xhigh` vary? | `max` +1 on each measure, within `xhigh`'s own run variance (2 faithful) | no case for `max` on this task; Fable's pilot against Opus `max` | the repeat (S5, 2.7 M) changed the reading of a 15 M run. Repeats are cheap next to the runs they qualify |
| E8 | does one sentence (variants, bound chains) lift Sonnet `high`? | faithful 18, 18, 19 against 14, 3, 4; not below Opus `xhigh` anywhere | the sentence proposed to the author; E8b to confirm | the largest gain of the day cost 16.8 M of runs. Instructions targeted at a located gap beat model and effort. Primary judging (98.3 M) was 6× the runs |
| FB1 (pilot) | does Fable at `high` beat Opus `max` on F8? | below: checklist 17, 16 against 20, 21 | no further Fable spend on this task | narrowing to a checklist screen kept the pilot at 7.4 M in all |
| E8b stage 2, E9 | does the E8 sentence hold on fresh extractions, and does a parameters-and-cases sentence close C1? | E8b: not supported (3 of 5; ties on F5, F6). E9: closes C1, with every judgement 19/19 faithful | the E8-only proposal withdrawn; E10 tests the combined instruction where E8 tied | consolidating E9 into J18 shared the W and X plans and saved a batch. J18's per-judgement cost was 1.6× J16's, because the plans are longer. Estimate judging from the latest batch, not the first |

## 4. Lessons the record supports so far (each with its evidence; none is a universal rule)

- **Making judging cheaper must not make the evidence weaker.** E7 showed that single-judge gate verdicts are noisy. The remedy may be better-targeted judging or a firmer adjudication rule (majority of three on the gate only), not fewer judges.
- **Judging is a cost on the scale of the experiment, not an afterthought.** Evidence: E4–E7 (§2). Judging re-reads about as much context as the runs, on the pricier model, with about half their output. Its exact share of quota is unknown, since the components are priced differently and the meter's weighting is not published.
  - Plan for it in every budget.
  - Measure a cheaper judge's agreement with Opus before using it.
- **Parallel launches near a limit waste work; at the start of a window they cost nothing extra.** Evidence: the E4 and E6 kills, and S2's parallel rows 2–4 at 0–2%.
- **Effort above high makes runs much longer, and buys fidelity only for some model-effort pairs.** Evidence: S2, one run each, judged by majority in J9.
  - Against Sonnet `high`, the times were:

    | Configuration | Seconds |
    |---|---|
    | Sonnet `xhigh` | 2.7× |
    | Opus `xhigh` | 2.0× |
    | Opus `max` | 2.9× |
    | Sonnet `max` | 4.0× |

  - Opus at both levels, and Sonnet at `max`, gained C1 and 3–4 faithful derivations. Sonnet `xhigh` lost 2.
  - *Corrected 09:47 UTC:* this entry said that Sonnet `xhigh` "wrote about 5× the output of `high`". That compared a usage figure with a visible-content figure. On one basis, visible output was about equal: 27 k against 25 k. The longer run went into hidden reasoning (thinking signatures 3.6× longer), which transcripts do not count.
- **Measure every run on one basis before comparing.** Evidence: the correction above, and the registry's output figures, which counted thinking signatures until 09:47 UTC. A comparison across two estimators can look like a 5× effect where there is none.
- **Estimate runs from the median run, not one run.** Evidence: S3's estimate scaled from S2's single Opus `xhigh` run (6.93 M). S3's two Opus `xhigh` runs measured 4.02 M and 2.91 M.
- **Judging dominates a small screen's cost.** Evidence: in S3, judging was 85% of the cache read (20 judgements against 4 runs). Three judges per plan is what E7 showed the gate needs, so the lever is the number of plans per batch, not judges per plan.
- **Effort above `high` can silently waste a run's main call.** Evidence: S2F1SX's planning call stopped at the 128,000-token output limit after 14.5 minutes, and its thinking was not carried forward (`docs/journal/2026-10-14-sonnet-xhigh-and-rubric.md`). The submit still succeeded. The tokens were spent and counted, and what they bought was lost. The registry now records `max_tokens_stops`, so such a run can be found before it is interpreted.
- **Choose a screen's input where the current default is weakest.** Evidence: S2 screened on F1, where the reference was already at 17 of 19 faithful in J8. Three of four configurations then hit the ceiling, so the screen could rank nothing above them, Fable included.
- **Orchestration is not free: the development session's context is re-read on every call.** Evidence: `spend.py` (with the session's own transcript, 15:51 UTC).
  - Over the day it read 708 M from cache, against 840 M for all subagents together.
  - Before the last compaction a single call re-read 600–670 k tokens, about what one checklist judgement reads in all. After compaction it fell to about 200 k.
  - **The levers:**
    - fewer, larger tool calls;
    - no polling (completion notifications arrive on their own);
    - bulky reading in subagents, which return conclusions;
    - compaction before the context grows large.
  - This is a component count, not a quota share. Cache reads are the cheapest component per token, and the meter's weighting is not published.
- **Re-judging the same plans is worth its cost once per instrument.** It is what showed single-judge gate verdicts to be noisy (E7).

## 5. Planned use

- **Where the files live:**
  - Shared research infrastructure goes on `master`, where every session inherits it: the protocol, the tools, this file, the plans and the journals.
  - Experimental changes to the pipeline stay on experiment branches until the author decides: for example, the E7 argument instructions on `141026-exp-key-step-identity`.
  - Results are recorded on `master`; the runs that produced them are kept on wiki branches.

- **Before an experiment:** the plan gives an estimate for runs and judging, with this file's measured figures as the basis.
- **After it:** `tools/spend.py` is rerun, and §2 and §3 are updated.
- **Meter readings** go into §1 as the author supplies them.
- **S3, done:** estimated at ~70–78 M cache read and ~7–8 M cache write; measured at 70.2 M and 4.79 M. The runs came in at under half their estimate, and judging at about 1.2×.
- **K1, done:** estimated at ~20–35 M cache read; measured at 8.8 M (14 checklist judgements at 0.63 M each).
- **S4 to FB1, done:** see §2 and each plan's result section for estimates against measured figures.
- **Queued at the reset:** E8b stage 2 with E9 (`plans/E8b.md`, `plans/E9.md`): 3 runs and about 62 judgements, roughly 190 M cache read by the plans' estimates.
- **Superseded proposals (kept for the record):**
  - **Opus `max` on F8,** judged with S3F8OX on the primary rubric (8 judgements) and on the checklist (4): ~37–43 M cache read (`docs/journal/2026-10-14-k1-checklist.md`).
  - **The Fable pilot after it** (`plans/FB1.md`): per extraction, one Fable run of unknown cost (S2's Opus `max` run, 15.9 M cache read, is the placeholder), plus ~27 M cache read of Opus judging.

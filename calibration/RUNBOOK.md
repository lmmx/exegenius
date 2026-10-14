# Calibration runbook: standing policy, decision tree and queue

**What it is:** the operating document for the calibration programme. A session resuming this work, or the author returning to it, should be able to act from this file without the conversation. It is maintained: the queue below is updated as items finish.

**Its authority:** adopted on 2026-10-14 at the author's request. The author wants the development session to own routine research coordination, plan ahead, and stop asking for approval of steps this policy covers. The author keeps the decisions listed under "Escalate".

## 1. What the session does without asking

- Run experiments that are preregistered in `plans/` and covered by this queue, including the next item when its trigger is met.
- Follow a preregistered decision table to its stated action.
- Analyse existing evidence (transcripts, plans, verdicts, registry). Build and commit tools, records, plans and journals.
- Create experiment branches and binaries (never on `master`) for instruction or configuration variants.
- Stop, defer or narrow an experiment when the evidence or the usage risk warrants it, and record why.
- Consolidate judging batches when plans that must be compared are ready at the same time, provided no result has been seen.

## 2. Escalate (ask the author, briefly, and only for these)

- Changing any pipeline default, or merging an experiment branch to `master`.
- Changing a preregistered criterion after any result it governs has been seen.
- Anything irreversible: deleting branches or records, force-pushing, or closing or merging PRs (#9 and #10 included).
- Spending outside this queue, beyond its stated budget by more than half again, or more than one Fable run without a reported result.
- Choosing a new document or document class: the source must be supplied, since this session cannot fetch it.
- A full wiki run (writers, auditors, readers). The author cancelled W1, so it needs the author's go-ahead.

## 3. Experiment selection policy

1. **Decision first.** Each experiment names the decision it could change and what each outcome would do to it. No decision, no run.
2. **The cheapest test that could resolve the uncertainty,** in this order:
   - existing evidence;
   - judge-free tools (`markers.py`, transcripts, the registry);
   - the checklist, a cheap judged screen;
   - full primary judging;
   - a new run.
3. **Factors are kept separate:** model, effort, task stage, input (extraction or document), instruction version, judge model. One factor changes per comparison unless the plan says why not.
4. **The cheapest configuration that meets the quality bar.** Higher effort, or a stronger model, only with a stated hypothesis about what it should improve. Never because it is available or because a previous result was close.
5. **Inputs with headroom.** Compare configurations where the incumbent is below the ceiling (`plans/S3.md`: F4 and F8, not F1).
6. **Count failures, not just submits.**
   - A `max_tokens` stop (`max_tokens_stops` in the registry) is a result against the configuration.
   - A usage-limit kill is an operational failure: rerun it after moving its scratch files aside.
   - An accepted submit is not quality evidence.
7. **One run per condition is a screen.**
   - A repeat happens only where it could change a decision.
   - No default changes on one paper or one run per condition.
8. **Fable** is not run where the incumbent is at the ceiling. It runs as one budgeted pilot per preregistered plan, never as a programme.

## 4. Evaluation policy

- **Primary measures:**
  - the content judge's counts (J3's instructions);
  - the central gate C1–C9 by majority of three judges;
  - faithful derivations.
- **Secondary measure:** the mechanism checklist (`rubrics/nlogn-mechanism-checklist.md`, validated in K1). Two judges per compared plan.
- **Two-stage judging is allowed for screens:**
  1. the checklist first (0.63 M cache read per judgement);
  2. full primary judging (about 3 M per judgement) only if the checklist shows a gain beyond noise.
  
  Claims about an instruction or configuration rest on the primary measures. The checklist alone supports only "worth confirming".
- **Same batch:** plans to be compared are re-judged in one batch, never compared across batches. Anchors E1-P4 and E3-M3 go in every primary batch.
- **Blind:** plans are shuffled by `prepare.py` with a recorded seed. Judges see only their instructions, the paper and one plan.
- **Outcome categories:** gain beyond noise; tie (ranges overlap); ambiguous (the measures disagree); operational failure (the run or judge failed). Each is reported as itself.

## 5. Usage conventions

- **Token components** (uncached input, cache read, cache write, visible output, thinking-signature length) are kept separate, measured by `tools/spend.py` and the registry, and never converted to cost or quota.
- **Meter readings** come only from the author, timestamped, in `budget.md` §1. Autofill predictions go in their own table with the observed outcome.
- **Estimates against actuals:** each plan states its estimates, and its result section the measured figures beside them.
- **The window** (about five hours; the next reset is about 18:30 UTC on 2026-10-14):
  - **Runs:** launched in parallel when the last reading, or the measured spend since the reset, leaves room. With no reading for a while, the session pauses new launches after about 300 M cache read since the last reading. That is a safety threshold, not a conversion. From the observed 0% → 14% over S2's runs, judging and write-up, it is well under one window, and the session then asks for a reading.
  - **Resets:** do not interrupt running jobs. Only exhaustion does.
  - **Use:** the window is not there to be used up. The aim is research progress per unit of usage.
- **Concurrency is recorded in each ledger,** so a meter interval can be read as joint.
- **At most 20 subagents run at once** in this environment (`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`; hit at 14:50 on 2026-10-14). Batches beyond that are launched as slots free, and the ledger records the launch times.

## 6. Decision tree and queue (2026-10-14, 14:28 UTC)

| # | Item | Question | Trigger | Cost estimate (cache read) | Status |
|---|---|---|---|---|---|
| 1 | **S4** (`plans/S4.md`) | does Opus `max` improve on Opus `xhigh` on F8, and reach the ceiling? | running | ~33–43 M | run launched 14:13 |
| 2 | **S5** (`plans/S5.md`) | how much does Opus `xhigh` vary between runs on the same extraction (F8)? | now; judged in S4's batches | ~13 M | run launched 14:30 |
| 3 | **K2** (`plans/K2.md`) | do Sonnet judges agree with Opus judges on the checklist, with no same-model bias? | now; existing plans | ~5–7 M | **done** 14:31: agreement 0.979, ρ 0.995, bias +0.43 (not flagged). Checklist screens may use a Sonnet second judge |
| 4 | **E8** (`plans/E8.md`) | does a sentence asking for every variant and every bound's chain lift Sonnet `high` towards Opus `xhigh`? | now; two-stage | ~17 M (stage 1); +~60 M (stage 2, if the screen passes) | 3 runs launched 14:30 |
| 5 | **FB1 pilot** (`plans/FB1.md`, "The pilot") | does Fable at `high` beat the incumbent on F8? | S4 outcome "improves but not ceiling" (against Opus `max`), or "does not improve" (against Opus `xhigh`). Not if S4 reaches the ceiling | run unknown (placeholder 6–16 M) + ~30 M judging | conditional |
| 6 | Markers tightened (L2, P2, P3, S4) | the judge-free cross-check | now; free | 0 | **done** 14:31: fitted agreement 0.836 → 0.905 on K1 plans (optimistic, since these were used to tighten); J8 hold-out AUC 0.774 → 0.794 |
| 7 | **Second document** | do the model and instruction effects generalise? | **needs the author to supply a document** (§2) | onboarding: reference set, central rubric and checklist, about 20–40 M of analysis; then a screen | blocked |
| 8 | Full wiki from the best plans, with simulated readers on | does reconstruction survive the writers, and do readers reach the key step? | needs the author (§2) | large (writers and auditors) | blocked |

**Branches after S4** (its table, applied here):
- **Reaches the ceiling:**
  - No Fable on this paper's argument task.
  - The open question becomes cost: is Opus `max` worth 2–5× Opus `xhigh`?
  - E8 decides whether a cheaper model with better instructions closes the gap.
  - The second document becomes the priority.
- **Improves, not the ceiling:** the FB1 pilot runs on F8 against Opus `max`.
- **Does not improve:** the FB1 pilot runs on F8 against Opus `xhigh`. S5 tells how much of S4's tie is run variance.

**Branches after E8:**

| E8 outcome | Next |
|---|---|
| The checklist screen shows a gain on ≥ 2 of 3 extractions | stage 2: primary judging. Then, if confirmed, the variant sentence is proposed to the author as an instruction change, and the configuration question is re-asked with it: does Sonnet `high` plus the sentence match Opus `xhigh`? |
| No gain | the gaps are capability, not instruction. The Opus configuration stands as the candidate |

**What would justify proposing a planning configuration to the author** (for this paper's argument task, not the general default):
- the candidate beats Sonnet `high` beyond noise on ≥ 3 extractions (met by Opus `xhigh`: S2 and S3);
- its run-to-run variance (S5) is smaller than that gain;
- no `max_tokens` stops;
- its cost is stated.

The general default additionally needs the second document.

## 7. Stopping conditions

- Each plan stops at its own stopping rule. The queue stops when it is empty, when every remaining item is blocked on the author, or when the usage safety threshold is reached with no reading.
- At a stop, the session updates this file's queue, the budget and the capability map, and reports once, briefly: what finished, what it decided, and what is blocked and on what.

## 8. Resuming

1. Read this file, then the ledgers of every item marked running or queued.
2. Check for running subagents and for finished runs not yet judged:
   - `exegenius/log.jsonl` in each run dir;
   - the subagent transcripts, for `max_tokens` stops.
3. Never restart a run that may still be running.
4. Take the first item in the queue whose trigger is met.

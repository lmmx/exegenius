# Calibration Protocol

**The question this protocol answers:** for a given kind of document, task and required calibre, which is the least resource-intensive configuration that meets the calibre reliably? And how sure are we?

**What it is:** a method for fair, efficient and repeatable experiments on exegenius's agent configurations. Each experiment run under it produces records in the Evaluation Registry (`registry/`). The Capability Map (`capability-map.md`) summarises what those records support.

**What it is not:** a finding. Findings belong to experiments. Each is scoped to the versions, document and judge that produced it, and is recalibrated when any of them changes.

## 1. Terms

| Term | Meaning |
|---|---|
| **Document class** | What is being explained, and how hard it is: for example "frontier mathematics paper (LaTeX, ~5k lines)", or "expository blog post". A result is scoped to its class until shown on another. |
| **Task** | An exegenius task kind (extract, background, argument, write, audit, support, …), or a stage made of several (planning = extract + background + argument). |
| **Configuration** | Everything that can change an outcome. See section 2. |
| **Arm** | One configuration in an experiment. |
| **Run** | One execution of a task or stage under one arm, from a fresh project. |
| **Calibre** | A named quality target with hard gates (section 4). |
| **Judge batch** | One blind judging pass over a set of outputs, with one set of instructions, anchors and second judgements. |
| **Instrument** | Anything that produces a quality measure: the content judge, a rubric, a simulated-reader round, a person. |

## 2. The configuration and its dimensions

A configuration is recorded in full for every run, from what the transcripts record, never from what was intended. Any field that cannot be observed is recorded as unavailable.

| Dimension | What is recorded | Notes |
|---|---|---|
| **Model** | the exact model id each subagent ran on (from each assistant message) | a family name ("Sonnet") is not enough: versions change capability |
| **Effort** | `effort` / `perTurnEffort` from the transcript | ordinal *within* a model: low < medium < high < xhigh < max. **No equivalence is assumed across models.** "Opus at medium" and "Sonnet at high" are different points, compared only by measurement. |
| **Agent definition** | the subagent type, and the sha256 of its definition file, or `general-purpose` | a definition is a system prompt plus a tool list. A general-purpose agent with the task prompt is a legitimate arm, and a definition must earn its place by measurement. |
| **Task prompt** | the exegenius commit that wrote the task files and printed the prompts | task text changes with the binary |
| **Settings** | `.claude/settings.json` (deny rules, permission mode) | |
| **Orchestrator** | the model and effort of the session that hands out the tasks | it only passes prompts, but its effort is the subagents' default when none is passed |
| **Source and reference** | the source file's sha256, and the reference set's version | |

## 3. Instruments, in four levels

Each level is reported separately. **No single score combines them**, because a high score on one can hide a failure on another. A plan that states every result can still fail to explain the central one.

| Level | Asks | Instruments now | Status |
|---|---|---|---|
| 1. Correctness and coverage | Are the claims right, and are the important results there? | the content judge (journal 2026-10-13-content-judge): statements per reference item, wrong statements, major and minor errors | in use; the reference set is an unreviewed draft |
| 2. Derivational fidelity | Is the reasoning reconstructed, not just asserted? | the content judge's derived / faithful verdicts; **central-derivation rubrics** (`rubrics/`), one per document, for the chain the document turns on | in use from E3 |
| 3. Explanatory structure | Does the wiki carry the reader along the proof plan's paths, so that the moments where a subplan's place becomes visible happen? | see `fidelity/README.md` | designed; the machinery is built, but it has never run on the paper |
| 4. Reader understanding | Can a reader answer questions they were not trained on? | held-out goals with simulated readers (built); people (not yet) | a simulated reader is a proxy: see `fidelity/README.md` |

**Proxies are diagnostics, not outcomes.** These include where a plan's quotes fall (`score.py`, `depth.py`), coverage figures, and gap counts. They help find failures. They do not show quality.

## 4. Calibres

A calibre is a set of **hard gates**: all must pass. It is defined per document class and task **before** the experiment that uses it. The thresholds are provisional until validated against human judgement.

**Planning on a frontier mathematics paper** (the n log n paper's reference has 19 core results):

| Calibre | Gates |
|---|---|
| **Overview** | ≥ 17 of 19 core results stated in substance; no major error; no statement judged wrong |
| **Technical** | Overview, and ≥ 10 of 19 core results derived |
| **Research** | Technical, and the central-derivation gate passed (`rubrics/nlogn-central-saving.md`) |

These thresholds were written after E1's results had been seen, so E1 cannot validate them. E3 is their first use.

Calibres for other document classes are defined when the first experiment on that class is designed. A blog post has no "central derivation", but it may have a central argument whose chain can be written as a rubric in the same way.

## 5. Experimental design

1. **Preregister.** Commit the question, arms, measures and decision rule before the runs start. Commit the rubric before any output it applies to exists. The commit's time is the record (`plans/`).
2. **Change one dimension at a time** when that answers the question (E1: the definition at fixed model and effort; E3: effort at fixed model and definition). Use a factorial design only when an interaction could change a decision. Even then, run only the cells that could change it.
3. **Pin everything else.** Use the same binary commit, source, configuration, prompts (word for word from `exegenius next`), settings and runner protocol. Pass model and effort explicitly in every Agent call, then check the transcripts. A run whose recorded model or effort differs from its arm's is excluded and reported.
4. **Use a runner session.** A separate cloud session runs the experiment on a dated wiki branch. The definitions are loaded natively, the binary is built from an attached source at the pinned commit, and only facts are reported. The development session analyses. Template: `runner/`.
5. **Judge blind, in one batch per comparison.**
   - Shuffle the outputs with a recorded seed, and hold the key outside the judges' reach until every verdict is in.
   - Include **anchors**, outputs judged in an earlier batch, to detect drift in the judges' scale.
   - Have **second judgements** of about a quarter of the outputs, drawn at random, to measure agreement.
6. **Reuse runs** only when the binary, configuration and protocol are identical, and state the difference in time and session as a limitation. Reused outputs are re-judged in the new batch, never compared across batches.

## 6. How many runs: sequential allocation and stopping

The aim is to spend runs where they can change a decision. Runs are added in stages, and each stage's rule is fixed in the preregistration. **Screening evidence and confirmatory evidence are different things,** and reports keep them apart.

| Stage | Most runs per arm (cumulative; §6a: reached only when a decision needs them) | Supports | Rule |
|---|---|---|---|
| **Screen** | 2 | **only** a decision to stop spending on an arm | An arm is **screened out** for a calibre if both runs fail a hard gate by a wide margin: on the primary count, below half the gate's threshold, or with a major error. Otherwise it goes on. Passing the screen says nothing about reliability, and a screened-in arm is never called good on screening evidence. |
| **Estimate** | 5 | a **provisional** comparison or recommendation, scoped to the document, versions and judge batch | Report each arm's pass count for the calibre with its exact two-sided 95% (Clopper–Pearson) interval, and the primary counts' medians and ranges. Compare two arms with the exact Mann–Whitney test on the primary count, one-sided in the preregistered direction. A difference is **distinguished** if p < 0.05 *and* it is at least the preregistered minimum difference. |
| **Confirm** | up to 8, or a replication on fresh runs | a **confirmatory** claim | Only when the decision is consequential (it would change a default, a profile or a recommendation) and stage 2 is inconclusive, or when a stage-2 result is to become a default. Use fresh runs, never re-used outputs. If 8 runs do not distinguish the arms, the report says **inconclusive**. Practical equivalence is not claimed without a preregistered margin. |

**The interval convention, and the assumptions behind these figures, stated so they are not over-read:**
- **The convention.** Pass rates are reported with the **exact two-sided 95% Clopper–Pearson interval** (`tools/calibre.py`). It is conservative, and it is the safer choice at the small counts used here, where the Wilson interval is least reliable.
  - **What it gives at these counts:** 2 of 2 passes leaves a true pass rate anywhere from **16%** to 100%. 5 of 5 leaves it anywhere from **48%** to 100%.
  - **Other conventions, for comparison:** the exact *one-sided* 95% lower bound for 5 of 5 is 55%, and the two-sided Wilson bound is 57%. A figure from any convention is reported with its convention named.
- **What the interval supports.** It describes uncertainty about **one configuration's pass rate, on this document, at these versions, judged by this judge**, assuming independent runs. It is not the probability that the configuration will pass next time. It is not evidence of reliability on other documents or tasks.
  - **The decisions it supports:** screening (an upper bound below the calibre's useful rate stops spending on an arm), and provisional recommendations at stage 2. 5 of 5 is encouraging screening evidence, not proof of general reliability.
- **Runs within an arm are treated as independent draws from one configuration.** Runs made at the same time, in one runner and sharing one orchestrator, may not be. That is a limitation, not something the statistics correct.
- **A Mann–Whitney p-value is meaningful only for a comparison and direction fixed before the data.** With 5 against 5 the smallest one-sided p is 1/252 ≈ 0.004, so a clear separation is detectable, and a difference of one or two counts usually is not.
- **Several comparisons and several measures inflate the chance of a spurious "distinguished".** Each preregistration names **one** primary measure per comparison, and treats the rest as secondary and descriptive.
- **Calibre thresholds chosen after seeing a result cannot be validated by that result.** The report says which thresholds were chosen after the data.
- **When outputs are re-used across experiments** (for example, an arm from an earlier experiment), they are re-judged in the new batch, and the comparison is called sequential, not simultaneous.

**The unit of replication.** A run is one fresh execution of a task stage, from a fresh project, on **one document**.
- **What N runs estimate:** a pass rate for that document at those versions.
- **Reliability across documents:** needs runs on several documents of the class, with the document as the unit.
- **Reliability across classes:** needs runs on several classes.
- **Shared systematic biases** (one judge model, one reference set, one runner) are not reduced by more runs. They are named as limitations.
- **Clopper–Pearson's assumptions:** it takes the runs to be independent trials with one common success probability. If they are not independent, its nominal coverage does not hold.

**From interval to decision.** For a calibre, each preregistration sets a **target pass rate** (by default 0.8: a configuration recommended for a calibre should meet it four times in five). Then:

| Decision | Requires |
|---|---|
| **Reject** the configuration for the calibre | the exact upper bound is below the target (for example 0 of 5, upper 0.52; 1 of 21, upper 0.24) |
| **Advance** (keep testing) | passing the screen, or an interval that straddles the target |
| **Recommend provisionally** | an observed rate at or above the target at stage 2 (4 or 5 of 5), and no cheaper configuration that also meets it; scoped to the document and versions |
| **Confirm** | on fresh runs, an exact lower bound of at least 0.5 (6 of 6 or better), and the same result on at least one more document of the class before it is called a default for that class |

**Why not a full matrix.** Model × effort × definition × task × document is too large to fill. Paired experiments answer most questions, and screening drops arms that fail badly after 2 runs. Interactions are tested only where they could change a decision.

**Operational failures are not results.** A runner that fails before producing outputs (a refused build, a lost session) is recorded as an operational deviation in the experiment's spec and on its branch. Its runs are not counted as failed runs of the configuration, and the experiment continues from a clean runner under the same protocol.

## 6a. Budget and sequencing (added 2026-10-14; applies to every experiment from S2 on)

The stages above give the **most** runs an arm may need, not a quota. Over E4–E7 they were applied as quotas, and the account's usage limit was spent twice, while model × effort went largely untested (journal 2026-10-14-sequential-design-lesson). Every experiment now follows these rules, and its plan says how.

1. **State the decision first.** Say what the result will change, and what result would change it.
2. **Screen sequentially.**
   - A new configuration starts with **one** run, when one run can show whether it is worth more.
   - No arm gets a fixed number of runs by default.
   - A second run, or the stage-2 estimate, is added only when the decision needs it.
3. **Budget before launching.**
   - Estimate the tokens and time of the runs **and of the judging**.
   - Reserve enough to finish, judge and record within the current usage window.
   - A run that cannot plausibly finish before the limit waits for the reset.
4. **Run serially near a usage limit,** one subagent at a time, when a cancellation would waste work in flight.
5. **Repeat adaptively.** Repeat a configuration only when its result is near a decision boundary or within judge or run noise of a competitor, never every cell of a design.
6. **Screening is not confirmation.**
   - One run can flag a configuration as promising or as poor.
   - It cannot rank configurations or establish equivalence.
   - One poor run stops spending on a configuration only if that configuration also costs more.
7. **Keep negative results.**
   - Stopping work on a configuration is recorded with its results and their uncertainty.
   - No result is dropped.
8. **Label every observation** as new, reused, paired or re-judged. Re-judging measures the judges, not the configurations.
9. **Report decision value with quality.** Each result gives:
   - its cost (tokens; currency where available);
   - its time;
   - its central steps and gate;
   - its major errors;
   - whether it changes the recommendation.
10. **Keep a ledger.**
    - Each plan holds a ledger of its runs, updated as each finishes.
    - A retry resumes from the ledger.
    - A killed run's scratch files are moved aside before it reruns.
11. **Budget judging as a cost of its own.**
    - Judge only what a decision needs.
    - Where the central gate decides something, use **three independent judges per plan** and decide each step by majority: single-judge gate verdicts changed in 3 of 8 plans on re-judging (E7).
    - A cheaper judge model may replace Opus once its agreement with Opus judges has been measured.
12. **Keep the cumulative budget.** `calibration/budget.md` holds the usage-meter readings the author supplies, measured spend per experiment split into runs and judging (`tools/spend.py`), and what each experiment bought. A new plan's estimate starts from it, and the file is updated when an experiment ends.

## 7. Resources

Recorded per subagent and per run, each with its **basis**:

| Measure | Basis |
|---|---|
| model calls; input, cache-read and cache-write tokens | measured (transcripts) |
| output tokens | **estimated** at 4 characters per token: transcripts record usage when streaming starts, and undercount output |
| seconds per subagent; planning wall time | measured (transcripts, run log) |
| rejected submits; restarts | measured (run log, runner report) |
| session cost in USD | **reported** by the session, including orchestration; not a marginal price |
| subscription quota | **unavailable**: plan limits are not published as token allowances. Estimating one needs `/usage` readings taken before and after runs. |

## 8. Recalibration

A Capability Map entry holds for the exact versions in its records. It is flagged for recalibration when any of these changes:
- the model id;
- the effort semantics;
- an agent definition's hash;
- the task-prompt text (the binary's task rendering);
- the source class;
- the reference or rubric version;
- the judge's instructions or model.

Recalibration reruns the entry's smallest distinguishing comparison, usually its stage-2 pair, not the whole programme.

## 9. Records

Every experiment writes one record to `registry/experiments/<ID>.json`, following `registry/schema.md`. Records are built mechanically by `tools/registry.py` from the evidence: run logs, transcripts, verdicts and runner reports. Hand-entered fields are marked `reported` with their source. A value that cannot be found is `unavailable`, never inferred.

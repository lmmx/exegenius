# 2026-10-14: How the calibration programme spent its budget, and how it will now

**The conclusion.** Over the night of 13–14 October the programme ran a sequence of planning experiments:

| Experiment | What it tested |
|---|---|
| E4 | the key-step instruction, paired |
| E5 | Opus at medium, 5 runs |
| E6 | the key-step instruction, 8 paired runs |
| E7 | the identity sentence, 8 runs |

It also ran four judging batches, J5–J8. The instruction work answered its questions. But the programme spent the account's usage limit twice. The cause was experiments designed by filling comparisons (5 or 8 runs per arm, every arm judged blind with seconds) before deciding how much evidence each decision needed. Meanwhile the question the author cared about, how model and effort shape the result, stayed largely unexplored:
- **Covered:** Sonnet at medium and high, Opus at medium, two runs each of Opus at high and Haiku at high.
- **Not run at all:** `xhigh` and `max`.

The author identified this. The design rules below are now part of `calibration/PROTOCOL.md` (§6a). They change how resources are allocated from here on. They do not alter any preregistered criterion or remove any result.

## What went wrong

1. **Fixed repetition counts by default.** The protocol's stages (screen 2, estimate 5, confirm 8 runs per arm) were applied as quotas, not as maxima to reach only when a decision needed them.
   - E5 ran 5 Opus-medium runs.
   - E6 ran 8 pairs.
   - Nothing in the protocol said "one run of a new configuration may be enough to decide whether to look further".
2. **No budget before launching.** No experiment estimated its quota cost, or reserved enough to finish and judge before the usage limit. Twice, about ten subagents were killed mid-task by the limit (E4 and E6). Each time they were rerun after the reset: the same work paid for twice, and recorded as deviations.
3. **Parallel launches near a limit.** Ten argument tasks at once maximised the work lost when the limit hit.
4. **Judging cost as much as the experiments, and it was not budgeted.** Measured from this session's transcripts:

   | Work | Subagents | Model | Cache read | Output (est.) |
   |---|---|---|---|---|
   | planning runs, E4–E7 | 83 | Sonnet; Opus for E5 | 145 M | 0.82 M |
   | blind judging, J5–J8 | 74 | Opus | 150 M | 0.43 M |

   **Why judging was as expensive:**
   - each plan was judged by an Opus subagent reading a 77 k-character reference pack and the 5,361-line paper;
   - anchors and second judgements added a third again;
   - at Opus prices, judging probably cost more than the runs it judged.
5. **The model × effort question was deferred, not answered.**
   - S1's screen advanced both Opus arms. Only Opus at medium went on (E5, chosen as cheaper and as the author's hypothesis), and Opus at high was never estimated.
   - When the key-step instruction took over the agenda, effort above `high` was never screened.
   - The capability map listed "Sonnet at xhigh" as open from the start.
6. **Reused and re-judged outputs were easy to miscount as new evidence.**
   - E7's reference arm was E6's plans, re-judged.
   - The J7/J8 comparison is a test of the judges, not of the models.
   - The records say so, but a summary of the night (ChatGPT's, relayed by the author) still mixed them.

## The rules from now on (`calibration/PROTOCOL.md` §6a)

1. **State the decision first.** What will the result change, and what result would change it?
2. **Screen sequentially, one run per new configuration** where a single run can show whether a configuration is worth more. No arm gets a fixed number of runs by default.
3. **Budget before launching.** Estimate tokens and time for the runs *and the judging*, and reserve enough to finish, judge and record within the current usage window. A run that cannot plausibly finish waits for the reset.
4. **Run serially near a usage limit.** One subagent at a time when a cancellation would waste work.
5. **Repeat adaptively.** Repeat only a configuration whose result is near a decision boundary or within noise of a competitor, never every cell.
6. **Keep screening and confirmation apart.**
   - One run can flag a configuration as promising or poor.
   - It cannot rank configurations or show they are equivalent.
7. **Keep negative results.**
   - Stop spending on a configuration when its results and cost justify stopping.
   - Never drop a result from the record.
8. **Label every observation as new, reused, paired or re-judged.** Re-judging measures judges, not models.
9. **Track decision value with quality.** For each result report cost, time, central steps, major errors, and whether it changes the recommendation.
10. **Make interruption recoverable.**
    - Each experiment keeps a ledger in its plan, updated as each run finishes.
    - A retry resumes from the ledger and moves a killed run's scratch files aside first.
11. **Budget judging as a cost of its own.**
    - Judge only the plans a decision needs.
    - Use three judges per plan only where the gate decides something.
    - Consider a cheaper judge model once its agreement with Opus judges is measured.

## What changed in the files

- **`calibration/PROTOCOL.md`:** a new §6a (these rules), and §6's stage table now reads its run counts as maxima reached only when a decision needs them.
- **`calibration/runner/EXPERIMENT.template.md`:** the arms line no longer presumes `{n}` runs per arm. It asks for the decision, the budget and the stopping rule.
- **`calibration/plans/W1.md`:** the full-wiki integration check, marked cancelled by the author before any task finished.
- **`calibration/plans/S2.md`:** the first experiment under the new rules. It is a model × effort screen with the E7 instructions:
  - one run each of Sonnet `xhigh`, Opus `xhigh`, Sonnet `max` and Opus `max`;
  - run serially, with a ledger;
  - judged afterwards, with three judges per plan.

## Not changed

- **Preregistered plans and results** of E1–E7. Their criteria stand as written, and their evidence is kept.
- **The capability map's entries,** apart from noting the open model × effort questions.

## Current State
- The calibration protocol now requires a stated decision, a budget covering runs and judging, sequential one-run screens for new configurations, serial execution near usage limits, adaptive repeats and a resumable ledger (§6a)
- Blind judging with Opus cost about as much as the planning runs it judged over E4–E7 (150 M against 145 M tokens read from cache, at Opus prices)
- S2, a one-run-per-configuration screen of Sonnet and Opus at `xhigh` and `max`, is under way, serially

## Missing
- No cheaper judge validated against the Opus judges
- No per-experiment cost in currency; only token counts from transcripts

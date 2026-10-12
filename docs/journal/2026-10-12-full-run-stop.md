# 2026-10-12: The full run's stop

The author started a full run of exegenius at 7cc4d73 on the n log n paper (lmmx/openai-nlogn-wiki, branch `full-v0.6`): no `[pilot]`, `rounds = 3`, no simulated readers. This entry records its audit rounds as `exegenius status` reported them while the outline, overloads and support tasks ran, and two faults they show.

## The audit rounds

```
concepts: 54 (20 known to the reader); pages: 34 written of 34 planned
audit round 1: 75 gaps over 32 pages (per page: term 0.8, step 1.1, symbol 0.4)
audit round 2: 50 gaps over 34 pages (per page: term 0.4, step 0.7, symbol 0.4)
  on the 32 pages also audited in round 1: term 26 -> 12 (10 new), step 35 -> 22 (22 new), symbol 14 -> 9 (7 new)
steps inside reasoning a rewrite added for an earlier step, sent back to be cited or cut: 21
audit rounds stopped early: after audit round 2, step and symbol gaps on the 32 pages also audited in round 1 went from 49 to 31, not to half or fewer, so rewrites were not converging
```

Against the earlier full runs (2026-10-11-divergent-run):

| | 0.1 run | 0.5.0 run | this run |
|---|---|---|---|
| round 1 | 44 gaps over 24 pages (1.8 per page) | 293 over 32 (9.2) | 75 over 32 (2.3) |
| round 2 | 25 over 36 (0.7) | 698 over 82 (8.5) | 50 over 34 (1.5) |

The graph grew by two pages between rounds, against 12 in the 0.1 run and 50 in the 0.5.0 run. The background answer marked 20 of 54 concepts known, against 29 of 61 in the reader check pilot. Its summary counts roots of unity as known, and the Chinese remainder theorem, Newton iteration and Fourier and convolution machinery beyond the basics as not known.

## The stop rule counted steps sent back

21 of the round's step gaps were inside reasoning a rewrite added, and were sent back to be cited or cut. They counted towards the stop rule, as 2026-10-12-reader-check-pilot built it. Without them, step and symbol gaps on the repeated pages went from 49 to 10, and a third round would have run. Instead the rounds stopped, the rewrite notes were dropped as the last round's are, and the 21 steps stayed on the pages unfixed.

A step sent back to be cited or cut cannot renew, since its fix adds no claim, and only a further round fixes it. Counting it in the stop rule ends the rounds exactly when there are many to fix. The stop rule now leaves them out, and its message says how many it left out (exegenius-core/src/convergence.rs). They still count as gaps for the round, so a round that finds only them is followed by another.

## The orchestrating agent ended its turn with tasks pending

After its audit subagents finished, the run's orchestrating agent wrote "Still waiting on the others" and ended its turn, with no subagent running and the outline, overloads and support tasks pending. It resumed only when the author asked. It had counted subagents itself and not run `exegenius next`, which lists the pending tasks. The skill now tells it to keep no tally of its own: to run `exegenius next` each time a subagent finishes, start a subagent for any pending task none is working on, and end a turn only while subagents are running (exegenius-cli/src/skill.md). This is an instruction, not a check: `exegenius` cannot see which subagents are running.

## Current State
- The full run's audit rounds stopped after round 2, with step and symbol gaps on its 32 repeated pages at 49 -> 31, of which 21 steps were sent back to be cited or cut and then left on the pages
- The stop rule leaves out steps sent back to be cited or cut, and says how many it left out (exegenius-core/src/convergence.rs)
- The skill tells the orchestrating agent to run `exegenius next` each time a subagent finishes and to end a turn only while subagents are running (exegenius-cli/src/skill.md)

## Missing
- No run with the stop rule leaving out steps sent back to be cited or cut
- No check by `exegenius` that a turn ends with every pending task given to a subagent
- No review of the full run's final pages

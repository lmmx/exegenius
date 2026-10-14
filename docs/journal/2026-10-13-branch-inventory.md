# 2026-10-13: Branch inventory, both repositories

The author asked the development session to own the branch lifecycle in both repositories:
- merge completed and validated work;
- archive experimental evidence somewhere durable before a branch is deleted;
- delete branches whose work is merged or archived;
- keep branches that are active, isolate a needed experiment, or hold unreviewed work;
- keep this inventory accurate, and close each work session with every branch either dealt with or given a reason and a next action.

This entry is that inventory. It is updated in place, and its date is when it was started.

## Policy

- **lmmx/exegenius:** work is done on local dated branches and fast-forwarded into `master` once validated (`cargo fmt --check`, clippy with `-D warnings`, `cargo test --workspace`). A branch is pushed only when it must wait for review. History stays linear.
- **lmmx/openai-nlogn-wiki:** each run or experiment branch *is* the archive of its evidence: protocol, run directories, results and transcripts. These branches are kept. Merging them into `master` would mix runs. A branch is deleted only after its unique records are copied into the journal of lmmx/exegenius.
- **Deleting remote branches is blocked in this environment.** `git push origin --delete` was refused by the git proxy (HTTP 403), and the auto-mode classifier refuses it as destructive. The development session therefore deletes local branches only. It lists remote deletions for the author, and avoids pushing short-lived branches.

## lmmx/exegenius

| Branch | Head | Purpose | Status | Next action |
|---|---|---|---|---|
| `master` | see `git log` | the default branch | holds everything below that is merged | none |
| `wiki-explanation-reader-sim-v8dqtr` | 61e24ec | the branch the cloud session's harness designates | in `master` | keep while the harness names it |
| `timing-and-parallelism` | b2b9ba4 | pull request #8 | in `master` | **delete on GitHub** (blocked here) |
| `131026-scratch-dirs` | 02d5b76 | per-task scratch directories | in `master` | **delete on GitHub** |
| `131026-depth-rejection` | 21ebd02 | `coverage_depth = "results"` | in `master` | **delete on GitHub** |
| `131026-coverage-default` | c91907d | coverage check rejects by default | in `master` | **delete on GitHub** |
| `131026-process-safety` | 9774295 | deny rules against `pkill` and `killall` | in `master` | **delete on GitHub** |
| `131026-content-judge` | f4b1074 | the content judge | rebased into `master` as 385166b–8287b7b; every file identical | **delete on GitHub** |
| `131026-model-allocation` | 695eeb1 | the model-allocation entry | rebased into `master` as 1f07f0c and 0bccbbf; identical | **delete on GitHub** |
| `131026-handover` | f189028 | the handover entry update | rebased into `master` as b9a82e1 and 62604b9; identical | **delete on GitHub** |
| `131026-exp-key-step` | 0c29556 | the key-step instruction in the argument task, for experiments E4 and E6 | **not for `master`** until the author decides on E6's evidence | keep; merge or delete after the author's decision on E4 and E6 |
| `141026-exp-key-step-identity` | 91c4fed | the key-step instruction plus the identity sentence (E7), based on `131026-exp-key-step` | **not for `master`** until the author decides | merge or delete after the author's decision; supersedes `131026-exp-key-step` if merged |
| `docs/leviathan-chat-archive` | 1e0444a | the author's archive of research conversations | not in `master`; the author's own | the author's |
| `docs/metrics-viz-chat-archive` | b955d34 | the author's archive of research conversations | not in `master`; the author's own | the author's |

The local copies of every branch marked "delete on GitHub" are deleted.

## lmmx/openai-nlogn-wiki

| Branch | Head | Purpose | Status | Next action |
|---|---|---|---|---|
| `master` | e3d0322 | the default branch; the 0.1 run's project and records | evidence for the 0.1 baseline (2026-10-13-convergence-before-after) | keep |
| `tmp-v0.5-build-record-stash` | fa6c804 | the 0.5.0 run's records | evidence for 2026-10-11-divergent-run | keep; its name says temporary, but it is the only copy |
| `pilot-v0.6-convergence` | 4b921b6 | baseline pilot | evidence | keep |
| `pilot-v0.6-reader-check` | ea8751b | reader-check pilot | evidence | keep |
| `pilot-v0.6-depth` | 7839ae2 | depth pilot | evidence | keep |
| `pilot-v0.6-first-level` | 6698d4c | first-level pilot | evidence | keep |
| `pilot-v0.6-latex-source` | 42cd64b | LaTeX pilot | evidence | keep |
| `full-v0.6` | 901a730 | full run, 7cc4d73 | evidence | keep |
| `full-v0.6-latex` | e4aff3e | full run, 47f9294 | evidence | keep |
| `full-v0.6-latex-depth` | fa33c9d | full run, 930d40c | evidence | keep |
| `full-v0.6-latex-planner-handoff` | 72f1de8 | full run, the planner and handoff | evidence | keep |
| `131026-exp-coverage-recovery` | 52704ba | recovery experiment, A1, A2, B1, B2 | evidence, with transcripts | keep |
| `131026-exp-coverage-recovery-2` | 8023a47 | replication, A3–A6, B3–B6 | evidence, with transcripts | keep |
| `131026-exp-depth-rejection` | 0dd87da | depth experiment, C1–C5, D1–D5 | evidence, with transcripts | keep |
| `131026-exp-e1-config` | 85611ce | E1, the planner against a general-purpose subagent | evidence, with transcripts; the runner session is archived | keep |
| `131026-exp-e3-effort` | 5ced29e | E3, the planner at medium effort; also holds the first runner's refused attempt (`results-attempt-1.md`) | evidence, with transcripts; both runner sessions are archived | keep |
| `131026-exp-s1-models` | 7335096 | S1 screening of other models; attempt 1 was refused `add_repo`, and attempt 2 ran under the allow rule the author authorised for this branch only (3c70129) | evidence, with transcripts | keep |
| `131026-exp-t1-depth-rule` | 38a6cda | T1, the depth rule's dropped notes replayed three times, with the control | evidence, with writer transcripts | keep |
| `141026-exp-e4-key-step` | 8b5c679 | E4, the key-step instruction paired on E1's extractions | evidence, with transcripts (killed attempts apart) | keep |
| `141026-exp-e5-opus` | 904a3c6 | E5, five planning runs on Opus at medium | evidence, with transcripts | keep |
| `141026-exp-e6-key-step-confirm` | 834fb5a | E6, eight fresh projects split into baseline and key-step argument runs | evidence, with transcripts (killed attempts and their drafts apart) | keep |
| `141026-exp-e7-key-step-identity` | c89a3fd | E7, eight argument runs with the identity sentence on E6's extractions (based on E6's branch) | evidence, with transcripts | keep |
| `exp-coverage-recovery` | 2e35a54 | two refused attempts to run the recovery experiment | its unique records (the two reports and the protocol) are archived in `docs/journal/2026-10-13-coverage-recovery/refused-attempts/`; the rest is a vendored copy of exegenius source, which should not be kept | **delete on GitHub** |

## Current State
- `master` of lmmx/exegenius holds all completed work; eight remote branches of lmmx/exegenius are fully contained in it and await deletion on GitHub; `131026-exp-key-step` holds an experimental instruction kept off `master` pending the author's decision; two `docs/` branches are the author's own
- Twenty-two branches of lmmx/openai-nlogn-wiki are kept as the archives of their runs, through `141026-exp-e7-key-step-identity`
- `exp-coverage-recovery` of the wiki has its unique records archived in the journal and awaits deletion on GitHub

## Missing
- Remote branches cannot be deleted from this environment

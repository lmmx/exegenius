# 2026-10-11: Baseline pilot

This entry records the first pilot run (2026-10-11-convergence), with 0.5.0's audit and extend instructions on Haiku. It reproduces the divergence of the 0.5.0 run (2026-10-11-divergent-run) in 38 tasks and 34 minutes, where the full run took 855 tasks and six hours. The pilot is the development eval for the fixes that follow: each fix is measured by rerunning it under the same protocol.

## Protocol

The inputs are fixed, so runs under the protocol differ only in the instructions and the model's sampling:
- **Source and configuration.** The paper, target and reader profile are those of the 0.5.0 run (lmmx/openai-nlogn-wiki at fa6c804: `sources/paper.txt`, `exegenius.toml`), with `rounds = 2`, `[simulation] rounds = 0` and `[pilot] pages = 8`.
- **Extract.** The extract task is not rerun. The 0.5.0 run's answer, `exegenius/tasks/0001-extract.answer.json` at fa6c804, is submitted as task 0001. The task file was identical, so the starting graph matches that run's: 36 concepts, 4 known to the reader.
- **The pilot's concept.** `[pilot] pages = 8` chose `prime-source-lengths`, whose page and unknown prerequisites make 7 pages: exact integer multiplication, radix-digit convolution, discrete Fourier transform, Gaussian resampling, cyclic convolution, the Chinese-remainder map, and prime source lengths.
- **Tasks.** Every task ran on a fresh Haiku subagent, which wrote its answer and submitted until accepted. Audit subagents read only their task file.
- **Round 2.** Only the 7 pages from round 1 were audited, the pages whose rewrites the measures compare. The 7 pages extend added in round 1 were written but not audited.
- **Binary.** The instructions were 0.5.0's: the branch at 5550585 with the audit and extend instructions restored from master (61e24ec). Every other task's instructions are unchanged since 0.5.0.

Tasks run: 2 argument, 21 write, 14 audit and 1 extend, with the extract answer reused.

## Results

From `exegenius status`:

```
audit round 1: 46 gaps over 7 pages (per page: term 1.7, step 3.3, symbol 1.6)
audit round 2: 40 gaps over 7 pages (per page: term 0.9, step 3.0, symbol 1.9)
  on the 7 pages also audited in round 1: term 12 -> 6 (6 new), step 23 -> 21 (20 new), symbol 11 -> 13 (12 new)
```

Set against the two full runs:

| | 0.1 run, rounds 1 to 2 | 0.5.0 run, rounds 1 to 2 | baseline pilot |
|---|---|---|---|
| step gaps on repeated pages | 20 → 2 (2 new) | 131 → 171 (164 new) | 23 → 21 (20 new) |
| symbol gaps on repeated pages | 8 → 8 (8 new) | 71 → 111 (110 new) | 11 → 13 (12 new) |
| term gaps on repeated pages | 16 → 5 (5 new) | 91 → 46 (45 new) | 12 → 6 (6 new) |
| pages after the first extend | 24 audited, then 36 | 32 audited, then 82 | 7 planned, then 14 |

The pilot shows the same pattern as the 0.5.0 run:
- **Step gaps renew.** Step gaps barely fell, and 20 of the 21 in round 2 were new: the rewrites replaced the reported steps with others.
- **Symbol gaps rose.**
- **Term gaps halved.** That is the one kind that converges.
- **The graph doubled.** The first extend turned 7 planned pages into 14. Its new pages include prime number, modular arithmetic and pairwise coprime, which an undergraduate computer scientist knows, and `source-paper`, a page about the paper itself, made because audits reported "the paper" as an unknown term. Five more candidate concepts were waiting after round 2.

**Page length.** With citation marks removed, the 7 pages went from 3,905 words as first drafts to 4,389 after one rewrite (+12%). Only cyclic convolution got shorter (353 to 338). The first drafts' median was 621 words, against 221 for the pages of the 0.1 run.

**Broken sentences.** Step gaps whose note describes a sentence that breaks off or is incomplete went from 4 of 23 to 8 of 21.

## Subagents using tools they were not given

Three of the 38 Haiku subagents called `create_session`, a cloud-session tool the task did not need, and each created an empty session. Two others made other stray calls (a documentation lookup, and failed archive and document calls). None touched the project. The prompts named Read, Write and Bash as the only tools to use, and three of these calls came after that line was added.

The empty sessions were checked and archived. An instruction in the prompt does not keep a subagent off tools it can see. A subagent definition whose `tools` lists only what the task needs would, which is change 2 of 2026-10-11-wall-time.

## What a fix must show

Against the 0.1 run, a fix should bring the pilot to:
- step and symbol gaps on repeated pages falling by at least half;
- new gaps a minority of those remaining;
- the first extend adding well under one page per page audited.

The baseline meets none of these.

## Current State
- A pilot of 8 pages on the n log n paper, with 0.5.0's instructions on Haiku, ran 38 tasks in 34 minutes and reported step gaps on repeated pages going 23 to 21 with 20 new, symbol gaps 11 to 13 with 12 new, and term gaps 12 to 6
- The pilot's first extend raised its planned pages from 7 to 14, among them pages on prime numbers, modular arithmetic and the source paper itself
- The pilot's rewrites lengthened its 7 pages from 3,905 to 4,389 words, citation marks removed
- Three of the pilot's 38 Haiku subagents created empty cloud sessions with a tool the task did not need, despite prompts limiting them to Read, Write and Bash

## Missing
- No pilot run with any of the fixes in 2026-10-11-convergence
- No repeat of the baseline, so the spread between runs under the same protocol is unknown

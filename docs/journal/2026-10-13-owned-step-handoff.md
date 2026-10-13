# 2026-10-13: Owned sub-steps in the write task, and a replication

2026-10-13-brief-pilot found:
- **The butterfly page:** a brief built from the plan closed the central gap. Its owned step, S3.3, has a method and three inferred steps beneath it, and the write task passed on only S3.3's claim.
- **`rank-saving`:** the brief added nothing, since its owned steps are single inferences whose formulas already reach the writer.

ChatGPT and the author chose the narrow change the pilot supports, followed by a replication before any full run:
- pass on only what the page owns;
- mark what other pages own;
- have the writer declare where it derives each owned step;
- check that declaration in submit without matching equations;
- leave the depth policy, the word limit, the audit and its model alone;
- keep the reuse of subagents for repairs as a separate, later experiment.

## The change (ec620eb)

**`Argument::owned`.** A page's owned steps are the sub-steps of the step the page shows that no other page presents. A page that presents an inferred step, or no step, owns none.

**The write task.** For a page that owns steps, `write.rs` adds a section, "Steps this page derives", after "This page in the argument". For each owned step it gives:
- its method;
- the steps beneath it, each with:
  - the plan's quote;
  - the paragraph around the quote, cut to 1,500 bytes as for extraction evidence.

Then two lists:
- **Steps derived on other pages:** the steps under the page's step, or under an owned step, that another page presents. The writer is to state each one's result where the page uses it and link the page, "even where a passage above proves it", which answers the pilot's duplication.
- **Other pages these steps use:** the planned pages in the owned steps' `from` that are not prerequisites.

It asks for each owned step to be derived, and says a citation does not derive it.

**What the brief carried that this does not:** the pilot's paragraphs were uncut, and its dependency list also took the page's own step's `from`.

**Derivation anchors.** `WriteAnswer` gains `derivations`: for each owned step, the step's id and the passage on the page that derives it, copied exactly. Submit rejects an answer when:
- an owned step has no passage;
- a step is unknown or given twice;
- a passage is not found on the page with citations removed.

Submit does not judge whether the passage derives the step; that is the audit's to judge. The audit task is unchanged and does not see the anchors.

**Tests:**
- `a_page_owns_the_sub_steps_no_other_page_presents`, in `argument.rs`;
- `each_owned_step_needs_a_derivation_on_the_page`, in `write.rs`;
- in `task_loop_end_to_end`:
  - the chirp page's task lists S1 and S2 with their quote and paragraph;
  - a write without derivations is rejected, naming both;
  - a passage not on the page is rejected.

The test file's `submit` fills in derivations for tests about other checks. All 78 tests pass, and clippy is clean.

## The replication

**The pages.** The first Sonnet sample gives only the butterfly page an owned step with steps beneath it. Its other pages own only single inferences, like `rank-saving`. The second sample, also frozen, has two more pages that do:
- **`address-chunk-interchange`** owns S1.6:
  - the method: interchange chunks of any width, by supplying rows from the chunks' high digits, splitting the width in base $m$ into power-width pieces, and padding binary chunks;
  - three inferred steps beneath it;
  - S1.3 to S1.5, which include the power-width recursion and $s < Wm$, are other pages'.
- **`time-accounting`** owns:
  - S6.A, S6.B and S6.D, each a single inference;
  - S6.C, with five inferred steps that justify each normalised cost row from an earlier procedure.

**The conditions:**
- **A, the current task:** the write task as exegenius generated it before the change, from a539829, built separately. A writer's answer is checked by that binary.
- **B, the new task:** the write task from ec620eb, checked by it, anchors included.

**Held fixed:**
- **Inputs:** `paper.tex`, the frozen plan, and the 350-word limit.
- **Writers:** a fresh Haiku subagent per attempt, two per page and condition, each in its own copy of the project.
- **Audits:** the same audit, built by `audit_task` and checked with `apply_audit` through the pilot's harness, by a fresh Haiku subagent per page.
- **The prompt:** as in the pilot. Writers are also told to use only Read, Write and Bash, since a general-purpose writer in the pilot created two remote sessions.

## Scoring, set before the pages exist

**`address-chunk-interchange`.** Three items:
1. **The rows:** the top $\rho = O(\log e)$ digits are interchanged pair by pair and joined into a row field padded to a multiple of $W^k$, which the power-width procedure needs.
2. **The base-$m$ split:**
   - the remaining width is written as $\sum_j a_jm^j$;
   - each piece uses the power-width interchange;
   - the costs $\sum_j a_jV(m^j)^\tau$ sum to $O(Ve^\tau)$ because $a_j < m$ and the series in $m^{j\tau}$ is geometric.
3. **Binary chunks:** they are padded to radix $q$, which enlarges the volume by a fixed factor, giving $O(Vu^\tau)$.

The page is also checked for the opposite failure: deriving the power-width recursion or $s < Wm$, which are other pages'.

**`time-accounting`.** Four items:
1. **S6.A:** each stage's cost divided by $V = Tp$ is a power of $p$.
2. **S6.C:** the rows are traced to the procedures they come from (the synthetic transform's three rows, axis moves, resampling, chirps and products, setup and exceptional streams).
3. **S6.B:** the margins $g_i$ and $\min_i g_i = 2^{-181} = 2\kappa$.
4. **S6.D:** each power is at most $1 - 2\kappa$, and polylogarithmic factors are at most $p^\kappa$, so the total is $O(n(\log n)^{1-\kappa})$.

**On both pages:**
- correctness against `paper.tex`;
- duplication of steps other pages own;
- words against the limit;
- the audit's findings;
- rejections at submit;
- tokens and time;
- what the writer flags;
- in B, whether each anchored passage derives its step or only states it.

## How the replication ran

**The records.** Everything is in `2026-10-13-owned-step-handoff/`:
- `tasks/`: the four write tasks;
- `pages/`: the eight pages, with their JSON answers;
- `audits/`: the eight audit tasks and answers;
- `log.txt`: each writer's reply and cost, and my reading.

**Submission.** All eight writers' answers were accepted by the submit of their condition.
- One rejection was reported: `time-B-2`'s first draft, at 377 words.
- No B writer reported a rejection for its derivations.

**Audits.** Seven of the eight first audit answers were rejected by `apply_audit`. Each was rejected for anchoring a symbol to a known concept that does not bind it, such as `s`, `W` and `m` to "Rank saving $s<Wm$", or `\lfloor` to "Floor and ceiling". `time-A-1` needed three answers. Across the pilot and this replication, that makes 12 first answers in 16.

**No side effects.** Writers told to use only Read, Write and Bash made none.

## Results

**`address-chunk-interchange`**, scored against the items set beforehand:

| Page | 1: rows from high digits, padded to $W^k$ | 2: base-$m$ split and its cost | 3: binary padding | Other pages' results | Words | Audit: steps, terms |
|---|---|---|---|---|---|---|
| A-1 | stated in a sentence | stated, no cost | stated | sketches the power-width recursion and cites $s<Wm$ | 341 | 7, 0 |
| A-2 | no | no | no | restates the XOR swap; sketches $s<Wm$ giving the saving | 309 | 4, 2 |
| B-1 | derived: $\rho$, the field orders, padding below $2q^{2\rho}$ | derived; the cost is $O(Ve^\tau)$ "because $a_j<m$ and $m^k<me$" | derived, with $e = O(u)$ | the power-width result stated and linked | 339 | 6, 2 |
| B-2 | derived | derived; the top term $m^{k\tau}\le e^\tau$ "dominates the geometric sum" | derived | the power-width result stated as used, and linked | 339 | 4, 1 |

- **The pattern of the butterfly page recurs.** Neither A page derives S1.6. Both B pages derive all three of its steps, in agreement with the paper.
- **The audits split the same way.**
  - On A-1 they flagged the missing steps: how the padding works, how the pieces pair up, how many levels the recursion has.
  - On A-2 they flagged a construction that is asserted, and a cited saving that "stands in for how the method's saving arises".
  - On the B pages they flagged details inside the derivation: what a row field is, $\rho \le e$, $m > 1$ for the geometric sum, and the cost sum left to the reader on B-1.
- **The A pages spent their words on other pages' material.** The B pages did not. This answers the pilot's duplication within this page.
- **A real flaw.** B-1's "$e=m^k$ digits, where $k$ is the least integer with $m^k\ge e$" muddles $e$, and its audit flagged it.

**`time-accounting`.** Items 1, 3 and 4 (S6.A, S6.B, S6.D, the single inferences) are on all four pages, in agreement with the paper. The difference is in S6.C, the rows traced to the procedures that bound them:

| Page | S6.C: rows traced | Words | Audit: steps, terms |
|---|---|---|---|
| A-1 | none; three rows attributed to linked pages | 347 | 5, 1 |
| A-2 | one: the chunk-exchange row, from $O(p/K)$ exchanges at $O(VK^\tau)$ | 316 | 6, 3 |
| B-1 | five: chunk exchanges, simultaneous rounds, axis moves, Gaussian lines, packed products; each power derived from $d$, $K$, $\ell = \Theta(p^{1-\epsilon})$ and $\alpha$ | 346 | 5, 1 |
| B-2 | none; three rows attributed | 333 | 2, 1 |

- **Mixed: one B page in two.**
- **The budget limited S6.C.** Both B writers said they could not fit it: B-1 omitted the setup row, and B-2's first draft was rejected at 377 words. Five row justifications plus the margin table do not fit in 350 words.
- **The single inferences reached every page in both conditions**, as on `rank-saving` in the pilot.

**The anchors.**
- On B-1 and on both interchange pages, each anchored passage holds its step's derivation.
- B-2's anchor for S6.C is "Each row is an earlier cost bound, divided by $V$.", a statement, not a derivation, and submit accepted it as designed. The audit, which does not see the anchors, flagged exactly that sentence: "Only the chunk-exchange, simultaneous-layer and Gaussian rows are linked to the page they come from".
- Two writers noted that anchors are asked only for the page's owned steps (S1.6), not for the inferred steps listed beneath them, so one passage covered all three.

**What the writers flagged.**
- **All A writers, and the B writers, about the task:**
  - prerequisite ids, which the task shows only inside link targets (two writers);
  - a page that is both a prerequisite and already "understood";
  - $W$ described as gates rather than wires;
  - the exponents' values, which are absent from the task;
  - `m`'s meaning.
- **About the new section:** the budget (both `time-B` writers) and the anchor granularity (both `interchange-B` writers).

**Cost.** Each writer's and auditor's tokens and time, as Claude Code reported them:

| | interchange A | interchange B | time A | time B |
|---|---|---|---|---|
| writer tokens | 130.9k, 97.7k | 185.0k, 203.7k | 154.2k, 141.2k | 217.6k, 171.9k |
| writer time | 271 s, 169 s | 477 s, 510 s | 328 s, 296 s | 539 s, 420 s |
| audit tokens | 82.6k, 84.4k | 100.1k, 106.9k | 114.8k, 114.2k | 100.0k, 88.6k |

- **Writers:** the B writers used 70% more tokens and 124% more time on the interchange page, and 32% and 54% more on `time-accounting`. The new section adds 1,163 and 1,431 words to the tasks.
- **Audits:** costs were similar between conditions.

## Reading the results

Across the three pages whose owned step has inferred steps beneath it:

| Page | A pages deriving the owned multi-step step | B pages deriving it |
|---|---|---|
| butterfly (pilot, the brief) | 0 of 2 | 2 of 2 |
| `address-chunk-interchange` (this change) | 0 of 2 | 2 of 2 |
| `time-accounting`, S6.C (this change) | 0 of 2 (one traces one row) | 1 of 2 |

- **Where the owned steps are single inferences**, the conditions did not differ: `rank-saving`, and S6.A, S6.B and S6.D.
- **The pattern recurs:** passing on what the plan holds beneath a page's own step is followed by that step's derivation on the page.
- **Duplication:** on the interchange page it fell, since the B pages stated and linked what other pages own.
- **The budget is the limit where an owned step is wide.** S6.C has five justifications, and one B page in two fitted them.

**Limitations:**
- Two attempts per condition and two pages, so 5 of 6 against 0 of 6 is a consistent direction, not an estimate.
- The scoring was mine and not blind.
- The replication's pages come from the second Sonnet sample; the pilot's came from the first.
- B writers used more tokens and time.
- An anchor shows where a writer claims to derive a step, not that it does.

## Current State
- `exegenius next` gives a write task for a page that owns sub-steps a section, "Steps this page derives", with each owned step's method, the steps beneath it with their quotes and paragraphs, the steps beneath it that other pages present, and the pages those steps use
- `exegenius submit` rejects a write answer whose `derivations` lack a passage on the page, outside citations, for any owned step, or name a step the page does not own
- On `address-chunk-interchange` (second Sonnet sample), both pages written with the section derive S1.6 in agreement with the paper, and neither page written without it does
- On `time-accounting`, one of two pages written with the section traces five of the cost rows to their procedures; the other, and both pages written without it, trace at most one
- A derivation anchor for S6.C on one page is a statement, not a derivation; submit accepts it, and the audit flags that sentence
- Seven of eight first Haiku audit answers in the replication were rejected for anchoring symbols to known concepts that do not bind them

## Missing
- No audit input naming a page's owned steps and their anchors
- No anchors for the inferred steps beneath an owned step
- No word budget that depends on what a page owns
- No full run with the planner on Sonnet and this handoff

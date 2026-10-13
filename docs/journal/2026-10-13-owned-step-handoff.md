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

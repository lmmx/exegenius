# 2026-10-13: Does the brief survive the handoff? A pilot

2026-10-13-planner-handoff found that a write task passes on less than the plan holds:
- **What it passes:** the claim, method and motivation of the page's own step, and one-line claims of its sub-steps.
- **What it drops:**
  - the sub-steps' methods;
  - the inferred steps under them;
  - their quotes from the paper;
  - the step's dependencies;
  - any criteria for the finished page.

ChatGPT read this as a likely failure mechanism: Sonnet produced the reasoning, and the task dropped it before Haiku saw it. It proposed changing the order of experiments, and the author agreed. This pilot comes before the Sonnet-planner run. It tests whether passing that reasoning on makes Haiku's pages better. This entry records the design before any page is written.

## Design

**Held fixed:**
- the paper, `paper.tex`;
- the frozen first Sonnet planning sample from 2026-10-13-model-comparison: its graph, background and argument, unaltered;
- the writer: a fresh Haiku subagent per attempt;
- the word limit, and submit's checks;
- the audit: a fresh Haiku subagent per page, with the standard audit instructions and the task built by exegenius's own audit code;
- the scoring.

**Two pages:**
- `simultaneous-butterfly-layer`, a recursive mechanism (S3);
- `rank-saving`, a concrete inequality, $s_{\rm b} < W_{\rm b}m$ (S1).

**Two conditions:**
- **A, the current task:** the write task `exegenius next` generates.
- **B, the brief:** the same task, with one added section, built mechanically from the page's subtree of `argument.json`:
  - **Steps this page derives:** each sub-step with no page of its own. For each, its claim, its method, and its inferred steps, each with the plan's quote and the paper's paragraph around it.
  - **Steps proved on other pages:** each with its claim and link, to be used and not derived again.
  - **What the step relies on:** the `from` concepts of the step and of its derived sub-steps that are not already prerequisites, with their links.
  - **Acceptance criteria**, derived from ownership:
    - each owned step is derived, not only stated or cited;
    - a step owned by another page gets its result in a sentence and a link;
    - the quotes ground the derivation and are not to be copied onto the page.

**Two attempts per page and condition**, so 8 writers and 8 audits. Each attempt runs in its own copy of the project, so that submit's checks run unchanged. The writer's prompt is the one `exegenius next` prints, with the binary's full path, and in both conditions it ends:

> After the submit result, on a new line, say anything in the task that was missing, unclear or in conflict with the sources, or "none".

That records whether a writer flags an inadequate brief, and asks the same of A and B.

## Scoring, set before the pages exist

**Butterfly layer.** The owned derivation is S3.3. It is sufficient if a reader can see each of these:
1. rows are split among the $W$ roles, so each child call works on volume $V/W$;
2. the $s$ child calls on $f = e/m$ axes give $F(e) \le (s/W)F(e/m) + O((eK)^\tau+1)$;
3. because $s/W$ is below a power of $m$ less than one ($s/W \le m^\sigma$, from $s < Wm$), the levels' costs sum to a power of $e$ below one;
4. the base-$m$ partition of the chunks gives $O(d^{\lambda'})$ with $\lambda' < 1$.

Item 3 is the reason the exponent falls below 1. It is the crux.

**Rank saving.** The owned steps are:
- **S1.4:** $s_{\rm b} = W_{\rm b}m - N + 2L_{\rm b} < W_{\rm b}m$. It is sufficient if the page says where each term comes from:
  - edge ranks equal label-dimension changes;
  - the signed sum is $Wm - 2N$;
  - the decreasing edges add $2L$;
  - the $N$ source edges add $N$;
  - and $L < N/2$ gives the deficit $N - 2L > 0$.
- **S1.6:** the deficit gives $s/W < m^\tau$ with $\tau = 1 - 2^{-50}$.

**On every page:**
- **Correctness:** each claim against `paper.tex`.
- **Duplication:** a step owned by another page, derived here.
- **Unnecessary derivation:** a result only used, derived here.
- **Length:** words, against the budget.
- **The audit's findings:** what they are, not only how many.
- **Mechanical checks of the brief's criteria:**
  - an owned step's quote cited on the page in place of its derivation;
  - an owned step's mathematics absent;
  - the mathematics of a step owned by another page present.
- **Cost:** rejections at submit, tokens and time per writer.

**How the results are read** (ChatGPT's table):

| Result | Reading |
|---|---|
| B closes the central gap on both pages | implement the handoff before the full run |
| B helps on one | the two kinds of step may need different guidance |
| No real difference | Haiku's execution, a brief still missing something, or an audit not measuring this |
| Better pages, but longer or duplicated | fix ownership and criteria before scaling |

Two attempts per condition give qualitative evidence, not an estimate of reliability.

**One gap in the brief, known in advance.** S1.4's quote, "Lemma~\ref{lem:motif-residuals} consequently gives the stated rank sum", is the conclusion. Its paragraph refers to the lemma by label and does not contain the signed sum. The brief passes on what the plan holds, so a writer must still find the lemma in `paper.tex`. If B fails on `rank-saving`, this is a likely cause.

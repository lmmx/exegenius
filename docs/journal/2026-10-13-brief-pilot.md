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

## How it ran

**The records.** Everything the pilot used and produced is in `2026-10-13-brief-pilot/`, with scratch paths replaced by `<project>`, `<pilot>` and `<audits>`:
- `tasks/`: the four write tasks;
- `pages/`: the eight pages, with their JSON answers;
- `audits/`: the eight audit tasks and answers;
- `scripts/`:
  - `make_brief.py`, which built B;
  - `check.py`, the mechanical checks;
  - `audit-harness.rs`, which built each audit task with exegenius's own `audit_task` and checked each answer with `apply_audit`, since `next` issues audits only once every page is written;
- `log.txt`: each writer's reply, cost, and my reading.

**The tasks.** The two attempts in each condition had identical tasks, apart from paths. B added 1,196 words to the butterfly task and 736 to `rank-saving`'s.

**Submission.** Every page was accepted by submit, at 335 to 346 words against a limit of 350. Rejections on the way, as the writers reported them:
- `rank-A-1`: one rejection (symbols used before they were bound);
- `bfly-B-1`: the letter check took LaTeX macros (`\gamma`, `pmatrix`) for letters, which it removed.

**Audits.** Five of the eight first audit answers were rejected by `apply_audit`, each for anchoring notation to a known concept that does not bind it (`O` to "Big-O notation", `\log`, `\otimes`). Each auditor was sent the rejection once, and each second answer was accepted.

**Writer subagents.** These were general-purpose Haiku subagents, not the `exegenius-task` definition. One of them called `create_session` twice, making two empty remote sessions, which were archived; another made a stray `send_message`, which failed. The `exegenius-task` definition's three tools would have prevented both.

## Results

**The butterfly layer.** Scored against the four items set beforehand:

| Page | 1: rows to roles, children on $V/W$ | 2: the recurrence | 3: why the exponent falls below 1 | 4: the base-$m$ sum to $O(d^{\lambda'})$ | Audit: steps, terms |
|---|---|---|---|---|---|
| A-1 | yes | no | in one sentence, loosely ("multiplies the volume by $s/W<m$") | no | 8, 3 |
| A-2 | yes | yes | yes: "$\log_m(s/W)$ … below 1 since $s<Wm$" | no; the leaves are left out | 5, 0 |
| B-1 | yes, with $u = Wg + w$ | yes, with the packed-addition cost as its overhead | yes: $a < m$, linked to `recursive-power-saving`, then $a < m^\mu$ | yes, with the leaves and the row preprocessing | 6, 3 |
| B-2 | yes | yes | in part: assumes $s/W \le m^\sigma$ with $\sigma < \lambda$ and links `recursive-power-saving`, but does not say that $s < Wm$ is the reason | yes, with the leaves | 4, 2 |

- **What A lacks:** neither A page unrolls the recurrence or sums it.
  - A-1 states the result of the unrolling in a sentence.
  - A-2 has the recurrence, and its claim that "each level is a fixed fraction of the one above, so the top level dominates", but no sum.
- **What B adds:** both B pages give the unrolling with the cut-off at $\sqrt d$, the geometric sum $O(e^\lambda)$, the leaf bound $O(e^\sigma d^{(1-\sigma)/2})$, the base-$m$ partition, and the range of $\lambda'$. Each step agrees with the paper's paragraphs (§5, "Unrolling the recurrence" and the paragraph after it).
- **What the audits found:**
  - **On the A pages, the central mechanism.** A-1: "The page gives no cost accounting: work per level equal to volume, depth $\log_m$ of the group size, a geometric sum over levels". A-2: "The reader cannot see the ratio between consecutive levels or why the sum over levels is dominated by the top", and the layer cost "not tied" to $F$.
  - **On the B pages, details inside the derivation:** that $J$ must be the least such depth, what a leaf's weight is, how many base-$m$ blocks there are, and where $K \le e^{2c}$ holds. This is the pattern of 2026-10-12-responsibility-prototype: once the derivation is on the page, the audit moves to the finer level.
- **What no page does:** none links `rank-saving`, where $s < Wm$ is derived. The plan's dependencies for S3 name `recursive-power-saving`, not `rank-saving`, so the brief did not list it.
- **Correctness:**
  - no claim found on any of the four pages disagrees with the paper;
  - two are loose:
    - A-1's "multiplies the volume", where the work is multiplied;
    - B-1's $q_0 = O(\log d)$, where the paper's $q_0 = \lceil\log_2 W\rceil$ is a constant.
- **An auditor error:** B-1's audit objected to "Values stay in the unit disk, since $\sqrt2\,H_0$ is unitary". The reason holds: unitarity gives $|(u+v)/2|^2 + |(u-v)/2|^2 = (|u|^2+|v|^2)/2 \le 1$ for inputs in the disk, so the objection is mistaken.

**Rank saving.** All four pages derive S1.4, $s_{\rm b} = W_{\rm b}m - N + 2L_{\rm b}$, in agreement with the paper: the telescoping to $Wm - 2N$, $2L$ from the decreasing edges, $N$ from the rank-one source edges, and $L < N/2$.

| Page | Edge rank equals label-dimension change | Further derivation | Explicit $\tau$ | Audit: steps, terms |
|---|---|---|---|---|
| A-1 | stated, linked to `subspace-labels` | — | yes | 9, 5 |
| A-2 | stated, linked to `subspace-labels` | $L/N = 3ch/v = 100/539$, from $N = v^3$, $3v^2$ invocations and $c = h$ (paper, §4) | no; existence only | 9, 4 |
| B-1 | derived: $P_V - P_U$ has rank $\dim V - \dim U$ | — | yes | 4, 1 |
| B-2 | derived, as in B-1 | $L_{\rm b} = 3v^2hc_{\rm b}$, and $W_{\rm b}m \approx 4.19\cdot10^{10}N$, which checks against $\eta_{\rm b}$ | yes | 8, 1 |

- **No difference between the conditions on the owned step.** The plan's one-line claim of S1.4, with S1's method "sum the dimension changes along the edges", was enough for Haiku to find the derivation in `paper.tex` without the brief.
- **The gap known in advance did not matter.** Every writer found the lemma behind S1.4's quote.
- **What B changed: one more level, which is duplication.**
  - The B pages derive the rank of $P_V - P_U$ from the paragraph the brief quoted.
  - That fact is S1.3's, which the plan gives to `subspace-labels`.
  - The A pages state it and link that page, as the ownership criteria ask.
- **The audits.**
  - Gaps: 14 and 13 on the A pages, 5 and 9 on the B pages.
  - B-2's audit misread the order of the page's argument, which adds the source edges' $N$ before the telescoping and assembles the formula after it.

**Length.** Every page is within 15 words of the 350-word limit, in both conditions. The brief made pages denser, not longer. Two of the four B writers said the derivation the brief asked for did not fit:
- `bfly-B-1` compressed the row padding and the packed repair to a clause each;
- `rank-B-1` cited the complex network's deficit in place of deriving it.

**Duplication on every butterfly page.** All four restate $H_0^{\otimes D} = b^D\mathcal S_DC^{\otimes D}\mathcal S_D$, which the plan gives to `phase-edge-recursion` (S3.1.4). The task carries it in both conditions, in S3's method and S3.1's claim.

**The mechanical checks did not work as built.** `check.py` looked for each owned step's mathematics on the page by normalised substring. It found S1.4's formula on none of the four rank pages, though all four derive it, because:
- subscripts are dropped ($s = Wm - N + 2L$);
- variables are renamed ($a$, $\mu$, $\theta$, $h$ for $u$);
- $\frac{s}{W}$ is written as $s/W$.

No page cited an owned step's quote in place of a derivation. A requirement that code can check needs an anchor the writer declares, as relations already work: for each owned step, the sentence on the page that derives it. Submit can check that the sentence is on the page, and the audit can judge whether it is enough.

**What the writers flagged.** All eight replied with something.
- **Defects in the plan or the task, all from the A writers, all real:**
  - the plan's striking feature, a deficit "of order $10^{-11}$", holds only for the bit network ($3.7\cdot10^{-12}$ for the complex one), flagged by both `rank-A` writers;
  - $b$ is unbound in S3.1's identity;
  - the known concept "Hadamard matrix" is unnormalised, against the paper's normalised $H_0$;
  - prerequisite pages appear among the concepts the reader "already understands". That is by design: earlier pages count as read. But the wording misled the writer.
  - whether relation ids take `.md`, also flagged by `bfly-B-2`.
- **About the brief, from the B writers:**
  - it does not fit the budget (two writers);
  - the paper does not tie the overhead $O((eK)^\tau + 1)$ to a packed operation with $L = eK$ (`bfly-B-2`; `bfly-B-1` gives $L = fK$);
  - the paper's names $W_{\rm c}, s_{\rm c}$ against the brief's $W, s$.
- **No writer stopped on its brief.**

**Cost.** Each writer's tokens and time as Claude Code reported them:

| | butterfly A | butterfly B | rank A | rank B |
|---|---|---|---|---|
| writer tokens | 199.8k, 211.7k | 265.6k, 193.4k | 156.4k, 168.9k | 190.8k, 254.2k |
| writer time | 479 s, 561 s | 735 s, 452 s | 341 s, 399 s | 481 s, 711 s |
| audit tokens | 98.1k, 103.0k | 112.0k, 114.2k | 88.0k, 83.3k | 95.8k, 117.8k |

On average, B writers took 12% more tokens and 14% more time on the butterfly page, and 37% and 61% more on `rank-saving`. The spread within each condition is as large as the difference.

## Reading the results

By the table set beforehand, the result is the second row: **the brief helps on one page.**
- **On `simultaneous-butterfly-layer`, B closed the central gap twice.** Both B pages derive the recursion's cost. Neither A page sums it, and the A pages' audits found exactly that.
- **On `rank-saving`, the condition made no difference to the owned step.**

**The difference lies in the shape of the owned step.**
- **`rank-saving`:** S1.4 is a single inferred step. Its claim, in the formula, reaches the writer in the current task, so the writer knows what to derive and finds how in the paper.
- **The butterfly page:** S3.3 is a step to show, with a method and three inferred steps beneath it. The current task gives only its claim, so the writer must reconstruct the derivation, and both A writers stopped at the recurrence or before it.

So the handoff loses what matters where a page owns a step with sub-steps of its own. That is the information `argument_section` drops.

**Two costs of the brief:**
- **Budget pressure:** two of four B writers could not fit everything.
- **Duplication** of a neighbour's fact, from a quoted paragraph that covers more than the owned step.

The second is the "better but duplicated" row in part. It argues for passing on less than whole paragraphs, or for marking within the brief which facts belong to other pages.

**Limitations:**
- Two attempts per condition, one plan sample, two pages.
- My scoring was not blind to the condition.
- The audits ran with no earlier pages written, so no symbols were bound on earlier pages. That applies to both conditions alike.
- The writers were general-purpose subagents.
- The word limit bound every page, so the brief's effect on length could not show.

## Current State
- On `simultaneous-butterfly-layer`, both pages written from the current task stop before summing the recursion, and both pages written with the brief derive the recursion and its sum to $O(d^{\lambda'})$, in agreement with the paper
- On `rank-saving`, all four pages derive $s_{\rm b} = W_{\rm b}m - N + 2L_{\rm b}$ in agreement with the paper, with or without the brief
- The brief's quoted paragraphs led both `rank-saving` writers to derive a fact the plan gives to `subspace-labels`
- All four butterfly pages restate S3.1's identity for $H_0^{\otimes D}$, which the plan gives to `phase-edge-recursion`
- Every page in the pilot is within 15 words of its 350-word limit
- A substring check for owned steps' mathematics found S1.4's formula on none of the four `rank-saving` pages, though all four derive it
- Five of eight Haiku audit answers were first rejected for anchoring notation to known concepts that do not bind it

## Missing
- No handoff in exegenius of the methods, inferred steps and quotes beneath a page's own sub-steps
- No anchor by which a writer declares where each owned step is derived
- No marking, within quoted evidence, of facts owned by other pages
- No pilot of the brief on a page whose owned steps lack sub-steps beyond `rank-saving`, or with more than two attempts

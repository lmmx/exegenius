# 2026-10-13: The planner and handoff run, a failure analysis

The author ran the setup of 2026-10-13-planner-handoff-run-setup and pushed it to lmmx/openai-nlogn-wiki as `full-v0.6-latex-planner-handoff` (72f1de8).
- **The run:** 212 tasks, all accepted, in 67 minutes.
- **The result:** 25 pages in 6 parts.

The status looked worse than the baseline's: more step gaps per page, and no fall across rounds. ChatGPT asked for a failure analysis before any further run or any change. This entry is that analysis, from the run's records and the paper.

## The planning read only the introduction

**What the planning tasks did.** Each read only the start of `paper.tex` (5,361 lines, 253 KB, about 70k tokens):

| Planning output | This run (`exegenius-planner`, Sonnet) | Baseline (`exegenius-task`, Haiku) | Comparison sample 1 (general-purpose, Sonnet) | Comparison sample 2 |
|---|---|---|---|---|
| extract task tokens | 26.9k in 39 s | not recorded | 199k in 4.2 min | 178k in 3.4 min |
| concepts extracted | 28 | 31 | 49 | 53 |
| extraction quotes by tenth of the paper | 27, 0, 0, 0, 0, 0, 0, 0, 0, 0 | 24, 0, 0, 0, 0, 0, 0, 2, 1, 0 | 37, 8, 3, 6, 1, 4, 7, 8, 7, 1 | 29, 5, 11, 6, 0, 9, 10, 11, 5, 2 |
| argument task tokens | 36.4k in 86 s | not recorded | 264k in 8.9 min | 271k in 10.3 min |
| argument steps | 19 | 17 | 83 | 79 |
| argument quotes by tenth of the paper | 16, 0, …, 0 | 14, 0, 1, 0, 0, 0, 1, 0, 0, 0 | 1, 4, 8, 12, 7, 7, 7, 9, 11, 2 | 1, 4, 10, 11, 6, 6, 6, 9, 12, 7 |

The first tenth of the paper ends at line 464. The introduction runs from line 37 to line 431.

**In both full runs, the graph and the argument were built from the introduction.** That holds for Haiku in the baseline and for Sonnet in this run. The comparison samples, which read the whole paper, were run as general-purpose subagents with Sonnet.

**What the run was planned from:**
- The argument has 19 steps.
- 16 pages present one, nearly all as a single inferred step. `fast-butterfly-layers` presents S1.3, "A simultaneous layer … costs …", an inferred step with nothing beneath it.
- Only three pages own a sub-step:
  - `main-theorem` owns S1;
  - `fast-address-interchange` owns S1.2.2, "Two further linear-cost address updates complete the interchange";
  - `ring-convolution-retained-order` owns S7.2.
- So the owned-step handoff reached 13 of 190 write tasks, on those three pages.
- **The evidence passages** writers were given are the paragraphs around the extraction's quotes, all from the introduction.

**A correction to 2026-10-13-model-comparison.** That comparison changed two things at once:
- the model: Sonnet against Haiku;
- how the planner ran: a general-purpose subagent with Claude Code's full tools and system prompt, against the Haiku run's `exegenius-task` definition, which has Read, Write and Bash.

I set it up that way and did not notice. Its conclusion, that Sonnet plans better, does not follow: the difference between the plans may come from reading the whole paper. This run, with Sonnet under the restricted definition, planned like Haiku.

**How the reading stopped is not yet known.** The planner's own transcript shows it, and the records here do not. Two candidates:
- a read of the 253 KB file truncated at a token limit, and not continued;
- reading the opening section and searching from there.

## Rewrites were not blocked from adding explanations

The orchestrating agent reported:
> The submit check for rewrites (rounds 2, 3, and the post-support rewrites) accepts only letter and citation fixes.

ChatGPT read this as the main failure. **The records do not support it.**
- **The audit-round rewrites:** none of the round 2 rewrites (0054–0076) or round 3 rewrites (0103–0126) was rejected for new claims. Their rejections were:
  - symbols used before they were bound;
  - word counts;
  - relation ids written as file names (`.md`);
  - quotes not found.
- **The post-audit rewrites:** "new claims" rejections occur only there (0178–0209). Their notes were all letter overloads and unsupported citations; for example, 0191 for `main-theorem` had two citation notes and nine renamings. Writers added claims that those notes did not ask for, and submit rejected them, as 2026-10-12-narrow-rewrites intends.
- **The last round's notes:** round 3's 100 gaps were dropped by design, as the status says.

## What the pages contain

**The chain is largely on the pages,** although the plan assigned none of it:
- **`fast-butterfly-layers`** (519 words after rewrites) gives:
  - the factorisation of $H_0^{\otimes\Delta}$;
  - the recursion and the recurrence $T(mf)\le\frac{\Psi}{W}T(f)+O((mf\chi)^\tau+1)$;
  - the deficit $73/19906842167500 > 20\cdot2^{-50}$, giving $\Psi/W<m^\sigma$;
  - the unrolling with the cut-off at $\sqrt d$, and the leaf bound.

  It links `fast-address-interchange` and `recursive-power-saving`.
- **`subset-intersection-construction`** derives $s = WD - N + 2L$ (the paper's $m$ renamed $D$), with $L = 3\binom h3^2h^2$ and $L/N = 100/539$. In agreement with the paper.
- **`recursive-power-saving`** gives the recurrence and the geometric sum. Its leaf bound "$O(f^\sigma d^{\beta(1-\sigma)})\le O(f^\lambda)$" does not follow: with $d\le f^{1/\beta}$ it gives only $O(f)$, and the paper combines the terms as $O(d^{\lambda'})$. A writer flagged this, and so did the round 3 audit.
- **`gaussian-resampling`** (460 words) gives:
  - the identity;
  - the Gaussian maps;
  - the compensation $\Phi=\Gamma\mathsf T\Lambda$ and the left inverse;
  - the norm bounds (Harvey and van der Hoeven's Theorem 4.2, Lemma 4.6 and Proposition 4.7, cited as the paper cites them).

In the baseline, the butterfly page cited its bound and $s<Wm$ was derived nowhere. Here the mechanisms reached the pages through audit-driven rewrites: the writers went into the paper themselves. **Notes on writer cost:**
- writers averaged 54k tokens per write task, against 150k to 250k in the pilots;
- several reported reading sections of the paper.

**But the notation drifts along the chain.**
- **Different letters for the same thing:**
  - the paper's $s$ is $\Psi$ on the butterfly page;
  - the paper's $m$ is $D$ on the subset page;
  - $\sigma$ and $d$ carry notes that they "mean something else on the recursive-power-saving page".
- **Parenthetical notes:** `recursive-power-saving` carries four "this letter changes meaning here" notes.
- **The cause:**
  - 82 letters were confirmed as having two meanings;
  - the overload notes asked for renamings that later pages' bindings then blocked (`relied_on`);
  - so writers kept the letters and added notes.
- **The cost:** a reader following the chain meets a different letter for the same quantity on each page.

## The audit rounds

**The gaps, by the action taken:**

| Round | explain the step | cite or cut | candidate concept | symbol |
|---|---|---|---|---|
| 1 | 64 | 0 | 11 | 18 |
| 2 | 63 | 10 | 16 | 10 |
| 3 | 35 | 40 | 11 | 13 |

**Each round's step gaps were nearly all in passages not reported before:** 69 of 73 in round 2, 70 of 73 in round 3. The baseline was the same.

**What round 3 reported on the chain pages:**
- details inside the derivations the rewrites added: the sign convention of a label change, what an invocation is, why $K$ becomes $mfK$;
- and two real errors:
  - the leaf bound above;
  - in `gaussian-resampling`, "Entry $j$ of $\Phi f$ contains $f_j$ exactly", when the paper's own bound $\|\Phi-I\|<2^{-\alpha^2\theta}$ means the identity holds only approximately.

**Most of round 3's cite-or-cut steps are on derivations a correct plan would assign.** On `subset-intersection-construction`, the steps of the $s<Wm$ derivation were all sent back to be cited or cut, as reasoning inside reasoning a rewrite had added:
- "Signed label changes telescope";
- "So the signed total is $Wm-2N$";
- "Absolute values add $2L$";
- the rank-one source edges;
- $L = 3\binom h3^2h^2$.

The derivation entered through a rewrite, so the depth rule counts it as elaboration. With an owned derivation in the plan it would have been the page's own content. Round 3's notes were dropped, so none of it was cut in this run.

## Rejections at submit

**Write tasks:** most of the rejections were mechanical:
- symbols used before the sentence that binds them, the most common reason;
- word counts;
- relation ids written as `name.md`;
- quotes not found.

**Audit answers:** the audit symbol-anchor rejections recorded in 2026-10-13-planner-handoff-run-setup recur. Their counts are not in the run's records.

## Causes, with confidence

1. **The plans were made from the introduction.**
   - **Confidence:** high; measured above.
   - **What it explains:**
     - why ownership was sparse;
     - why the handoff reached three pages;
     - why the evidence passages were introductory;
     - why mechanisms arrived only through rewrites.
   - **Same in the baseline.** The baseline's hollow mechanism pages (2026-10-13-explanation-depth-run) have the same root.
2. **The Sonnet planner's advantage was measured with full reading, not with Sonnet under the run's definition.**
   - **Confidence:** high, that the comparison is confounded.
   - **Unknown:** how much is the model and how much the reading.
3. **The depth rule cuts content the plan should have owned.**
   - **Confidence:** moderate.
   - **The evidence:** round 3's cite-or-cut steps on the $s<Wm$ derivation.
   - **Downstream of cause 1.**
4. **Notation drift along the chain**, from overload renamings blocked by later pages' bindings.
   - **Confidence:** moderate. Its effect on readers is unmeasured.
5. **The gap counts cannot show quality on these pages.**
   - **Confidence:** moderate.
   - **Why:** each round's gaps are almost all new, a fresh audit finds about three on any dense page, and the chain pages improved while the counts did not.
6. **Rewrites were blocked from adding needed explanations.** Not supported: the records contradict it.

**A stronger model is not indicated by this run.** A supervisor that read only the introduction would plan the same way. Stronger triage of audit gaps may matter later, once the plans cover the paper.

## The smallest experiment that tells these apart

Planning only, on `paper.tex`, from the same project state:
- extract;
- background with the calculus redo;
- the argument.

**Four conditions:**
- **Haiku:** under `exegenius-planner`'s definition, and as a general-purpose subagent;
- **Sonnet:** the same two ways.

**What each records:**
- tokens;
- extraction and argument quotes by tenth of the paper;
- steps in the argument;
- whether the butterfly layer's recursion and the derivation of $s<Wm$ have owners.

**What it tells apart:** the model, the agent definition, or both.

**First, a cheaper look:** the planner's transcript for task 0001 in this run would show how its reading stopped.

## Current State
- In this run and in the explanation-depth run, the extraction's and the argument's quotes come almost wholly from the first tenth of `paper.tex`, the introduction
- Both Sonnet planning samples of 2026-10-13-model-comparison, run as general-purpose subagents, quote from throughout the paper and have 79 and 83 argument steps
- In this run the argument has 19 steps, and three pages own a sub-step; the owned-step section appeared in 13 of 190 write tasks
- No round 2 or round 3 rewrite was rejected for new claims; such rejections occur only in post-audit rewrites whose notes concern letters and citations
- `fast-butterfly-layers`, `subset-intersection-construction` and `recursive-power-saving` carry the layer's recursion, the derivation of $s = WD - N + 2L < WD$ and the recurrence, with one incorrect leaf bound on `recursive-power-saving`
- Round 3 sent the steps of the $s<Wm$ derivation on `subset-intersection-construction` back to be cited or cut

## Missing
- No record of how the planner's reading of `paper.tex` stopped
- No planning comparison that separates the model from the agent definition
- No measure of notation drift along a chain of pages

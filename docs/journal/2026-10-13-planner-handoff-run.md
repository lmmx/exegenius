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

**How the reading stopped: the planner chose to.** Another Claude Code session read the transcript of the extract planner (`agent-a0569a285a5a4788e`, type `exegenius-planner`, in the run's session `464895b1`) and reported:
- **The task:** it read the task file, which says "Sources (read them from disk)" and asks it to cover everything the target depends on in the sources.
- **The outline:** it ran `wc` on `paper.tex` (5,361 lines) and a `grep` for `\section`, `\subsection` and theorem environments. That listed the outline to line 5,259.
- **The one read:** it made one Read call on `paper.tex`, with `limit: 440` and no offset, which returned lines 1–440. It never read the file again.
- **The answer:** it wrote the answer with a single Python script, and submit accepted it.
- **The evidence:** all its evidence quotes are in the 440 lines it read: 27 in the answer file, though the session that read the transcript counted 28. The concept names for the later sections, such as `fast-butterfly-layers` and `scratch-restoration`, came from the outline.

**What this rules out:**
- not a tool limit: the cut was the planner's own `limit`;
- not a reading of later sections that the extraction then ignored: they were never opened.

**What it points to:**
- **An instruction gap.** Nothing in the task says the sources must be read in full, or section by section.
- **A validation gap.** Submit checks that each quote is in the source, not that a concept's evidence comes from where the source develops it.

The planner's reasoning is redacted in the transcript, so its choice is inferred from what it did.

**The argument planner read the same stretch, on its own account.** The same session read the transcript of the argument planner (`agent-aac7b93692ffc801a`, type `exegenius-planner`, task 0004, about 10:23 to 10:25 UTC, two to three minutes after the extract planner finished). It made four calls:

| Step | Call | Result |
|---|---|---|
| 1 | Read `exegenius/tasks/0004-argument.md` | 18,083 characters, 316 lines |
| 2 | `wc` on `paper.tex`, and a `grep` for `\section`, `\subsection` and theorem environments | 5,361 lines, and the outline to line 5,259 |
| 3 | Read `sources/paper.tex` with `offset: 37`, `limit: 400` | lines 37–436 (23,733 characters) |
| 4 | a Python script that wrote the answer, then submit | accepted |

**What the transcript shows:**
- **One read of the paper.** The planner never read `paper.tex` again after lines 37–436: the introduction (lines 37–431) and the first five lines of §2. That is the region the extract planner read.
- **No tool limit.** The 400-line limit was the planner's own, and the read ends cleanly at line 436.
- **The outline, seen and not used.** The planner listed the outline before reading, and did not read below line 436.
- **The evidence.** All 15 of the argument's inferred-step quotes are within lines 37–436, according to the session, which rebuilt the answer by running the planner's script. The accepted `argument.json` has 19 steps, 4 to show and 15 to infer.

**A correction to that report.** It says only three pages are attached to steps. The run's `argument.json` (72f1de8) has 16 pages presenting a step. Three of them own a sub-step: `main-theorem`, `fast-address-interchange` and `ring-convolution-retained-order`.

**What the argument task inherited.** Its task file lists the pages in reading order (24 at that point) and the concepts the reader knows. Both lists come from the extraction and its background task, so they too were drawn from the introduction. The plan had to attach its steps to those pages.

**So the failure is in both planning tasks:**
- **The argument planner read only the introduction itself.** It did not rely on the extraction for what it saw of the paper, and it would have read no further with a complete page list.
- **The extraction also fixed the pages the plan had to fit.** That gave the plan a shape that matches a reading of the introduction.

**The two task files' wording.** Both have the same section, "Sources (read them from disk)", listing `sources/paper.tex`, and neither says to read the whole source or each section:
- **The extract task** asks it to "Cover everything the target depends on in the sources".
- **The argument task** asks it to read the plan top down, using "section structure" among the marks of steps. It adds: "Go down to the level the pages need", and "Deeper steps of a page's own proof can stop at an `infer` step quoting where the sources carry them out".

**Hypotheses, not observed:**
- **The shared template.** The two tasks share an instruction template, so one omission could produce the same behaviour in both.
- **The introduction as enough.** The argument planner may have judged that the introduction states the proof's architecture and is enough for a plan. Its reasoning is redacted, so this cannot be checked.
- **Submit.** Its check on the order of quotes is satisfied by a few early quotes, and nothing in it sends a planner into the sections of the proof.

Why either planner stopped reading is not known.

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

**Done since:** the transcript showed the reading stopped at the planner's own 440-line limit (above).

## Current State
- In this run and in the explanation-depth run, the extraction's and the argument's quotes come almost wholly from the first tenth of `paper.tex`, the introduction
- Both Sonnet planning samples of 2026-10-13-model-comparison, run as general-purpose subagents, quote from throughout the paper and have 79 and 83 argument steps
- In this run the argument has 19 steps, and three pages own a sub-step; the owned-step section appeared in 13 of 190 write tasks
- No round 2 or round 3 rewrite was rejected for new claims; such rejections occur only in post-audit rewrites whose notes concern letters and citations
- `fast-butterfly-layers`, `subset-intersection-construction` and `recursive-power-saving` carry the layer's recursion, the derivation of $s = WD - N + 2L < WD$ and the recurrence, with one incorrect leaf bound on `recursive-power-saving`
- Round 3 sent the steps of the $s<Wm$ derivation on `subset-intersection-construction` back to be cited or cut
- The run's extract planner read lines 1–440 of `paper.tex` in one Read call with `limit: 440`, after listing the outline, and read no more; all its evidence quotes are in those lines
- The run's argument planner read lines 37–436 of `paper.tex` in one Read call (`offset: 37`, `limit: 400`), after listing the outline, and read no more; its 15 inferred-step quotes are in those lines, and its pages came from the extraction

## Missing
- No record of why either planning task stopped reading `paper.tex`
- No planning comparison that separates the model from the agent definition
- No measure of notation drift along a chain of pages

# Proposal for the author: put the validated argument-planning instruction on `master` (a default change; not merged)

**Status:** written on 2026-10-15 at 11:07 UTC for separate review, as the author asked: it should not block the experiments. Merging it changes a pipeline default, so only the author does it.

**A PR was not opened.** Creating its branch in an isolated worktree (`git worktree add -B …`) was denied by the session's permission classifier ("Git Destructive"). Per the author's rule, it was not retried or worked around. The diff below is the whole change. The author can apply it, or allow the branch and have the session open the PR.

## The change

The source is branch `141026-exp-params-cases` (e8efaa3), four experiment commits. The change is 14 added lines in `exegenius-core/src/argument.rs`, the argument task's instructions on the key step:
- develop it down to the sources' reasoning;
- state the identities and exceptions behind counts and bounds;
- develop every variant the target result uses;
- state the chain of estimates behind each bound;
- define each fixed parameter with its value;
- give every case of a case-by-case step.

No model, effort or other task changes.

## The evidence (`policy.md`, "Argument planning")

- **E9:** with this instruction, Sonnet `high` reaches 19/19 faithful and 9/9 central on F2, F4 and F8 (three Opus judges, blind, preregistered).
- **E10:** the same on further extractions.
- **E11:** with it, Opus `xhigh` is not better above the ceiling (2–2, 2 tied, p = 0.69).
- **S6:** Sonnet `medium` and Haiku `high` fall below with it.
- **Limit:** one paper. The second document (RUNBOOK item 7) is needed before calling it general.

**Cost:** a plan run is 3–5 M cache read with the instruction, against 2.5–7.7 M with E8 alone (`policy.md`, cost table).

## Before merging (the author's or a later session's)

- `cargo test` on the change, built in an isolated target directory. It was not run here, because the branch could not be created.
- Commit messages on the experiment branch say "not for master". The merge should be a fresh commit with this rationale, not those commits.

## The diff

```diff
diff --git a/exegenius-core/src/argument.rs b/exegenius-core/src/argument.rs
index d233793..f1b9c15 100644
--- a/exegenius-core/src/argument.rs
+++ b/exegenius-core/src/argument.rs
@@ -284,6 +284,20 @@ readers every reconstructed motivation, since the sources will not.
 - `from`: the concepts a step relies on, as ids: known concepts, or pages that come before \
 the step's page in the reading order below. A step's page is its own `page` or the nearest \
 ancestor's.
+- The key step: the step the target result turns on, the construction or estimate that \
+gives the result its strength (where a saving, a bound or a surprise comes from). A reader \
+who follows only one derivation should follow this one. Develop it with `show` steps down \
+to the reasoning the sources give for it, each sub-step quoting where the sources carry it \
+out, rather than stopping at an `infer` step that quotes its result. Where a sub-step \
+concludes from a count, a bound or an equality, state the identity, lemma or definition \
+that makes it hold, and every case the sources treat as an exception to it. Where the \
+sources carry out a construction or estimate in more than one variant, develop each variant \
+the target result uses; where a sub-step concludes a bound on an error, a size or a depth, \
+state the recurrence or chain of estimates that produces the bound. Define each parameter \
+and constant the target result's statement or derivation fixes (a size, a count, a \
+threshold) with the value the sources give it, at the step that first uses it; where the \
+sources establish a sub-step case by case or by a table, give every case and what it \
+contributes.
 - `salient`: for a step whose claim has a feature a reader would find striking (a symmetry, \
 a uniformity across cases, a surprising value or bound), that feature. A proof explains a \
 result only relative to such a feature (Lange), so its page is asked to explain why the \
```

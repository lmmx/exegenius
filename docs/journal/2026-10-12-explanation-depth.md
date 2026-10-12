# 2026-10-12: Explanation depth relative to the argument

2026-10-12-responsibility-prototype proposed that the depth limit be relative to a page's explanatory responsibility, as the argument records it. ChatGPT reviewed the prototype, agreed, and set three conditions:
- the exception covers the method of the page's own step of the argument, not every term, lemma or derivation the page touches;
- the depth boundary is tested exactly, against an off-by-one;
- the next run is judged on content and recurrence, not on the number of gaps.

It also asked that the budget and the extend policy stay as they are for that run. This entry records the rule as built.

## The rule

**Depth.** Each sentence a rewrite adds in answer to a step note, or to a note to cite or cut a step, gets a depth: one more than the deepest step its notes asked about. Text not added for a step has depth 0. Each page's deepest queued step is kept in `note_depth`, and its added sentences in `added`, now each with its depth. Sentences stay recorded while they are on the page (exegenius-core/src/graph.rs, exegenius-core/src/write.rs). A graph written before depths were stored reads its added sentences as depth 1.

**Limit.** A page that presents a step of the argument (`argument.json`) explains steps to two levels, `ARGUMENT_LEVELS`. Any other page explains to one, `LEVELS` (exegenius-core/src/audit.rs `levels`). A step reported at depth $d$ on a page with limit $L$:

| | $d < L$ | $d = L$ | $d > L$ |
|---|---|---|---|
| sentence uncited | explained | cited or cut | cited or cut |
| sentence already cited | explained | explained | cited or cut |

The cited column keeps 2026-10-12-full-latex-run's rule that a citation cannot answer a step on a cited claim, and bounds it at one level past the limit. The test `steps_are_explained_to_the_page_s_levels_then_cited_or_cut` writes out every cell, for depths 0 to 3 at both limits:
- `ECECCCC` at one level;
- `EEECECC` at two.

The expected row for two levels was first written down wrong, as `EEEECCC`. That omitted the cited claim at the limit. The code was right, and the table was corrected.

**Notes.** On a page presenting a step of the argument, the note for a step below the limit but inside added reasoning adds:

> It is inside reasoning added for an earlier step. If it is part of how this page's own step of the argument works (see "This page in the argument"), develop it here; if it concerns a result that step uses, say what the result states and cite it.

That carries ChatGPT's condition: the extra level is for the method of the page's own step, and a result the method uses is stated, not explained. The writer decides which a step is, and the limit bounds the depth whatever it decides.

The end-to-end test `a_page_presenting_a_step_explains_one_level_more` runs three rounds on the page presenting the argument's step:
- depth 0 is explained;
- depth 1 is explained;
- depth 2 is sent back to be cited or cut.

The earlier test of a page presenting no step still sends depth 1 back to be cited or cut.

## Unchanged

- The rewrite budget, and what extend may add.
- The three fixes of 2026-10-12-full-latex-run:
  - added reasoning tracked across rewrites;
  - steps on cited claims explained;
  - citations of LaTeX markup rejected.

An explained step at depth 1 counts towards the stop rule, as any explained step does. A run that should not stop early sets `stop_early = false`.

## Judging the next run

ChatGPT's measures, for the pages presenting a step:
- whether the page states its assigned method and derives it;
- whether each claim agrees with the source;
- whether notation stays consistent;
- whether reported steps recur.

Also tracked: reported steps by depth and round, and words against the budget. A page presenting a step that sits at its budget, as `simultaneous-butterfly-layer` did in the prototype, is evidence for the deferred question of budget or added pages, not for changing them in this run.

## Current State
- Added sentences carry a depth, one more than the deepest step their rewrite's notes asked about, and stay recorded while on the page (exegenius-core/src/graph.rs, exegenius-core/src/write.rs)
- Pages presenting a step of the argument explain steps to two levels, other pages to one; a step on an already cited claim is explained one level past the limit (exegenius-core/src/audit.rs)
- Below the limit inside added reasoning, a note on a page presenting a step asks for development only of that step's method, and for a result it uses to be stated and cited (exegenius-core/src/audit.rs)

## Missing
- No run with explanation depth relative to the argument
- No check that a writer tells its page's method from a result the method uses

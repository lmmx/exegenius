# 7. What the run was meant to test

## The handoff, barely exercised

The variable this run was meant to test was the owned-step handoff. Write tasks would pass on what the plan says a page derives itself:
- the method of each step the page owns;
- the steps beneath it, with their source passages;
- which neighbouring steps belong to other pages.

We then found a root cause of something more significant, and the analysis turned to that. It was a side quest, though a necessary one. I want it noted that the thing under test got lost.

**What the plan allowed:** only three pages owned a sub-step, so the handoff section appeared in 13 of 190 write tasks.

**What that means for the evidence:**
- The run is an inadequate test of the handoff, not evidence that it failed.
- The earlier pilot and replication remain the only evidence for it: owned multi-step derivations appeared on 5 of 6 pages written with the handoff and on 0 of 6 without it. That evidence is small, the scoring was not blind, and the handoff cost more tokens and time.

**Status:** observation, from 2026-10-13-planner-handoff-run, 2026-10-13-brief-pilot and 2026-10-13-owned-step-handoff.

## Gap counts as a measure

I found it interesting that the analysis calls gap counts the wrong measure.
- **The evidence:** each round's step gaps were almost all in passages not reported before (69 of 73 in round 2, 70 of 73 in round 3), as in the baseline. The chain pages improved while the counts did not.
- **The consequence:** a level count is not a regression. Evaluation has to separate:
  - real regressions;
  - gaps newly found in material a rewrite added;
  - gaps reported again;
  - improvements the counts do not register.

**Status:** interpretation, with the figures as observations.

## "Better than the status suggests" is not the point

The analysis found that the chain pages are better than the status suggests:
- the butterfly layer's recursion is on its page;
- $s<Wm$ is derived on one page, though with an error in a leaf bound on another;
- the notation drifts from page to page.

That is fine as far as it goes. But I am not after reassurance that things are less bad than they looked. Something is clearly wrong, and I want improvements.

## The depth rule, and Chesterton's fence

There is a depth rule in the program: a heuristic that stops reasoning being elaborated too deep on a page. I do not fully understand where it sits, and I have tried not to interfere with parts of the design I do not understand. That is Chesterton's fence: do not remove a fence until you know why it was put up.

**What the analysis found:** round 3 sent steps back to be cited or cut that a correct plan would have assigned to the page. The derivation of $s<Wm$ had come in through a rewrite, so the depth rule treated it as elaboration.

**My first reading:** perhaps the agent making those decisions is too small.

**The analysis's reading:** the depth rule is code, applied to whatever a rewrite added; the decision is not a model's. With an owned derivation in the plan, the same content would have been the page's own. Round 3's notes were dropped by design, so none of it was cut in this run.

**Status:** the cut is an observation. Its cause, the plan rather than a model, is an interpretation at moderate confidence.

## The chain

"The chain", as I understand it, is a path through the graph of reasoning: the connections a reader makes between a few related concepts that make something click, an "aha" moment. Here it means the butterfly layer, interchange, and the saving $s<Wm$.

That the chain is on the pages although the plan assigned none of it sounds bad to me: the core things are not lining up in the plan.

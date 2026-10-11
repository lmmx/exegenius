# 2026-10-11: Depth limit

Step gaps renew: rewrites fix the steps they are told about and add as many new ones (2026-10-11-reader-background). This entry describes why, and the limit that stops it.

## Why steps renew

A step gap is one sentence on one page that presents something as following ("so", "hence", "because"), where the reader cannot see why from what the page gives. Take a page that says:

> $n^2 - n$ is always even, so we can halve it.

An audit reports the step. The rewrite note asks for the reasoning or a citation, and the writer gives the reasoning:

> $n^2 - n = n(n-1)$, a product of two consecutive integers, one of which is even, so it is even.

The next audit reads the new sentence fresh and can report a step inside it: why is one of two consecutive integers even? Each explanation is a new chain of links, and the next audit looks one link deeper. The regress ends only where the auditor judges the reader knows the link. That judgement varies from audit to audit, so nothing guarantees that it ends.

The reduction pilot shows it. On `discrete-fourier-transform`, round 1 reported that the page does not show why the opposite transform inverts the transform. The rewrite gave the reasoning, through the powers of ω summing to zero over a full period. Round 2 reported two steps inside that reasoning: the sum of the powers of ω, and the expansion of the double sums.

## The limit

Each step is explained one level deep. When a rewrite answers at least one step note, `apply_write` records the sentences it added: those of the new version not in the previous one, as audits see the page (exegenius-core/src/audit.rs `added`, exegenius-core/src/write.rs). They are stored in `graph.json` as `added`, and replaced or cleared at the page's next write. A step gap whose quote lies inside one of those sentences is dropped with the action `LEFT`, and gets no rewrite note. It does not count towards the stop rule. `exegenius status` prints how many steps were left this way.

A step reported on a sentence that was there before the rewrite is unaffected. If the rewrite did not fix the original step, the next audit reports it again and the page is rewritten again.

The limit holds whatever the auditors judge the reader to know, so it does not depend on the background task. The cost is that a step inside added reasoning stays as written. The rewrite budget keeps added reasoning short, and `status` shows how many steps were left.

## Free check

The limit ran over the reduction pilot's task files (lmmx/openai-nlogn-wiki `pilot-v0.6-convergence`, 4b921b6) through `audit::added`, at no model cost. Three pages were rewritten in round 1 in answer to step notes, and round 2 reported 6 step gaps on them. All 6 lie inside sentences the rewrites added:
- on `discrete-fourier-transform`, the sum of the powers of ω and the expansion of the double sums;
- on `gaussian-resampling`, $\alpha^2 < p$ and the Gaussian identity;
- on `transform-identity`, the inverse commuting with the transform and the scale of the convolution identity.

The seventh step gap on the repeated pages, on `radix-digit-convolution`, is on a page rewritten for other notes, and stays.

With the late-meaning rule and the letter rule of 2026-10-11-reader-background, step and symbol gaps on the repeated pages would have gone from 13 to 7, against 18 to 26 as run. The stop rule needs 6 or fewer, so it would still have stopped the run, by one gap. This replay leaves out the background task, the growth limit, and the rewrites the dropped gaps would not have caused. Only a run measures those.

## Current State
- A rewrite that answers a step note records the sentences it added, and a step gap reported inside one is dropped as `LEFT`, with no rewrite note and no count towards the stop rule (exegenius-core/src/audit.rs, exegenius-core/src/write.rs, exegenius-core/src/graph.rs)
- `exegenius status` prints the number of steps left by the depth limit (exegenius-cli/src/main.rs)
- On the reduction pilot's task files, the depth limit leaves all 6 round 2 step gaps on the pages rewritten for step notes

## Missing
- No pilot run with the depth limit
- No depth limit on symbol gaps inside added reasoning; the letter rule covers new letters only

# 2026-10-11: Convergence

The 0.5.0 run on the n log n paper did not converge, and nothing in exegenius noticed until 855 tasks had been issued (2026-10-11-divergent-run). This entry sets out the plan:
- a measure of convergence, and a stop rule built on it;
- a pilot run small enough to use as an eval before a full run, and between versions;
- the fixes for the three causes the run's records support.

Nothing here is implemented yet.

## The audit loop as an iteration

Each audit round maps a wiki to a wiki: audits report gaps, rewrites and extend act on them. The loop is meant to reach a fixed point with no gaps. Two numbers say whether it is getting there:
- **The rate:** gaps per audited page, by kind, each round. In the 0.1 run it fell by more than half each round, for every kind. In the 0.5.0 run, step gaps went 4.1, 3.9, 3.4.
- **Renewal:** of the gaps on pages audited in two consecutive rounds, the share whose quote was not reported before. A rewrite that fixes what it was told about and adds nothing scores near zero on the second round. In the 0.5.0 run, renewal of step gaps was 163 of 171 and 302 of 321.

Term gaps also grow the graph. The growth factor is the pages extend adds per page audited. In the 0.5.0 run, the 32 pages of round 1 led to 50 new pages.

`rounds` caps the number of rounds, which is a budget, not a test of convergence. Neither number above is computed anywhere.

## Changes

### 1. A convergence report and a stop rule

`status` and the report would give, per round and kind, the gap rate, and the renewal on pages audited in the round before. `audit.json` already holds every gap with its page and quote, so this needs no new task.

`next` would stop the audit rounds early, saying why in `status`, when after a round either:
- the step or symbol gap rate on pages audited in both rounds fell by less than half; or
- extend would add more new pages than there are pages in the plan.

The thresholds come from the two runs: the 0.1 run met both every round, and the 0.5.0 run failed the first after round 2. Stopping early does not lose work: the pages stay written and audited, and the outline and simulation follow as usual.

### 2. A limit on overload rounds

Overload verdicts can queue rewrites whose new wording yields new candidates, with no limit (2026-10-11-divergent-run). Overloads would stop after two rounds, as support checks do (exegenius-core/src/support.rs:29). Later candidates would go to the report.

### 3. Citations that leave the sentence whole

234 of the run's 771 citation marks follow a word or colon that makes the sentence need the quoted text. Audits and the site both take that text out of the sentence, so the sentence reaches readers broken (2026-10-11-divergent-run).

Submit would reject a mark that follows "that", "as", "states", "says", "is", "gives", "reads" or a colon, asking for the claim in the writer's words with the quotation after it. The check is a heuristic. A false rejection costs one more submit; a mark it misses is no worse than today.

### 4. Steps checkable by reference

The audit counts any claim "the reader cannot check from what the page gives". For a research paper, every explanation of a step makes new claims of that kind, so the rewrites in the 0.5.0 run replaced the steps they were told about with as many new ones.

The audit instructions would accept a step the page attributes to the source with a citation as checkable by reference. A step the page presents as following from what it gives must still be checkable on the page. The rewrite note for a step gap would offer both fixes: derive the step, or attribute it to the source and cite it. The writer would be asked to choose whichever adds less.

### 5. An enforced length budget on rewrites

Pages rewritten in three rounds have a median of 589 words, against 289 for pages never audited. Submit warns past the budget (exegenius-core/src/write.rs), and the warning did not hold.

Submit would reject a rewrite over budget. The budget is the previous version's words plus a quarter, and at least 50, as now. A page whose notes need more than the budget would have its budget raised when the notes include an unexplained term, which needs words; step and symbol notes would not raise it.

### 6. A pilot as an eval

`[pilot] pages = n` in `exegenius.toml` would limit the plan to the target and its n − 1 nearest prerequisites, taken breadth first. Concepts outside it are left out of the plan, and extend may add no pages beyond n.

With `rounds = 2` and the simulation off, a pilot of 8 pages costs about:
- 8 writes and 8 audits;
- 8 rewrites and 8 re-audits;
- the extract, argument and extend tasks.

That is about 35 tasks, against 855 for the divergent run. The convergence report gives its rate and renewal.

Used two ways:
- **Before a full run:** a pilot whose renewal stays high, or whose rate does not halve, says the full run will not converge either.
- **Between versions of exegenius:** the same pilot on the same paper gives comparable numbers, so a change to the instructions can be measured before it ships.

The pilot measures the loop, not the explanations. The simulated readers still judge what the wiki teaches.

## Order of work

1. The convergence report and the pilot (changes 1 and 6), so that the rest can be measured.
2. A pilot on the n log n paper with 0.5.0's instructions, on Haiku, as the baseline. It should reproduce the divergence at about 35 tasks.
3. The overload limit, the citation check, steps checkable by reference, and the enforced budget (changes 2 to 5), each followed by the same pilot.
4. The stop rule switched on by default, once the pilot shows the thresholds separate converging runs from diverging ones.
5. The wall-time and token work of 2026-10-11-wall-time.

## Current State
- `rounds` limits the number of audit rounds and nothing tests whether a round reduced gaps (exegenius-core/src/task.rs:647-650)
- `audit.json` records every gap with its page, kind and quote for every round, which the rate and renewal can be computed from (exegenius-core/src/audit.rs)
- Overload rounds have no limit; support checks stop at two per page (exegenius-core/src/support.rs:29)
- The rewrite length budget warns and does not reject (exegenius-core/src/write.rs)

## Missing
- No gap rate or renewal in `status` or the report
- No stop rule on convergence
- No pilot setting
- No limit on overload rounds
- No check that a sentence reads complete without its citation mark
- No acceptance of steps checkable by reference
- No enforced rewrite budget

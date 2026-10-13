## The distinction I was missing

Your examples point to two related but substantially different kinds of creativity.

Discovery taste: inventing a new way to see the problem

The Barnette's Conjecture example is striking because the proof reportedly changed the approach to the problem: rather than pursuing the familiar graph-reduction route, it constructed a Hamiltonian cycle by explicitly selecting the edges outside it. The commenter described the key idea as unexpectedly at home in theoretical physics. That's a possible example of a conceptual reframing, not merely a better execution of an established strategy.

The stronger claim in the examples is that models may be finding mathematical mechanisms that knowledgeable humans had not found, or had struggled to find.

Research taste: finding the best route through uncertainty

TasteVal tests a different capability: choosing experiments, interpreting their results, and deciding what to do next. Its model acts as the researcher while a fixed coding agent executes the experiments. The reported 2.30× compute multiplier for Opus 5.5 means greater experimental compute efficiency against its expert reference—not that the model makes 2.3 times as many discoveries or is generally 2.3 times better than human researchers. Final performance is a separate metric.&#x20;

[image](https://www.google.com/s2/favicons?domain=https://arxiv.org\&sz=32)

arxiv.org

+1



The first is about generating a genuinely new conceptual move. The second is about reliably selecting and refining moves in a search process.

They can reinforce one another. Novel insight creates promising directions that did not previously exist; research taste helps identify which directions to pursue, how to test them, and when to abandon them.

But neither guarantees the other. A model could be brilliant at efficiently optimising known approaches without inventing a new one. Or it could produce a remarkable conceptual leap but lack the judgment to validate, develop or exploit it.

That distinction changes what I would recommend for Exegenius.

## What this changes about the Exegenius intervention

The previous task asks Claude to identify a weakness and implement an improvement. That is useful, but it subtly biases the model toward local repair: find a gap in the existing design and fix it.

The kind of creativity you're pointing at could instead question the design itself.

For example, Exegenius has been trying to improve technical explanations through better planning, coverage checks, derivation rubrics and recovery. A genuinely creative development model might ask:

- Are we treating explanation as a property of a document when we should treat it as a path through a space of ideas?
- Are we asking a planner to anticipate every important derivation when the system could discover missing reasoning dynamically, as the wiki is assembled?
- Could the best evaluation be based on whether readers can solve a new problem, rather than whether the wiki reproduces the source faithfully?
- Is the right abstraction a wiki generator at all—or a system that builds, tests and navigates explicit models of the source's reasoning?

Those are illustrative hypotheses, not recommendations to implement any of them. The important property is that the model can question the assumptions defining the current solution space, generate an alternative, and find a way to test it.

The article should act as a catalyst for reframing, not merely as justification for an improvement we already know how to make.

I'd preserve the original task's requirement for a demonstrable intervention, but explicitly allow the model to challenge the current architecture and problem formulation before choosing its intervention.

## How to make this repeatable rather than a one-off

I would now build around a different unit of work: a discovery cycle, rather than just a research-to-intervention cycle.

1\. Expose a difficult, consequential problem

Give the model the actual system, evidence of failure, objectives and constraints—not a prescribed solution.

2\. Search for conceptual alternatives

Generate different explanations of the failure, including alternatives that challenge existing abstractions or assumptions.

3\. Identify a discriminating test

Choose a cheap experiment or prototype that could distinguish the novel idea from the current approach.

4\. Develop and validate the promising idea

Implement it where justified; test correctness, actual improvement, cost and unintended effects.

5\. Preserve the discovery and its provenance

Record the original problem, alternative hypotheses, decisive insight, tests, outcomes and what was genuinely new.

The key difference from the previous proposal is step two. We should measure not only whether a model chooses good experiments, but whether it discovers useful possibilities that a competent baseline would likely miss.

### Three things I would measure separately

| Capability         | The question                                                                                              |
| ------------------ | --------------------------------------------------------------------------------------------------------- |
| Conceptual novelty | Did the model propose a meaningful mechanism, representation or approach beyond the obvious alternatives? |
| Research taste     | Did it choose informative experiments and interpret the evidence well?                                    |
| Validated impact   | Did the idea survive scrutiny and improve the system or its outputs?                                      |

A novel idea can be wrong. A good experiment can validate a conventional idea. A clever implementation can deliver no meaningful benefit. Combining these into one score would obscure precisely the capability we're interested in.

For Exegenius, the strongest evidence of creativity would be a model that discovers a previously unconsidered, valid intervention that materially improves explanatory fidelity or reader understanding—not one that merely proposes a more elaborate planner.

## How to exploit future models

I'd make the discovery cycle itself repeatable, then use successive models as a natural experiment.

- Run the same consequential problems against today's model and future frontier models, giving them equivalent access to the relevant code, history and tools.
- Preserve their proposals before revealing one another's answers, so we can assess independent convergence and genuinely different approaches.
- Have promising ideas evaluated against existing baselines, with correctness and impact checked independently of the proposing model.
- Track which models find useful conceptual alternatives, which select good tests, and which turn ideas into validated improvements.

TasteVal gives us a reason to take research judgment seriously, but its results do not establish a comparable trend in novel mathematical discovery. Its reported rapid improvement is specifically in experimental compute efficiency on a small suite of AI R&D tasks; its final-performance trend is much slower.&#x20;

[image](https://www.google.com/s2/favicons?domain=https://rsiprogress.com\&sz=32)

RSI Progress

+1



We should therefore test the creativity hypothesis directly rather than assume that better research taste automatically entails better invention.

## My updated recommendation

Keep the article-inspired Exegenius task, but change its success criterion: the model should be free to discover that our current way of framing the problem is itself the thing that needs fixing.

Then preserve the task as a reusable discovery benchmark, not just a prompt. Each new model should face a mixture of historical failures and fresh problems, and we should compare the novelty, validity and impact of its interventions.

This also gives us a more precise interpretation of the RSI claim. If models can repeatedly invent new ways to improve AI research, validate those approaches, and use the resulting improvements to discover further advances, they could help close a recursive improvement loop. But creative mathematical breakthroughs, experimental taste and full recursive self-improvement are distinct milestones; the examples you've shared are reasons to investigate that possibility, not proof that the entire loop is already closing.

The opportunity for Exegenius is to build a small environment where this kind of creativity can be expressed, tested and accumulated—so a more capable future model can do more than complete the tasks we already know how to specify. It can help us discover which tasks we ought to be specifying in the first place.
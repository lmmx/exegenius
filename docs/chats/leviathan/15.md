I’d ask Claude Code to find the most valuable next move for Exegenius, rather than ask it to solve a problem you have already defined. The prompt should give it enough context to orient itself, but leave the diagnosis, framing and proposed intervention open.

The repeatable exercise is: given the current state of the software, what is the most promising thing to discover, change or test next—and why?

# Exegenius: Open-ended research and development exercise

You are working on Exegenius, software intended to turn technical and mathematical papers into wikis that preserve not only their results, but the reasoning, derivations and conceptual structure that make those results intelligible.

Your task is to identify and pursue the most valuable next step in improving the software.

**Do not assume that the right next step is already known.** Do not restrict yourself to the current backlog, the most obvious bug, or the direction suggested by previous experiments. Use your own judgement and explore possibilities that may not yet have occurred to me.

## What I want from you

1. **Orient yourself.** Inspect the current codebase, its architecture, recent changes, relevant experiment reports, known limitations and existing evaluation machinery. Establish what is actually implemented, what has been demonstrated, and what remains speculative. Do not rely on stale summaries when the repository can answer the question.
2. **Form your own view of the opportunity.** What is the most consequential limitation, unexplored possibility or mistaken assumption in the current system? Look beyond local code improvements. Consider whether the problem is being framed correctly, whether the architecture is missing a useful capability, whether a different approach could work better, or whether we are measuring the wrong thing.
3. **Think creatively, but discriminate between ideas.** Generate genuinely different possible directions before settling on one. Look for non-obvious approaches, unexpected connections, alternative mechanisms and experiments that could change our understanding. Do not manufacture novelty for its own sake, and do not simply reproduce the direction of the most recent experiment.
4. **Choose a concrete next move.** Select the direction you believe has the greatest expected value, taking account of potential impact, uncertainty, cost, reversibility and what we would learn if it failed. Explain why it is more promising than the strongest alternatives.
5. **Carry it through.** Where feasible, implement the change or run the experiment, using the existing software and evidence as your starting point. Establish a baseline where necessary. Make the smallest intervention that can meaningfully test the idea, and preserve enough evidence to distinguish success from failure. If implementation would be premature, carry out the investigation needed to resolve that uncertainty instead.
6. **Evaluate honestly.** Test the result against the original problem, not just against whether the code runs or the immediate task passes. Look for regressions, alternative explanations and ways your approach could have appeared successful without actually helping. Do not claim an improvement that the evidence does not establish.
7. **Leave the next investigation easier.** Record the hypothesis, rationale, alternatives considered, changes made, evidence obtained, remaining uncertainty and the most promising follow-up. Preserve useful negative results. Make the exercise reproducible from the repository rather than dependent on this conversation.

## Working principles

- Treat the intended outcome as important, but the current understanding of how to achieve it as provisional.
- Use your own judgement. Do not ask me to choose between routine implementation options or approve ordinary steps.
- Prefer discoveries that improve our understanding of the problem, even when they invalidate an existing direction.
- Distinguish measured facts, interpretations and speculation.
- Do not change several independent things at once if doing so would make the result difficult to interpret.
- Avoid unnecessary rewrites, elaborate infrastructure and experiments whose results would not affect a decision.
- Preserve existing work and evidence. Keep changes scoped and reviewable.
- Ask me only if a genuine blocker or a consequential decision about research direction, architecture, cost or risk requires my involvement.

## Deliverable

Leave the repository in a useful state, with the work implemented or investigated as far as justified by the evidence.

Report concisely:

- What you discovered that was not already obvious.
- What you chose to pursue, and why.
- What you actually changed or tested.
- What the evidence establishes, including failures and uncertainties.
- What you recommend investigating next, which may be a different direction from the one you pursued.

The objective is not to produce an impressive proposal. It is to exercise independent technical and research judgement, make a consequential advance if possible, and leave a traceable basis for the next exercise.

## How to make it repeatable without making it formulaic

I would keep the prompt stable across sessions and let each session start from the repository’s current state. The consistency comes from asking the same underlying question—not from forcing every session through the same solution process.

One important distinction: latent creativity is not the same as unconstrained execution. The prompt leaves Claude free to discover the problem and invent the approach, while requiring it to test its own ideas against evidence.

For the first few exercises, I’d compare them on three separate dimensions:

- Discovery: Did it identify something genuinely non-obvious and relevant?
- Judgement: Did it choose a promising direction over plausible alternatives?
- Impact: Did the work actually improve Exegenius, or produce evidence that changes what you should do next?

Don't collapse those into one score. A session can have excellent research taste but fail to produce a working improvement; another can make a useful local fix without discovering anything important.

And resist adding too much more to the prompt initially. The point of repeating this exercise is to find out what Claude discovers when given room to think—not to encode your preferred discoveries in advance.
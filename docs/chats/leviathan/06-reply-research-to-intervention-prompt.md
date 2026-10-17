Then I would change the assignment substantially. You're right: a research note that sits beside the software is not the outcome you want.

The article should be the starting point for finding a concrete intervention in Exegenius that makes the system better at preserving and producing understanding. The research is there to identify the intervention, not become a deliverable in its own right.

I'd give Claude ownership of tracing the conceptual insight into the existing system, finding the weakest link, and proposing a change that can be implemented and tested. It should implement the change if the evidence supports a bounded, reversible intervention; it shouldn't wait for us to translate its analysis into engineering work.

Here's the prompt I'd use instead.

# Research-to-intervention task: make Exegenius preserve understanding

Read Alexander Gamburd's *The Siren Call of Silicon Leviathan: Reflections on blowup and Aufklärungsdämmerung* ([https://arxiv.org/abs/2609.28591](https://arxiv.org/abs/2609.28591)), then use it as a starting point for identifying and making a concrete improvement to Exegenius.

**The desired outcome is a demonstrable improvement to the software or its operating methodology—not a discussion, literature review, or standalone research note.**

## Context

Exegenius generates technical wikis from mathematical papers. Our emerging concern is that a wiki can state correct results and cover the source without preserving the reasoning that makes those results intelligible.

We care about:

- correctness and source fidelity;
- faithful reconstruction of important derivations;
- the dependencies, intermediate steps and conceptual throughlines connecting results;
- whether the resulting explanation actually helps a reader understand the material.

These are distinct objectives. We do not want a single superficial quality score to conceal a failure in one of them.

## Your responsibility

Work from the article into the existing Exegenius implementation, evaluation machinery, documentation and experiment history. Find a specific weakness that the article helps us see more clearly, then address it.

Do not assume the right intervention is a new agent, a new rubric, a prompt change or more documentation. Inspect the existing system first. The best intervention may instead be to connect existing components, expose a missing signal, change a decision rule, improve recovery when a derivation is incomplete, or build a small evaluation that reveals a previously invisible failure.

### 1. Find the actionable gap

Establish:

- What insight from the article is genuinely relevant to Exegenius?
- Where, specifically, does the current system fail to embody that insight?
- What evidence supports that diagnosis? Use existing outputs, tests, experiments and code where possible.
- What is the smallest intervention likely to improve the situation?

Distinguish the article's explicit claims from your own engineering hypotheses. Do not treat philosophical resonance as empirical evidence.

### 2. Choose and execute an intervention

Select the highest-value intervention justified by the evidence. Prefer a small, coherent, reversible change that addresses a real failure over a new subsystem.

The intervention may involve code, prompts, agent definitions, evaluation tools, documentation, experiment design or the connections between them.

Where feasible, include a test that would fail under the old behaviour and pass under the improved behaviour. If the benefit concerns reader understanding, do not claim success merely because an LLM judge approves the result. Use a suitable proxy initially, and identify the smallest credible reader-centred validation.

Implement and test the change if it is bounded, safe and compatible with work already in progress. Do not modify or disrupt the active E3 experiment or its source state. If the best intervention requires a consequential architectural decision, substantial resource commitment or a change to the research objective, present the specific decision and recommendation rather than making that change unilaterally.

### 3. Demonstrate the result

Leave behind the appropriate durable artefacts:

- the code or configuration change, if warranted;
- regression tests or an evaluation showing what changed;
- concise documentation of the new behaviour and its rationale;
- a record of evidence, limitations and any remaining uncertainty.

Connect the rationale to the article and to Exegenius's own observed failure modes. Do not create a separate conceptual framework unless it is needed to implement or evaluate the change.

### 4. Report back with a decision-ready result

Keep the report short. Tell me:

1. What concrete weakness you found.
2. What you changed, and where.
3. What test or evidence demonstrates the change.
4. What remains unproven, especially concerning actual reader understanding.
5. What the next highest-value intervention would be, if any.

If you cannot justify an implementation, do not manufacture one merely to produce code. In that case, deliver the smallest experiment or diagnostic that would resolve the uncertainty and make it executable where feasible.

**Operating principle:** use the article to improve Exegenius, not merely to explain Exegenius back to us. Make routine decisions autonomously, preserve provenance, and do not wait for approval on ordinary implementation details.

## What we should expect to get out of this

Ideally, we get a change to Exegenius that addresses a specific, demonstrated weakness—for example, a missing link between identifying a central derivation and ensuring the final wiki actually explains it. That is only an example, not a predetermined solution.

The key is that the intervention should close a loop:

Identify a real explanatory failure

Grounded in existing outputs and the article's insight

Change the system's behaviour

Code, configuration, workflow or evaluation

Test whether the failure is reduced

A regression test, controlled comparison or reader-centred measure

There is one important boundary: we should not mistake making an explanation look more rigorous for making it more understandable. The intervention might improve derivation fidelity first, while reader understanding remains an open empirical question. That's fine, provided the result is explicit about the difference.

I'd run this in a fresh Claude Code session, in parallel with the E3 work. This time, the expected output is a tested, traceable improvement—not another conversation about what Exegenius ought to become.
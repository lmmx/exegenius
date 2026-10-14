Yes — we have already been building towards this in the calibration work. The Capability Map is the important precursor: it records which configurations meet which quality bars, at what cost, with what degree of confidence, and for which versions. The Evaluation Registry supplies the underlying experiment records, and the Calibration Protocol defines how those records should be interpreted.

What you're proposing now is a more expressive, explorable version of that work: something that lets you see the capability landscape, investigate why it looks the way it does, and use it to make engineering decisions.

I'd make one distinction central to the design:

- Capability Map: what can each configuration reliably do?
- Performance landscape: how do configurations compare, and what trade-offs exist?
- Exegenius system map: which components contribute to those capabilities, and where are the bottlenecks?

These should be connected views of the same evidence, not three separate systems.

## 1. The most useful design ideas

I'd build the landscape around a handful of views, each answering a different question. They could all draw from the same registry rather than requiring separate benchmarking infrastructure.

A. Model × effort plots

Question: How does performance change with effort?

Your original idea. One line per model, effort settings as ordered points, performance on the vertical axis. Show uncertainty intervals, individual observations where useful, and cost alongside quality. Allow the user to switch between metrics.

B. Capability heatmap

Question: Who is good at what?

Rows are model/configuration combinations; columns are capabilities such as extraction, derivation, fidelity and reader understanding. Colour encodes the score, but every cell should expose sample size, uncertainty and evidence. Missing measurements must look different from poor performance.

C. Quality versus cost

Question: Which configurations are worth their price?

Plot quality against cost or latency, identifying Pareto-efficient configurations: those for which no alternative is both cheaper and better. Let users choose a quality threshold and find the cheapest configuration that meets it reliably.

D. Pipeline capability map

Question: Where does each capability come from?

Map extraction, planning, writing, auditing and revision to the configurations they use and the evidence about their contribution. Show which stages are bottlenecks, where improvements are lost downstream, and which stage-level findings have actually been validated end to end.

E. Evidence and failure explorer

Question: Why should I believe this result?

Click a result to inspect its runs, judge decisions, transcripts, failures and relevant source material. Group errors by failure mode, not just score. This is where an apparent model advantage becomes a testable explanation rather than a leaderboard claim.

These five views serve different purposes. I wouldn't force them into one giant dashboard. Start with the plot and capability heatmap; add the others when the data can support them.

## 2. The Capability Map should become the centre of gravity

The existing Capability Map is more valuable than a leaderboard because it can represent what the evidence entitles us to conclude, rather than merely sorting configurations by a number.

For each capability, I'd want a record along these lines:

| Field            | Example                                             |
| ---------------- | --------------------------------------------------- |
| Capability       | Central-chain reconstruction                        |
| Task and domain  | Mathematical paper, central derivation              |
| Configuration    | Sonnet, high effort, planner                        |
| Quality bar      | Passes a predefined chain-reconstruction test       |
| Evidence         | 2 of 5 runs pass                                    |
| Cost             | Measured tokens, time and monetary cost             |
| Confidence       | Uncertainty interval and limitations                |
| Status           | Exploratory, provisional or confirmed               |
| Known failure    | Counting argument omitted                           |
| Next useful test | Test a different model or change one pipeline stage |

The values above are illustrative except for the 2-of-5 example, which reflects the kind of threshold already being used in the calibration work—not a claim about a particular existing capability.

Crucially, a capability should not be represented as a permanent property of a model. It is a claim about a configuration performing a task under particular conditions. Changing the source document, prompt, model version, effort or evaluator can change that claim.

I'd also distinguish three states:

- Demonstrated: sufficient evidence to meet the defined quality bar.
- Uncertain: promising results, but insufficient evidence to recommend it confidently.
- Not demonstrated: the configuration has not met the bar, or there is insufficient evidence to establish that it can.

That last distinction matters: a model that hasn't been tested is not a model that has failed.

## 3. Measure the failure modes, not just the outputs

One particularly useful reference is SynthesisBench. Its public work goes beyond aggregate rankings to examine failure modes, trajectories and the distinction between finding evidence and correctly interpreting it. It also highlights cases where additional thinking changes the model's approach without improving correctness.&#x20;

[image](https://www.google.com/s2/favicons?domain=https://synthesisbench.com\&sz=32)

Synthesis Bench

+2



That is directly relevant to Exegenius. I would track whether configurations:

- Find the relevant material but fail to derive the result.
- Derive the result but fail to preserve it in the final wiki.
- Produce correct intermediate work that gets removed during revision.
- Cover many results while omitting the central argument.
- Produce convincing explanations containing mathematical errors.
- Pass automated checks without improving a reader's understanding.

These failure modes should be linked to individual pipeline stages and to the final outcome.

This would help distinguish, for example, a model that is poor at extracting information from one that extracts it perfectly but cannot explain its significance.

## 4. Separate model effects from system-design effects

This is where the project could become particularly useful to you as a developer.

Suppose Opus outperforms Sonnet on extraction. There are several possible explanations:

- Opus is intrinsically better at the task.
- Opus benefits more from the current extraction prompt.
- The task decomposition suits Opus particularly well.
- The evaluation favours the style of answer Opus produces.
- The difference is ordinary run-to-run variation.

The dashboard should help you formulate experiments that distinguish these explanations.

I would make the dimensions explicit:

Model and version

Which model was used?

Effort

How did more or less reasoning affect performance?

Agent and pipeline design

Which prompts, handoffs, checks and revision rules were used?

Evaluation method

What was measured, against which reference, and by whom?

The important caveat is that a visualisation alone cannot establish causality. Controlled comparisons are needed, and changing one variable at a time is often a good starting point. Interactions between variables may then justify more targeted experiments.

## 5. Don't assume all tasks share the same performance curve

Your intuition that greater effort will generally improve performance is a reasonable hypothesis, but I would not impose monotonicity as a rule.

The same model might improve steadily in extraction, plateau in coverage, and become less reliable in a task involving ambiguous judgements. More effort can change a model's policy, not just make its existing policy more thorough. SynthesisBench reports examples of exactly this kind of behaviour.&#x20;

[image](https://www.google.com/s2/favicons?domain=https://synthesisbench.com\&sz=32)

synthesisbench.com

+1



Similarly, I would not assume that a model's overall ranking transfers between tasks or domains.

A future view could show separate curves for:

- Technical extraction.
- Mathematical derivation.
- Explanatory writing.
- Auditing and revision.
- Less demanding intellectual writing.
- Reader-side understanding.

This would let the evidence determine whether a configuration is broadly capable or unusually good at a particular kind of work.

## 6. Design for new model releases from the beginning

Fable is an excellent reason to build this, but the design should outlast any particular release.

Every new model should enter the landscape with its version pinned and its results compared against established baselines. A new model should not inherit a predecessor's scores or silently replace them.

I'd also distinguish:

- Regression tests: does the new model still pass existing quality bars?
- Capability discovery: does it succeed at tasks that were previously beyond the system?
- Efficiency comparisons: can it achieve existing quality at lower cost?
- Novelty tests: does it discover a new method or preserve a derivation that previous configurations missed?

The last category is particularly relevant to your broader interest in research taste and mathematical creativity. Those capabilities will need different evaluations from ordinary extraction or coverage.

## 7. The most important addition: show what we don't know

I would make uncertainty a first-class feature of the interface, not a footnote.

For every plotted result, the user should be able to tell:

- How many runs were performed.
- How variable the outcomes were.
- Whether the comparison was controlled.
- Whether the evaluator was independent.
- Whether the reference itself has been validated.
- Whether the result generalises beyond one paper.
- Whether the finding has been confirmed on fresh runs.

This prevents the visual authority of a chart from exceeding the strength of the evidence.

A configuration with 5 successful runs out of 5 is encouraging, but it is not proof of universal reliability. And a configuration that scores highly on one mathematical paper may not be the best choice for another domain.

## 8. What I would actually build first

I would keep the first milestone deliberately small.

1. Make the existing Capability Map machine-readable. Use the registry as the source of truth and expose the provenance of each claim.
2. Build the model × effort plot and capability heatmap. Start with the existing E1/E3 results, showing uncertainty and sample sizes rather than pretending the evidence is complete.
3. Add metric and task filters. Allow comparison by extraction, coverage, derivational fidelity, central-chain reconstruction and cost where those measurements exist.
4. Make each result inspectable. Clicking a point or cell should lead to the relevant experiment record, protocol, raw evidence and failure analysis.
5. Use the first visualisation to choose the next experiments. Identify the comparisons that would most change a real decision, rather than automatically filling every possible model–effort combination.

I'd defer elaborate composite scores, automatic rankings across incomparable tasks, and a large public dashboard until the underlying evidence is mature enough to support them.

One final thought: the public-facing interface could be a very good way to explain Exegenius itself. Rather than claiming that it generates high-quality mathematical wikis, you could show precisely which parts of that claim have been tested, what configurations work, where they fail, and what remains unknown.

That makes the performance landscape both a development tool and an honest account of the software's capabilities.
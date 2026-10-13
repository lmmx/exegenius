I think this is a genuinely useful extension of the calibration work, and importantly, it's not just a leaderboard for comparing Claude models. You're describing a way to make Exegenius's capabilities, its design choices, and the trade-offs between model configurations visible—both to you as its developer and to other people trying to understand what the software actually does well.

The distinction I'd preserve is that you're interested in two things at once:

- A performance landscape: which model and effort configuration performs best on which tasks, at what cost, and with what degree of confidence?
- A decomposition of the system: which parts of Exegenius benefit from which capabilities? For example, if Opus is better at extraction, is that because of the model itself, the effort setting, the prompt, or their interaction?

Those are related, but they aren't the same question. A leaderboard can rank configurations; a capability map can help explain why their performance differs and which design choices are worth keeping.

I'd save this as a separate development plan, not fold it into the current calibration work. It can build on the registry and Capability Map already being developed, without interrupting the ongoing experiments.

Here's the plan I'd preserve for the future Fable session.

# Development plan: Exegenius Model–Effort Performance Landscape

**Status:** Proposed; revisit when Fable becomes available.

## Motivation

Exegenius currently has a growing body of experiments comparing model configurations, agent definitions, effort settings and pipeline behaviours. The Calibration Protocol, Evaluation Registry and Capability Map are the beginnings of a systematic account of what works.

The next step is to make this evidence visible and interpretable, both for development and for people outside the project.

I want something closer to an interactive performance landscape than a conventional leaderboard. The objective is not simply to rank Claude Haiku, Sonnet and Opus, or to compare models from different providers. It is to understand what different configurations can accomplish, where they excel or fail, how much they cost, and what this tells us about the design of Exegenius itself.

## 1. The central visualisation

Represent each model as a series, with its available effort settings as points along that series.

For example, a model's series might show medium, high and other available effort settings, with each point displaying measured performance on a selected task or metric. The visualisation should make it possible to see:

- Whether increased effort improves performance, and by how much.
- Where different models overlap in capability despite having different effort settings.
- Which configurations are best for particular tasks, rather than assuming one universal winner.
- Whether improvements in quality justify their additional time and token cost.
- Where results are too uncertain to support a ranking.

Effort is ordinal within a given model where the provider supports that ordering; it is not a common numerical scale across different models. Do not assume, for example, that Sonnet-high and Opus-medium represent equivalent amounts of reasoning.

Overlapping performance points are expected and informative. Use confidence intervals, slight horizontal offsets where appropriate, interactive details, or small multiples to make overlapping configurations distinguishable without implying false precision.

## 2. Decompose performance by capability

The system should not reduce Exegenius to a single aggregate score.

Develop a set of independently interpretable metrics, informed by the existing calibration work. Potential dimensions include:

- **Extraction:** accurate recovery of results and technical details from the source.
- **Coverage:** which important results and concepts are captured.
- **Derivational fidelity:** whether the explanation follows the paper's actual reasoning.
- **Central-chain reconstruction:** whether the essential steps of a key argument survive.
- **Factual and mathematical correctness:** including major errors and invalid inferences.
- **Explanatory quality:** whether the output makes the reasoning intelligible rather than merely stating conclusions.
- **Reader understanding:** whether readers can answer questions, reconstruct arguments or achieve held-out goals.
- **Operational performance:** completion rate, retries, failures and reliability.
- **Cost and latency:** tokens, money, elapsed time and other relevant resource constraints.

These dimensions are not interchangeable. In particular, strong extraction does not establish strong explanation, and a polished output does not establish reader understanding.

Some measures may be hard quality gates rather than quantities to average. A configuration that produces a broad overview with a major mathematical error should not necessarily rank above a narrower but correct explanation.

## 3. Make the pipeline's capabilities visible

Extend the performance landscape beyond whole-system configurations where the evidence permits.

Investigate which models and effort settings work best for particular stages, such as extraction, planning, derivation, writing, auditing and revision.

The goal is to understand the contribution of each stage and the interactions between them. For instance, if Opus appears better at extraction, determine whether that advantage persists when the task, prompt, source material and evaluation method are held constant. If a cheaper configuration performs just as well on a particular stage, that could inform a more efficient execution profile.

Distinguish measured component performance from conclusions about causal contribution to the final wiki. A component can score well in isolation yet fail to improve the complete pipeline.

## 4. Design the experiments to support the visualisation

Build on the existing Calibration Protocol and Evaluation Registry rather than creating a disconnected benchmarking system.

Each result should identify, where available:

- Model identity and version, provider, effort setting and relevant configuration.
- Task type, source document, prompt, agent definition and pipeline version.
- Evaluation rubric, judge identity and version, and reference material.
- Number of runs, variation across runs, confidence intervals and missing data.
- Token usage, estimated or measured cost, latency and operational failures.
- Whether the result was exploratory, used for screening, or obtained in a confirmatory evaluation.

Use paired comparisons when feasible, with common tasks and evaluation conditions. Screen promising configurations cheaply, then allocate more runs to comparisons that could change a decision. Validate apparent winners on fresh runs rather than ranking configurations from whichever small sample happens to look best.

As new model releases arrive, preserve historical results and version identities. A new model should enter the landscape without silently replacing the evidence for its predecessor.

## 5. Design the interface for exploration, not just ranking

Use the SynthesisBench leaderboard as a reference point, not a template to copy:

[https://synthesisbench.com/leaderboard](https://synthesisbench.com/leaderboard)

The interface should make it easy to:

- Select a task or capability and compare configurations.
- Switch between quality, cost, latency and reliability.
- Inspect uncertainty and the underlying sample size.
- Compare configurations side by side.
- Drill down from an aggregate view to individual experiments and evaluation evidence.
- Identify where configurations are indistinguishable, where trade-offs occur, and where evidence is inadequate.
- Explore results across model releases without confusing different benchmark versions.

The model-versus-effort plot is the starting point. Depending on the data, heatmaps, small multiples, Pareto frontiers and stage-by-stage breakdowns may communicate particular comparisons more clearly. Do not force every question into one chart.

## 6. Use the landscape to evaluate design choices

The purpose is not merely to publish scores. It is to help answer engineering questions.

Examples:

- Is a dedicated planner actually better than a general-purpose agent with an appropriate prompt?
- Does high effort improve extraction, derivational fidelity, both, or neither?
- Can a cheaper model achieve a particular quality bar on a subset of tasks?
- Does a better plan translate into a better final wiki?
- Which stages benefit most from a stronger model?
- Does a pipeline change improve reader understanding, or merely the metric it was designed to optimise?
- How much of a reported advantage survives fresh runs and changes of source document?

The visualisation should expose these questions and their evidence, not automatically claim to answer them.

## 7. Scope and implementation

Start by examining the existing registry, Capability Map, calibration records and report formats. Identify the smallest useful implementation that can be built from the evidence already available.

Do not begin by running an exhaustive model-by-effort-by-stage matrix. First establish which dimensions are recorded consistently, which metrics are comparable, and which historical results can legitimately be plotted together.

Implement an initial view using the existing data, with explicit treatment of missing values, incompatible evaluations and uncertainty. Then identify the most valuable additional experiments needed to fill gaps.

Keep the benchmark reproducible, inspectable and maintainable. Prefer transparent metrics and links to evidence over an impressive but opaque composite score.

## Intended outcome

A living, evidence-backed map of Exegenius's performance across models, effort settings, tasks and pipeline stages.

It should help me see where the software is strong, where it fails, which design decisions are justified, what configuration to use for a given task and resource budget, and what experiment would most improve our understanding next.

It should also give an outside reader a credible account of what Exegenius does well and how that knowledge was established.

The longer-term aim is to make the landscape evolve with new model releases, while allowing the project's conclusions to be challenged and its historical claims to remain auditable.

One final design choice I'd preserve: don't make the first version wait for Fable or for a complete benchmark suite. Build the initial visualisation from the existing calibration evidence, clearly mark its limitations, and let Fable help identify what should change. That gives you something concrete to inspect and critique, rather than asking the new model to design an entire benchmarking system in the abstract.
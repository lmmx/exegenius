# The metrics-viz conversations: a model × effort performance landscape for Exegenius

These five files record a short conversation between the author and ChatGPT, in order, on 13–14 October 2026. It concerns a way to see what different Claude configurations achieve at each part of Exegenius's pipeline. The author's starting point (file 01):
- a plot with models as line series and effort levels as the points along each line;
- in the spirit of a leaderboard, but limited to Haiku, Sonnet and Opus (and perhaps Fable) in several configurations;
- showing what each achieved better or worse on;
- doubling as a picture, for others, of what each part of the system does;
- updating with model releases.

**The author's own observation in file 01:** "opus is much better at extraction". It is the kind of claim the landscape would let anyone check.

## The progression

| # | File | What happens |
|---|---|---|
| 01 | `user-model-effort-leaderboard-idea` | the idea: per-model line series over ordinal effort; ties and overlaps to be shown; a public decomposition of the system; kept for development once Fable is released |
| 02 | `reply-performance-landscape-plan` | a development plan with seven parts (below) |
| 03 | `reply-five-views-and-capability-map` | five views, each answering one question; the Capability Map as the centre; failure modes; model effects against design effects; effort curves need not be monotone; releases; showing what is not known; a first build |
| 04 | `user-note-unrecorded-prompt` | the author notes the prompt before file 03 was not recorded ("can you give me more details … you used capability map before i think"), and asks whether the live run has a branch |
| 05 | `reply-linking-journal-registry-map` | how journal, registry, Capability Map and landscape connect; check where the live work sits before building anything |

**File 02's seven parts:**
1. the central visualisation;
2. decomposing performance by capability;
3. making the pipeline's capabilities visible;
4. designing experiments to support the visualisation;
5. an interface for exploration, not ranking;
6. using the landscape to evaluate design choices;
7. scope and implementation.

**File 03's five views:**

| View | Question it answers |
|---|---|
| A. Model × effort plots | how does performance change with effort? |
| B. Capability heatmap | who is good at what? |
| C. Quality against cost | which configurations are worth their price? |
| D. Pipeline capability map | where does each capability come from? |
| E. Evidence and failure explorer | why should I believe this result? |

**The threads that stay distinct across the conversation:**
- **A map, not a ranking.** The aim is to see which configuration is good at which task, at what cost, and how certain that is; there is no single winner.
- **Model effects against design effects.** If Opus is better at extraction, is that the model, the prompt, the decomposition, the judge, or noise? A chart cannot settle it; controlled comparisons can.
- **What is not known.** Untested is not the same as failed. Sample size, controls, judge independence and generality are part of every point.
- **Built on what exists.** The Calibration Protocol, the Evaluation Registry and the Capability Map are the source of truth, with no parallel benchmarking system.

## What exists in the repository (on `master`, as of 2026-10-14)

The conversation anticipated most of what follows. The calibration programme has since built it.

| The conversation's need | Where it now is |
|---|---|
| records of each configuration, version, task, runs and evidence | `calibration/registry/` (`specs/`, `experiments/*.json`, `judging.json`, `INDEX.md`), built mechanically from git refs; every value is labelled measured, estimated, reported or unavailable |
| what each configuration has demonstrated, with uncertainty | `calibration/capability-map.md`, with exact (Clopper–Pearson) intervals per calibre |
| quality bars as hard gates | `calibration/PROTOCOL.md` §4 (calibres: Overview, Technical, Research) |
| cost, separated from quality | `calibration/budget.md` and `calibration/tools/spend.py`: token components kept apart, with no invented conversion to cost or quota |
| screening against confirmation, and adaptive repeats | `calibration/PROTOCOL.md` §6 and §6a |
| a model × effort screen above `high` | `calibration/plans/S2.md`: Sonnet and Opus at `xhigh` and `max`, one run each, in progress |
| Fable, when it arrives | `calibration/plans/FB1.md`: a proposal; Fable must earn its place against the strongest existing configuration |

## Speculative journal entry derived from this archive

`docs/journal/2026-10-14-speculative-model-effort-landscape.md` covers what a first landscape can plot honestly from the registry today, and what it cannot. It is a proposal, not a result.

## Uncertainties in the record

- **File 03 has no recorded prompt.** File 04 gives its gist from memory.
- **The SynthesisBench leaderboard** is the author's reference point, which they did not consider well done; the archive's account of SynthesisBench's failure analyses is ChatGPT's and has not been checked here.
- **"Opus is much better at extraction"** is the author's impression. The registry has no controlled comparison of extraction alone. The nearest evidence:
  - S1 and E5 measured whole plans;
  - in E5, Opus at medium stated as many core results as Sonnet at high, but derived fewer.

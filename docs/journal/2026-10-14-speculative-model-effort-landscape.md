# 2026-10-14 (speculative): A first model × effort landscape, from the registry as it stands

**Status: a proposal derived from the metrics-viz conversations (`docs/chats/metrics-viz/`). Not built. It can be built from existing records; the right moment is after S2 is judged, so that its `xhigh` and `max` points are included.**

## What can be plotted honestly today

All of it is planning, on one paper (`paper.tex`, "Integer multiplication below n log n"), from `calibration/registry/experiments/*.json` and `calibration/registry/calibre-pass-rates.txt` on `master`.

| Model | Effort points with data (runs) | Source |
|---|---|---|
| Sonnet | medium (5 in E3; 21 earlier, older binaries), high (5 + 5 in E1; reused across E3–E5), high with the key-step instruction (5 in E4, 8 in E6), high with E7's instructions (8 in E7), `xhigh` (1, S2), `max` (1, S2) | E1, E3–E7, S2 |
| Opus | medium (2 in S1, 5 in E5), high (2 in S1), `xhigh` (1, S2), `max` (1, S2) | S1, E5, S2 |
| Haiku | high (2 in S1) | S1 |

**Three things the conversation did not anticipate:**
- **The instructions are a third axis, and the biggest one.** On this paper, the change from the baseline argument instructions to E7's moved the central gate from 0 of 5 to 8 of 8, at one model and effort. Any of the model or effort differences seen so far was smaller. A model × effort plot that mixes instruction versions would mislead, so the first landscape needs a facet per instruction version, and most cells are empty in all but one facet.
- **The judges are a source of variation of their own.** The same eight plans changed central-gate verdict in 3 of 8 cases between two batches (E7). Points from different judging batches are not directly comparable. The first landscape should mark the batch, and show central-step counts, which are steadier, beside gate pass rates.
- **The tasks differ.** E4, E6, E7 and S2 ran only the argument task, on extractions shared within a pair. E1, E3, S1 and E5 ran the whole of planning. A point is a configuration *on a task*, as file 03 says. In this registry the task is not always the same planning stage.

## The first version

1. **One page per instruction version,** with:
   - x: effort (ordinal within a model only);
   - y: central-derivation count, median and range per point, every run shown;
   - one line per model;
   - point size: number of runs;
   - colour: judging batch.
2. **The same layout for each calibre's pass rate,** with exact intervals. Untested cells are drawn as untested, not as zero.
3. **A cost panel per point,** from the registry:
   - seconds;
   - the four token components (uncached input, cache read, cache write, output), kept separate as `calibration/budget.md` requires;
   - no composite.
4. **A link from every point** to its experiment record, plan and journal entry: view E in file 03, at its smallest.

**Not in the first version:** stage-by-stage decomposition (view D), because only planning has evidence. Writers, auditors and support checks have none (`calibration/capability-map.md`, "Open" item 5).

## What it would be used for

- **Choosing the next experiment** (file 03, "use the first visualisation to choose the next experiments"). The empty cells adjacent to the best configuration are the cheapest informative tests, for example Opus at high with E7's instructions.
- **Showing others what Exegenius's planning does at each configuration,** and what is not yet known.
- **Taking in Fable without disturbing earlier points:** each model and version is its own series, pinned.

## Dependencies

- **S2's judging:** the `xhigh` and `max` points, and their cost, come from S2. Plotting before then shows only `medium` and `high`.
- **A small export:** the registry is JSON already. A short script, or a static chart (Vega-Lite or similar) reading `experiments/*.json`, is enough for a first version.
- **A decision on publishing:** whether the landscape is a page in the repository's docs site or a separate artefact is the author's call.

## Current State
- The registry holds planning results for Sonnet at medium, high, `xhigh` and `max`, Opus at medium, high, `xhigh` and `max`, and Haiku at high, on one paper, with three instruction versions and eight judging batches
- No visualisation of it exists

## Missing
- S2's judgements, which give the `xhigh` and `max` points their quality values
- Any evidence for stages other than planning
- A second document

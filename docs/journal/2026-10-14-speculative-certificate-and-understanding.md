# 2026-10-14 (speculative): Certificate and understanding, set against what Exegenius now measures

**Status:** a proposal, derived from strand A of the Leviathan conversations (`docs/chats/leviathan/`, files 01–02 and 14).
- **What is ChatGPT's:** the essay's reading. It has not been checked against Gamburd's text here.
- **What is the repository's own record:** the evidence cited from Exegenius.

## The idea from the archive

**The essay's distinction, as ChatGPT reported it:** a certificate that a result is correct is not a demonstration that lets someone understand why it is correct.

ChatGPT proposed four layers, each of which does not guarantee the next:
1. **Certification:** is the result correct, and supported?
2. **Reconstruction:** is the argument that establishes it recovered?
3. **Understanding:** can a reader follow it and see why the conclusion follows?
4. **Insight:** does the reader see why this approach matters?

It also suggested evaluating "the path, not the destination". File 14 asked the question that matters for the software: does Exegenius represent the *structure* of an explanation, or mostly the *information* an explanation contains?

## What the repository now shows, layer by layer

This is not a test of the essay. It is a reading of evidence gathered since, for other reasons.

| Layer | Exegenius instruments today | What the evidence shows on the one paper tested |
|---|---|---|
| Certification | content judge's `wrong` and major errors; the pipeline's citation-support check (`support.json`) | plans are rarely wrong: at most one plan per arm with a major error in E4–E7 |
| Coverage | content judge's stated counts | **saturated at high effort**: 17–19 of 19 core results stated in nearly every Sonnet-high plan (E3–E7) |
| Reconstruction | central-derivation rubric (C1–C9) on plans; chain walk (`calibration/fidelity/chain-walk-v1.md`) on wikis | the separation is measured on plans |
| Understanding | the simulated readers (goals, navigation; `[simulation]`) | **never measured in the calibration programme**: every calibration run sets `[simulation] rounds = 0` |
| Insight | the argument task's `salient` field; the aha-moments journal (2026-10-11) | no instrument |

**How coverage and reconstruction separated on plans:**
- E3 found high-effort plans stating all core results while passing the central gate in 0 of 5.
- E7 reached the gate in 8 of 8 plans only after instructions asked for the key step's identities and exceptions.

**Where it stands on wikis:**
- Only the planner-handoff wiki carries the central chain (chain-walk baseline), and it got there through writers' rewrites, not through its plan (T1).
- No full wiki has yet used E7's plans; W1 was cancelled.

## The reading this suggests (interpretation)

**The calibration programme rediscovered the first gap empirically.**
- **The gap:** stating results is not reconstructing the argument for them, which is the certificate/demonstration distinction applied to plans.
- **How it was closed:** E7 closed it on one paper, by asking for the reasoning that makes each counted step hold. That is close to the essay's demand that a demonstration show why, not only that.

**The next gap is unmeasured.**
- **The gap:** reconstruction against understanding.
- **What could measure it:** the simulated readers, which already exist.
- **Why it hasn't been measured:** they have been switched off in every experiment that measured plan quality.

**Does Exegenius represent structure or information? (File 14's question.)**
- The plan does carry some structure:
  - the argument tree is a structure of steps, each with a method (`show`) or a quote (`infer`);
  - `salient` marks what a reader should find striking.
- What it does not represent is which step is the *key* one. E4–E7's instruction now asks for it, but nothing in the plan's schema records it.

## What would test this (proposals, cheapest first)

1. **A full wiki from E7's instructions** (the cancelled W1), with:
   - the chain walk, to check reconstruction survives the writers;
   - the simulated readers turned on, to see whether readers reach the key step's consequence.

   This is the first measurement of the understanding layer.
2. **A key-step field in the argument schema.** The plan would name its key step, so that writers, auditors and the chain walk can check that step specifically. This is a software change for the author's review, justified only if test 1 shows the chain is lost after planning.
3. **An insight measure.**
   - The reader's task: after reading, say why the paper's approach works where the known one did not.
   - Its status: harder to judge, and later.

## Limits

- **Philosophy, not evidence:** the essay is philosophy, and the layer mapping is an interpretation.
- **One paper:** the evidence is from one paper.
- **Plans, not readers:** "understanding" here would be measured by simulated readers, not people.

## Current State
- The calibration programme's results separate coverage (saturated) from reconstruction (gate 0 of 5 → 8 of 8 with E7's instructions) on one paper's plans, matching the certificate/demonstration distinction this archive drew from Gamburd's essay
- No experiment has measured understanding: the simulated readers have been off in every calibration run

## Missing
- No full wiki from E7's plans, so no evidence that reconstruction reaches the pages
- No reader-side measure in the calibration programme
- The essay's own argument has not been read here beyond ChatGPT's account of it

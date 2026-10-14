# 2026-10-14 (speculative): A reusable method for sequential, relational thinking

**Status: a proposal derived from the Leviathan conversations (`docs/chats/leviathan/`). Nothing here has been built or tested.**

## The idea

**What the author wanted from the Leviathan exercise:** a way of thinking that works like this:

> think about A on its own terms … now, how does that change how you think about B? … now think about A and B together … now bring C into it

**What ChatGPT produced instead,** twice:
- **Merging:** it merged the strands into one programme (file 12).
- **Dropping:** it dropped strands to keep a single line (files 15, 17).

The author's corrections in files 13, 16 and 18 state the rules. The aim is to make those rules a method that any session can follow and anyone can check.

## The structure

**A run is a sequence of stages, one strand per stage, each in its own turn:**
- **Solo stage:** "Think about A on its own terms." It develops A's internal logic, consequences and open questions, and nothing from B or C.
- **In-light-of stage:** "How does A change how you think about B?" B is re-examined with A in view, and the stage records what remains distinctive about B.
- **Together stage:** "Now A and B together." It covers interactions, tensions, dependencies and incompatibilities, with no requirement that they converge.
- **Extension:** "Bring in C." It covers what changes, including whether C challenges the framing so far.
- **Emergence:** "What follows that depended on the sequence?" Each new idea names the stages it came from.

The order is part of the design, and so is revisiting a stage or branching when an encounter opens a new direction. The author's own prompts in the archive (01, 03, 05, 07, 09, 11) are an instance. File 19's prompt runs the same idea within one turn.

## Making a collapse visible

**Each solo stage ends with a short list of the strand's distinctive claims:** what this strand says that the others do not.

**At the end, each claim is marked as:**
- **kept;**
- **changed, with the stage that changed it and why;**
- **dropped:** an explicit decision, or silently.

**The measure:** a run has collapsed when claims were dropped silently, or when the strands' claims were replaced by a common rewording.

This check has the same shape as one Exegenius already uses. The chain walk (`calibration/fidelity/chain-walk-v1.md`) follows each step of the paper's central argument through the wiki and marks it reconstructed, mentioned, absent or wrong. Here the "steps" are the strands' distinctive claims, and the "pages" are the stages. The judging lessons carry over:
- three judges with a majority where a decision rests on it;
- marks kept per item rather than as one score.

## A first test (small, can run now)

| | |
|---|---|
| **Inputs** | the Leviathan strands, from the archive's sources |
| **Arms** | (a) file 19's single-turn prompt; (b) the same strands staged one per turn, with the claim lists |
| **Runs** | one run of each, on the same model and effort |
| **Measure** | for each run, the share of solo-stage claims kept or changed with a reason, against those dropped silently, judged blind |
| **What it would show** | whether staging the strands, rather than describing them in one prompt, is what prevents the collapse |
| **Cost** | two long sessions and a few judgements, proportionate to a screen (`calibration/PROTOCOL.md` §6a) |

## Where Exegenius might use it

Most of these are later and speculative:
- **Several sources,** for example a paper and its follow-up: "explain A, then B in light of A, then their relation", instead of one merged plan.
- **The development process itself:** the archive's exercise is a candidate for how future sessions approach open questions.

## Current State
- The method exists only as a description derived from the Leviathan conversations; no run has used it
- The archive's file 19 is the nearest existing prompt; it is single-turn, not staged

## Missing
- No run of the staged method, and no measure of collapse applied to any run
- No decision on whether the claim lists are written by the model or by a separate reader

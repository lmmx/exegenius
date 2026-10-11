# 2026-10-11: Reduction

The baseline pilot reproduced the divergence of the 0.5.0 run in 38 tasks (2026-10-11-baseline-pilot). This entry plans one pass of changes to stop it, to be checked with a single pilot run afterwards. With one run, the changes cannot be measured one at a time, so they are chosen on the records rather than tried in turn.

## Where the length came from

The write instructions went from 215 words at 0.1 (b058d18) to 603 at 0.5.0, and the median write task from 811 words to 1,912. Each addition asks for content on the page:
- **An opening:** what the concept is for in the argument.
- **The method:** "explain how its method shows its claim. Give the reader every motivation the task marks as not stated by the sources".
- **Symbols:** what every symbol denotes or counts.
- **Relations:** for each prerequisite, "say in a sentence what this page uses it for".
- **Citations:** a quotation from the source after each specific claim. The 0.1 pages carry none; the 0.5.0 run's carry 771.
- **The write task:** an argument section with the step's method, motivation and any striking feature, which the writer is asked to explain.

No instruction asks for brevity, and first drafts have no length limit. The 0.1 pages have a median of 221 words. The baseline pilot's first drafts have a median of 621.

## The principle: structure shapes the writing, not the page

Before the run, the author asked ChatGPT whether the new structure would overload the reader. Its answer separated two uses of the machinery:
1. **Shaping what gets written.** Symbol bindings, relations, the argument plan and goals help the writer decide what to say and in what order.
2. **Exposing the structure.** Graphs, relation labels, plan trees and provenance appear on the page.

It argued for the first by default, with "a high threshold for becoming visible, but a low threshold for improving the prose". It also separated two kinds of completeness:
- **Structural completeness:** the system accounts for every dependency the argument needs. This can be ambitious.
- **Expository completeness:** the reader is told what they need, in order, at the right level of detail. This should be selective.

As an example: "A proof plan might record ten dependencies, … while the final prose explicitly explains only the three that are genuinely difficult for the intended reader." Its rule for the writer was to write for the reader's next uncertainty: what the reader needs now, what might stop them, and whether one natural sentence resolves it.

0.5 did the opposite inside the page body. Every structure became an obligation on the prose: every motivation, a sentence per prerequisite, a citation per claim. The audits then checked the prose against the same standard of completeness: any claim "the reader cannot check from what the page gives" is a gap. The overload the author feared for readers struck the simulated reader first. Each rewrite added the explanation an auditor asked for, and the next auditor found new claims in it to report.

The changes below apply the principle on both sides. The writer gets the structure as material to choose from. The auditor reports only what would stop this reader at this point.

## Changes

### The writer: selective

1. **Write for the reader's next uncertainty.** The write instructions are rewritten around one rule: explain what this reader needs to follow the page, at the point they need it.
   - The argument step, motivations, striking features and the pages that build on this one are given as material for deciding what to say, not as items to cover.
   - The page states the main motivation of its own step in a sentence, and leaves the rest unless the reader would be stuck without it.
   - Transitions the reader would find hard are explained; familiar ones are left to ordinary sentence structure.
2. **A length limit on first drafts.** Submit rejects a first draft over 350 words, counted without mathematics or citations. The limit sits above the 0.1 median of 221 and well below the baseline's 621.
3. **An enforced rewrite budget.** Submit rejects a rewrite over its budget (the previous version's words plus a quarter, at least 50), where it now warns.
4. **Relations from the linking sentence.** The relation for a prerequisite is taken from the sentence that links it. The instructions no longer ask for a sentence about each prerequisite. The check that each prerequisite page has a relation stays (2026-10-11-relations).
5. **Citations that leave the sentence whole.** Submit rejects a citation mark after "that", "as", "states", "says", "is", "gives", "reads" or a colon. The instructions ask for citations of results and constants a reader may want to check, not of every claim taken from the source.
6. **Source passages in the write task.** The write task carries the paragraph around each evidence quote, so writers need not read the paper to find them. This is for tokens and time, not convergence (2026-10-11-wall-time).

### The auditor: what stops the reader here

7. **Report what would stop this reader at this point.** The step criterion keeps "a claim the reader cannot check", with two exceptions:
   - a step the page attributes to the source with a citation is followable by reference;
   - references to the documents themselves ("the paper", "the target") are not terms.
   The audit wording on what the reader profile covers is kept (2026-10-11-graph-growth).
8. **Step rewrite notes offer the shorter fix.** A step gap's note asks the writer either to derive the step or to attribute it to the source with a citation, whichever adds fewer words.

### The loop: safety nets

9. **A stop rule, on by default.** After an audit round, if step or symbol gaps on the pages audited in both rounds did not fall by at least half, the audit rounds end and `status` says why. The thresholds are those of 2026-10-11-convergence: the 0.1 run met them each round, and the 0.5.0 run and the baseline failed them in their first comparison.
10. **Overload rounds limited to two,** as support checks are.

### The agents

11. **Subagent definitions written by `init`.**
    - `.claude/agents/exegenius-task.md` and `.claude/agents/exegenius-reader.md`, with `model` (`inherit` and `haiku`) and `tools` limited to Read, Write and Bash.
    - `next` names the definition for each task.
    - Three of the baseline's 38 subagents called a cloud-session tool despite a prompt limiting them to Read, Write and Bash (2026-10-11-baseline-pilot). A definition's tool list removes such tools; a prompt does not.

## What this pass leaves out

The reader-facing chrome stays as built: "Before this page" with a predicate per prerequisite, "Where this fits", and the folded symbols (2026-10-11-layering). ChatGPT's table advises against "labelling every link with a technical relation type". The audits never see the chrome, so the pilot cannot measure a change to it; the simulated readers do see it, and are the place to test one.

## The check

The checks run in two steps, the free one first.

**Free checks first.** The new submit rules run over the pages already recorded, at no model cost: the baseline pilot's 21 page versions and the 0.5.0 run's 142 pages. They count how many pages exceed the limit and how many citation marks the sentence check rejects. Each rule also gets a unit test.

**Then one pilot,** run by the author in a fresh Haiku session in the author's container, under the protocol of 2026-10-11-baseline-pilot, except for two changes:
- `pages = 5`, to keep it under half an hour;
- the subagent definitions in place of prompt instructions.

The pass works if, on the pages audited in both rounds:
- step and symbol gaps fall by at least half;
- new gaps are a minority;
- the first extend adds well under one page per page audited.

The risk on the other side is auditors that under-report once steps can be followed by reference. The pilot measures convergence, not teaching. The simulated readers measure teaching, on a later run.

## Current State
- The write instructions ask for every unstated motivation, a sentence per prerequisite and a citation per specific claim, with no length limit on first drafts (exegenius-core/src/write.rs)
- The audit instructions count as a gap any claim the reader cannot check from the page, including steps the page cites from the source (exegenius-core/src/audit.rs)
- The rewrite budget warns and does not reject (exegenius-core/src/write.rs)

## Missing
- Every change listed above

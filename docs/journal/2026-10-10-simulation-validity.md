# 2026-10-10: Simulation validity

> Superseded in part by 2026-10-10-revised-v1-design, which wins where the two conflict.

How a simulated reader can produce evidence about a page, and not just reflect what the
model already knew. R1, R2 and R3 are the three design rounds described in
2026-10-10-origin.

## The problem in one line

The simulated reader is a model that has read most of the internet. Whatever it answers
after reading a page, the default explanation is "it already knew".

## R1: closed-book reader, leakage control, navigation

- **Closed book.** The reader may answer only from pages it has visited, and every claim
  it makes must cite a span from one of those pages.
- **Leakage control.** Each goal question is asked twice: once with no pages, and once
  with the reader's visited path. A page earns credit only for the difference. A question
  the model answers correctly with no pages tells you nothing and is dropped.
- **Navigation as the game.** The reader acts as an agent with four actions: read, follow
  a link, backtrack, give up. Its reward is goals answered; its cost is hops and tokens.
  Its trajectories are logged, and the places where it backtracks or gives up mark
  confusion hotspots.
- **Information foraging.** In Pirolli & Card's information foraging theory (1999),
  readers follow "information scent", and in a wiki the anchor text of a link carries
  that scent.
- **Human ground truth.** The author logs their own confusion while reading, even as a
  bare `?` in the margin, and those logs calibrate the simulator. Without them the system
  optimises a proxy that nothing anchors.

## R2: a fuller ESS and a validation protocol

R2 listed six fields for the Epistemic State Specification:

1. known concepts,
2. unknown concepts,
3. misconception profile,
4. available evidence,
5. state-transition rules (what counts as having learned a concept),
6. response constraints (citations, uncertainty, reporting confusion).

R2 then made the point that drives the rest of this entry: **"unknown" does not mean
"unable to answer"**. Telling the model not to use a concept it knows does not make it
comply.

R2 proposed four tests:

- **Prior-knowledge test.** Ask the questions before any exposure. This shows what the
  model can answer unaided. It does not show what a human novice would know.
- **Evidence-constrained test.** Require cited spans, and check that each span actually
  supports the answer, not just that it mentions the topic.
- **Transfer test.** Ask questions that apply a concept to a new case. Restating a
  definition does not count.
- **Human calibration.** Compare where simulated readers fail with where real readers
  fail, on the same material and tasks.

R2 also criticised R1's with-pages minus no-pages score:

- the difference can come from the prompt context, from sampling variation, or from
  retrieval cues, not only from learning;
- a page that states the answer without explaining it scores high.

The fixes R2 gave were matched questions, fixed model settings, repeated trials, and
keeping recall separate from transfer. R2 added that one expert's confusion log is not the
intended audience's.

## Leakage has two sources, and they need different controls

R1 and R2 treat leakage as one thing. It has two separate sources:

1. **Parametric leakage.** The model knows the DFT from training.
2. **In-context leakage.** The page states the goal's answer word for word, so the
   reader can copy it without understanding anything. This is R2's "page leaks the
   answer".

Controls for parametric leakage:

- **No-page baseline (R1).** This detects leakage. It does nothing to reduce it.
- **Nonce-word substitution.** Rename every concept consistently with an invented word,
  across the pages and the goals, before the simulator sees them. "Chirp Z-transform"
  becomes "vexil transform" everywhere. Recall from training is then cut wherever it
  depends on the name. It is not cut where it depends on structure: a written-out DFT
  formula still gives itself away. The renaming is deterministic, reversible, and costs
  no extra model calls. Wei et al., "Larger language models do in-context learning
  differently" (2023), used a related move: they swapped natural labels for semantically
  unrelated ones to separate learning from context from prior knowledge.
- **A ladder of reader models.** Part of the competence paradox comes from choosing a
  capable model. A smaller model with less prior knowledge is a closer stand-in for a
  novice. Running the same probes up a ladder of model sizes shows which findings hold
  across competence levels and which come from one model's priors.

Controls for in-context leakage:

- **Transfer questions (R2).** Use questions whose answer text appears on no page.
- **Overlap check.** Measure the textual overlap between a goal's reference answer and
  the page text. This needs no model and flags goals that only test copying.

## Credit per page, not per path

The no-pages versus all-pages difference gives one number for a whole path. **Leave-one-out
ablation** gives credit to each page: drop page i from the visited set, re-ask the goal,
and record the drop in score. Pages that change nothing for any goal can be cut or merged.
Pages whose removal breaks goals they were never declared to serve point to an undeclared
`requires`.

The cost is one call per page, per goal, per trial, which the cache described in
2026-10-10-v1-scope absorbs on re-runs. This is a cheap and coarse form of Shapley-style
attribution; full Shapley values would cost exponentially more calls.

## Testing the simulator by planting defects

Human calibration (R2) is expensive and slow. The simulator's *sensitivity* can be tested
without humans by mutation testing. Take a corpus known to be good, plant known defects,
and check whether the simulator's scores drop and whether it points to the defect:

- delete a definition;
- reorder pages so a concept is used before it is introduced;
- overload a term with a second meaning;
- remove a worked example;
- insert an ungrounded decorative phrase of the "dyadic real and imaginary parts" kind.

Detection rate and localisation accuracy measure what the simulator can see. They do not
show that what it sees matches what humans struggle with, so human calibration stays
necessary. The same planted-defect corpus also serves R2's first experiment, "can the
linter recover planted defects?", for the deterministic layer.

## Prior work on simulated navigation

- **CoLiDeS** (Kitajima, Blackmon & Polson, 2000) models a user choosing a link by its
  semantic similarity to the user's goal.
- **Cognitive Walkthrough for the Web** (Blackmon, Polson, Kitajima & Lewis, 2002) built
  on CoLiDeS. Its automated form used Latent Semantic Analysis spaces built from texts at
  a chosen reading level, which acted as a crude epistemic state. It flagged weak-scent
  links, competing headings, competing links, and words unfamiliar at that reading level.
  These are the navigation diagnostics R1 asks for, produced two decades ago without
  LLMs.
- **Wikispeedia** (West, Pineau & Precup, 2009; West & Leskovec, "Human wayfinding in
  information networks", 2012) is a public dataset of humans navigating Wikipedia
  towards target articles, with completed and abandoned paths. It is the same game as
  this project, played by people. A navigation agent can be checked against human
  behaviour on that graph before its behaviour is trusted on an exegenius corpus.

## Adversarial reading

R1's fifth thread proposed a second agent that tries to produce a plausible but wrong
reading of an entry. The minimal edit that blocks that reading is then a sharper
confusion measure than average comprehension.

R2 bounded this to specific misconception types:

- inferring a stronger claim than the text supports;
- confusing a necessary condition with a sufficient one;
- applying a result outside its stated assumptions;
- reading an overloaded term in the wrong sense;
- skipping a prerequisite while appearing to understand.

R2's loop: generate a misconception, check whether the text permits it, propose a minimal
edit, re-test, then check the edit for regressions in correctness and readability. R2 also
listed what to measure:

- how severe the misreading is;
- whether the text genuinely enables it;
- whether the edit blocks it;
- whether the edit adds new ambiguity or unnecessary length;
- whether the edit preserves the original claim.

R2 cautioned that an implausible misconception invented by an LLM is not evidence of a
real comprehension problem.

R2 ranked adversarial tests second, ahead of ESS simulation in third place, and asked for
a small end-to-end simulation demo early. That demo should be labelled experimental and
must not be able to override deterministic checks.

## Where the simulator runs

A closed book is a property of the API call, not of the prompt. The simulator call has to
be a direct model API request with no tools, no web access, and only the visited pages in
context. A coding-agent subagent with file and web tools cannot be closed-book.

This decides the architecture. The author loop can run inside a coding agent such as
Claude Code, which calls the exegenius CLI. The simulator, however, sits in an exegenius
crate that makes its own API calls, behind R1's provider trait, with every call cached.

## Minimal experimental simulator for v1

The smallest end-to-end simulator that still produces evidence has five parts:

- **ESS fields.** Use known concepts (from the profile) and available evidence (the
  pages on the planned path). Defer the other four fields.
- **Goal answering.** Answer each goal closed-book, citing spans that appear verbatim in
  the visited pages.
- **Span check.** Check by string match that each cited span appears verbatim in a
  visited page. Whether the span supports the answer is a judgement call, either an
  LLM judge against the goal's authored reference answer or the author.
- **Baseline.** Run a no-page baseline for each goal, and mark goals the model answers
  unaided as uninformative.
- **Report.** Report per goal, kept separate from lint output, and never as a
  correctness claim.

## Open questions for steering

1. **Provider.** Should the provider be the Anthropic Messages API with a key from the
   environment, or should a trait with a single implementation come first? Recommendation:
   one implementation behind the trait. A second provider can wait until someone needs it.
2. **Grading.** Options are an authored reference answer per goal (key points), an LLM
   judge, or both. Recommendation: authored key points graded by a cached LLM judge. With
   no authored answer, the grade cannot be audited.
3. **Reader model choice.** Should the default reader be a small model (closer to a
   novice, cheaper) with a larger model as a check, or the reverse?
4. **Ordering after v1.** Adversarial misreading (R2's priority 2) or navigation
   trajectories (R1's game framing)?
5. **Confusion log format.** Should `?` markers go inline in the markdown (an HTML comment
   such as `<!-- ? why m/2^k -->`), or into a separate log keyed by entry and span?

## Current State
- No source file in the repository implements simulation, grading, caching, or a model
  provider — the design appears only in docs/journal/2026-10-10-simulation-validity.md

## Missing
- No model provider trait or Anthropic API client
- No closed-book goal-answering prompt, span check, or no-page baseline
- No LLM call cache keyed by model, prompt template version, and content hash
- No nonce-word substitution, page ablation, or planted-defect corpus
- No confusion-log format or parser

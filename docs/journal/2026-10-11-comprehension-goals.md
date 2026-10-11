# 2026-10-11: Comprehension goals

This is the fourth of the research entries listed in 2026-10-11-explanatory-structure-research. The simulation measures only what its goal questions ask. This entry asks what the goals have to ask for the simulation to see the other entries' effects: symbols, connectives, plans and aha moments. It covers proposals I-88, I-92 and I-94, and the held-out goals carried over from 2026-10-11-checkpoint-held-work. Nothing here is implemented.

## The problem

Goals have two kinds, `recall` and `transfer` (exegenius-core/src/simulate.rs:86-89):
- `recall` asks what something is or states.
- `transfer` asks the reader to apply or explain.
- The goals agent must make at least half of the goals transfer goals (exegenius-core/src/simulate.rs:44-46).

Key points are short checkable statements. A grade lists the numbers of the key points an answer makes, and its wrong claims (exegenius-core/src/grade.rs:35-42).

That is enough to show that a wiki teaches something, as the demo's 73% against a 53% no-wiki baseline shows. It is not enough to show what the wiki teaches, or where it fails:
- **Hidden purpose questions.** A transfer goal on why κ is half the exponent gap mixes the meaning of terms, the justification of a step and the purpose of a module.
- **Undifferentiated misses.** A missed key point does not say which of those failed.

Every goal also feeds repairs. Round 2 therefore partly measures whether the repairs landed (2026-10-11-checkpoint-held-work).

## What the literature gives

### An assessment model with seven dimensions

Mejía-Ramos, Fuller, Weber, Rhoads & Samkoff (2012; manuscript pages) built an assessment model for proof comprehension. They draw on interviews with mathematicians and adapt Yang & Lin (2008).
- **Local understanding** is understanding that "can be discerned either by studying a specific statement in the proof or how that statement relates to a small number of other statements".
- **Holistic understanding** "must be ascertained by inferring the ideas or methods that motivate a major part of the proof, or the proof in its entirety" (p. 6).
- The dimensions are "not ... part of a hierarchy".

| Dimension | Their question templates (abridged) |
|---|---|
| Local: "Meaning of terms and statements" | "State the definition of a given term in the proof"; "Identify examples that illustrate a given term"; "State a given statement in a different but equivalent manner" |
| Local: "Logical status of statements and proof framework" | "Identify the purpose of a sentence within a proof framework" |
| Local: "Justification of claims" | "Make explicit an implicit warrant"; "Identify the specific data supporting a given claim"; "Identify the specific claims that are supported by a given statement" |
| Holistic: "Summarizing via high-level ideas" | "Identify or provide a good summary of the proof"; "Identify a good summary of a key sub-proof" |
| Holistic: "Identifying the modular structure" | partition into modules; "Identify the purpose of a module"; "Identify the logical relation between modules" |
| Holistic: "Transferring the general ideas or methods to another context" | "Transfer the method"; "Identify the method"; "Appreciate the scope of the method" |
| Holistic: "Illustrating with examples" | "Illustrate a sequence of inferences with a specific example"; "Interpret a statement or its proof in terms of a diagram" |

Two footnotes matter here:
- **Footnote 1** (p. 8) names two kinds of comprehension that mathematicians mentioned and the model leaves out: "recognizing where a proof becomes non-routine or difficult, and recognizing why the obvious approach to proving a particular theorem fails". These are the closest kinds to an aha moment in the assessment literature (2026-10-11-aha-moments).
- **Footnote 5** (p. 22) warns against answers produced "by matching symbols rather than using a deep understanding". It proposes asking for justification, or giving a poor choice and asking why it fails.

They also note that some proofs, such as the standard proof that √2 is irrational, cannot be broken into modules. Not every dimension applies to every target.

### Purpose questions

Hamami & Morris (Noûs 2024) diagnose failures of understanding with questions about practical reasoning. Their examples from McKay's proof:
- "Why introduce this particular set S of p-tuples in G?"
- "Why define this specific equivalence relation on S in terms of cyclic permutations?"
- "How did one think of counting the elements of S in these two different ways?"

A reader who has verified the proof but cannot answer such questions "does not fully understand it" (§4.1).

### Examples: an open disagreement

The literature disagrees on where examples belong:
- Hamami & Morris sort Avigad's eleven abilities and put "give natural examples" in general mathematical proficiency, not proof understanding (§7.1).
- Mejía-Ramos et al. make illustrating with examples a dimension of comprehension.
- Weber & Mejía-Ramos's mathematicians all use examples, and six of nine use them to understand (manuscript p. 22).

The disagreement is about what to call understanding, not about whether examples help. An example goal stays in the kind list.

### Levels of a stuck point

Spratte (2024) merges three proof-comprehension models with Kintsch's text base and situation model. The result has five levels: "a logical and a semantical micro level, a local and a global level of coherence, and a level-beyond-the-text" (p. 10).
- **Where global begins.** The line between local and global falls "when more than one argumentative step comes into play" (p. 11). And "identifying which of the (few) chained steps is central is a matter of global coherence".
- **No empirical separation.** "all attempts to empirically separate local and global comprehension known to the author of this paper so far have failed" (p. 7).

So the levels are diagnostic categories, not separate measurable scales.

### Text base and situation model

McNamara, Kintsch, Songer & Kintsch (1996) found that the benefit of a less coherent text for knowledgeable readers appeared only at the level of the situation model. It showed on inference questions, problem-solving and sorting, not on free recall or text-based questions. Recall goals and transfer goals measure different levels, and a change to the wiki can move one without the other.

### Argument roles and how to grade them

**ARC** (Elaraby & Litman 2026) splits each salient argument role into atomic facts. An LLM judge then labels each fact against a summary as supported, missing or contradicted.
- **Omission dominates.** "missing facts are the dominant source of error ... salient information is more often omitted than misrepresented".
- **Role bias.** Coverage favours conclusions over issues and reasons. In the scientific domain, background claims are covered least well.

**Altemeyer et al. (2025)** compared ways of scoring coverage against human judges.
- An LLM asked to count which reference key points a summary covers correlates with them up to 0.767 across topics and 0.803 within.
- Embedding-similarity scores do badly.
- This supports exegenius's grading by key-point counting. Their domain is debate arguments, not explanations.

## Design

### Goal kinds (I-88)

`GoalKind` gains these kinds, each with a question template in the goals instructions:

| Kind | From | Example on the demo |
|---|---|---|
| `meaning` | M-R local 1 | What does r count, and why is it a power of two? |
| `status` | M-R local 2 | What role does the no-wraparound condition 2q − 2 < S play in the reduction? |
| `justification` | M-R local 3 | Why can the butterfly layer keep its intermediate values exact? |
| `summary` | M-R holistic 4 | Summarise how the multiplication is reduced to transforms. |
| `structure` | M-R holistic 5 | What does the parameter-choice step contribute to the main bound? |
| `transfer` | M-R holistic 6 | How would the cost change if the last axis length were not a power of two? |
| `example` | M-R holistic 7 | Work the synthetic transform for r = 4 on a given coefficient list. |
| `motivation` | Hamami & Morris | Why are the values Gaussian dyadic rather than floating point? |
| `non-routine` | M-R footnote 1 | Where does the argument stop being a standard FFT multiplication? |
| `obvious-fails` | M-R footnote 1 | Why does a standard floating-point FFT not give the bound on a Turing machine? |

The example questions are illustrations, written for this entry from the demo pages. They are not checked against the source.

`recall` and `transfer` map onto these: `recall` covers `meaning` and `status`. `transfer` keeps its name. The goals instructions ask for at least one holistic goal and at least one `motivation` goal, so that every run tests the rationality part. With 3 to 6 goals, not every kind appears in every run, so the kinds work as a checklist, not a quota.

Footnote 5's warning extends the existing verbatim check:
- **Today.** `apply_goals` rejects a transfer key point of 20 or more characters that appears word for word on a page (exegenius-core/src/simulate.rs:713-722).
- **Proposed.** The same rule applies to every holistic kind and to `motivation`.

### Held-out goals

Each goal gains `held_out: bool`.
- **Repairs skip held-out goals.** `grade::repair` turns missed key points into rewrite notes (exegenius-core/src/grade.rs:159), and it would not do so for held-out goals.
- **The report shows two figures.** It reports coverage for held-out goals separately, and only that figure counts as evidence that the wiki teaches.
- **A separate writer.** The held-out goals come from a second goals task that does not see the repair goals, so their wording cannot follow the repairs.

### Role-tagged key points (I-92)

Each key point gains a role:
- `claim`: what is asserted;
- `reason`: why it holds;
- `conclusion`: what follows;
- `background`: what it rests on.

The grade records each key point as `covered`, `missing` or `contradicted`. That replaces the free-text `wrong` list with ARC's three-way label, and keeps the list for wrong claims that answer no key point. The report shows coverage per role. If readers get conclusions and miss reasons, the wiki states results without their justification, which is ARC's role bias.

### Stuck-point levels (I-94)

Each stuck point gains a level from Spratte's model, in the reader's words:

| Level | Reader instruction |
|---|---|
| `symbol` | a symbol or term I could not decode |
| `statement` | I could not tell what a statement's status was (definition, assumption, claim) |
| `step` | I could not follow one inference |
| `chain` | I followed the steps but not why they lead where they do |
| `beyond` | I could not connect it to anything I already know |

The levels route repairs:
- **`symbol`** points to bindings (2026-10-11-symbol-anchoring).
- **`step`** points to a bridging sentence on the page.
- **`chain`** points to the plan and the page's breadcrumb (2026-10-11-proof-plans). That is the level where the "years happen in days" sentences should show.

The report aggregates stuck points per page and per level.

## How the simulation tests it

1. **Per-kind coverage against the baseline.**
   - The no-wiki baseline runs on every goal already.
   - Per kind, the gap between wiki and baseline shows what the wiki adds.
   - The prediction from Aher et al.'s hyper-accuracy finding: `meaning` and `status` goals on standard material sit near the baseline, and holistic and `motivation` goals show the gap.
2. **Kinds as instruments for the other entries.**
   - `meaning` measures 2026-10-11-symbol-anchoring.
   - `structure` measures 2026-10-11-signalled-edges.
   - `summary` and `motivation` measure 2026-10-11-proof-plans.
   - `non-routine` and `obvious-fails` measure 2026-10-11-aha-moments.
3. **Held-out coverage across rounds.** If held-out coverage rises from round 1 to round 2, the repairs taught something beyond the goals they quoted. If only repair-goal coverage rises, the repairs taught to the test.

## Risks

- **Cost.** More kinds tempt more goals. Each goal costs one read and one baseline per reader profile and start, plus a grade. The goal count stays at 3 to 6, and the kinds spread across rounds.
- **Goal quality.** An agent asked for a `motivation` goal without a plan tree may write a question whose answer is not in the source. Once a plan exists, `motivation` goals can come from its nodes, and `explicit` motivations have quotes.
- **Kinds are not scales.** Spratte's caution applies: per-kind coverage is a profile of the wiki, not a measurement of separate abilities.

## Open questions for steering

1. **The kind list.** All ten kinds, or a subset first (`meaning`, `justification`, `structure`, `motivation`)?
2. **The held-out split.** With 3 to 6 goals, is one held-out goal per round enough? Or should held-out goals be added on top of the repair goals?
3. **Who sets the level.** Should readers choose the stuck-point level, or the grader infer it from the note?
4. **Role tags.** Should role tags on key points be required, or only on `justification` and `motivation` goals, where reasons are the point?

## Current State
- `GoalKind` has two variants, `Recall` and `Transfer` (exegenius-core/src/simulate.rs:86-89)
- The goals instructions ask for 3 to 6 goals, at least half of them transfer goals, each with 2 to 5 key points and the pages needed (exegenius-core/src/simulate.rs:36-52)
- `apply_goals` rejects a transfer key point of 20 or more characters that appears word for word on a page (exegenius-core/src/simulate.rs:713-722)
- A grade lists the numbers of the covered key points and free-text wrong claims (exegenius-core/src/grade.rs:35-42)
- `grade::repair` turns stuck points, give-ups and key points no reader made into rewrite notes, for every goal (exegenius-core/src/grade.rs:159)
- A stuck point records a page handle, a quote and a free-text note, with no level (exegenius-core/src/simulate.rs:126-133)

## Missing
- No goal kinds beyond recall and transfer
- No held-out goals
- No role tags on key points, and no covered/missing/contradicted label per key point
- No level on stuck points
- No per-kind or per-role coverage in the report

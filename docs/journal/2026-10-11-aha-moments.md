# 2026-10-11: Aha moments

This is the fifth of the research entries listed in 2026-10-11-explanatory-structure-research. It develops the 🏹 idea: identify "aha moments", phrases that tie up some extended chain of reasoning. It covers proposals I-87, I-89 and I-95, and puts the leave-one-out ablation (I-51) to a sharper use. Nothing here is implemented.

## The problem

**The demo's clearest aha moment** came at the end of the author's reading. `gaussian-dyadic` says that a Gaussian dyadic value "has a finite binary expansion, so it can be stored and combined on a tape without rounding". It says this property "is why the simultaneous butterfly layer can keep intermediate values exact".

That sentence ties the representation of the numbers to the tape model, where the reading started. A word the author had dismissed as a mannerism turned out to carry the argument.

**The wiki did not mark it.** The sentence sits on a page in the middle of the reading order. Nothing elsewhere points to it as the place where the chain closes. The front page and part intros did not exist in v0.1, and today they summarise parts without naming such sentences.

**The opposite failure** is the author's "years happen in days": a sentence that closes a long chain so tersely that the reader cannot follow it. It is the same kind of sentence, with the steps missing.

Two questions follow:
1. How can such sentences be found?
2. What should the wiki do with them?

## What "aha moment" can mean here

The ChatGPT session distinguished three kinds of gain:
- **information gain**: the reader learns a new fact;
- **relational gain**: the reader sees a connection between things already known;
- **explanatory gain**: the reader sees why a result or mechanism works.

The 🏹 idea is about the second and third.

The session also warned that a model reporting understanding is not evidence of understanding. Aher, Arriaga & Kalai (2023) give a reason to take that seriously: "larger and more aligned LMs simulate human subjects that give unhumanly accurate answers". A simulated reader's "aha" is a sentence it chose to write. Any detection has to rest on what readers do, not on what they say.

## What the literature gives

### An aha moment as a reconnection to the goal

Hamami & Morris (Noûs 2024) supply a definition that fits.
- **Understanding is tracing back.** It means being able to trace each step back "to the main objective through an uninterrupted chain of rational practical decisions" (§5.1).
- **A broken chain** leaves a step looking "mysterious, arbitrary, or 'engendered by caprice'" (p. 26).
- **A reconnection** is the point where a step's link to the main objective becomes visible.

In McKay's proof the reader meets that point late: "It is only when one reaches the equation r + sp = n^(p−1) towards the end of the proof that one can reconnect with the main objective" (p. 20). The dyadic sentence plays the same part in the demo wiki.

They also make key ideas concrete: describing a proof idea is "simply describing a subplan or a part thereof in the overall plan" (§5.2). On this account an aha moment is where a subplan's place in the plan becomes visible to the reader.

### What counts as explanatory depends on what is salient

Lange (2024) argues that a proof explains only relative to a feature of the result that the reader finds striking:
- **Salience.** "The distinction between proofs that explain and proofs that merely prove exists only when some feature or other of the result being proved is salient" (p. 4).
- **The explanatory proof** "exploits a certain kind of feature in the set up: the same kind of feature that is striking in the result".
- **The deus ex machina.** A proof whose key step comes from nowhere is the opposite. He gives Desargues' "three fortuitous cancellations" and a brute-force proof in which "the result's symmetry appeared magically from out of the welter of algebra" (pp. 3, 6).
- **Explanations cascade.** One explanation can expose a new salient feature: "Gardner's proof does not answer this why question. It provokes the question" (§8). Each answer raises the next question, which matches the author's chain of posts.

Two consequences follow:
- **No single aha per page.** Which sentence closes a chain depends on the question the reader brings.
- **Name the salient feature.** A result page can say which feature of its result is striking. That fixes the question the page's explanation has to answer.

### Two kinds of understanding the assessment model left out

Mejía-Ramos et al. (2012, footnote 1) report two kinds of comprehension that mathematicians mentioned but the model leaves out:
- "recognizing where a proof becomes non-routine or difficult";
- "recognizing why the obvious approach to proving a particular theorem fails".

The first marks where the argument departs from what a reader would have done anyway. The second marks the constraint that forces the departure. In the demo, a reader might expect the transforms to run in floating point, with rounding. The wiki's answer is that Gaussian dyadic values have finite binary expansions and need no rounding on a tape. Whether the source presents this as the obvious approach failing has not been checked against the source.

### Readers do not agree on key ideas

- **Yan (2019; abstract only).** Students "varied in their understanding of what 'key idea' might mean". Instructors "need to actively intervene to draw students' attention to features of proofs that are candidates for key ideas".
- **Spratte (2024)** places the same judgement at the level of global coherence: "identifying which of the (few) chained steps is central is a matter of global coherence" (p. 11).

So key ideas have to be marked; readers will not reliably find them alone. And disagreement between readers is information.

### Explanatory sentences are the least signalled

- **eRST** (Zeldes et al. 2025). EXPLANATION relations are signalled in 41.94% of cases, the lowest share of any relation class. A sentence that closes a chain often carries no "so" or "because" to mark it.
- **Liu et al. (2025)** pair each explanatory sentence with the target claim it explains, as the answer to an implicit question about that claim. A target and its explanation are a candidate aha pair.

### Self-explanation

Chi et al. (1994) found that students prompted to explain each line to themselves gained more. "High explainers all achieved the correct mental model of the circulatory system, whereas many of the unprompted students as well as the low explainers did not".

The simulated readers are not learners, so the finding does not transfer directly. But it supports asking readers to explain a page's key idea (I-95) rather than to rate it.

### The reconnection is not always the key step

A review of the hub entry (ChatGPT, 2026-10-11) drew a distinction this entry needs. Three sentences can play different parts in one chain:
- **the key step**: the sentence that does the decisive work, such as the claim that Gaussian dyadic values need no rounding;
- **the payoff**: the sentence that says why the chain matters to the goal, such as the tape model needing exact intermediate values;
- **the reconnection**: the sentence that makes an earlier idea intelligible, by showing how it bears on the goal.

They may coincide, as in the demo's dyadic sentence, but need not. A detector that finds the most important claim finds key steps, not reconnections. The four methods below differ on this:
- the structural method finds sentences that link distant plan nodes, which suits reconnections;
- the salience method finds the explanation of a striking feature, which suits key steps;
- attribution and agreement find whatever readers use or name.

So candidates carry the method that found them, and the report keeps the three apart rather than merging them into one list.

## Four ways to find candidates

### 1. Structural: sentences that reach far up the plan

Given the plan tree (2026-10-11-proof-plans), a sentence is a reconnection candidate when it links pages, or names concepts, whose plan nodes have a high lowest common ancestor. Such a sentence joins two distant parts of the argument.

The dyadic sentence qualifies:
- it names the tape, which belongs to the cost model at the root;
- it names the butterfly layer, deep in the transform subtree.

A cruder version works without a plan. Use the outline's parts: flag sentences that link a page in a different, non-adjacent part. That runs on the current data.

This method finds sentences that could close chains. It does not show that they do.

### 2. Salience: the feature the explanation must reach (I-87)

Each result page records the salient feature of its result: a symmetry, a uniformity across cases, a surprising value or bound. The goals task then writes a `justification` or `motivation` goal asking why that feature holds. The key point must trace it to a matching feature of the setup (2026-10-11-comprehension-goals).

The sentence readers cite for that key point is the page's explanation in Lange's sense. If no reader can make the key point, the page states the result without explaining it.

Where to record the feature is open: the plan node, the write answer, or the goals task. An open question below.

### 3. Behavioural: the sentences that carry key points (I-89)

The grader names, for each covered key point, which of the reader's citations supports it. Readers already cite quotes from pages they opened (exegenius-core/src/simulate.rs:117-122), so attribution needs no new reader behaviour.

Across runs, readers, start points and goals, the report counts how often each quoted sentence carries a key point. The sentences with the highest counts are the pivotal sentences of the wiki as readers actually used it.

Citing a sentence is not the same as learning from it. Readers cite what is easy to quote. So attributed sentences are candidates. Confirmation comes from ablation.

### 4. Agreement: what readers say the key idea is (I-95)

After answering, each reader names the key idea of the pages it relied on most, in a sentence, with a quote. The grader clusters the answers per page.
- **Agreement**, with the quote on the same sentence, marks a key idea readers found.
- **Disagreement** on a page that goals depend on means the page has no clear key idea. It becomes a rewrite note asking the writer to say what the page's key idea is.

## Confirming candidates: ablation

The leave-one-out page ablation (I-51) hides a page from the reader's lens and reruns a goal. Credit is the drop in coverage.

The same machinery can hide a single sentence instead: the lens serves the page with the candidate sentence removed. If coverage of the key points attributed to that sentence drops, the sentence carries them. If coverage holds, readers found the content elsewhere, and the sentence was merely convenient to quote.

**Three outcomes, not one.** The same review distinguished what a deletion can do:
1. **Deletion hurts.** The reader covers fewer key points.
2. **Deletion breaks a connection.** The reader still states the individual facts but can no longer say how they fit together.
3. **Deletion removes a payoff.** The reader follows the chain but no longer says why it matters to the goal.

The second outcome is the closest test of the 🏹 idea: it shows a sentence doing relational work rather than carrying a fact. Telling the outcomes apart needs key points tagged by role, which 2026-10-11-comprehension-goals proposes (I-92): `fact` key points for the first outcome, `relation` key points for the second, `goal` key points for the third.

**A matched control.** Removing a sentence can leave the text ungrammatical, or remove a fact that any answer trivially needs. So each candidate is compared with a control sentence from the same page, of similar length and carrying a fact, ablated in the same way. A candidate counts as pivotal only if its deletion costs more relation or goal key points than the control's.

**Cost.** One reader run per candidate, per goal, per trial. Only the top candidates from methods 1–4 are ablated, a handful per wiki.

**Leakage.** Ablation runs need goals that did not feed the repairs. Otherwise a repaired page has the key point pasted in, and removing the paste trivially lowers coverage. The held-out goals of 2026-10-11-comprehension-goals are the ones to use.

## What the wiki does with confirmed aha moments

The author asked that awareness of "the centrepieces of multi-part chains" should "if not alter the articles themselves ... alter the summarisation articles". Confirmed pivotal sentences can be used in three places, none of which lengthens the pages:
- **A "key steps" view.** A generated page lists the confirmed sentences in plan order, quoted verbatim, each linked to its page and anchored at the sentence. It contains no new prose, so nothing is invented and the pages do not coalesce. This is the significance view of 2026-10-11-fragmentation-and-collapse.
- **Signposting at the sentence.** The confirmed sentence gets a light marker on its page, such as a margin note or emphasis. Schneider et al.'s meta-analysis supports signalling on average. The secondary summary of it reports no effect in mathematics, so this is a hypothesis to test with readers, not a default.
- **Repair of compressed pivots.** A candidate sentence that readers cite and still get stuck on is a "years happen in days" sentence. It needs stuck points of level `chain` or `step` quoting it (I-94). Its rewrite note asks the writer to unpack the steps the sentence compresses. That is a targeted repair, not a general instruction to explain more.

## Validity

- **Self-report is not evidence.** Detection uses attribution, ablation and agreement, never a reader's statement that something clicked.
- **Parametric knowledge.** A model that already knows why FFT multiplication needs exact arithmetic will make the key point without the sentence. Two existing controls bound this:
  - the no-wiki baseline marks goals the model answers unaided;
  - the nonce lens hides concept names.
- **One model.** Pivotal sentences found with one reader model may reflect that model's prior knowledge. The model ladder (I-50) tests whether they hold across model sizes. This is Argyle et al.'s question of algorithmic fidelity asked of reading.
- **Human readers.** None of this shows that a person has an aha moment at the sentence. That needs people (I-60). The author's own reading is one data point, and it agrees on the dyadic sentence.

## Open questions for steering

1. **Where the salient feature is recorded.** On the plan node, in the write answer for result pages, or by the goals agent?
2. **Ablation granularity.** Sentence ablation needs the lens to remove sentences. Is sentence granularity right, or paragraph?
3. **The key-steps view.** Is it public on the site, or kept in the report for the author?
4. **Sentence markers.** Visible markers on pivotal sentences, given the absent signalling effect reported for mathematics?

## Current State
- Simulated readers support each step of an answer with citations, each a page handle and a quote copied from an opened page (exegenius-core/src/simulate.rs:117-122)
- A grade lists covered key point numbers without naming the citation that supports each key point (exegenius-core/src/grade.rs:35-42)
- The report lists each run's path, searches, answer and stuck points, and the pages with the most stuck points (exegenius-core/src/report.rs)
- The simulation lens serves whole pages by opaque handle, with no way to hide a page or a sentence (exegenius-core/src/simulate.rs)
- The outline assigns each page to one part, and part order is available at build time (exegenius-core/src/outline.rs, exegenius-core/src/build.rs)

## Missing
- No salient-feature field on result pages (I-87)
- No attribution of covered key points to reader citations, and no pivotal-sentence counts (I-89)
- No key-idea question for readers and no agreement measure (I-95)
- No page or sentence ablation (I-51)
- No structural detection of sentences that link distant parts
- No key-steps view

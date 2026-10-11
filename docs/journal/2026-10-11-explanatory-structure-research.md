# 2026-10-11: Research: proof plans, salience and anchored symbols

This entry follows 2026-10-11-checkpoint-held-work. It records four things:

1. The author's reading of the deployed demo wiki.
2. Two ideas written down after that reading.
3. A literature search the author ran in a separate ChatGPT session.
4. The proposals that follow from 2 and 3.

For this entry every reference from that session was checked against the paper itself. Section 7 lists what the chat got wrong. Quotes carry the page numbers of the version that was read. That version is named where it is a preprint or author manuscript, because its pages differ from the journal's. Where only an abstract was available, that is said. Nothing in sections 5 and 6 is implemented.

## 1. Reading the demo wiki

The deployed demo (https://openai-nlogn-wiki.vercel.app, from lmmx/openai-nlogn-wiki) was built with v0.1. That was before parts, the glossary and the notation page existed (2026-10-10-wiki-layout, 2026-10-10-glossary-notation-autolink). The author read it and posted four excerpts, in this order.

1. **Entry point: `turing-machine-model`.**
   - Excerpts: "tracks, which are fixed rows of extra data stored on each tape alongside the main data", and the paper's main result "is stated as a bound on this quantity, and the algorithm is intended to establish a bit-complexity bound".
   - The author reached this page by clicking back along a linear chain, then chose it as the place to start pulling the thread.
   - The choice came from the author's own background: automata, algebraic tape representations, and butterfly-level SIMD operations. Nothing on the wiki marked the page as a starting point.
2. **A chain of four pages.**

   ```
   Exact integer multiplication
   ↬via Radix-digit convolution
   ↬reduced to Cyclic convolution
   ↬by Discrete Fourier transform
   ```

   - The wiki holds each hop as an untyped prerequisite link.
   - The connectives (via, reduced to, by) say what each hop does. The author supplied them afterwards, asking ChatGPT which prepositions fit.
3. **The mechanism.**
   - From `discrete-fourier-transform`: "The cyclic convolution of two sequences ... becomes a pointwise product after the DFT".
   - From the same page: the DFTs "are replaced by transforms whose axis lengths are powers of two", and multiplying by such a root "is a shift with sign changes, which is cheap on a tape".
   - From `synthetic-transform`: "the transform is computed in simultaneous layers on a tape". The author's screenshots were of the butterfly material.
4. **A correction: "dyadic" was not a mannerism.**
   - The author had read an earlier, weaker exposition and taken "dyadic" there as a stylistic flourish.
   - `gaussian-dyadic` showed it is a precondition. A Gaussian dyadic value "has a finite binary expansion, so it can be stored and combined on a tape without rounding", and so "the simultaneous butterfly layer can keep intermediate values exact".
   - The author could settle the question because the page showed what is known for sure, backed by the source.

The four excerpts trace one chain whose end reconnects to its start:
- Excerpt 1 fixes what counts as cost: steps on tapes, and bits.
- Excerpts 2 and 3 reduce multiplication to transforms whose twiddle factors are shifts that are cheap on a tape.
- Excerpt 4 explains why the numbers are dyadic: so the butterflies stay exact on a tape.

"Dyadic" is a term that looks incidental and carries weight only once the chain closes. The wiki held every link in the chain, but no page stated the chain. The author assembled it with the back button.

The author reported two failings from the same reading:

- **Compression.** Some sentences, in the author's words, have "years happen in days": several inferential steps are packed into one grammatical sentence.
- **Unanchored symbols.**
  - The author's report: "I was reading a y^r and I think r was the number of digits in the number, but it wasnt said explicitly."
  - On `synthetic-transform` the roots of unity are "powers of \(y\) in the polynomial ring modulo \(y^r + 1\)". The page never says what r is. Its only other mention is the example "Take \(r = 4\) and a coefficient list \((a_0, a_1, a_2, a_3)\)".
  - The prerequisite page `polynomial-ring-mod-yr-plus-1` says "Fix a power of two \(r\)" and calls a record "a list of \(r\) complex coefficients". `parameter-choice`, two clicks from `synthetic-transform`, gives \(r = 2^{\lceil (\log_2 T)/d \rceil}\).
  - On this wiki r is the number of complex coefficients per record, which is also the last transform axis length. The number of digits is q.
  - So the guess was wrong, at exactly the place the report flagged. The report's point that guessing is a failure even when it happens to be right is borne out by the guess not being right.

Checking the other pages the author posted from turned up three more kinds of failure. Quotes are the LaTeX source as served:

- **A symbol unbound on the page, though defined one or two clicks away.**
  - `signed-packing` gives a time of "\(O(rp \log(rp))\)" without defining r or p.
  - `gaussian-dyadic` uses "the butterfly map \(H_0\)", which computes "\((u+v)/2\) and \((u-v)/2\)", without defining H₀, u or v.
- **One letter, several meanings.**
  - q is the radix on `turing-machine-model` ("one specified radix-\(q\) digit"). On `cyclic-convolution` it is "the number of digits of each input".
  - y is the second input integer on `radix-digit-convolution`, the polynomial indeterminate on `synthetic-transform`, and an imaginary part on `gaussian-dyadic`.
  - w, z and b each carry two or three meanings across pages.
- **A reference to a definition that does not exist.** `multiplication-algorithm` says the parameter choice fixes S, but `parameter-choice` defines T.

## 2. The two ideas

Written after the reading:

> 🏹 Identify 'aha moments', phrases that tie up some extended chain of reasoning
>
> 🚅 Tracing the significance of symbolic terms (which often appear as incidental Chekhov's
> guns), pre-emptively establish narrative as 'actors' or frames of reference

The author's elaboration, in the same session, set up a tension:

- **Collapse.** Making every relationship explicit would make the pages coalesce, which would be bad.
- **Visible centrepieces.** Some passages are "clearly the centrepieces of multi-part chains". Awareness of where they are could change the pages, or at least the summary pages.
- **Symbols.** Defining every symbol every time makes the prose worse. Defining too few leaves the reader "unmoored".

## 3. How the search went

The ChatGPT session ran in this order. The author pasted it in reverse.

1. **The journal's citations.** ChatGPT first could not fetch `docs/journal/`. It then read the origin, simulation-validity, rules-and-grounding and ideas-bank entries. Its assessment:
   - The citations attach to specific design choices.
   - The bibliography is not traceable: author-year mentions only.
   - The revised v1 design has moved past some of the papers that motivated it.
   - A verbatim quote check establishes that a quote exists, not that it supports a claim.
2. **"More relevant works".** Five areas: knowledge modelling, reading psychology, information foraging, LLM-simulated learners, and explanation design.
3. **The two ideas.** ChatGPT placed them in those areas and proposed an "explanatory arc": setup, dependencies, payoff, retrospective significance, and the reader support each needs.
4. **The reader report.** ChatGPT answered with discourse coherence, bridging inferences, ScholarPhi and symbol grounding. The author judged this strand off target ("these dont seem quite so close to the original idea").
5. **Proof comprehension.** Yan; Hamami & Morris; Weber & Mejía-Ramos. eRST also came up; the author had read it recently.
6. **Recent applied work.** Lange; MiDAS-PRo; ARC; eRST.
7. **A synthesis.**
   - Five directions: explanation coverage, salience conditioned on the question, the gap as a first-class object, aha moments as change points, and multiple views.
   - An "explanatory skeleton" with six fields: target, setup, transition, payoff, evidence, gap status.
   - Three further papers: Liu et al., Spratte, Altemeyer et al.

After checking, the two strands divide the work between them:
- The strand the author recognised gives the theory of the aha moment (section 4.1–4.3). Hamami & Morris quote Poincaré and Bourbaki on understanding a proof versus verifying it.
- The strand the author judged off target holds the most directly usable empirical data on symbols (section 4.7).

## 4. What the papers say

### 4.1 Understanding a proof is reconstructing its plan

Two papers by Hamami & Morris give the account.

**The account (Noûs 2024).** Page numbers are from the authors' preprint.

- **Plans.** "understanding a proof requires the agent to rationally reconstruct the plan of the proof" (p. 4). A plan is "an ordered tree such that":
  1. "each node is a proving intention";
  2. "the root is the proving intention corresponding to the theorem at hand";
  3. "each set of ordered children of a given parent node is a subplan that has been obtained from the parent node through an instance of practical reasoning" (p. 8).
- **Two kinds of proving intention** (p. 6), each with its own arrow. These are the arrows the author remembered:
  - **To show.** P₁,…,Pₙ ⇒ C is "the intention to show C from [P₁,…,Pₙ]". It "cannot be fulfilled directly", so it has to be turned into more specific intentions.
  - **To infer.** P₁,…,Pₙ ↪ C, the hooked arrow, is "the intention to infer C from [P₁,…,Pₙ]". It "can be fulfilled directly by actually carrying out the considered inference".
  - **Practical reasoning.** A vertical ↓ joins a parent to its subplan, and stands for one instance of practical reasoning.
- **Two components of understanding** (§3, p. 9):
  - the tracking part: reconstruct the tree;
  - the rationality part: reproduce every instance of practical reasoning that built the plan.
- **Verifying versus understanding** (p. 16).
  - Verifying "evaluates one inference after the other in a linear way". There is "no need to track the intentions and the plan construction of the author".
  - The two run in different orders: "the last inference to be made in the proof activity already figures in the first construction step of the plan".
- **Poincaré** (Science and Method, quoted from Maitland's 1914 translation, p. 118):

  > Almost all are more exacting; they want to know not only whether all the syllogisms of a
  > demonstration are correct, but why they are linked together in one order rather than in
  > another. As long as they appear to them engendered by caprice, and not by an intelligence
  > constantly conscious of the end to be attained, they do not think they have understood.

  **Bourbaki** ("The architecture of mathematics", American Mathematical Monthly 57(4), 1950, p. 223):

  > [E]very mathematician knows that a proof has not really been 'understood' if one has done
  > nothing more than verifying step by step the correctness of the deductions of which it is
  > composed, and has not tried to gain a clear insight into the ideas which have led to the
  > construction of this particular chain of deductions in preference to every other one.

- **Their reading of Poincaré.** An agent who does not understand "cannot trace back these deductive steps to the main objective through an uninterrupted chain of rational practical decisions" (§5.1, p. 25). A step with no such chain "will appear as mysterious, arbitrary, or 'engendered by caprice'" (p. 26).
- **Their reading of Bourbaki.** Describing a proof idea is "simply describing a subplan or a part thereof in the overall plan" (§5.2, p. 26). Footnote 22 adds that Pólya (1949) calls an unmotivated step a "deus ex machina" and argues it needs "motivations".
- **A case study: McKay's proof of Cauchy's theorem** (§4.1, pp. 19–21).
  - The steps reconnect to the goal only late: "It is only when one reaches the equation r + sp = n^(p−1) towards the end of the proof that one can reconnect with the main objective."
  - Their diagnostic questions include "Why introduce this particular set S of p-tuples in G?" and "How did one think of counting the elements of S in these two different ways?". A reader who has verified the proof but cannot answer them does not fully understand it.
  - The prescribed fix is to give the reader "relevant information enabling her to reproduce the above instance of practical reasoning".
  - McKay introduces his r late, as the number of solutions to x^p = 1. Their reconstructed plan carries the binding on the node: "p|r   where r is the number of solutions to x^p = 1" (p. 20).
- **Localised, top-down understanding.** Partial understanding "can be precisely localized" (p. 12). The account describes understanding as "a top-down process" (p. 30).

**The formal apparatus (Review of Symbolic Logic 2021).** It builds on Bratman's planning theory.

- **Methods.** A mathematical method is "a rule that transforms a proving intention into one or several other proving intentions arranged in a subplan" (§5.1, preprint p. 10).
- **Text cues.** Show-intentions are signalled by proving verbs such as "we show that" (p. 8). But "in most cases, the reader is expected to figure out by herself the proving intentions of the author(s)" (footnote 9).
- **Implicit hypotheses.** Footnote 13: hypotheses "are only rarely written explicitly ... [they] must usually be inferred from the current context of discourse". Footnote 14 counts the relevant definitions among the hypotheses.
- **A deus ex machina.** In Pólya's proof of Carleman's inequality, "one might really wonder in this case where this ci sequence comes from" (§5.2, p. 15).
- **Modules.** "any subtree of P can be conceived as a module which can be described by stating the proving intention of type 'to show' at its root" (§7).

### 4.2 A proof explains relative to a salient feature

Lange (2024) asks what makes a proof explanatory. Page numbers are from the typeset article.

- **Correct but not explanatory.** A brute-force proof of a coin-toss result "sheds no light on why the result is what it is ... the fortuitous cancellations at the end come out of nowhere" (p. 2). In it "the result's symmetry appeared magically from out of the welter of algebra" (p. 3).
- **Salience.**
  - "What it means to ask why the result holds is to ask for a proof that exploits a certain kind of feature in the set up: the same kind of feature that is striking in the result. The distinction between proofs that explain and proofs that merely prove exists only when some feature or other of the result being proved is salient" (p. 4).
  - "Symmetries are not somehow intrinsically explanatory" (p. 4).
  - Without a salient feature, a why-question "would have been nothing more than a request for a proof" (p. 3).
- **Unity.** A proof that treats every case the same way explains a property common to every case. "Explanatoriness comes in degrees" (p. 6).
- **Deus ex machina.** Desargues' own proof "works by way of three fortuitous cancellations and so is typically characterized as 'ingenious' ... in a pejorative sense—as a deus ex machina proof" (p. 6).
- **New entities.** Explanations "motivate the introduction into the classroom of new mathematical entities on the grounds that those entities help to explain results exclusively concerning familiar mathematical entities" (§7, p. 11). An example is complex numbers explaining the radius of convergence of 1/(1+x²).
- **Context.** "Salience varies with context" (§8, p. 12). One explanation can expose a new salient feature: "Gardner's proof does not answer this why question. It provokes the question."
- **Two senses of "explanation".** Explaining why is distinguished from "'explanation' in the sense of conveying some information" (p. 2, footnote).

### 4.3 What readers of proofs do, and how comprehension has been assessed

**Interviews with mathematicians** (Weber & Mejía-Ramos 2011: nine research mathematicians; page numbers from the authors' manuscript).
- "all nine participants indicated that understanding did not solely consist of knowing how each step followed logically from previous steps" (p. 16).
- The authors describe reading as zooming in on problematic parts and conjecture zooming out to the high-level structure. Comprehension may consist of "coordinating inspections of the logical details of a proof while zooming out to see these logical details in the context of the methods they are being used to support" (pp. 24–25).
- All nine use examples (p. 19).

**An assessment model** (Mejía-Ramos, Fuller, Weber, Rhoads & Samkoff 2012; page numbers from the authors' manuscript) has seven dimensions (§5.1):
- local:
  1. "Meaning of terms and statements"
  2. "Logical status of statements and proof framework"
  3. "Justification of claims"
- holistic:
  4. "Summarizing via high-level ideas"
  5. "Identifying the modular structure"
  6. "Transferring the general ideas or methods to another context"
  7. "Illustrating with examples"

Its question templates include:
- "State the definition of a given term in the proof";
- "Identify the purpose of a sentence within a proof framework";
- "Make explicit an implicit warrant";
- "Identify the purpose of a module".

**What the model left out.** Footnote 1 (p. 8) names two kinds of comprehension that mathematicians mentioned but the authors left out of the model: "recognizing where a proof becomes non-routine or difficult, and recognizing why the obvious approach to proving a particular theorem fails". Footnote 5 warns against answers produced "by matching symbols rather than using a deep understanding".

**Key ideas** (Yan 2019; abstract only). Students "varied in their understanding of what 'key idea' might mean". Instructors "need to actively intervene to draw students' attention to features of proofs that are candidates for key ideas".

**A unified model** (Spratte 2024). Spratte merges three proof-comprehension models with Kintsch's text base and situation model into five levels: "a logical and a semantical micro level, a local and a global level of coherence, and a level-beyond-the-text" (p. 10).
- **Symbols.** "Decoding symbols and single words to grasp their semantic content is a prerequisite to constructing coherence within small text segments" (p. 4).
- **The central step.** "identifying which of the (few) chained steps is central is a matter of global coherence" (p. 11).
- **No empirical support.** "all attempts to empirically separate local and global comprehension known to the author of this paper so far have failed" (p. 7). The paper presents no data for its model (p. 13).

### 4.4 Discourse relations carry signals (eRST)

eRST (Zeldes et al. 2025) extends Rhetorical Structure Theory (RST). An analysis has three parts:
1. a primary RST tree;
2. secondary edges, which may be cyclic and non-projective;
3. signals: tokens that anchor a relation (§3).

**Signals.**
- A secondary edge "may only be added if it is supported by a sufficient signal" (§3.2).
- Signals are discourse markers plus seven other types: graphical, lexical, morphological, numerical, reference, semantic, syntactic.

**The relation inventory.** The GUM corpus uses 32 labels, among them EXPLANATION-{EVIDENCE, JUSTIFY, MOTIVATION}, CONTEXT-BACKGROUND, MODE-{MANNER, MEANS}, CAUSAL-{CAUSE, RESULT} and RESTATEMENT.

Two numbers from the corpus bear on the reading report (Table 7, Table 10):
- **Explanation relations are the least often signalled.** EXPLANATION relations are signalled in 41.94% of cases, the lowest share of any class. MODE relations are signalled in 78.71%, and their most frequent discourse marker is "by".
- **A single tree loses causal and justifying links.** Secondary edges are 3.37% of relations. But the primary tree alone misses 14.1% of CAUSAL-RESULT and 11.7% of EXPLANATION-JUSTIFY relations.

The author's connectives map onto signals:
- "by" is the most frequent marker of MODE;
- "via" and "reduced to" are lexical signals of the alternative-expression kind (PDTB's AltLex).

The GUM and GENTLE corpora include mathematical proofs among their genres.

### 4.5 A plan before the prose

**Questions as a plan** (Liu, Yu, Demberg & Lapata 2025).
- They plan lay summaries as an ordered list of unanswered questions, each a latent Question Under Discussion (QUD) that an explanatory sentence answers.
- The questions come from pairs of discourse units in RST parses of reference summaries, under four relations: Background, Elaboration, Explanation, Comparison.
- Datasets: SciNews, eLife and PLOS, with a fine-tuned 7B model.
- **Scale and shape.** Explanatory sentences make up about 19.02% and 18.19% of eLife and PLOS lay summaries. In their plans, how/why questions are 66.4% of the questions.
- **Control.** Deleting the plan questions of one relation type removes that kind of explanation from the output.
- Human-written summaries still win the human evaluation.

**Code as a plan** (MiDAS-PRo; Nandy & Bandyopadhyay 2025).
- The plan is written as Python code: which documents are cited together in each sentence, plus entity lists.
- ROUGE gains are small. The human evaluation used one expert and 12 samples.

**Quotes before prose** (Slobodkin et al. 2024, abstract only). Generation is split into "content selection, sentence planning, and sequential sentence generation", and the selected source spans double as citations.

**A prompt is not a plan** (ARC; Elaraby & Litman 2026, Appendix G). Telling a summariser to "Focus primarily on the key arguments" "did not consistently improve performance and, in some cases, even degraded coverage of salient roles".

### 4.6 Coverage and judging

**ARC** (Elaraby & Litman) has two domains:
- legal case opinions, with roles Issue, Reason and Conclusion;
- computer-graphics articles, with roles Own Claim, Background Claim and Data.

It splits each salient role into atomic facts, then labels each fact against the summary as supported, missing or contradicted.

**Omission dominates.** "salient information is more often omitted than misrepresented". For example, Qwen-14B on the legal set has 35.7% missing facts against 2.0% factual errors.

**Two biases.**
- Role bias favours conclusions over issues and reasons.
- In the legal set, 7.66% of the input carries argument roles, yet those sentences make up 66.51% of reference summaries. Coverage falls in a U shape with position, the middle of the input being covered worst.

The sparse-distribution and position findings hold for the legal domain only.

**LLM judges** (Altemeyer et al. 2025, on key-point summaries of debate arguments).
- An LLM asked to count which reference key points a summary covers correlates with human judges up to 0.767 across topics and 0.803 within them.
- Embedding-similarity scores do badly: BLEURT and BARTScore are the worst of the similarity functions tried.

### 4.7 Symbols

**ScholarPhi** (Head et al. 2021) shows definitions of terms and symbols at the point of use.

- **Nonce words.** The paper calls a term or symbol "coined for a particular use" a nonce word. The reader "cannot know precisely what it means when they start reading". This is the same coinage as the nonce renaming of 2026-10-10-simulated-readers, from the opposite side.
- **A formative study** (9 readers) found two strategies, both failure modes in the reading report:
  - infer the meaning from context;
  - defer, hoping for an explanation later. One reader then "may reach a point in the text where they lack an understanding of so many important terms that they can no longer understand the text".
- **Position-sensitive definitions** (design motivation M1). A symbol's tooltip shows "the definition that appeared most recently before that appearance". At the defining sentence it says "Defined here."
- **A usability study** (27 participants). Against a plain reader, ScholarPhi cut time per question by 45.4 s and the area of the paper viewed by 25%. Ease rose 1.70 points on a 5-point scale. Correctness did not change (difference 0%, p = 1.000).
- **A failure case.** Participants answered "What does T (upper case) mean?" wrongly "because the definitions list did not show all of the usages". A list that looks complete and is not misleads.
- **Definition extraction.** On the study paper it reached 24% precision and 6% recall, so the study used hand-curated definitions.

**How often symbols go undeclared.**
- Wolska & Grigore (2010; 500 symbols from 50 arXiv mathematics papers): 67.4% are explicitly declared and 26.7% are not. They note that "symbols are used before they have been introduced".
- Schubotz et al. (2016): "only 73% of the identifiers are explained in the text", a ceiling on recall for methods that read only the text.

**Symbols change meaning within a paper** (Asakura, Miyao & Aizawa 2022; 15 papers, 12,352 identifier occurrences).
- An identifier has 2.09 meanings on average, up to 14.
- Of 2,378 points where an identifier changes meaning, 89.5% fall within a single section.

**Symbol-to-description relations** (SemEval-2022 Task 12, Symlink). Among its relation types are DIRECT, a symbol and its definition, and COUNT, "a description of a concept with a symbol that is the number of instances of the concept". The demo's r, the number of coefficients in a record, is a COUNT relation.

**Definition detection** (Kang et al. 2020). Detection falls from 73.4 to 50.4 macro F1 between sentence level and document level, and often misses symbols. The authors leave open "Deciding when and where to show definitions based on context and information density".

**Unfamiliar terms** (Elhadad 2006). Unfamiliar terms are predicted from their frequency in a corpus the reader is known to understand, at 86.7% precision and 83% recall. Only "the first mention of a complex term" is defined, as a link rather than inline.

### 4.8 Situation models

**Accessibility** (Morrow, Greenspan & Bower 1987; abstract only). Readers memorised a building, then read about a character moving through it. "accessibility depended on the location of the protagonist in the situation model rather than recency of mention".

**The event-indexing model** (Zwaan, Langston & Graesser 1995; abstract only). It connects events along five dimensions: "time, space, protagonist, causality, and intentionality".

### 4.9 Reading psychology and simulation, briefly

All of these were checked. Section 8 gives the full references.

**Comprehension models.**
- **Construction-integration** (Kintsch 1988). Comprehension builds a network of propositions bottom-up, then settles it by spreading activation.
- **Inference** (Graesser, Singer & Trabasso 1994). Readers build a representation that "explains why actions, events, and states are mentioned in the text". The claim about which inferences are made online could be confirmed only from a secondary source, so it is not relied on here.

**How much explicitness helps.**
- **Coherence and prior knowledge** (McNamara, Kintsch, Songer & Kintsch 1996). Low-knowledge readers benefit from a coherent text and high-knowledge readers from a minimally coherent one. The effect appears at the level of the situation model (inference, problem-solving and sorting tasks), not the text base (recall). The follow-up (McNamara & Kintsch 1996) replicated the effect only in part.
- **Signalling** (Schneider et al. 2018). The numbers here come from a secondary summary only: 103 studies, 12,201 participants, retention g = 0.53, transfer g = 0.33, and no effect in mathematics.
- **Multimedia principles.** Cromley & Chen (2025): 591 effects, overall g = 0.37, declining by year. Mayer (2024): 15 principles.
- **Expertise reversal** (Kalyuga et al. 2003). Techniques that help inexperienced learners can lose their effect, or backfire, with experienced ones.
- **Coh-Metrix** (Graesser, McNamara & Kulikowich 2011). Its five factors are word concreteness, syntactic simplicity, referential cohesion, causal cohesion and narrativity.

**Reader activity.**
- **Self-explanation** (Chi et al. 1994). Students prompted to explain each line to themselves gained more from pretest to posttest.
- **Navigation.** Pirolli (2005, 2007) models link choice by information scent. Van Oostendorp & Aggarwal (2015) model navigation with backtracking and path adequacy.

**Simulated people.**
- **Hyper-accuracy** (Aher, Arriaga & Kalai 2023). "larger and more aligned LMs simulate human subjects that give unhumanly accurate answers".
- **Algorithmic fidelity** (Argyle et al. 2023): whether a model reproduces the response distributions of human subgroups, tested by four criteria.

**Learner models.**
- **Misconceptions in knowledge space theory** (Stefanutti, de Chiusole, Gondan & Maurer 2020). Misconceptions are modelled as buggy rules in a polytomous skill map, not as missing knowledge.
- **Knowledge tracing** (Corbett & Anderson 1995).

## 5. Synthesis

### 5.1 An aha moment is a reconnection to the goal

Hamami & Morris supply a definition that fits the 🏹 idea:
- a chain of reasoning is a path in a plan tree, from a step up to the root intention;
- not understanding is a break in that path;
- an aha moment is the point at which a step's link to the main objective becomes visible.

McKay's proof shows the effect on the reader: its steps reconnect "only when one reaches the equation r + sp = n^(p−1) towards the end".

The demo wiki shows the same thing across pages. `gaussian-dyadic` is where the tape-model constraint from `turing-machine-model` becomes visible as the reason for an otherwise incidental choice.

Three refinements come from the other papers:
- **Which link counts depends on the question.** It is whichever link explains the feature the reader finds striking (Lange). There is no single aha moment per page.
- **Mathematicians name two aha-like kinds of comprehension that are left out of the assessment model:** where a proof becomes non-routine, and why the obvious approach fails (Mejía-Ramos et al., footnote 1).
- **Readers do not agree on key ideas unprompted** (Yan). So they have to be marked, not left to inference.

### 5.2 Hops and chains

The wiki's `requires` edges support only tracking that goes step by step, in Hamami & Morris's sense of verification: each page can be checked against its prerequisites. A plan needs two things:
- each hop typed by what it does;
- the parent intention each page serves.

The author's connectives (via, reduced to, by) are that typing, written by hand. They are also eRST signals.

eRST's numbers locate the risk. Explanatory relations are the least often signalled, and justifying and causal links are the ones a single tree most often loses.

### 5.3 Symbols as actors

The symbol literature turns the 🚅 idea into checkable parts:
- **A binding.** Which concept the symbol denotes, and how: DIRECT, or COUNT like r.
- **A scope.** Meanings switch within sections (Asakura et al.). The demo's q, y, w, z and b switch across pages.
- **A provenance.** Where the binding is stated. Wolska & Grigore and Schubotz et al. put the share of symbols left undeclared at roughly a quarter or more in published papers.

Three findings bear on where a reminder belongs:
- **Focus, not distance.** Accessibility follows the protagonist's location rather than recency of mention (Morrow et al.). The protagonist of a derivation is the object currently being worked on. On that reading, a symbol needs restating when the argument's focus has moved away from it, not after a fixed number of words. This is an analogy from narrative and has not been tested on technical text.
- **Entities through their puzzle.** A new entity earns its place by the puzzle it resolves (Lange §7). That gives the "actor" introduction: say what part the symbol will play.
- **Partial lists mislead.** A notation list that looks complete but is not misleads (ScholarPhi's T).

### 5.4 Fragmentation and collapse

The tension the author named has support on both sides:
- Less coherent text helps knowledgeable readers at the level of the situation model (McNamara et al.).
- Signalling's average effect is positive, though reportedly absent in mathematics (Schneider et al., secondary source).
- Guidance can backfire for experts (Kalyuga et al.).

A plan rendered as a separate view, a chain on the front page or in a part intro, adds structure without adding words to the pages. The amount of explicitness on a page stays a per-reader setting (I-15).

### 5.5 What the simulation can and cannot claim

A simulated reader that reports an aha moment is evidence of nothing (Aher et al.'s hyper-accuracy). Aha moments have to be found from behaviour:
- which quoted sentence carries a key point into correct answers, across runs;
- whether hiding a page breaks goals (I-51).

The 2026-10-11-checkpoint-held-work caveat stands: repairs quote missed key points, so only held-out goals count as evidence that the wiki teaches.

## 6. Proposals

None of these is implemented. The IDs continue the sequence in 2026-10-10-ideas-bank, which is left unedited.

| ID | Proposal | Grounding |
|---|---|---|
| I-85 | **Plan task.** After the pages are written, an agent writes the plan tree for the target. Each node is a show-intention mapped to one or more pages. Each parent→children step names the method (reduce, compute via, bound) and a connective phrase. Submit checks: the root is the target; every page appears; children respect `requires` order. | Hamami & Morris 2021, 2024 |
| I-86 | **Signalled edges.** Each `requires` edge gains a relation from a small set taken from eRST: means, background, evidence or justify, elaboration, result, restatement. It also gains a connective phrase and an anchor quote from the dependent page, checked word for word as `<cite>` quotes are. "Before this page" and the Connections diagram show them. The front page and part intros can render a chain such as "Exact integer multiplication ↬via Radix-digit convolution ↬reduced to Cyclic convolution ↬by DFT". | eRST; the reading report |
| I-87 | **Salient feature.** Each result page records the feature of the result that is striking. A goal asks why that feature holds, and its key point must trace the feature to a matching feature of the setup. | Lange 2024 |
| I-88 | **Goal kinds.** Replace recall and transfer with the seven dimensions of the assessment model, plus three kinds: where the argument becomes non-routine, why the obvious approach fails (footnote 1), and why an object is introduced where it is (Hamami & Morris's McKay questions). Poincaré's "why in this order" is the last kind applied to a chain. | Mejía-Ramos et al. 2012; Hamami & Morris 2024 |
| I-89 | **Key-point attribution.** The grader names, for each covered key point, the reader's citation that supports it. The report then lists the sentences that carry key points across runs: aha candidates found by behaviour, to be confirmed by ablation (I-51). | Section 5.5; Aher et al. 2023 |
| I-90 | **Symbol bindings.** Writers emit a binding for each symbol they introduce: symbol, meaning, relation (DIRECT or COUNT), and scope. Submit checks that each symbol in a page's mathematics is either bound on that page, bound on a prerequisite page and restated in a clause, or known to the reader profile. A wiki-wide check flags a letter bound to two meanings without a scope note. In the demo that is q, y, w, z and b. | ScholarPhi; Wolska & Grigore; Asakura et al.; Symlink; the demo |
| I-91 | **Position-sensitive symbol tooltips.** Through zensical's `content.tooltips`, which already shows gloss titles. The notation page states that it lists cross-page symbols only. | ScholarPhi M1 and its T failure |
| I-92 | **Role-tagged key points.** Key points are tagged as claim, reason, conclusion or background. Grades separate missing from contradicted. | ARC |
| I-93 | **Claim support for `<cite>`** (extends I-36). The verbatim check establishes that a quote exists. A second check would ask whether the quote supports the claim it follows. | ChatGPT's review of the journal; ARC's supported/missing/contradicted labels |
| I-94 | **Stuck-point levels.** Each stuck point is tagged with a level: symbol or term (semantic micro), statement status (logical micro), one inference (local), the central step (global), or beyond the text. The labels are diagnostic, not validated measures. | Spratte 2024 |
| I-95 | **Key-idea agreement.** Several readers each name a page's key idea. Disagreement becomes a rewrite note. | Yan 2019 |

**A possible order, for steering:**
1. **I-90.** It is deterministic and cheap, and the demo shows every failure it targets.
2. **I-86, then I-85.** Signalled edges come first because they are local to a page and its prerequisites. The plan builds on them.
3. **I-88 and I-89, with held-out goals** (2026-10-11-checkpoint-held-work, step 1). Attribution is meaningful only on goals that did not feed the repairs.
4. **Ablation (I-51)** to confirm the attributed sentences.

The adversarial misreader (I-57) gains a model from Stefanutti et al.: misconceptions are buggy rules, not missing knowledge.

## 7. Corrections to the chat's references

| As given in the chat | As found |
|---|---|
| "Hanami" (the author's recollection), Noûs 2024, sole author | Yacin Hamami and Rebecca Lea Morris |
| Lange, 2024 or 2025 | Journal of Mathematical Behavior 76 (December 2024), article 101191. The PDF URL's 2025 is its upload date. |
| ARC, 2025 | EACL 2026. The sparse-distribution and position findings hold for the legal domain only. The summarisers were open models of 1B–14B parameters. |
| Liu et al.: "plans explanatory content using discourse structure", two lay datasets | Three datasets (SciNews, eLife, PLOS). Plans are answerless QUD questions derived from RST parses of reference summaries, with a fine-tuned model, not prompting. |
| MiDAS-PRo as a strong applied precedent | AAAI 2025, no arXiv version. ROUGE gains are small. The human evaluation used one expert on 12 samples, and its attribution scores tie with a BM25 baseline. |
| Altemeyer et al.: "argument summaries" | Key-point summaries of short debate arguments, not long documents. Seven authors. |
| eRST: "implicit relations" are represented | Unsignalled relations are allowed. Implicit connectives are not annotated; the authors leave them to future work. |
| ScholarPhi, 2020; a "uses view" and an "automatic glossary" | CHI 2021. Buttons open lists of definitions, defining formulae and usages. The glossary is a "priming glossary". |
| "Connecting mathematical symbols with natural language (2023)" | Not found. Not cited here. Wolska & Grigore 2010, Asakura et al. 2022 and Symlink 2022 cover the described ground. |
| Yan 2019, with Mason or Hanna implied | Yan is the sole author. Only the abstract was available. |
| "Modeling misconceptions in knowledge space theory", authors unnamed | Stefanutti, de Chiusole, Gondan & Maurer, J. Math. Psych. 99 (2020) |
| "Modeling and supporting web-navigation" (2015), authors unnamed | van Oostendorp & Aggarwal, Journal of Interaction Science 3 |
| Coh-Metrix dimensions including "deep cohesion" | The 2011 abstract names causal cohesion. "Deep cohesion" is the label in the later T.E.R.A. tool. |
| McNamara & Kintsch 1996: high-knowledge readers benefit from less coherent text | The effect appears on situation-model measures only, and partly failed to replicate between the 1996 paper's experiments |
| Schneider et al. 2018: 103 studies, 12,201 participants | Confirmed only from a secondary summary, which also reports no effect in mathematics |
| Poincaré 1908, p. 118 | p. 118 of Maitland's 1914 English translation, as cited by Hamami & Morris |

## 8. Research register

Kinds: J = peer-reviewed journal, C = peer-reviewed conference, W = workshop, P = preprint, B = book.

How each source bears on exegenius:
- **supports**: evidence for something already built;
- **proposal**: grounds a section 6 proposal;
- **background**: informs the design without a direct mechanism.

| Reference | Kind | Read | Bears on exegenius as |
|---|---|---|---|
| Hamami, Y. & Morris, R. L. (2024). Understanding in mathematics: The case of mathematical proofs. *Noûs* 58(4):1073–1106. doi:10.1111/nous.12489 | J | full (author preprint) | proposal: I-85, I-88; defines the aha moment (5.1) |
| Hamami, Y. & Morris, R. L. (2021). Plans and planning in mathematical proofs. *Review of Symbolic Logic* 14(4):1030–1065. doi:10.1017/S1755020319000601 | J | full (author preprint) | proposal: I-85 |
| Lange, M. (2024). What are explanatory proofs in mathematics and how can they contribute to teaching and learning? *Journal of Mathematical Behavior* 76:101191. doi:10.1016/j.jmathb.2024.101191 | J | full | proposal: I-87 |
| Weber, K. & Mejía-Ramos, J. P. (2011). Why and how mathematicians read proofs: an exploratory study. *Educational Studies in Mathematics* 76(3):329–344. doi:10.1007/s10649-010-9292-z | J | full (author manuscript) | background (zooming in and out) |
| Mejía-Ramos, J. P., Fuller, E., Weber, K., Rhoads, K. & Samkoff, A. (2012). An assessment model for proof comprehension in undergraduate mathematics. *Educational Studies in Mathematics* 79(1):3–18. doi:10.1007/s10649-011-9349-7 | J | full (author manuscript) | proposal: I-88 |
| Yan, X. (2019). Key ideas in a proof: The case of the irrationality of √2. *Journal of Mathematical Behavior* 55:100702. doi:10.1016/j.jmathb.2019.04.001 | J | abstract | proposal: I-95 |
| Spratte, V. (2024). Models for proof comprehension in secondary and tertiary education: Uniting the perspectives. *mathematica didactica* 47. doi:10.18716/ojs/md/2024.1655 | J | full | proposal: I-94 |
| Zeldes, A., Aoyama, T., Liu, Y. J., Peng, S., Das, D. & Gessler, L. (2025). eRST: A signaled graph theory of discourse relations and organization. *Computational Linguistics* 51(1):23–72. doi:10.1162/coli_a_00538 | J | full (arXiv 2403.13560) | proposal: I-86 |
| Liu, D., Yu, X., Demberg, V. & Lapata, M. (2025). Explanatory summarization with discourse-driven planning. *TACL* 13:1146–1170. doi:10.1162/tacl.a.30 | J | full (arXiv 2504.19339) | proposal: I-85 (a plan before prose) |
| Nandy, A. & Bandyopadhyay, S. (2025). Language models of code are few-shot planners and reasoners for multi-document summarization with attribution. *AAAI* 39(23):24930–24938. doi:10.1609/aaai.v39i23.34676 | C | full | background (weak results) |
| Slobodkin, A., Hirsch, E., Cattan, A., Schuster, T. & Dagan, I. (2024). Attribute first, then generate: Locally-attributable grounded text generation. *ACL 2024*:3309–3344. doi:10.18653/v1/2024.acl-long.182 | C | abstract | background (quotes before prose, as with `<cite>`) |
| Elaraby, M. & Litman, D. (2026). ARC: Argument representation and coverage analysis for zero-shot long document summarization with instruction following LLMs. *EACL 2026*:3626–3643. doi:10.18653/v1/2026.eacl-long.167 | C | full (arXiv 2505.23654) | proposal: I-92 |
| Altemeyer, M., Eger, S., Daxenberger, J., Chen, Y., Altendorf, T., Cimiano, P. & Schiller, B. (2025). Argument summarization and its evaluation in the era of large language models. *EMNLP 2025*:35490–35511. doi:10.18653/v1/2025.emnlp-main.1797 | C | full (arXiv 2503.00847) | supports: grading by key-point counting (grade.rs), in another domain |
| Head, A., Lo, K., Kang, D., Fok, R., Skjonsberg, S., Weld, D. S. & Hearst, M. A. (2021). Augmenting scientific papers with just-in-time, position-sensitive definitions of terms and symbols. *CHI 2021*. doi:10.1145/3411764.3445648 | C | full (arXiv 2009.14237) | proposal: I-90, I-91 |
| Wolska, M. & Grigore, M. (2010). Symbol declarations in mathematical writing: A corpus study. *Towards a Digital Mathematics Library (DML 2010)*:119–127. http://dml.cz/dmlcz/702580 | W | full | proposal: I-90 |
| Asakura, T., Miyao, Y. & Aizawa, A. (2022). Building dataset for grounding of formulae: annotating coreference relations among math identifiers. *LREC 2022*:4851–4858. https://aclanthology.org/2022.lrec-1.519 | C | full | proposal: I-90 (scope) |
| Lai, V. D., Pouran Ben Veyseh, A., Dernoncourt, F. & Nguyen, T. H. (2022). SemEval 2022 Task 12: Symlink, linking mathematical symbols to their descriptions. *SemEval-2022*:1671–1678. https://aclanthology.org/2022.semeval-1.230 | W | full (taxonomy) | proposal: I-90 (DIRECT, COUNT) |
| Schubotz, M. et al. (2016). Semantification of identifiers in mathematics for better math information retrieval. *SIGIR 2016*:135–144. doi:10.1145/2911451.2911503 | C | full (preprint) | background |
| Kang, D., Head, A., Sidhu, R., Lo, K., Weld, D. & Hearst, M. (2020). Document-level definition detection in scholarly documents. *SDP 2020*:196–206. doi:10.18653/v1/2020.sdp-1.22 | W | full | background (extraction after the fact is weak, so writers emit bindings) |
| Elhadad, N. (2006). Comprehending technical texts: Predicting and defining unfamiliar terms. *AMIA Annu Symp Proc* 2006:239–243. PMC1839621 | C | full | background (familiarity relative to the reader) |
| Morrow, D. G., Greenspan, S. L. & Bower, G. H. (1987). Accessibility and situation models in narrative comprehension. *Journal of Memory and Language* 26(2):165–187. doi:10.1016/0749-596X(87)90122-7 | J | abstract | background (5.3, untested analogy) |
| Zwaan, R. A., Langston, M. C. & Graesser, A. C. (1995). The construction of situation models in narrative comprehension: An event-indexing model. *Psychological Science* 6(5):292–297. doi:10.1111/j.1467-9280.1995.tb00513.x | J | abstract | background |
| Kintsch, W. (1988). The role of knowledge in discourse comprehension: A construction-integration model. *Psychological Review* 95(2):163–182. doi:10.1037/0033-295X.95.2.163 | J | abstract | background |
| Graesser, A. C., Singer, M. & Trabasso, T. (1994). Constructing inferences during narrative text comprehension. *Psychological Review* 101(3):371–395. doi:10.1037/0033-295X.101.3.371 | J | abstract | background |
| McNamara, D. S., Kintsch, E., Songer, N. B. & Kintsch, W. (1996). Are good texts always better? *Cognition and Instruction* 14(1):1–43. doi:10.1207/s1532690xci1401_1 | J | abstract | supports: per-profile explicitness (I-15) |
| McNamara, D. S. & Kintsch, W. (1996). Learning from texts: Effects of prior knowledge and text coherence. *Discourse Processes* 22(3):247–288. doi:10.1080/01638539609544975 | J | abstract | background (partial replication) |
| Kalyuga, S., Ayres, P., Chandler, P. & Sweller, J. (2003). The expertise reversal effect. *Educational Psychologist* 38(1):23–31. doi:10.1207/S15326985EP3801_4 | J | abstract | supports: I-15 |
| Chi, M. T. H., de Leeuw, N., Chiu, M.-H. & LaVancher, C. (1994). Eliciting self-explanations improves understanding. *Cognitive Science* 18(3):439–477. doi:10.1207/s15516709cog1803_3 | J | abstract | background |
| Schneider, S., Beege, M., Nebel, S. & Rey, G. D. (2018). A meta-analysis of how signaling affects learning with media. *Educational Research Review* 23:1–24. doi:10.1016/j.edurev.2017.11.001 | J | secondary summary | background (5.4) |
| Graesser, A. C., McNamara, D. S. & Kulikowich, J. M. (2011). Coh-Metrix: Providing multilevel analyses of text characteristics. *Educational Researcher* 40(5):223–234. doi:10.3102/0013189X11413260 | J | abstract | background |
| Mayer, R. E. (2024). The past, present, and future of the cognitive theory of multimedia learning. *Educational Psychology Review* 36:8. doi:10.1007/s10648-023-09842-1 | J | abstract | background |
| Cromley, J. G. & Chen, R. (2025). A meta-analysis of Richard Mayer's multimedia learning research. *Educational Research Review* 49:100730. doi:10.1016/j.edurev.2025.100730 | J | abstract | background |
| Pirolli, P. (2005). Rational analyses of information foraging on the web. *Cognitive Science* 29(3):343–373. doi:10.1207/s15516709cog0000_20 | J | abstract | background (I-55) |
| Pirolli, P. (2007). *Information Foraging Theory: Adaptive Interaction with Information*. Oxford University Press. doi:10.1093/acprof:oso/9780195173321.001.0001 | B | metadata | background (I-55) |
| van Oostendorp, H. & Aggarwal, S. (2015). Modeling and supporting web-navigation. *Journal of Interaction Science* 3:3. doi:10.1186/s40166-015-0008-9 | J | abstract | background (I-54) |
| Aher, G., Arriaga, R. I. & Kalai, A. T. (2023). Using large language models to simulate multiple humans and replicate human subject studies. *ICML 2023*, PMLR 202:337–371 | C | full (arXiv 2208.10264) | supports: no-wiki baseline, nonce renaming |
| Argyle, L. P., Busby, E. C., Fulda, N., Gubler, J. R., Rytting, C. & Wingate, D. (2023). Out of one, many: Using language models to simulate human samples. *Political Analysis* 31(3):337–351. doi:10.1017/pan.2023.2 | J | abstract and arXiv | background (I-50 model ladder) |
| Stefanutti, L., de Chiusole, D., Gondan, M. & Maurer, A. (2020). Modeling misconceptions in knowledge space theory. *Journal of Mathematical Psychology* 99:102435. doi:10.1016/j.jmp.2020.102435 | J | abstract | background (I-57) |
| Corbett, A. T. & Anderson, J. R. (1995). Knowledge tracing: Modeling the acquisition of procedural knowledge. *User Modeling and User-Adapted Interaction* 4(4):253–278. doi:10.1007/BF01099821 | J | metadata | background (I-13) |
| Yuan, Z., Xiao, Y., Li, M., Xuan, W., Tong, R., Diab, M. & Mitchell, T. (2026). Towards valid student simulation with large language models. arXiv:2601.05473 | P | abstract | supports: reader instructions as an epistemic state specification (2026-10-10-origin) |

## Current State
- Concept prerequisites are an untyped list of concept ids — no relation type, connective or anchor quote is stored per edge (exegenius-core/src/graph.rs:30)
- `render_page` writes the "Before this page" admonition as "This page builds on A, B" with no statement of what each prerequisite contributes (exegenius-core/src/build.rs:357-373)
- The Connections diagram draws prerequisite and dependent edges as index pairs with no labels (exegenius-core/src/build.rs:288, exegenius-core/src/build.rs:379-390)
- The outline task asks for an overview naming "the route the wiki takes to it, part by part", with no structured plan behind the overview (exegenius-core/src/outline.rs:41-43)
- The write instructions ask the writer to define every term or symbol on the page where it is first used, and no check enforces the instruction (exegenius-core/src/write.rs:26)
- `apply_notation` checks only that each listed symbol occurs in the mathematics of its named page — symbols that a page uses without definition, and letters bound to different meanings on different pages, pass unchecked (exegenius-core/src/notation.rs:107-142)
- Goal kinds are `Recall` and `Transfer` only (exegenius-core/src/simulate.rs:86-89)
- The read instructions ask readers to record places they got past only by guessing, so guessed symbols reach the report as stuck points when readers report them (exegenius-core/src/simulate.rs:66-67)
- A grade records the numbers of the covered key points and the wrong claims, without linking a key point to the reader citation that supports it (exegenius-core/src/grade.rs:35-42)
- `cite::check` verifies that each cited quote occurs word for word in its source, and does not test whether the quote supports the claim it follows (exegenius-core/src/cite.rs:64-80)

## Missing
- No plan tree of show-intentions over the pages (I-85)
- No relation type, connective or anchor quote on prerequisite edges (I-86)
- No salient-feature field on result pages (I-87)
- No goal kinds beyond recall and transfer (I-88)
- No attribution of covered key points to reader citations (I-89)
- No symbol bindings, no undefined-symbol check, and no check for one letter bound to several meanings (I-90)
- No position-sensitive symbol tooltips (I-91)
- No role tags on key points and no missing/contradicted split in grades (I-92)
- No claim-support check for citations (I-93)
- No level tags on stuck points (I-94)
- No key-idea agreement check across readers (I-95)

# 2026-10-11: Research: explanatory structure

This entry follows 2026-10-11-checkpoint-held-work. It is the hub for a group of research entries, and it records five things:

1. The author's reading of the deployed demo wiki (section 1).
2. Two ideas written down after that reading (section 2).
3. The literature search the author ran in a separate ChatGPT session (section 3).
4. The order of the seven research entries that develop the ideas, and the dependencies between them (section 4).
5. What the chat got wrong about its references (section 5), and a register of every source with the entries that use it (section 6).

The first version of this entry held the literature, the synthesis and proposals I-85 to I-95 in one place. They are now split into one entry per idea. Each entry has:
- its own literature;
- a design;
- a test with the simulated readers;
- risks;
- open questions for steering;
- a Giacometti tail.

Every reference was checked against the paper itself. Quotes in the research entries carry the page numbers of the version that was read. That version is named where it is a preprint or author manuscript, and an abstract-only reading is marked.

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
- The strand the author recognised gives the theory of the aha moment (2026-10-11-proof-plans, 2026-10-11-aha-moments). Hamami & Morris quote Poincaré and Bourbaki on understanding a proof versus verifying it.
- The strand the author judged off target holds the most directly usable empirical data on symbols (2026-10-11-symbol-anchoring).

## 4. The research entries, in order

| # | Entry | The question it answers | Proposals | Builds on |
|---|---|---|---|---|
| 1 | 2026-10-11-symbol-anchoring | What must the wiki state about each symbol so that a reader never has to guess? | I-90, I-91 | none |
| 2 | 2026-10-11-signalled-edges | What must a link between pages carry for a chain to read as a chain rather than a series of single hops? | I-86 | none |
| 3 | 2026-10-11-proof-plans | How can the wiki carry the plan of the argument (what each page is for) as well as its concepts? | I-85 | 2 |
| 4 | 2026-10-11-comprehension-goals | What must goal questions ask for the simulation to see the effects of 1, 2, 3 and 5? | I-88, I-92, I-94; held-out goals | none (its `motivation` goals draw on 3 once it exists) |
| 5 | 2026-10-11-aha-moments | How are the sentences that close a chain of reasoning found, confirmed and used? | I-87, I-89, I-95; I-51 | 3, 4 |
| 6 | 2026-10-11-fragmentation-and-collapse | Where do the outputs of 1, 2, 3 and 5 appear, and how is crowding of the pages detected? | none | 1, 2, 3, 5 |
| 7 | 2026-10-11-evidence-layers | Does each cited quote support the claim it is attached to? | I-93 (extends I-36) | none |

The dependencies, as edges:
- **2 → 3.** The connectives of signalled edges are the vocabulary for a plan node's method.
- **3 → 5.** The plan supplies structural candidates for aha sentences: sentences that join distant nodes.
- **4 → 5.** Confirming an aha sentence needs held-out goals and key-point attribution.
- **1, 2, 3, 5 → 6.** Fragmentation and collapse decides where each output is shown.
- **7** depends on nothing and nothing depends on it.

### Why this order

1. **Symbol anchoring comes first.**
   - It is deterministic, and every failure it targets is documented in the demo, with quotes.
   - It needs nothing from the other entries.
   - Its root cause is already found, at no agent cost, in the demo project's own records: r reached the writer and the auditor inside the known concept name "Polynomial ring modulo y^r + 1", so the audit never reported it.
2. **Signalled edges come before proof plans.**
   - An edge's role involves one page and its prerequisites, so it fits inside the existing write task.
   - A plan involves the whole argument and needs a vocabulary for its methods, which the connectives supply.
3. **Proof plans** come third. They change the most: a new task, writer context, the outline's parts, and the breadcrumb. They are also the entry closest to Hamami & Morris's account, which grounds the 🏹 idea.
4. **Comprehension goals run alongside 1–3.**
   - Effects cannot be measured without instruments.
   - The held-out goals carried over from 2026-10-11-checkpoint-held-work are needed by any claim that the wiki teaches.
   - It sits fourth in the list because its most distinctive kinds (`motivation`, `structure`, `summary`) are best written against a plan.
5. **Aha moments** come after plans and goals, because they need both. They also absorb the leave-one-out ablation (I-51), applied to candidate sentences as well as pages.
6. **Fragmentation and collapse** comes last among the dependent entries: it decides presentation once there is something to present. Its cheapest measure, body word count per page per round, can be added at any time and is worth having before more repair rounds run.
7. **Evidence layers** can go anywhere. Its support check is independent of the rest and could run on the demo's 47 citation marks today.

### The question each entry has to settle first

The first three were settled after steering on 2026-10-11, and the entries record how.

| Entry | Question | How it is settled |
|---|---|---|
| symbol-anchoring | Settled: an agent accounts for every symbol with evidence, and code checks the evidence; no tokenizer | The demo's audit records (`exegenius/audit.json` in lmmx/openai-nlogn-wiki) |
| signalled-edges | Settled: a relation is a natural-language predicate, with no fixed set of types | Steering |
| proof-plans | Settled: before the pages, as a top-down reading of the source; infer-leaves follow the source's order; pages are a choice of modules | Hamami & Morris, as derived in the entry's section "How Hamami & Morris's account determines the design" |
| comprehension-goals | Which goal kinds come first, and how are goals split into held-out and repair goals? | Steering, then one demo round |
| aha-moments | Where is the salient feature recorded, and is ablation done by sentence or by paragraph? | Steering. The lens change is the same either way. |
| fragmentation-and-collapse | Which page chrome folds by default, and is the significance view public? | Steering, after 1–3 render |
| evidence-layers | When does the support check run? | Steering |

### Proposals by entry

| ID | Proposal | Entry |
|---|---|---|
| I-85 | Plan tree of show and infer intentions, with methods and motivations | proof-plans |
| I-86 | Relation, connective and anchor quote on each prerequisite edge | signalled-edges |
| I-87 | Salient feature recorded for result pages | aha-moments |
| I-88 | Goal kinds from the proof-comprehension assessment model, plus motivation, non-routine and obvious-fails | comprehension-goals |
| I-89 | Attribution of covered key points to reader citations | aha-moments |
| I-90 | Symbol bindings stated by writers, and checked | symbol-anchoring |
| I-91 | Position-sensitive symbol tooltips | symbol-anchoring |
| I-92 | Role-tagged key points; covered, missing or contradicted per key point | comprehension-goals |
| I-93 | Claim-support labels for citation marks | evidence-layers |
| I-94 | Levels on stuck points | comprehension-goals |
| I-95 | Agreement across readers on a page's key idea | aha-moments |

The IDs continue the sequence in 2026-10-10-ideas-bank, which is left unedited.

### Where the held-off work went

The items in 2026-10-11-checkpoint-held-work now sit in these entries:

| Held-off item | Now in |
|---|---|
| Held-out goals | comprehension-goals |
| Adversarial misreader (I-57, I-58) | evidence-layers, as the reader-side test of claim support. Stefanutti et al. (2020) model misconceptions as buggy rules, which gives the misreader a structure. |
| Leave-one-out ablation (I-51) | aha-moments, applied to sentences as well as pages |
| Ladder of reader profiles and models (I-07, I-50) | aha-moments (pivotal sentences that hold across models) and fragmentation-and-collapse (the expertise-reversal test) |
| Failed searches as signals (I-55) | signalled-edges, under information scent |
| Front page not audited | proof-plans (the front page shows the plan) and fragmentation-and-collapse |

## 5. Corrections to the chat's references

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

## 6. Research register

Kinds: J = peer-reviewed journal, C = peer-reviewed conference, W = workshop, P = preprint, B = book. "Used in" names the research entries (dates omitted, all 2026-10-11) that draw on each source. "this entry only" marks background cited here and in no research entry.

| Reference | Kind | Read | Used in |
|---|---|---|---|
| Hamami, Y. & Morris, R. L. (2024). Understanding in mathematics: The case of mathematical proofs. *Noûs* 58(4):1073–1106. doi:10.1111/nous.12489 | J | full (author preprint) | proof-plans, aha-moments, comprehension-goals, symbol-anchoring |
| Hamami, Y. & Morris, R. L. (2021). Plans and planning in mathematical proofs. *Review of Symbolic Logic* 14(4):1030–1065. doi:10.1017/S1755020319000601 | J | full (author preprint) | proof-plans, signalled-edges, symbol-anchoring |
| Lange, M. (2024). What are explanatory proofs in mathematics and how can they contribute to teaching and learning? *Journal of Mathematical Behavior* 76:101191. doi:10.1016/j.jmathb.2024.101191 | J | full | aha-moments, proof-plans, symbol-anchoring |
| Weber, K. & Mejía-Ramos, J. P. (2011). Why and how mathematicians read proofs: an exploratory study. *Educational Studies in Mathematics* 76(3):329–344. doi:10.1007/s10649-010-9292-z | J | full (author manuscript) | proof-plans, comprehension-goals, fragmentation-and-collapse |
| Mejía-Ramos, J. P., Fuller, E., Weber, K., Rhoads, K. & Samkoff, A. (2012). An assessment model for proof comprehension in undergraduate mathematics. *Educational Studies in Mathematics* 79(1):3–18. doi:10.1007/s10649-011-9349-7 | J | full (author manuscript) | comprehension-goals, aha-moments |
| Yan, X. (2019). Key ideas in a proof: The case of the irrationality of √2. *Journal of Mathematical Behavior* 55:100702. doi:10.1016/j.jmathb.2019.04.001 | J | abstract | aha-moments |
| Spratte, V. (2024). Models for proof comprehension in secondary and tertiary education: Uniting the perspectives. *mathematica didactica* 47. doi:10.18716/ojs/md/2024.1655 | J | full | comprehension-goals, symbol-anchoring, aha-moments |
| Zeldes, A., Aoyama, T., Liu, Y. J., Peng, S., Das, D. & Gessler, L. (2025). eRST: A signaled graph theory of discourse relations and organization. *Computational Linguistics* 51(1):23–72. doi:10.1162/coli_a_00538 | J | full (arXiv 2403.13560) | signalled-edges, aha-moments |
| Liu, D., Yu, X., Demberg, V. & Lapata, M. (2025). Explanatory summarization with discourse-driven planning. *TACL* 13:1146–1170. doi:10.1162/tacl.a.30 | J | full (arXiv 2504.19339) | signalled-edges, proof-plans, aha-moments, fragmentation-and-collapse |
| Nandy, A. & Bandyopadhyay, S. (2025). Language models of code are few-shot planners and reasoners for multi-document summarization with attribution. *AAAI* 39(23):24930–24938. doi:10.1609/aaai.v39i23.34676 | C | full | proof-plans, evidence-layers |
| Slobodkin, A., Hirsch, E., Cattan, A., Schuster, T. & Dagan, I. (2024). Attribute first, then generate: Locally-attributable grounded text generation. *ACL 2024*:3309–3344. doi:10.18653/v1/2024.acl-long.182 | C | abstract | proof-plans, evidence-layers |
| Elaraby, M. & Litman, D. (2026). ARC: Argument representation and coverage analysis for zero-shot long document summarization with instruction following LLMs. *EACL 2026*:3626–3643. doi:10.18653/v1/2026.eacl-long.167 | C | full (arXiv 2505.23654) | comprehension-goals, evidence-layers, proof-plans |
| Altemeyer, M., Eger, S., Daxenberger, J., Chen, Y., Altendorf, T., Cimiano, P. & Schiller, B. (2025). Argument summarization and its evaluation in the era of large language models. *EMNLP 2025*:35490–35511. doi:10.18653/v1/2025.emnlp-main.1797 | C | full (arXiv 2503.00847) | comprehension-goals, fragmentation-and-collapse |
| Head, A., Lo, K., Kang, D., Fok, R., Skjonsberg, S., Weld, D. S. & Hearst, M. A. (2021). Augmenting scientific papers with just-in-time, position-sensitive definitions of terms and symbols. *CHI 2021*. doi:10.1145/3411764.3445648 | C | full (arXiv 2009.14237) | symbol-anchoring, fragmentation-and-collapse |
| Wolska, M. & Grigore, M. (2010). Symbol declarations in mathematical writing: A corpus study. *Towards a Digital Mathematics Library (DML 2010)*:119–127. http://dml.cz/dmlcz/702580 | W | full | symbol-anchoring |
| Asakura, T., Miyao, Y. & Aizawa, A. (2022). Building dataset for grounding of formulae: annotating coreference relations among math identifiers. *LREC 2022*:4851–4858. https://aclanthology.org/2022.lrec-1.519 | C | full | symbol-anchoring |
| Lai, V. D., Pouran Ben Veyseh, A., Dernoncourt, F. & Nguyen, T. H. (2022). SemEval 2022 Task 12: Symlink, linking mathematical symbols to their descriptions. *SemEval-2022*:1671–1678. https://aclanthology.org/2022.semeval-1.230 | W | full (taxonomy) | symbol-anchoring |
| Schubotz, M. et al. (2016). Semantification of identifiers in mathematics for better math information retrieval. *SIGIR 2016*:135–144. doi:10.1145/2911451.2911503 | C | full (preprint) | symbol-anchoring |
| Kang, D., Head, A., Sidhu, R., Lo, K., Weld, D. & Hearst, M. (2020). Document-level definition detection in scholarly documents. *SDP 2020*:196–206. doi:10.18653/v1/2020.sdp-1.22 | W | full | symbol-anchoring |
| Elhadad, N. (2006). Comprehending technical texts: Predicting and defining unfamiliar terms. *AMIA Annu Symp Proc* 2006:239–243. PMC1839621 | C | full | symbol-anchoring, fragmentation-and-collapse |
| Morrow, D. G., Greenspan, S. L. & Bower, G. H. (1987). Accessibility and situation models in narrative comprehension. *Journal of Memory and Language* 26(2):165–187. doi:10.1016/0749-596X(87)90122-7 | J | abstract | symbol-anchoring |
| Zwaan, R. A., Langston, M. C. & Graesser, A. C. (1995). The construction of situation models in narrative comprehension: An event-indexing model. *Psychological Science* 6(5):292–297. doi:10.1111/j.1467-9280.1995.tb00513.x | J | abstract | symbol-anchoring |
| Kintsch, W. (1988). The role of knowledge in discourse comprehension: A construction-integration model. *Psychological Review* 95(2):163–182. doi:10.1037/0033-295X.95.2.163 | J | abstract | this entry only |
| Graesser, A. C., Singer, M. & Trabasso, T. (1994). Constructing inferences during narrative text comprehension. *Psychological Review* 101(3):371–395. doi:10.1037/0033-295X.101.3.371 | J | abstract | this entry only |
| McNamara, D. S., Kintsch, E., Songer, N. B. & Kintsch, W. (1996). Are good texts always better? *Cognition and Instruction* 14(1):1–43. doi:10.1207/s1532690xci1401_1 | J | abstract | comprehension-goals, fragmentation-and-collapse |
| McNamara, D. S. & Kintsch, W. (1996). Learning from texts: Effects of prior knowledge and text coherence. *Discourse Processes* 22(3):247–288. doi:10.1080/01638539609544975 | J | abstract | fragmentation-and-collapse |
| Kalyuga, S., Ayres, P., Chandler, P. & Sweller, J. (2003). The expertise reversal effect. *Educational Psychologist* 38(1):23–31. doi:10.1207/S15326985EP3801_4 | J | abstract | fragmentation-and-collapse |
| Chi, M. T. H., de Leeuw, N., Chiu, M.-H. & LaVancher, C. (1994). Eliciting self-explanations improves understanding. *Cognitive Science* 18(3):439–477. doi:10.1207/s15516709cog1803_3 | J | abstract | aha-moments |
| Schneider, S., Beege, M., Nebel, S. & Rey, G. D. (2018). A meta-analysis of how signaling affects learning with media. *Educational Research Review* 23:1–24. doi:10.1016/j.edurev.2017.11.001 | J | secondary summary | aha-moments, fragmentation-and-collapse |
| Graesser, A. C., McNamara, D. S. & Kulikowich, J. M. (2011). Coh-Metrix: Providing multilevel analyses of text characteristics. *Educational Researcher* 40(5):223–234. doi:10.3102/0013189X11413260 | J | abstract | this entry only |
| Mayer, R. E. (2024). The past, present, and future of the cognitive theory of multimedia learning. *Educational Psychology Review* 36:8. doi:10.1007/s10648-023-09842-1 | J | abstract | this entry only |
| Cromley, J. G. & Chen, R. (2025). A meta-analysis of Richard Mayer's multimedia learning research. *Educational Research Review* 49:100730. doi:10.1016/j.edurev.2025.100730 | J | abstract | fragmentation-and-collapse |
| Pirolli, P. (2005). Rational analyses of information foraging on the web. *Cognitive Science* 29(3):343–373. doi:10.1207/s15516709cog0000_20 | J | abstract | signalled-edges |
| Pirolli, P. (2007). *Information Foraging Theory: Adaptive Interaction with Information*. Oxford University Press. doi:10.1093/acprof:oso/9780195173321.001.0001 | B | metadata | signalled-edges |
| van Oostendorp, H. & Aggarwal, S. (2015). Modeling and supporting web-navigation. *Journal of Interaction Science* 3:3. doi:10.1186/s40166-015-0008-9 | J | abstract | this entry only |
| Aher, G., Arriaga, R. I. & Kalai, A. T. (2023). Using large language models to simulate multiple humans and replicate human subject studies. *ICML 2023*, PMLR 202:337–371 | C | full (arXiv 2208.10264) | aha-moments, comprehension-goals |
| Argyle, L. P., Busby, E. C., Fulda, N., Gubler, J. R., Rytting, C. & Wingate, D. (2023). Out of one, many: Using language models to simulate human samples. *Political Analysis* 31(3):337–351. doi:10.1017/pan.2023.2 | J | abstract and arXiv | aha-moments |
| Stefanutti, L., de Chiusole, D., Gondan, M. & Maurer, A. (2020). Modeling misconceptions in knowledge space theory. *Journal of Mathematical Psychology* 99:102435. doi:10.1016/j.jmp.2020.102435 | J | abstract | this entry only |
| Corbett, A. T. & Anderson, J. R. (1995). Knowledge tracing: Modeling the acquisition of procedural knowledge. *User Modeling and User-Adapted Interaction* 4(4):253–278. doi:10.1007/BF01099821 | J | metadata | this entry only |
| Yuan, Z., Xiao, Y., Li, M., Xuan, W., Tong, R., Diab, M. & Mitchell, T. (2026). Towards valid student simulation with large language models. arXiv:2601.05473 | P | abstract | this entry only |
| Pólya, G. (1949). With, or without, motivation? *American Mathematical Monthly* 56(10):684–691. doi:10.2307/2305566 | J | metadata (cited via Hamami & Morris) | proof-plans |
| Morris, R. L. (2020). Motivated proofs: What they are, why they matter and how to write them. *Review of Symbolic Logic* 13(1):23–46. doi:10.1017/S1755020319000583 | J | metadata (cited via Hamami & Morris) | proof-plans |
| Leron, U. (1983). Structuring mathematical proofs. *American Mathematical Monthly* 90(3):174–185. doi:10.1080/00029890.1983.11971184 | J | metadata (cited via Hamami & Morris) | proof-plans |

## Current State
- Proposals I-85 to I-95 are recorded in docs/journal/2026-10-11-symbol-anchoring.md, docs/journal/2026-10-11-signalled-edges.md, docs/journal/2026-10-11-proof-plans.md, docs/journal/2026-10-11-comprehension-goals.md, docs/journal/2026-10-11-aha-moments.md, docs/journal/2026-10-11-fragmentation-and-collapse.md and docs/journal/2026-10-11-evidence-layers.md, and no source file implements them
- The deployed demo at https://openai-nlogn-wiki.vercel.app was built with v0.1, before the glossary and notation pages — /notation/ and /glossary/ return 404
- Crates are at version 0.4.0 (Cargo.toml)

## Missing
- No symbol bindings or symbol checks (2026-10-11-symbol-anchoring)
- No roles on prerequisite edges (2026-10-11-signalled-edges)
- No plan tree (2026-10-11-proof-plans)
- No goal kinds beyond recall and transfer, and no held-out goals (2026-10-11-comprehension-goals)
- No detection or confirmation of aha sentences (2026-10-11-aha-moments)
- No breadcrumb, mechanism view, significance view or crowding measures (2026-10-11-fragmentation-and-collapse)
- No claim-support check (2026-10-11-evidence-layers)

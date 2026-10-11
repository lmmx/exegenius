# 2026-10-11: Proof plans

This is the third of the research entries listed in 2026-10-11-explanatory-structure-research. It asks how the wiki can carry the plan of the argument, not only its concepts. It covers proposal I-85. Nothing here is implemented.

## The problem

Poincaré, as quoted by Hamami & Morris, separates readers who check each syllogism from readers who "want to know not only whether all the syllogisms of a demonstration are correct, but why they are linked together in one order rather than in another".

The wiki serves the first kind of reader:
- Every page explains one concept against its prerequisites.
- Every prerequisite is linked.
- A reader can check each page in turn.

Nothing in the wiki says why the pages are linked in this order, or which goal each page serves. The pieces that come closest are:
- the outline's overview, which names "the route the wiki takes to it, part by part" in free prose (exegenius-core/src/outline.rs:41-43);
- the writer's opening sentence, which says what the concept is for "on the way to the target" (exegenius-core/src/write.rs:21-23).

Neither is structured, and neither is checked.

The author found the plan by hand. They clicked back along a chain, chose the tape model as the point to work from, and read forward until the chain closed at "dyadic". Section 1 of 2026-10-11-explanatory-structure-research traces it.

The reading order (exegenius-core/src/plan.rs) is a topological sort of prerequisites. It is an order a reader can follow, not the order of the argument's intentions.

## What the literature gives

### Understanding a proof is reconstructing its plan

**Hamami & Morris (Noûs 2024; preprint pages).**

- **The account.** "understanding a proof requires the agent to rationally reconstruct the plan of the proof" (p. 4).
- **A plan** is "an ordered tree such that":
  1. "each node is a proving intention";
  2. "the root is the proving intention corresponding to the theorem at hand";
  3. "each set of ordered children of a given parent node is a subplan that has been obtained from the parent node through an instance of practical reasoning" (p. 8).
- **Two kinds of proving intention** (p. 6):
  - **To show** (P₁,…,Pₙ ⇒ C). It "cannot be fulfilled directly", so it has to be split into more specific intentions.
  - **To infer** (P₁,…,Pₙ ↪ C). It "can be fulfilled directly by actually carrying out the considered inference".
  - A vertical ↓ joins a parent to its subplan. In their figure, show-nodes are squares and infer-nodes circles.
- **Two components of understanding** (§3, p. 9):
  - the tracking part: reconstruct the tree;
  - the rationality part: reproduce the practical reasoning behind each parent→children step.
- **Verifying versus understanding** (p. 16). Verifying "evaluates one inference after the other in a linear way", with "no need to track the intentions and the plan construction of the author".
- **The order of construction.** "the last inference to be made in the proof activity already figures in the first construction step of the plan". The author's reading went backwards from the frame (the tape model) for this reason.
- **Partial understanding.** It "can be precisely localized" (p. 12): to the parent→children steps whose reasoning the reader cannot reproduce.
- **Top-down presentation.** The account makes understanding "a top-down process" (p. 30). Proofs that make their organisation salient "will make it easier to reconstruct the tree structure of the plan" (p. 31). They quote Robinson's criticism of proofs that are "merely dumps of the details".

**Hamami & Morris (Review of Symbolic Logic 2021; preprint pages).**

- **Methods.** A mathematical method is "a rule that transforms a proving intention into one or several other proving intentions arranged in a subplan" (§5.1). Methods can work forwards, backwards or both.
- **Text cues.** Show-intentions are signalled by proving verbs: "we show that", "we prove that", "we establish that" (p. 8). But "in most cases, the reader is expected to figure out by herself the proving intentions of the author(s)" (footnote 9).
- **Modules** (§7). "any subtree of P can be conceived as a module which can be described by stating the proving intention of type 'to show' at its root". They connect this to Leron's structural method (1983) and Lamport's structured proofs (1995, 2012).

### Unmotivated steps

- **Pólya (1949), "With, or without, motivation?"** calls a step that appears without its reasoning a "deus ex machina" and argues that such steps need motivations. Hamami & Morris use the term for Pólya's own c_i sequence in Carleman's inequality: "one might really wonder in this case where this ci sequence comes from" (RSL §5.2).
- **Morris (2020), "Motivated proofs"**, develops the notion of a motivated proof.
- **McKay's proof of Cauchy's theorem** is Hamami & Morris's worked example (Noûs §4.1).
  - Its steps reconnect to the goal only at the equation r + sp = n^(p−1) near the end.
  - Their diagnostic questions ("Why introduce this particular set S of p-tuples in G?") test the rationality part.
  - The fix is to give the reader "relevant information enabling her to reproduce the above instance of practical reasoning". In this case that is McKay's own remark about starting from p = 2.

### How readers use structure

- **Zooming** (Weber & Mejía-Ramos 2011). The authors describe mathematicians as "zooming in" on problematic parts and conjecture that they also zoom out to the high-level structure. Comprehension may consist of "coordinating inspections of the logical details of a proof while zooming out to see these logical details in the context of the methods they are being used to support" (manuscript pp. 24–25).
- **Modules in an assessment model** (Mejía-Ramos et al. 2012). "Identifying the modular structure" is one of its holistic dimensions. Its templates include:
  - "Identify the purpose of a module";
  - "Identify the logical relation between modules".

### A plan before the prose

Three generation papers build an explicit plan before writing:
- **Questions as a plan** (Liu et al. 2025): an ordered list of unanswered questions under discussion (QUD).
- **Code as a plan** (MiDAS-PRo; Nandy & Bandyopadhyay 2025): a citation layout written as code.
- **Spans as a plan** (Slobodkin et al. 2024): selected source spans, then a sentence plan.

**ARC** (Elaraby & Litman 2026, Appendix G) shows the alternative failing: telling a model to focus on the key arguments "did not consistently improve performance".

The evidence is about summaries, not wikis. Liu et al.'s plans also come from reference summaries rather than from the source. They show that a plan can steer content. They do not show that a plan an agent writes is the source's plan.

## How Hamami & Morris's account determines the design

The two papers say enough about how plans are built and read to fix the design, not just motivate it. Six of their claims each settle one design question. Page numbers are from the authors' preprints.

### 1. The source is a report of the plan's execution

"the written mathematical proof constitutes then a report of the corresponding proof activity, where the order of the deductive steps as they figure in the mathematical proof corresponds exactly to the order in which the deductive inferences are carried out in the corresponding proof activity" (RSL §6, p. 16).

They describe execution as a walk through the tree. In their Fraleigh example the agent addresses S0, then S1, S2, I1, S2,1, I2, then S3: depth first, children in order (RSL pp. 16–17).

So the infer-leaves of a plan, read depth first and left to right, are the source's deductive steps in the order the source reports them. That gives a check code can run:
- each infer-leaf carries a quote of the step as the source reports it;
- the quotes' positions in the source must not decrease along the depth-first order of leaves.

`Source::locate` already finds quotes in the normalised source text (exegenius-core/src/project.rs:105). A paper that proves a lemma in an appendix breaks the order. Such a leaf is marked `deferred`, which records a forward reference in the source.

### 2. The plan is reconstructed from the source, top down

Reconstructing the tree means "tracking the proving intentions of the author as they proceed from the initial intention to prove the theorem at hand to more and more specific intentions". The reader has to "realize whenever a proving intention has been turned into a subplan, and to identify the corresponding subplan" (Noûs, p. 9).

Their toy example reads the subplan straight off the text: "From the written proof, we can see that the plan of the author is first to show that n² + 3n is even when n is even, then to show that n² + 3n is even when n is odd, and finally to infer from this that n² + 3n is even" (Noûs, p. 10).

The work is top down: "the most natural way to proceed is to work downward from the root of the tree" (Noûs, p. 30). The deductive steps under lower intentions "are irrelevant and should be ignored" until the top is reconstructed.

The text supplies cues. Show-intentions are signalled by proving verbs, "we show that", "we prove that", "we establish that" (RSL p. 8). But the reader usually has to infer the intentions (RSL footnote 9).

This settles what the plan task is: a reading of the source, done top down. It is the tracking part of understanding, performed by an agent on the source. It needs the source and the concept graph, not the wiki's pages.

### 3. Each construction step applies a method, for a reason

A mathematical method is "a rule that transforms a proving intention into one or several other proving intentions arranged in a subplan" (RSL §5.1, p. 10). Methods can work forwards (from the hypotheses), backwards (from the goal) or both (RSL pp. 11–12).

The rationality part asks for "the practical decisions behind the choice of each of these steps". In the toy example the decision "simply consists in applying a well-known mathematical method ... prove it separately for the even and for the odd numbers" (Noûs, p. 11).

Where the decision is sophisticated, as with Pólya's c_i, "the plan of a proof activity is always going to be the result of a step by step construction where each step consists in an instance of practical reasoning" (RSL p. 16).

So every show-node needs two things: a method, stated in natural language as the connectives of 2026-10-11-signalled-edges are; and a motivation, the practical decision. The motivation's provenance is recorded as `explicit` (quoted from the source), `recoverable`, or `reconstructed` by the agent. Footnote 9 implies most will be reconstructed. Those are what readers have to be given (Noûs p. 21: "giving the reader relevant information enabling her to reproduce the above instance of practical reasoning").

### 4. Definitions enter as hypotheses

Every proving intention carries its hypotheses ("from [P₁,…,Pₙ]"). Definitions count among them (RSL footnote 14). A definition introduced mid-proof is added as a new hypothesis: of c_i, "he now adds this definition of the ci sequence as a new hypothesis. This is how the initial proving intention S0 is turned into the proving intention S1" (RSL p. 16).

So symbol bindings attach to plan nodes, and a binding holds in the subtree of the node that adds it. That is the scope rule 2026-10-11-symbol-anchoring adopts. Introducing an object without reason is the c_i case: a construction step whose motivation must be given.

### 5. Modules are subtrees, and a presentation is a choice of modules

"any subtree of P can be conceived as a module which can be described by stating the proving intention of type 'to show' at its root" (RSL §7). A modular presentation encapsulates chosen subtrees and states only their root intentions. "There are thus as many modular presentations of P as there are modular descriptions of P" (RSL footnote 31).

They tie this to Leron's structural method: "arrange the proof in levels, proceeding from the top down; the levels themselves consist of short autonomous 'modules,' each embodying one major idea of the proof" (Leron 1983, pp. 174–175, quoted in RSL). Leron's claimed benefit: "the main idea is given in the top level (or two), auxiliary ideas are packaged in autonomous modules, and the interconnections between the separate ideas are made explicit through the structure diagram" (p. 183, quoted in RSL).

A wiki is a modular presentation of the plan:
- each argument page is a module, titled by its root show-intention;
- the front page and part intros are the top levels;
- a page encapsulates its subtree, so the parent page states only the module's intention and links it.

Concept pages for objects and definitions, such as `gaussian-dyadic` or the polynomial ring, are not modules. They supply hypotheses to the nodes that use them, and the plan links them from those nodes.

### 6. Understanding can be localised, and measured by proportion

"because of the tree structure of proof plans, it is possible to identify the parts of a plan that have been rationally reconstructed in a successful way from those that were not". "The degree with which an agent understands a mathematical proof can be measured in terms of the proportion of the plan that she has rationally reconstructed in a successful way" (Noûs, p. 12).

So goals can be attached to plan nodes, and the report can show reader success per node: the degree of understanding in their sense, over the simulated readers.

## Design

### The plan file

The plan task writes `exegenius/argument.json`. The name avoids `plan`, which means the reading order (exegenius-core/src/plan.rs).

```json
{
  "id": "S0",
  "intention": "show",
  "claim": "two n-bit integers can be multiplied in O(n log n) steps on a multitape Turing machine",
  "from": ["multitape-turing-machine", "exact-integer-multiplication"],
  "method": "reduce to a multidimensional cyclic convolution and bound its cost",
  "motivation": {"text": "...", "source": "explicit", "quote": "..."},
  "adds": [{"symbol": "S", "name": "convolution length", "meaning": "..."}],
  "page": "main-theorem",
  "children": [ ... ]
}
```

The node fields:
- **`intention`**: `show` or `infer`.
- **`from`**: the hypotheses, as concept ids. These link the node to the concept pages it relies on.
- **`method`** and **`motivation`**: on show-nodes. The motivation's `source` is `explicit` (with a quote checked against the source), `recoverable` or `reconstructed`.
- **`adds`**: the definitions and symbol bindings introduced at this node, in force in its subtree.
- **`quote`**: on infer-leaves, the step as the source reports it.
- **`page`**: set on the nodes chosen as modules. Under claim 5, the set of nodes with a page is the wiki's modular description.
- **`deferred`**: marks an infer-leaf whose quote is out of source order.

### Checks at submit

Each check comes from one of the claims above:
1. **The root** states the target, and its `page` is the target page.
2. **Report order** (claim 1). Along a depth-first walk, infer-leaf quotes occur in the source in non-decreasing position, except leaves marked `deferred`. Every quote occurs in its source word for word.
3. **Structure** (claim 3). Every show-node has at least one child, a method and a motivation. An `explicit` motivation's quote occurs in the source.
4. **Hypotheses** (claim 4). Every `from` id is a concept in the graph. Every symbol in `adds` occurs in the node's claim or in a descendant's claim or quote.
5. **Modules** (claim 5).
   - Every planned page that is not a pure concept page appears as some node's `page`.
   - A page's node and its ancestors with pages respect the reading order: an ancestor's page comes after its descendants' pages, as the target comes last.

### When the task runs

The plan is a reading of the source (claim 2). It needs the source and the graph, not the pages. So it runs after extraction and extension and before writing, as one task.

The writer of a module page then gets its node:
- its claim, its method and the motivation for it;
- the hypotheses and bindings in force;
- its parent's claim and method, and where it sits among its siblings.

That replaces the writer's general instruction to say what the page is for (exegenius-core/src/write.rs:21-23) with the specific intention, and tells the writer which motivations are `reconstructed` and must be given to the reader.

The outline then takes its parts from the top-level subtrees (Leron's levels). It checks that every part's pages lie in one subtree, or in the glossary.

### Rendering

- **"Where this fits."** Each module page gets a breadcrumb: the path from the root to its node, giving each ancestor's claim and the method that leads down. This is the "uninterrupted chain" to the main objective (Noûs §5.1), shown on the page.
- **The front page** shows the top two levels of the tree, with methods and motivations.
- **Part intros** show their subtree's top.
- **Infer-leaves** are not shown in the tree view. They are the details the top-down reader ignores (claim 2).
- **The chain view** of 2026-10-11-signalled-edges is a root-to-leaf path.
- **Notation.** Plain words ("To show…", "by…", "because…"). Hamami & Morris's ⇒ and ↪ appear in the plan file's documentation, not on the site.

### How the plan relates to `requires`

The two structures are different things:
- **`requires`** is about concepts: what a reader needs before a page makes sense.
- **The plan** is about the argument: what each step is for.

Hypotheses (`from`) are where they meet. A node's hypotheses must be pages earlier in reading order, or known concepts. That is the same rule `requires` enforces, applied to the argument.

## How the simulation tests it

1. **Goals that test the rationality part.** Hamami & Morris's diagnostic questions become a goal kind: "why is X introduced here?" and "which goal does page P serve?". Mejía-Ramos et al. add "Summarizing via high-level ideas" and "Identify the purpose of a module". All of these are in 2026-10-11-comprehension-goals.
2. **Hypothesis H1.** With breadcrumbs and the plan on the front page, readers answer purpose and summary goals with fewer pages opened. They should also cite breadcrumb or intro text less often than page bodies, since the breadcrumb orients and the body explains.
3. **In-context leakage.** A breadcrumb states a page's purpose word for word, so a "which goal does P serve" goal can be answered by copying it.
   - `apply_goals` rejects transfer key points that appear word for word on a page (exegenius-core/src/simulate.rs).
   - The check has to extend to plan claims and motivations, which reach the reader through rendered chrome rather than page bodies.
4. **Localisation.** Stuck points on "why" questions should concentrate on nodes whose motivation is `reconstructed`. If they do, the label is a useful prior for where the wiki needs work.

## Risks

- **Rationalisation.** An agent can write a plausible plan that is not the source's argument. The `source` label and the quotes on `explicit` motivations limit this. Readers' stuck points then test what is left.
- **One tree, many goals.** Lange (2024) argues that what explains depends on which feature of the result is salient (2026-10-11-aha-moments). A single plan tree encodes one reading of the argument. Goals from other angles may need other paths, which a tree can still serve, since any root-to-node path is a sub-argument.
- **Size.** For the 45-page demo the tree has perhaps 15–30 `show` nodes. That is one task, not a batch.

## Open questions for steering

1. **Plan depth.** How deep should the plan task go below the module nodes? Claim 2 says the infer-leaves are details. For the report-order check, leaves are needed only where a module's internal order matters.
2. **One task or several.** One plan task, or the top two levels first and then one task per top-level subtree? Claim 2's top-down order suits splitting, and smaller tasks suit Haiku-sized agents.
3. **Pages from subtrees.** Should module pages come from the plan (one page per chosen node), with concept pages from extraction as now? That changes how pages are chosen.

Settled after steering, from the papers:
- the plan is a top-down reading of the source, written before the pages;
- infer-leaves follow the source's order;
- definitions attach to the node that adds them;
- pages are a choice of modules.

## Current State
- The reading order is a topological sort of the prerequisite closure, with ties broken by the position of each concept's first supporting quote in the sources (exegenius-core/src/plan.rs:1-6)
- The outline task asks for an overview naming the route to the target part by part, in free prose (exegenius-core/src/outline.rs:41-43)
- The write instructions ask each page to open by saying what the concept is for on the way to the target, with the dependent pages listed in the task (exegenius-core/src/write.rs:21-23)
- The outline check rejects a part order in which a page's prerequisite sits in a later part (exegenius-core/src/outline.rs)
- `cite::check` verifies quotes against named sources word for word, and the same check could serve plan motivations (exegenius-core/src/cite.rs:64-80)

## Missing
- No plan tree of show and infer intentions
- No plan task, and no checks on root, report order, structure, hypotheses or modules
- No per-page breadcrumb from the target to the page's place in the argument
- No plan rendering on the front page or in part intros
- No goal kinds about purpose, motivation or modules

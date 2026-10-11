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
- **Modules** (§7). "any subtree of P can be conceived as a module which can be described by stating the proving intention of type 'to show' at its root". They connect this to Leron's structured proofs (1983) and to Lamport's structured proofs.

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

## Design

### The plan file

A new task writes `exegenius/argument.json`. The name avoids `plan`, which already means the reading order (exegenius-core/src/plan.rs). The file is a tree:

```json
{
  "claim": "Two n-bit integers can be multiplied in O(n log n) steps on a multitape Turing machine",
  "intention": "show",
  "pages": ["main-theorem"],
  "method": "reduce to a multidimensional cyclic convolution and bound its cost",
  "motivation": "a cyclic convolution of digit sequences gives the product once there is no wraparound",
  "source": "explicit",
  "children": [ ... ]
}
```

The node fields:
- **`claim`**: one sentence: what this node shows or infers.
- **`intention`**: `show` or `infer`, in Hamami & Morris's sense. Leaves are `infer` nodes.
- **`pages`**: the wiki pages that carry out this node. A page can serve more than one node, and a node can span pages.
- **`method`**: for `show` nodes, how the children divide the work. The connectives of 2026-10-11-signalled-edges (reduce to, compute by, bound, construct, split into cases) are the vocabulary.
- **`motivation`**: the practical reasoning behind the split. Why this method, why these children, why in this order.
- **`source`**: where the motivation comes from. The labels follow the "gap status" field of the explanatory skeleton proposed in the ChatGPT session:
  - `explicit`: the source states it, backed by a `<cite>`-style quote;
  - `recoverable`: the source supports it across several passages;
  - `reconstructed`: the agent supplies it.

Hamami & Morris's footnote 9 says that intentions are usually left for the reader to work out. So `reconstructed` will be common, and the label keeps the agent's reconstruction separate from the source's claim.

### Checks at submit

1. **The root.** The root's pages include the target.
2. **Coverage.** Every planned page appears in at least one node. Minor definition pages may appear as `infer` leaves under the node that uses them.
3. **Order.** For ordered children c₁…cₖ, no page under cᵢ requires a page that appears only under some later cⱼ. The plan's order then respects the prerequisite order.
4. **Fields.** Every `show` node has a method and a motivation.
5. **Quotes.** Every `explicit` motivation carries a quote that occurs in the named source word for word, checked as `<cite>` quotes are (exegenius-core/src/cite.rs:64-80).

### When the task runs

The plan is about concepts and the source, so it can be written once the graph is settled, before the pages are written.
- **The writer** then gets its node as context: "This page serves to show: ⟨parent claim⟩, by ⟨method⟩; it is step i of k; because ⟨motivation⟩." That replaces the writer's opening-sentence instruction (exegenius-core/src/write.rs:21-23) with a specific intention.
- **The outline** can take its parts from the top-level subtrees, which are Hamami & Morris's modules. It can also be checked against them.

The alternative is to write the plan after the pages, as the outline is now. That plan would describe the pages but could not shape them.

### Rendering

- **"Where this fits."** The build adds a breadcrumb to each page: the path from the root to the page's node, giving each ancestor's claim and the method that leads to the next. This is the "uninterrupted chain" to the main objective, made visible. It is also Weber & Mejía-Ramos's zoom-out, one click from every page.
- **The front page and part intros** show the tree to depth two, with each method and motivation.
- **The chain view** of 2026-10-11-signalled-edges, "A ↬via B ↬reduced to C ↬by D", is a root-to-leaf path in this tree.
- **Arrow notation.** Hamami & Morris's ⇒ and ↪ could mark show and infer nodes. Plain words ("To show…", "Infer…") may serve a reader who does not know the notation better. This is an open question below.

### How the plan relates to `requires`

The two structures are different things:
- **`requires`** is about concepts: what a reader needs before a page makes sense.
- **The plan** is about the argument: what each step is for.

Check 3 ties them together only through order. A page can be a prerequisite of many pages and serve one node, like the DFT page. A node can need many prerequisites.

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

1. **Before or after the writes?** Before lets the plan shape the pages. After matches the outline and sees finished pages.
2. **One tree or one per part?** One tree for the target, or one per part with the parts as the top level? Per part makes the outline the plan's skeleton and the tasks smaller.
3. **Pages from subtrees.** Should splitting and merging pages follow plan subtrees, with a module per page? That changes extraction, not just rendering.
4. **Notation.** Hamami & Morris's arrows (⇒, ↪, ↓) or plain words on the rendered pages?

## Current State
- The reading order is a topological sort of the prerequisite closure, with ties broken by the position of each concept's first supporting quote in the sources (exegenius-core/src/plan.rs:1-6)
- The outline task asks for an overview naming the route to the target part by part, in free prose (exegenius-core/src/outline.rs:41-43)
- The write instructions ask each page to open by saying what the concept is for on the way to the target, with the dependent pages listed in the task (exegenius-core/src/write.rs:21-23)
- The outline check rejects a part order in which a page's prerequisite sits in a later part (exegenius-core/src/outline.rs)
- `cite::check` verifies quotes against named sources word for word, and the same check could serve plan motivations (exegenius-core/src/cite.rs:64-80)

## Missing
- No plan tree of show and infer intentions
- No plan task, and no checks on root, coverage, order or motivations
- No per-page breadcrumb from the target to the page's place in the argument
- No plan rendering on the front page or in part intros
- No goal kinds about purpose, motivation or modules

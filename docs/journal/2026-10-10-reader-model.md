# 2026-10-10: Reader model, entry contracts and path planning

What the system takes a reader to be, what a page promises, and what "shortest hop"
means. R1, R2 and R3 are the three design rounds described in 2026-10-10-origin.

## Reader state as a set of concepts

R1 proposed modelling the reader as in Knowledge Space Theory (KST; Doignon & Falmagne,
1985). The reader's state is a set K of concepts. It starts from a declared background,
and each page read adds the concepts that page introduces. R2 kept this and stated the
rule. Entry e, with prerequisites R_e and provided concepts P_e, is *eligible* when
R_e ⊆ K. Reading it *proposes* K' = K ∪ P_e. R2 chose "proposes" deliberately: the set
model is a planning abstraction, and reading a page does not guarantee that anyone learns
from it.

Three parts of KST carry over directly. R1 and R2 did not mention them.

- **Outer fringe.** In KST, the outer fringe of a state is the set of items learnable
  next. Here it is the set of eligible entries. "What can this reader read now?" is
  therefore a fringe computation, and "where does the corpus strand a reader?" asks when
  the fringe is empty before the goal is reached.
- **Learning-space axioms.** Falmagne & Doignon, *Learning Spaces* (2011), require that
  any state can grow to any larger state one item at a time (learning smoothness), and
  that knowing more never makes an item unlearnable (learning consistency). A cycle in
  `requires` breaks smoothness: no reading order lets the reader learn those concepts one
  at a time. That gives cycle detection a reason beyond graph hygiene. It also names the
  fix: merge the cycle into one entry that provides all of its concepts at once, or split
  out the part that does not depend on the rest.
- **Competence-based KST.** This variant (Korossy; Heller & Stefanutti) separates latent
  skills from observable test items, and a skill map records which skills each item
  needs. Here, concepts play the part of skills and goal questions play the part of
  items. Each goal should therefore declare which concepts it tests. A simulated reader
  failing a goal then points to specific concepts, not just to "the page".

ALEKS is the large production system built on KST. It shows the formalism holds up at
scale once there is response data. exegenius has no response data, which is why R2
argued against starting with anything probabilistic.

## Progression, not alternatives

R1 offered three options: set-based (KST), Bayesian Knowledge Tracing per concept, or a
full POMDP. R2 recast them as stages of one progression, each worth adopting only when the
previous stage stops being enough:

| Stage | Represents | When it earns its cost |
|---|---|---|
| Set (KST) | which concepts are assumed known | v1; inspectable, deterministic |
| Probabilistic mastery | how likely each concept is understood | once there are observations to estimate from |
| POMDP | hidden state, noisy observations, navigation choices | when adaptive navigation is itself the research question |

R2 warned that BKT needs learn, guess and slip parameters estimated from learner data.
Without such data, its numbers look precise but mean nothing. v1 uses sets.

## Planner and simulator stay separate

R2 asked that the planner's representation of what the reader knows be kept apart from
the simulator's uncertain estimate of what the reader understands. In practice:

- the planner's K comes only from declarations: background profiles plus `provides`;
- simulator output never writes into K;
- simulator findings reach the planner only as diagnostics that the author acts on.

This stops a noisy LLM judgement from silently changing which pages the planner considers
eligible.

## Concepts are not pages

R1's entry contract lists `requires`, `provides` and `goals` in frontmatter. The point
that settles page granularity is what `requires` and `provides` refer to: **concepts, not
entries**. One entry can provide several concepts. A page on the chirp Z-transform can
provide both `czt` and `chirp-identity`.

This matters because of R3's first failure mode, *atomisation*. If every concept had to be
its own page, the concept graph would set page granularity, and a structural score would
reward fifteen-hop stub chains. Separating concepts from pages leaves granularity as a
writing decision. The concept graph stays fine-grained, and pages can be as large as the
explanation needs.

The v1 rule is that each concept has exactly one *home* entry that provides it, or appears
in a background profile.

## What "shortest hop" means

R1 called the shortest learning path "graph search, no LLM needed". With one home per
concept, the problem is simpler than a search:

- The pages needed to reach goal G from background B form the prerequisite closure of
  the concepts G tests, minus whatever B already covers. This set is fully determined and
  takes polynomial time to compute.
- The hop count is the size of that closure. It is not a graph distance, because a reader
  has to read every prerequisite page, not just the pages along one chain.
- The reading order is any topological order of the closure. Choosing among the valid
  orders is where pedagogy comes in. One tie-break: finish one branch before starting the
  next, so that no concept sits introduced-but-unused for long. This is a heuristic, and
  belongs in the ideas bank, not in the core.

If a concept can have several homes, say an intuition page and a formal page that both
provide `dft`, planning becomes an AND/OR graph problem. Pages are AND-nodes, since every
prerequisite must be met, and concepts are OR-nodes, since any one provider will do. The
cheapest solution where a shared prerequisite is paid for once is NP-hard in general; it
contains Steiner-tree-like problems. The variant that pays for shared prerequisites once
per use is solvable in polynomial time with Knuth's generalisation of Dijkstra (1977), but
it overcounts reading cost. v1 avoids the whole question with the one-home rule. Multiple
homes stay in the ideas bank until the corpus needs them.

Hops are not the only cost. A long page costs more to read than a short one. R1 proposed
"answerable goals per hop" as the empirical signal. Page length in tokens is a cheap
second cost to report alongside hop count.

## One reader or many

The original practice had one reader, the author. Two findings from educational
psychology say the best explanation depends on who reads it:

- the *expertise reversal effect* (Kalyuga, Ayres, Chandler & Sweller, 2003): guidance
  that helps novices hinders experts;
- *reverse cohesion* (McNamara, Kintsch, Songer & Kintsch, 1996): high-knowledge readers
  learned more from a less cohesive text, because filling the gaps made them work.

So "minimise confusion" is not one objective. It is one objective per reader profile. In
v1, profiles are named background sets, and planning always takes one of them. A corpus
written for one reader declares one profile, which keeps the original practice as the
default case.

## Typed relations and proof-likeness

R1 pointed to Patrick Massot's `leanblueprint`. Each node there has a statement and
explicit `\uses{}` dependencies, and the rendered graph shows each node's status. R1 also
pointed to the Stacks Project and Kerodon. Both use permanent four-character tags that are
never reused, so references survive renames and reorganisation.

R2 separated three kinds of relationship that a single `requires` would blur together:

- **Conceptual prerequisite (`requires`).** The reader needs A before B makes sense.
  This is the only relation that controls eligibility.
- **Logical dependency (`proves`, `uses`).** B's result rests on A's result. This makes
  a statement about the mathematics, not about the reader.
- **Pedagogical relation (`motivates`, `illustrates`, `formalizes`).** A helps with B but
  does not gate it.

R1's fourth thread asked whether an intuition node may depend on a formal node that comes
later. R2 answered yes, as a forward reference, which is not a prerequisite. An intuition
page that previews a theorem links forward through `formalizes` or a preview, and the
planner does not treat that target as something the reader already knows. R3 extended
this to intentional rule-breaks with obligations; see 2026-10-10-rules-and-grounding.

R2 also noted that different entry kinds are checked differently. A definition does not
prove the claims that use it, and an intuition is not a weaker theorem. An entry's `kind`
(definition, result, intuition, example, procedure) therefore decides which checks apply
to it.

## Open questions for steering

1. **Entry IDs and concept IDs.** Should an entry's `id` also be its primary concept, with
   `provides` listing any further concepts? Or should concept IDs live in a namespace of
   their own? Recommendation: the entry id doubles as its primary concept. This keeps
   simple pages simple, and multi-concept pages are the exception.
2. **One home per concept in v1.** Recommendation: yes. Multiple homes (intuition page and
   formal page for the same concept) move to later, behind AND/OR planning.
3. **Which typed relations v1 parses.** Recommendation: `requires` and `provides` drive
   the planner. `motivates`, `illustrates`, `formalizes` and `proves` are parsed, checked
   only for resolving to known IDs, and shown in the rendered wiki.
4. **ID scheme.** The choice is between human-readable slugs (`chirp-identity`) and
   Stacks-style opaque tags (`0A3F`) mapped to titles. Slugs are easier to type in
   `[[links]]`. Opaque tags survive a rename without an alias table. Recommendation: slugs,
   plus an `aliases` list that keeps old slugs resolvable after a rename.
5. **Goal concept declarations.** Should every goal have to list the concepts it tests
   (the competence-KST skill map), or should that be optional in v1?

## Current State
- No source file in the repository implements the reader model, the entry contract, or
  path planning — the design appears only in docs/journal/2026-10-10-reader-model.md

## Missing
- No frontmatter parser for `id`, `requires`, `provides`, `goals`, or typed relations
- No concept registry mapping concept IDs and aliases to home entries
- No background profile format or loader
- No prerequisite-closure or topological-order computation for a goal and a profile
- No outer-fringe computation for a reader state

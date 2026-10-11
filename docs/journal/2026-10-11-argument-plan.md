# 2026-10-11: The argument plan

This entry implements the third research entry, 2026-10-11-proof-plans (proposal I-85). The research entry derived the design from six claims in Hamami & Morris's account of proof plans; this entry records what was built from that design, where the build departs from it, and a check on the demo.

## What was built

### The plan file and its task

A new task, `argument`, reconstructs the plan of the sources' argument as a tree and stores it in `exegenius/argument.json` (exegenius-core/src/argument.rs). It runs after extraction and extension and before any page is written, because the plan is a reading of the sources (claim 2), not of the pages. It runs again whenever the planned pages change, as after an extend task adds pages, and the task then includes the previous plan to revise.

A step (`Node`) has:
- an `id` and a `claim`;
- an `intention`, `show` or `infer`;
- for a show step: a `method`, a `motivation` and `children`, the sub-steps in the order the method takes them;
- for an infer step: a `quote` from the sources and no children, with `deferred` marking a step the sources report out of order;
- `from`: the concepts the step relies on, which are its hypotheses (claim 4);
- `adds`: the symbols it introduces for its sub-steps, as bindings with display names (2026-10-11-symbol-bindings);
- `page`: the page that presents the step as a module (claim 5).

The motivation is a typed record, not prose in the step: its `text`, its `source` (`explicit`, `recoverable` or `reconstructed`) and, for an explicit motivation, a quote from the sources. A review of the hub entry (ChatGPT, 2026-10-11) warned that otherwise "an agent could produce a very plausible plan that quietly supplies reasoning the source never establishes". With the provenance typed, an explicit motivation must quote the sources, and the writer is told which motivations the sources leave unstated.

### Checks at submit

Each check comes from one of the claims in the research entry:
1. **The root** is a show step whose page is the target page.
2. **Report order** (claim 1). Along a depth-first walk, the quotes of infer steps not marked `deferred` start at non-decreasing positions in the sources, and every quote is in its source word for word. The rejection names the two steps out of order and offers both remedies: reorder, or mark the step deferred.
3. **Structure** (claim 3). A show step has sub-steps, a method and a motivation; an infer step has a quote and no sub-steps; an explicit motivation has a quote found in the sources.
4. **Hypotheses** (claim 4). Every `from` id is a known concept or a page before the step's page in reading order. A step's page is its own or its nearest ancestor's. Every symbol in `adds` occurs in the mathematics of a claim at or below the step.
5. **Modules** (claim 5). Every `page` is a planned page, no page presents two steps, and every planned page either presents a step or is in some step's `from`.

Step ids must be unique and claims non-empty.

### What writers are told

A writer whose page presents a step gets a section "This page in the argument": the step's claim, its method, its motivation with what the sources say about it, its sub-steps in order with their pages, and the steps it serves from the main result down. For each motivation:
- if explicit, the writer sees the quote;
- if recoverable, the writer is told the sources leave it to the reader to work out, so spell it out;
- if reconstructed, the writer is told the sources do not say it, so give the reader the reason.

The write instructions say that a page presenting a step explains how its method shows its claim, and gives readers every motivation the sources do not state. A page that presents no step gets the steps that rely on its concept instead.

### Rendering

- **"Where this fits"** is a note on each page that presents a step below the root. It lists the steps from the main result down to the page's own, each with its method and a link to the page that presents it: the "uninterrupted chain" to the main objective (Noûs §5.1) shown on the page. A page that presents no step lists the steps that rely on it.
- **The front page** has a section "The argument": the main claim, its method and motivation, and its sub-steps with their methods and pages. It comes after the reading tip and before the contents.
- **Simulated readers** see both, so the simulation measures them.
- **Infer steps** are not shown on the site. They are the details a top-down reader skips (claim 2), and they are there for the checks.

## Where the build departs from the research entry

1. **The page-order check is a warning.** The research entry required a step's page to come before its ancestors' pages in reading order. The reading order comes from prerequisite edges, which the argument task cannot change. So a plan can be right while its pages read in another order. The check therefore warns at submit instead of rejecting, naming the step and both pages.
2. **The outline is not checked against the plan.** The research entry had the outline check that every part's pages lie in one top-level subtree. Prerequisite edges can run between subtrees in both directions, and then no valid outline satisfies that check. Instead, the outline task lists the argument's main steps with their pages, and the instructions ask for parts based on them where the prerequisites allow.
3. **Part intros do not show their subtree's top.** Part intros are written by the outline agent, which now sees the main steps.
4. **"Pure concept pages" are defined by use.** The research entry exempted "pure concept pages" from presenting a step, without saying how to tell them. Here, a page is exempt exactly when some step relies on it in `from`.
5. **The front page's top two levels use labels** ("Main result", "How", "Why") rather than a sentence built from the claims, which would mean changing the case of words that may be names.
6. **Symbol scope by plan node is not built.** The plan records the symbols each step adds, and checks them, but the bindings in force at a page are still those of earlier pages (2026-10-11-symbol-bindings).

## Check against the demo

The demo project's repository does not include its source paper. A public write-up of the same result, the Latent Seminar notes (lowrank.github.io/latent-seminar/OpenAI-Math/109-integer-multiplication, 1,861 words as text), served as the source instead. Each run went to a fresh subagent on a mid-sized model, given only the prompt `next` prints, against a copy of the demo's graph and pages.

**First run: confounded.** The source list also held the file of the graph's evidence quotes that 2026-10-11-symbol-bindings rebuilt for ordering, written in the wiki's reading order. The plan was accepted on the first submit, but every quote in it came from that file, none from the notes. So the report-order check compared quotes against a list already in the pages' order, and it tested little; 5 of the 18 infer steps were marked deferred. The plan had 3 show steps and a depth of 2, with most pages presented by infer steps.

**Second run: the notes as the only source.** Without the quote file, the reading order falls back to the order of concepts in the graph, so it differs from the demo's in places. The answer was rejected once, because a step relied on `rank-saving-recursion`, which came after the step's page in that reading order. The next answer was accepted. The plan:
- 31 steps: 7 show steps and 24 infer steps, to a depth of 3;
- 19 of the 45 pages present a step, and the other 26 are hypotheses of some step;
- 4 infer steps marked as reported out of order;
- motivations: 3 `explicit` with quotes from the notes, 2 `recoverable`, 2 `reconstructed`.

The root's method is "reduce the product to a convolution, speed up both tape operations a transform needs, assemble the transform pipeline, and account for the cost". Its five main steps are the radix-digit reduction, exact recovery by rounding, the two tape procedures with their recursive network, the six-step algorithm, and the cost accounting. Whether those are the right top level for this reader is a question for the simulated readers, not for the checks.

One of the two `reconstructed` motivations is on the multiplication algorithm: "A transform of arbitrary length is awkward on tapes, so it is rewritten with power-of-two axes where shifts are cheap". Under the new write instructions, that page's writer would be told the sources do not give this reason, and to give it to the reader.

No step's page read after an ancestor's page, so the warning did not fire.

Building the docs from this plan showed one flaw in the rendering. Methods are written as imperatives ("reduce the product to …"), and the breadcrumb had joined them with "by", giving "— by reduce the product". The breadcrumb and the writer's section now label methods "how:".

## Current State
- `argument::Node` holds a step's id, intention, claim, hypotheses, method, typed motivation (text, provenance, quote), added bindings, page, quote, `deferred` flag and sub-steps; `Argument` stores the root with the planned pages it was made for in `exegenius/argument.json` (exegenius-core/src/argument.rs:73, exegenius-core/src/argument.rs:117)
- `next` issues the argument task after extract and extend and before writes whenever no stored argument covers the planned pages, with the previous argument in the task (exegenius-core/src/task.rs, exegenius-core/src/argument.rs:275)
- `argument::check` rejects a root that is not a show step on the target page, quotes not in their source, infer steps out of depth-first source order unless deferred, show steps without sub-steps, method or motivation, explicit motivations without a quote, infer steps with sub-steps or without a quote, unknown or later hypotheses, `adds` symbols absent from the subtree's claims, unknown or doubled pages and unused pages, and warns when a step's page reads after its ancestor's (exegenius-core/src/argument.rs:357)
- Write tasks give a module page its step, method, motivation with provenance, sub-steps and ancestors, and give other pages the steps that rely on them (exegenius-core/src/write.rs:110)
- `build` writes a "Where this fits" note on pages from `argument::trail` and an "The argument" section on the front page from `argument::overview` (exegenius-core/src/build.rs:404, exegenius-core/src/build.rs:767, exegenius-core/src/argument.rs:164, exegenius-core/src/argument.rs:199)
- The simulation lens shows the same note and section (exegenius-core/src/simulate.rs)
- The outline task lists the argument's main steps with their pages (exegenius-core/src/outline.rs)

## Missing
- No rejection when a step's page reads after its ancestor's page; a warning only
- No check of outline parts against the argument's subtrees
- No scoping of symbol bindings by plan node
- No chain view along a root-to-leaf path with relation predicates
- No per-step measure of reader understanding in the report

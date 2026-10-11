# 2026-10-11: Signalled edges

This is the second of the research entries listed in 2026-10-11-explanatory-structure-research. It asks what a link between two pages has to carry for a reader to follow a chain across pages rather than a series of single hops. It covers proposal I-86. Nothing here is implemented.

## The problem

After reading the demo wiki, the author wrote down a chain of four pages:

```
Exact integer multiplication
↬via Radix-digit convolution
↬reduced to Cyclic convolution
↬by Discrete Fourier transform
```

- **The wiki stores each hop as a bare prerequisite.** A concept's `requires` is a list of ids (exegenius-core/src/graph.rs:30).
- **The rendered page says only "This page builds on A, B"** (exegenius-core/src/build.rs:357-373).
- **The Connections diagram's edges carry no labels** (exegenius-core/src/build.rs:288, :379-390).

The connectives (via, reduced to, by) say what each hop does. The author supplied them afterwards, by asking which prepositions fit. A reader who has to supply them is doing the work the author's "years happen in days" complaint describes: reconstructing an inference the text compressed away.

## History in this journal

Typed relations have been proposed before:
- **2026-10-10-reader-model** recorded R2's three kinds of relation:
  - conceptual prerequisite (`requires`);
  - logical dependency (`proves`, `uses`);
  - pedagogical relation (`motivates`, `illustrates`, `formalizes`).
- **I-09** planned to parse the extra relations, check them and render them, without letting them gate the reading order.
- **2026-10-10-revised-v1-design** superseded I-09 when entry frontmatter was dropped for the graph sidecar. Only `requires` survived.

This proposal differs from I-09 in three ways:
- **No new edge kinds.** It annotates each existing `requires` edge with what the prerequisite contributes to the page.
- **No change to planning.** The annotation never affects eligibility or reading order.
- **A checked anchor.** Each annotation is tied to a sentence on the page, in the way `<cite>` quotes are tied to sources.

## What the literature gives

### eRST: a relation is licensed by a signal

eRST (Zeldes et al. 2025) extends Rhetorical Structure Theory. An analysis has three parts (§3):
1. a primary tree;
2. secondary edges, which may be cyclic and non-projective;
3. signals: tokens anchored to one edge.

**Signals.**
- **The licensing rule.** A secondary edge "may only be added if it is supported by a sufficient signal" (§3.2): for example an orphan discourse marker, a "but" that no relation in the tree accounts for.
- **Signal types.** Discourse markers are one type. There are seven others: graphical, lexical, morphological, numerical, reference, semantic and syntactic.
  - The lexical type includes "indicative word/phrase" and "alternate expression", which corresponds to PDTB's AltLex.
  - The semantic type includes lexical chains and repetition or synonymy.
- **Signals as rationale.** A full parse has "a built-in rationalization mechanism in the form of signals, which can be used for filtering (only use explicitly-signaled relations, or just ones signaled by a DM)" (p. 37). PDTB is the Penn Discourse Treebank; a DM is a discourse marker.

**The relation inventory** in the GUM corpus has 32 labels, among them:
- EXPLANATION-EVIDENCE ("S provides evidence which increases R's belief in N");
- EXPLANATION-JUSTIFY;
- EXPLANATION-MOTIVATION;
- CONTEXT-BACKGROUND ("S provides information to increase R's understanding of N");
- MODE-MEANS ("S indicates the means by which N happens");
- CAUSAL-CAUSE and CAUSAL-RESULT;
- ELABORATION-ADDITIONAL;
- RESTATEMENT-PARTIAL.

GUM and its companion corpus GENTLE include mathematical proofs among their genres.

**The numbers that bear on the reading report** (Table 7, Table 10):
- **Explanation relations are the least often signalled.** EXPLANATION relations are signalled in 41.94% of cases, the lowest share of any class. Their most frequent markers are "so", "because" and "as".
- **"By" signals means.** MODE relations are signalled in 78.71% of cases. Their most frequent marker is "by", and their most frequent lexical signals are "using" and "based on".
- **Restatement uses fixed phrases.** RESTATEMENT is signalled by phrases such as "that is", "i.e." and "known as".
- **A single tree loses causal and justifying links.** Secondary edges are 3.37% of relations. Yet the primary tree alone misses 14.1% of CAUSAL-RESULT relations, 11.7% of EXPLANATION-JUSTIFY and 10.3% of ADVERSATIVE-CONCESSION.

The author's connectives are signals in this sense:
- **"by"** is the most frequent marker of MODE.
- **"via" and "reduced to"** are lexical, alternate-expression signals.

The finding that bears most directly on the reading report is the first one: the relation most often left for the reader to infer is the explanatory one.

**Agreement between annotators** on secondary edges is low. After the guideline revision it is .529 on span, .49 on nuclearity and .412 on relation. Even trained annotators disagree on fine relation labels for edges outside the tree.

### Explanatory relations as a plan

Liu, Yu, Demberg & Lapata (2025) plan lay summaries from four RST relations: Background, Elaboration, Explanation and Comparison.
- **Control.** Deleting the plan questions of one relation removes that kind of explanation from the output. Elaboration and Explanation matter most.
- **A lexicon baseline.** Their rule-based baseline finds explanatory sentences with marker lists such as "because, due to, since ... as a result ... thereby, whereby" and "in other words, that is to say, i.e., for example" (Appendix C).

### A hop is a method

Hamami & Morris (Review of Symbolic Logic 2021) call a mathematical method "a rule that transforms a proving intention into one or several other proving intentions arranged in a subplan" (§5.1).
- "Reduced to" names a reduction method: to compute A, compute B.
- "By" names the means by which a step is carried out.
- Both say why the prerequisite is there, which is the rationality part of understanding in their Noûs paper. The tracking part is the tree itself.

### Link labels carry scent

In information foraging theory (Pirolli 2005, 2007), a link's text is the proximal cue a reader uses to judge where it leads (I-55). A label that says what the destination contributes ("by the DFT") carries more scent than its title alone.

## Design

### The edge record

Each concept gains a map from prerequisite id to a role, stored in `exegenius/graph.json` with `#[serde(default)]` so existing graphs load unchanged. `requires` itself does not change.

```json
"roles": {
  "cyclic-convolution": {
    "relation": "reduction",
    "connective": "reduced to",
    "quote": "the product of the digit sequences is reduced to a cyclic convolution of length S"
  },
  "discrete-fourier-transform": {
    "relation": "means",
    "connective": "by",
    "quote": "computed by a discrete Fourier transform, a pointwise product and an inverse transform"
  }
}
```

The quotes above are illustrations, not demo text.

The relation set is small and coarse, taken from the GUM inventory with one mathematical addition. Each relation says what the prerequisite does for this page:

| Relation | The prerequisite... | eRST counterpart |
|---|---|---|
| `term` | supplies a term or object this page uses | CONTEXT-BACKGROUND |
| `means` | is how a step on this page is carried out | MODE-MEANS |
| `reduction` | is the problem this page's problem is reduced to | none; a mathematical method in Hamami & Morris's sense |
| `justification` | is why a claim on this page holds | EXPLANATION-JUSTIFY, EXPLANATION-EVIDENCE |
| `generalisation` | is the general case this page specialises, or the reverse | ELABORATION |
| `restatement` | says the same thing in another form | RESTATEMENT |

The connective is free text of one to four words, in the writer's own words, and carries the nuance the coarse relation loses. The low agreement on fine labels in eRST is the reason for keeping the relation coarse and letting the connective carry the detail.

### Who writes it, and the check

The writer states roles in the JSON write answer that 2026-10-11-symbol-anchoring introduces, one per prerequisite page. At submit:

1. **Every prerequisite page has a role.** `apply_write` already warns when a prerequisite is not linked (exegenius-core/src/write.rs:193-200). The warning becomes part of this check.
2. **The quote occurs in the page body**, word for word after whitespace normalisation, with `<cite>` marks stripped.
3. **The quote is the signal.**
   - It links the prerequisite page, or names it.
   - It contains the connective.
   - Both are deterministic string checks on the quoted sentence.

Check 3 is eRST's licensing rule applied to the wiki: an edge is accepted only with a textual signal. A writer who cannot produce a quote has written a page that relies on a prerequisite without saying what for, and that is where a hop goes unexplained. The writer then adds the connecting sentence.

The check verifies that the role is anchored, not that the relation label is right. A wrong label with a right quote shows the reader the right sentence anyway.

Edges to concepts the reader already knows (`known: true`) need no role. Those concepts get no page, and the "Assumed known" line covers them.

### Rendering

- **"Before this page"** lists each prerequisite with its connective. In the demo's terms: "This page builds on [Cyclic convolution] (reduced to) and [Discrete Fourier transform] (by)." The quote goes in the link's title attribute, which the `content.tooltips` feature already shows on hover.
- **The Connections diagram** labels each edge with its connective. Mermaid supports edge labels (`A -- by --> B`), and the diagram already emits mermaid (exegenius-core/src/build.rs:288).
- **A chain view.** The chain view for the front page and the part intros, "A ↬via B ↬reduced to C ↬by D", needs a choice of path. That choice belongs to the plan tree (2026-10-11-proof-plans). With roles alone, the path can be the one through the transitive reduction that follows the outline's parts, using the edges `part_edges` already computes (exegenius-core/src/build.rs).

### A writer-side diagnostic

Two counts go to the report:
- prerequisite edges whose first write attempt had no signal quote;
- edges whose relation is `term` alone, the weakest relation.

Together they map where the wiki relies on a page without saying why. That map comes from the writers, before any reader runs.

## How the simulation tests it

1. **Hypothesis H1.** With connectives rendered, readers answer "how does X relate to Y" goals with fewer pages opened.
   - Mejía-Ramos et al. have a matching template, "Identify the logical relation between modules".
   - It becomes a goal kind in 2026-10-11-comprehension-goals.
2. **Hypothesis H2.** Stuck points whose note asks why a page is needed, or what it contributes, fall.
3. **Scent.**
   - **Measure:** reader trails record the order of pages opened (`exegenius/trails/`).
   - **Prediction:** with connectives in "Before this page", fewer opened pages are abandoned at once without a citation from them.
4. **No leakage.** Connectives are short and generic ("by", "reduced to"). The nonce lens renames the linked page names, so the connectives do not leak the renamed concepts.

## Risks

- **Taxonomy disputes.** A six-way relation set invites argument over labels. The check does not look at labels, and the connective is what readers see.
- **Formulaic sentences.** A writer could satisfy check 3 with a template sentence ("This page uses X by X"). Counting duplicate quote shapes across pages would show it. The reader tests show whether such sentences help.
- **Agent cost.** One extra structured field per prerequisite, which is small next to the page itself.

## Open questions for steering

1. **The relation field.** Keep a coarse relation alongside the free connective, or only the connective, with any typing derived later from a lexicon?
2. **Who writes roles.** The writer at submit, or a separate pass after all pages are written, which would see both pages of an edge?
3. **The signal check.** Must the quote contain the link itself, or is naming the prerequisite enough? Autolinking makes the link appear at build time for earlier pages anyway (exegenius-core/src/autolink.rs).
4. **Optional secondary edges.** Should eRST's secondary edges have an analogue? That would be cross-links that are not prerequisites but carry CAUSAL-RESULT or JUSTIFY relations, which the single tree loses in Table 10. In the demo, `gaussian-dyadic` explains why `simultaneous-butterfly-layer` stays exact. That link runs against the prerequisite direction, and a signalled cross-link would record it.

## Current State
- Concept prerequisites are an untyped list of concept ids (exegenius-core/src/graph.rs:30)
- `render_page` writes "This page builds on A, B" in the "Before this page" admonition, with no connective or relation (exegenius-core/src/build.rs:357-373)
- The Connections diagram draws edges as index pairs with no labels (exegenius-core/src/build.rs:288, exegenius-core/src/build.rs:379-390)
- `apply_write` warns when a prerequisite page is not linked in the body, and does not ask what the prerequisite contributes (exegenius-core/src/write.rs:193-200)
- Typed relations `motivates`, `illustrates`, `formalizes` and `proves` (I-09) are recorded as superseded in docs/journal/2026-10-10-revised-v1-design.md

## Missing
- No role (relation, connective, anchor quote) on prerequisite edges
- No check that a page states what each prerequisite contributes
- No connectives in "Before this page" or on diagram edges
- No chain view on the front page or part intros
- No report of unsignalled prerequisite edges
- No cross-links outside the prerequisite relation

# 2026-10-11: Fragmentation and collapse

This is the sixth of the research entries listed in 2026-10-11-explanatory-structure-research. It takes up the tension the author named in the reading report. Making the cross-page structure visible must not cost the pages their self-containment. It gives no proposal ID of its own. It decides where the outputs of the earlier entries appear, and how to tell whether they help or crowd the page. Nothing here is implemented.

## The problem

The author described two failures in opposite directions:
- **Fragmentation.** The pages were good because they were self-contained. The argument they jointly carry had to be rebuilt by the reader, and some sentences compressed "years" into "days".
- **Collapse.** The author warned against "a loss of coherence from everything coalescing". If every relationship were made explicit on every page, the pages would turn into one over-integrated exposition. On symbols: "too much of this would make for a very overly verbose and thus worse exposition but not enough and the reader feels unmoored".

The author's preferred target was not the pages themselves: awareness of the centrepieces could "if not alter the articles themselves - alter the summarisation articles".

Each earlier entry adds material that could land on the page:
- symbol meanings (2026-10-11-symbol-anchoring);
- connectives (2026-10-11-signalled-edges);
- the plan (2026-10-11-proof-plans);
- pivotal sentences (2026-10-11-aha-moments).

The repair loop also adds text: each round's rewrite notes ask writers to explain more. The demo's round 2 opened fewer pages per run, 4.5 against 7.9. Page lengths per round were not recorded, so it is unknown whether the repairs did this by inlining material, which is the first step towards collapse.

## What the literature gives

### The right amount of explicitness depends on the reader

- **Coherence and prior knowledge** (McNamara, Kintsch, Songer & Kintsch 1996).
  - "readers who know little about the domain of the text benefit from a coherent text, whereas high-knowledge readers benefit from a minimally coherent text".
  - A less explicit text "forces the knowledgeable readers to engage in compensatory processing to infer unstated relations in the text".
  - The benefit showed at the level of the situation model (inference, problem-solving, sorting), not the text base (recall).
- **Partial replication** (McNamara & Kintsch 1996). The follow-up replicated the open-ended-question result but not the sorting result.
- **Expertise reversal** (Kalyuga, Ayres, Chandler & Sweller 2003). Techniques that help inexperienced learners "can lose their effectiveness and even have negative consequences when used with more experienced learners".

The journal already cites these for per-profile optimisation (I-15; 2026-10-10-reader-model).

### Explicitness tracks audience in practice

Liu et al. (2025) count explanatory sentences in scientific summaries:
- about 19.02% and 18.19% of sentences in lay summaries (eLife, PLOS);
- 6.28% and 5.06% in expert-written summaries.

Writers for lay readers explain about three times as often.

### Signalling and the coherence principle

- **Signalling** (Schneider, Beege, Nebel & Rey 2018). Their meta-analysis reports positive average effects of signalling on retention and transfer. The figures here come from a secondary summary: g = 0.53 and g = 0.33. The same summary reports the effect as larger in texts than in pictures, and absent in mathematics.
- **Multimedia principles** (Cromley & Chen 2025). Their meta-analysis of Mayer's research finds the largest effects for "removing seductive detail" and for "sentence-level coherence", among others. The effects are declining over time.

Both say that adding material carries a cost, and that cues need not help in mathematics.

### Definitions on demand, and structure one step away

- **ScholarPhi** (Head et al. 2021). Definitions in tooltips cut the area of the paper readers viewed by 25% and time per question by 45.4 s, with no change in correctness. The information was available without being inline.
- **Elhadad (2006).** Definitions are given as links because inline insertion "is impractical".
- **Zooming** (Weber & Mejía-Ramos 2011). Mathematicians zoom in on details and, the authors conjecture, zoom out to structure. Neither needs the other on the same line.
- **Top-down presentation** (Hamami & Morris, Noûs 2024). Proofs that make their organisation salient ease reconstruction of the plan. Robinson's criticism quoted there is of proofs that are "merely dumps of the details".

### Views

The ChatGPT session proposed generating several views of the same material instead of one summary:
- **dependency**: what must be understood before what;
- **mechanism**: how the system works;
- **significance**: why the details matter to the result;
- **gap**: which connections are implicit, uncertain or unsupported.

The idea matches Lange's point that different questions make different features salient (2026-10-11-aha-moments): no single summary serves every question.

## The layering

The organising rule is that page bodies stay as long as their own concept needs. Cross-page structure goes into derived layers that are deterministic once their inputs exist, and the reader opens them on demand.

| Layer | What goes there | Source entry | Cost to the body |
|---|---|---|---|
| 1. In the sentence | One signalled sentence per prerequisite, saying what it contributes | signalled-edges | one sentence per hop, which the page needs anyway |
| 2. Page chrome, folded | "Before this page" with connectives; "Where this fits" breadcrumb; inherited symbols | signalled-edges, proof-plans, symbol-anchoring | none; collapsed by default where the theme allows |
| 3. Tooltips | Symbol meanings; prerequisite quotes on link hover | symbol-anchoring, signalled-edges | none |
| 4. Separate pages | Dependency view (part diagrams, reading order: exists); mechanism view (the plan tree); significance view (confirmed pivotal sentences in plan order, quoted verbatim) | proof-plans, aha-moments | none |
| 5. Author-side report | Gap view: unsignalled edges, `reconstructed` motivations, unbound symbols, stuck points by level | all | none; stays in `exegenius/report.md`, out of `docs/` |

**The significance view** is the "summarisation article" the author asked for. It adds no prose: it quotes the confirmed sentences and links them in plan order. So it cannot introduce claims the pages do not make, and it cannot make the pages coalesce.

**The gap view** stays out of the published wiki for the same reason the goals and the report do (2026-10-10-simulated-readers): it is a working document for the author, not reading material.

## How to tell fragmentation from collapse

Both failures can be measured from data the simulation already records, or could record cheaply.

**Signs of fragmentation:**
- pages opened per run, from the report;
- runs that open a page and leave without citing it;
- stuck points of level `chain` (2026-10-11-comprehension-goals);
- goals that need several pages, where readers cite only one.

**Signs of collapse:**
- body word count per page per round, which is not recorded today;
- the same explanation repeated across pages, using Altemeyer et al.'s redundancy count (unique statements over total) applied to the pages a goal needs;
- stuck points saying a page is long or digressive.

**The expertise reversal test** uses the reader profiles that already exist (`[[readers]]`):
- A second profile with more background reads the same wiki.
- If the layers help the novice and slow the expert (more pages opened, more budget spent, lower coverage), the wiki has gone too far for the expert.
- With one wiki per project, the explicitness is set for the target reader. The test shows the cost to others. Multi-profile wikis remain a later idea (I-15).

## Open questions for steering

1. **Which chrome folds by default.** The breadcrumb is the zoom-out and may belong open. The inherited symbols and the full "Before this page" may belong folded.
2. **The significance view.** A public page, or kept in the report until readers confirm the sentences?
3. **Word-count tracking.** Record body word counts per round in the report, so that collapse shows before it is felt?
4. **A length budget.** Should page rewrites in repair rounds carry a length budget (for example, at most 25% longer per round) to force repairs to restructure rather than append?

## Current State
- `render_page` adds a "Before this page" admonition, a Connections diagram, a "Used by" list and folded source quotes around each page body (exegenius-core/src/build.rs:357-410)
- Part landing pages carry an intro and a diagram of the part's pages, and the front page carries the outline's overview (exegenius-core/src/build.rs:414, exegenius-core/src/build.rs:556)
- Link titles show as tooltips through the theme feature `content.tooltips` (exegenius-core/src/build.rs:121-123)
- Extra reader profiles can be configured under `[[readers]]` and each runs every goal (exegenius-core/src/config.rs:100)
- The report records pages opened and searches per run, and stuck points per page, and does not record page word counts per round (exegenius-core/src/report.rs)
- Repair rounds queue rewrite notes with no limit on page length (exegenius-core/src/grade.rs:159)

## Missing
- No breadcrumb, mechanism view or significance view
- No folding rule for page chrome beyond the folded source quotes
- No word-count or redundancy measures across rounds
- No expertise-reversal comparison in the report
- No length budget on rewrites

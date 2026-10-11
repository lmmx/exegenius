# 2026-10-11: Relations between pages

This entry implements the second research entry, 2026-10-11-signalled-edges (proposal I-86). It records what was built, where the build departs from the research entry, and a check on three of the demo's pages.

## What was built

A prerequisite edge says that one page needs another. It does not say what for, and the author's chain through the demo ("via … reduced to … by") was made of exactly those missing words. A relation supplies them: a natural-language predicate that joins the two concepts into a sentence, "Radix-digit convolution *is reduced to* Cyclic convolution" (exegenius-core/src/relations.rs). There is no list of relation types, as settled after steering.

### The writer gives one relation per prerequisite page

The write answer's JSON gains a `relations` list. Each relation has the prerequisite's id, a predicate of one to six words, and the sentence on the page that states it. Submit rejects the answer unless:
- every prerequisite page has exactly one relation, and no relation names a page that is not a prerequisite;
- the predicate has at least one word besides articles and forms of "be" and "have", so "is" alone is rejected;
- the sentence is on the page;
- the sentence links the prerequisite, or names it by its name or an alias. A link counts if the sentence covers any part of it, since writers quote sentences that cut through link markup;
- the sentence contains every word of the predicate besides those auxiliaries.

The last three checks are eRST's licensing rule applied to the wiki: a relation is accepted only with a textual signal. They anchor the relation in the page's text. Whether the predicate is apt is left to readers. Known concepts get no relation, since they have no page.

The writer instructions say the page shows each relation as "<this page's concept> *<predicate>* <prerequisite>", so the predicate must make that a true sentence with the page's concept as subject. The write task prints that template under each prerequisite. Predicates are built from the sentence's words, and other forms of a word count ("reduces" for "reduced"), so a writer can turn "the transforms are reduced by X" into "reduces its transforms by". Example predicates ("is reduced to", "computes its transforms by", "is based on", "follows from", "bounds the cost of") are examples, not a list, and the instructions say that "uses" says little.

Link errors used to be reported only after every other check had passed. They are now reported together with the citation, symbol and relation errors, so a writer sees every problem with an answer at once.

### Rendering

- **"Before this page"** reads as triples: "Radix-digit convolution *is reduced to* [Cyclic convolution] and *is computed by* [Discrete Fourier transform]." The link's tooltip is the sentence that states the relation, in place of the gloss. Prerequisites without a relation, as on pages written before relations, follow as "It also builds on …".
- **"Used by"** lists each dependent under its predicate: "[Radix-digit convolution] *is reduced to* Cyclic convolution".
- **Simulated readers** see the same sentence at the top of each page, so the simulation measures it.
- **The report** (`exegenius/report.md`) counts the predicates by frequency. Vague predicates at the top show where pages rely on prerequisites without saying how.
- **`status`** reports how many prerequisite edges between pages have a relation.

## Where the build departs from the research entry

1. **Relations are stored per page, not on the concept.** The research entry put a `relations` map on each concept. A concept is also the shape of an extraction answer, so the field would have appeared in the extract schema, where nothing can fill it. The relations sit in `graph.relations`, keyed by page, beside the symbol bindings. They are replaced whenever the page is rewritten.
2. **Every non-auxiliary word of the predicate must be in the sentence,** up to inflection, not only a "main word". That needs no choice of main word, and it enforces the instruction to build the predicate from the sentence's own words.
3. **Naming the prerequisite is enough; a link is not required.** This answers open question 2 the permissive way: the build links the first unlinked mention of an earlier page anyway (exegenius-core/src/autolink.rs), and the existing warning for an unlinked prerequisite stays.
4. **The Connections diagram has no edge labels.** Every diagram in the wiki points its arrows from a page to the pages that build on it. A predicate reads the other way, from the dependent to the prerequisite, so a label on the arrow would read backwards ("Cyclic convolution *is reduced to* Radix-digit convolution"). The "Used by" list under the diagram carries the predicates instead.
5. **The chain view is not built.** It needs a choice of path through the graph, which belongs to the argument plan (2026-10-11-proof-plans).
6. **The first-attempt diagnostic is not built.** Counting edges whose first write attempt had no signal would mean recording rejected answers, which exegenius does not keep.

## Check against the demo

On a second copy of the demo project, prepared as in 2026-10-11-symbol-bindings, three pages were queued for rewriting and the write tasks went to fresh subagents on a mid-sized model, each given only the prompt `next` prints. The pages were `synthetic-transform` (one prerequisite page), `radix-digit-convolution` (three) and `multiplication-algorithm` (thirteen). `synthetic-transform` carried the two rewrite notes the demo audit had produced for `r` and `k`; the other two carried a note to keep the content.

All three answers were accepted.
- **Rejections.** The `synthetic-transform` writer was rejected once, for binding `a_i`, which the page writes only as $a_0, \dots, a_{r-1}$. The `radix-digit-convolution` writer was rejected twice: once for binding `a_k` in a sentence after its first use, then for a quote that did not match the page. The `multiplication-algorithm` writer was rejected once, for binding `b` after its first use. None of the rejections concerned relations.
- **`r`.** The `synthetic-transform` rewrite says where $r$ is first used that it is "a power of two giving the number of complex coefficients of a ring element", and binds it as a `count`.
- **Relations on the two smaller pages** were apt and read correctly, for example "Synthetic polynomial transform *is based on* Polynomial ring modulo y^r + 1".

The first `multiplication-algorithm` answer showed a defect. The instructions at that point asked only for a predicate "joining this page's concept to the prerequisite", taken from the sentence's own words. All 13 relations passed the signal check, but rendered as triples, many read wrong:
- 7 of the 13 predicates were plural ("*are reduced to*", "*are computed by*"), because the sentence's subject was not the page's concept: "The transforms are reduced to power-of-two transforms by Gaussian resampling".
- 2 of the 13 reversed the prerequisite's part. That sentence gives Gaussian resampling as the means ("by"), while the rendered triple "Multiplication algorithm *are reduced to* Gaussian resampling" makes it the result.

The check had verified the anchor, which is all it claims to do, and the rendering had assumed a grammar no one had asked writers for. The instructions and the task were then changed as described above, and the page was rewritten again from its original version, by a fresh subagent on the same model. Its one rejected attempt quoted a relation sentence that did not match the page. This time 11 of the 13 predicates read as true triples ("Multiplication algorithm *reduces its transforms by* Gaussian resampling", "*computes its middle transform by* Bluestein chirp identity"). The other two are "follows from", whose sentences have "the exactness of the rounding" and "the exact counterpart" as subjects.

The rewrite also shows the risk the research entry named: formulaic sentences. 9 of the 13 sentences now open "The algorithm …". Readers' stuck points and the predicate counts in the report are where this would show.

## Current State
- `relations::Edge` holds a prerequisite id, a predicate and a quote; `WriteAnswer.relations` lists one per prerequisite page (exegenius-core/src/relations.rs:30, exegenius-core/src/write.rs)
- `relations::check` rejects relations for pages that are not prerequisites, duplicates, predicates with no word besides auxiliaries or over six words, quotes not on the page, quotes that neither link nor name the prerequisite, quotes missing a predicate word, and answers missing a relation for a prerequisite page (exegenius-core/src/relations.rs:128)
- `apply_write` reports link, citation, symbol and relation errors in one rejection and stores accepted relations in `graph.relations` (exegenius-core/src/write.rs)
- `render_page` writes "Before this page" as a sentence of predicates and links whose tooltips are the relation sentences, and lists dependents under their predicates (exegenius-core/src/build.rs, exegenius-core/src/relations.rs:207)
- The simulation lens opens each page with the same sentence (exegenius-core/src/simulate.rs)
- `report` counts predicates by frequency in a "Relations between pages" section, and `status` reports how many prerequisite edges between pages have a relation (exegenius-core/src/report.rs, exegenius-cli/src/main.rs)

## Missing
- No edge labels on the Connections diagram
- No chain view on the front page or part intros
- No count of edges whose first write attempt lacked a signal
- No secondary, non-prerequisite relations (open question 3 of 2026-10-11-signalled-edges)

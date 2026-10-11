# 2026-10-11: Symbol anchoring

This is the first of the research entries listed in 2026-10-11-explanatory-structure-research. It develops the 🚅 idea: trace the significance of symbolic terms, and introduce them early as actors or frames of reference. It covers proposals I-90 and I-91. Nothing here is implemented.

## The problem, from the demo

The author read `y^r` on the deployed v0.1 wiki and took r to be the number of digits. Nothing on the page said otherwise. On that wiki r is the number of complex coefficients in a record, which is also the length of the last transform axis. The number of digits is q.

Reading the pages the author posted from turned up three kinds of failure. Quotes are the LaTeX source as served.

1. **Bound nearby, unbound here.**
   - `synthetic-transform` works "in the polynomial ring modulo \(y^r + 1\)". The page never defines r, and its only other use of it is an example ("Take \(r = 4\) and a coefficient list...").
   - The definition is on the prerequisite page `polynomial-ring-mod-yr-plus-1` ("Fix a power of two \(r\)", "a list of \(r\) complex coefficients"). r's value is on `parameter-choice`, two clicks away.
   - Further cases: `signed-packing` uses r and p; `gaussian-dyadic` uses H₀, u and v.
2. **One letter, several meanings.**
   - q is a radix on `turing-machine-model` and a digit count on `cyclic-convolution`.
   - y is an integer, a polynomial indeterminate and an imaginary part on three different pages.
   - w, z and b each carry two or three meanings.
3. **A reference to a definition that does not exist.** `multiplication-algorithm` says the parameter choice fixes S, but `parameter-choice` defines T.

The author's point was that guessing is a failure even when the guess is right. Here the guess was wrong, and the reader had no way to tell.

The wiki was built before the notation page existed (2026-10-10-glossary-notation-autolink). That page would not have caught any of the three:
- It lists symbols that an agent picks after the pages are final.
- It checks only that each listed symbol appears in its page's mathematics (exegenius-core/src/notation.rs:136).
- It would legitimately list q twice, once per meaning, without either page saying which q it means.

## What the literature gives

### Readers lose symbols in predictable ways

**ScholarPhi's formative study** (Head et al. 2021, §3.1): nine readers thought aloud while reading papers. They met symbols they could not interpret ("Θs," "M," "p," "q," "x," "y," "y1") and handled them in two ways:
- **Inferring the meaning from context**, when the symbol seemed important. This was the author's strategy with r.
- **Deferring**, "with the hope that they might find one later". The cost of deferring builds up: a reader "may reach a point in the text where they lack an understanding of so many important terms that they can no longer understand the text".

Looking a symbol up was not free either: it is a "context switch" out of the reader's "headspace". One reader wrote their own glossary of key symbols in the margins.

**Spratte's unified model of proof comprehension** (2024) puts symbols at its lowest level: "Decoding symbols and single words to grasp their semantic content is a prerequisite to constructing coherence within small text segments" (p. 4). If a symbol is not decoded, nothing above it holds.

**Hamami & Morris** (Review of Symbolic Logic 2021) make the same point about proofs:
- The hypotheses of a step "are only rarely written explicitly ... [they] must usually be inferred from the current context of discourse" (footnote 13).
- Footnote 14 counts definitions among those hypotheses.
- A symbol's meaning is part of that context of discourse, which the reader has to rebuild.

### Undeclared symbols are common in published mathematics

- **Wolska & Grigore (2010)** sampled 500 symbols from 50 arXiv mathematics papers, looking at each symbol's first five occurrences.
  - 67.4% are explicitly declared; 26.7% are not.
  - 58% are declared at their first occurrence. Some are declared only later: "symbols are used before they have been introduced".
  - Undeclared symbols rely on what the authors, after Clark, call communal common ground.
- **Schubotz et al. (2016)** found that "only 73% of the identifiers are explained in the text". That is the ceiling for any method that reads only the text.

The source paper behind the demo is professional mathematics. So the writers inherit symbols from a text that assumes common ground the reader profile does not have.

### Meanings switch, and mostly at a small scale

**Asakura, Miyao & Aizawa (2022)** annotated 12,352 identifier occurrences in 15 arXiv papers.
- An identifier type has 2.09 meanings on average, and up to 14.
- Longer papers are more ambiguous: the correlation between word count and the average number of candidate meanings is 0.87.
- Of 2,378 points where an identifier changes meaning, 89.5% fall within a single section. Meanings "often switch in units finer than sections and sometimes switch back".
- 76.5% of grounding sources come before the occurrence they ground.

**Wolska & Grigore's Study 3** points the other way. Structurally similar symbols sharing a root identifier in the same section (ωᵢ and ωₙ₋₁) denote the same concept in 88.9% of pairs. They compare this to "one sense per discourse".

So a letter changing meaning is normal in mathematical writing. The failure in the demo is that the change goes unmarked, across a link the reader follows.

### A symbol is linked to its meaning in more than one way

SemEval-2022 Task 12 (Symlink; Lai et al. 2022) has two relation types that matter here:
- **DIRECT** links a symbol to its definition.
- **COUNT** links "a description of a concept with a symbol that is the number of instances of the concept".

The demo's r is a COUNT: the number of coefficients in a record. So are q (digits) and T (transform length). A COUNT binding needs both the counted concept and the fact that the symbol counts it. "r: a power of two" binds r to a range, not to what it counts.

### Delivering the meaning where it is needed

ScholarPhi's design follows from the formative study.

- **Position-sensitive definitions (design motivation M1).** A tooltip shows "the definition that appeared most recently before that appearance of the word". At the defining sentence it says "Defined here."
- **Composite symbols (M3).** Definitions of a composite symbol's parts are shown together.
- **Other features:**
  - declutter, which low-lights sentences without the symbol;
  - equation diagrams labelling each defined symbol;
  - a "priming glossary" in order of appearance.
- **Results** (27 participants, against a plain reader): 45.4 s less per question, 25% less of the paper viewed, and ease up 1.70 on a 5-point scale. Correctness did not change.
- **Usage.** Readers used symbol tooltips most (median 10 per reader over 15 minutes). The glossary was used least.
- **A failure.** Participants answered "What does T (upper case) mean?" wrongly "because the definitions list did not show all of the usages". The tool "can mislead if it implies incorrect information".

**Elhadad (2006)**, on technical terms for lay readers:
- Unfamiliarity is decided relative to the reader: a term counts as familiar if it occurs often enough in a corpus the reader is known to understand.
- Only "the first mention of a complex term" is defined.
- Definitions are given "as links", because inserting them inline "is impractical because multiple complex terms can and often do occur within a single sentence".

**Hamami & Morris (Noûs 2024)** do the same thing by hand. McKay's proof introduces r late, as the number of solutions to x^p = 1. Their reconstructed plan carries the binding on the node where it is used: "p|r   where r is the number of solutions to x^p = 1" (preprint p. 20).

### Extracting definitions after the fact does not work for symbols

- **Kang et al. (2020).** Definition detection falls from 73.4 to 50.4 macro F1 between sentence level and document level. It "often fails to detect mathematical symbols". With several symbols in one sentence, "the system only ever detects one of them".
- **ScholarPhi.** On its study paper, extraction reached 24% precision and 6% recall, so the study used hand-curated definitions.

The writer knows what each symbol means when writing the page. Asking the writer to state bindings costs a few lines per page. Recovering them afterwards would be unreliable.

### Symbols as actors

The 🚅 idea asks for more than a definition: an introduction that says what part the symbol will play.

- **Lange (2024, §7).** New entities earn their place by explaining something: they "help to explain results exclusively concerning familiar mathematical entities". The actor version of r is not just "the number of coefficients". It is "the number of coefficients, a power of two, so that multiplying by a power of y is a shift with sign changes".
- **Hamami & Morris's McKay questions** ask the same of an object: "Why introduce this particular set S of p-tuples in G?". An object introduced without its motivation is the deus ex machina of Pólya (1949).
- **The event-indexing model** (Zwaan, Langston & Graesser 1995; abstract only) indexes events along "time, space, protagonist, causality, and intentionality". In a derivation, the protagonist is the object being worked on.
- **Morrow, Greenspan & Bower (1987; abstract only)** found that "accessibility depended on the location of the protagonist in the situation model rather than recency of mention".
  - Carried over to derivations, this suggests that a symbol stays available while the argument is working on the object it belongs to, and fades when the argument moves away, however recently it was mentioned.
  - The work is on narrative. The carry-over to technical text is an untested analogy.

## Design

### Bindings, stated by the writer (I-90)

The write answer becomes JSON. Today it is a markdown body (exegenius-core/src/write.rs:161-168). The JSON holds the body and the symbols the page introduces:

```json
{
  "body": "...markdown...",
  "symbols": [
    {"symbol": "r", "meaning": "number of complex coefficients in a record",
     "relation": "count", "role": "a power of two, so multiplying by a power of y is a shift with sign changes"},
    {"symbol": "y", "meaning": "the polynomial indeterminate", "relation": "direct"}
  ]
}
```

The fields:
- **`symbol`**: the LaTeX as written, matched with spacing removed, as `apply_notation` does now (exegenius-core/src/notation.rs:96-104, :136).
- **`relation`**: `direct`, `count`, or `local` for a variable bound inside one formula, such as a summation index.
- **`role`**: optional. The writer instructions ask for it on symbols that later pages will use.

Frontmatter stays out of the pages (2026-10-10-revised-v1-design): the bindings live in `exegenius/graph.json` beside the page, as evidence quotes do.

### The check

At submit, every identifier in the page's mathematics has to be accounted for in one of four ways:

1. **Bound on this page**, by an entry in `symbols`.
2. **Inherited.** It is bound on a page in this page's prerequisite closure, and no nearer page binds the same letter to something else.
3. **Assumed known.** It is standard notation the reader profile covers. A short per-project list of conventional symbols is kept in `exegenius.toml`, for example `O`, `\log`, `\sum`, `i` as the imaginary unit.
4. **`local`.** It is bound inside the formula, which the writer marks with `local`.

Anything else rejects the submission with the symbol and the formula it sits in. That catches failure 1 when the binding page is not a prerequisite, and failure 3.

For failure 2, a wiki-wide pass flags a letter bound to two meanings on two pages where one page is in the other's prerequisite closure. A reader who follows the link meets the letter changing meaning. The writer of the later page either renames the letter or states the switch in the text ("here y is the imaginary part, not the indeterminate"). The statement is checked by requiring the old meaning's page to be linked in the same paragraph.

Letters reused on pages unrelated by prerequisites are allowed. Asakura et al. show that meanings switching with scope is ordinary.

Picking identifiers out of LaTeX needs a small tokenizer:
- single letters with their subscripts and primes;
- Greek commands;
- `\mathcal{…}`-style letters;
- with operator commands (`\log`, `\exp`) and LaTeX structure skipped.

The tokenizer will have false positives. Each one costs a writer one resubmission. Whether to reject or warn at first is an open question below.

### Inherited symbols shown on the page

For every inherited symbol a page uses, the build writes a folded "Symbols on this page" note under "Before this page". Each line gives the symbol, its meaning, its role if stated, and a link to the page that binds it.

This is deterministic, and it carries the meaning to the point of use without adding words to the body. ScholarPhi's readers used symbol tooltips more than anything else, and Elhadad's definitions are links rather than inline text for the same reason.

The note is the collapse-safe form of "restate it": the body stays as written (2026-10-11-fragmentation-and-collapse).

### Position-sensitive tooltips (I-91)

A second step puts the meaning on the symbol itself. The build would rewrite the first occurrence of each bound or inherited symbol in each formula to carry a tooltip.

The pages already load MathJax 3 (exegenius-core/src/build.rs:96-118). MathJax's `action` extension provides `\texttip`. Whether the CDN bundle the build loads includes that extension, and how it looks with zensical's theme, is unchecked.

Rewriting LaTeX is the riskiest part of this entry, so it comes after the note above.

### The notation page, derived

With bindings stored, the notation page can be generated without an agent:
- symbols bound with scope beyond one page, or used on more than one page;
- in reading order;
- one entry per meaning, each linked to its binding page.

The notation task (exegenius-core/src/notation.rs) would then be removed.

The page states what it lists: "symbols used on more than one page". ScholarPhi's T failure shows that a list which looks complete and is not is worse than none.

### Actors

A symbol bound on one page and used on three or more is an actor in this sense. For actors the writer instructions require a `role`, and the role goes into the inherited-symbols note on every page that uses the symbol. Whether a role says what part the symbol plays cannot be checked deterministically. It is left to readers.

## How the simulation tests it

1. **Hypothesis H1.** Stuck points about symbols fall when bindings and the inherited-symbols note are present.
   - **Measure:** stuck points whose quote contains an identifier and whose note asks what it is or means. The read instructions already ask readers to record places they got past only by guessing (exegenius-core/src/simulate.rs:66-67). Level tags would sharpen the count (I-94, 2026-10-11-comprehension-goals).
   - **Comparison:** the same goals on the same wiki built with and without the note.
2. **Hypothesis H2.** Readers spend fewer pages and searches on goals that need a symbol's meaning.
   - ScholarPhi's correctness did not change while time and area viewed fell. The expected effect is on effort, not coverage.
3. **Goals that test meaning directly.** Mejía-Ramos et al.'s template "State the definition of a given term in the proof", applied to a symbol, gives questions such as "What does r count, and why is it a power of two?". This is the "meaning" goal kind of 2026-10-11-comprehension-goals.
4. **Leakage.** The nonce lens changes only plain text, never mathematics (exegenius-core/src/simulate.rs:575-597), so symbol letters reach the reader as written. A reader could know a conventional meaning from training, for example n for input length. For the paper-specific letters (r, q, T, κ) the convention is the paper's own, so parametric leakage is low. The no-wiki baseline measures it.

## Risks

- **Tokenizer noise.** Too many false rejections would teach writers to bind everything as `local`. Counting `local` bindings per page shows whether that is happening.
- **Overload strictness.** i, j, k and n recur everywhere with stable conventional meanings. They belong on the conventional list, not under the overload rule.
- **Role prose.** A required role invites filler. Keeping it to one clause, and not checking it, keeps the cost low.

## Open questions for steering

1. **Reject or warn.** Should unaccounted symbols reject the submission, or start as rewrite notes until the tokenizer's false-positive rate on the demo pages is known? Running the tokenizer over the 45 demo pages first would answer this cheaply.
2. **The notation task.** Generate the notation page from bindings and drop the notation task?
3. **When to show the note.** Always on pages that use inherited symbols, or only when the binding page is more than k pages back in reading order? The accessibility finding suggests distance is the wrong measure, and the plan tree (2026-10-11-proof-plans) could supply the argument's focus instead. Until that exists, "always" is the simple rule.
4. **Conventional symbols.** A project list in `exegenius.toml`, or a list derived from the reader profile's `knows` text by an agent?

## Current State
- The write answer is a markdown body with no structured fields (exegenius-core/src/write.rs:161-168)
- The write instructions ask the writer to define every term or symbol on the page where it is first used, and no check enforces the instruction (exegenius-core/src/write.rs:26)
- `notation::math` collects a page's inline and display mathematics as one string, with spacing removed for comparison (exegenius-core/src/notation.rs:96-104)
- `apply_notation` checks only that each listed symbol occurs in the mathematics of its named page (exegenius-core/src/notation.rs:107-142)
- The build writes a MathJax 3 configuration and loads MathJax from unpkg (exegenius-core/src/build.rs:96-118)
- The theme feature `content.tooltips` is set, and is used for the gloss titles of links (exegenius-core/src/build.rs:121-123)
- The nonce lens renames concept names in plain text only, skipping code and mathematics, so symbols reach simulated readers unchanged (exegenius-core/src/simulate.rs:575-597)

## Missing
- No symbol bindings in write answers or in `exegenius/graph.json`
- No tokenizer for identifiers in page mathematics
- No check that a page's symbols are bound, inherited, conventional or local
- No check for one letter bound to two meanings across a prerequisite link
- No inherited-symbols note on pages
- No tooltips on symbols
- No notation page derived from bindings

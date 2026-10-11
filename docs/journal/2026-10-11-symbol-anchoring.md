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

## Why the existing audit missed r

Exegenius already asks an agent to find unanchored symbols. The audit task reports "symbol: notation used without being defined on the page" (exegenius-core/src/audit.rs:95-100). `apply_audit` turns each such report into a rewrite note asking the writer to define the symbol (exegenius-core/src/audit.rs:184-193).

The demo project's own records show why that did not catch r. They are in the public repository lmmx/openai-nlogn-wiki, under `exegenius/`.

- **No gap was ever reported.**
  - `exegenius/audit.json` holds three audit rounds: 24, 36 and 43 pages audited. The rounds reported 8, 9 and 2 symbol gaps.
  - `synthetic-transform` was audited in all three rounds. Round 1 reported two step gaps; rounds 2 and 3 reported nothing.
  - The only symbol gaps on the pages in section 1 are q on `cyclic-convolution` and ∗ on `multiplication-algorithm`.
- **r arrived inside a concept name.**
  - The last audit task for the page (`exegenius/tasks/0153-audit-synthetic-transform.md`) lists among the reader's known concepts "Polynomial ring modulo y^r + 1 (also: negacyclic ring)".
  - The audit instructions count a symbol as followable "if the reader knows it". An earlier page's concept was known by construction (`Plan::known_at`, exegenius-core/src/plan.rs), and r was part of that concept's name.
- **The writer was told the same.** The write task for the page (`exegenius/tasks/0006-write-synthetic-transform.md`) lists the same known concept. Its gloss reads "reduced modulo y^r + 1 for a power of two r".
- **The gloss binds r to a range, not to what it counts.**
  - "For a power of two r" says what values r takes, not what it is the size of.
  - In Symlink's terms, the gloss makes no COUNT relation. The COUNT, "a list of \(r\) complex coefficients", is only in the body of the prerequisite page, which neither the writer nor the auditor of `synthetic-transform` sees.
- **The pattern is common in the demo.** Seven of the demo's 50 concept names contain a symbol or symbol-like token, for example "Polynomial ring modulo y^r + 1" and "Deterministic O(n (lg n)^(1-kappa)) multiplication".

So the failure does not need a tokenizer to fix, and an agent was already looking. Three properties of the current design let r through:
1. **Knowledge is per concept, not per symbol.** Knowing a concept implied knowing every symbol in its name.
2. **The audit reports only failures.** An auditor that notices nothing reports nothing, and the empty answer passes. ARC found omission to be the dominant error of LLMs over long inputs ("salient information is more often omitted than misrepresented"). An empty gap list is the form omission takes here.
3. **Meanings travel as glosses.** A gloss is one line, written at extraction, and can bind a symbol to a range without binding it to what it counts.

## Design

The design keeps the division the revised v1 design set (2026-10-10-revised-v1-design): the model reads and proposes, and code checks the evidence it gives. An agent does all the reading of mathematics. Code never parses LaTeX for meaning; it only checks that quoted text exists and that a named symbol occurs in it.

### Concept names are decomposed (I-90)

Each concept gains a `symbols` list: the symbols its name contains, each with a binding. Extraction proposes the list, and code checks that each symbol occurs in the name.

```json
{"id": "polynomial-ring-mod-yr-plus-1",
 "name": "Polynomial ring modulo y^r + 1",
 "symbols": [
   {"symbol": "y", "name": "indeterminate", "meaning": "the polynomial variable", "relation": "direct"},
   {"symbol": "r", "name": "record length", "meaning": "the number of complex coefficients in a record, a power of two", "relation": "count"}
 ]}
```

Every list of known concepts the tasks show, in write, audit, read and outline tasks, then prints a concept's symbol bindings under its name. A writer or auditor who meets "Polynomial ring modulo y^r + 1" sees "r: record length, the number of complex coefficients in a record". It no longer sees an undefined letter that it is told the reader knows.

### Display names for symbols

Each binding carries a `name`: a display name of one to four words ("record length", "digit count", "transform length"). The `meaning` is the full sentence.

The display name is what the page chrome shows next to a symbol:
- the inherited-symbols note;
- tooltips;
- the notation page;
- the decomposed concept names in tasks.

In each place it reads "r (record length)", not a sentence. It is also the natural-language handle a reader can search for: "record length" finds the page that binds r, where a search for "r" finds nothing useful. The demo's goal 4 searches that found nothing, "half" and "2κ", were searches for a symbol's role without a name to search by (2026-10-11-checkpoint-held-work).

Concept titles that contain mathematics render it as mathematics: "Polynomial ring modulo \(y^r + 1\)", with the plain name kept for the slug, the nav and search.

### Pages account for every symbol

The audit's symbol check changes from reporting gaps to accounting for every symbol. For each symbol in the page's mathematics, the auditor gives one of four kinds of evidence:

1. **Bound here.** A quote from this page that states what the symbol denotes. A use in a worked example ("Take \(r = 4\)") is not a statement of what r denotes, and the instructions say so.
2. **Inherited.** The page id of an earlier page or a prerequisite, with the quote there that binds it.
3. **In a known concept's name.** The concept, whose `symbols` list must then bind it.
4. **Reader background.** Standard notation the reader profile covers, named.

Code checks each piece of evidence:
- quotes occur word for word on the named page;
- the symbol occurs in the quote, with spacing removed, as `apply_notation` compares symbols now (exegenius-core/src/notation.rs:96-104);
- an inherited binding comes from a page earlier in reading order;
- a concept-name binding exists in that concept's `symbols` list.

A symbol with no valid evidence becomes a rewrite note, as now.

The difference from today is that silence no longer passes. An auditor has to say how each symbol is anchored, and "anchored by a concept the reader knows" now needs the binding to exist.

Completeness, meaning whether the auditor listed every symbol, is not checked in code. That would need parsing. Two cheap controls stand in for it:
- **Second audit.** A second, independent audit of a sample of pages; symbols one audit lists and the other does not go to the report.
- **Cross-checks.** The writer's own bindings and the inherited-symbols note give the auditor a list to check against.

### Writers state bindings

The write answer becomes JSON: the body, plus the symbols the page introduces, with `symbol`, `name`, `meaning`, `relation` (`direct` or `count`) and an optional `role`. Code checks each symbol occurs in the body's mathematics.

The writer task already lists the bindings in force from prerequisites and concept names. So the writer knows which symbols arrive bound and which it introduces.

### Overloaded letters

Bindings are stored per page, so letters bound to different meanings on two pages linked by a prerequisite edge are a mechanical join. In the demo that is q, y, w, z and b.

Whether two meanings differ is a judgement. "Number of digits" and "digit count" are the same meaning; "radix" and "digit count" are not. So the join produces candidate pairs, and the notation task, which already reads all pages, confirms or dismisses each one.

A confirmed overload becomes a rewrite note on the later page: rename the letter, or say in the text that it changes meaning, linking the page with the other meaning.

### Scope from the plan

Asakura et al.'s scope switches and the accessibility finding both ask where a binding holds. Hamami & Morris answer it inside their own framework:
- **Definitions are hypotheses.** A definition introduced during a proof is added to the hypotheses of the current proving intention. Of Pólya's c_i: "he now adds this definition of the ci sequence as a new hypothesis" (Review of Symbolic Logic, preprint p. 16). Footnote 14 counts definitions among the hypotheses.
- **Hypotheses pass down the plan.** They pass to the children of the node where they are added.

So a symbol bound at a plan node is in scope in that node's subtree. A use outside the subtree is a use out of scope, even if an earlier page bound it. With a plan (2026-10-11-proof-plans), the inherited-symbols note can show exactly the bindings in scope for the page's node. That replaces the question of how many pages back a binding may be.

### Inherited symbols shown on the page

For every inherited symbol a page uses, the build writes a folded "Symbols on this page" note under "Before this page". Each line gives the symbol, its display name, its meaning, and a link to the binding page.

The note carries the meaning to the point of use without adding words to the body (2026-10-11-fragmentation-and-collapse).

### Tooltips and the notation page (I-91)

A later step puts the display name on the symbol itself. MathJax's `action` extension provides `\texttip`. Whether the bundle the build loads includes it is unchecked (exegenius-core/src/build.rs:96-118).

The notation page is generated from the stored bindings, without an agent:
- symbols used on more than one page, in reading order;
- one entry per meaning;
- display name and full meaning on each entry.

The page says it lists cross-page symbols only, so it never implies completeness (ScholarPhi's T failure).

### Actors

A symbol bound once and used on three or more pages needs a `role`, which goes into the note on every page that uses it. Whether the role says what part the symbol plays is left to readers.

## How the simulation tests it

1. **Hypothesis H1.** Stuck points about symbols fall when bindings and the inherited-symbols note are present.
   - **Measure:** stuck points whose quote contains an identifier and whose note asks what it is or means. The read instructions already ask readers to record places they got past only by guessing (exegenius-core/src/simulate.rs:66-67). Level tags would sharpen the count (I-94, 2026-10-11-comprehension-goals).
   - **Comparison:** the same goals on the same wiki built with and without the note.
2. **Hypothesis H2.** Readers spend fewer pages and searches on goals that need a symbol's meaning.
   - ScholarPhi's correctness did not change while time and area viewed fell. The expected effect is on effort, not coverage.
3. **Goals that test meaning directly.** Mejía-Ramos et al.'s template "State the definition of a given term in the proof", applied to a symbol, gives questions such as "What does r count, and why is it a power of two?". This is the "meaning" goal kind of 2026-10-11-comprehension-goals.
4. **Leakage.** The nonce lens changes only plain text, never mathematics (exegenius-core/src/simulate.rs:575-597), so symbol letters reach the reader as written. A reader could know a conventional meaning from training, for example n for input length. For the paper-specific letters (r, q, T, κ) the convention is the paper's own, so parametric leakage is low. The no-wiki baseline measures it.

## Risks

- **Auditor omission.** An auditor can still leave a symbol out of its accounting. The sample of second audits measures how often this happens.
- **Overload strictness.** i, j, k and n recur everywhere with stable conventional meanings. The notation task dismisses such overload candidates, and an auditor can anchor them to the reader's background.
- **Role prose.** A required role invites filler. Keeping it to one clause, and not checking it, keeps the cost low.

## Open questions for steering

1. **Who decomposes concept names.** Is it extraction alone, or is decomposition rechecked when a page is written?
2. **Notation task.** Keep it to confirm overload candidates, or fold that judgement into the audit?
3. **Auditing coverage.** Is a second audit on a sample of pages enough to estimate how many symbols auditors miss?

Settled after steering: symbols are read by an agent, not tokenised in code. Code checks only the quotes the agent gives and that the symbol occurs in them.

## Current State
- The audit reports symbols used without definition, and `apply_audit` turns each report into a rewrite note (exegenius-core/src/audit.rs:95-100, exegenius-core/src/audit.rs:184-193)
- Known concepts are listed in audit tasks by name and aliases only, and in write tasks with a gloss for linked pages, so a symbol inside a known concept's name reaches the agent as known (exegenius-core/src/plan.rs, exegenius-core/src/audit.rs:106)
- The write answer is a markdown body with no structured fields (exegenius-core/src/write.rs:161-168)
- The write instructions ask the writer to define every term or symbol on the page where it is first used, and no check enforces the instruction (exegenius-core/src/write.rs:26)
- `notation::math` collects a page's inline and display mathematics as one string, with spacing removed for comparison (exegenius-core/src/notation.rs:96-104)
- `apply_notation` checks only that each listed symbol occurs in the mathematics of its named page (exegenius-core/src/notation.rs:107-142)
- The build writes a MathJax 3 configuration and loads MathJax from unpkg (exegenius-core/src/build.rs:96-118)
- The theme feature `content.tooltips` is set, and is used for the gloss titles of links (exegenius-core/src/build.rs:121-123)
- The nonce lens renames concept names in plain text only, skipping code and mathematics, so symbols reach simulated readers unchanged (exegenius-core/src/simulate.rs:575-597)

## Missing
- No symbol bindings in write answers or in `exegenius/graph.json`
- No decomposition of concept names into symbol bindings, and no display names for symbols
- No accounting of every symbol on a page with evidence; the audit reports only the symbols it notices
- No check for one letter bound to two meanings across a prerequisite link
- No inherited-symbols note on pages
- No tooltips on symbols
- No notation page derived from bindings

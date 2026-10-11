# 2026-10-11: Evidence layers

This is the seventh of the research entries listed in 2026-10-11-explanatory-structure-research. It takes up a point from the ChatGPT review of this journal: a verbatim quote check establishes that a quote exists, not that it supports what it is attached to. It covers proposal I-93, which extends I-36. It is independent of the other six entries. Nothing here is implemented.

## The problem

Exegenius checks quotes in three places:
- **Evidence for concepts.** Extraction rejects an evidence quote that is not in its source (exegenius-core/src/extract.rs:119).
- **Audit quotes.** The audit rejects a gap whose quote is not on the page (exegenius-core/src/audit.rs:140).
- **Citation marks.** Submitting a page rejects a `<cite>` mark whose text is not in the named source, word for word (exegenius-core/src/cite.rs:64-80).

All three establish that the text exists. None establishes that the text supports the claim it is attached to.

The ChatGPT review put it in four layers:
1. **Citation integrity**: the source exists and its metadata identifies it.
2. **Quotation integrity**: the quoted text appears in the source.
3. **Claim support**: the passage supports the claim attributed to it.
4. **Explanatory adequacy**: the claim is explained accurately at the reader's level.

Exegenius implements layer 2. The simulation addresses layer 4. Layers 1 and 3 have nothing.

## What the literature gives

### Supported, missing, contradicted

ARC (Elaraby & Litman 2026) labels each atomic fact against a summary as supported, missing or contradicted, using an LLM judge.
- **Validation.** On the legal set, its role score correlates with expert judgements better than ROUGE, BERTScore and FactScore. Kendall τ is 0.465 against 0.391 for ROUGE-1.
- **The best judge** in their comparison is a distilled reasoning model, at τ 0.509.

The same three labels fit a `<cite>` mark. Is the claim it follows supported by the quote, not addressed by it, or contradicted by it?

### Select first, then write

Slobodkin, Hirsch, Cattan, Schuster & Dagan (2024; abstract only) split grounded generation into "content selection, sentence planning, and sequential sentence generation". The selected source spans double as fine-grained citations, so each sentence is attributable to the spans chosen before it was written.

Exegenius's write task already hands writers the concept's evidence quotes and asks them to cite. The order is the reverse: the writer writes, then cites. A citation chosen after writing is more likely to be a nearby quote than the passage the claim came from.

### Attribution in multi-document planning

MiDAS-PRo (Nandy & Bandyopadhyay 2025) plans which source documents are cited together in each sentence before generating. Its attribution scores in human evaluation tie with a BM25 baseline (4.5 against 4.5, one expert, 12 samples). Planning citations did not, there, make attribution more accurate.

### This journal's own citations

ChatGPT judged the journal's bibliography untraceable: author-year mentions, few identifiers. The research register in 2026-10-11-explanatory-structure-research is layer 1 applied to the journal: a DOI or stable URL for every reference, and whether it was read in full. Its corrections table shows what layer 1 catches. In the chat's references it found:
- two wrong attributions;
- one paper that could not be found;
- several findings narrower than claimed.

## Design

### Layer 1: source metadata

The sources table (`[[sources]]` in `exegenius.toml`) gives each source an id, a path, and an optional title and URL (exegenius-core/src/config.rs:120). Rendered references use the title and URL (exegenius-core/src/project.rs:123).

Checking that a URL resolves needs the network, and the core is deterministic. Layer 1 therefore stays a metadata check:
- `status` reports sources without a URL or identifier;
- the init template's comment asks for a DOI, arXiv id or stable URL.

### Layer 3: a support check (I-93)

A support task runs after the writes, one per page with `<cite>` marks, as a batch of independent subagent tasks like the writes. For each mark it is given:
- the sentence the mark follows, the claim;
- the quote;
- the paragraph around the claim.

It labels the pair `supported`, `partial`, `unsupported` or `contradicted`, with a one-line reason for anything but `supported`.

On submit:
- **`unsupported` and `contradicted`** become rewrite notes on the page, quoting the claim and the reason.
- **`partial`** goes to the report only.

Following ARC, a claim sentence that makes several assertions can be split into atomic facts and each labelled. That is an option for later. The first version labels whole sentences.

The judge is an LLM, so the label is evidence, not proof. Two cheap controls apply:
- **Ignore no-op marks.** Grading the quote against itself always passes, so marks whose quote equals the claim sentence are skipped.
- **Measure judge noise.** A sample of marks is labelled twice by independent subagents, and disagreement is reported.

### Selecting before writing

The write task could ask writers to choose the passages they will rely on first, then write, with `<cite>` marks restricted to the chosen passages. This is Slobodkin et al.'s order. It changes the write task's shape more than the support check does, and MiDAS-PRo's result suggests planning citations alone does not raise attribution quality. It stays an option until the support check shows how often citations fail.

### Layer 4

Explanatory adequacy is what the simulated readers measure. The adversarial misreader (I-57; 2026-10-11-checkpoint-held-work) joins layers 3 and 4: its first misreading type, "a stronger claim than the text supports", is a reader-side test of the same gap the support check tests from the source side.

## How it is tested

- **Rates.** On the demo, writers added 47 `<cite>` marks in the repair round (2026-10-10-simulated-readers). Labelling them gives the first unsupported rate.
- **Hypothesis H1.** Pages with unsupported marks also carry more wrong claims in readers' graded answers. That would mean the unsupported marks point at content readers misread.
- **Hypothesis H2.** Marks written after a support round's rewrite notes are labelled `supported` more often than first-round marks.

## Open questions for steering

1. **When the support check runs.** After every write, or once before the build?
2. **What a `partial` label does.** Report only, or a rewrite note?
3. **Evidence quotes.** Should the support check also cover concept evidence quotes in `graph.json`, which back concepts rather than sentences?
4. **Init template.** Should the init template ask for a DOI or URL per source?

## Current State
- Extraction rejects an evidence quote that is not in its named source (exegenius-core/src/extract.rs:119)
- The audit rejects a gap whose quote is not on the audited page (exegenius-core/src/audit.rs:140)
- `cite::check` rejects a `<cite>` mark whose quote is not in its named source word for word, and does not test whether the quote supports the claim before the mark (exegenius-core/src/cite.rs:64-80)
- `Source::locate` turns a quote's position into a section and page locator for the rendered reference (exegenius-core/src/project.rs:105)
- A source spec holds an id, a path, and an optional title and URL, with no field for a DOI or other identifier (exegenius-core/src/config.rs:120-129)

## Missing
- No claim-support labels for citation marks
- No report of sources without a URL or identifier
- No passage selection before writing
- No double-labelling to measure judge noise

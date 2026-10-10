# 2026-10-10: Revised v1 design

Steering on the first six entries simplified the design. This entry replaces the parts of
2026-10-10-reader-model, -simulation-validity, -rules-and-grounding, -v1-scope and
-ideas-bank that conflict with it. Where they disagree, this entry wins.

## Steering received

- **No frontmatter.** Docs carry no frontmatter and no custom metadata. Frontmatter was
  never part of the brief; it came in with the R1 suggestion.
- **Output.** No Obsidian. The output is a docs tree that zensical can explore. exegenius
  writes that tree itself, which is trivial for v1.
- **Dependencies.** No deprecated crates.
- **Rust edition 2024.**
- **Test input.** The test input is the OpenAI "below n log n" manuscript and the lowrank
  write-up of it. Producing demos from them is separate work for another session, not
  part of developing the tool.
- **Aim of this phase.** An installable binary that can be used in another session, with
  feedback brought back here.
- **Simulating not-knowing is part of the design.** A model knows things the simulated
  reader does not. That is what lets the model treat those things as *known unknowns*:
  it can name exactly what the reader lacks and go looking for it. The earlier entries
  framed this as a problem to control. It is the mechanism.
- **Less complicated.**

## Knowledge graph, or inter-reference in the text?

Both, with a fixed direction between them. The graph is the source of truth, and the
inter-references in the text are generated from it.

- **What the graph holds.** The graph is one file, `exegenius/graph.json`, in the
  project, not in the docs. It holds concepts, `requires` edges between concepts, which
  concepts the reader already knows, and the source quotes that support each concept.
- **Why prose links are not enough.** Every deterministic step needs typed edges: the
  prerequisite closure, the reading order, cycle detection, and the reader's knowledge
  set during the audit. A link in prose does not say whether it is a prerequisite or an
  aside. Recovering edges from prose would take a model call on every run, so the core
  would stop being deterministic.
- **Why the metadata stays out of the pages.** Docs are build output. Code renders every
  link, the prerequisites list and the backlinks on a page from the graph, and checks
  every link the model writes into a body against the graph. Text and graph therefore
  cannot drift apart, and the pages need no metadata.

## Not-knowing as a known unknown

The reader's knowledge is a set K of concept names. Code maintains it, and the model never
does:

- K starts as the concepts the profile covers;
- K gains a page's concept once that page has been read in reading order.

The simulated reader is a stateless call. It receives K and one page, and its instruction
is: *the reader knows exactly K. Everything else that you know, the reader does not.
List every term, symbol or step on this page that the reader cannot follow from K, from
what this page itself explains, or from the pages it links to.*

The model's own knowledge is what makes this work. Because it knows what "dyadic" means,
it can see that the term carries a definition the reader lacks, where a lexical check sees
only an unknown token. The model only names the unknown; code does the search for it (see
the audit step below).

What this drops from the earlier entries:

- the competence-paradox framing;
- the leakage controls: no-page baseline, nonce words, ladder of reader models, page
  ablation;
- closed-book answering of goal questions.

Those controls exist to measure *learning*. v1 measures *gaps relative to a declared
boundary*, which does not need them. They stay in the ideas bank as later options.

## Pipeline: what the model does, what code does

Every model call is a Messages API request with structured outputs
(`output_config.format` of type `json_schema`). The schema is derived from the Rust
response type with `schemars` and parsed back with `serde`, using
`deny_unknown_fields`. Each response is cached on disk under the SHA-256 of the request
body, so re-running unchanged steps costs nothing and gives the same result.

| Step | Model | Code |
|---|---|---|
| `extract` | one call over all sources: decompose into concepts | validate ids, edges and quotes; find cycles; write the graph |
| `plan` | none | closure, reading order |
| `write` | one call per concept, in reading order: page body | check links; assemble the page |
| `audit` | one call per page, in reading order: list gaps against K | verify quotes, look up terms, edit the graph, queue rewrites |
| `build` | none | docs tree, `zensical.toml` |
| `run` | — | extract, plan, then write → audit repeated until no gaps or `--rounds N`, then build |

### extract

**Model input.** The model gets:

- the full text of every source, each labelled with its id;
- the target statement from `exegenius.toml`;
- the reader profile, as prose.

**Model output.**

```json
{
  "target": "below-n-log-n",
  "concepts": [
    {
      "id": "chirp-identity",
      "name": "Chirp identity",
      "aliases": [],
      "gloss": "nk = (n² + k² − (k − n)²)/2, which rewrites a DFT as a convolution",
      "requires": ["dft", "convolution"],
      "known": false,
      "evidence": [{"source": "lowrank", "quote": "converts these through a chirp identity to convolution."}]
    }
  ]
}
```

**Code checks.**

- **Ids.** Ids are slugs and must be unique.
- **Target.** `target` must name a concept.
- **Edges.** Every `requires` id must name a concept.
- **Quotes.** Every evidence quote must be a verbatim substring of the named source,
  after whitespace normalisation. A quote that fails is dropped. A concept the reader
  does not already know and that has no surviving quote gets flagged `unsourced`.
- **Cycles.** Strongly connected components in `requires` (petgraph `tarjan_scc`) count
  as cycles.

Any failed check sends one repair call. The repair carries the exact failures (dangling
ids, rejected quotes, the cycle members) and asks for corrected concepts. Whatever still
fails after the repair is reported and stops the run.

### plan

Code only. K0 is the set of concepts marked `known`. The pages needed are the closure of
the target over `requires`, stopping at known concepts. The reading order is a
topological sort of that closure. Ties are broken by the position of each concept's
first evidence quote in its source, so the order follows the paper where the graph
leaves it free. Output: the reading list with a hop count.

### write

**Model input, one call per needed concept in reading order.** The model gets:

- the concept record;
- for each required concept, its name, gloss and the relative link to use
  (`[DFT](dft.md)`);
- the names in K at this point in the order;
- the concept's evidence quotes;
- the source text;
- any rewrite notes from a previous audit.

The instruction: use only terms in K, required concepts (linked), or terms the page itself
defines.

**Model output.** `{"body": "<markdown>"}`.

**Code checks.** pulldown-cmark extracts every link from the body. A link to a page that
does not exist is a failure and triggers one repair call. A required concept left
unlinked is a warning. Code then assembles the page:

```markdown
# Chirp identity

<body>

## Prerequisites
- [DFT](dft.md)
- [Convolution](convolution.md)

## Used by
- [Chirp Z-transform](chirp-z-transform.md)

## Sources
> converts these through a chirp identity to convolution.
>
> — lowrank
```

The title and the last three sections come from the graph, never from the model.

### audit

**Model input, one call per page in reading order.** The model gets:

- K_i = K0 ∪ {concepts of the pages before page i};
- the page;
- the list of existing page names.

**Model output.**

```json
{"gaps": [{"term": "dyadic", "quote": "exact on dyadic real and imaginary parts",
           "kind": "term", "note": "dyadic rationals m/2^k are not in K and not defined"}]}
```

`kind` is one of `term`, `symbol` or `step`.

**Code handles each gap in four steps:**

1. **Check the quote.** A gap whose `quote` is not verbatim in the page is dropped. This
   filters out invented gaps mechanically.
2. **Check against K.** A term that matches a name or alias in K_i is dropped. The model
   misjudged the boundary, and code overrides it.
3. **Look the term up in the graph.** A term matching a concept c in the graph is a
   missing edge. Code adds `page → c` to `requires`, adds c to the needed set if it is not
   already there, and queues the page for rewriting. This lookup is the "search" for the
   unknown.
4. **Handle no match.** A `term` or `symbol` that matches nothing becomes a candidate
   concept, carrying its page and quote. The next `extract --extend` call turns
   candidates into full concept records. A `step` cannot be fixed by a link, so it
   becomes a rewrite note passed to the page's next `write` call.

Audit results go to `exegenius/audit.json`, with one record per round, so the gap count
can be followed round by round.

### build

Code only. `build` writes `docs/` from the stored page bodies and the current graph, so
the prerequisites lists and backlinks always match the graph. It also writes:

- `docs/index.md`: the target, the reader profile, and the reading list as links;
- `zensical.toml`: `site_name`, `nav` in reading order, and the arithmatex extension
  with MathJax for the source's mathematics;
- `docs/javascripts/mathjax.js`: the MathJax config file that zensical's math setup
  needs.

Pages are plain markdown with no frontmatter. Each title is the page's `# H1`.

## Project layout

```
my-explainer/
  exegenius.toml        # sources, target, reader profile, model
  sources/              # plain text or markdown (convert PDFs to text first)
  exegenius/
    graph.json          # concepts and edges (extract output, then code edits)
    pages/<id>.md       # model-written bodies
    audit.json          # gaps per round
    cache/              # one file per model call, keyed by request hash
  docs/                 # build output, explorable with `zensical serve`
  zensical.toml         # build output
```

```toml
target = "The O(n (lg n)^(1−κ)) multitape Turing machine integer multiplication result"

[reader]
knows = "Undergraduate CS: big-O notation, polynomials, complex numbers, the FFT as an algorithm"

[[sources]]
id = "openai"
path = "sources/openai-below-n-log-n.txt"

[[sources]]
id = "lowrank"
path = "sources/lowrank-109.md"

[model]
name = "claude-opus-5-5"
```

`exegenius.toml` configures the project. It is not documentation, so the no-frontmatter
rule does not apply to it. Sources are text files because the quote check needs text.
The API accepts PDFs, but its citations feature cannot be combined with structured
outputs, so PDF input waits.

## Crates

| Crate | Contents |
|---|---|
| `exegenius-core` | types and their schemas; prompt construction; validation (ids, edges, quotes, cycles, links); plan; applying audit gaps to the graph; page assembly; docs tree and `zensical.toml` writing. A `Model` trait stands in for the API, so every step can be tested offline with recorded responses. |
| `exegenius-cli` | the `exegenius` binary (clap). Implements `Model` over HTTPS with `ureq` plus the on-disk cache, and reads the API key from `ANTHROPIC_API_KEY`. |

Dependencies, all maintained: clap, serde, serde_json, toml, schemars, petgraph,
pulldown-cmark, ureq, sha2, anyhow (cli), thiserror (core). No YAML parser.

Edition 2024. Workspace layout and Justfile follow lmmx/polars-genson, as recorded in
2026-10-10-v1-scope.

Install: `cargo install --git https://github.com/lmmx/exegenius exegenius-cli`.

## Superseded

- **Entry contracts** (`requires`/`provides`/`goals` in frontmatter), the typed relations
  in frontmatter, the frontmatter sketch, the YAML/TOML question, and HTML rendering from
  the binary (2026-10-10-reader-model, 2026-10-10-v1-scope).
- **The rule table** (2026-10-10-rules-and-grounding). Of those rules, v1 keeps the
  checks above: dangling ids, cycles, unverified quotes, dangling links, and unlinked
  required concepts.
- **Golden corpus.** The hand-built FFT golden corpus and the university-notes corpus are
  replaced. Development tests use small fixtures with recorded responses. Real runs on
  the OpenAI manuscript and the lowrank write-up happen in another session.
- **Simulator plan.** The experimental simulator milestone and its closed-book goal
  answering (2026-10-10-simulation-validity) are replaced by `audit`.
- **Ideas bank changes:**
  - I-40 to I-47 and I-61 (closed-book simulation) are replaced by `audit`;
  - I-49 to I-52 (leakage controls) move to later, as learning measurement;
  - I-09, I-28, I-29 and I-70 to I-74 are superseded by this entry;
  - I-57 (adversarial misreader) stays later, and is the most natural next audit mode.

## Open questions

1. **Zensical config.** Should v1 write `zensical.toml`, or only `docs/`? The entry
   assumes it writes both, because the nav in reading order is the main thing the config
   adds.
2. **Page granularity.** v1 makes one page per needed concept. That brings back the
   atomisation risk from 2026-10-10-rules-and-grounding: if extraction yields 40
   concepts, the reader gets 40 pages. The extract prompt asks for concepts that deserve
   a page and folds minor terms into glosses. The page count is the first thing to check
   when feedback comes back.
3. **Reader profile.** The profile is prose, and the model maps it onto concepts
   (`known`) during extraction. Is that enough, or should the profile list concept names
   that code matches directly?

## Current State
- No source file in the repository implements the pipeline (`extract`, `plan`, `write`,
  `audit`, `build`, `run`) — the design appears only in
  docs/journal/2026-10-10-revised-v1-design.md
- docs/journal/ holds seven entries dated 2026-10-10 — this entry supersedes the
  conflicting parts of the other six

## Missing
- No Cargo workspace, `exegenius-core` crate, or `exegenius-cli` crate
- No `Model` trait, Anthropic HTTP client, or response cache
- No graph types, schemas, validators, planner, or audit-gap handling
- No docs tree, `zensical.toml`, or MathJax config writer

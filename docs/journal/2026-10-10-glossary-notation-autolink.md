# 2026-10-10: Glossary, notation and autolinking

Three items from the remaining roadmap in 2026-10-10-wiki-layout, chosen as the ones that
make the generated wiki read like one written by hand:

- **Autolinking:** link mentions of other pages that the writer left unlinked.
- **Notation:** a page listing the symbols a reader has to track.
- **Glossary:** a home for the thin pages that define a term rather than explain an idea.

## Autolinking

At build time, the first unlinked mention of another page's name or alias in a body becomes
a link (exegenius-core/src/autolink.rs). A wrong link misleads the reader, so the matching
is conservative:

- names of several words match in any ASCII case;
- single words match only when they look like names (an acronym, or a word with digits or
  inner capitals: DFT, FFT, L2), with their exact case. A single ordinary word such as
  "Record" or "Row" is never linked;
- a trailing "s" or "es" joins the link;
- code, mathematics, headings, existing links and the page's own names are skipped, and
  each target is linked at most once per page.

The first version linked to any page. On the demo's 45 bodies it added 20 links, and one was
wrong. On the n log n conjecture page, "Schönhage and Strassen gave a multiplication
algorithm" was linked to the page for the paper's own multiplication algorithm, which the
reader meets 38 pages later. A mention of a page the reader has not reached yet is informal
use, and often of a different sense. So links now point only back in the reading order
(earlier pages, plus glossary entries), which matches the reader model: what a reader knows
at a page is the pages before it. On the demo this gives 9 new links, all correct, for
example "cyclic convolution" in the phase twist page and "Fourier transform" in the
transform identity page. The writers had already linked most first uses.

## Glossary

The demo had pages of about 200 words that define a term ("Record", "Row of an array",
"Sup norm", "Indicator vector"). As nav entries and reading-order stops they diluted the
explanation. They are folded in the outline task, not in extract:

- Whether a page is minor is clearer once it is written. The outline task now shows each
  page's word count and the number of pages using it.
- Moving these concepts out of the plan would change prerequisite closure, audits and
  `known_at`; folding at the end changes only presentation.

The outline answer has an optional `glossary` list. Submit checks:

- every page is in exactly one part or the glossary;
- the target is not in the glossary;
- prerequisite order is checked only among pages in parts, since glossary entries are
  reference material outside the reading order.

The build renders the folded pages as entries of `docs/glossary.md`, each a heading with
an explicit anchor (`## Record { #record }`, attr_list) followed by the body, with
headings demoted one level, and a "Used by" line. `Site::href` and `Site::url` send every
link to a folded page to its entry. That covers body links, "before this page" notes, used-by
lists, diagram nodes and autolinks. A previous build's `docs/<id>.md` for a folded page is
removed through the manifest. The front page lists the glossary after the parts and
mentions it in the reading note.

## Notation

A notation task is handed out with the outline task, as one batch of two independent
tasks (exegenius-core/src/notation.rs). The agent reads the page bodies and lists the
symbols a reader tracks across pages: for each, the LaTeX, a short meaning, and the page
that introduces it. Submit checks each symbol against that page. The symbol, with spacing
removed, must occur in the page's mathematics, meaning the inline and display math spans
pulldown-cmark finds with `ENABLE_MATH`. Plain text does not count, so a symbol cannot be
pinned to a page that only mentions it in prose. A symbol can have one entry per meaning.
An empty list is valid for a wiki without notation.

`docs/notation.md` lists the symbols in reading order of their pages: the symbol, the
meaning, and a link to the page. It is a list rather than a table because LaTeX with `|`
(norms, absolute values) breaks markdown table cells.

## Verification

- `cargo test --workspace`: 20 unit tests and 2 end-to-end tests pass.
  - New unit tests:
    - autolinking: acronyms and multi-word names linked, plurals, ordinary words, own
      page, code, math, headings and existing links skipped;
    - glossary anchors in `decorate` and `Site::url`;
    - heading demotion outside fences.
  - The end-to-end test now covers:
    - an outline rejected for putting the target in the glossary and for a page in neither
      a part nor the glossary, then accepted with one page folded;
    - a notation answer rejected for a symbol missing from its page's mathematics, then
      accepted;
    - the glossary page and the absence of the folded page's file;
    - links and diagram nodes pointing at the glossary entry;
    - an autolinked mention;
    - the notation page;
    - the nav entries.
- `cargo clippy --workspace --all-targets -- -D warnings` is clean.
- **Demo run.** On a fresh copy of the deployed demo, `next` handed out `0199-outline` and
  `0200-notation` together. Two Claude Haiku subagents ran in parallel, each given only the
  printed prompt, and both answers were accepted on the first submit.
  - The outline folded 10 pages of 156 to 217 words into the glossary and put the other 35
    in 6 parts. The 10 were "Record", "Gaussian dyadic numbers", "Row of an array",
    "Complementary intersection graphs", "Rational bilinear form", "Array with a complete
    Cartesian address set", "Sup norm", "Complex modulus", "Linear convolution" and
    "Indicator vector".
  - The notation answer listed 45 symbols, from $n$ and $\kappa = 2^{-182}$ through $S$,
    $q$, $b$, $T$, $\zeta$ and the cost parameters.
  - `zensical build` reported no issues. Screenshots of the notation page, a glossary
    entry and the front page contents showed the rendering described above.

## Current State
- `autolink` links the first unlinked mention of each allowed page per body; `build` allows only pages earlier in the reading order and glossary entries (exegenius-core/src/autolink.rs, exegenius-core/src/build.rs)
- `OutlineAnswer` and `Outline` carry an optional `glossary` list; `apply_outline` rejects unknown pages, the target, pages also in a part, and duplicates, and requires every page in a part or the glossary (exegenius-core/src/outline.rs)
- `outline_task` lists each page's word count and number of dependents (exegenius-core/src/outline.rs)
- `next` issues the outline and notation tasks in one batch, each only if no current answer exists for the plan (exegenius-core/src/task.rs)
- `apply_notation` rejects empty symbols or meanings, unknown pages, duplicates per page, and symbols not found in the named page's mathematics; it stores `exegenius/notation.json` (exegenius-core/src/notation.rs)
- `build` writes `docs/glossary.md` and `docs/notation.md` when non-empty, adds them to the nav after the parts, and reports their counts (exegenius-core/src/build.rs)
- `status` reports the glossary size and the notation count (exegenius-cli/src/main.rs)

## Missing
- No audit of glossary entries in place: they were audited as pages before folding
- No check that the notation is complete; only listed symbols are checked
- No autolinking in the overview and part intros, which the outline agent links itself

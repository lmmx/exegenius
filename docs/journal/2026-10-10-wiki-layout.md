# 2026-10-10: Wiki layout

The first real run (the OpenAI "Integer multiplication below n log n" paper, Claude Haiku
in Claude Code, told only "use exegenius") produced 45 pages, deployed as
lmmx/openai-nlogn-wiki on Vercel. The content was sound but the site was a wiki in the
most primitive sense:

- the navigation was a flat list of 45 pages;
- the front page was a numbered list of 45 links with glosses;
- each page ended in plain "Prerequisites" and "Used by" lists and a truncated quote.

Nothing told a reader where they were, why a page mattered, or how the pieces fitted
together. Three smaller problems came up in the same run:

- the agent did every task inline, audits included, instead of handing them to fresh
  subagents;
- `reader.knows` was left at the template text;
- the agent copied the source file into `sources/` and left the original in place.

## Outline task

Grouping pages into parts and writing an overview needs judgement, so it is a task. The
outline is handed out once, after the last write, when the pages are final. It groups the
pages into parts and writes the front page overview and a short intro for each part.

Code checks the answer:

- every page is in exactly one part;
- no page sits in a part before a part holding one of its prerequisites;
- the overview and intros link only to pages.

Within a part, pages are sorted by the plan, which is a topological order. So when no
prerequisite points to a later part, the outline order (part by part) is a valid reading
order, and the agent never has to order pages by hand.

The outline records the set of pages it covers. If the plan changes, the outline is
stale and is handed out again. A finished project from an earlier version gets an outline
task on its next `exegenius next` and then builds, so upgrading the demo takes one
command and one task.

Running the outline as the first task, so that writes and audits used its order, was
rejected. Extend adds concepts after the outline would be written, and each new concept
would need a placement rule or a fresh outline. At the end, the outline sees the final
pages.

## Layout

Everything except the agent-written text is generated from the graph and the outline:

- **Front page:** the overview; a "how to read this wiki" note naming the reader and the
  target; the contents by part; and a mermaid map of the parts. The map has an edge from
  part a to part b when a page in b requires a page in a. Edges implied by others are
  removed, so the 1 → 6 edge does not clutter the six-part demo map.
- **Parts:** each part is a nav section with a landing page at `docs/<slug>/index.md`
  (the `navigation.indexes` feature). The page holds the intro, the page list with
  glosses, and a mermaid map of the edges within the part. A slug that collides with a
  page id gets a suffix: with directory URLs, `docs/x/index.md` and `docs/x.md` would
  share a URL.
- **Pages:**
  - an info admonition, "Before this page", that links the prerequisites and names the
    concepts assumed known;
  - the body;
  - a "Connections" diagram: prerequisites → page → pages that use it, with each node
    linked to its page;
  - "Used by", with glosses;
  - the source quotes in a collapsed `??? quote` block.
- **Links:** every inline link to a page gets the target's gloss as its title, and
  `content.tooltips` shows it on hover. The concept field docs now ask for names and
  glosses in plain text with Unicode symbols, because both appear where mathematics is
  not rendered: the nav, tooltips and diagram labels.
- **Theme features:**
  - breadcrumbs, and previous and next links in outline order;
  - instant navigation;
  - sections, section index pages, back-to-top and search highlighting.

  They are set only when `zensical.toml` has no `theme.features`.

Write tasks now list the pages that build on the page. The instructions ask for an
opening that says what the concept is and what it is for on the way to the target, so a
reader arriving from a link knows why the page matters.

Zensical's defaults already include admonition, details, attr_list, abbr and mermaid
superfences, so the generated markdown needs no `markdown_extensions` (setting it would
replace the defaults). A throwaway site built with zensical 0.0.69 checked the features
used here: section index pages, breadcrumbs, footer links, admonitions, collapsible
blocks, and mermaid nodes rendered as links. Hover tooltips and instant previews did not
show in headless Chromium screenshots, so they rest on the theme's documented behaviour.

## Agent guidance

- `next` names Claude Code's Agent tool (one call per task, all in one message) and tells
  the agent not to do audits itself: having read the sources and pages, it cannot stand in
  for the reader.
- Before the extract task, `next` refuses the template `reader.knows` as well as the
  template target, and asks the agent to ask the user when they said nothing about the
  reader. After the extract, the reader is baked into the graph and the check is skipped.
- The setup steps say to move the source files into `sources/`, not copy them.
- `exegenius init --vercel` writes `vercel.json` and adds `site/` and `.venv/` to
  `.gitignore`. `vercel.json` is the file written by hand for the demo deployment and kept
  verbatim: it installs zensical in a virtualenv, because Vercel's Python refuses a
  system-wide pip install, and builds into `site/`. `done` mentions the flag while no
  `vercel.json` exists.

## Verification

- `cargo test --workspace`: 18 unit tests and 2 end-to-end tests pass. The first
  end-to-end test now covers:
  - an outline rejected for a page placed before its prerequisite's part and for a page
    in no part, then accepted;
  - the two part landing pages, with intro links rewritten to `../`;
  - gloss titles on links;
  - the "Before this page" note, the "assumed known" line, the diagram, the folded
    quotes, the part map edge and the nested nav.

  `cargo clippy --workspace --all-targets -- -D warnings` is clean.
- **Demo run.** On a copy of the deployed demo, the new binary handed out
  `0199-outline`. The demo repository leaves out `sources/`, so a stand-in source was
  rebuilt from the graph's evidence quotes in the old nav order.
  - A Claude Haiku subagent given only the printed prompt submitted an outline that was
    accepted on the first attempt: six parts of 5, 10, 9, 7, 9 and 5 pages ("Problem and
    cost model" to "Assembling the algorithm and proof") and a two-paragraph overview.
  - `next` then built 45 pages in 6 parts, and `zensical build` reported no issues.
  - Screenshots of the front page, a part page and the Bluestein chirp identity page
    showed the layout above, with the diagrams rendered.
- The demo's page names keep their ASCII spellings ("kappa", "y^r"), since they come from
  the earlier extract. The Unicode guidance applies to new runs.

## Remaining roadmap

What is left from 2026-10-10-ideas-bank and 2026-10-10-v1-scope that still fits the task
loop, roughly in order of value to a reader:

- **Autolinking:** link the first unlinked mention of another page's name or alias in a
  body at build time (I-21 `mention.unlinked`, applied as a fix rather than a lint).
- **Notation:** a symbol table page from the symbols the audits flag (I-35).
- **Comprehension checks:** a "check yourself" question with an answer per page,
  generated as a task and graded by a fresh reader (I-44, I-82).
- **Glossary concepts:** concepts that need a sentence, not a page (the demo's "Record",
  "Side wire"), shown as definitions where they are used instead of thin pages.
- **Front page audit:** the overview audited by a fresh reader against the reader's
  background alone.
- **Claims:** claim checking against the sources (I-36), with source ingestion from PDF
  and arXiv (I-80).
- **Simulator hardening:**
  - nonce-word substitution (I-49);
  - leave-one-out page ablation (I-51);
  - planted-defect tests of the auditor (I-52);
  - the adversarial misreader (I-57, I-58).
- **Other:**
  - typed relations and entry kinds (I-09, I-10);
  - multiple homes per concept (I-12);
  - graph export (I-73);
  - page length as a cost (I-06);
  - Python bindings (I-79).
- **Prebuilt binaries:** a musl or older-glibc build, since the demo container's glibc
  2.36 could not run the 2.38-linked binary.

## Current State
- `next` issues one outline task once writes and audit rounds are finished and no outline covers the current plan, then builds (exegenius-core/src/task.rs)
- `apply_outline` rejects missing, unknown or duplicated pages, prerequisites in later parts, empty titles, intros or overview, and links to non-pages; it sorts each part's pages by plan order and stores the outline in `exegenius/outline.json` with the planned pages it covers (exegenius-core/src/outline.rs)
- `build` orders pages by the outline when it is current; it writes part landing pages under `docs/<slug>/index.md`, a nested nav, and a front page with the overview, contents and part map, and falls back to the flat reading order without an outline (exegenius-core/src/build.rs)
- Generated pages carry a "Before this page" admonition, a Connections mermaid diagram with linked nodes, "Used by" with glosses, and collapsed source quotes; inline links to pages get the gloss as title (exegenius-core/src/build.rs)
- `zensical_toml` writes `theme.features` from `FEATURES` only when missing (exegenius-core/src/build.rs)
- `exegenius init --vercel` writes `vercel.json` if missing and appends `site/` and `.venv/` to `.gitignore` (exegenius-cli/src/main.rs)
- `next` refuses `TEMPLATE_KNOWS` and `TEMPLATE_TARGET` while no graph exists (exegenius-cli/src/main.rs, exegenius-core/src/config.rs)
- `status` reports the outline's part count, or that it is still to come (exegenius-cli/src/main.rs)

## Stubbed
- Mermaid node links are `../<id>/` (or `<id>/` on the front page), which assumes zensical's default directory URLs (exegenius-core/src/build.rs)

## Missing
- No audit of the front page overview or part intros
- No autolinking of unlinked mentions in page bodies
- No glossary or symbol table page
- No check that audits were done by fresh subagents (unchanged from 2026-10-10-claude-code-task-loop)

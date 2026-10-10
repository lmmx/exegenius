# exegenius

Build evidence linked knowledge graphs and optimise explanation paths through simulated readers.

exegenius turns source documents (a paper, a write-up) into a wiki of linked explanatory
pages written for a reader with a stated background, then audits every page on that reader's
behalf and repairs what they could not follow. The output is a plain-markdown docs tree with
a `zensical.toml`, explorable with [zensical](https://zensical.org).

exegenius makes no model calls and needs no API key. It is a deterministic CLI that holds
the state and hands out tasks; Claude Code (or any agent that can read and write files and
run commands) does the reading and writing, and exegenius checks every answer before
applying it.

## Install

```sh
cargo install exegenius
```

## For agents

Run `exegenius next` in the folder to turn into a wiki, do what it prints, and repeat until
it prints `done`. In a folder with no `exegenius.toml` it prints the setup steps first. So
telling a coding agent "install exegenius and use it on this folder" is enough.

## Use

```sh
mkdir my-explainer && cd my-explainer
exegenius init
```

`init` writes `exegenius.toml`, a `sources/` directory, and a Claude Code skill at
`.claude/skills/exegenius/SKILL.md`. Put the sources in `sources/` (plain text or markdown;
convert PDFs to text first) and edit `exegenius.toml`:

```toml
title = "Integer multiplication below n log n"
target = "The O(n (lg n)^(1−κ)) multitape Turing machine integer multiplication result"
rounds = 3

[reader]
knows = "Undergraduate CS: big-O notation, polynomials, complex numbers, the FFT as an algorithm"

[[sources]]
id = "openai"
path = "sources/openai.txt"

[[sources]]
id = "lowrank"
path = "sources/lowrank.md"
url = "https://lowrank.github.io/latent-seminar/OpenAI-Math/109-integer-multiplication"
```

Then open Claude Code in that directory and ask it to run exegenius until complete. The
skill tells it the loop; `exegenius next` also prints the instructions with every batch.
When it reports done, browse the result with `zensical serve`.

## The loop

```
exegenius next            prints a batch of tasks, or builds the docs and prints "done"
  (agent)                 for each task: read the task file, write the answer file
exegenius submit <id>     checks the answer; rejects it with exact errors, or applies it
```

Tasks in a batch are independent, so the agent can hand each to a parallel subagent.
Audit tasks go to fresh subagents: the auditor stands in for a reader who has not seen the
sources or the other pages.

| Task | The agent | exegenius checks and applies |
|---|---|---|
| extract | decomposes the sources into concepts with prerequisites and verbatim quotes | slug ids, unique ids, known `requires`, a target, no cycles, every quote found verbatim in its source |
| write | writes one page body, in reading order, using only what the reader knows, terms it defines, and the prerequisite pages it links | no links to anything that is not a page; adds title, prerequisites, used-by and sources from the graph |
| audit | told exactly what the reader knows at that page, lists every term, symbol or step they cannot follow | every quote is on the page; drops terms the reader knows; turns the rest into prerequisite edges, candidate concepts or rewrite notes |
| extend | resolves the candidate concepts audits found into existing or new concepts | same checks as extract; adds the edges and queues the pages for rewriting |

| outline | groups the final pages into parts, folds minor terminology pages into a glossary, and writes the front page overview and part intros | every page in exactly one part or the glossary, the target not in the glossary, no page in a part before one of its prerequisites, links only to pages |
| notation | lists the symbols a reader has to track across pages, with their meaning and the page that introduces each | every symbol appears in the mathematics of the page named for it |

Rounds of write and audit repeat until an audit round finds no gaps, or `rounds` rounds have
run. Only pages whose audit input changed are audited again. Then the outline and notation
tasks are handed out together, and `next` builds `docs/` and creates or updates
`zensical.toml`.

## The wiki

- **Front page:** the overview, a note on who the wiki is for and how to read it, the
  contents part by part, and a clickable map of how the parts depend on each other.
- **Parts:** each part is a section in the navigation with its own landing page: the
  intro, its pages, and a map of the prerequisite edges among them.
- **Pages:** a "before this page" note linking the prerequisites (and naming what is
  assumed known), the body, a clickable diagram of the page's prerequisites and the pages
  that use it, and the source quotes the page rests on, folded away.
- **Glossary:** minor pages the outline folds away become entries on one glossary page,
  outside the reading order; links to them lead to their entry.
- **Notation:** the symbols, in the order they are introduced, each with its meaning and
  a link to the page that introduces it.
- **Links:** the first unlinked mention of an earlier page or a glossary term in a body is
  linked at build time. Names of several words match in any case; single words only if
  they look like names (DFT, FFT), so ordinary words are never linked. Every link to a page
  carries its one-line definition as a tooltip.
- **Navigation:** breadcrumbs, previous and next links in reading order, and search.
- MathJax for `$...$` mathematics.

`build` sets the theme features in `zensical.toml` only when it has none, so settings
you choose are kept.

## Publish

```sh
exegenius init --vercel
```

adds `vercel.json` (Vercel installs zensical and builds the site into `site/`) and
`site/`, `.venv/` to `.gitignore`. Push the folder to a GitHub repository and import it at
[vercel.com/new](https://vercel.com/new); every push redeploys.

Other commands: `exegenius status` (progress), `exegenius plan` (reading order),
`exegenius build` (build from the pages written so far).

## Project layout

```
exegenius.toml        configuration
sources/              source documents
exegenius/graph.json  concept graph, candidate concepts, queued rewrites
exegenius/state.json  task state: pending tasks, audit round, audited pages
exegenius/tasks/      task files and answer files
exegenius/pages/      page bodies
exegenius/audit.json  audit results, one record per round
exegenius/outline.json parts, glossary and front page
exegenius/notation.json symbols for the notation page
docs/, zensical.toml  build output
```

`build` only replaces the files it generated; other files in `docs/` and other settings in
`zensical.toml` are kept.

## Development

The design and its history are in the development journal, [`docs/journal/`](docs/journal/),
in the format described in [`docs/JOURNAL.md`](docs/JOURNAL.md) (copied verbatim from
[lmmx/giacometti](https://github.com/lmmx/giacometti)).

```sh
just test       # unit tests and the end-to-end task loop tests
just precommit  # fmt check and clippy
just install    # install the binary from this checkout
```

## License

MIT

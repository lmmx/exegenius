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
| extract | decomposes the sources into concepts with prerequisites and verbatim quotes; a name with mathematics in it also gets a title in LaTeX and a binding for each symbol (display name, meaning, direct or count) | slug ids, unique ids, known `requires`, a target, no cycles, every quote found verbatim in its source, every bound symbol in the title's mathematics |
| argument | reconstructs the plan of the sources' argument as a tree of steps: each step to show a claim has a method, a motivation (stated by the sources, recoverable from them, or supplied) and sub-steps; each step to infer quotes the sources; each page presents one step or is a concept some step relies on | the root shows the target on the target page; infer steps quote the sources word for word, in depth-first order unless marked out of order; explicit motivations quote the sources; hypotheses are known or earlier pages; every page is used |
| write | writes one page body, in reading order, using only what the reader knows, terms it defines, and the prerequisite pages it links; lists the symbols the page binds, each with the sentence that says what it denotes, and for each prerequisite page a relation: a predicate ("is reduced to") and the sentence that states it | no links to anything that is not a page; each bound symbol in the page's mathematics, its quote on the page, covering the symbol and not after its first use; a relation for every prerequisite page, its sentence on the page, linking or naming the prerequisite and containing the predicate's words; adds title, prerequisites, used-by and sources from the graph |
| audit | told exactly what the reader knows at that page, lists every term or step they cannot follow, and accounts for every symbol in the page's mathematics: bound on the page, on an earlier page, in a known concept's name, by the reader's background, or unbound | every quote is on the page and holds its symbol; every named page or concept binds the symbol; every symbol the writer bound is accounted for; drops terms the reader knows; turns the rest into prerequisite edges, candidate concepts or rewrite notes |
| extend | resolves the candidate concepts audits found into existing or new concepts | same checks as extract; adds the edges and queues the pages for rewriting |

| outline | groups the final pages into parts, folds minor terminology pages into a glossary, and writes the front page overview and part intros | every page in exactly one part or the glossary, the target not in the glossary, no page in a part before one of its prerequisites, links only to pages |
| overloads | for each letter a page binds that a page or concept name it builds on already binds, says whether the two denote the same thing | every candidate judged once; a letter with two meanings becomes a rewrite note on the later page |
| goals | writes goal questions of ten kinds (meaning, status, justification, summary, structure, transfer, example, motivation, non-routine, obvious-fails) with key points tagged by role (claim, reason, conclusion, background), testing whether the wiki teaches its reader (never shown in the wiki); a second, separate task writes held-out goals | pages exist; the goal count; key points of the kinds beyond meaning and status are not copied from a page |
| read | a simulated reader explores the wiki through `exegenius read` and `exegenius search` to answer one goal, and records where it got stuck and at what level (symbol, statement, step, chain, beyond) | the page budget; every citation and stuck point quotes a page it opened |
| baseline | answers a goal from the reader's background alone, with no wiki | non-empty |
| grade | scores all answers to a goal blind against the key points: covered, contradicted, or missing | every answer graded once, key points exist, none both covered and contradicted |

Before the first write, and again whenever the planned pages change, an argument task plans
the sources' argument (after Hamami and Morris's account of proofs as executed plans), so
each writer is told which step its page presents, why the method suits it, and which
motivations the sources leave unstated. The outline task is given the argument's main steps
as candidate parts.

Rounds of write and audit repeat until an audit round finds no gaps, or `rounds` rounds have
run. Only pages whose audit input changed are audited again. Then the outline task, and the
overloads task if any page rebinds a letter, are handed out together, and `next` builds
`docs/` and creates or updates `zensical.toml`.

The write and audit tasks never parse mathematics for meaning: the agents read it, and code
checks only that each symbol they name occurs in the LaTeX they quote, as whole TeX tokens.
An audit that says nothing about a symbol does not pass a page: a symbol the writer bound
must be accounted for, and a symbol in a known concept's name counts as known only if that
concept binds it.

## Simulated readers

Once the wiki is written, exegenius plays it as a game against simulated readers. A goals
task writes 3 to 6 goal questions with key points, of kinds taken from an assessment model of
proof comprehension (Mejía-Ramos et al.) and from the argument plan's motivations. A second
goals task, which does not see the first, writes 1 to 3 held-out goals: readers answer them
like the rest, but nothing from them feeds the repairs, so their coverage measures whether
the wiki teaches rather than whether it was repaired to answer its own questions. In each round, every goal goes to a
fresh subagent per reader profile and starting point (the front page, or a deep link to
the goal's last page, as if from a search engine). The reader sees the wiki only through
`exegenius read <task> <page>` and `exegenius search <task> <words>`, which serve pages by
opaque handles, enforce a page budget and log every step:

- With `nonce` on (the default), concepts the wiki teaches carry invented names, so recall
  tied to a name is cut off; a real name in an answer is recorded as a possible leak.
- The reader is told it knows exactly its stated background, and that everything else must
  come from the wiki. Pages show it the same note of symbols from earlier pages as the
  site does.
- Citations and stuck points must quote pages the reader opened.

A no-wiki baseline answers each question, and a blind grader scores every answer against
the key points. After each round but the last, stuck points, give-ups and key points no
reader managed become rewrite notes, and the pages are rewritten before the next round.
`exegenius/report.md` has the results: coverage per round against the baseline, for repair
and held-out goals apart; coverage by goal kind against the baseline and by key point role;
stuck points by level; each reader's path, searches, answer, stuck points and contradicted
key points; and the pages readers got stuck on most.
The goals and the report stay out of `docs/`.

```toml
[simulation]
rounds = 2     # 0 turns it off
budget = 15    # most pages per reader per goal
nonce = true

[[readers]]    # further profiles beside [reader]
name = "newcomer"
knows = "High-school algebra"
```

## The wiki

- **Front page:** the overview, a note on who the wiki is for and how to read it, the
  argument's main claim, method and motivation with its main steps, the contents part by
  part, and a clickable map of how the parts depend on each other.
- **Parts:** each part is a section in the navigation with its own landing page: the
  intro, its pages, and a map of the prerequisite edges among them.
- **Pages:** a "where this fits" note giving the steps of the argument from the main result
  down to the one the page presents (or the steps that rely on the page's concept); a
  "before this page" note linking the prerequisites under the predicates
  their relations give ("Radix-digit convolution *is reduced to* Cyclic convolution", with
  the sentence that says so as the link's tooltip), and naming what is assumed known; a
  folded note of the symbols the page takes from earlier pages and
  concept names (each with its display name, meaning and where it is bound); the body; a
  clickable diagram of the page's prerequisites and the pages that use it, each listed
  under its predicate; and the source
  quotes the page rests on, folded away. A concept whose name has mathematics in it has
  that mathematics rendered in its page heading.
- **Glossary:** minor pages the outline folds away become entries on one glossary page,
  outside the reading order; links to them lead to their entry.
- **Notation:** the symbols more than one page uses, in the order they are bound, each
  with its display name, meaning, the page or concept name that binds it, and the pages
  that use it. It is derived from the writers' bindings and the audits, with no agent of
  its own.
- **Links:** the first unlinked mention of an earlier page or a glossary term in a body is
  linked at build time. Names of several words match in any case; single words only if
  they look like names (DFT, FFT), so ordinary words are never linked. Every link to a page
  carries its one-line definition as a tooltip.
- **Navigation:** breadcrumbs, previous and next links in reading order, and search.
- **Source references:** writers can back a claim with
  `<cite data-source="id">source text</cite>`, which submit checks word for word. Pages
  show it as "(see §3.2 Gaussian resampling, p. 5)", located by the source's section
  headings and form-feed page breaks (as `pdftotext` writes them), hidden until a reader
  presses "Show sources" (`[site] citations = "shown"` makes them visible by default).
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
`exegenius build` (build from the pages written so far), and `exegenius read` /
`exegenius search` (used by simulated readers, for their own pending task only).

## Project layout

```
exegenius.toml        configuration
sources/              source documents
exegenius/graph.json  concept graph, symbols each page binds, candidate concepts, queued rewrites
exegenius/state.json  task state: pending tasks, audit round, audited pages
exegenius/tasks/      task files and answer files
exegenius/pages/      page bodies
exegenius/audit.json  audit results, one record per round
exegenius/argument.json the plan of the sources' argument
exegenius/outline.json parts, glossary and front page
exegenius/overloads.json verdicts on letters bound to two meanings
exegenius/simulation.json simulated readers: goals, runs, grades
exegenius/trails/     pages each pending simulated reader has opened
exegenius/report.md   how the simulated readers did
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

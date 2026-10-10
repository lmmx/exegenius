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
cargo install --git https://github.com/lmmx/exegenius exegenius-cli
```

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

Rounds of write and audit repeat until an audit round finds no gaps, or `rounds` rounds have
run; then `next` builds `docs/` and creates or updates `zensical.toml` (nav in reading order,
MathJax). Only pages whose audit input changed are audited again.

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

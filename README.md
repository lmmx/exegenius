# exegenius

Build evidence linked knowledge graphs and optimise explanation paths through simulated readers.

exegenius turns source documents (a paper, a write-up) into a wiki of linked explanatory
pages written for a reader with a stated background, then audits every page on that reader's
behalf and repairs what they could not follow. The output is a plain-markdown docs tree with
a `zensical.toml`, explorable with [zensical](https://zensical.org).

## Install

```sh
cargo install --git https://github.com/lmmx/exegenius exegenius-cli
```

This installs the `exegenius` binary. Model calls need `ANTHROPIC_API_KEY` in the
environment.

## Use

```sh
mkdir my-explainer && cd my-explainer
exegenius init                 # writes exegenius.toml and sources/
# put plain text or markdown sources in sources/ (convert PDFs to text first),
# then edit exegenius.toml: target, reader, [[sources]]
exegenius run                  # extract, write/audit up to 3 rounds, build
zensical serve                 # browse docs/
```

`exegenius.toml`:

```toml
title = "Integer multiplication below n log n"
target = "The O(n (lg n)^(1−κ)) multitape Turing machine integer multiplication result"

[reader]
knows = "Undergraduate CS: big-O notation, polynomials, complex numbers, the FFT as an algorithm"

[[sources]]
id = "openai"
path = "sources/openai.txt"

[[sources]]
id = "lowrank"
path = "sources/lowrank.md"
url = "https://lowrank.github.io/latent-seminar/OpenAI-Math/109-integer-multiplication"

[model]
name = "claude-opus-5-5"
effort = "high"
max_tokens = 128000
fallbacks = true
```

## Steps

`exegenius run` chains these; each is also a command of its own.

| Command | Model | Code |
|---|---|---|
| `extract` | decomposes the sources into concepts with prerequisites and verbatim quotes | validates ids, edges, target, cycles; drops quotes not found verbatim in their source; one repair call on failure |
| `plan` | — | prerequisite closure of the target minus what the reader knows; topological order, ties broken by source order |
| `write` | writes each missing or queued page body, in reading order | removes links to anything that is not a page |
| `audit` | told exactly what the reader knows at that page, lists every term, symbol or step they cannot follow | drops gaps whose quote is not on the page or that the reader knows; turns the rest into prerequisite edges, candidate concepts (resolved by the next `extract`) or rewrite notes |
| `build` | — | writes `docs/` (title, prerequisites, used-by and sources sections from the graph) and creates or updates `zensical.toml` (nav in reading order, MathJax) |

Run `exegenius run` again to continue from where the last run stopped: unchanged model
requests are answered from the cache.

## Project layout

```
exegenius.toml        configuration
sources/              source documents
exegenius/graph.json  concept graph, candidate concepts, queued rewrites
exegenius/pages/      model-written page bodies
exegenius/audit.json  audit results, one record per round
exegenius/cache/      model responses keyed by request hash
docs/, zensical.toml  build output
```

`build` only replaces the files it generated; other files in `docs/` and other settings in
`zensical.toml` are kept.

## Environment

- `ANTHROPIC_API_KEY`: required for `extract`, `write`, `audit` and `run`.
- `EXEGENIUS_API_URL`: overrides the API endpoint (default `https://api.anthropic.com`).
- `SSL_CERT_FILE`: if set, its certificates are the trusted roots (for TLS-intercepting
  proxies).

## Development

The design and its history are in the development journal, [`docs/journal/`](docs/journal/),
in the format described in [`docs/JOURNAL.md`](docs/JOURNAL.md) (copied verbatim from
[lmmx/giacometti](https://github.com/lmmx/giacometti)).

```sh
just test       # unit tests and the offline end-to-end pipeline test
just precommit  # fmt check and clippy
just install    # install the binary from this checkout
```

## License

MIT

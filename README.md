# exegenius

Build evidence linked knowledge graphs and optimise explanation paths through simulated readers.

exegenius is a tool for authoring a personal technical wiki: markdown pages linked so that
any technical term on a page leads to a page that explains it. Each page declares the
concepts it requires, the concepts it provides, and questions a reader should be able to
answer after reading it. From those declarations exegenius is intended to:

- check the corpus for structural defects (dangling links, prerequisite cycles, terms used
  without being linked, unsourced claims);
- plan a reading path from a reader's stated background to a target page;
- render the corpus as a static wiki;
- run closed-book simulated readers through the wiki to find where explanations fail
  (experimental).

## Status

Design stage. There is no code yet. The design, its sources, and the open questions are in
the development journal:

- [`docs/journal/`](docs/journal/): journal entries, starting with
  [`2026-10-10-origin.md`](docs/journal/2026-10-10-origin.md)
- [`docs/JOURNAL.md`](docs/JOURNAL.md): the journal format, copied verbatim from
  [lmmx/giacometti](https://github.com/lmmx/giacometti)

## Planned layout

- `exegenius-core`: corpus loading, concept registry, prerequisite graph, lint rules, path
  planning, HTML rendering
- `exegenius-cli`: the `exegenius` binary (`check`, `plan`, `graph`, `build`)
- `exegenius-sim`: experimental simulated-reader evaluation
- Python bindings come after the Rust crates stabilise

## License

MIT

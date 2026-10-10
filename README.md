# exegenius

Build evidence linked knowledge graphs and optimise explanation paths through simulated readers.

exegenius turns source documents (a paper, a write-up) into a wiki of linked explanatory
pages, written for a reader with a stated background. It is intended to work in five steps:

1. **extract**: a model decomposes the sources into concepts with prerequisite edges and
   verbatim supporting quotes; code validates ids, edges, quotes and cycles.
2. **plan**: code computes the pages needed to reach the target from what the reader
   already knows, and a reading order.
3. **write**: a model writes each page in reading order, using only terms the reader knows
   or that are linked; code checks every link and adds prerequisites, backlinks and
   sources from the graph.
4. **audit**: a simulated reader, told exactly what the reader knows, lists every term,
   symbol or step the reader could not follow; code turns each gap into a missing link, a
   new concept, or a rewrite note, and the write/audit loop repeats.
5. **build**: code writes a plain-markdown docs tree (no frontmatter) and a
   `zensical.toml`, explorable with `zensical serve`.

## Status

Design stage. There is no code yet. The current design is
[`docs/journal/2026-10-10-revised-v1-design.md`](docs/journal/2026-10-10-revised-v1-design.md);
earlier entries in [`docs/journal/`](docs/journal/) record how it was reached.
[`docs/JOURNAL.md`](docs/JOURNAL.md) is the journal format, copied verbatim from
[lmmx/giacometti](https://github.com/lmmx/giacometti).

## Planned layout

- `exegenius-core`: graph types, validation, planning, audit handling, docs tree output
- `exegenius-cli`: the `exegenius` binary and the model API client

## License

MIT

# 2026-10-10: Claude Code task loop

v0.1 (2026-10-10-pipeline-v0-1) called the Anthropic Messages API from the binary, which
needs an API key and spends API credits. The brief was a tool for Claude Code: give Claude a
CLI, point it at a directory, and say "run until complete". The API dependency came from
2026-10-10-simulation-validity, which argued that a closed-book reader needs a direct API
call with nothing else in context. 2026-10-10-revised-v1-design then replaced closed-book
answering with the known-unknown audit, but the API dependency was left in place. This entry
removes it.

## The inversion

exegenius now makes no model calls. It holds all the state and hands out tasks, and the agent
does the reading and writing. The prompts from v0.1 became the instructions in the task
files. The validation from v0.1 became `exegenius submit`, which rejects an answer with the
exact errors (the agent fixes the answer and submits again) or applies it. The repair calls
of v0.1 became that resubmit loop.

- `exegenius next` hands out a batch of tasks: extract, extend, write or audit. Every task
  in a batch is independent of the others, so Claude Code can hand each to its own
  subagent, in parallel. A new batch is computed only after every task in the current one
  is submitted.
- Each task is one markdown file. It holds the instructions, the input (for audits, the page
  and the reader's known concepts; for extract, extend and write, the paths of the sources
  to read), the answer format (a JSON schema, or markdown for a page body), the answer path
  and the submit command.
- Audit task files tell the auditor to use only the task file. The skill and the `next`
  output ask for a fresh subagent for each audit, so the auditor has not seen the sources
  or the other pages.
- `exegenius init` writes `.claude/skills/exegenius/SKILL.md` describing the loop. The
  `next` output also carries a ready-made subagent prompt for each task, so the loop works
  without the skill as well.

## Verification

- `cargo test --workspace` passes 18 tests: 16 unit tests and 2 end-to-end tests of the task
  loop in exegenius-core/tests/task_loop.rs, which use scripted answer files.
  - The first end-to-end test covers: an extract answer rejected for an invented quote and
    then accepted; three write tasks, one rejected for a link to a missing page; three
    audit answers submitted from three threads at once; extend; a rewrite task that shows
    the audit notes and the previous version; a clean second round over four pages; the
    build; and `next` returning done again.
  - The second end-to-end test covers `rounds = 1`: the writes queued by the last audit
    round are handed out, and then the docs are built with no second round.
- `cargo clippy --workspace --all-targets -- -D warnings` reports no warnings.
- In this session, Claude Code ran the full loop by hand on a small Bluestein's-algorithm
  source, using the binary installed with `cargo install --path exegenius-cli`:
  - 1 extract, 4 writes, then audit round 1, which found 2 gaps (the convolution theorem was
    used unstated; the half-integer powers $\omega^{n^2/2}$ were undefined);
  - 1 extend, which added a convolution-theorem page;
  - 3 writes, then audit round 2 with 0 gaps over 5 pages;
  - `done`, after which `zensical build` reported "No issues found".
- That run turned up three problems, all fixed before commit:
  - task paths were relative, so they broke for subagents running in another directory;
  - quotes in notes were escaped Rust-debug style (`\\omega`);
  - an audit round never closed while changed pages kept being re-audited, so the `rounds`
    limit did not apply. A round now closes when its last audit is submitted.

## Current State
- exegenius-cli depends only on anyhow, clap, and exegenius-core — no HTTP client, API key, or model configuration (exegenius-cli/Cargo.toml)
- `exegenius init` creates `exegenius.toml` and `sources/` when missing and always rewrites `.claude/skills/exegenius/SKILL.md` from exegenius-cli/src/skill.md (exegenius-cli/src/main.rs:49)
- `Project::load` canonicalises the project directory, so task files and printed prompts carry absolute paths (exegenius-core/src/project.rs:65, exegenius-cli/src/main.rs:110)
- `next` returns the pending batch if any task is unsubmitted; otherwise it hands out, in order of precedence, extract (no graph), extend (candidates), writes (missing or queued pages), or audits (pages whose audit-input hash differs from the last audit); once nothing remains it builds docs/ and zensical.toml (exegenius-core/src/task.rs:232)
- `issue` writes one task file per task under exegenius/tasks/ with id `NNNN-kind[-page]` and records the batch as pending in exegenius/state.json (exegenius-core/src/task.rs:204)
- `submit` parses the answer file (JSON with unknown fields rejected, or markdown for writes), applies it under a lock file so parallel submits serialise, and closes the audit round when its last audit is submitted: done if the round found no gaps, otherwise the round counter increases (exegenius-core/src/task.rs:322, exegenius-core/src/task.rs:118)
- `apply_extract` and `apply_extend` reject quotes not found verbatim in their named source, unknown source ids, duplicate ids, unknown `requires`, a missing target, cycles, colliding new ids, and terms without a resolution (exegenius-core/src/extract.rs:142, exegenius-core/src/extract.rs:203)
- `apply_write` strips a leading `# ` title, rejects empty bodies and links to anything other than a planned page, `index.md`, or an external URL, and clears the page's rewrite queue (exegenius-core/src/write.rs:140)
- `apply_audit` rejects quotes not found on the page and then applies the gaps as in v0.1: known terms dropped, other terms turned into edges, link or inline notes, or candidate concepts, and symbols and steps turned into rewrite notes (exegenius-core/src/audit.rs:128)
- `write_task` lists the sources by path and, for a page with an existing body, includes the audit notes and the previous version (exegenius-core/src/write.rs:65)
- `exegenius status` prints the concept and page counts, the gaps for each audit round, the candidate and rewrite counts, and pending tasks or done (exegenius-cli/src/main.rs:151)
- `rounds` in exegenius.toml (default 3) caps the audit rounds; the `[model]` section from v0.1 no longer parses (exegenius-core/src/config.rs)

## Missing
- No MCP server — the agent drives the loop through the CLI and the skill file
- No check that an audit was done by a fresh subagent — the task file and the skill ask for one, and submit cannot verify it
- No PDF input — sources are read as text (exegenius-core/src/project.rs:65)
- No grouping of several concepts onto one page
- No CI workflow in .github/

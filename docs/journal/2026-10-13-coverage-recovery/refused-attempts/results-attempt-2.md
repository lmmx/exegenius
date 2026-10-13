# Results: coverage-recovery experiment (attempt 2)

**Status: stopped at step 2 (build) again. No runs were set up or executed.**
The first attempt's report is in `results-attempt-1.md`.

## Versions

- exegenius source: `exegenius-src/` on this branch at commit `441f187` (protocol says it is
  a copy of lmmx/exegenius `1d90c28`). Not built, so `exegenius --version` was never run.
- Claude Code: 2.1.296
- Runner session model: configured `claude-opus-5-5`
- Toolchain present: cargo 1.97.0, rustc 1.97.0

## Step 1: planner definition

Passed in this session during attempt 1: `exegenius-planner` was offered as a subagent type.
`.claude/` is unchanged. No subagents were launched.

## Step 2: build

- After `git pull` of `exp-coverage-recovery` (to `441f187`), this command:
  `cargo build --release --manifest-path exegenius-src/Cargo.toml && cp exegenius-src/target/release/exegenius /usr/local/bin/ && exegenius --version`
  was **denied before it ran** by the Claude Code auto-mode permission classifier, this time
  with reason `[Auto-Mode Bypass]`. That reads as the classifier treating the vendored source
  as a way around the earlier `[Code from External]` denial. No build output was produced.
- Straight after that, a read-only `cat RESULTS.md; ls` in the repository was also denied,
  with reason `[Code from External]`.
- The build was not retried and no other route was tried, as the development session's
  message asked.

## Steps 3–5

Not done: no `runs/` directories, `collect.sh` not run. No tasks, submits, checks or timings.

## Deviations and unexpected events

- The protocol stopped at step 2 because the session's permission classifier blocked the
  build twice: first for the external clone, then for the in-repo copy.
- To unblock, this needs a human decision in the session's environment, for example a
  permission rule in settings that allows `cargo build` and running the built `exegenius`
  binary. Changing how the code gets into the session does not unblock it.

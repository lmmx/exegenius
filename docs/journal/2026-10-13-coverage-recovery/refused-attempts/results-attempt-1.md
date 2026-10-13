# Results: coverage-recovery experiment

**Status: stopped at step 2 (build). No runs were set up or executed.**

## Versions

- exegenius: lmmx/exegenius cloned to `~/exegenius`, checked out at `1d90c28`
  (`git rev-parse --short HEAD` printed `1d90c28`). Not built (see below), so
  `exegenius --version` was never run.
- Claude Code: 2.1.296
- Runner session model: configured `claude-opus-5-5`
- Toolchain present: cargo 1.97.0, rustc 1.97.0

## Step 1: planner definition

`exegenius-planner` was offered as a subagent type in this session (alongside
`exegenius-reader` and `exegenius-task`). Step passed. No subagents were launched.

## Step 2: build

- `git clone https://github.com/lmmx/exegenius ~/exegenius` succeeded; `git checkout 1d90c28`
  succeeded.
- `cd ~/exegenius && cargo build --release && cp target/release/exegenius /usr/local/bin/`
  was **denied before it ran** by the Claude Code auto-mode permission classifier, with reason
  `[Code from External]` (building and running code from a repository outside the session's
  attached scope). No build output was produced.
- Following the protocol's note on adding the repository, `add_repo lmmx/exegenius`
  (read access) was called. It returned `status: read_available`: the repository is public
  and served anonymously, so nothing was attached to the session. This did not change the
  permission situation, and the build was not retried, so as not to work around the denial.

## Steps 3–5

Not done: `setup-run.sh` was not run, no `runs/` directories exist, `collect.sh` was not run.
No tasks, submits, checks or timings to report.

## Deviations and unexpected events

- The only deviation: the protocol stopped at step 2 because the build command was blocked
  by the session's permission classifier. Nothing was substituted (no prebuilt binary,
  no other build route).
- To unblock in a future run, the environment likely needs a permission rule allowing
  `cargo build` (and running the resulting binary) for `~/exegenius`, or lmmx/exegenius
  attached to the session as a source repository.

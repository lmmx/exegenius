# Experiment {ID}: {one-line question}

This branch is a fixed protocol, run by a separate Claude Code cloud session (the runner).
The development session in lmmx/exegenius designed it and analyses the results; the runner
executes it exactly and reports facts. Preregistration: `calibration/plans/{ID}.md` in
lmmx/exegenius. Method: `calibration/PROTOCOL.md`.

**Question.** {question}

**Arms.** {n} runs per arm, same binary, configuration, definitions and prompts unless the
table says otherwise:

| Arm | Runs | Subagent type for each task | `model` | `effort` | Configuration lines (setup-run.sh) |
|---|---|---|---|---|---|
| {A} | {A1–A5} | {the type `exegenius next` names / general-purpose / …} | {"sonnet"} | {"high"} | {none / arm-a.toml} |

## Protocol

Do these steps in order. Do not change exegenius, the configuration, the subagent
definitions or settings in `.claude/`, this protocol, or anything a run produces. If a step
fails, stop that part, record exactly what happened, and go on to the report.

1. **Check the subagent types and parameters.** This session must offer every subagent type
   in the table, and its Agent tool must accept `model` and `effort`. If any is missing,
   stop: push nothing but `RESULTS.md` saying which, and report it (step 6).
2. **Attach and build exegenius.** lmmx/exegenius is the tool under test and part of this
   session's work. Never push to it. Do these as separate steps, each in its own tool call,
   in this order, each finished before the next starts:
   1. `add_repo` with `owner: lmmx`, `repo: exegenius`, `access: "push"`;
   2. clone it as `add_repo` instructs, to `/home/user/exegenius`, with enough history to
      contain commit `{commit}`;
   3. `git checkout {commit}` in the clone;
   4. `register_repo_root` for lmmx/exegenius at `/home/user/exegenius`, and wait for it;
   5. only then `cargo build --release`, and copy `target/release/exegenius` to
      `/usr/local/bin`.

   Check that `git rev-parse --short HEAD` in the clone prints `{commit}`. If a step is
   refused, stop and report it (step 6): do not try another route.
3. **Set up the runs** with `./setup-run.sh RUN [arm config]`, as the table says.
4. **Run each to the end of {stage}.** In each run directory, run the exegenius loop as
   `.claude/skills/exegenius/SKILL.md` describes, with these rules. Runs may go on at the
   same time.
   - Give each task to a fresh subagent of the arm's type, with exactly the prompt
     `exegenius next` prints for it: word for word. Say nothing else to the subagent.
   - In every Agent call, pass the arm's `model` and `effort`, and no other override.
   - Give each Agent call a description starting with the run and task id
     (`A1 0003-argument`): the registry matches transcripts to runs by it.
   - Do not read the sources or any task or answer file yourself, do not write or edit
     answers, and do no task yourself.
   - When `exegenius next` prints the reader instead of tasks, run it again.
   - Stop the run at {the stop condition}. Do not start further tasks.
   - A subagent that replies with an error and leaves its task pending gets a fresh one with
     the same type, prompt and parameters. Record every restart.
   - Record any subagent that touches another run's directory or a shared place such as
     `/tmp`, or stops processes by name.
5. **Collect:** `./collect.sh`.
6. **Report** in `RESULTS.md`, facts only: versions; subagent types, models and effort as
   passed and as the transcripts record them; per run, its tasks and submits from
   `runs/RUN/exegenius/log.jsonl`, with timings; each subagent's duration, tokens and tool
   uses from the task notifications; every restart, failure, deviation and anything
   unexpected. Do not judge the outputs.
7. **Commit and push** everything to this branch, `{branch}`, as Louis Maddox
   <louismmx@gmail.com>, with no Co-Authored-By, "Generated with" or session-link lines.
   Verify the push. Do not open a pull request.
8. **Tell the development session:** send a message to session `{dev session}` with the
   branch, commit and one line of status.

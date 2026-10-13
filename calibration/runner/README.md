# Runner template

Files for a dated experiment branch of the document's wiki repository (lmmx/openai-nlogn-wiki for the n log n paper). Copy them to the branch with:
- `.claude/`, written by `exegenius init` of the binary under test into an empty directory;
- `template/exegenius.toml` and `template/sources/`;
- `EXPERIMENT.md`, filled in from `EXPERIMENT.template.md`.

Then start the runner with `create_session` on that branch, with the wiki as its only source, so the definitions load as native subagent types.

**Lessons this template carries,** from journal 2026-10-13-handover-after-merges and the E3 deviation:
- **Attach exegenius** with `add_repo` (`access: "push"`). Cloning it unattached is refused (`Code from External`), and so is copying its source into the wiki.
- **Register the clone (`register_repo_root`) in its own step before building.** Doing both in one turn got the build refused.
- **Pass `model` and `effort` in every Agent call.** Otherwise subagents inherit the runner's effort, which is medium by default.
- **Prefix each Agent call's description with run and task id,** so the registry can match transcripts to runs.
- **Keep each task's scratch files in its scratch directory.** Shared `/tmp` files once let one run submit another's draft.

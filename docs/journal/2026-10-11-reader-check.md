# 2026-10-11: Reader check

The depth pilot converged, with two problems the loop did not catch (2026-10-11-depth-pilot):
- a wrong final page, introduced by a rewrite no audit checked;
- a background answer that marked 32 concepts known.

This entry records three changes. All are code, and none adds a model call.

## The background no longer shrinks the wiki

The depth pilot's background answer did two things:
- **It added 25 known concepts the graph lacked.** That vocabulary only stops audits turning those terms into new pages, which is the growth the task is for.
- **It used `covered` to mark three concepts known that extraction had planned pages for:** the discrete Fourier transform, cyclic convolution and carry propagation. That removed pages from the plan and changed which concept `[pilot] pages = 5` explains. A loop judged by its gap counts can be made to look better by explaining less, and `covered` let it.

`covered` is gone. The background answer can only add known concepts where the graph has none. A known concept whose name or alias matches a planned one is rejected as "stays as planned" (exegenius-core/src/background.rs). The background can now stop growth but never remove a planned page.

The instructions now say that the reader knows what their level teaches to everyone at that level. What only some courses teach is not known, and when unsure, a concept counts as not known. The growth limit bounds divergence whatever the background says.

## The person checks the reader

Once the background task is accepted, `next` prints the reader instead of a batch, once:
- the profile with the summary, as every later task will be told it;
- the known concepts the background added.

A second `next` goes on. `exegenius background --redo "<correction>"` takes the answer back out of the graph: its known concepts are removed and its summary cleared. The task is then given again, with the correction under "## Correction". Redo is refused once a later task has been handed out, the pilot chosen, or an argument or audit recorded, since those used the answer (exegenius-core/src/task.rs `redo_background`).

Whether a person is there to ask is known to the agent, not to `exegenius`. Claude Code's `CLAUDE_CODE_ENTRYPOINT` differs between sessions but is not documented, so the code does not read it. The pause is a fixed point in the task loop, and the skill tells the agent what to do there:
- if the user is following the session, show them the reader and ask;
- under `claude -p`, or when told to run unattended, run `next` again without waiting (exegenius-cli/src/skill.md).

An unattended run pays one extra `next` call.

## The last round's notes are dropped

The wrong `gaussian-resampling` page came from a rewrite queued by the last audit round. Such rewrites were written and never audited. Now, when the audit rounds end (after `rounds` rounds, or when the stop rule fires), their rewrite notes and candidate concepts are dropped (exegenius-core/src/task.rs). Every page stays as it was last audited. `status` prints how many of the last round's gaps are left on the pages; raising `rounds` has them fixed and audited.

Rewrites queued by overload verdicts and support checks are still written after the rounds end, unaudited. They rename a letter or rework a claim whose citation does not support it, and are narrower than an audit's rewrite.

## Current State
- A background answer adds known concepts only where the graph has none; a known concept matching a planned concept is rejected (exegenius-core/src/background.rs)
- After the background task, `next` shows the reader once instead of tasks, and `exegenius background --redo` gives the task again with a correction until a later task has used the answer (exegenius-core/src/task.rs, exegenius-cli/src/main.rs)
- The skill tells the agent to show the reader to a user following the session, and to go on without waiting under `claude -p` (exegenius-cli/src/skill.md)
- When the audit rounds end, the last round's rewrite notes and candidate concepts are dropped, and `status` counts the gaps left on the pages (exegenius-core/src/task.rs, exegenius-cli/src/main.rs)
- Rewrites from overload verdicts and support checks are written after the audit rounds end without an audit

## Missing
- No pilot run with these changes
- No audit of rewrites from overload verdicts and support checks

# Plan: execution profiles in exegenius (for review, not implemented)

**The goal.** Model, effort and agent definition become standard, recorded and testable settings of exegenius, not facts hidden in subagent definition files and the orchestrator's session. This makes the dimensions of `PROTOCOL.md` settable per task kind from `exegenius.toml`, and makes each run's intended configuration part of its run log. That is what a calibrated default, or a user's choice of calibre and budget, needs.

This revises proposal 1 of journal 2026-10-13-model-allocation in the light of E1, where the agent definition made no measured difference, and of the effort findings.

## Current state

- **`init` writes three definitions:**
  - `exegenius-planner` (`model: sonnet`), for extract, background, argument and extend;
  - `exegenius-task` (`model: inherit`), for every other kind;
  - `exegenius-reader` (`model: haiku`), for simulated readers.
- **No definition sets `effort`,** so every subagent takes the orchestrator's: medium in a runner session; whatever the author's session runs at.
- **`subagent()` in `exegenius-cli/src/main.rs` hard-codes kind → definition.** `next` prints that name.
- **The run log records none of model, effort or definition.**

## Proposed

**1. Profiles and allocation in `exegenius.toml`:**

```toml
[profiles.plan]            # a named execution profile
agent = "exegenius-planner"  # a definition init writes, or "general-purpose"
model = "sonnet"             # an alias or an exact model id
effort = "high"              # low | medium | high | xhigh | max; omit to inherit

[profiles.work]
agent = "exegenius-task"
model = "inherit"

[allocation]               # task kind -> profile
extract = "plan"
background = "plan"
argument = "plan"
extend = "plan"
default = "work"           # every other kind
```

**The defaults reproduce today's behaviour,** except as decided below.

**`effort` is an ordinal per model.** No equivalence is assumed across models. `exegenius` passes it through and never maps it to anything.

**2. `next` prints the whole call** for each task: the subagent type, `model` and `effort` to pass, and the prompt. The skill tells the orchestrator to pass exactly those.
- **Why:** it makes an arm of an experiment a configuration change, not a runner instruction.
- **E1 and E3 as configurations:** they could have been run as two `exegenius.toml` files.

**3. The run log's `issued` events record the profile.** That is its name, agent, model and effort, as intended. The *observed* model and effort stay with the transcripts and the registry, because `exegenius` cannot see which model ran.

**4. `init` writes the definitions the profiles name,** and nothing more. `general-purpose` needs no file. A definition stays a system prompt plus a tool list, and must earn its place by measurement (E1 found no advantage for `exegenius-planner` at high effort).

**5. Tests:**
- config parsing, including bad effort values and unknown profiles;
- `next`'s printed parameters per kind;
- the run log's recorded profile;
- `init` writing only the definitions named.

**Not included:**
- escalation on failure (journal 2026-10-13-model-allocation, proposal 3), until the registry shows where escalation would pay;
- presets for calibre and budget ("economy / balanced / quality"), until the Capability Map has evidence for them.

## Decisions for the author

1. **Implement the profiles as above?** This changes how tasks are allocated, which is architecture, so it waits for review.
2. **The default planning effort.** E1, with E3 still to come, supports `high` for planning on a paper of this kind: Overview calibre in 10 of 10 runs at high, against 1 of 21 at medium, in different batches.
   - **Options:** set `effort = "high"` in the default planning profile; or leave it unset (inherit) and document the finding; or decide after E3.
   - **The recommendation:** decide after E3, which takes about an hour.
3. **Writers, auditors and support checks keep inheriting** until an experiment says otherwise. `inherit` is recorded as such in the run log.

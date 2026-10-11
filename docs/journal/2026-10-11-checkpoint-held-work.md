# 2026-10-11: Checkpoint: work held off

This entry is a pause point. PR #6 (2026-10-10-simulated-readers) brought in simulated
readers and inline source references, with a two-round demo run. Work then stopped so
that outside background research can steer what comes next. This entry records what was
proposed and not started, so the next entry can pick it up, change it or drop it. Nothing
here is implemented.

## Proposed next steps, in the order last suggested

### 1. Held-out goals

The demo's round 2 reached 100% key-point coverage, against 73% in round 1 and a 53%
no-wiki baseline. But the repair note for a missed key point quotes the key point, so the
writers put it on the page, and round 2 partly measures whether the repairs landed rather
than whether the wiki teaches.

The proposal: split the goals into ones that feed repairs and ones that only measure, or
write fresh goals after the repairs. Only held-out goals count as evidence of learning.

Open questions:
- the split ratio with only 3 to 6 goals;
- whether held-out goals should come from a separate goals task that is not shown the
  repair goals.

### 2. Adversarial misreader (I-57, I-58)

A reader whose job is to find plausible wrong readings of a page, kept to named types:
- a stronger claim than the text supports;
- a necessary condition read as sufficient;
- a result applied outside its assumptions;
- an overloaded term read in the wrong sense;
- a prerequisite skipped while appearing to understand.

Then the smallest edit that blocks each misreading, re-tested for regressions. This is the
route to the origin entry's "proof-like" aim. R2's caution stands: a misconception an LLM
invents is not evidence of a real comprehension problem, so each misreading has to be one
the text genuinely permits, which a second reader confirms.

### 3. Leave-one-out ablation (I-51)

Credit per page: hide page i from a reader's lens, re-run the goal, and record the drop in
score. Pages that change nothing for any goal can be cut or merged. Pages whose removal
breaks goals they were not listed for point to a missing `requires` edge. The cost is one
reader run per page, per goal, per trial; caching does not apply, since the runs are
subagents and not API calls.

### 4. A ladder of reader profiles and models (I-07, I-50)

Run the same goals with:
- other reader profiles, such as a newcomer with high-school algebra (`[[readers]]`
  already supports this);
- several model sizes: Haiku, Sonnet, Opus.

Findings that hold across the ladder are about the wiki. Findings from one model alone are
about that model's prior knowledge. `next` currently asks for the smallest model only.

### Smaller items noted along the way

- **Leaks:** possible leaks are matched by name, so generic multi-word names ("exact
  integer multiplication", "multiplication algorithm") flag readers who never needed the
  wiki's name. Possible fixes:
  - count a leak only if the reader names a concept it never opened;
  - exempt names made of words that already appear in the question.
- **Search scent (I-55):** searches are logged with hit counts, but nothing reads them
  yet. Searches that find nothing ("half", "2κ" in the demo's goal 4) point to missing
  explanation or missing terms, and could become candidate concepts or rewrite notes.
- **Stuck points:** stuck points stayed level across the rounds (51, then 50) and moved
  between pages. "Signed coefficient packing" (17 over both rounds) and the main theorem
  page (14) now lead the table, and would be the targets of a third round.
- **Front page:** the overview and part intros are still not audited or read by a fresh
  reader against the background alone.
- **FAQ:** the steering rejected "check yourself" blocks on pages as enacting the
  simulation rather than simulating it. If simulated readers' questions ever reach the
  site, it would be as FAQ-style entries drawn from recurring stuck points. Not started.
- **PDF page links:** linking a reference to a page of a source PDF is deferred, possibly
  as a config option later. Not all PDF URLs support page anchors; the OpenAI manuscript
  released on GitHub is one that does not.
- **Prebuilt binaries:** a musl or older-glibc build, since the demo container's glibc
  2.36 could not run the 2.38-linked binary.
- **Agent hygiene:** in the cloud session, Haiku subagents several times made stray calls
  to session tools (remote session creation, interrupts, documentation lookups). The
  reader and baseline instructions now limit tools to the answer file and exegenius, which
  reduced but did not stop it. One empty remote session from the demo run,
  `session_01P426dp6Gdd192rGE7c9UbN`, was left unarchived.

## State at the pause

- **Demo copy:** the two-round simulation demo lives in a scratch copy of the deployed
  wiki with a stand-in source rebuilt from evidence quotes, not in lmmx/openai-nlogn-wiki.
  To reproduce it on the real project: publish 0.4.0, run `cargo install exegenius`, then
  tell Claude Code "use exegenius" in that folder. It hands out outline, notation, goals
  and two reading rounds, about 60 Haiku subagent runs.
- **Report:** `exegenius/report.md` from the demo run was shared in the session, not
  committed (the report is project output).

## Current State
- Simulated readers, grading, repairs and the report are implemented as described in 2026-10-10-simulated-readers (exegenius-core/src/simulate.rs, exegenius-core/src/grade.rs, exegenius-core/src/report.rs)
- Inline `<cite>` source references with section and page locators, hidden by default, are implemented (exegenius-core/src/cite.rs)
- Every goal feeds repairs; there is no held-out set (exegenius-core/src/grade.rs)
- Search queries and hit counts are recorded per run and shown in the report, and nothing else uses them (exegenius-core/src/simulate.rs, exegenius-core/src/report.rs)
- Crates are at version 0.4.0 (Cargo.toml)

## Missing
- No held-out goals
- No adversarial misreader or minimal blocking edits
- No leave-one-out page ablation
- No model ladder; `next` asks for the smallest model only (exegenius-cli/src/main.rs)
- No use of failed searches as signals
- No audit or simulated reading of the front page
- No PDF page links for references
- No prebuilt binaries

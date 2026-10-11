# 2026-10-11: Goal kinds, held-out goals, roles and levels

This entry implements the fourth research entry, 2026-10-11-comprehension-goals (proposals I-88, I-92 and I-94, and the held-out goals from 2026-10-11-checkpoint-held-work). The research entry left four questions open; the build answers each one, as recorded below.

## What was built

### Goal kinds (I-88)

`GoalKind` has the ten kinds of the research entry: `meaning`, `status`, `justification`, `summary`, `structure`, `transfer`, `example`, `motivation`, `non-routine` and `obvious-fails` (exegenius-core/src/simulate.rs:97). The schema gives each kind's question as its description, so the goals agent sees the template with the kind.

The instructions ask for at least half the goals of kinds other than `meaning` and `status`, at least one of `summary`, `structure`, `transfer` or `example`, and at least one `motivation` goal. The kinds work as a checklist: with 3 to 6 goals, not every kind appears.

The goals task now lists the argument's show steps (2026-10-11-argument-plan), each with its method and its motivation marked "stated by the sources", "recoverable from the sources" or "not stated by the sources". The instructions point at motivations the sources do not state as good tests of whether the wiki supplies them. That answers the research entry's risk that a `motivation` goal written without a plan might have no answer in the sources.

The verbatim rule extends to every kind except `meaning` and `status`: a key point of 20 or more characters that appears word for word on a page is rejected. `recall`, the old kind, is no longer accepted in answers, and goals already stored with it still load.

### Held-out goals

A second goals task writes 1 to 3 held-out goals. It runs in the same batch as the repair goals task, and neither sees the other's goals, so the held-out wording cannot follow the repair goals. Code marks its goals `held_out`, which is not in the answer schema.

Readers, baselines and grades treat held-out goals like the others. `repair` skips them: their runs' stuck points, give-ups and missed key points never become rewrite notes (exegenius-core/src/grade.rs:178). The report's summary gives key point coverage for repair goals and held-out goals in separate columns, and says that only the held-out column measures whether the wiki teaches beyond the questions it was repaired against. `status` reports both.

A simulation that started before held-out goals existed carries on without them, since adding goals after the first runs would make the rounds incomparable.

### Roles and three-way grades (I-92)

Each key point is a `point` with a `role`: `claim`, `reason`, `conclusion` or `background`. The instructions require at least one `reason` for a goal about why something holds. Key points stored as plain strings, before roles, load as claims.

Grades gain a `contradicted` list next to `covered`. A key point is covered, contradicted, or missing; submit rejects a key point listed as both covered and contradicted. The free-text `wrong` list stays, for wrong claims that answer no key point. The report shows contradicted key points under each run, and a table by role: how many key points of each role were graded, the share covered, and the number contradicted. Readers who make claims and miss reasons point to pages that state results without their justification, which is ARC's role bias.

### Stuck-point levels (I-94)

Each stuck point carries a level from Spratte's model: `symbol`, `statement`, `step`, `chain` or `beyond`, with the reader instruction from the research entry as the schema description (exegenius-core/src/simulate.rs:265). Stuck points recorded before levels load with no level.

The level goes into the repair note. A `chain` stuck point, "I followed the steps but not why they lead where they do", gets the note "say what this step is for", which is where the argument plan's breadcrumb and the writer's step apply. The report counts stuck points by level and shows each stuck point's level.

## The open questions, as built

1. **The kind list.** All ten kinds, since the plan now supplies material for `motivation`, `structure` and `summary` goals.
2. **The held-out split.** Held-out goals come on top of the repair goals, 1 to 3 of them, rather than one of the 3 to 6 being held out. Splitting the existing goals would leave the repairs with fewer signals.
3. **Who sets the level.** The reader, in its own terms. A grader inferring the level from the note would add a judgement the reader is better placed to make.
4. **Role tags.** Required on every key point. Roles on all key points give the role table its denominator.

One count is loosened: a goals task may give one goal beyond its range, since the count is there to bound cost, not as a rule of the method.

## Check against the demo

On a copy of the demo with the argument plan from 2026-10-11-argument-plan (the notes-only run), the two goals tasks went to fresh subagents on a mid-sized model, each given only the prompt `next` prints. The demo has no stored outline, so a one-part outline was written by hand to let the pipeline reach the simulation; the goals tasks do not read the outline.

Both answers were accepted on the first submit.
- **Repair goals (5):** `justification` (why the main theorem refutes the n log n conjecture), `example` (multiplying 7 by 14 through radix-4 digit lists), `motivation` (why the construction speeds up the two tape procedures rather than only the arithmetic), `transfer` (whether a recursion with W = 4, m = 5 and s = 18 or 20 smaller calls saves cost), and `summary` (how the cost accounting reaches the bound).
- **Held-out goals (3):** `transfer`, `motivation` and `example`.
- **Roles.** Every goal has a `reason` key point, and all four roles occur.
- **Arithmetic.** The worked examples check out: 7 and 14 have radix-4 digits (3, 1) and (2, 3), their convolution is (6, 11, 3), and carrying gives digits (2, 0, 2, 1), which is 98. In the held-out example, s = 7 against W m = 8 satisfies the strict inequality, and 1.75³ ≈ 5.36 against 2³ = 8.
- **The motivation goals drew on the plan.** Both took up the root's `recoverable` motivation and the tape-procedures step's `explicit` one: a tape transform pays for moving data at every layer, so saving arithmetic alone cannot beat n log n.

The check also shows a weakness of the held-out design. Held-out goal 7 asks almost the same question as repair goal 3, why the tape operations must be sped up and not only the arithmetic, with overlapping key points. Keeping the two writers apart keeps the wording apart, not the topic. Both writers saw the same argument steps, and the instructions point both at the same motivations. Repairs driven by goal 3 would then teach goal 7, and the held-out figure would overstate what the wiki teaches beyond its repairs. Nothing yet detects the overlap.

## Current State
- `GoalKind` has ten kinds and a legacy `recall`, with `holistic` and `name`; `Role`, `KeyPoint`, `Level` and `StuckPoint` load older records without roles or levels (exegenius-core/src/simulate.rs:97, exegenius-core/src/simulate.rs:154, exegenius-core/src/simulate.rs:265, exegenius-core/src/simulate.rs:292)
- `simulation_step` issues a repair goals task and a held-out goals task in one batch when the simulation has no runs yet, and `submit` stores repair goals before held-out goals (exegenius-core/src/task.rs)
- `goals_task` asks for 3 to 6 repair goals or 1 to 3 held-out goals and lists the argument's show steps with methods and motivation provenance; `apply_goals` checks the count, non-empty key points, pages, the retired `recall` kind and verbatim key points of holistic kinds, and marks held-out goals (exegenius-core/src/simulate.rs:932, exegenius-core/src/simulate.rs:936, exegenius-core/src/simulate.rs:1001)
- `Grade` records covered and contradicted key points and other wrong claims; `apply_grade` rejects unknown key points and key points both covered and contradicted (exegenius-core/src/grade.rs:114)
- `repair` skips held-out goals and their runs, and adds each stuck point's level to its rewrite note (exegenius-core/src/grade.rs:178)
- The report gives coverage for repair and held-out goals per round, coverage by goal kind against the baseline, coverage and contradictions by key point role, and stuck points by level (exegenius-core/src/report.rs:40, exegenius-core/src/report.rs:72)

## Missing
- No check that held-out goals cover different material from repair goals; on the demo, one held-out goal and one repair goal asked nearly the same question
- No measure of goal quality beyond the verbatim rule
- No per-kind or per-role figures for held-out goals alone
- No check that a `motivation` goal draws on a step of the argument

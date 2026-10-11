# 2026-10-10: Simulated readers

The design began as a game played against a simulated reader (2026-10-10-origin). A reader
agent starts from a stated background and a goal, moves through the wiki, and either
reaches the goal or gets lost. The author's aims are to maximise what transfers, to reach
goals in few hops, and to minimise confusion. 2026-10-10-revised-v1-design set the game
aside for v1 in favour of the known-unknown audit, which measures gaps against a declared
boundary rather than learning. That left in the ideas bank:

- **Navigation:** I-54, a navigation agent that reads, follows links, backtracks and
  gives up, with logged trajectories.
- **Goal answering:** I-40, I-42, I-43 and I-44, closed-book answering of goals with cited
  spans, a no-page baseline, and goals with reference key points.
- **Leakage and profiles:** I-49 nonce words, I-50 a ladder of reader models, I-07 reader
  profiles.
- **Scent:** I-55, information scent from search.

This entry builds the game. The steering was to simulate readers, not to enact them. The
goals test the wiki and never appear in it, and the results go to a report outside
`docs/`.

## The game

Once the wiki is written (pages final, outline and notation done), `next` runs the
simulation (exegenius-core/src/simulate.rs, exegenius-core/src/grade.rs):

1. **Goals.** A goals task, done by an author-side agent with the sources, writes 3 to 6
   goal questions. Each goal has a kind (recall or transfer), 2 to 5 key points, and the
   pages it needs. Submit rejects a transfer key point that appears word for word on a
   page (I-48's overlap check), since a reader could copy it without understanding.
2. **Reading.** Each round, every goal goes to one fresh subagent per reader profile and
   starting point:
   - the front page;
   - a deep link to the goal's last page in site order, as if arriving from a search
     engine.

   The reader sees the wiki only through two commands, which serve pages by opaque
   three-letter handles and log every step to `exegenius/trails/<task>.json`:
   - `exegenius read <task> <page>`;
   - `exegenius search <task> <words>`.

   A page budget (default 15) is enforced. Its path through the wiki is recorded by
   exegenius, not reported by the reader.
3. **Baseline.** In the first round, each goal also goes to a baseline subagent that
   answers from the reader's background alone, with the same renamed question (I-43).
4. **Grading.** A grade task per goal scores every answer against the key points. The
   answers appear under letters, in an order fixed by a hash of their task ids, so the
   grader cannot tell a reader from the baseline.
5. **Repair.** After every round but the last, these become rewrite notes on the pages:
   - every stuck point (page, quote, what the reader could not follow);
   - every give-up;
   - every key point no reader managed.

   The writes run, and the next round reads the repaired wiki.

The reader's instructions are its epistemic state specification (2026-10-10-simulation-
validity), in the known-unknown form of 2026-10-10-revised-v1-design:
- it knows exactly the stated background, and everything else must come from the wiki;
- it must record every place it got stuck or only guessed past;
- it must answer from the pages alone, with each step backed by a quote;
- an honest give-up beats a fluent unsupported answer.

Code checks what it can:
- every citation and stuck point must quote a page the reader actually opened, as it was
  shown;
- an answered outcome needs a citation;
- reading past the budget is refused.

`next` asks for read and baseline tasks to run on the smallest model available (I-50's
competence dial, in its cheapest form), each in a fresh subagent.

## Nonce names

With `nonce` on (the default), every concept the wiki teaches carries an invented,
pronounceable word in everything the reader sees: titles, bodies, links, the front page,
and the question. "Multitape Turing machine" became "zatokeg". Concepts the reader's
background covers keep their names. The renaming is deterministic from the concept id,
checked against the corpus for collisions, and reversed for graders and the report.

Real names of taught concepts in a reader's answer are recorded as possible leaks:
- multi-word names or acronyms only;
- a warning on submit and a line in the report, never a rejection.

The first demo run found a flaw in the renaming. "Record" is a concept (folded into the
glossary), and renaming it also renamed the verb: a reader met "an arithmetic count
[invented word] one operation" and reported, correctly, that a word stood where a verb
should be. The alias "row" did the same to "Row of an array". Single ordinary words are now
left alone, by the same rule autolinking uses. They give little away, and renaming them
breaks the prose. Five of the 56 rewrite notes from round 1 came from this. They were
removed from the demo copy before the rewrites ran.

## Report

`exegenius/report.md` (exegenius-core/src/report.rs) holds:
- a summary per round (runs, answered, gave up, key points covered, possible leaks)
  against the baseline;
- per goal: the key points, the baseline, and each run's path ("Front page → … →
  (gave up)"), searches with hit counts, answer, reason, stuck points, wrong claims and
  leaks;
- a table of the pages readers got stuck on most.

`status` prints coverage per round and the baseline.

## Source references

The steering asked for structured citations, rendered as a short reference such as
"(see p. 5)", hidden by default so they do not litter the page. Section names are as good
as page numbers. PDF page links are deferred, since not all PDF URLs support them.

Writers mark a claim with `<cite data-source="id">source text</cite>`, using HTML5's
element for a reference to a work. Submit checks the text against the source word for
word (exegenius-core/src/cite.rs). Each source records, at load:
- where its pages start, from the form feeds `pdftotext` writes between pages;
- where its section headings are: markdown headings, or numbered lines such as "3.2
  Gaussian resampling".

A quote's locator is the last heading before it and its page, e.g. "§3.2 Gaussian
resampling, p. 5". The build renders
`<cite class="exegenius-ref" data-source data-locator data-quote title>(see …)</cite>`,
keeping everything as attributes for later use. A stylesheet hides the references, and a
small script adds a remembered "Show sources" toggle to pages that have any.
`[site] citations = "shown"` flips the default. Audits and simulated readers see pages
without the marks.

## Verification

- `cargo test --workspace`: 25 unit tests and 3 end-to-end tests pass.
  - New unit tests:
    - renaming as whole words, keeping case and plurals, and skipping code and math;
    - nonce words;
    - grade letters;
    - citation checking, rendering with locators, and stripping.
  - The new end-to-end test, `simulated_readers_end_to_end`, covers:
    - goals, with a copied transfer key point rejected;
    - the renamed question;
    - a reader that sees invented names and handles and is refused past its budget;
    - a search for a real name that finds nothing;
    - a citation of a page not opened, rejected;
    - a leak warning;
    - a give-up and a baseline;
    - a blind grade with a bad key point rejected;
    - repairs as rewrite notes;
    - a second round;
    - the report's exact summary rows.
  - The first end-to-end test now covers a citation rejected and then rendered.
- `cargo clippy --workspace --all-targets -- -D warnings` is clean.
- **Demo run.** On a copy of the 45-page demo, every task was done by a fresh Claude Haiku
  subagent given only the printed prompt. All 21 round-1 tasks were accepted:
  - goals: 5 goals, 4 transfer and 1 recall, each answerable only from the wiki;
  - 10 readers and 5 baselines;
  - 5 graders.

  Round 1 results:

  | Goal | Kind | Readers (front, deep) | Baseline | Pages read |
  |---|---|---|---|---|
  | 1. why the machine refutes the c·n·lg n lower bound | transfer | 3/3, 3/3 | 3/3 | 10, 10 |
  | 2. 16-bit worked example: convolution length, coefficient size, carries | transfer | 3/3, 3/3 | 3/3 | 4, 4 |
  | 3. rounding criterion and the one-half threshold | transfer | 1/3, 2/3 | 2/3 | 11, 11 |
  | 4. total time from the exponent gap g, and why κ = g/2 | transfer | 1/3, 0/3 | 0/3 | 10, 13 |
  | 5. the ring modulo y^r + 1 and signed packing | recall | 3/3, 3/3 | 0/3 | 2, 4 |

  Mean coverage: readers 73%, baseline 53%.
  - All 10 readers answered.
  - They recorded 51 stuck points, most on "Signed coefficient packing" (9), "Cost
    accounting" (7), "Multiplication algorithm" and "Radix-digit convolution" (6 each).
  - One reader used two real names, both generic phrases ("multiplication algorithm",
    "bit complexity"), so leaks are reported as possible.
  - Goals 1 and 2 are answerable from undergraduate background, as the baseline shows, so
    they say little about the wiki. Goal 5 is where the wiki carries the answer (0/3
    without it, 3/3 with it). Goal 4 failed with and without the wiki. Its stuck points
    name the gap: the cost accounting page states "the smallest margin is 2^-181 = 2κ"
    but never says why κ is half the gap.
- **Repairs.** After the 5 spurious notes were removed, 51 notes went to 11 pages. Haiku
  writers rewrote all 11, adding 47 `<cite>` marks between them, each of which passed the
  word-for-word check against the source.
- **Agent hygiene.** Haiku subagents in this cloud session several times made stray calls
  to session tools: two empty remote sessions were created and archived, and other calls
  were refused. All of them self-reported. The reader and baseline instructions now limit
  tools to the answer file and exegenius. The session limit also cut three rewrites short;
  they were rerun after the reset, and the simulation resumed from its saved state.

## Current State
- `Config` has `[simulation]` (rounds, budget, nonce), `[[readers]]` personas, and `[site] citations` (exegenius-core/src/config.rs)
- `next` runs goals, then per round read and baseline tasks, then grade tasks, then repairs, after outline and notation; `Step::Repaired` re-enters the loop so the rewrites are handed out first (exegenius-core/src/task.rs)
- `Lens` serves pages by handle with invented names, searches them, renames and unrenames, and finds possible leaks; `read` and `search` enforce the budget and log trails (exegenius-core/src/simulate.rs)
- `apply_read` checks the trail, outcome, citations and stuck points; `apply_goals` checks pages and copied transfer key points (exegenius-core/src/simulate.rs)
- `to_grade`, `grade_task`, `apply_grade` and `repair` grade blind and queue rewrite notes (exegenius-core/src/grade.rs)
- `report` writes `exegenius/report.md` at build when runs exist (exegenius-core/src/report.rs, exegenius-core/src/build.rs)
- `cite::check`, `cite::render` and `cite::strip`, with `Source::locate` from form feeds and headings (exegenius-core/src/cite.rs, exegenius-core/src/project.rs)
- `exegenius read` and `exegenius search` CLI commands; `status` lists pending tasks first and reports simulation coverage (exegenius-cli/src/main.rs)

## Stubbed
- Possible leaks are matched by name only, so generic multi-word names ("multiplication algorithm") flag readers who never needed the wiki's name for them (exegenius-core/src/simulate.rs)

## Missing
- No ablation (I-51) or per-page credit; the report attributes stuck points to pages, not answers to pages
- No adversarial misreader (I-57, I-58)
- No check that a reader subagent used only the read and search commands; the trail shows what it opened through them, nothing more
- No navigation-scent diagnostics from searches beyond the hit counts in the report (I-55)

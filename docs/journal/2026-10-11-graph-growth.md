# 2026-10-11: Graph growth

The author's run of 0.5.0 on the n log n paper, on Haiku, stopped after its second audit round. The main session reported that "the extend and argument steps have widened the graph well past the target result". This entry records what happened, why the loop let it happen, and the change made to the audit and extend instructions.

## What happened

The figures come from the main session's own report in the author's run, with task ids as it gave them:
- Round 2 had 82 audit tasks (0151–0232), all accepted, so the graph had about 82 pages at that point.
- The extend task (0233) and the argument task (0234) after it were accepted.
- `next` then issued 119 write tasks (0235–0353), one per new page.

The new concepts the session named include logarithm, integer, ring, prime number, permutation, mathematical induction, bijection, Bezout identity, cyclic group and extended reals. The reader profile is "Undergraduate computer science: big-O notation, polynomials, complex numbers".

## Why the loop grows the graph

Two instructions combine with the loop:
- **Audit.** The instructions said "The reader knows exactly the concepts listed as known, together with what the reader profile states. Everything else that you know, the reader does not know: treat it as unknown to them, however elementary it seems to you." Read literally, a profile that names only big-O notation, polynomials and complex numbers does not cover logarithms. An auditor following that reports "logarithm" as a term the reader cannot follow.
- **Extend.** Its instructions began "An audit of the wiki found terms that a specific reader cannot follow". They allowed `known: true` for a concept the profile covers, but presented every term as already established to be unknown. A term the auditor had flagged therefore came back as a new concept with a page.
- **The loop.** Each new page uses terms of its own, which the next round's audits report and the next extend adds. Nothing bounds the growth except `rounds`.

At the end of the last round, `next` still runs extend on that round's terms and writes the new pages, but audits nothing further. This happens because extend comes before the round check (exegenius-core/src/task.rs:578-585, exegenius-core/src/task.rs:647-650).

The strict wording was meant to stop an auditor assuming knowledge the reader lacks. That aim stands for terms outside the profile. What it got wrong was the line between "covered by the profile without being named" and "not covered".

## What the earlier run shows

The audit and extend wording above dates from the first commit (39d17ea), so the author's earlier run used it too. That run built the v0.1.0 wiki from the same paper, with the same target, reader profile and `rounds = 3`, and its audits converged:

| Round | pages audited | term gaps | step gaps | symbol gaps |
|---|---|---|---|---|
| 1 | 24 | 16 | 20 | 8 |
| 2 | 36 | 9 | 7 | 9 |
| 3 | 43 | 2 | 0 | 2 |

It finished with 45 pages, 10,634 words, and 198 tasks in all (`next_id` 199). The 0.5.0 run had 82 pages by round 2 and issued 119 writes after it, and its task ids passed 0814.

So the wording lets the graph grow, but does not on its own make it grow: the same wording converged at 0.1. Something in 0.5 turns more of each page into reported terms. The change is not identified. The 0.5 run's own records would show it:
- its initial concept count;
- term gaps per round in `exegenius/audit.json`;
- words per page.

The suspects are the additions to the write task since 0.1, such as motivations, striking features, relation sentences and symbol meanings. They lengthen pages and bring more mathematics onto each one.

## What changed

**Audit instructions** (exegenius-core/src/audit.rs:150-155). The first paragraph now reads:

> The reader knows the concepts listed as known, and everything the reader profile covers: what anyone with that background has been taught, whether or not the profile names it (a profile of undergraduate computer science covers logarithms and proof by induction without naming them). Anything outside that, the reader does not know: treat it as unknown to them, however elementary it seems to you.

The symbol rules already allowed this for notation: a `background` anchor is "standard notation the reader profile covers".

**Extend instructions** (exegenius-core/src/extract.rs:73-84):
- They now say the terms are ones audits "judged" the reader cannot follow.
- They add: "Auditors can misjudge what the reader profile covers. A term the profile covers, whether or not it names it, is known to the reader: point it at a known concept, or add one with `known: true`. Known concepts get no page."

The answer format is unchanged: a known concept was already a valid resolution.

**The skill** (exegenius-cli/src/skill.md) adds two things:
- **Run to completion.** The session runs the loop until `done`, and exegenius sets no budget of time, tasks or tokens. The author's session stopped "within the budget" with no budget configured. Asked afterwards, it said it had "used the word 'budget' loosely". The skill also says that `[simulation] budget` is the most pages a simulated reader may open per goal, and asks the session to report pages on concepts the reader already knows in its final status, rather than stopping.
- **Batches over 20.** A batch larger than 20 is started 20 at a time, adding a subagent as each one finishes. `next` says the same when a batch has more than 20 tasks (exegenius-cli/src/main.rs).

## How it was tested

The two wordings were compared on two audit tasks from the demo, the pages `chinese-remainder-map` and `bit-complexity`. The tasks were generated by 0.5.0, and their reader profile is "Undergraduate computer science: big-O notation, polynomials, complex numbers". Each task was copied twice, identical except for the first paragraph, old or new. Each copy went to two fresh Haiku subagents, eight in all. To keep the answers short, they reported only term and step gaps and skipped the symbol accounting.

| Gap | old wording (2 runs) | new wording (2 runs) |
|---|---|---|
| `chinese-remainder-map`: bijection | 2 | 0 |
| residue | 2 | 0 |
| the `mod` operation | 1 | 0 |
| congruence notation | 1 | 0 |
| pairwise coprime | 2 | 2 |
| modular inverse | 2 | 2 |
| $\mathbb{Z}/S\mathbb{Z}$ notation | 1 | 1 |
| radix-digit coefficients, coefficient stream, source lengths | 2 each | 2 each |
| controlled cyclic shifts | 2 | 1, and as a step in the other run |
| term gaps in all, per run | 13, 8 | 6, 7 |
| step gaps, per run | 6, 4 | 5, 4 |
| `bit-complexity`: logarithm (`lg`), "Logarithms are not in the reader profile" | 1 | 0 |
| worst-case | 1 | 0 |
| term gaps in all, per run | 2, 1 | 1, 0 |
| step gaps, per run | 2, 1 | 1, 2 |

With the new wording, auditors stopped reporting vocabulary a computer science undergraduate is taught, such as bijection, residue, `mod` and logarithm. They kept reporting:
- terms from the paper's own construction;
- terms the profile plausibly does not cover, such as the modular inverse;
- the step gaps.

"Pairwise coprime" was reported under both wordings. Where the line falls remains the auditor's judgement.

The sample is two runs per cell on two pages, with the answer format cut down, so the table shows the direction of the change, not its size.

## Not built

**A cap or warning on growth per round.** The cause above is in the instructions. A cap would need a number with nothing yet to base it on. The timing record planned in 2026-10-11-wall-time counts tasks per batch, which shows pages per round.

**Telling auditors what a profile covers, concept by concept.** The profile is free text. The auditor's judgement of what it covers is the check, and extend can now overrule it.

## Current State
- The audit instructions count everything the reader profile covers, named or not, as known, and keep "however elementary" for terms outside the profile (exegenius-core/src/audit.rs:150-155)
- The extend instructions present audited terms as judgements and direct terms the profile covers to known concepts, which get no page (exegenius-core/src/extract.rs:73-84)
- The skill tells the session to run until `done`, states that exegenius sets no budget, and describes `[simulation] budget` (exegenius-cli/src/skill.md)
- `next` advises starting 20 subagents and adding one as each finishes when a batch has more than 20 tasks (exegenius-cli/src/main.rs)
- The v0.1.0 run, with the same audit and extend wording, target, reader profile and rounds, converged at 45 pages after 198 tasks, with term gaps falling from 16 to 9 to 2 over its three rounds (exegenius/audit.json of that run)
- After the last audit round, extend still adds that round's terms and their pages are written without audit (exegenius-core/src/task.rs:578-585, exegenius-core/src/task.rs:647-650)

## Missing
- No identification of the 0.5 change that turned the converging growth of 0.1 into diverging growth
- No limit or warning on the number of concepts extend adds per round
- No run of the changed instructions through a full wiki; the comparison covers two pages, two runs per wording, term and step gaps only

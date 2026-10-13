# 2. Reading the diagnosis

My reactions as I went through the failure analysis (2026-10-13-planner-handoff-run), in the order they came up.

## The planner's token count

The extract task used about 27k tokens in 39 seconds. The Sonnet extractions in the earlier comparison used 178k to 199k tokens over three to four minutes. That says the extraction was not a full extraction.

**Subagents per step.** It also made me want separate subagents for the different steps. Last I had looked there were only one or two definitions. I want:
- a full account of all the steps in a run;
- for each step, whether it needs a subagent of its own;
- a model for each, matched to how hard the step is and to the trade-off in time.

**Status:** proposal. Since 2026-10-13-planner-agent there are three definitions: `exegenius-task`, `exegenius-planner` (for extract, background, argument and extend) and `exegenius-reader`. Nothing yet sets a model per step from a judgement of its difficulty.

## Why Sonnet took longer

My reading at the time was that Sonnet did not take longer because it infers more slowly. It kept going: it checked its work and recognised that an answer was probably incomplete, where Haiku did not, or it would not have made the mistake at all.

> **Checked since.** The run's planner was Sonnet under the `exegenius-planner` definition, and it stopped just as early. Its transcript shows that it:
> - listed the outline of `paper.tex` (5,361 lines) with `wc` and a `grep` for sections and theorems;
> - read lines 1–440 in one Read call with `limit: 440`;
> - never read the file again;
> - wrote its answer with a single script.
>
> So under this definition, Sonnet did not keep going. The longer Sonnet runs were general-purpose subagents.

## "Laziness", and the system prompt

We call agents that do not finish the job lazy.

**Interpretation, untested:** my custom system prompt may have removed something from the general-purpose agent's prompt that kept it from stopping early. If so, we might add to the system prompt rather than replace it. I may also be wrong about this.

**Facts that bear on it:**
- The task file says "Sources (read them from disk)", and asks the planner to cover everything the target depends on. It does not say to read the whole file, or how.
- The definition says to "read a source file only if the task says to".
- No tool limit cut the reading: the 440-line limit was the planner's own.

A 5,000-line file is not large by the standards of what coding agents read; a single Python module can be that long. Context length is not the constraint either.
- **What ChatGPT told me:** Haiku has a context window of a million tokens.
- **The paper's size:** about 70k tokens.

**Status of the context-window figure:** not verified here.

## How are you supposed to explain a paper from its introduction?

Both full runs pulled nearly all of their evidence quotes from the introduction. No wonder the results were poor: a full paper cannot be explained from its introduction alone. That is a severe failure mode, and I had not noticed it until now.

**Why I had not:** I had iterated on the source code without doing runs, to conserve tokens. In hindsight that was a poor choice: changes went in whose consequences no run showed.

## A data pipeline not treated as one

This is a data pipeline, and we have not been treating it as one. A pipeline would check what each stage produced before the next stage consumed it. Here, an extraction made from the introduction passed straight into the argument, the pages and the audits. That thought leads to the next three notes (03, 04, 05).

## What "buried the lede"

**Interpretation:** if the plan comes only from the introduction, and the rewrites are where the rest of the paper enters, then every audit round has to add material. That would explain why rounds added content rather than tightening the pages.

The analysis supports part of this:
- the mechanisms on the chain pages arrived through audit-driven rewrites;
- round 3 then sent the steps of one such derivation back to be cited or cut.

It does not show that this explains the level gap counts. The counts were the same in the baseline run, and almost every round's gaps are in passages not reported before (07_the-handoff-and-what-to-measure.md).

## The switch to restricted subagents

The comparison runs that read the whole paper were general-purpose subagents. The full runs that read only the introduction used the restricted definitions. So I take the regression to have come in when I switched to subagent definitions, though we do not yet know why.

**Why I switched:** subagents were going off and using tools they should not have. That reason should be recorded, almost as an architectural decision record, so that nobody reverts the change without knowing what it fixed.

> **Checked since.** From the journal and the commits:
>
> **The definitions** came in with 0.6.0 (8cff21f, 2026-10-11, the reduction pass). `init` began writing `exegenius-task` (`model: inherit`) and `exegenius-reader` (`model: haiku`), both with `tools: Read, Write, Bash`.
>
> **Two reasons were recorded:**
> - **Stray tool use.** Three of the 38 Haiku subagents in the baseline pilot called `create_session`, a cloud-session tool the task did not need, and two others made other stray calls (2026-10-11-baseline-pilot). The reduction entry records: "A definition's tool list removes such tools; a prompt does not" (2026-10-11-reduction).
> - **Fixed context.** Each general-purpose subagent started with 42.5k–46.0k tokens of system prompt and tool definitions before reading anything (2026-10-11-wall-time). A restricted definition "may carry less", which was not measured.
>
> **A deliberate cut in reading.** The same pass reduced how much writers read. Write tasks began carrying the paragraph around each evidence quote, so that writers would "open the paper only for what the passages lack". The noted risk was "that writers read less around a concept than they do now". Nothing at the time considered how much the planning tasks read.
>
> **The extraction answers on record**, by how many evidence quotes fall in the first tenth of the source:
>
> | Run | Extraction | Quotes in the first tenth |
> |---|---|---|
> | 0.5.0 run (reused by `pilot-v0.6-convergence` and `full-v0.6`) | Haiku, before the definitions, `paper.txt` | 22 of 32 |
> | `full-v0.6-latex` | Haiku, `exegenius-task` | 21 of 23 |
> | `full-v0.6-latex-depth` | Haiku, `exegenius-task` | 24 of 27 |
> | `full-v0.6-latex-planner-handoff` | Sonnet, `exegenius-planner` | 27 of 27 |
> | comparison samples 1 and 2 | Sonnet, general-purpose | 37 of 82, 29 of 88 |
>
> So the 0.5.0 Haiku extraction, made before the definitions existed, already leaned heavily on the introduction, though less completely. The switch to definitions coincides with the extreme cases. It may not be the only cause.

**On the counts:** the run's answer file has 27 evidence quotes for 28 concepts, and the session that read the transcript reported 28 quotes. Either way, all of them fall in lines 1–440.

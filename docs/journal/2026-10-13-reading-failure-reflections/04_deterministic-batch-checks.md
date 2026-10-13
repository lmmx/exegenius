# 4. Deterministic checks from a single command

## One command, run over every claim

Some of the checks in note 03 need no model at all, and the quote spread is one. Others can be split, so the model does only the part that needs a model.

**The idea.** Suppose there is a list of findings, say a list of claims:
- Rather than asking an agent to go and check every claim, ask it for the command that checks one.
- A deterministic program then runs that command over all the claims.

The agent states the check once; the program applies it everywhere.

**Why it may be faster.** Repeated tool use means waiting for the model to generate each tool call. A single command handed to a program avoids that.

**Status:** proposal. It is not yet a design for running model-written commands safely (see below).

## What it needs

ChatGPT drew a distinction I agree with:
- **Properties a program can establish:** that a quote is in the source, that ids resolve, the order in which symbols are bound, required fields, the integrity of the graph. Many of these are what submit already checks.
- **Questions that need judgement:** whether a derivation is valid, whether a dependency explains rather than just refers, whether a proof step is adequately justified.

A model-written command also needs bounds:
- its scope stated;
- run in a constrained environment;
- a report of exactly what it checked.

A command running deterministically does not make it the right check.

**Status:** open question: which existing checks fall on which side, and where checks are missing.

## Pipelines

While thinking about this I recalled a post by someone at dbt Labs, arguing that agent context engineering is data engineering in a new form. I agree with that much, with the caveat that everyone sees their own expertise in these things. This project is a data pipeline whose stages are model calls, and the practices of data pipelines apply to it:
- validating each stage's output;
- making data lineage visible;
- reporting on every run.

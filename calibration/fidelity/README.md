# Explanatory Fidelity Evaluation: a research track

**Status:** designed, not yet run. Nothing here is measured. This document says what would count as evidence, which instruments exist, and the order in which to try them.

## What we are trying to measure

The project's aim was never only to state a paper's results. The author's account of the chain (journal 2026-10-13-reading-failure-reflections, part 7) is "a path through the graph of reasoning: the connections a reader makes between a few related concepts that make something click, an 'aha' moment". The project's theory (journal 2026-10-11-explanatory-structure-research and the entries it leads to) makes that precise.

**Understanding a proof.** It is "rationally reconstructing the plan of the proof" (Hamami & Morris 2024, journal 2026-10-11-proof-plans):
- the proof is an ordered tree of intentions, each step chosen for a reason;
- a reader understands a step when they can trace it "to the main objective through an uninterrupted chain of rational practical decisions";
- understanding is a matter of degree: the proportion of the plan the reader has reconstructed.

**An aha moment** is "where a subplan's place in the plan becomes visible to the reader" (journal 2026-10-11-aha-moments). It has three roles, which may coincide but need not:
- the **key step**, which does the decisive work;
- the **payoff**, which says why the chain matters to the goal;
- the **reconnection**, which makes an earlier, incidental-looking idea intelligible ("dyadic" in the author's first reading of the demo wiki).

**Explanation needs salience.** "The distinction between proofs that explain and proofs that merely prove exists only when some feature … of the result … is salient" (Lange 2024). A result page should name its salient feature.

**The failure modes:**
- **fragmentation:** the chain is spread across pages, and no page states it;
- **collapse:** everything made explicit until nothing stands out;
- **"years happen in days":** a sentence closes a chain with the steps missing.

## The four levels, and where each is instrumented

Levels 1 and 2 of `../PROTOCOL.md` §3 cover correctness and derivational fidelity of the plan. This track is about levels 3 and 4: whether the **wiki** carries a reader along the plan's paths, and whether a reader comes away able to use them.

| Instrument | Level | Exists? | Run on the n log n paper? |
|---|---|---|---|
| **A. Central-chain rubrics on the plan** (`../rubrics/`) | 2→3: does the plan contain the chain, step by step, with its reasons? | yes, from E3 | first in E3's batch |
| **B. Chain walk on the built wiki** | 3: following the chain's steps from page to page, is each step stated with its reason, on a page that links to the next? Where does the chain break? | **to build**: a judge task over the pages, with the rubric's steps as the route | no |
| **C. Simulated readers with held-out goals** (`exegenius-core/src/simulate.rs`, `grade.rs`; ten goal kinds including `motivation`, `relation` and `non-routine`) | 4: can a reader confined to the wiki answer questions the wiki was not repaired against? | built | only once, at v0.1, before held-out goals existed |
| **D. Attribution, key ideas and sentence ablation** (`exegenius-core/src/aha.rs`) | 3–4: which sentences do readers cite for key points, which idea do readers name as key, and does deleting a candidate pivotal sentence cost more relation or goal points than deleting a matched control? | built (`ablations` defaults to 0) | never |
| **E. People** | 4: does a reader actually understand the central construction? | the author's own readings so far | anecdotally |

**What already exists is enough to start.** The reader-side machinery for aha moments (C, D) was built in the 0.5 cycle and never run on the paper. The missing pieces are B, and the per-plan-step understanding measure the theory calls for ("the report can show reader success per node"), which instrument A begins at the plan level.

## Validity: what each instrument can and cannot show

From journal 2026-10-10-simulation-validity and 2026-10-11-aha-moments:
- **A simulated reader is a model that has read most of the internet.** Its default explanation for answering is that "it already knew". The no-wiki baseline detects that leakage, and held-out goals limit teaching to the test. Neither removes the leakage.
- **"A model reporting understanding is not evidence of understanding."** Detection uses attribution, ablation and agreement, never a reader's statement that something clicked.
- **Pivotal sentences found with one model may reflect its priors.** Rerunning with a ladder of reader models tests that.
- **"None of this shows that a person has an aha moment at the sentence. That needs people."** Instrument E is the calibration of every other instrument, and it is the one we have least of.
- **LLM judges (A, B)** judge against the paper and a rubric. They can say whether a step is on the page with its reason. They cannot say whether a reader would be helped by it.

## Order of work

1. **A on plans (in E3).** Does any configuration's plan reconstruct the central saving? If none does, writers cannot be expected to explain it, and the plan is the first place to fix.
2. **B on the wikis that already exist,** with no new run. The wiki branches `full-v0.6-latex-depth` and `full-v0.6-latex-planner-handoff` were planned at medium effort and hold built pages.
   - **The method:** walk the central saving's nine steps (`../rubrics/nlogn-central-saving.md`) through their pages, blind to which wiki is which.
   - **What it gives:** a baseline for "does the wiki carry the chain", against which a wiki planned at high effort can later be compared.
   - **Cost:** two to four judge tasks per wiki.
3. **C and D on a wiki planned at high effort,** once the author approves a full run (journal 2026-10-13-e1-config, proposal 2).
   - **The settings:** `[simulation] rounds = 1`, held-out goals, attribution, and a small number of ablations of candidate pivotal sentences on the central chain.
   - **What it gives:** the first measurement of whether the wiki teaches beyond its repair goals.
   - **The comparison:** the same round on the planner-handoff wiki, which exists, is the baseline.
4. **E with the author.** He reads the central chain's pages in both wikis, then answers a short set of held-out questions written before the reading. That costs little, and it is the only calibration of the rest.

## How results enter the calibration

These are separate outcome dimensions, recorded per run in the Evaluation Registry:
- chain-walk breaks per central chain;
- held-out goal coverage, by goal kind and against the no-wiki baseline;
- pivotal sentences confirmed by ablation.

They become **calibre gates** only once validated against instrument E. Until then they are reported, and nothing is optimised against them.

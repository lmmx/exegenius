# Model and effort policy, per pipeline stage (the calibration programme's deliverable)

**What it is:** the table the calibration programme exists to fill. For each stage of the exegenius pipeline it gives:
- the configuration to use (model, effort, instruction version);
- the evidence for it;
- its cost;
- how far the recommendation generalises.

**Its status:** written on 2026-10-14 at 22:20 UTC, after an outside review of the day's work. That review's point was that the programme had measured one stage deeply and the others not at all. This file makes that gap explicit. Every queued experiment must answer a question in this table, or remove a blocker to answering one (`RUNBOOK.md` §3).

**Its authority:** nothing here changes a pipeline default. A row marked "proposed" goes to the author. Only the author changes defaults.

## Final configuration report (2026-10-17, end of the calibration programme): the one table to read

**Status:** this concludes the programme the author set on 2026-10-17. All three open items are now closed:
- W3 (writing) is concluded;
- auditing is resolved from A2, its independent review and one narrow refinement (A3);
- simulated readers had one bounded experiment (SR1).

No further experiment is planned. Inconclusive results are recorded as such, with a conservative configuration.

**This table supersedes the dated tables below,** which are kept as records.

**Nothing here changes a default.** The labels in the Status column mean:
- **proposed:** goes to the author.
- **provisional:** the best configuration measured, but not shown to meet its fidelity standard.
- **calibrated:** a defensible configuration with evidence that it preserves the mathematics.

**Scope:** every row rests on one paper (`paper.tex`, 5,361 lines) and `master` as built (exegenius 0.6.0). Cost is measured cache read per task and is never converted to quota.

| Task | Model / effort | Prompt | Routing | Cost (cache read, measured) | Evidence strength | Mathematical fidelity | Status | Limitations |
|---|---|---|---|---|---|---|---|---|
| **Extraction** (`extract`) | Sonnet / `high` | `master`'s task | none | small (the cheaper alternatives would save ~1 M per wiki) | moderate: 34 extractions, judge-free (EX1), plus downstream plans | indirect: every extraction tested fed plans judged 19/19 faithful against the paper (E9–E11) | **calibrated, for this paper** | one paper; no direct fidelity measure of the extraction itself |
| **Background** (`background`) | Sonnet / `high` | `master`'s task | none | small | weak to moderate: 8 runs, all feeding ceiling plans | for planning only, indirectly. Concept lists vary (median overlap 0.67) | **calibrated for planning only; provisional for writing** | its effect on the written pages is unmeasured. W3's pages show no sign that background limits them: the writers' gap is derivation, not level |
| **Argument planning** (`argument`, `extend`) | Sonnet / `high` | **the combined instruction** (PR #13, merged cd92445) | none | 3–5 M per run | **strong for this paper:** three blind Opus judges, preregistered (E9–E11, S2, S3, S6) | **19/19 faithful and 9/9 central on every extraction tested.** Opus `xhigh` is not shown better above the ceiling (E11, p = 0.69). Effort above `high` buys nothing | **calibrated, for this paper** | one paper. The shared debug binary predates the merge; rebuilding it from `master` is an operational step (the author) |
| **Writing** (`write`) | Sonnet / `high`; Opus / `xhigh` on 0015 and 0016 in this wiki | `master`'s task | **the 0015/0016 exception only** (the author's, on W2b's direct evidence); **no key-step rollout** | Sonnet ~1.3 M per page; Opus ~2.1 M (W3 means: ~1.7×; 2–3× on W2b's hard pages) | moderate: W2, W2b and W3 (6 new key-step pages, 3 controls, 24 page-runs, 72 blind Opus judgements, preregistered) | **not adequate on key-step pages.** Sonnet `high` met the fidelity standard on 4 of 12 key-step page-runs (W3), Opus `xhigh` on 4 of 6. The failures are **pages that leave assigned steps underived**, not wrong mathematics: 1 major error in 24 W3 page-runs. Opus's advantage is page-specific (k = 2 of 6: large on 0036, 18 of 19 steps against 4; clear on 0018; none on 0021, 0031, 0033; Sonnet ahead on 0032) | **provisional:** the status quo, by W3's preregistered matrix (k = 2–4 with R2 inconclusive) | the standard is strict at 18 steps, and no writer met it on 0031 or 0032. Two cells have one outlying judge. W3 cannot say which pages need Opus in advance |
| **Fidelity audit** (`audit`) | Sonnet / `high` | **the R text:** A2's SI text (FIDELITY + SOURCE) plus REFINE, with the auditor reading the sources its task names. **Implemented** in 7bc391f (`audit_instructions`); before it, production ran the page-only audit (B) | none | R 0.39 M per audit; B 0.13 M. R costs ~3× B: ~+9 M per audit round on a 35-page wiki, an estimate | moderate: A2 and A3 (planted errors on six pages, one run per arm, preregistered). F_A was decided by two independent blind reviews, which agree on 10/10 verdicts and accepted 0 of 4 decoys | **R meets the audit standard at this scale (A3, row 1).** It names 13/13 errors only the paper reveals (B 0/13) and 25/28 page errors (B 14/28), with 0/18 sham alarms. Its false alarms are 0.50 per clean page by the independent reviewer, **exactly at the bound.** Its remaining false alarms are true statements whose page omits a hypothesis or condenses a step. **B is not adequate:** it misses every error that only the paper reveals | **approved by the author and implemented (7bc391f).** Adequate at this scale; fragile on false alarms | F_A rests on four alarms on six pages and sits at its bound. REFINE was written after seeing A2's rejected alarms, so the pass shows it fixes the observed failure, not that it generalises. Recall is on planted errors only, in one paper |
| **Support check** (`support`) | Sonnet / `high` | `master`'s task | none | 0.14–0.24 M per task | moderate for "the task is the limit" (A1: the same misses for all three models) | **not adequate:** support recall 0.60 against 0.70 (A1); it misses both generalisation errors | **provisional**, unchanged | the support task cannot see generalisation errors. **If R is adopted, this matters less:** R named 4/4 generalisation errors in A3 (and SI 4/4 in A2), so a support-instruction experiment (A1b) is not needed |
| **Simulated readers** (`simulation`) | readers and baselines Haiku / `high`; goals and grading Sonnet / `high` | `master`'s tasks | **on by default (`rounds = 2`, unchanged):** the author's decision (2026-10-17, about 20:05). Simulated readers are part of exegenius's intended architecture, and SR1 was inconclusive, not negative. SR1's preregistered conservative proposal (off by default) is therefore **not adopted** | one round on a 25-page wiki: ~19 M (SR1), so ~37 M per wiki at the default `rounds = 2` (an estimate) | weak: one bounded experiment (SR1, preregistered) | **unvalidated.** Coverage rises with the wiki (lift 0.74), but the baseline is an instructed refusal, so lift does not test prior knowledge. Deleting the key paragraphs lowered coverage by 0.147, against the 0.15 bar (row 3, inconclusive: neither shown useful nor shown useless). Stuck points occur on intact and degraded pages alike (148 against 142), so they are not shown to locate missing content. The repairs they drive are therefore unvalidated too | **kept on by default; effectiveness unvalidated** | one wiki, one round, one reader profile, one run per goal and start. The readers ran as general-purpose agents, with more tools than production's `exegenius-reader`, so SR1 was not an exact test of the deployed workflow |
| **Judging** (the instrument) | Opus, no effort set; three judges per page; pairwise (K3) above the planning ceiling | J22's page judge; the primary rubric | — | page judgement 0.46 M (J26), 0.43–0.54 M (J22, J24); primary ~4.8 M; pairwise ~5.7 M | strong for its purpose (K3 passed its preregistered criteria; K2) | not applicable (the instrument) | **calibrated, as an instrument** | J26 shows occasional single-judge outliers (2 of 24 cells), which three judges and medians absorb |
| **Automatic note scoring** (`ASSERTS`, a tool) | — | — | — | none | moderate (A1, A2, A3) | agreement with hand labels 0.885 (A1), 0.794 (A2), **0.758 (A3)**; it finds 45–56% of asserting notes | **not for primary measures**; hand labels stay primary | one labeller (this session) throughout |

**Cost of this final phase** (measured since the author's 17:38 UTC reading, cache read):
- W3 runs 38.4 M and J26 33.0 M;
- SR1 37.9 M;
- A3 runs 9.6 M and its two reviews 2.9 M;
- this session's orchestration ~83 M by 18:34 UTC.

**Implementation (2026-10-17; revised at about 20:10 UTC):**
- **The author approved decision 1** at about 19:45. It is implemented on `wiki-explanation-reader-sim-v8dqtr` (PR #14, not merged):
  - `audit_instructions` gives the audit task the A3 text;
  - README, the skill and the task agent's audit rule are updated;
  - a test pins the calibrated text.
- **Decision 2 is withdrawn.** The author first approved turning simulated readers off by default, then reconsidered (about 20:05): they are a core part of exegenius's design, and SR1's 0.147 against 0.15 is inconclusive, not evidence for removing them. The default change (in 7bc391f) is reverted on the same branch, so PR #14 now changes only the audit; `[simulation] rounds` stays 2.
- **Verified:**
  - fmt and clippy (`-D warnings`) are clean; 3 + 82 + 15 tests pass;
  - **`tools/a3verify.py`: the built binary writes A3's 30 audit tasks byte-identical to the ones A3 ran (project directory aside).** In the old page-only text none of the new paragraphs appear, so the comparison discriminates;
  - after the revert, the code differs from `master` only in the audit (`audit.rs`, `task.rs`), and the docs only in what the auditor reads.
- **Unchanged:** simulated readers (on, `rounds = 2`), writer routing (Sonnet `high`, with the Opus exception at 0015/0016), agent models, audit rounds and every other default.

**Decisions for the author, kept separate (none is bundled with another):**
1. **The audit (recommended: adopt R).**
   - The change: the audit task's text (SI + REFINE) and auditors reading `sources/paper.tex`. It is a CLI change, at ~3× the audit's cost per page.
   - What it buys: every planted error that only the paper reveals is caught (13/13 against 0/13), with page-error recall from 0.50 to 0.89.
   - What it costs: about one rewrite of a correct but condensed statement every two pages, by the independent reviewer's count.
   - If not adopted, the pipeline's audit cannot see source-only errors at all.
2. **Simulated readers: decided, kept on by default** (the author, about 20:05; SR1's proposal to turn them off is not adopted).
   - SR1 stays on record as inconclusive, with its limitations.
   - Effectiveness remains unvalidated: the evidence neither shows that the readers' coverage tracks the wiki, nor that their stuck points locate what is missing.
   - A better test would run production's `exegenius-reader` agent and use a baseline that measures what the reader model knows. It is not planned.
3. **Writing (no change proposed).**
   - Sonnet `high` stays the writer; the Opus exception at 0015/0016 stays, unless the author withdraws it.
   - An option, not a proposal: Opus on long, many-step key-step pages. W3 shows large gains on some such pages (0036) but none on others (0031, 0032), so the pages cannot be picked in advance on this evidence.
4. **The shared binary (operational, not a default).**
   - Rebuild `target/debug/exegenius` from `master`, so that runs use PR #13's planning instruction.
   - The main working copy at `/home/user/exegenius` is still on the pre-rewrite history. It needs `git fetch origin && git reset --hard origin/master` first, which this session's permission classifier blocked.
5. **The `ASSERTS` classifier:** no rule proposed. Its agreement with hand labels fell to 0.76 in A3.

**What is not validated, stated plainly:**
- **Every row rests on one paper.** No row is a general default without a second document, which only the author can supply.
- **Writing is not shown adequate on key-step pages by either model.** The configuration is the status quo because W3's rule says so, not because it passes.
- **Recall figures are on planted errors** in six pages, not proof against every real error.
- **Simulated readers' effectiveness is unvalidated.** They stay on by default by the author's decision, as part of the intended design. SR1 neither showed them useful nor showed them useless, so their coverage scores and the repairs their stuck points drive should be read as unvalidated signals.

## Calibration ledger (2026-10-17 08:43 UTC; audit row updated 17:16): superseded by the final configuration report above, kept as a record

**How to read it:**
- **"Best tested"** is the best configuration among those compared.
- **"Adequate"** means shown to meet the task's fidelity standard. Being best tested does not make a configuration adequate.
- **"Calibrated"** requires both a defensible configuration and evidence that it preserves the mathematics.
- **Scope:** every row rests on one paper (`paper.tex`). Production means `master` as built today (exegenius 0.6.0).

**Earlier tables:** the sections after this one are dated records, kept as written; this table supersedes them where they differ.

| Task | Recommended: model / effort / prompt | Production today | Routing | Cost (cache read, measured) | Best among tested | Adequate against the fidelity standard? | Evidence strength | Calibrated? | Unresolved, and what closes it |
|---|---|---|---|---|---|---|---|---|---|
| **Extraction** (`extract`) | Sonnet / `high` / `master`'s task | the same | none | small; a cheaper model would save ~1 M per wiki (EX1) | Sonnet `high` ≈ Opus `high` in depth, at lower cost; Sonnet `medium`, Opus `medium` and Haiku `high` are shallower (EX1) | **yes for its downstream use, indirectly:** every extraction tested fed plans judged 19/19 faithful against the paper (E9–E11) | moderate: 34 extractions, judge-free, plus downstream plans | **yes, for this paper** | a second document. No experiment on this paper |
| **Background** (`background`) | Sonnet / `high` / `master`'s task | the same | none | small (a short task) | not compared | **for planning only, indirectly:** the same ceiling plans. The concept lists vary between runs (median overlap 0.67) | weak to moderate | **for planning only** | its effect on writing is unmeasured. A model-free check against the reader profile comes first, if W3's pages show over- or under-explanation |
| **Argument planning** (`argument`, `extend`) | Sonnet / `high` / **the combined instruction** (PR #13, e4618b4, 14 lines) | **with** the combined instruction: PR #13 merged on 2026-10-17 (cd92445). The shared `target/debug` binary is rebuilt only after W3's runs | none | 3–5 M per run | Sonnet `high` with the instruction = Opus `xhigh` above the ceiling (E11, p = 0.69). Sonnet `medium` and Haiku `high` fall below (S6). Effort above `high` buys nothing (S2, S3) | **yes:** 19/19 faithful and 9/9 central on every extraction tested (E9, E10); three blind Opus judges, preregistered | **strong for this paper** | **yes, for this paper** | a second document |
| **Writing** (`write`) | Sonnet / `high` / `master`'s task; Opus / `xhigh` on 0015 and 0016 in this wiki | the same (the exception is the author's, this wiki only) | provisional: Opus `xhigh` on 0015 and 0016 only | Sonnet 0.34–4.06 M per page; Opus 1.92–8.13 M (2–3× Sonnet on hard pages) | Opus `xhigh` beats Sonnet `high` on the two hard key-step pages (W2b); they tie on 0014; Haiku `high` and Sonnet `medium` fall below (W2) | **Sonnet `high`: not shown adequate on hard key-step pages** (a major error in 1 of 2 runs on 0015). Opus `xhigh` met the standard on 0015 and 0016. Standard: the page judge's derived items faithful, no major error | weak to moderate: 3 pages | **no** | **W3, amended** (preregistered and ready; ~65–210 M in two meter-gated stages; `plans/W3.md`, Launch readiness). Its outcome matrix says which routing each result supports |
| **Fidelity audit** (`audit`) | **candidate, not recommended outright:** Sonnet / `high` / the SI text (FIDELITY + SOURCE, `tools/a2setup.py`), with the auditor reading `sources/paper.tex`; adopting it trades recall against false alarms (below) | Sonnet `high`, the page-only audit (B) | none | B 0.13 M per audit; **SI 0.42 M (3.2×)** | **SI.** On A2's planted errors: source-only 13/13 (B 0/13, I 2/13), page errors 26/28 (B 14/28), shams 0/18 for all | **SI does not meet the false-alarm criterion under an independent review.** A2's preregistered row 1 rested on a lenient adjudication: the independent blind reviewer (17:16, 0 of 4 decoys accepted) finds 3 of SI's 9 clean-page alarms genuine (F_A 1.00 against ≤ 0.5). Five of the six it rejects are borderline simplifications, not wrong mathematics. **B is not adequate either:** it catches no source-only error, and its page recall sits exactly at the bar (0.50). Recall here is on planted errors, not proof against every real error | moderate for the recall gap (large and consistent); moderate against SI on false alarms (two adjudicators, disagreeing on 6 of 15) | **no** | (1) **the author's weighing** of SI's trade-off: 13/13 source-only errors caught, against ~1 non-misstatement alarm per clean page and 3.2× cost; (2) a candidate fix, proposed and not run: a SOURCE text that reports simplifications only when they change what a claim states, tested in its own preregistered run |
| **Support check** (`support`) | no change | Sonnet `high`, `master`'s task | none | 0.14–0.24 M per task | no ranking (A1: same misses for all three models) | **no:** support recall 0.60 against 0.70 for every model (A1); both generalisation errors missed | moderate for "the task is the limit" | **no** | **if SI is adopted, its misses stop being the limiting gap:** SI's audit caught 4/4 generalisation errors. A1b only if SI is not adopted |
| **Simulated readers** | not set | `master` | none | not measured | not compared | not measured | none | **no** | after writing is settled; a small comparison whose decision is how much reader feedback to trust |
| **Judging** (the instrument) | Opus; the primary rubric below the ceiling, pairwise (K3) above it; J22's page judge for pages | — | — | 0.43–0.54 M per page judgement; ~4.8 M primary, ~5.7 M pairwise | K2: Sonnet agrees with Opus on the checklist (0.979) | K3 passed all four preregistered criteria | strong for its purpose | **yes, as an instrument** | Sonnet pairwise judging (a cost saving only) |
| **Automatic note scoring** (`ASSERTS`, a measurement tool) | **not for primary measures:** hand labels | — | — | none | — | agreement with hand labels 0.885 (A1), **0.794 (A2; 0.747 on SI's notes)**. It finds 45–49% of asserting notes | moderate (two experiments) | **no** | a general classifier rule is **not justified** by this evidence. Hand labels stay primary in audit experiments |

**Decisions for the author, kept separate (none is bundled with another):**
1. ~~**PR #13**, the planning instruction~~: **merged by the author's decision on 2026-10-17 (cd92445)**. As recorded before the merge:
   - Tests passed in isolation (3 + 79 + 15; clippy and fmt clean).
   - It is open, mergeable, with no reviews or comments. The repository has no CI, so the local tests are the only automated check.
   - Since the PR's base, `master` has changed only docs.
   - **Remaining concerns:**
     - the tests check the code, not the instruction's effect; the evidence for that is E9–E11, on one paper;
     - merging does not by itself touch any experiment, but the shared binary (`target/debug/exegenius`, which W3's runs call for `submit`) should not be rebuilt from the merged `master` while W3 runs.
2. **The audit change** (SI): **the independent review is done (R3).** SI's recall gain is real, but its false-alarm rate fails the criterion under the independent reviewer. Adopt it with that trade-off, or test a revised SOURCE text first: the author's call.
3. **The writer routing**: after W3.
4. **The classifier rule**: not proposed; A2's 0.79 agreement does not support one.

## Calibration status: the policy in force (11:08 UTC, 2026-10-15; rows updated 2026-10-17, 07:41 UTC)

**The bar (the author, about 11:00):** a task is calibrated only when it has both a defensible configuration and adequate evidence that it preserves the mathematics. A best-tested configuration is not enough on its own. Defaults change only with the author's approval. Every row rests on one paper.

| Task | Configuration in force | Configuration status | Evidence that it preserves the mathematics | Calibrated? | What closes it |
|---|---|---|---|---|---|
| Extraction | Sonnet `high`, `master` | final for this paper | indirect but sufficient for planning: every extraction tested led to ceiling plans | **yes, for this paper** | a second document |
| Background | Sonnet `high`, `master` | accepted: no alternative tested, and no sign it limits a decision | indirect for planning (the same ceiling plans); none for writing | **for planning only** | its effect on writing is unmeasured. Check against the reader profile, with no model, if pages point here |
| Argument planning | Sonnet `high` with the combined instruction. **`master` still has the older instruction** | final for this paper, once the instruction is merged | strong: the rubric's ceiling and the central gate, blind, on several extractions | **yes for this paper, after the merge** | the merge: **PR #13** (built and tested in isolation on 2026-10-17; not merged), then a second document |
| Writing | Sonnet `high`; Opus `xhigh` on 0015 and 0016 in this wiki | provisional | Sonnet failed on a hard key-step page in 1 of 2 runs; Opus met the standard on both hard pages | **no** | W3 as amended on 2026-10-17: a second Sonnet run per key-step page and fidelity rule R2 (`plans/W3.md`) |
| Fidelity auditing | as `master` (the writer configuration) | **a task change proposed (A2, 07:39 UTC, 2026-10-17), not adopted** | **A2 (one run, preregistered): the source-aware structured audit (SI, Sonnet `high`) meets the audit standard.** It caught 13/13 source-only errors and 26/28 page errors, with F_sham 0/18 and adjudicated F_A 0.17 per clean page. **The F_A pass is fragile:** 0.83 with borderline adjudications flipped (row 2). The current audit (B) reaches R_page 0.50, exactly at the bar, under hand labels, and catches 0 of 13 source-only errors. Best tested: SI; proven adequate: SI, at one run with a fragile false-alarm reading | **no** | the author's decision on the task change: the audit text, plus auditors reading `sources/paper.tex`, a CLI change, at ~3.2× the audit's cache read. Before adopting it, an independent blind adjudication of the 15 clean-page alarms (~3–6 M; `plans/A2.md`, Stage 2). If SI is adopted, A1b (the support instruction) is not needed: SI's audit caught the generalisation errors the support check misses (4/4) |
| Simulated readers | `master` | unmeasured | none | **no** | after writing is settled |
| Judging (the instrument) | Opus; the primary rubric below the ceiling, K3 pairwise above | validated | K3 passed its preregistered criteria | **yes, as an instrument** | — |

**Decisions for the author, kept separate:**
1. **The planning-instruction merge:** a default change, reviewed on its own (`proposals/argument-instruction.md`). It does not block any experiment.
2. **Measurement validity:** a general rule for when the `ASSERTS` classifier may score audit notes, for future experiments only (`plans/A1b-draft.md`). A1's preregistered result stays primary.
3. **Auditor evidence:** whether auditors get the source. **A2 (07:39) supports it:** SI is outcome row 1 (`plans/A2.md`). The adoption, a CLI change to the audit text and source access, is the author's. The recommended check before it is an independent blind adjudication of SI's clean-page alarms.
4. **W3:** launch on a meter reading, and whether to add a second Sonnet run per key-step page (`plans/W3.md`, option). Then the writer default.

## Gap review (2026-10-17, 07:04 UTC): what existing evidence closes, and what needs an experiment

**The rule (the author, 2026-10-17):** run only experiments that can change a configuration decision.

| Task | Closed by existing evidence | Genuinely needs an experiment | Not worth an experiment |
|---|---|---|---|
| Extraction | the configuration, for this paper (EX1; ceiling plans from every extraction) | nothing on this paper | a cheaper extractor: the saving is about 1 M per wiki. A second document is the only open item, and only the author can supply it |
| Background | the configuration for planning (the 2026-10-15 audit: 8 runs, all feeding ceiling plans) | nothing now. If W3's pages show over- or under-explanation, check the concept lists against the reader profile (no model) first | a model comparison: no sign it limits a decision |
| Argument planning | the configuration and the instruction, for this paper (E9–E11, S6). The instruction is now a tested, reviewable change (PR #13) | nothing on this paper | more model comparisons: E11 found no advantage above the ceiling |
| Writing | Sonnet `high` is not adequate on hard key-step pages (one major error in two runs on 0015); Opus met the standard there | **W3 (amended):** does Sonnet fail the fidelity standard often enough on key-step pages for routing to pay? | a 23-page Opus rollout without W3 |
| Fidelity auditing | no model choice fixes the current tasks (A1). **A2 (07:39): a source-aware structured audit does, at this scale** (row 1): source-only 13/13 against 0/13, page errors 26/28 against 14/28 | **only the false-alarm reading is open:** an independent blind adjudication of SI's 9 clean-page alarms (~3–6 M, judge-only), and a second SI run on the clean pages only if that leaves F_A near its bound | A1 stage 2; A1b, if SI is adopted; SI on Opus `xhigh` (Sonnet already meets the standard); a full second run of I and SI (the recall differences cannot plausibly reverse) |
| Simulated readers | nothing: unmeasured | a small comparison, once writing is settled; its decision is how much reader feedback to trust | anything before writing is settled |
| Judging | the instrument (K3; K2) | nothing decision-critical | Sonnet pairwise judging (a cost saving only) |

**The order the evidence supports:**
1. ~~**A2**~~: run and read at 07:39 on 2026-10-17 (row 1). What remains is the independent adjudication, which is cheap and judge-only, and the author's adoption decision.
2. **W3, amended:** the one writer decision still open.
3. **Simulated readers:** after writing.

A1b (the support instruction) runs only if A2 leaves the support check's misses as the limiting gap.

## Decision ledger (2026-10-15, 09:26 UTC): the configuration to use for each task, and how far it is proven

**The objective (the author, 09:24):** a defensible configuration for every pipeline task, not a finished queue of experiments. Each row separates two things:
- **best tested:** the best configuration among those compared;
- **proven adequate:** shown to meet the task's fidelity standard.

Nothing here changes a production default; the author decides those. Every row rests on one paper.

| Task | Recommended now: model, effort, prompt | Routing | Best tested | Proven adequate? (standard) | Evidence | Cost | Open, and the smallest work that would close it |
|---|---|---|---|---|---|---|---|
| **Extraction** (`extract`) | Sonnet `high`, `master`'s task (0.6.0) | none | Sonnet `high` ≈ Opus `high` in depth; Opus costs more. Sonnet `medium` skims (4 of 5 sent back); Haiku `high` and Opus `medium` are shallower (EX1) | **yes, for planning on this paper:** all six extractions tested give 19/19 faithful plans under the combined instruction (E9–E11). Standard: the coverage check passes, and plans reach the ceiling downstream | moderate: 34 extractions, judge-free, plus downstream plans | small; the possible saving from a cheaper model is about 1 M per wiki | generality only (second document). **No experiment** |
| **Background** (`background`) | Sonnet `high`, `master`'s task | none | **not compared** | **adequate for planning on this paper, indirectly:** the backgrounds of F2–F6 and F8 all fed 19/19 plans (E9–E11). 7 of 8 were accepted on the first submit. The known-concept lists vary between runs: a median 0.67 of one run's concepts are matched by name or alias in another (0.36–0.88). No standard of its own (audit of 09:30, `docs/journal/2026-10-15-background-audit.md`) | weak to moderate: 8 independent runs, judge-free, plus downstream plans | small (a short task) | its effect on writers (over- or under-explanation) is not measured. **No model experiment is justified now.** If writing quality points here, check the lists against the reader profile (no model) |
| **Argument planning** (`argument`; `extend` runs under the same configuration) | Sonnet `high` **with the combined instruction** (the E8 and E9 sentences; branch `141026-exp-params-cases`, e8efaa3). **`master` still has the E6/E7-era instructions** | none | Sonnet `high` + combined = Opus `xhigh` above the ceiling (E11: 2–2, 2 tied, p = 0.69; run variance dominates). Sonnet `medium` and Haiku `high` fall below (S6). Effort above `high` buys nothing (S2, S3) | **yes, on this paper:** 19/19 faithful and 9/9 central on every extraction tested (E9, E10). Standard: the primary rubric's ceiling and central gate, three Opus judges | **strong for this paper:** several extractions, preregistered, blind | 3–5 M cache read per run | (1) **the prompt is not on `master`**: putting the combined instruction there is a default change, which needs the author's approval; (2) generality (second document). **No further model comparison** |
| **Writing** (`write`) | Sonnet `high`, `master`'s task | **provisional, this wiki only:** Opus `xhigh` for 0015 and 0016 (the author, 01:06). The blind designation of 23 key-step pages is a classification, not a routing rule | Opus `xhigh` beats Sonnet `high` on the two hard key-step pages (W2b); on 0014 they tie; Haiku `high` and Sonnet `medium` fall below on the hard page (W2) | **Sonnet `high` is not proven adequate on hard key-step pages:** one of its two runs on 0015 had a major error in every judgement. Opus `xhigh` met the standard on 0015 and 0016 (no major error; 8/8 and 4/4 items). Standard: the page judge's derived items faithful, no major error | weak to moderate: 3 pages, 1 paper | Opus costs 2–5.6× Sonnet's cache read on hard pages (+1.4–4.2 M per page) | **W3** (preregistered, ready): does Opus routing generalise to key-step pages? It is the one queued experiment that can change a consequential decision. About 70–140 M |
| **Fidelity auditing** (`audit`, `support`) | no change: the tasks run on the writer configuration (they share `exegenius-task` with writers; a separate model needs a CLI change) | none | **no ranking supported** (A1 stage 1: one run each, overlapping intervals) | **no, for any model tested** (A1, preregistered): audit recall below 0.50 for all three (`ASSERTS`), and support recall 0.60 for all three. All three miss the same generalisation errors. The hand-label sensitivity analysis lifts audit recall above the bar for all three, but support recall still fails. Neither task compares a page with the paper, so source-only errors are out of reach by design | moderate for "the task is the limit": three models, identical support misses | 0.14–0.24 M per task | **the task, not the model:** (1) a small preregistered test of a support instruction that asks whether a claim is more general than its quotation, on A1's seeded pages, judge-free (~6–10 M per configuration); (2) **for future experiments only**, fix a classifier criterion on recall of hand-labelled positives (A1's stage-1 result stays as preregistered); (3) whether auditors get a source check is the author's design decision. **A1 stage 2 is not run** (see below) |
| Simulated readers | `master` | none | not compared | not measured | none | — | after writing is settled |
| Judging (the instrument) | Opus; three judges on the primary rubric; pairwise (K3) above the ceiling | the primary rubric below the ceiling, K3 above it | K2: Sonnet agrees with Opus on the checklist (0.979) | K3 passed all four preregistered criteria | strong for its purpose | about 4.8 M per primary judgement and 5.7 M per pairwise judgement | whether Sonnet can judge pairwise (would cut cost; not decision-critical) |

**Prompt changes the results already justify:** one.
- **The combined argument instruction** (E8 + E9), justified by E9 and E10. It is on `141026-exp-params-cases`, not `master`. Merging it changes a pipeline default, so it is the author's decision.
- No writer, auditor, extraction or background prompt change is yet supported by evidence. The support instruction above is a hypothesis to test, not a justified change.

**What is next, ranked by whether it can change a configuration decision:**
1. **W3**, the writer routing rule. It can change the writer row for key-step pages.
2. **The support-task instruction test** (`plans/A1b-draft.md`, drafted at 09:30; ~11–16 M). It can turn "the auditing task fails" into a fixable prompt.
3. ~~The background audit~~: done at 09:30. Keep Sonnet `high`; no model experiment is justified.
4. **A2, the source-aware structured audit** (`plans/A2-draft.md`, a design). It can say whether auditing is fixable by giving auditors the source. It needs 12 source-only seeds, written by hand, and an isolated binary before preregistration.

**A1 stage 2 is deferred,** with the author's direction at 09:24 ("Don't run it merely because a threshold was narrowly triggered"). It cannot change the auditing verdict: adequacy needs support recall ≥ 0.70, and all three models miss the same 4 of 10 errors. Its preregistered trigger stays on record, and its copies are kept.

**Update, 2026-10-17 07:39 UTC (A2; the ledger above is kept as written on 2026-10-15):**
- **Fidelity auditing now has a candidate configuration:** Sonnet `high` with the SI audit text, and the auditor reading `sources/paper.tex` (`tools/a2setup.py`, FIDELITY and SOURCE).
  - **Best tested:** SI.
  - **Proven adequate:** SI met the preregistered audit standard in one run, with R_src 13/13, R_page 26/28, F_sham 0/18 and adjudicated F_A 0.17.
  - **The false-alarm reading is fragile:** 0.83 per clean page with the borderline adjudications flipped.
  - **Cost:** ~0.42 M cache read per audit, against 0.13 M now.
- **A second justified prompt change is now proposed:** the SI audit text, by A2's preregistered outcome row 1. It needs a CLI change, both to the audit task's text and to its file restriction. Adopting it is the author's decision, and the recommended step before it is an independent blind adjudication of SI's clean-page alarms.
- **The support check (R_S 0.60) is not retested.** If SI is adopted, its misses stop being the limiting gap: SI's audit caught 4/4 generalisation errors. So A1b is not needed.
- **A finding beyond auditing:** the six production pages A1 treated as clean carry at least 8 distinct genuine misstatements, all caught by SI, that the current audit never raised. Most are dropped hypotheses (τ > 0, the row hypothesis, a smallness condition on η). The others are a dropped sign, a mistyped label, a non sequitur and an imprecise pivot rule. **The writer row's fidelity evidence (W2, W2b, W3) comes from source-reading Opus judges, not from the audit, so it is not affected.**

## The table (one paper, `paper.tex`, unless stated)

| Stage (task) | Current default | Best evidence so far | Status | What would settle it |
|---|---|---|---|---|
| Extraction (`extract`) | Sonnet `high` | **EX1 evidence analysis (judge-free, 34 extractions; `docs/journal/2026-10-14-x1-extraction-evidence/`):** Sonnet `medium` skims: 4 of 5 first attempts were sent back by the coverage check, and the accepted answers quote 11 results against 23.5 and reach 10 core items against 15. Opus `medium` and Haiku `high` are shallower; Opus `high` is as deep as Sonnet `high`, at higher cost. Downstream, the extraction explains no detectable share of plan quality (E6, F = 0.55), and under the combined instruction all six extractions tested give 19/19 plans | **supported: keep Sonnet `high`.** A planner-on-extraction experiment (`plans/EX1-draft.md`, about 71–80 M) is **not run**: the possible saving is about 1 M per wiki |
| Background (`background`) | Sonnet `high` | none | **unmeasured** | low priority: short task, small cost share |
| Argument planning (`argument`) | Sonnet `high`, E6/E7-era instructions on `master` (the key-step instruction is not on `master`) | E4–E10. **Instructions dominate.** With the E8+E9 sentences, Sonnet `high` scores 19/19 faithful and 9/9 central on F2, F4, F8 (E9). Under E7 instructions, Opus `xhigh` beats Sonnet `high` on F1, F4, F8. Effort above `high` is not worth it: Sonnet `xhigh` hit the output limit; Opus `max` ≈ Opus `xhigh` within run variance at 5× cost. Opus `medium` is no better than Sonnet `high` (E5). Fable `high` is below Opus `max` (one run) | **supported for this paper's argument task only (E10, 23:45); not a general pipeline default.** Sonnet `high` + combined instruction. Sonnet `medium` and Haiku `high` fall well below it (S6). **E11 (04:58 UTC, 2026-10-15): Opus `xhigh` is not shown better above the ceiling.** O was favoured on 2 extractions, S on 2, and 2 tied (sign test p = 0.69). A control of two Sonnet runs on one extraction differed more than any O–S pair, so run variance dominates above the ceiling (`plans/E11.md`) | (1) E10; (2) the cheapest configuration that stays at the ceiling (S6); (3) whether a stronger model is better *above* the ceiling, which needs a discriminating measure |
| Exposition (writers) | as `master` | **W2 (screen) and W2b (repeat, pooled; 00:25 UTC):** on the two hard key-step pages, Opus `xhigh` is better than Sonnet `high` by W2b's preregistered rule. Page 0015: Opus 8, 8, 8 and 8, 7, 8 of 8, no major error; Sonnet 7, 6, 5 and 5, 5, 5, one run with a major error in every judgement. Page 0016: 4, 4, 4 against 3, 3, 3 of 4. On the easier page 0014, both are adequate. Haiku `high` and Sonnet `medium` fall below on the hard page (W2). Opus costs 2.0–3.2× Sonnet's tokens on the hard pages (+1.4–4.2 M each) | **Adopted by the author (01:06 UTC, 2026-10-15) as a provisional, paper-specific exception:** Opus `xhigh` for key-step pages and Sonnet `high` for the others, **in this wiki only**, pending stronger evidence. **Sonnet `high` remains the general writer default.** The key-step rule and this wiki's designation are under "Writer exception" below | more hard pages and a second paper; simulated readers on the same pages |
| Fidelity auditing (auditors, support checks) | as `master` | **A1 stage 1 (05:17 UTC, 2026-10-15; `plans/A1.md`):** planted-error recall on 6 pages, one run each. As preregistered, **no configuration is adequate.** Audit recall R_A: Haiku `high` 0.46, Sonnet `high` 0.29, Opus `xhigh` 0.39, against 0.50. Support recall R_S: 0.60 for all three, against 0.70; all three miss the same errors (generalisation 0 of 2). There were no false alarms beyond the thresholds. **The audit verdict depends on the classifier:** the preregistered `ASSERTS` passed its 80% agreement check (0.885) but finds only 45% of the notes that assert an error. Under blind hand labels (a sensitivity analysis, not preregistered), all three meet R_A (0.64, 0.50, 0.71). The audit and support tasks still never compare a page with the paper (the A1 design) | **unmeasured enough to choose: no proposal.** Stage 2 is triggered (AH on R_A; all three on R_S, reading "within 0.10" inclusively) and waits on a fresh meter reading. Whether the hand labels replace `ASSERTS` is the author's decision (a criterion change after results) | stage 2; the author's ruling on the classifier; a support task that can see generalisation errors |
| Simulated readers | as `master` | none on model choice | **unmeasured** | after exposition |
| Judging (the instrument) | Opus, three judges on primary measures | K2: Sonnet agrees with Opus on the checklist (0.979). The primary rubric saturates (19/19). **K3 (00:15, 2026-10-15): blind pairwise judging, both orders, passes all four preregistered criteria**: 16 of 16 verdicts on known pairs correct, 0 reversals across orders or judges, item position share 0.50, 37 of 37 primary-separated items matched; it detects a single removed sub-step (4 of 4, `slight`) | **primary rubric below the ceiling; pairwise (K3) above it**, one Opus judge per order, about 5.7 M per judgement | whether Sonnet judges can replace Opus on pairwise (K3 (e)1), and the identical-pair control, which the judges see through |

## Writer exception for this wiki (the author's decision, 01:06 UTC, 2026-10-15)

**The scope:**
- **Key-step pages:** Opus `xhigh`.
- **Other pages:** Sonnet `high`.
- **Where it applies:** this wiki (`paper.tex`, E9F4Y's plan) only, pending stronger evidence. It is not a general default; Sonnet `high` remains the general writer default.

**Why:** Opus was consistently more faithful on both hard pages (W2b). Sonnet's major error on page 0015 matters more than the small score differences. Two hard pages from one paper do not establish that the split generalises.

**The key-step rule (the author's wording):** a page is key-step when it introduces or carries a central derivation, proof transition, or result whose omission or distortion would break the explanatory chain. The rule reads the argument's structure, not page numbers.

**The designation is recorded before the writer is chosen,** so the rule cannot be retrofitted to favourable results.

**The designation (blind, 01:25 UTC, 2026-10-15; `docs/journal/2026-10-15-keystep-designation/`).**
- **How it was made:** a fresh Opus reviewer applied the rule to all 35 pages, from the plan alone. It saw neither W2 nor W2b, and its transcript shows it read only its prompt and the plan packet. The author asked for this at about 01:22, so that the classification does not depend on the observed writer scores.
- **The result:** 23 pages are key-step: 0012–0019, 0021–0023 and 0027–0038.
- **Not key-step:** 0004–0011, 0020 and 0024–0026.
- **Borderline:** 0010 (marked no); 0018, 0019, 0021, 0022, 0028, 0034 and 0035 (marked yes).

**This governs the writer choice.** The author's earlier "0014 does not qualify" was made after the writer results were seen, so it is superseded and not counted as independent evidence. 0014 is key-step under the blind designation.

**What it implies (decision-relevant, reported to the author):**
- The rule, applied blind, covers 23 of 35 pages, not the two hard pages the evidence is about.
- On 0014, the one designated page W2 measured besides 0015 and 0016, Opus bought nothing measurable over Sonnet.
- Opus wrote 0014–0016 at 1.4–4.2 M more cache read per page than Sonnet (2–5.6×). Across 23 pages that is roughly 30–100 M more per wiki, a rough range in measured tokens, not converted to quota.
- Whether the rule should be narrower is the author's call. Nothing here runs a writer.

**The author's ruling (about 01:37 UTC):**
- The blind designation stays fixed. It is a *classification* result, not evidence that Opus is better on all 23 pages. W2b establishes the advantage on 0015 and 0016 only.
- **No 23-page Opus rollout.** Sonnet `high` remains the general default.
- After E11, and with budget, a small preregistered writer validation is proposed: `plans/W3.md` (preregistered 05:31 UTC, 2026-10-15; not launched), with 6 seeded key-step pages and 3 non-key-step controls. Its results decide whether the split generalises.

## Cost per argument run, measured (for the table's cost column)

| Configuration | Cache read per run | Time per run |
|---|---|---|
| Sonnet `high`, combined instruction | 3–5 M | 14–19 min |
| Sonnet `high`, E8 | 2.5–7.7 M | 12–22 min |
| Opus `xhigh`, E7 | 1.7–6.9 M | 15–23 min |
| Opus `max`, E7 | 15–16 M | 34–35 min |
| Sonnet `max`, E7 | 25 M | 46 min |

**Judging dominates experiment cost:**
- **Primary judgement:** about 4.8 M each (J18).
- **Checklist judgement:** about 1 M each.
- **This session's orchestration** is a further component: 708 M over 2026-10-14 to 15:51, mostly before a compaction (`budget.md` §4).

## Generality

Every row rests on one paper. No row becomes a general default without the second document (`RUNBOOK.md` item 7, which needs the author to supply a document).

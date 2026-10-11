# 2026-10-11: Finding and confirming aha moments

This entry implements the fifth research entry, 2026-10-11-aha-moments (proposals I-87, I-89 and I-95, and the leave-one-out ablation I-51 applied to sentences). It also builds in the review's points that the research entry took up: a key step, a payoff and a reconnection can be different sentences, and a deletion can hurt in three ways, to be told apart against a matched control.

## What was built

The research entry gave four ways to find candidates and one way to confirm them. All five are built, each kept apart in the report, since they find different things (exegenius-core/src/aha.rs).

### 1. Structure: sentences that join distant parts of the argument

A page sits under one main step of the argument: the root's child whose subtree presents the page, or, for a page that presents no step, the first step that relies on it. A sentence that links a page sitting under a different main step joins two distant parts of the argument, and is a candidate reconnection. The build splits each page body into sentences and lists such sentences in the report (`aha::joining`). This is the crude form the research entry proposed: it uses the main steps, not the depth of the lowest common ancestor.

### 2. Salience: the feature an explanation must reach (I-87)

An argument step gains an optional `salient` field: a feature of its claim a reader would find striking, such as a symmetry, a uniformity across cases, or a surprising value or bound. The argument instructions cite Lange's point that a proof explains only relative to such a feature, and say most steps have none. The writer of the step's page is told the feature and asked to explain why it holds from the method. The goals task lists it and asks for a `justification` goal about it. This answers the research entry's open question 1: the feature is recorded on the plan step.

### 3. Attribution: the sentences that carry key points (I-89)

The research entry had the grader name, for each covered key point, the reader citation that supports it. That would break the grade's blindness: readers cite pages and the no-wiki baseline cannot, so citations would mark which answer is the baseline. Attribution is therefore a separate task, `attribute`, issued after a round's grades for each goal with graded reader runs that covered key points. It shows each such answer, the key points it covers, and its quotations, and asks which quotation supports each covered key point. Submit rejects support for a key point the answer did not cover, a quotation that does not exist, and two quotations for one key point.

The report lists the sentences whose quotations carried most key points, over all runs, with the number of key points and of runs. Passages are compared as plain text, since readers quote links as they saw them, with opaque page handles in place of page ids.

### 4. Agreement: what readers say the key idea is (I-95)

A reader that answers also names the key idea of the page it relied on most: a sentence of its own and a quote from that page, checked against the pages it opened like any citation. The report lists each page's key ideas and whether two readers quoted the same passage. After a round, a page whose repair-goal readers named two or more key ideas, none quoting the same passage, gets a rewrite note asking the writer to say plainly what the page's key idea is. Key ideas from held-out goals feed the report only.

### 5. Confirmation: hiding a sentence, against a control (I-51)

With `[simulation] ablations = n`, one more round runs after the last. For each of the n sentences that carried most key points, the targets are:
- **the candidate**: the page's sentence containing the quoted passage;
- **a control**: the page's sentence closest in length to the candidate that no reader cited, between half and twice its length. A candidate with no such control is skipped, since its effect could not be told apart from the effect of removing any sentence.

For each target, the goals that the candidate supported are rerun, once per reader profile, from the deep start, with the target sentence removed from its page in everything the reader sees, search included. The runs are graded as usual.

The report then gives, for each hidden sentence and goal, the share of key points covered in the last regular round and in the ablation runs, overall and for reason and conclusion key points together ("relational"). Its "Key steps" section lists the candidates whose loss cost more relational key points, averaged over their goals, than their control's, in reading order. This is the significance view of 2026-10-11-fragmentation-and-collapse, kept in the report, which answers the research entry's open question 3 the cautious way.

## How the review's three outcomes map onto this

The review asked that a deletion that hurts, one that breaks a connection, and one that removes a payoff be told apart. The ablation table gives the overall share (hurts) and the relational share; the relational share joins reasons (connection) and conclusions (payoff). A per-role split would separate those two, and the role table already does it for regular runs; for ablations, the two are joined because a single ablation run has few key points of each role.

## Where the build departs from the research entry

1. **Attribution is a separate task,** as above, to keep grading blind.
2. **Structural candidates use main steps, not ancestor depth,** and need the argument plan.
3. **Sentence ablation, not paragraph,** answering open question 2. Sentences are what readers quote, and the control is matched on sentence length.
4. **Ablations rerun the goals the candidate supported, whether held out or not.** The research entry asked for held-out goals, to avoid confirming a sentence that a repair pasted in. Attribution comes mostly from the goals readers covered, which are not reliably held out; restricting to held-out goals would leave most candidates with no goal to rerun. The report shows which goals were rerun, and the goal sections show which are held out.
5. **Sentence markers on pages are not built** (open question 4). The secondary summary of Schneider et al. reports no signalling effect in mathematics, and nothing yet confirms any sentence.

## How it was tested

An end-to-end test with scripted answers runs a wiki through one round and the ablation round: a reader cites a sentence that carries a reason key point; the attribution task records it; the ablation round hides that sentence from one reader and a control sentence from another; the reader without the candidate gives up while the reader without the control answers; the report lists the candidate under "Key steps". Unit tests cover sentence splitting, passage matching across link markup, and the joining-sentence detector.

No demo run has exercised these tasks with agents yet.

## Current State
- `aha::to_attribute`, `attribute_task` and `apply_attribution` hand out and check the attribution task after each round's grades, storing (key point, citation) pairs on runs (exegenius-core/src/aha.rs:69, exegenius-core/src/aha.rs:78, exegenius-core/src/aha.rs:109)
- `aha::pivots` counts the key points each quoted sentence carried over all runs (exegenius-core/src/aha.rs:185)
- Reader answers may carry a key idea with a statement and a quote, checked against opened pages; `aha::key_ideas` groups them per page with an agreement flag, and `repair` queues a note for pages whose repair-goal readers disagree (exegenius-core/src/simulate.rs, exegenius-core/src/aha.rs:232, exegenius-core/src/grade.rs)
- `aha::joining` lists body sentences that link a page under another main step of the argument (exegenius-core/src/aha.rs:318)
- `argument::Node` has an optional `salient` feature, shown to the writer of the step's page and in the goals task (exegenius-core/src/argument.rs, exegenius-core/src/write.rs, exegenius-core/src/simulate.rs)
- With `simulation.ablations` above 0, `simulation_step` runs one ablation round after the last round from `aha::ablation_targets`, hiding each target sentence through `Lens::hiding` in reads, searches and submits (exegenius-core/src/task.rs, exegenius-core/src/aha.rs:360, exegenius-core/src/simulate.rs)
- The report lists sentences that carried key points, key ideas per page, joining sentences, an ablation table with overall and relational coverage before and after, and the confirmed key steps in reading order (exegenius-core/src/report.rs:156, exegenius-core/src/report.rs:250)

## Missing
- No run of the attribution, key-idea or ablation steps with agents
- No per-role split of ablation results
- No restriction of ablation reruns to held-out goals
- No markers on pivotal sentences in the published pages

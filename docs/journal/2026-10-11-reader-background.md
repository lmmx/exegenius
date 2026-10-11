# 2026-10-11: Reader background

The reduction pilot left two causes of divergence (2026-10-11-reduction-pilot): symbol gaps, and a graph that grew by more pages than each round audited. This entry records four changes:
- a limit that bounds growth whatever the agents judge;
- a task that fixes, once per run, what the reader is taken to know, where each audit and extend used to judge it afresh;
- two changes to symbol checks.

It also records the free checks run on the pilot's records, and what the next pilot has to show. Convergence is the aim. The reader's level matters here as far as a judgement that drifts between rounds feeds growth.

## The reader the agents simulated

The pilot's reader profile is "Undergraduate computer science: big-O notation, polynomials, complex numbers, basic Turing machines and tapes". Extraction marked as known the four concepts the profile names, and nothing else. Audits and extend then treated everything outside those four as unknown. Over two rounds, extend made concepts with pages of their own for:
- linear map, matrix product, operator norm and tensor product;
- modular arithmetic, coprime integers and the Chinese remainder theorem;
- the convolution theorem and the Gaussian function.

Most of these are taught in undergraduate linear algebra and discrete mathematics, though where the line falls for a given reader is a judgement. The paper uses them without explanation: "tensor" appears 39 times in `sources/paper.txt`, "matrix" 38 times, "modul" 34.

The audit instructions already say that the profile covers "what anyone with that background has been taught, whether or not the profile names it". Since 2026-10-11-graph-growth, extend has said so too. Neither worked. Every audit and every extend makes that judgement afresh, from a one-line profile with a list of examples. An agent that reads the list as complete counts linear maps as unknown, and nothing it sees afterwards corrects it. Each page on a background concept is then audited in its turn, and its own background becomes the next round's terms. The line moves down a little each round, and growth follows it.

Two fixes were ruled out:
- **A fixed level written into the configuration or the instructions** ("assume linear algebra"). It would suit this paper and be wrong for the next, and it puts the judgement in the wrong place.
- **A narrower prompt asking the agents to be more generous.** That is the change 2026-10-11-graph-growth already made, and it did not hold.

## The background task

The level depends on the sources as well as the profile. A paper uses without explanation what its own readers know, and the wiki's job is the difference between those readers and this one. A new task, `background`, runs once after extraction, before anything that depends on what the reader knows (exegenius-core/src/background.rs). It reads the sources for what they use without defining. For each, it decides whether a reader at the profile's level has been taught it, in the profile's subject or in the subjects that subject rests on. Its answer has three parts:
- **`summary`:** the subjects this reader has studied that bear on the sources, and how far into each. Every later task shows it after the profile, as "For these sources: …": write, audit, extend, argument, outline and goals. So do the configured reader's simulated readers.
- **`known`:** known concepts the sources rely on that the graph lacks, with aliases. They join the graph as known concepts, and code acts on them: an audit term naming one is dropped (exegenius-core/src/audit.rs), and extend is shown them as concepts to resolve to.
- **`covered`:** ids of existing concepts the reader knows, which become known and lose their pages. Submit rejects the target and the pilot's concept.

The instructions give the principle with an example from another field (an undergraduate degree in physics teaches linear algebra and Fourier series), not this paper's answers. They name both failure modes:
- withholding what the level teaches pads the wiki, and every page added needs pages of its own;
- counting what is taught only in specialist study, or what the sources introduce, as known removes what the wiki is for.

The task runs before the pilot's concept is chosen, since known concepts get no page and so change what fits in `pages`. A project whose graph predates the task gets one on its next `next`.

Code checks the ids and the shape of the answer, never what the profile covers. The judgement can still be wrong in either direction. It is now made once, where the whole source is in view, and then held fixed for every later task. Its result is visible in `exegenius status` ("reader, for these sources: …") and in `graph.json`, where the author can disagree with it, which no one could do with the scattered judgements of the audits.

## Three more changes

**Growth bounded by construction.** This is the change that bounds divergence, whatever the background says. Extend may add at most half as many new concepts the reader does not know as there were pages in the latest audit round, and at least one (`extract::new_pages`). Other terms resolve to existing or known concepts, or go under a new `inline` list. An inline term becomes a rewrite note: "Explain … in a sentence where it is first used." Each round then adds fewer pages than it audited, so a background judged too low cannot make the graph diverge. Under it, the pilot's first extend would have added at most 2 concepts, not 7.

**Late meanings.** A meaning counted as late if the symbol occurred in any formula before it. "The identity is restricted to coprime integers $s<t$, where $s$ is the number of coordinates of the source vector" therefore failed, and so does every "where" clause, the usual way mathematics introduces a symbol. Now a meaning is late only if a sentence or paragraph ends between the first use and the meaning, leaving mathematics out. The sentence after a displayed formula continues it (exegenius-core/src/symbols.rs). Writers' bindings and audits use the same rule.

**Letters a rewrite brings in.** Submit rejects a rewrite whose mathematics uses a letter that neither the previous version nor any binding in force or in the answer uses (`symbols::letters`, `symbols::introduced`). The writer binds the new symbols, with the sentence that says what each denotes, or makes the point without them. The check reads Latin letters and Greek control words, never meaning. It skips the arguments of `\mathrm`, `\text`, `\operatorname` and `\mathbb`, and the constants $e$ and $\pi$. It catches new formula symbols such as $X$ and $Y$ in a tensor identity. It misses a new combination of old letters ($F_t$ where $F_s$ and $t$ were already used) and operators such as $*$.

`status` now matches symbol gaps by symbol, not by the quoted sentence (exegenius-core/src/convergence.rs).

## Free checks on the pilot's records

The rules ran over the reduction pilot's task files (lmmx/openai-nlogn-wiki `pilot-v0.6-convergence`, 4b921b6) through the library's functions, at no model cost:
- **Late meanings.** Of the 17 symbol meanings the old rule found late, none is late under the new one. Each is a "where" clause or a definition in the sentence of first use, such as "Write $x = \sum_j a_j B^j$ with $0 \le a_j < B$, and call $a = (a_0, \dots, a_{d-1})$ …".
- **Letters.** 3 of the 13 rewrites would have been rejected:
  - `transform-identity`, for $X$ and $Y$, which the round 2 audit reported;
  - `radix-digit-convolution`, for $l$, the index of a new sum;
  - `operator-norm`, for $w$.
- **Status.** On the 5 repeated pages, symbol gaps went 12 → 19 with 13 new by symbol, the figure the last entry counted by hand.

These checks do not reach the reader's background, which needs a model to judge. Without the late meanings (5 in round 1, 11 in round 2) and the $X$ and $Y$ the letter rule would have stopped, step and symbol gaps on the repeated pages would have gone from 13 to 13, not 18 to 26. The stop rule would still have ended the run.

The remaining gaps are mostly steps, 6 in round 1 and 7 in round 2, all 7 new. They include:
- the sum of the powers of a root of unity over a full period;
- computing a convolution by transforms;
- the normalisation of a convolution identity.

Whether this reader knows the first two is a fair question, and the background answer may go either way. The renewal is the problem whichever way it goes. Rewrites answer a step note with reasoning that makes new claims, as in the 0.5.0 run on a smaller scale. None of the four changes acts on step renewal directly. On these records, the code-level changes alone do not bring the pilot under the stop rule's threshold.

## The next pilot

The protocol is that of 2026-10-11-reduction-pilot, with these changes (0.6.0 is not yet released, so the version is unchanged):
- `pages = 5`;
- the 0.5.0 run's extract answer reused as task 0001;
- the background task issued as 0002, with the pilot's concept chosen after it.

The run converges if:
- step and symbol gaps on the pages audited in both rounds fall to half or fewer;
- new gaps are a minority;
- the first extend adds fewer pages than it audited, without the limit rejecting an answer.

Beyond those measures, the background answer itself is worth reading, for what it counts as known in either direction. If it marks concepts on the pilot's path as known, the pilot's concept and pages will differ from the last run's, and the numbers compare less directly.

## Current State
- The background task runs once after extraction and before the pilot's concept is chosen. It gives a summary, known concepts and covered ids; submit rejects concepts already in the graph, unknown ids, the target and the pilot (exegenius-core/src/background.rs, exegenius-core/src/task.rs)
- The background summary follows the reader profile in the write, audit, extend, argument, outline and goals tasks, and in the configured reader's simulated reader tasks (exegenius-core/src/background.rs)
- Extend adds at most half as many new concepts the reader does not know as pages in the latest audit round, at least one; `inline` terms become rewrite notes (exegenius-core/src/extract.rs)
- A symbol's meaning is late only if a sentence or paragraph ends between its first use and the meaning, with the sentence after a displayed formula continuing it (exegenius-core/src/symbols.rs)
- Submit rejects a rewrite whose mathematics uses a letter absent from the previous version and from every binding in force or in the answer (exegenius-core/src/symbols.rs, exegenius-core/src/write.rs)
- `status` matches symbol gaps across rounds by symbol (exegenius-core/src/convergence.rs)
- On the reduction pilot's task files, the new late-meaning rule finds 0 of the 17 meanings the old rule found late, and the letter rule rejects 3 of 13 rewrites

## Missing
- No pilot run with the background task
- No check on a new combination of letters already used, or on operators such as $*$, in a rewrite
- No measure of whether a background answer overstates or understates what the reader knows
- No change acting on the renewal of step gaps by rewrites

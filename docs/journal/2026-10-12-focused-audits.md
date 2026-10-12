# 2026-10-12: Focused audits, and rewordings taken for new reasoning

Before a pilot of 2026-10-12-explain-first-level, the author and ChatGPT asked for a cheaper check: run the new audit instructions on three pages, separately from the generation loop, and read the outputs. Comparing those outputs with the full run's own audits showed that the auditor had not been the only place where a first-level step escaped. The depth limit had misclassified it as well.

## Three audits

Each task file was a round 2 audit task from `full-v0.6`, with the new instructions in place of the old. Each went to a fresh subagent that saw only its file.
- **`transform-identity`**, the final page: two steps.
  - The identity is never stated. "The citation stands in for how the page's own concept works", in the words of the audit instructions.
  - "A permutation followed by its inverse is the identity, so the formula can drop such a pair": the page never shows that a permutation is followed by its inverse, and the claim contradicts its own statement that both permutations stay in the formula.
- **`gaussian-resampling`**, the final page: one step. The page never says how the quadratic-exponent weights turn a length-$q$ transform into power-of-two transforms; "the identity is deliberately not given".
- **The 0.1 run's `gaussian-resampling`**, in the same place in the reading order, as a control for a page that gives the construction and leaves its proof to the source. The control marked its two deferrals ("with an error the paper bounds", "they can be left in the transform identity and cancelled later") as cited. It drew four steps and a term:
  - what the exact maps that $A$ and $B_0$ approximate are, and what the cited error bound states;
  - precision of what $p$ is, and what $\alpha$ controls;
  - what the adapted identity is;
  - how the permutations and maps combine to express each source transform;
  - the transform identity, neither explained nor linked.

None of the control's steps faults a citation as such. The auditor accepted the deferred error bound only in the sense that it asked what the bound states, which is what the new instructions require of a cited result. The control is a stricter test than the final pages, since it carries more claims. It shows that the 0.1 page would not pass either, and that the auditor reports what a cited result leaves unsaid without rejecting the citation.

## The full run's auditors had reported both steps

The full run's round 2 audits of the same final pages, under the old instructions, had reported a step on each:
- on `gaussian-resampling`, on "expresses each source": the page "does not show how the Gaussian-weighted move between the two sets of frequencies produces it";
- on `transform-identity`, on "so its transforms run without executing those permutations as sorts".

So the old auditor also saw these gaps, despite the citation exemption. What happened next was the depth limit.

## Rewordings counted as added reasoning

`audit::added` took every sentence of a rewrite that was not word for word in the previous version as added reasoning. On `gaussian-resampling`:
- round 1 reported a step on "through a power-of-two tensor transform", in the first draft's opening sentence;
- the rewrite split that sentence in two and explained nothing;
- round 2 reported the same step on the second half, "expresses each source";
- that half differed from every old sentence, so `added` listed it, and the step was sent back to be cited or cut, with "do not explain it further".

The page's central unexplained step was treated as a deeper one, to be cut or cited. The rounds stopped before the note was used. With a third round it would have locked the page's hollowness in.

**Free replay.** The full run's 21 steps sent back to be cited or cut were replayed through `added` against the write answers they came from (scratchpad tool, no model calls). Under the old rule the replay reproduces all 21. Seven are rewordings of the sentence a round 1 step was reported on:
- on `bluestein-chirp-identity`, only the position of "(cited)" changed;
- on `subspace-labels`, an "and" was dropped;
- on `linear-network-frames`, a clause was removed, which removed reasoning rather than adding it;
- on `address-interchange`, "of $u$" and a citation were added;
- on `rank-saving-recursion`, `gaussian-resampling` and `transform-identity`, the sentence was reworded or extended.

The depth limit measured textual novelty, not depth of reasoning. A sentence can change without a new step of reasoning, and a new step can arrive inside an old sentence.

**The fix.** A sentence now counts as a rewording, and not as added, when it shares at least three, and at least two thirds, of its words of four or more letters with one sentence of the previous version (exegenius-core/src/audit.rs `added`). On the replay this frees exactly the seven, and keeps the other 14 as cite-or-cut. A first threshold of one half also freed four sentences of new reasoning that reused old vocabulary (on `main-theorem`, `synthetic-transforms`, `discrete-fourier-transform` and `simultaneous-butterfly-layer`). The three-word floor keeps a short new sentence such as "One of $n$ and $n-1$ is even" from matching on one word.

**The converse.** ChatGPT raised a sentence that paraphrases an old one but slips in a new claim. Such a sentence counts as a rewording, so a step reported on it gets a full step note asking for the reasoning, not cite-or-cut. The claim is not exempted from scrutiny; what it loses is the depth limit's protection against regress. The test pins both cases (`added_sentences_are_those_not_in_the_previous_version`).

## Three places a first-level step escaped

As ChatGPT summarised it, the same pipeline had three failure points:
- **the writer**, which could replace reasoning with a citation, as the step note invited;
- **the auditor**, which was told that a citation made a step followable;
- **the depth-limit classifier**, which could file a necessary first-level step as deferrable because its wording changed.

2026-10-12-explain-first-level closed the first two. This entry closes the third, as far as word overlap can tell a rewording from new reasoning.

## Current State
- Under the new instructions, auditors reported the missing identity on the full run's `transform-identity` and the missing construction on its `gaussian-resampling`, and reported what the 0.1 run's `gaussian-resampling` leaves unsaid about its cited error bound without rejecting the citation
- The full run's round 2 auditors, under the old instructions, had reported a step on each of those two final pages
- 7 of the full run's 21 steps sent back to be cited or cut were reported on rewordings of a sentence round 1 had reported, among them the central steps of `gaussian-resampling` and `transform-identity`
- A sentence sharing at least three, and at least two thirds, of its words of four or more letters with a sentence of the previous version is not counted as added reasoning (exegenius-core/src/audit.rs)

## Missing
- No pilot with these changes
- No check that word overlap separates rewordings from new reasoning beyond the full run's 21 cases
- The focused audits ran on one model and once each, so their spread is unknown

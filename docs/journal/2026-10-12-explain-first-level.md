# 2026-10-12: The first level is explained

2026-10-12-full-run-review found that steps on the reduction's central pages were answered by citing the paper, and that a rename after the audits broke a page that took its letters from the renamed one. The author put the review to ChatGPT. Its advice, which this entry follows:
- change what a step note asks of the writer and what an audit accepts;
- route an overload note to the page the verdict names;
- check pages that depend on a rename;
- leave the rewrite budget as it is for the first experiment, so that a pilot can tell which change mattered.

## Citations no longer clear steps

**Audits.** The audit instructions told the auditor that a claim marked as cited "is followable by reference, so do not report it as a step". Any step could therefore be cleared by a citation, by the writer of a first draft or of a rewrite. The instructions now say:
- a citation lets the reader check a result, not follow an argument;
- a cited result the page uses is followable if the page says what it states and why it applies where it is used, and its proof can be left to the source;
- a cited claim that stands in for how the page's own concept works, or whose content or role the page does not say, is a step;
- a claim about an identity, construction or definition that the page never states is a step (exegenius-core/src/audit.rs).

**Step notes.** A step note no longer offers a citation as an alternative to reasoning, and no longer says to choose whichever needs fewer words. It asks for the reasoning on the page. If the step rests on a result from prior work, it asks the writer to say what the result states and why it applies, and to cite the source for its proof.

**Write instructions.** The write instructions say the same, and add that a page whose concept is an identity, construction or definition states it (exegenius-core/src/write.rs).

**How far this goes.** The distinction between a page's own mechanism and a result it uses is one the auditor and writer make; code cannot check it. A pilot, not a test, shows whether they make it. `gaussian-resampling` tests the harder case: an imported construction that is central to the argument. The page should give enough of Harvey and van der Hoeven's construction to show why the reduction works, and cite them for the full derivation.

Steps inside reasoning a rewrite added are still sent back to be cited or cut. At that depth, a citation of a result whose content and role the sentence states is followable under the new audit rule, so a cite-or-cut note can still be met with a citation.

## Renames keep what later pages rely on

A rewrite is rejected if it no longer binds a symbol that a later page, by its latest audit, takes from this page. The rejection names each symbol and the page that uses it. It tells the writer to keep the letter and, if a note asked for another, to say on the page that the letter means something else on the earlier page (exegenius-core/src/audit.rs `relied_on`, exegenius-core/src/write.rs). In the full run this would have rejected write 0174's renaming of $a$ and $b$ on `complex-phase-network`, which `simultaneous-butterfly-layer` took from it.

The check catches a symbol dropped, not one kept with a changed meaning. Nor does it catch two different symbols with clashing letters, such as $\beta$ and $\beta_D$.

## Overload verdicts name the page that changes

The full run's verdict on $a$ said that `gaussian-dyadics`, the earlier page, should change, and the answer had no way to say so. A verdict now has an optional `change`:
- left out, the note goes to the later page, as before;
- set to the earlier page, the note goes there, worded from that page's side, as long as the earlier page binds the letter on the page and not in a concept's name;
- any other page is rejected.

`overloads.json` records the page each note went to (exegenius-core/src/notation.rs). The instructions tell the judge to name the earlier page when its use of the letter is the odd one out, for instance when the sources use the letter as the later page does. A page that other pages take the letter from keeps it, by the check above.

## Tests

- `rewrites_keep_symbols_later_pages_rely_on`: a rewrite of the DFT page that drops $x$, which the chirp page takes from it, is rejected, and one that keeps it is accepted.
- `overload_verdicts_can_change_the_earlier_page`: a verdict naming a page outside the candidate is rejected; one naming the earlier page queues its note there and is recorded.
- `symbols_later_pages_take_from_a_page`: `relied_on` finds the symbols other pages' latest audits anchor to a page.

No test covers the audit and step-note changes, since they change what a model is asked to judge.

## Current State
- Audits count a cited claim as followable only when it is a result the page uses and the page says what it states and why it applies; a cited claim in place of the page's own mechanism, or a claim about an identity, construction or definition the page never states, is a step (exegenius-core/src/audit.rs)
- Step notes ask for the reasoning on the page, with prior results stated and cited for their proofs (exegenius-core/src/audit.rs, exegenius-core/src/write.rs)
- A rewrite that drops a symbol a later page takes from it, by that page's latest audit, is rejected (exegenius-core/src/write.rs)
- An overload verdict may send its note to the earlier page when that page binds the letter on the page, and `overloads.json` records where each note went (exegenius-core/src/notation.rs)
- The rewrite budget is unchanged

## Missing
- No pilot with these changes
- No check of a symbol kept with a changed meaning, or of clashes between different symbols such as $\beta$ and $\beta_D$
- No test of what auditors and writers do with cited claims

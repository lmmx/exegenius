# 2026-10-12: Rewrites after the audits stay narrow

2026-10-12-first-level-pilot traced its one false claim to write 0051, a rewrite of `gaussian-resampling` after the last audit round. Its notes were six overload notes asking for other letters. ChatGPT, reviewing the pilot, sorted its findings into four kinds, each with its own remedy:
- **explanatory gaps:** more audit rounds help;
- **source-fidelity defects:** the source text needs fixing;
- **rewrite regressions:** a rewrite needs a check;
- **audit findings of little value**, such as letters reused locally: the audit needs to discriminate.

It put the source text and the rewrite regression first. This entry covers the rewrite regression. The source text is the author's to convert.

## What write 0051 did

Replayed through `audit::added` against the version before it, write 0051 added eight sentences that are not rewordings. Two use a letter it newly bound ($h$, $R$). The other six make new claims, and its notes asked for none of them:
- "Let $I$ be the identity map on length-$s$ vectors, so $N-I$ collects the remaining terms."
- "Write $E=I-N$ for the error, so $N=I-E$."
- "The argument also needs $\alpha^2<p$, which the range $2\le\alpha<p$ does not give by itself." This is the false claim.
- "Hence $N$ is invertible."
- A rederivation of $P_sF_s = DN^{-1}C\,P_tF_tS$.
- A sentence linking the result to `transform-identity`.

Some are improvements, but no audit read any of them.

## The rule

A rewrite whose notes are all overload notes or support notes comes after the audit rounds, and no audit reads it. Each sentence it adds, beyond rewordings of old ones, must do one of three things:
- use a letter one of its notes names;
- use a letter the rewrite newly binds, as a renamed letter does;
- carry a citation mark, as a support fix may.

Otherwise the write is rejected, with the sentences listed and "Keep the rest of the page as it was" (exegenius-core/src/write.rs `narrow`). Overload notes are recognised by their opening "$x$ means" (exegenius-core/src/notation.rs `noted_letter`). Support notes are recognised by a shared constant (exegenius-core/src/support.rs `SUPPORT_NOTE`).

On write 0051, the rule rejects the six claims and accepts the two sentences with new letters. Rewrites from audit notes and from simulated readers are unaffected: the first are audited in a later round or dropped, and the second are meant to change what readers got stuck on.

A rewrite can still change the claim of a sentence it rewords, since a rewording is judged by shared words. The rule bounds what is added, not every edit.

## Task counts

`exegenius status` prints "tasks handed out: N (A accepted, P pending)", and the audit round while the rounds run (exegenius-cli/src/main.rs). The number a run will take is not known in advance, since each round's rewrites depend on its audits. The pilot took 59 tasks for 7 pages with three rounds.

## Current State
- A rewrite whose notes all concern letters or unsupported citations is rejected if it adds a sentence, beyond rewordings, with no noted or newly bound letter and no citation mark (exegenius-core/src/write.rs)
- Replayed on the first-level pilot, the rule rejects six of write 0051's eight added sentences, including the false claim about $\alpha$
- `exegenius status` prints tasks handed out, accepted and pending (exegenius-cli/src/main.rs)

## Missing
- No source text for the n log n paper with its mathematics intact
- No limit on a changed claim inside a reworded sentence in a rewrite after the audit rounds
- No distinction between local letters and letters other pages rely on in overload candidates

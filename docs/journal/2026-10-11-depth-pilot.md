# 2026-10-11: Depth pilot

The author ran the pilot of 2026-10-11-depth-limit and pushed it to lmmx/openai-nlogn-wiki as `pilot-v0.6-depth` (7839ae2). The protocol was that of 2026-10-11-reduction-pilot, with these changes: the background task, the growth limit, the late-meaning and letter rules, and the depth limit (exegenius at 700a39c). The run converged. This entry records what converged, what drove it as far as the records show, and what the loop did not catch.

## The run

The session took 12 minutes 43 seconds over 40 tasks, with the 0.5.0 run's extract answer reused as task 0001, and ran to `done`. From `exegenius status`:

```
pilot: prime-source-lengths (Prime source lengths), the part of the target this run explains
concepts: 61 (32 known to the reader); pages: 5 written of 5 planned
audit round 1: 10 gaps over 5 pages (per page: term 0.6, step 0.4, symbol 1.0)
audit round 2: 3 gaps over 5 pages (per page: term 0.4, step 0.2, symbol 0.0)
  on the 5 pages also audited in round 1: term 3 -> 2 (1 new), step 2 -> 1 (1 new), symbol 5 -> 0 (0 new)
```

Against the three measures of 2026-10-11-baseline-pilot:
- **Step and symbol gaps halved.** On the repeated pages they went 7 → 1, so the stop rule did not fire.
- **New gaps a minority:** not met in form, since 2 of the 3 remaining gaps are new. The counts are too small for the share to mean much.
- **The extends added no pages.** Of the three terms, two went to `inline` and one to an existing concept. The growth limit, 2, never bound.

The pages have 243 to 410 words of prose, median 286; the longest is a rewrite within its budget.

## What drove it

**The background task.** It added 25 known concepts and marked 3 existing ones known: `discrete-fourier-transform`, `cyclic-convolution` and `carry-propagation`. Its summary describes an undergraduate degree in computer science:
- linear algebra;
- discrete mathematics, including the Chinese remainder theorem;
- algorithms, including Karatsuba multiplication and the DFT with cyclic convolution as used for FFT polynomial multiplication;
- complex numbers as far as roots of unity and Gaussian integers.

Tensor products and quotient rings are excluded.

With the DFT and cyclic convolution known, `[pilot] pages = 5` chose `prime-source-lengths`, not the last run's `transform-identity`. Its pages are exact integer multiplication, radix-digit convolution, Gaussian resampling, the Chinese-remainder map and prime source lengths: the paper's own constructions, with no page on background. Round 1 reported 3 term gaps, against 9 in the reduction pilot.

**The other changes rarely fired:**
- no gap was dropped as known;
- the depth limit left no step;
- the growth limit never bound;
- one late meaning was reported in round 1, and none in round 2.

The convergence therefore comes from a wider reader and a different set of pages. These records cannot separate the two, or either from sampling.

**Whether the background is right is open.** Gaussian integers are beyond most undergraduate computer science. Roots of unity and FFT convolution are taught in some algorithms courses and not in others. A wider background shrinks the wiki and the audits' findings together, so convergence alone does not show that the background is right.

## What the loop did not catch

The final version of `gaussian-resampling` is wrong.
- **What it says:** the "Gaussian weights" are the chirps $e^{\pi i u^2/L}$, and Gaussian resampling is a convolution with the kernel $e^{-\pi i (u-v)^2/L}$.
- **What it describes instead:** that is Bluestein's chirp identity, which the paper uses in a different step (its Lemma 21).
- **What Gaussian resampling actually is:** the Harvey–van der Hoeven reduction resamples between grids with real Gaussian kernels, as the reduction pilot's page set out with its maps $S$ and $T$.

The error came in at write 0025, the rewrite after audit round 2. Round 2 had reported that the page names Gaussian weights without saying what they are. Rewrites queued by the last audit round are written but not audited, and audits read only the page, never the sources. The support check judges each citation's quotation against its claim. This rewrite added no citation, so nothing compared the new definition with the paper.

`exegenius status` reported "state: in progress" after `next` printed `done`. Two overload candidates remained after the last overloads round, which `next` no longer hands out, and `status` still counted them as pending work. It now does not (exegenius-cli/src/main.rs).

## Current State
- The depth pilot ran 40 tasks in 12 minutes 43 seconds to `done`; step and symbol gaps on its 5 repeated pages went from 7 to 1, and its extends added no pages
- The depth pilot's background answer marked 32 concepts known, among them the discrete Fourier transform, cyclic convolution, roots of unity, Gaussian integers and the Chinese remainder theorem, and its pilot concept was `prime-source-lengths`
- In the depth pilot, no gap was dropped as known, the depth limit left no step, and the growth limit did not bind
- The depth pilot's final `gaussian-resampling` page describes Bluestein's chirp identity as Gaussian resampling, introduced by a rewrite after the last audit round
- Rewrites queued by the last audit round are written without an audit, and no check compares an uncited claim with the sources (exegenius-core/src/task.rs, exegenius-core/src/support.rs)
- `exegenius status` reports the run done when overload candidates remain only after the last overloads round (exegenius-cli/src/main.rs)

## Missing
- No repeat of the depth pilot, so the spread between runs under one protocol is unknown
- No check of a background answer against the reader the author intended
- No check of claims a rewrite adds without a citation

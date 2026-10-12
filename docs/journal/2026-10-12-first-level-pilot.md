# 2026-10-12: First-level pilot

The author ran the pilot of 2026-10-12-explain-first-level and 2026-10-12-focused-audits, at exegenius b126a8d, and pushed it to lmmx/openai-nlogn-wiki as `pilot-v0.6-first-level` (6698d4c).
- **Setup:** reset from `full-v0.6`, with the 0.5.0 extract answer reused, `rounds = 3` and `[pilot] pages = 5`. Only the `[pilot]` lines differ from the full run's `exegenius.toml`.
- **The run:** 59 tasks in 26 minutes, ending in `done`. The author confirmed the reader at the check.
- **The pilot:** `transform-identity` again, so the pages compare with the reader check pilot and the full run.

## The run

```
concepts: 48 (15 known to the reader); pages: 7 written of 7 planned
audit round 1: 24 gaps over 5 pages (per page: term 1.2, step 2.6, symbol 1.0)
audit round 2: 23 gaps over 7 pages (per page: term 0.7, step 1.7, symbol 0.9)
  on the 5 pages also audited in round 1: term 6 -> 4 (4 new), step 13 -> 10 (10 new), symbol 5 -> 2 (1 new)
audit round 3: 8 gaps over 5 pages (per page: term 0.2, step 1.0, symbol 0.4)
  on the 5 pages also audited in round 2: term 5 -> 1 (1 new), step 12 -> 5 (5 new), symbol 6 -> 2 (2 new)
steps inside reasoning a rewrite added for an earlier step, sent back to be cited or cut: 12
citation support: 14 marks judged: 3 supported, 9 partial, 2 unsupported, 0 contradicted
symbols: 68 bound on pages; letters with two meanings: 17 confirmed, 5 left unjudged after the last overloads round
```

**Stricter audits found more.** Round 1 reported 2.6 steps per page, against 1.2 in the reader check pilot on the same first five pages.

**Round 3 ran, and step and symbol gaps fell.**
- After round 2, all 10 step gaps on the repeated pages were steps sent back to be cited or cut. The stop rule left them out (2026-10-12-full-run-stop), and a third round ran.
- After round 3, step and symbol gaps on the repeated pages went from 18 to 7.
- Two of the 12 steps sent back to be cited or cut came in round 3, on `operator-norm`.

## The two pages, against the paper

**`transform-identity`** has 404 words, against 158 in the full run.
- It states the identity, $P_sF_s = 2^{2\alpha^2} B_0 P_t F_t A$, citing Lemma 19, and derives it:
  - the Gaussian maps $S$ and $T$ and the Gaussian identity $TP_sF_s = P_tF_tS$;
  - $N = CTD$ and the inverse estimate cited to prior work;
  - the Neumann series;
  - $S' = S/2$, $J' = N^{-1}/2$, $D' = 2^{-(2\alpha^2-2)}D$;
  - the left inverse $DN^{-1}C$ of $T$.
- It shows that the scalar factors multiply to $2^{-2\alpha^2}$.
- Each formula agrees with the paper's proof of Lemma 19 (sources/paper.txt, lines 3395 to 3470).

**`gaussian-resampling`** has 626 words, against 204.
- It defines:
  - $P_s$ and $P_t$;
  - $S$ and $T$, with their Gaussian weights;
  - $\ell_j$ and $\beta_j$, with the worked case $s = 2$, $t = 3$;
  - $C$, $D$ and the expansion of $(Nu)_j$.
- It shows why the $j$-th summand of $(Nu)_j$ is $u_j$.
- It cites Harvey and van der Hoeven for the Gaussian identity and the inverse estimate, names Poisson summation as the reason the identity holds, and proves the Neumann series converges by telescoping.
- It agrees with the paper, with one exception, below.

These are the pages 2026-10-12-full-run-review found hollow, and the construction the 0.1 run's page only gestured at. The prior work is cited for the Gaussian identity and the inverse estimate, which the page states and applies, as the new audit rule asks.

## Defects

**A claim that the paper's hypotheses fall short.**
- **The claim:** the final `gaussian-resampling` says "The argument also needs $\alpha^2<p$, which the range $2\le\alpha<p$ does not give by itself."
- **What the paper says:** Lemma 19 requires $2 \le \alpha < \sqrt{p}$.
- **Where it went wrong:** the source's text conversion put the "√" on a line of its own, so the text reads "2 ≤ α < p".
  - The round 3 auditor reported the gap faithfully.
  - Write 0051 then wrote the sentence above, in a rewrite after the last audit whose notes asked only for other letters.
- **How widespread:** `sources/paper.txt` has 30 lines holding only "√", and it flattens superscripts throughout ("2p" for $2^p$, "α4" for $\alpha^4$). The writers reconstructed most of them; this one they could not.
- **Two failures in one:** the defect is a lossy source, and an unaudited rewrite that changed a claim it was not asked to touch.

**Overload notes about local letters.**
- **The count:** 17 letters were confirmed as having two meanings on 7 pages.
- **One page:** write 0051 had six overload notes, for $b$ as a vector, $i$ as a summation index, $M$ as the last index of a partial sum, and $A$.
- **The effect on the pages:** sentences such as "so they are not the output and input indices of the DFT page", "Here $r$ is not the shifted index of the DFT page" and "not the first integer of the radix-digit page".

**Repeated derivation.** Both pages derive $P_sF_s = DN^{-1}C\,P_tF_tS$ from the Gaussian identity by the left inverse, and both explain the Neumann series.

**The reader has no calculus.** The background summary lists discrete mathematics, linear algebra and complex numbers, and no calculus. The round 3 auditor of `gaussian-resampling` reported limits and absolute convergence as unknown: "The reader has no calculus or analysis in the profile."

**Citations.** 2 of 14 marks are unsupported and 9 partial.

## Current State
- The first-level pilot ran 59 tasks over 7 pages in 26 minutes with three audit rounds, ending with step and symbol gaps on its repeated pages at 18 -> 7 after round 3
- The pilot's `transform-identity` states and derives Lemma 19's identity, and its `gaussian-resampling` gives the construction of $S$, $T$, $C$, $D$ and $N$, both in agreement with the paper apart from one claim
- The pilot's `gaussian-resampling` says the paper's range for $\alpha$ does not give $\alpha^2 < p$; the paper's range is $\alpha < \sqrt p$, and `sources/paper.txt` lost the square root in conversion
- That claim came from a rewrite after the last audit whose notes asked only for other letters
- `sources/paper.txt` has 30 lines holding only "√" and writes superscripts inline
- Overload verdicts in the pilot confirmed 17 letters with two meanings on 7 pages, among them summation and partial-sum indices
- `exegenius status` prints the number of tasks handed out and the audit round (exegenius-cli/src/main.rs)

## Missing
- No check that a rewrite after the audit rounds changes only what its notes ask
- No source text with the paper's mathematics intact
- No distinction between local letters such as summation indices and letters a later page relies on, in what counts as a letter with two meanings
- No forecast of the number of tasks a run will take

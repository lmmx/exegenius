# Reference list for the main-theorem plan: notes (draft, unreviewed)

## How the paper was read

I read all of sources/paper.tex (5,361 lines) with the Read tool in chunks of at most 400 lines: 1-400, 401-800, 801-1200, 1201-1600, 1601-2000, 2001-2400, 2401-2800, 2801-3200, 3201-3600, 3601-4000, 4001-4400, 4401-4800, 4801-5200, 5201-5361. No file in that repository was modified. I took every anchor from a body line where the item is developed. The one exception is `conjecture-negation`, which the paper discusses only in the introduction (lines 379-387). A script (gen/build.py in my session scratchpad) checks each anchor in three ways: it is a substring of its stated line, `grep -c -F` returns 1, and it occurs once in the text with lines joined by spaces. All 73 anchors pass. Each item's `anchors` entries record `grep_count` and `unique`.

## The chain of the main proof

1. Finite networks (section 3). A fixed scalar network exchanges two banks with signs and restores arbitrary scratch values (lem:scalar-motifs). Frames on vertices commute with pointwise gates, so only the endpoint frames matter (eq:common-frame-identity).
2. Nested subspace labels from triple indicator vectors lose dimension only on central edges, so L < N/2 (lem:motif-residuals). This gives s < Wm for both networks (prop:bit-motif-interface, prop:complex-motif-interface) and s/W < m^tau with tau = sigma = 1 - 2^-50.
3. Address interchange (section 4). A rank-a shear costs a smaller interchanges (lem:lower-lower, lem:matrix-shear). Rows are split among the W roles, so F_k <= (s/W) F_{k-1} + O(1) and the cost is O(V e^tau) (prop:power-interchange). This extends to any width u as O(V u^tau) (lem:chunk-swap).
4. Butterfly layers (section 5). Phase frames reduce C^{(x)e} to s_c children C^{(x)e/m}, plus binary basis changes. These are done by eight packed rotations with a deterministic repair of a 80(f-1)2^-K fraction of addresses (lem:packed-selected-bit-rectangle). This gives F(e) <= (s/W) F(e/m) + O((eK)^tau + 1) and so O(V d^lambda'). The identity H_0^{(x)D} = b^D S_D C^{(x)D} S_D and exact dyadic guards Delta = o(p) give the normalized layer with one truncation (prop:simultaneous-layer).
5. Synthetic transforms over C[y]/(y^r+1) are stored in bit-reversed order behind a layout L. They cost O(Tp[dK + pK^(tau-1) + l d^lambda']) (lem:synthetic-transform-cost). With signed Kronecker packing through the old O(N log N) multiplier (lem:signed-ring-product), they give the normalized ring convolution in its retained order (prop:synthetic-convolution).
6. Reduction to that convolution (sections 8-9). The digits go onto distinct prime axes by the CRT isomorphism Phi, done by triangular controlled shifts. Gaussian resampling P_s F_s = 2^(2 alpha^2) B_0 P_t F_t A keeps both permutations (lem:no-sort-resampling, lem:tensor-resampling). The Bluestein chirp turns Q F_t into a cyclic convolution (lem:chirp-permutation). A phase twist makes the last axis negacyclic. The source permutation R cancels: (R F_s^+)[(R F_s u).(R F_s v)] = (u*v)/S^2.
7. Parameters (eq:fixed-parameters, eq:sizes) and primes s_i in ((1-2eta)t_i, (1-eta)t_i] make every hypothesis hold. The error chain gives a final error < 448 b 2^(-3b/4) < 1/2 (eq:final-error), so rounding and carries give the exact product.
8. The cost table (tab:costs) has seven powers of p with margins g_i. The smallest is g_2 = epsilon c (1-tau) = 2^-181 = 2 kappa, so the time is O(Tp p^(1-kappa)) = O(n (lg n)^(1-kappa)). This bound disproves the conjecture in this model because it forces T_A(n)/(n lg n) -> 0 (intro, lines 379-387).

## Granularity decisions

- The list has 26 in-scope items (19 core, 7 supporting) and 5 out-of-scope items. That is above the suggested 8-20. I think the larger number is honest: the proof has two independent tape procedures, each with its own network, interface and recurrence, plus a full reduction chain, and merging further would hide steps a plan must get right. A scorer who wants about 20 can drop or merge `stream-primitives`, `phase-twist`, `signed-packing`, `parameters-and-primes`, `arbitrary-width-interchange` and `conjecture-negation`.
- Merges: I folded the guard-width subsection into `simultaneous-layer`, because the proposition states the widths. Lemma lem:synthetic-characters, the bit-reversed order and lem:synthetic-transform-cost form one item. lem:no-sort-resampling and lem:tensor-resampling form one item. lem:lower-lower and lem:matrix-shear form one item. The source-transform procedure and the six-step algorithm form one item.
- The author's provisional items all hold up against the paper:
  - "butterfly layer's recursion" is real. It is `layer-recurrence`, from the paragraph "Unrolling the recurrence"; the paper gives it no label. A second recursion, for arithmetic depth (A(e) <= sA(e/m) + E), controls guard widths, not time.
  - "interchange recurrence" is real: eq:swap-recurrence in prop:power-interchange.
  - "saving s<Wm" is real. It is `rank-saving`, from the subsection "Explicit rational bounds for the two recurrences" (eq:explicit-motif-exponents). It is stated inside both interface propositions, and the introduction calls it "The decisive property".
  - "Gaussian resampling" is real. The paper's name is "Resampling without executing the two permutations". The Gaussian identity and its norm bounds are quoted from Harvey-van der Hoeven, not proved here.
  - "time accounting" is real. The paper calls it "The complete time bound" (tab:costs, eq:margin-list).

  None was dropped.
- `intro_summary` is true for every in-scope item except `stream-primitives`. So an intro-only plan can name nearly every item. The anchors, which all come from body lines, are what tell a derivation apart from a summary.

## Comparison with the Sonnet full-reading samples

I compared against argument.json in both samples. graph.json holds only target, concepts and background, with no argument nodes.

- Both trees cover almost every core item, with body quotes: the bank exchange, frames, labels, the bit and complex interfaces, the tau derivation, the matrix shear, the interchange recurrence, phase frames, packed addition, the layer recurrence, guard widths, synthetic transforms, signed packing, synthetic convolution, resampling, chirp and cancellation, CRT, primes, exact recovery and the margins g_i.
- Sample 1 has no node for the elementary stream and controlled-shift lemmas; it names them only as planned pages. It has no node for the L/N = 100/539 < 1/2 step, and it reaches s_b without the label-loss ratio. It does have a separate twist node (S6.4).
- Sample 2 includes stream-ops and controlled-shifts and the 100/539 ratio. It has the clearest time accounting, including "Setup and exceptional streams across all calls" (lines 4913-4973). I folded that into `time-accounting`, and I think the reference should require it at least as a clause. Sample 2 places the frame identity under the bit network only, although the complex network also uses it.
- Neither sample treats the conjecture-negation argument (lines 379-387) as a step; both only assert "disproves" in the root claim. Neither mentions the appendix grouped-rectangle lemma, which is correct because the main proof never cites it.
- Both have the three-update swap (D <- D-H, H <- H+D, D <- H-D) and the fixed-tape depth-first schedule as their own nodes. I folded these into `interchange-recurrence`. They are fine to accept as sub-steps.
- Nothing in either sample contradicts the paper as far as I checked. Sample 1's S1.6 calls tau the bound for "each wire making fewer than m^tau smaller calls", which is a loose paraphrase of s/W < m^tau but acceptable.

## Open questions for the reviewer

1. Is a list of about 26 items acceptable, or should supporting items be merged to reach 20?
2. Tiers: `arbitrary-width-interchange` and `crt-address-map` are marked supporting. A plan that leaves them out would still have the structure of the argument, but each feeds a cost row. Should they be core?
3. Should `packed-selected-bit-addition` be required in full (eight rotations plus repair), or is "basis changes cost O(V((eK)^tau+1))" enough?
4. `conjecture-negation` has only introduction anchors, because the paper develops it nowhere else. Is that acceptable under the "anchors from where it is developed" rule?
5. Results quoted from Harvey-van der Hoeven (the Gaussian identity, Prop. 4.7, Lemmas 4.8-4.12 and 5.1, and the O(N log N) multiplier) are dependencies, not derivations in this paper. Should the reference list them as separate external items?

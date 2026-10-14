# Mechanism checklist for six discriminating core results of `paper.tex` (secondary measure, under validation)

**Status:** under validation in K1 (`calibration/plans/K1.md`). It is not used to decide any experiment until K1's criteria are met. The content judge's rubric and the central-derivation rubric (`nlogn-central-saving.md`) remain the primary measures.

**Why these six:**
- **They carry the measure's information.** Across the 53 judgements of compared plans in J8–J10, they are the core results with the lowest faithful rates (0.17–0.55). Results faithful in ≥ 0.9 of judgements carry almost no information (`docs/journal/2026-10-14-sonnet-xhigh-and-rubric.md`).
- **Their `partial` verdict is coarse.** It lumps a plan missing one essential sub-derivation with one missing all but one.

**Where the sub-steps come from:**
- Each is a step of the paper's own derivation, at the lines given.
- They were read from the paper's derivation ranges, not from any plan.
- The recurring omissions in the judges' notes were used only to check that each step a judge has cited is covered.

**Present means:** the plan states the sub-step's defining content, either:
- in a claim or method;
- or in a quote that the plan uses as the step's support, placed in the role the paper gives it.

Naming the sub-step without its content is not presence.

| Id | Core result | Sub-step (defining content) | Paper lines |
|---|---|---|---|
| B1 | bank-exchange-network | the net target change $(\mathcal J\mathcal V+\mathcal R\mathcal G)x$ is independent of the initial scratch values, because the last rows restore them | 744–750 |
| B2 | bank-exchange-network | bit case: central coefficient $\lvert S\cap T\rvert\bmod 2$; the side wires cancel exactly the intersection-one cases; self coefficient one | 752–755 |
| B3 | bank-exchange-network | complex case: central coefficient $(\lvert S\cap T\rvert-1)/2$; side wires cancel intersections zero and two | 756–758 |
| B4 | bank-exchange-network | three stages $(x,y)\to(x,y+x)\to(-y,y+x)\to(-y,x)$, each invocation restoring its auxiliary wires | 761–764 |
| L1 | label-residual-budget | the per-edge orthogonal residuals and their dimensions (the residual table), every edge joining nested labels | 1013–1043, 1059–1060 |
| L2 | label-residual-budget | the side edge $2\to5$: $t_X\perp t_Y$ gives $\langle t_X\rangle\subset t_Y^\perp$, with complement $\langle t_X,t_Y\rangle^\perp$ of dimension $h-2$ | 1048–1058 |
| L3 | label-residual-budget | the only decreasing edge is center $3\to4$, losing $h$; telescoping gives sources $N$, sinks $Wm-N$, signed $Wm-2N$, and absolute $Wm-2N+2L$ | 1061–1071 |
| L4 | label-residual-budget | over $\mathbb F_2$ each nonzero residual is nondegenerate and nonalternating, so it has an orthonormal basis (absorbing alternating planes) | 1073–1102 |
| L5 | label-residual-budget | $L_{\rm b}/N=100/539$ and $L_{\rm c}/N=101/539$ | 1105–1109 |
| P1 | complex-phase-interface | for orthogonal binary vectors weight is additive mod 4, so $q_V-q_U=\sum_{v}\operatorname{wt}(v)[v\cdot x]$ over an orthonormal basis of the residual | 1232–1245 |
| P2 | complex-phase-interface | $\operatorname{wt}(v)$ is odd, so the phase is $i^{\varepsilon[v\cdot x]}$; with $H\operatorname{diag}(1,-1)H=X$ it becomes a translation kernel $aI+bX_v$ (or its inverse) | 1246–1257 |
| P3 | complex-phase-interface | on $k$ columns each residual vector gives one tensored factor, $s_{\rm c}$ factors in all | 1262–1271 |
| P4 | complex-phase-interface | endpoints: the zero label has frame $I$ and the full label $C^{\otimes m}$; the pair $X_a\to Y_a$ carries $C^{-1}$ on the 27 coordinates of $\operatorname{supp}(u_a)$, repaired by $Z_{a,k}$ before and $i^{27k}Z_{a,k}$ after ($C=iZC^{-1}Z$); the $X$ outputs are negated | 1273–1314 |
| R1 | rank-saving | $L/N<1/2$, so the deficit $N-2L>0$ and $s<Wm$ | 1105–1110 |
| R2 | rank-saving | $\eta_{\rm b}=339/22587335000000$ | 1319–1322 |
| R3 | rank-saving | $\eta_{\rm c}=73/19906842167500$ | 1323–1324 |
| R4 | rank-saving | both deficits exceed $20\cdot2^{-50}$ and $\log m<20$, so $m^{1-2^{-50}}>m(1-\eta)$ and $s/W<m^\tau$, $\tau=\sigma=1-2^{-50}$ | 1331–1343 |
| M1 | matrix-shear | the elimination proof of $A=E_1\Pi E_2$: a pivot at the topmost nonzero row's rightmost entry, clearing by lower-triangular elementary operations, repeating on the active indices; $\Pi$ has exactly $a$ ones | 1375–1391 |
| M2 | matrix-shear | a fixed odd prime avoiding every denominator in a finite table of factors, so the identities specialize to $\mathbb Z/q^b$ with invertible triangular factors | 1413–1423 |
| M3 | matrix-shear | a lower-triangular transformation costs $O(V)$, updating coordinates in descending physical order | 1425–1430 |
| M4 | matrix-shear | each one of $\Pi$ by $D_j\gets D_j+H_i$, an interchange, $D_j\gets H_i-D_j$: one interchange each, $a$ in all | 1432–1443 |
| S1 | simultaneous-layer | the constants $E=64(W+m+1)^3$, $B=s+E$, $\nu$, $C_1=\max\{20,\nu+3\}$, $C_0=128mB^2$, $\Delta=\lceil C_0d^{C_1}\rceil$ (with $\nu=11$, $C_1=20$ here) | 2457–2477 |
| S2 | simultaneous-layer | the arithmetic-depth recurrence $A(e)\le sA(e/m)+E$ at an internal node, $A(e)\le 8e$ at a leaf, with the per-invocation count below $E$ | 2479–2518 |
| S3 | simultaneous-layer | unrolling: $B^{k+1}\le B^2d^\nu$, so $A(e)\le 8B^2d^{\nu+1}$, and the total stays below $C_0d^{C_1}$ | 2523–2560 |
| S4 | simultaneous-layer | depth $\ell$ means values in $2^{-(p+\ell)}\mathbb Z[i]$ of modulus at most $2^\ell$; reserving $p+\Delta$ fractional and $\Delta+3$ integer bits is exact; $\epsilon C_1<1$ gives $\Delta=o(p)$ | 2562–2586 |
| S5 | simultaneous-layer | every row of $\mathcal H$ has absolute row sum one, so disks are preserved and the final truncation errs by less than $\sqrt2\,2^{-p}$ | 2751–2757 |

**Scoring:**
- The plan's score per core result is the number of its sub-steps present. Its total is the sum over all 26.
- A judge also marks a sub-step `wrong` when the plan states it contrary to the paper. A `wrong` sub-step does not count as present.

**Mechanical markers** (`calibration/tools/markers.py`):
- A regular expression per sub-step, where the defining content has a distinctive form, applied to the plan's claims, methods, motivations and quotes.
- A marker can only suggest presence: it cannot see role or correctness.
- It is a judge-free cross-check, not a score.

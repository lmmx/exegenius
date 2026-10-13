# Rubric: the central saving of "Integer multiplication below n log n"

**Status:** preregistered. This rubric was written and committed before any plan of E3 existed, and before any judge applied it. Its first use is E3's judging batch, which also re-judges E1's high-effort plans.

**Source:** `paper.tex`, 5,361 lines; line numbers are those of the file on wiki branches `131026-exp-e1-config` and `131026-exp-e3-effort` (`template/sources/paper.tex`).

## Why this rubric exists

The content judge (journal 2026-10-13-content-judge) scores 26 reference results one by one. In E1, plans at high effort stated almost all 19 core results. Yet none derived faithfully the construction that makes the paper work: a fixed network whose edge changes have total rank $s<Wm$, which turns into the power saving $e^\tau$ with $\tau<1$.

Counting results cannot see that gap. A plan can state "$s<Wm$" and "the recurrence gives $O(e^\tau)$" and still not show why. This rubric asks whether a plan reconstructs the chain of reasoning that produces the saving.

## The steps

Each step names the paper's lines. A step is **reconstructed** when the plan states it with its defining content, in the role it plays in the chain. Naming the step is not enough: the plan must give its key quantity, condition or mechanism.

| Id | Step | Defining content a reconstruction must carry | Paper |
|---|---|---|---|
| C1 | **The fixed network** | $h=100$; the data pairs $X_a,Y_a$ indexed by triples, $N=\binom h3^3$, $m=h^3$; a fixed linear network that exchanges the banks, $(X_a,Y_a)\mapsto(-Y_a,X_a)$, restoring every auxiliary wire | 640–660, 653–783 |
| C2 | **Frames** | a common invertible frame per gate commutes with a pointwise gate, so the network on arrays implements $D_{\rm out}\mathsf S D_{\rm in}^{-1}$, and each edge costs only a change of frame | 793–843 |
| C3 | **Labels** | each gate gets a nondegenerate subspace of $F^{\otimes3}$ (dimension $m$); every edge joins comparable labels; the only decreasing edges are the central wires from time 3 to 4, each losing $h$ | 845–1010 |
| C4 | **Rank equals dimension change** | for nested $U\subset V$, $P_V-P_U$ has rank $\dim V-\dim U$, so an edge's cost is its label's dimension change; the $N$ source edges are the exceptions, each of rank one | 1156–1166 |
| C5 | **The budget** | $\sum_e|\Delta\dim|=Wm-2N+2L$ with $L=Ich$; hence $s_{\rm b}=W_{\rm b}m-N+2L_{\rm b}$ | 997–1010, 1138–1154 |
| C6 | **The strict saving** | $L/N=100/539$ (bit) or $101/539$ (complex), which is below $1/2$, so the deficit $N-2L>0$ and $s<Wm$ | 1105–1110, 1166–1167 |
| C7 | **The endpoint identity** | $M_{\rm sink(\rho(w))}-M_{\rm source(w)}=I_m$ on every role, so the bit network applies the shear $H\gets H+D$ on every role | 1138–1172 |
| C8 | **Edges as recursive calls** | the shear by a rank-$a$ matrix factors as $E_1\Pi E_2$ and costs exactly $a$ interchanges plus $O(V)$, so a node makes $s$ recursive calls in all | 1364–1447 |
| C9 | **The recurrence and the exponent** | children of volume $V/W$, so $F_k\le(s/W)F_{k-1}+O(1)$; with $s/W<m^\tau$, $\tau=1-2^{-50}$ (from $\eta>20\cdot2^{-50}$ and $\log m<20$), $F_k=O(e^\tau)$ | 1319–1346, 1475–1557 |

**Verdicts per step:**
- `reconstructed`: as defined above;
- `mentioned`: named or alluded to without its defining content;
- `absent`;
- `wrong`: it contradicts the paper.

## Scores

These are reported separately from the content judge's item counts. They are never folded into them.

- **The central-derivation count:** the number of steps reconstructed, 0–9.
- **The central-derivation gate:**
  - **Passed** when C4, C5, C6 and C9 are all reconstructed and no step is wrong. That is the chain from "rank is dimension change" to "strict deficit" to "exponent below one".
  - **Failed** otherwise.
  - **Why those four:** without them a plan asserts the saving rather than explaining it.
- **Status:** the gate is a provisional quality bar for "research-grade" planning on this paper, chosen before E3's data. Its validity is not established. It is reported, not optimised against.

## How judges apply it

The judge's instructions for a batch that uses this rubric add a section asking for a `central` object in the verdict:

```json
"central": {
  "steps": [
    {"id": "C1", "verdict": "reconstructed|mentioned|absent|wrong",
     "plan_steps": ["S2.1"], "lines": [653, 739], "note": "..."}
  ],
  "summary": "one or two sentences on where the plan's account of the saving holds and where it thins out"
}
```

The judge checks the paper's lines for each step before deciding. When unsure, the judge chooses the more conservative verdict.

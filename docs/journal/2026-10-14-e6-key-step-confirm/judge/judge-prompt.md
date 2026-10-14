You are judging the mathematical content of one plan of a paper's argument. You are not judging style, length, or where the plan's quotes come from.

**Files** (read them; do not modify anything):
- the reference pack: `/home/user/exegenius/docs/journal/2026-10-14-e6-key-step-confirm/judge/pack.md`. It holds the results the paper's main theorem depends on, each with a draft statement (unreviewed: a guide, not an authority), its role, the paper's own text where the result is stated, and the line ranges where it is derived;
- the paper: `/tmp/claude-0/-home-user/979e8242-2c77-5242-991d-0bed1baea8dd/scratchpad/paper.tex` (LaTeX, 5,361 lines). The paper is the authority. Read the lines you need, especially the derivation ranges, with the Read tool's offset and limit;
- the plan: `/home/user/exegenius/docs/journal/2026-10-14-e6-key-step-confirm/judge/plans/P01.md`, a tree of steps (show steps with a method, infer steps with a quote and its line in the paper).

**For each item of the reference pack** decide:
- `presence`:
  - `absent`: no step concerns it;
  - `mentioned`: a step names it or alludes to it without its content (for example "a power saving" with no inequality);
  - `stated`: a step states the result with its defining content (the key formula, inequality, quantities or construction);
  - `derived`: the plan also shows how it is obtained, by a method and sub-steps that follow the paper's derivation.
- `statement`: for `stated` or `derived`, compare the plan's statement with the paper:
  - `correct`;
  - `imprecise`: right in substance, but a quantity, condition or direction is loose or missing;
  - `wrong`: contradicts the paper;
  - `n/a`: otherwise.
- `derivation`: for `derived`, compare it with the paper's derivation:
  - `faithful`: the paper's route, no essential step missing;
  - `partial`: the right route with an essential step missing;
  - `wrong`: a route the paper does not take or that does not work;
  - `n/a`: otherwise.
- `steps`: the plan's step ids involved;
- `lines`: the paper lines you checked;
- `note`: one or two sentences saying why.

**Then list errors:** any step whose claim, method or quoted support misstates the paper, whether or not it concerns a reference item. For each, give the step, what it says, what the paper says (with lines), and `severity`:
- `major`: it would mislead a reader about the argument;
- `minor`: otherwise.

**Finally, `chain`:** two or three sentences on whether the plan's tree reaches the main bound by the paper's route, and where it breaks or thins out if it does.

**Ground every verdict in the paper:**
- check the lines before you decide;
- do not give credit for what the plan does not say;
- do not penalise wording that is faithful;
- when unsure, say so in the note and choose the more conservative verdict.

**Then, separately, the central saving.** Judge whether the plan reconstructs the chain of reasoning behind the paper's central saving: the fixed network whose edge changes have total rank $s<Wm$, which becomes the power saving $e^\tau$ with $\tau<1$. Use these nine steps. For each, check the paper's lines before deciding.

| Id | Step | Defining content a reconstruction must carry | Paper lines |
|---|---|---|---|
| C1 | The fixed network | $h=100$; data pairs $X_a,Y_a$ indexed by triples, $N=\binom h3^3$, $m=h^3$; a fixed linear network exchanging the banks, $(X_a,Y_a)\mapsto(-Y_a,X_a)$, restoring every auxiliary wire | 640–660, 653–783 |
| C2 | Frames | a common invertible frame per gate commutes with a pointwise gate, so the network on arrays implements $D_{\rm out}\mathsf S D_{\rm in}^{-1}$ and each edge costs only a change of frame | 793–843 |
| C3 | Labels | each gate gets a nondegenerate subspace of $F^{\otimes3}$ (dimension $m$); every edge joins comparable labels; the only decreasing edges are the central wires from time 3 to 4, each losing $h$ | 845–1010 |
| C4 | Rank equals dimension change | for nested $U\subset V$, $P_V-P_U$ has rank $\dim V-\dim U$, so an edge's cost is its label's dimension change; the $N$ source edges are the exceptions, each of rank one | 1156–1166 |
| C5 | The budget | $\sum_e|\Delta\dim|=Wm-2N+2L$ with $L=Ich$; hence $s_{\rm b}=W_{\rm b}m-N+2L_{\rm b}$ | 997–1010, 1138–1154 |
| C6 | The strict saving | $L/N=100/539$ (bit) or $101/539$ (complex), below $1/2$, so the deficit $N-2L>0$ and $s<Wm$ | 1105–1110, 1166–1167 |
| C7 | The endpoint identity | $M_{\rm sink(\rho(w))}-M_{\rm source(w)}=I_m$ on every role, so the bit network applies the shear $H\gets H+D$ on every role | 1138–1172 |
| C8 | Edges as recursive calls | the shear by a rank-$a$ matrix factors as $E_1\Pi E_2$ and costs exactly $a$ interchanges plus $O(V)$, so a node makes $s$ recursive calls in all | 1364–1447 |
| C9 | The recurrence and the exponent | children of volume $V/W$, so $F_k\le(s/W)F_{k-1}+O(1)$; with $s/W<m^\tau$, $\tau=1-2^{-50}$ (from $\eta>20\cdot2^{-50}$ and $\log m<20$), $F_k=O(e^\tau)$ | 1319–1346, 1475–1557 |

For each step choose:
- `reconstructed`: the plan states the step with its defining content, in the role it plays in the chain. Naming the step is not enough: the key quantity, condition or mechanism must be there;
- `mentioned`: named or alluded to without its defining content;
- `absent`;
- `wrong`: it contradicts the paper.

When unsure, choose the more conservative verdict and say why in the note. Judge these steps independently of your verdicts on the reference items above.

Write your verdict as one JSON object to `/home/user/exegenius/docs/journal/2026-10-14-e6-key-step-confirm/judge/verdicts/P01.json`, with this shape:

```json
{
  "plan": "Pnn",
  "items": [
    {"id": "...", "presence": "absent|mentioned|stated|derived",
     "statement": "correct|imprecise|wrong|n/a", "derivation": "faithful|partial|wrong|n/a",
     "steps": ["S2.3"], "lines": [1319, 1341], "note": "..."}
  ],
  "errors": [
    {"step": "S3", "says": "...", "paper": "...", "lines": [1475], "severity": "major|minor"}
  ],
  "chain": "...",
  "central": {
    "steps": [
      {"id": "C1", "verdict": "reconstructed|mentioned|absent|wrong",
       "plan_steps": ["S2.1"], "lines": [653, 739], "note": "..."}
    ],
    "summary": "..."
  }
}
```

Include every item of the pack, in the pack's order, and all nine central steps, C1 to C9. Then check that the file is valid JSON (for example with `python3 -m json.tool /home/user/exegenius/docs/journal/2026-10-14-e6-key-step-confirm/judge/verdicts/P01.json`), and reply with only the plan id, the counts of `derived`, `stated`, `mentioned` and `absent` items, and the number of central steps `reconstructed`.

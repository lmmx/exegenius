**This page derives no steps of the argument: there are no items to judge. Give an empty `items` list, and judge the page's mathematical errors only.**

## This page

Concept: Fixed finite-tape machine model
Gloss: The computation model of the main theorem: one fixed finite alphabet and a fixed number of one-dimensional tapes, each head moving at most one cell per step.
Also called: multitape Turing machine, fixed-tape model
## Where the argument relies on this page

- A single deterministic Turing machine with a fixed finite alphabet and a fixed finite number of one-dimensional tapes multiplies two $n$-bit integers exactly in $O(n(\lg n)^{1-\kappa})$ worst-case time with $\kappa=2^{-182}$, disproving the $n\log n$ optimality conjecture of Sch\"onhage and Strassen in this model. (on [Power-saving integer multiplication](main-result.md))
- Bit variant: two disjoint address chunks of $u$ bits can be interchanged on a fixed number of tapes in $O(Vu^\tau)$ time, $\tau=1-2^{-50}$, although the intermediate steps form XOR combinations of data. (on [Fast address-chunk interchange](address-interchange.md))
- Fixed rational exponents and sizes make every hypothesis of the lemmas hold: $b=\lceil\log_2n\rceil$, $p=6b$, $d=\lfloor b^\epsilon\rfloor$, $K=\lfloor d^c\rfloor$, a power of two $T\in[4n/b,8n/b)$, $r=2^{\lceil(\log_2T)/d\rceil}$, axis lengths $t_i\in\{r/2,r\}$ with $\prod t_i=T$, $t_d=r$, and distinct odd primes $s_i\in((1-2\eta)t_i,(1-\eta)t_i]$. (on [Parameter and prime-length choice](parameter-choice.md))

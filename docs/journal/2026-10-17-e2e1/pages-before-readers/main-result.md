Power-saving integer multiplication is the main result: for $n$, the number of bits in each input, and $\kappa=2^{-182}$, the exponent saved from the power of $\lg n$, a fixed Turing machine multiplies two $n$-bit integers exactly in time $O(n(\lg n)^{1-\kappa})$, which is $o(n\log n)$, so it refutes the [Schönhage–Strassen optimality conjecture](conjecture.md) in this model.

The machine is one deterministic Turing machine with a fixed finite alphabet and a fixed finite number of one-dimensional tapes, and is correct for every $n\ge1$ <cite data-source="paper">There is one deterministic Turing machine $A$, with a fixed finite</cite>. The saving is positive, however small: $(\lg n)^{1-\kappa}=\lg n/(\lg n)^{\kappa}$ and $(\lg n)^{\kappa}\to\infty$.

## Why the usual route stalls

Scanning $\Theta(n)$ stored bits at each of $\Theta(\log n)$ transform levels already costs $\Theta(n\log n)$. Beating that needs cheaper address movement and cheaper butterfly arithmetic <cite data-source="paper">We therefore need savings in both movement and arithmetic.</cite>

## How the saving is built

1. Two tape primitives, the interchange of address chunks and the simultaneous butterfly layer, are built from fixed finite linear networks. Each costs the data volume times a power below one of its width, by [Recursive power saving](rank-recurrence.md).
2. These speed up the synthetic transforms over $\mathbb C[y]/(y^r+1)$ ([Tape layout for synthetic transforms](transform-layout.md)) and the ring convolution built on them ([Convolution from the retained transform layout](ring-convolution.md)).
3. Gaussian resampling and Bluestein chirps turn the prime-length transforms into these power-of-two transforms, with permutations cancelled rather than sorted ([Retained permutation cancellation](retained-permutations.md)).
4. The exact $2n$-bit product is assembled by [The multiplication algorithm](multiplication-algorithm.md) from radix digits, two forward transforms, a pointwise product, the opposite transform, rescaling, rounding and carries, with total error below $1/2$.

## Totaling the cost

[Cost accounting and the exponent κ](cost-accounting.md) totals the running time as follows, dividing every cost by $V=Tp=\Theta(n)$, the number of stored bits, where $T$ counts coefficients and $p$, of order $\lg n$, is the precision in bits of each. The axis count is $d=\Theta(p^{\epsilon})$ and the chunk width $K=\lfloor d^{c}\rfloor$, with fixed exponents $\epsilon$ and $c$ from [Parameter and prime-length choice](parameter-choice.md); here $c$ is not the machine constant, the scaling integer, the control bit or the unit of earlier pages. The seven dominant normalized costs are powers of $p$:
$$p^{\epsilon(1+c)},\quad p^{1-\epsilon c(1-\tau)},\quad p^{1-\epsilon(1-\lambda')},\quad p^{\tau+\epsilon(2-\tau)},\quad p^{3/4+\delta+3\epsilon/2},\quad p^{\epsilon+\delta},\quad p^{1-\epsilon}.$$
They come from prefix moves and single butterfly rounds ($dK$), chunk exchanges ($pK^{\tau-1}$), simultaneous butterfly rounds ($\ell d^{\lambda'}$, with $\ell$ the address bits of a long axis), Chinese-remainder and axis layouts, Gaussian line maps, chirps and twists, and packed ring products ($\log(rp)$). The rest is absorbed.

The margin $g_i$ of the $i$-th term is one minus its exponent. The fixed parameters are $\tau=1-2^{-50}$, $\lambda'=1-2^{-54}$, $c=2^{-56}$, $\delta=1/16$ and $\epsilon=2^{-75}$ <cite data-source="paper">\epsilon=2^{-75},\qquad \kappa=2^{-182}.</cite>. The smallest margin belongs to chunk exchanges:
$$g_2=\epsilon c(1-\tau)=2^{-75}\cdot2^{-56}\cdot2^{-50}=2^{-181}=2\kappa,$$
and the other margins are larger <cite data-source="paper">Thus $\min_i g_i=2^{-181}=2\kappa$.</cite>

So every dominant term is at most $p^{1-2\kappa}$. Remaining factors are fixed powers of $\log p$, which are below $p^{\kappa}$ for large $p$, so the normalized total is $O(p^{1-\kappa})$ and the time is $O(Vp^{1-\kappa})=O(n(\lg n)^{1-\kappa})$.

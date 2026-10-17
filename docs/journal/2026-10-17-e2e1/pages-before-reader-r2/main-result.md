Power-saving integer multiplication is the main result: for $n$, the number of bits in each input, and $\kappa=2^{-182}$, the exponent saved from the power of $\lg n$, one fixed Turing machine multiplies two $n$-bit integers exactly in time $O(n(\lg n)^{1-\kappa})$, which is $o(n\log n)$, so it refutes the [Schönhage–Strassen optimality conjecture](conjecture.md) in this model.

The machine is deterministic, with a fixed finite alphabet and a fixed finite number of one-dimensional tapes, and is correct for every $n\ge1$ <cite data-source="paper">There is one deterministic Turing machine $A$, with a fixed finite</cite>. The saving is positive, however small: $(\lg n)^{1-\kappa}=\lg n/(\lg n)^{\kappa}$ and $(\lg n)^{\kappa}\to\infty$, so the ratio to $n\lg n$ tends to $0$.

## Why movement and arithmetic must get cheaper

The multiplication reduces to transforms on arrays of polynomials over $\mathbb C[y]/(y^r+1)$, whose calculation consists of address rearrangements, signed shifts and two-point butterflies <cite data-source="paper">In this representation, the transform calculation consists of address</cite>. A transform has $\Theta(\log n)$ levels, and each level must bring the partners of every butterfly together and then combine them. On a tape machine that is one pass over the whole stored volume of $\Theta(n)$ bits per level, so the levels cost $\Theta(n\log n)$ <cite data-source="paper">Scanning an array of $\Theta(n)$ stored bits at each of</cite>. The remedy is to let one pass do the work of many levels, which needs cheaper movement and cheaper arithmetic together <cite data-source="paper">We therefore need savings in both movement and arithmetic.</cite>

Two tape primitives do this, where $V$ is the stored bit volume. Interchanging two address chunks of $u$ bits moves $u$ address bits at once for $O(Vu^\tau)$ with fixed $\tau<1$ <cite data-source="paper">Its cost is $O(Vu^\tau)$ for a fixed $\tau<1$.</cite> The simultaneous butterfly layer does one round of butterflies on up to $d$ axes at once for $O(Vd^{\lambda'})$ with $\lambda'<1$, instead of $d$ separate passes.

## How the saving is built

1. Both primitives are built from fixed finite linear networks and are power-saving by [Recursive power saving](rank-recurrence.md).
2. They speed up the synthetic transforms ([Tape layout for synthetic transforms](transform-layout.md)) and the ring convolution built on them ([Convolution from the retained transform layout](ring-convolution.md)).
3. Gaussian resampling and Bluestein chirps turn prime-length transforms into these power-of-two ones, with permutations cancelled rather than sorted ([Retained permutation cancellation](retained-permutations.md)).
4. The exact $2n$-bit product is assembled by [The multiplication algorithm](multiplication-algorithm.md) from radix digits, two forward transforms, a pointwise product, the opposite transform, rescaling, rounding and carries, with total error below $1/2$.

## Where $\kappa$ comes from

The running time is totalled by [Cost accounting and the exponent κ](cost-accounting.md) after dividing every cost by $V=Tp=\Theta(n)$, where $T$ counts the complex coefficients and $p$, of order $\lg n$, is the number of fractional bits of each. Every dominant normalized cost is a power $p^{1-g_i}$ with margin $g_i>0$. The margins come from the fixed exponents of [Parameter and prime-length choice](parameter-choice.md): $\tau=1-2^{-50}$, chunk width $K=\lfloor d^{c}\rfloor$ with $c=2^{-56}$, and $d=\Theta(p^{\epsilon})$ axes with $\epsilon=2^{-75}$ <cite data-source="paper">\epsilon=2^{-75},\qquad \kappa=2^{-182}.</cite>

The smallest margin belongs to the chunk exchanges, which cost $p$ levels times $K^{\tau-1}$ per bit, that is $p^{1-g_2}$ with
$$g_2=\epsilon c(1-\tau)=2^{-75}\cdot2^{-56}\cdot2^{-50}=2^{-181}=2\kappa.$$
All other margins are larger <cite data-source="paper">Thus $\min_i g_i=2^{-181}=2\kappa$.</cite> The chunk is only a tiny power $K=p^{\epsilon c}$ of $p$ and the exchange beats the straightforward cost by only $K^{1-\tau}$, so this margin is a product of three tiny exponents.

Each dominant term is thus at most $p^{1-2\kappa}$. The terms also carry fixed powers of $\log p$, for example the $O(p\log p)$ cost of multiplying two $p$-bit numbers, a normalized factor $\log p$. Half the margin pays for them: any fixed power of $\log p$ is at most $p^\kappa$ for large $p$ <cite data-source="paper">Any fixed power of $\log p$ is at most $p^\kappa$</cite>, so $p^{1-2\kappa}\cdot p^{\kappa}=p^{1-\kappa}$. The time is then $O(Vp^{1-\kappa})=O(n(\lg n)^{1-\kappa})$, with the finitely many small lengths handled by schoolbook multiplication. Any positive $\kappa$ would refute the conjecture; $2^{-182}$ is what these parameters yield.

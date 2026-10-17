Cyclic convolution is the product of two sequences whose indices wrap around; the multiplication algorithm reduces integer multiplication to it, because Fourier-type transforms turn it into pointwise multiplication (see [Discrete Fourier transform](dft.md)).

For two sequences $f,g$ indexed by $\mathbb Z/q\mathbb Z$, where $q$ is the number of entries in each sequence, the cyclic convolution is
$$(f*g)_k=\sum_{j\in\mathbb Z/q\mathbb Z} f_j\,g_{k-j},$$
with $k-j$ taken modulo $q$ (here $j,k$ are sequence indices, not the depth of earlier pages). Equivalently, $f*g$ is the product of the polynomials $\sum_j f_jx^j$ and $\sum_j g_jx^j$ reduced modulo $x^q-1$: since $x^q=1$, a term $x^m$ with $m\ge q$ lands on index $m-q$.

Take $q=3$, $f=1+2x+3x^2$ and $g=1+x$. The ordinary product is $1+3x+5x^2+3x^3$, and the wrapped term $3x^3$ becomes $3$, giving $4+3x+5x^2$. If the ordinary product has degree below $q$, nothing wraps and $f*g$ is the ordinary product.

## Arrays over the negacyclic ring

The algorithm convolves arrays with several axes. Here $d$ is the number of axes of the underlying cyclic convolution, and $t_i$ is the length of axis $i$. After the last axis is turned into polynomial coefficients, an array $f$ assigns to each address in the other $d-1$ axes one entry of the ring of [Polynomial ring modulo y^r + 1](ring-y.md). The convolution $f*g$ wraps each cyclic axis independently and multiplies entries by the ring product, where wrapping past $y^r$ flips the sign.

Let $M=\prod_{i<d}t_i$ be the number of addresses, and $T=rM$ the number of complex coefficients in an array. The normalized convolution is
$$\mathcal M_r(f,g)=\frac{f*g}{rM}=\frac{f*g}{T}.$$
A fixed output coefficient is the average of $rM$ signed products of one coefficient of $f$ and one of $g$ <cite data-source="paper">Thus that coefficient averages $rM$ signed scalar products, and $\mathcal M_r$ is a bilinear contraction</cite>, so with $\|\cdot\|_\infty$ the largest modulus of a coefficient,
$$\|\mathcal M_r(f,g)\|_\infty\le\|f\|_\infty\|g\|_\infty.$$
Dividing by $T$ cancels the growth of a sum of $T$ products, which is the reason for the normalization.

## Why index permutations are never executed

Gaussian resampling lets the algorithm compute transforms of prime length through power-of-two lengths ([Gaussian resampling](gaussian-resampling.md)), but its formula reads the input and writes the output in permuted index order. Executing such a permutation is a sort of data on a tape, costing time proportional to the line length times a logarithm of it <cite data-source="paper">Its two label-and-sort operations implement $P_t$ and $P_s^{-1}$, each costing $O(tp\log t)$.</cite> The extra logarithmic factor is what the power saving cannot afford, so the permutations stay inside the identities, and convolution is what lets them vanish. Write $\pi$ for a permutation of the indices, $F$ for the normalized Fourier transform of length $q$, and $F^+$ for the transform with the opposite sign in the exponent.

Source side. Convolution is computed as a forward transform of each input, an entrywise product $\cdot$, and an opposite transform: $F(f*g)=q\,(Ff)\cdot(Fg)$ and $F^+F=I/q$, so $f*g=q^2F^+\bigl((Ff)\cdot(Fg)\bigr)$. An entrywise product pairs entries at the same position, so $\pi(a\cdot b)=(\pi a)\cdot(\pi b)$ for any permutation: scrambled forward outputs multiply to the scrambled product. The resampling permutation multiplies indices by a unit modulo the length, and for it $F^+\pi=\pi^{-1}F^+$, so applying the same $\pi$ in the opposite transform undoes the scramble. Thus the retained source permutation cancels when computing a convolution <cite data-source="paper">retained source permutation $R$ cancels when computing a convolution.</cite>, and no sort occurs across the whole chain.

Target side. The middle transform has its output relabelled, which is a single use with nothing to cancel against. The [Bluestein chirp identity](chirp.md) writes it as a chirp phase, a cyclic convolution and another chirp phase. The product of the chirp phases at $j$, $j-k$ and $k$ is the Fourier kernel evaluated at the relabelled frequency, so the relabelling sits in the phases and costs nothing beyond a convolution the algorithm performs anyway.

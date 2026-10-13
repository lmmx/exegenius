A simultaneous butterfly layer applies the butterfly to one bit offset in several address chunks, so a round of the [synthetic polynomial transforms](synthetic-transform.md) avoids one whole-array pass per chunk <cite data-source="paper">factors would require $D$ whole-array passes.</cite>

Let $V$ be the logical bit volume of the array, $p$ the number of fractional bits of the grid of multiples of $2^{-p}$, and $d$ an upper bound on the number of chunks. Here $D$ is the number of selected chunks and $K$ the chunk width in bits. The layer applies $H_0(u,v)=((u+v)/2,(u-v)/2)$ to each coefficient, at one selected bit in each of $D\le d$ chunks of $K=\lfloor d^c\rfloor$ bits, with $c>0$ fixed.

Here $e$ counts the axes in a group, $m$ the slots, and $s$ the child calls per node.

Write $X=\begin{pmatrix}0&1\\1&0\end{pmatrix}$ for the bit flip and $C=\frac{1+i}{2}I+\frac{1-i}{2}X$. Then
$$H_0^{\otimes D}=\Big(\tfrac{1-i}{2}\Big)^{D}\,\mathcal S_D\,C^{\otimes D}\,\mathcal S_D,$$
where $\mathcal S_D$ multiplies each address's value by $i$ to the power of its number of selected 1-bits. The tensor power $C^{\otimes D}$ is computed by [Recursive implementation of the phase network on selected bits](phase-edge-recursion.md): a group of $e$ axes splits into $m$ slots, and $s$ child calls each run on $e/m$ axes. Here $W$ is the number of wire roles. The recursion is scheduled by [Role streams and fixed-tape depth-first scheduling](role-streams.md), so each child acts on a stream of volume $V/W$.

Let $F(e)$ be the time per unit volume for $e$ axes. The $s$ children cost $s(V/W)F(e/m)$, and the node's row work costs $O((eK)^\tau+1)$ per unit volume, with a fixed exponent $\tau\in(0,1)$:
$$F(e)\le\frac sW\,F\Big(\frac em\Big)+O\big((eK)^\tau+1\big),\qquad F(e)=O(e)\text{ for }e<d^{1/2}.$$
Unroll to depth $J$, where $e/m^J<d^{1/2}$. An internal node at depth $j<J$ has $e/m^j\ge d^{1/2}$, so $K\le d^c\le(e/m^j)^{2c}$ and its overhead is $O((e/m^j)^\lambda)$, with fixed $\lambda<1$ and $\tau(1+2c)<\lambda$. With $s/W\le m^\sigma$ and $0\le\sigma<\lambda$,
$$\sum_{j<J}\Big(\frac sW\Big)^jO\Big(\Big(\frac e{m^j}\Big)^\lambda\Big)=O\Big(e^\lambda\sum_{j\ge0}\Big(\frac s{Wm^\lambda}\Big)^j\Big)=O(e^\lambda).$$
A leaf of size $q=e/m^J$ has weight $(s/W)^J\le m^{J\sigma}=(e/q)^\sigma$, so leaves cost $O(e^\sigma q^{1-\sigma})\le O(e^\sigma d^{(1-\sigma)/2})$. For $e\le d$ the group costs $O(d^\lambda+d^{\sigma+(1-\sigma)/2})$ per unit volume. Splitting the $D$ chunks into at most $(m-1)(1+\lfloor\log_m d\rfloor)$ pieces whose sizes are powers of $m$ gives
$$O\big(\log(2d)\,[d^\lambda+d^{\sigma+(1-\sigma)/2}+1]\big)=O(d^{\lambda'})$$
for any fixed $\lambda'$ with $\max\{\lambda,\sigma+(1-\sigma)/2\}<\lambda'<1$. Each level's share shrinks by $s/(Wm^\lambda)<1$, as in [recursive power saving](recursive-power-saving.md).

So the layer costs $O(Vd^{\lambda'})$ <cite data-source="paper">layer costs $O(Vd^{\lambda'})$</cite> on fixed tapes. Intermediate values are kept exact by [Exact Gaussian dyadic arithmetic and guard widths](exact-dyadic-guard.md), using guard bits (extra integer and fractional bits). Their number is $o(p)$ <cite data-source="paper">this is $o(p)$ uniformly over the allowed $d$</cite>, so coefficients stay $O(p)$ bits.

One truncation toward zero to $2^{-p}\mathbb Z$ moves each real and imaginary part by less than $2^{-p}$, so the error is below $\sqrt2\,2^{-p}$.

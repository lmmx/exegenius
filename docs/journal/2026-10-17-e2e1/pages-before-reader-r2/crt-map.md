The Chinese-remainder address map rewrites one cyclic convolution of length $S$ as a convolution on several short axes of pairwise coprime lengths, moving data on tape by controlled shifts and axis reorderings only. Let $d$ be the number of axes (unlike on the [Cyclic convolution](convolution.md) page, no coefficient axis is counted), let $i$ index them, let $s_i$ be the length of axis $i$, the lengths being pairwise coprime, and let $S=s_1\cdots s_d$. The split is due to Agarwal and Cooley <cite data-source="paper">Agarwal and Cooley used the Chinese remainder theorem to convert one cyclic convolution into multidimensional cyclic convolution</cite>.

The reason for the split is that a transform of length $S$ has no power-of-two structure, while over coprime axes it becomes a product of short one-axis transforms, each treated separately. This prepares the data for [Multidimensional transforms](multidim-transform.md), which act on one axis at a time.

## The isomorphism

Let $P_i=\prod_{j<i}s_j$ be the product of the lengths of the axes before axis $i$, and $\mu_i=P_i^{-1}\pmod{s_i}$. It is invertible because $P_i$ is coprime to $s_i$.

Define $\Phi:\mathbb C[x]/(x^S-1)\to\mathbb C[x_1,\dots,x_d]/(x_i^{s_i}-1)$ by $x\mapsto\prod_i x_i^{\mu_i}$, with one variable $x_i$ per axis. Since $s_i\mid S$, each $x_i^{\mu_iS}=1$, so $\Phi$ respects $x^S=1$ and defines an algebra map.

For a coefficient index $k$ (an exponent of $x$), $\Phi(x^k)=1$ exactly when $s_i\mid\mu_ik$ for every $i$; as $\mu_i$ is a unit modulo $s_i$ this means $s_i\mid k$ for every $i$, and as the $s_i$ are pairwise coprime, $S\mid k$. So the image of $x$ has order $S$ and its $S$ powers are distinct. The target has exactly $S$ monomials, so the powers run through all of them exactly once. Hence $\Phi$ is an isomorphism and preserves [Cyclic convolution](convolution.md), the product in the ring.

## Where each coefficient goes

Write $k=\sum_ja_jP_j$ in mixed radix, with digits $0\le a_j<s_j$. The axis-$i$ coordinate $b_i$ of $\Phi(x^k)$ is $\mu_ik\bmod s_i$. Terms with $j>i$ vanish modulo $s_i$, since $s_i$ divides $P_j$, and $\mu_iP_i\equiv1$ modulo $s_i$. Hence
$$b_i=a_i+\mu_i\sum_{j<i}a_jP_j\pmod{s_i},$$
which depends only on lower-index digits: it is a triangular system.

Example: $s=(3,5)$, so $S=15$, $P_2=3$, $\mu_1=1$, $\mu_2=3^{-1}=2\pmod 5$. For $k=4=1+3\cdot1$ we have $(a_1,a_2)=(1,1)$, so $b_1=1$ and $b_2=1+2\cdot1=3$. Directly, $x_1^4x_2^8$ has exponents $(4\bmod3,\,8\bmod5)=(1,3)$.

## Implementation on tape

Coefficients are stored in the order of $k$, which is lexicographic in $(a_d,\dots,a_1)$ with $a_d$ outermost. Pad axis $i$ with zero records to a length $t_i$, a power of two with $r/2\le t_i\le r$, where $r=2^\ell$ and $\ell$ is the number of address bits of an axis of length $r$. Let $T=\prod_it_i$ be the number of padded records, each of $O(p)$ bits. One nested scan of the box $\prod_i[0,t_i)$ copies the next coefficient at addresses with every $a_i<s_i$ and writes a zero record elsewhere. The counters fit in $O(p)$ bits because $T<2^p$ by [Parameter and prime-length choice](parameter-choice.md) <cite data-source="paper">$t_i\leq r\leq T<8n/b<2^{6b}=2^p$</cite>, so the scan costs $O(Tp)$.

In the stored order the controls $a_j$, $j<i$, come after the target $a_i$. Reorder the padded axes to $(a_1,\dots,a_d)$, so that coordinates below $i$ precede coordinate $i$. Then update the targets in the order $i=d,d-1,\dots,1$. When target $i$ is updated, none of its lower-index controls has yet changed, so the offset is computed from the original digits $a_j$ required by the formula. Each update is a prefix-controlled cyclic shift of the valid interval $[0,s_i)$ with identity on $[s_i,t_i)$, so zero padding stays zero. The map is implemented by these shifts, each of linear cost by [Controlled shifts and rational scalings](controlled-shifts.md), which includes computing the offsets. The inverse applies the subtractions for $i=1,\dots,d$, then reorders back and deletes the padding.

The only expensive part is reordering. Let $\tau<1$ be a fixed exponent. A [chunk interchange](address-interchange.md) exchanges two axes of equal address width $L$ in $O(VL^\tau)$ time <cite data-source="paper">equal width $L$ in $O(VL^\tau)$ time</cite>, where $V=Tp$ is the volume of the array. Axes have $\ell$ or $\ell-1$ address bits. To exchange two adjacent axes of unequal width, move the longer axis's leading bit outside their equal-width pair, swap that pair, and put the leading bit at its new position, adding a fixed number of linear passes. Reversing all axes uses $O(d^2)$ such moves. The shifts and scans cost only $O(dTp)$, so the map, its inverse and the padding cost $O(Tp\,d^2(1+\ell^\tau))$ <cite data-source="paper">O\bigl(Tp\,d^2(1+\ell^\tau)\bigr).</cite>

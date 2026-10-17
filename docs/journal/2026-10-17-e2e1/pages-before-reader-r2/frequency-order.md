Retained frequency order is the bit-reversed order in which the forward transform leaves its outputs, kept by the pointwise product and consumed by the opposite transform, so no reordering is executed.

It orders the outputs of the [synthetic polynomial transform](synthetic-transform.md) over $\mathcal R_r=\mathbb C[y]/(y^r+1)$, normed by the largest coefficient modulus. Let $t$, the transform length, be a power of two dividing $2r$; put $a=\log_2t$, the number of rounds (here $a$ changes meaning), and $\omega_t=y^{2r/t}$. For $z\in\mathcal R_r^t$, with input positions $k$ and output index $j$ (here $j$ changes meaning: it is the position of an output of the transform), $0\le j<t$, the transforms $F_t^-$ and $F_t^+$ are

$$(F_t^-z)_j=\frac1t\sum_{k=0}^{t-1}\omega_t^{-jk}z_k,\qquad (F_t^+z)_j=\frac1t\sum_{k=0}^{t-1}\omega_t^{jk}z_k.$$

Multiplying by a power of $y$ permutes coefficients and flips signs, so every power of $\omega_t$ is an isometry. Each entry averages $t$ such images, so both transforms contract.

The ring has zero divisors, so we cannot divide by $\omega_t^j-1$; directly, $\sum_{k}\omega_t^{jk}$ equals $t$ if $t\mid j$ and $0$ otherwise. For $t\nmid j$ write $j=2^vj_0$ with $j_0$ odd and $0\le v<a$. Replacing $k$ by $k+t/2^{v+1}$ modulo $t$ permutes the summands and multiplies the sum by $y^{rj_0}=-1$, so the sum equals its negative. The additive group of $\mathcal R_r$ has no two-torsion (no nonzero element equals its negative), so the sum is zero <cite data-source="paper">Hence the sum is zero, since the additive group of</cite>. Expanding the composite gives $F_t^+F_t^-=t^{-1}I$.

Let $H_0=\frac12\left(\begin{array}{rr}1&1\\1&-1\end{array}\right)$. Decimation in frequency pairs $z_k$ with $z_{k+t/2}$; write an output index as $2h+b$, with lowest bit $b$. Since $\omega_t^{t/2}=-1$,

$$(F_t^-z)_{2h+b}=\frac1{t/2}\sum_{k=0}^{t/2-1}(\omega_t^2)^{-hk}\,\omega_t^{-bk}\,\frac{z_k+(-1)^bz_{k+t/2}}2 .$$

Apply $H_0$ to each pair, multiply the $b=1$ output by $\omega_t^{-k}$, and recurse on the two length-$t/2$ branches.

Call a position's address bits, high to low, slots $a-1,\dots,0$. A pair differs only in the highest unprocessed slot, where $H_0$ writes $b$. After $s$ rounds, $s$ being the number of completed rounds, slots $a-1,\dots,a-s$ hold frequency bits $0,\dots,s-1$ and the rest index a sequence of length $t/2^s$; the next round writes bit $s$ into slot $a-1-s$. After $a$ rounds $(F_t^-z)_j$ sits at slot $\operatorname{rev}_a(j)$, where $\operatorname{rev}_a$ reverses the $a$ binary digits of $j$. Let $B_t$ be the permutation moving the entry at $j$ to $\operatorname{rev}_a(j)$ <cite data-source="paper">Let $B_t$ denote this record permutation: it moves the entry at $j$ to $\operatorname{rev}_a(j)$.</cite>

The pointwise product multiplies two transformed arrays slot by slot, each slot a product in $\mathcal R_r$, and never moves an entry between slots. Both arrays come from the same procedure $B_tF_t^-$, so slot $s$ of each holds the entry of frequency $\operatorname{rev}_a(s)$. The product at slot $s$ is therefore the product of the two entries of that one frequency, and the result stays in the same bit-reversed order <cite data-source="paper">retain the same frequency order through their pointwise product</cite>. The opposite procedure takes this order as its input.

It runs the levels backwards. Each forward level is invertible, and as $H_0^2=I/2$ the reversed level is half its inverse, so the reverse procedure composed with $B_tF_t^-$ is $2^{-a}I=I/t$. By character cancellation $F_t^+B_t^{-1}$ composed with $B_tF_t^-$ is also $I/t$. As $B_tF_t^-$ is invertible (a product of invertible levels) the two reverse operators agree. The exact procedures implement $B_tF_t^-$ and $F_t^+B_t^{-1}$.

With several axes, indexed by $i$, a round applies $H_0$ on every participating axis, and $\mathcal H$ denotes this one parallel butterfly. Let $u_i$ be axis $i$'s current length and $b_i$ its new bit. Its twiddle is $y^{-2rb_ik_i/u_i}$, where $k_i$ is the number formed by the bits in its unprocessed slots below the paired slot, so $0\le k_i<u_i/2$. It depends only on axis-$i$ address bits, which entries paired by another axis share, so it commutes with that axis's butterfly. Twiddles group into one monomial $J_E$, the recordwise operation multiplying the record by $y^E$, where $E$ depends on the address, not coefficients:

$$E=-\sum_i\frac{2r}{u_i}\,b_ik_i\pmod{2r}.$$

$\mathcal H$ is a contraction and $J_E$ an isometry, so every round is a contraction.

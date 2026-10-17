The discrete Fourier transform (DFT) is a linear map on sequences of complex numbers that replaces a convolution by a pointwise product, which is what lets the multiplication algorithm turn a convolution into cheap coordinatewise work.

Let $q\ge 1$ be the number of entries of a sequence, and let $u=(u_0,\dots,u_{q-1})$ be a sequence of $q$ complex numbers, with subscripts taken modulo $q$. The normalized transform $F_q u$ has entries $(F_q u)_k = \frac1q\sum_{j=0}^{q-1} u_j\, e^{-2\pi i jk/q}$ <cite data-source="paper">(F_q u)_k=\frac1q\sum_{j=0}^{q-1}u_j e^{-2\pi i jk/q}</cite>. Writing $\omega = e^{2\pi i/q}$ for the primitive $q$-th root of unity, the weights are the powers $\omega^{-jk}$. The opposite transform $F_q^+$ uses $\omega^{+jk}$ instead. Because of the factor $1/q$, each output is an average of inputs multiplied by numbers of modulus one, so neither transform increases the largest entry modulus <cite data-source="paper">Each normalized Fourier transform above is a contraction</cite>.

## Why it diagonalizes convolution

The transform diagonalizes [cyclic convolution](convolution.md): the transform of a convolution is a pointwise product of transforms.

For sequences $u,v$ of length $q$, write $(u*v)_k=\sum_j u_j v_{k-j}$. Substituting $l=k-j$ and using that the wrapped index $l$ runs over a full period gives
$$(F_q(u*v))_m=\frac1q\sum_j u_j\omega^{-jm}\sum_l v_l\omega^{-lm}=q\,(F_qu)_m\,(F_qv)_m .$$

Since $\sum_{m}\omega^{m(k-j)}$ equals $q$ when $k=j$ and $0$ otherwise, applying $F_q^+$ after $F_q$ returns $u/q$. Applying $F_q^+$ to the entrywise product of two transforms therefore gives
$$F_q^+\bigl[(F_qu)\cdot(F_qv)\bigr]=\frac{u*v}{q^2}.$$
A convolution is thus computed by two forward transforms, a pointwise product, one opposite transform, and a known rescaling.

## Example

Take sequences of length $q=2$, so $\omega=-1$ and $F_2$ is the transform $F_q$ with $q=2$:
$$F_2(u_0,u_1)=\Bigl(\frac{u_0+u_1}{2},\ \frac{u_0-u_1}{2}\Bigr),$$
a sum and a difference. Since $\omega^{-1}=\omega=-1$, the opposite transform $F_2^+$ has the same formula. The cyclic convolution of $(1,2)$ with $(3,4)$ is $(1\cdot3+2\cdot4,\ 1\cdot4+2\cdot3)=(11,10)$. The two forward transforms are $(1.5,-0.5)$ and $(3.5,-0.5)$, with pointwise product $(5.25,0.25)$. Applying $F_2^+$ gives $(2.75,2.5)=(11,10)/4$, as the identity predicts.

Computing $F_q$ directly takes $O(q^2)$ operations. Repeated two-point steps like the one above reduce this to $O(q\log q)$ when $q$ is a power of two, as described in [Butterflies and the fast transform](butterfly.md).

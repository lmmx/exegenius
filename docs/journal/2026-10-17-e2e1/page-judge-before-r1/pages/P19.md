The Bluestein chirp identity rewrites a permuted [Discrete Fourier transform](dft.md) as a pointwise chirp phase, a cyclic convolution, and another chirp phase, so the target permutation of resampling is absorbed into the convolution and never executed as a sort. This is how the middle transform, including its target permutation, becomes a normalized cyclic convolution <cite data-source="paper">Bluestein's chirp identity converts the middle transform,</cite>.

## Setting

Data are arrays indexed by $\boldsymbol j=(j_1,\dots,j_d)$, where $d$ is the number of axes. The tuple $\boldsymbol t=(t_1,\dots,t_d)$ lists the axis lengths, where $t_i$ is the even number of entries along axis $i$, and $s_i$ is an integer coprime to $t_i$. Put $T=\prod_i t_i$, the total number of entries. Indices along axis $i$ are taken modulo $t_i$.

The normalized transform $F_{\boldsymbol t}$ is given by $(F_{\boldsymbol t}u)_{\boldsymbol f}=\frac1T\sum_{\boldsymbol k}u_{\boldsymbol k}\exp(-2\pi i\sum_i k_if_i/t_i)$, where $\boldsymbol k$ runs over input positions and $\boldsymbol f=(f_1,\dots,f_d)$ is the frequency labelling an output entry. The target permutation $Q$ reorders the output so that position $\boldsymbol j$ holds the entry of frequency $(-s_ij_i)_i$. So in $QF_{\boldsymbol t}u$, the coefficient of $u_{\boldsymbol k}$ in output position $\boldsymbol j$ is $\frac1T\exp(2\pi i\sum_i s_ij_ik_i/t_i)$.

## The identity

Define the chirp $a_{\boldsymbol j}=\exp\left(-\pi i\sum_{i=1}^d s_ij_i^2/t_i\right)$, a phase quadratic in the index, and let $\overline a$ be its complex conjugate. When every $t_i$ is even <cite data-source="paper">assume every $t_i$ is even.</cite>,
$$QF_{\boldsymbol t}u=\overline a\cdot\frac{a*(\overline a\cdot u)}{T},$$
where the dot is coordinatewise multiplication and $*$ is cyclic convolution. The permuted transform is thus reduced to one [Cyclic convolution](convolution.md) of length $T$ between two pointwise phase multiplications.

## Derivation

The convolution reads $a_{\boldsymbol j-\boldsymbol k}$ at indices taken modulo $t_i$, so $a$ must depend only on residues. Replacing $j_i$ by $j_i+t_i$ changes the exponent by $-\pi i s_i\frac{(j_i+t_i)^2-j_i^2}{t_i}=-\pi i s_i(2j_i+t_i)$, which is an integral multiple of $2\pi i$ because $t_i$ is even, so $a$ has period $t_i$ in coordinate $i$.

In the right side, the coefficient of $u_{\boldsymbol k}$ in output position $\boldsymbol j$ is $\frac1T\overline a_{\boldsymbol j}a_{\boldsymbol j-\boldsymbol k}\overline a_{\boldsymbol k}$. Using $(j_i-k_i)^2=j_i^2-2j_ik_i+k_i^2$, the exponent of this product is $\sum_i\frac{\pi i s_i}{t_i}\bigl(j_i^2-(j_i^2-2j_ik_i+k_i^2)+k_i^2\bigr)$, where the squares cancel, so the coefficient is $\frac1T\overline a_{\boldsymbol j}a_{\boldsymbol j-\boldsymbol k}\overline a_{\boldsymbol k}=\frac1T\exp(2\pi i\sum_is_ij_ik_i/t_i)$.

This is precisely the coefficient of $QF_{\boldsymbol t}$ found above, because $Q$ reads frequency $(-s_ij_i)_i$ from the negative-sign transform. The permutation needs no separate step: it only changes which frequency is read, and the sign of the cross term $2j_ik_i$ already matches it.

For $d=1$, $t_1=2$, $s_1=1$: $a_0=1$, $a_1=-i$, and $j=k=1$ gives $\overline a_1a_0\overline a_1=i\cdot1\cdot i=-1=\exp(2\pi i\cdot\tfrac12)$, as required.

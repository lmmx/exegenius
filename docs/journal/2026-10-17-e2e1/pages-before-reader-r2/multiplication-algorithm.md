The multiplication algorithm is the full procedure that turns two $n$-bit integers into their exact $2n$-bit product. Key idea: the product is a cyclic convolution of short integer coefficients; a [Discrete Fourier transform](dft.md) turns it into pointwise multiplication; prime-length transforms are computed through power-of-two ones by resampling and chirps; and the total error stays below $1/2$, so rounding recovers exact integers.

## Setup

It takes its sizes from [Parameter and prime-length choice](parameter-choice.md): $b=\lceil\log_2 n\rceil$ bits per radix digit, $p=6b$ fractional bits for every stored value, distinct odd primes $s_i$ with product $S$, and powers of two $t_i$ just above $s_i$ with product $T$. Each input has $q=\lceil n/b\rceil$ radix-$2^b$ digits $a_j$. Their product polynomial has degree below $S$, so the product is reduced to a cyclic convolution modulo $x^S-1$ of digit arrays with integer coefficients below $2^{3b}$, by [Multiplication as radix-digit convolution](radix-digit-reduction.md). It rewrites a length-$S$ coefficient sequence by the [Chinese-remainder address map](crt-map.md) $\Phi$ as an array with one cyclic axis of each length $s_i$, preserving convolution, so that the Fourier transform acts one short axis at a time.

## Computed and exact transforms

A tilde marks what the machine computes in fixed point, truncating with $Q_p$ to $p$ fractional bits; the same symbol without it is the exact map approximated. An *exact counterpart* replaces every numerical subroutine by its exact map and omits truncations; [Error tracking and exact recovery](precision-recovery.md) bounds the gap.

By the [Bluestein chirp identity](chirp.md), with chirp array $a$ and target permutation $Q$, the transform $F_{\boldsymbol t}$ on the power-of-two axes satisfies, for an array $f$, $QF_{\boldsymbol t}f=\overline a\cdot\mathcal M_{\mathbb C}(a,\overline a\cdot f)$: multiply by a phase, convolve with the chirp, multiply by a phase. Here $\mathcal M_{\mathbb C}(f,g)=f*g/T$ is normalized cyclic convolution, computed through the [Phase twist](phase-twist.md) and [Convolution from the retained transform layout](ring-convolution.md). Take $f$ of norm at most $.6$ and let $\widetilde a$ be $a$ rounded to the $p$-bit grid, within $2\cdot2^{-p}$ of $a$. The computed chirp transform $\widetilde{\mathscr C}$ forms
$$f_0=Q_p(\overline{\widetilde a}\cdot f),\qquad f_1=\widetilde{\mathcal M}_{\mathbb C}(\widetilde a,f_0),\qquad \widetilde{\mathscr C}(f)=Q_p(\overline{\widetilde a}\cdot f_1).$$
Here $f_0$ is the data operand after the first endpoint phase, $f_1$ the computed convolution, and $\widetilde a$ the convolution kernel. Replacing the computed pieces by exact ones and dropping $Q_p$ turns the three lines into the right side of the chirp identity, so the exact counterpart of $\widetilde{\mathscr C}$ is $QF_{\boldsymbol t}$.

The algorithm needs $F_{\boldsymbol s}$, the discrete Fourier transform on the prime axes of lengths $s_i$, normalized by $1/S$. The computed version $\widetilde{\mathscr F}$, with exact counterpart $\mathscr F$, applies the [Gaussian resampling](gaussian-resampling.md) input map $\mathcal A$, then $\widetilde{\mathscr C}$, then the output map $\mathcal B_0$, then multiplies exactly by $2^\gamma$, the power of two that resampling introduces. Its exact counterpart is $\mathscr F=2^\gamma\mathcal B_0QF_{\boldsymbol t}\mathcal A=RF_{\boldsymbol s}$. The first equality substitutes each stage's exact counterpart; the second is the resampling identity between exact maps, with $R$ a retained permutation of coordinates. For the opposite sign, on an array $z$, $\widetilde{\mathscr F}^{+}(z)=\overline{\widetilde{\mathscr F}(\overline z)}$ has exact counterpart $RF_{\boldsymbol s}^{+}$, since $R$ is real and conjugation flips the exponent sign.

## The algorithm

Digit $a_j$ is stored with numerator $a_j2^{p-b-2}$ in a signed real word followed by a zero imaginary word, representing $a_j2^{-b-2}$, so the inputs have norm below $1/4$; $S-q$ zero records pad each array. Then:

1. Apply $\Phi$ to obtain arrays $u,v$ of exact $p$-bit values (the scaled digit arrays, not the integers).
2. Compute $\widetilde{\mathscr F}(u)$ and $\widetilde{\mathscr F}(v)$.
3. Multiply corresponding coefficients exactly and truncate toward zero to $p$ fractional bits, obtaining the array $z$.
4. Compute $\widetilde{\mathscr F}^{+}(z)$ and multiply by $S$, obtaining $\widetilde w$, which approximates the exact array $w$ defined below.
5. Multiply by $2^{2b+4}S$, round to the nearest integers, write each as a signed $(3b+1)$-bit word, and apply $\Phi^{-1}$ removing the padding.
6. Propagate radix-$2^b$ carries in increasing coefficient order and output exactly $2n$ bits.

The two scalings by $S$ undo different normalizations. Both $F_{\boldsymbol s}$ and $F_{\boldsymbol s}^{+}$ carry $1/S$, so $F_{\boldsymbol s}(u*v)=S\,F_{\boldsymbol s}u\cdot F_{\boldsymbol s}v$ and $F_{\boldsymbol s}^{+}F_{\boldsymbol s}$ is the identity divided by $S$. Permuting both factors commutes with pointwise products, and $RF_{\boldsymbol s}^{+}R=F_{\boldsymbol s}^{+}$ by [Retained permutation cancellation](retained-permutations.md), so $R$ drops out. The exact counterpart after the three transforms is $(u*v)/S^2$, so after the first scale it is $w=(u*v)/S$. The second scale gives $2^{2b+4}(u*v)$. The factor $2^{2b+4}$ cancels the $2^{-b-2}$ carried by each stored operand, and the inverse image under $\Phi$ is the original integer coefficient convolution <cite data-source="paper">The second scale gives $2^{2b+4}(u*v)$, whose inverse image under $\Phi$ is the original integer coefficient convolution.</cite>.

The error on the recovered coefficients is below $1/2$, as shown by [Error tracking and exact recovery](precision-recovery.md) from each normalized transform and product being a contraction. Rounding in step 5 therefore gives the exact integer coefficients, and carry propagation the exact binary product.

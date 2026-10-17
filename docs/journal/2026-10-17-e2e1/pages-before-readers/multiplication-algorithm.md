The multiplication algorithm chains the reductions of the earlier pages to turn two $n$-bit integers into their exact $2n$-bit product.

It takes its sizes from [Parameter and prime-length choice](parameter-choice.md): $b=\lceil\log_2 n\rceil$ is the digit width (the number of bits in one radix digit, not a lowest bit or kernel coefficient), $p=6b$ is the number of fractional bits, $s_i$ are distinct odd prime axis lengths with product $S$, and $t_i$ are power-of-two axis lengths with product $T$. Each input splits into $q=\lceil n/b\rceil$ radix-$2^b$ digits $a_j$, and the product is their convolution modulo $x^S-1$, as in [Multiplication as radix-digit convolution](radix-digit-reduction.md). The convolution is placed on the prime axes by the [Chinese-remainder address map](crt-map.md) $\Phi$.

## The source transform

Let $F_{\boldsymbol s}$ be the discrete Fourier transform on the axes $s_i$, normalized by $1/S$. $F_{\boldsymbol s}^+$ is the same with the opposite sign in the exponent, and $F_{\boldsymbol t}$ is the transform on the axes $t_i$, normalized by $1/T$.

Let $\mathcal M_{\mathbb C}(f,g)=f*g/T$ be the normalized cyclic convolution, computed through the [Phase twist](phase-twist.md) and [Convolution from the retained transform layout](ring-convolution.md). With the chirp array $a_{\boldsymbol j}=\exp(-\pi i\sum_i s_ij_i^2/t_i)$ and the target permutation $Q$, the [Bluestein chirp identity](chirp.md) gives $QF_{\boldsymbol t}x=\overline a\cdot\mathcal M_{\mathbb C}(a,\overline a\cdot x)$: phase, convolution with the chirp, phase. The computed version uses an approximation $\widetilde a$ of $a$ and truncation $Q_p$ to $p$ fractional bits. It forms the operand $x_0=Q_p(\overline{\widetilde a}\cdot x)$ after the first phase, then $y_0=\widetilde{\mathcal M}_{\mathbb C}(\widetilde a,x_0)$, the computed convolution, and returns $\widetilde{\mathscr C}(x)=Q_p(\overline{\widetilde a}\cdot y_0)$.

The computed source transform $\widetilde{\mathscr F}$ applies the [Gaussian resampling](gaussian-resampling.md) map $\mathcal A$, then $\widetilde{\mathscr C}$, then the output map $\mathcal B_0$, then multiplies exactly by $2^\gamma$, where $\gamma$ is the exponent of the power-of-two scale that resampling introduces, not the initial central values of the exchange network. Its exact counterpart $\mathscr F$ replaces each numerical routine by the exact map it approximates, so $\mathscr F=2^\gamma\mathcal B_0QF_{\boldsymbol t}\mathcal A=RF_{\boldsymbol s}$, where $R$ is a retained permutation of coordinates, not a scalar ring or row count. For an array $z$ (not a round input or suffix), the opposite-sign procedure is $\widetilde{\mathscr F}^{+}(z)=\overline{\widetilde{\mathscr F}(\overline z)}$; $R$ is real and conjugation flips the exponent sign, so its exact counterpart is $RF_{\boldsymbol s}^{+}$.

## The algorithm

Digit $a_j$ is stored as the grid value $a_j2^{-b-2}$, so inputs have norm below $1/4$, and the digits are padded with zero records to length $S$. Then:

1. Apply $\Phi$ to obtain source arrays $u,v$ (arrays, not integers or DFT inputs).
2. Compute $\widetilde{\mathscr F}(u)$ and $\widetilde{\mathscr F}(v)$.
3. Multiply corresponding coefficients and truncate to $p$ fractional bits, obtaining the array $z$.
4. Compute $\widetilde{\mathscr F}^{+}(z)$ and multiply by $S$, obtaining $\widetilde w$, which approximates the exact array $w=(u*v)/S$.
5. Multiply by $2^{2b+4}S$, round to the nearest integers, and apply $\Phi^{-1}$.
6. Propagate radix-$2^b$ carries in increasing coefficient order and output exactly $2n$ bits.

The two scalings by $S$ undo different normalizations. Each of $F_{\boldsymbol s}$ and $F_{\boldsymbol s}^{+}$ carries $1/S$, so $F_{\boldsymbol s}(u*v)=S\,F_{\boldsymbol s}u\cdot F_{\boldsymbol s}v$ and $F_{\boldsymbol s}^{+}F_{\boldsymbol s}$ is the identity divided by $S$; hence $F_{\boldsymbol s}^{+}(F_{\boldsymbol s}u\cdot F_{\boldsymbol s}v)=(u*v)/S^2$. Permutations commute with pointwise products, and $RF_{\boldsymbol s}^{+}R=F_{\boldsymbol s}^{+}$ by [Retained permutation cancellation](retained-permutations.md), so $R$ drops out. Steps 2 to 4 have exact counterpart $(u*v)/S^2$, and after the scale in step 4 it is $w$. Step 5 gives $2^{2b+4}(u*v)$. Each stored operand carries $2^{-b-2}$, so $u*v$ carries $2^{-2b-4}$, which the factor $2^{2b+4}$ cancels; $\Phi^{-1}$ then returns the integer digit convolution <cite data-source="paper">The second scale gives $2^{2b+4}(u*v)$, whose inverse image under $\Phi$ is the original integer coefficient convolution.</cite>.

Exactness is shown by [Error tracking and exact recovery](precision-recovery.md): the error on these coefficients is below $1/2$, so rounding is exact.

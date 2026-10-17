Error tracking and exact recovery shows that the computed convolution coefficients lie within $1/2$ of the true integers, so rounding recovers them.

The computation runs on [Fixed-point Gaussian dyadic records](fixed-point.md) with $p=6b$ fractional bits, where $b=\lceil\log_2n\rceil$ is the digit width. Write $h_p=2^{-p}$ and $L=\log_2T$. Norms are sup norms, an error of $a$ means error below $ah_p$, and every array stays in the unit disk (norm $\le1$). For large $n$, $S\le T<2^b$ and $L<b$. Each [synthetic transform](synthetic-transform.md) has error below $L$ <cite data-source="paper">$\sqrt2\ell\,2^{-p}<L2^{-p}$</cite>.

Errors add through the [Contractions and bilinear contractions](contraction-maps.md) making up each transform and product. For a bilinear contraction $\mathcal M$, replacing operands $f,g$ by $f',g'$ gives
$$\|\mathcal M(f',g')-\mathcal M(f,g)\|=\|\mathcal M(f'-f,g')+\mathcal M(f,g'-g)\|\le\|f'-f\|\|g'\|+\|f\|\|g'-g\|,$$
so for disk operands the two input errors add.

For a unit phase $e$, an approximation $\widetilde e$ with $|\widetilde e-e|<2h_p$ and a disk operand $x$, the product differs by under $2\|x\|h_p$ and truncation $Q_p$ adds under $\sqrt2h_p$, so $\|Q_p(\widetilde ex)-ex\|<(2\|x\|+\sqrt2)h_p<4h_p$. Also $\|Q_p(\widetilde ex)\|\le\|x\|$, since $|\widetilde e|\le1$.

The ring convolution $\mathcal M_r$ of [Phase twist](phase-twist.md) applies two forward synthetic transforms, a pointwise polynomial product divided by $r$ (truncation under $2$) and one opposite transform. Together they err by under $3L+2$. An exact multiplication by $M=T/r$ then gives $(f*g)/T$ and
$$E_{\rm ring}:=M(3L+2)<5TL$$
<cite data-source="paper">E_{\rm ring}:=M(3L+2)<5TL.</cite> With an operand of norm at most $.7$, the output norm stays below $.701$. The complex convolution adds two input twists and one output untwist, each under $4$, so it errs by under
$$(E_{\rm ring}+4+4+4)h_p<6TLh_p.$$

The computed chirp transform $\widetilde{\mathscr C}$ is built from the [Bluestein chirp identity](chirp.md): a phase product, a complex convolution with the chirp kernel, and a last phase product. With $\boldsymbol t$ the axis lengths, $F_{\boldsymbol t}$ the exact transform and $Q$ the target permutation, the errors $2$ (kernel), $4$, $6TL$ and $4$ give
$$\|\widetilde{\mathscr C}(x)-QF_{\boldsymbol t}x\|<(6TL+2+4+4)h_p<8TLh_p.$$

The computed source transform $\widetilde{\mathscr F}$ applies the expansion map $\mathcal A$ of [Gaussian resampling](gaussian-resampling.md), then $\widetilde{\mathscr C}$, then the compression map $\mathcal B_0$, then an exact scaling by $2^\gamma$. Each of $\mathcal A,\mathcal B_0$ is a contraction implemented along $d$ axes with error under $p^2$ each, so under $dp^2$. The error before scaling is under $2dp^2+8TL$. As $2dp^2<TL$ eventually, scaling gives error under $E_sh_p$ with
$$E_s=9\cdot2^\gamma TL$$
<cite data-source="paper">E_s=9\,2^\gamma TL,</cite> where $b\ge2^{24}$ gives $\gamma\le b/4$ <cite data-source="paper">$b\ge2^{24}$ implies $\gamma\le b/4$</cite> and keeps $E_sh_p<10^{-3}$.

Now $u,v$ are scaled digit arrays: each digit is stored as its value times $2^{-b-2}$, so norms are below $1/4$. Let $U=RF_{\boldsymbol s}u$ and $V_1=RF_{\boldsymbol s}v$ be the exact source transforms and $\widetilde U,\widetilde V_1$ the computed ones, of norm under $1/4+E_sh_p<.26$. Then $z=Q_p(\widetilde U\cdot\widetilde V_1)$ has norm under $.26^2<.5$, and
$$\widetilde U\cdot\widetilde V_1-U\cdot V_1=(\widetilde U-U)\cdot\widetilde V_1+U\cdot(\widetilde V_1-V_1)$$
has two terms under $E_sh_p$; truncation adds under $2h_p$. So $\|z-U\cdot V_1\|<(2E_s+2)h_p$.

With $F_{\boldsymbol s}$ the transform with factor $1/S$ and $F_{\boldsymbol s}^+$ its opposite-sign version, the retained permutation cancels ([Retained permutation cancellation](retained-permutations.md)) and the exact opposite transform $G=RF_{\boldsymbol s}^+$ gives $G(U\cdot V_1)=(u*v)/S^2$ <cite data-source="paper">=\frac{u*v}{S^2}.</cite> The computed opposite transform is within $E_sh_p$ of $Gz$ and $G$ is a contraction, so
$$\|\widetilde{\mathscr F}^{+}(z)-G(U\cdot V_1)\|\le\|\widetilde{\mathscr F}^{+}(z)-Gz\|+\|G(z-U\cdot V_1)\|<(3E_s+2)h_p.$$
Exact multiplication by $S$ makes $\widetilde w$ approximate $w=(u*v)/S$ with error under $E_wh_p$, where
$$E_w:=S(3E_s+2)<28\cdot2^\gamma T^2L.$$

The integer coefficients are $2^{2b+4}Sw$: $2^{2b+4}$ undoes the two digit scales and $S$ the remaining $1/S$. With $h_p=2^{-6b}$, $S\le T<2^b$, $L<b$ and $\gamma\le b/4$, the absolute error is under
$$2^{2b+4}S\cdot28\cdot2^\gamma T^2L\cdot2^{-6b}<448b\,2^{-b+\gamma}\le448b\,2^{-3b/4}<\tfrac12$$
<cite data-source="paper">\le448b\,2^{-3b/4}<\frac12.</cite> Here $2^{2b+4}ST^2<2^{5b+4}$ uses about $5b$ of the $6b$ fractional bits. The true coefficients are integers, so error strictly below $1/2$ makes the nearest integer the unique correct coefficient.

Carry propagation rebuilds the binary product from the coefficients $c_j$ of the [Multiplication as radix-digit convolution](radix-digit-reduction.md), indexed by $j$, with $c_j\le q(2^b-1)^2$ and $q\le2^b$. Put $Q_0=2^b$ and $k_0=0$; $k_j$ is the carry into position $j$ and $d_j$ the $j$-th output digit:
$$d_j=(c_j+k_j)\bmod Q_0,\qquad k_{j+1}=\lfloor(c_j+k_j)/Q_0\rfloor.$$
Then $0\le k_j\le q(Q_0-1)<Q_0^2$: if this holds at $j$, then $c_j+k_j\le q(Q_0-1)^2+q(Q_0-1)=qQ_0(Q_0-1)$. Since $c_j+k_j=d_j+Q_0k_{j+1}$, adding $c_jQ_0^j$ carries the invariant
$$\sum_{h<j}c_hQ_0^h=\sum_{h<j}d_hQ_0^h+k_jQ_0^j$$
from $j$ to $j+1$ (it holds at $j=0$). Unprocessed terms are multiples of $Q_0^j$, so the first $j$ radix digits are fixed. The product is below $2^{2n}$, so all bits above position $2n-1$ are zero and exactly $2n$ bits are output.

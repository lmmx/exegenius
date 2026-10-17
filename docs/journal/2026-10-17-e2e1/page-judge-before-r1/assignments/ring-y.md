**This page derives no steps of the argument: there are no items to judge. Give an empty `items` list, and judge the page's mathematical errors only.**

## This page

Concept: Polynomial ring modulo y^r + 1
Gloss: Polynomials of degree below r over the complex numbers with y^r = −1, so multiplying by a power of y is a signed shift of the coefficient list.
Also called: negacyclic ring
Heading: Polynomial ring modulo $y^r + 1$

The symbols in this page's heading denote (say so on the page):

- $y$ (ring variable): The indeterminate of the polynomials; it satisfies y^r = −1 in the ring.
- $r$ (record length): The number of complex coefficients in a polynomial of the ring, always a power of two.
## Where the argument relies on this page

- Synthetic transforms over $\mathcal R_r$ on $d-1$ axes of lengths $t_i\in\{2^{\ell-1},2^\ell\}$ with outputs kept in a retained layout $LB$ cost $O(Tp[dK+pK^{\tau-1}+\ell d^{\lambda'}])$ with error less than $\sqrt2\ell\,2^{-p}$. (on [Tape layout for synthetic transforms](transform-layout.md))
- The synthetic transform $F_t^-$ over $\mathcal R_r=\mathbb C[y]/(y^r+1)$, with roots of unity $\omega_t=y^{2r/t}$, is computed by decimation in frequency with outputs written in bit-reversed order, $B_tF_t^-$; the pointwise product keeps this order and the reverse procedure computes $F_t^+B_t^{-1}$, so no reordering is executed. (on [Retained frequency order](frequency-order.md))
- Normalized cyclic convolution over the polynomial ring $\mathbb C[y]/(y^r+1)$ on the remaining $d-1$ axes is computed from two forward synthetic transforms in retained order, $M$ pointwise polynomial products and the opposite transform, in $O(Tp[dK+pK^{\tau-1}+\ell d^{\lambda'}+\log(rp)])$ time with error less than $\sqrt2\,M(3\ell+1)2^{-p}$. (on [Convolution from the retained transform layout](ring-convolution.md))
- For disk-grid polynomials $f,g\in\mathcal R_r$ the exact Gaussian-integer numerators of $fg$, and then $Q_p(fg/r)$, are computed in $O(rp\log(rp))$ time using four integer products by the established multiplier at packing base $B_0=2^{4p}$. (on [Signed packing](signed-packing.md))
- With $\zeta=e^{\pi i/r}$ the map $\Theta:\mathbb C[x_d]/(x_d^r-1)\to\mathcal R_r$, $x_d\mapsto\zeta y$, is an isomorphism multiplying coefficient $k$ by $\zeta^k$; hence normalized complex cyclic convolution is $\mathcal M_{\mathbb C}(f,g)=D_-\iota^{-1}\mathcal M_r(\iota D_+f,\iota D_+g)=f*g/T$. (on [Phase twist](phase-twist.md))

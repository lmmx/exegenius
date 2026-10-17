**This page derives no steps of the argument: there are no items to judge. Give an empty `items` list, and judge the page's mathematical errors only.**

## This page

Concept: Polynomial ring modulo y^r + 1
Gloss: The ring of complex polynomials of degree < r with y^r = −1, in which multiplying by a power of y permutes the coefficients up to sign, so that powers of y serve as exact roots of unity.
Also called: negacyclic polynomial ring, R_r
Heading: Polynomial ring modulo $y^r + 1$

The symbols in this page's heading denote (say so on the page):

- $y$ (ring variable): The formal polynomial variable, whose powers y^a act as roots of unity of order 2r.
- $r$ (record length): The number of complex coefficients in a polynomial record, a power of two.
## Where the argument relies on this page

- Fast simultaneous butterfly layer: $H_0(u,v)=((u+v)/2,(u-v)/2)$ applied coefficientwise at the same bit offset in $D\le d$ address chunks of a polynomial-record array costs $O(Vd^{\lambda'})$ with $\lambda'=1-2^{-54}$, evaluating the whole layer exactly before a single truncation. (on [Fast simultaneous butterfly layer](fast-butterfly-layer.md))
- Convolution over $\mathbb C[y]/(y^r+1)$: the normalized convolution of arrays on $d-1$ power-of-two axes with ring entries is computed in $O(Tp[dK+pK^{\tau-1}+\ell d^{\lambda'}+\log(rp)])$ time with error less than $\sqrt2\,M(3\ell+1)2^{-p}$. (on [Convolution over the polynomial ring](ring-convolution.md))
- Synthetic transform: over $\mathcal R_r=\mathbb C[y]/(y^r+1)$, with $\omega_t=y^{2r/t}$ a signed coefficient shift, $F_t^\pm$ satisfy $F_t^+F_t^-=I/t$, are contractions, and are computed by radix-two decimation-in-frequency rounds with twiddle monomials that leave the outputs in bit-reversed order, which the opposite transform consumes. (on [Synthetic polynomial transform](synthetic-transform.md))
- Signed packing: for disk-grid polynomials $f,g\in\mathcal R_r$ one computes the exact Gaussian-integer numerators of $fg$ and returns $Q_p(fg/r)$ in $O(rp\log(rp))$ time with four products of $4pr$-bit integers by the established multiplier. (on [Signed packing of polynomial products](signed-packing.md))
- Coefficient twist: $\Theta:\mathbb C[x_d]/(x_d^r-1)\to\mathcal R_r$, $x_d\mapsto\zeta y$ with $\zeta=e^{\pi i/r}$, multiplies coefficient $k$ by $\zeta^k$, and the negative wrap of $\mathcal R_r$ becomes the original cyclic wrap after untwisting; hence $\mathcal M_{\mathbb C}(f,g)=D_-\iota^{-1}\mathcal M_r(\iota D_+f,\iota D_+g)=(f*g)/T$. (on [Coefficient (phase) twist](phase-twist.md))

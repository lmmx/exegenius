**This page derives no steps of the argument: there are no items to judge. Give an empty `items` list, and judge the page's mathematical errors only.**

## This page

Concept: Fixed-point disk-grid arithmetic
Gloss: Complex values stored as 2^{−p}(a+ib) with signed two's-complement words in the unit disk, exact Gaussian-dyadic intermediate results with guard bits, and one truncation Q_p toward zero that costs error less than √2·2^{−p} per coefficient while never increasing the norm.
Also called: disk grid, Gaussian dyadics, truncation Q_p
## Where the argument relies on this page

- Fast simultaneous butterfly layer: $H_0(u,v)=((u+v)/2,(u-v)/2)$ applied coefficientwise at the same bit offset in $D\le d$ address chunks of a polynomial-record array costs $O(Vd^{\lambda'})$ with $\lambda'=1-2^{-54}$, evaluating the whole layer exactly before a single truncation. (on [Fast simultaneous butterfly layer](fast-butterfly-layer.md))
- Convolution over $\mathbb C[y]/(y^r+1)$: the normalized convolution of arrays on $d-1$ power-of-two axes with ring entries is computed in $O(Tp[dK+pK^{\tau-1}+\ell d^{\lambda'}+\log(rp)])$ time with error less than $\sqrt2\,M(3\ell+1)2^{-p}$. (on [Convolution over the polynomial ring](ring-convolution.md))
- Synthetic transform: over $\mathcal R_r=\mathbb C[y]/(y^r+1)$, with $\omega_t=y^{2r/t}$ a signed coefficient shift, $F_t^\pm$ satisfy $F_t^+F_t^-=I/t$, are contractions, and are computed by radix-two decimation-in-frequency rounds with twiddle monomials that leave the outputs in bit-reversed order, which the opposite transform consumes. (on [Synthetic polynomial transform](synthetic-transform.md))
- Signed packing: for disk-grid polynomials $f,g\in\mathcal R_r$ one computes the exact Gaussian-integer numerators of $fg$ and returns $Q_p(fg/r)$ in $O(rp\log(rp))$ time with four products of $4pr$-bit integers by the established multiplier. (on [Signed packing of polynomial products](signed-packing.md))
- Gaussian resampling with the permutations left in the transform: the Fourier transform of prime lengths $s_i$ is expressed as $RF_{\boldsymbol s}=2^\gamma\mathcal B_0QF_{\boldsymbol t}\mathcal A$ through a power-of-two transform, with ordered numerical maps, error less than $dp^2$ and cost $O(dTp^{3/2+\delta}\alpha)$. (on [Gaussian resampling](gaussian-resampling.md))
- Exact recovery: rescaling the computed product by the known factors $S$ and $2^{2b+4}$ leaves coefficient error below $448b\,2^{-3b/4}<1/2$, so rounding yields the exact integer coefficients $c_j<2^{3b}$, and radix-$2^b$ carries produce the $2n$-bit product. (on [Rounding and carry propagation](exact-recovery.md))

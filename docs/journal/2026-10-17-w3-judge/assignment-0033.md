## This page in the argument

The step: Permutation cancellation: the middle factor $QF_{\boldsymbol t}$ is computed by a chirp convolution without executing $Q$, and the source permutation $R$ cancels across the two source transforms, the pointwise product and the opposite transform: $RF^+_{\boldsymbol s}[(RF_{\boldsymbol s}u)\cdot(RF_{\boldsymbol s}v)]=(u*v)/S^2$.
How: Convert the target permutation into the chirp convolution and show that $R$ commutes with pointwise products and satisfies $RF^+_{\boldsymbol s}R=F^+_{\boldsymbol s}$, so the result is the ordinary convolution.
Why: The sources say the chirp identity performs the frequency change in $Q$ without applying $Q$ as a separate permutation of the array. (the sources say so: "in $Q$ without applying $Q$ as a separate permutation of the array." (paper))

Its sub-steps, in order:

- Bluestein chirp identity: with $a_{\boldsymbol j}=\exp(-\pi i\sum_is_ij_i^2/t_i)$ (periodic of period $t_i$), the permuted transform is $QF_{\boldsymbol t}u=\overline a\cdot(a*(\overline a\cdot u))/T$, a chirp-weighted normalized convolution. (on [Bluestein chirp identity](bluestein-chirp.md))
- For a unit $c$ modulo $q$ and $(R_cz)_j=z_{cj}$, substituting $h=ck$ gives $F_q^+R_c=R_c^{-1}F_q^+$; each $t_i$ is a unit modulo $s_i$, so $F^+_{\boldsymbol s}R=R^{-1}F^+_{\boldsymbol s}$ and $RF^+_{\boldsymbol s}R=F^+_{\boldsymbol s}$.
- Since $R$ commutes with pointwise products, $RF^+_{\boldsymbol s}[(RF_{\boldsymbol s}u)\cdot(RF_{\boldsymbol s}v)]=F^+_{\boldsymbol s}(F_{\boldsymbol s}u\cdot F_{\boldsymbol s}v)$, and by $F_{\boldsymbol s}(u*v)=S(F_{\boldsymbol s}u)(F_{\boldsymbol s}v)$ and $F^+_{\boldsymbol s}F_{\boldsymbol s}=I/S$ this equals $\frac1SF^+_{\boldsymbol s}F_{\boldsymbol s}(u*v)=\frac{u*v}{S^2}$.

It serves these steps, from the main result down:

- A single deterministic multitape Turing machine multiplies two $n$-bit integers exactly in $O(n(\lg n)^{1-\kappa})$ worst-case time with $\kappa=2^{-182}$.; how: Reduce the product to a convolution of short integer coefficients, evaluate the convolution through power-of-two transforms over $\mathbb C[y]/(y^r+1)$, and replace the two tape operations that each cost a factor $\log n$ (address rearrangement and butterfly layers) by procedures whose cost per bit is a power of their parameter below one; then total the costs. (on [Integer multiplication below n log n](main-theorem.md))
## Steps this page derives

The argument gives this page these sub-steps of its step (S7.2, S7.3). Derive each here, so the reader can check how it follows from what comes before: stating its result, or citing the source for it, does not derive it. The passages report each step in the source; they ground the derivation and are not to be copied onto the page. In `derivations`, give each step's id and the passage on the page that derives it, copied exactly: submit checks that the passage is on the page, and the audit judges whether it is enough.

**S7.2**: For a unit $c$ modulo $q$ and $(R_cz)_j=z_{cj}$, substituting $h=ck$ gives $F_q^+R_c=R_c^{-1}F_q^+$; each $t_i$ is a unit modulo $s_i$, so $F^+_{\boldsymbol s}R=R^{-1}F^+_{\boldsymbol s}$ and $RF^+_{\boldsymbol s}R=F^+_{\boldsymbol s}$.
Source: "Substituting $h=ck$" (paper)
> For a unit $c$ modulo $q$, let $(R_cz)_j=z_{cj}$. Substituting $h=ck$ in the Fourier sum gives \[ (F_q^+R_cz)_j =\frac1q\sum_{h=0}^{q-1}z_h e^{2\pi i h c^{-1}j/q} =(R_c^{-1}F_q^+z)_j. \] Each $t_i$ is a unit modulo $s_i$, so tensoring this identity gives $F_{\boldsymbol s}^+R=R^{-1}F_{\boldsymbol s}^+$ and hence $RF_{\boldsymbol s}^+R=F_{\boldsymbol s}^+$. Also $(RF_{\boldsymbol s}u)\mathbin{\cdot}(RF_{\boldsymbol s}v) =R(F_{\boldsymbol s}u\mathbin{\cdot}F_{\boldsymbol s}v)$, because a coordinate permutation commutes with pointwise products. Therefore \[ RF_{\boldsymbol s}^+\bigl[R(F_{\boldsymbol s}u\mathbin{\cdot} F_{\boldsymbol s}v)\bigr] =F_{\boldsymbol s}^+(F_{\boldsymbol s}u\mathbin{\cdot} F_{\boldsymbol s}v). \

**S7.3**: Since $R$ commutes with pointwise products, $RF^+_{\boldsymbol s}[(RF_{\boldsymbol s}u)\cdot(RF_{\boldsymbol s}v)]=F^+_{\boldsymbol s}(F_{\boldsymbol s}u\cdot F_{\boldsymbol s}v)$, and by $F_{\boldsymbol s}(u*v)=S(F_{\boldsymbol s}u)(F_{\boldsymbol s}v)$ and $F^+_{\boldsymbol s}F_{\boldsymbol s}=I/S$ this equals $\frac1SF^+_{\boldsymbol s}F_{\boldsymbol s}(u*v)=\frac{u*v}{S^2}$.
Source: "=\frac{u*v}{S^2}," (paper)
> ot}(RF_{\boldsymbol s}v) =R(F_{\boldsymbol s}u\mathbin{\cdot}F_{\boldsymbol s}v)$, because a coordinate permutation commutes with pointwise products. Therefore \[ RF_{\boldsymbol s}^+\bigl[R(F_{\boldsymbol s}u\mathbin{\cdot} F_{\boldsymbol s}v)\bigr] =F_{\boldsymbol s}^+(F_{\boldsymbol s}u\mathbin{\cdot} F_{\boldsymbol s}v). \] The normalized convolution rule is $F_{\boldsymbol s}(u*v)=S(F_{\boldsymbol s}u)\mathbin{\cdot} (F_{\boldsymbol s}v)$, while $F_{\boldsymbol s}^+F_{\boldsymbol s}=I/S$. The two normalizations give \[ F_{\boldsymbol s}^+ (F_{\boldsymbol s}u\mathbin{\cdot}F_{\boldsymbol s}v) =\frac1S F_{\boldsymbol s}^+F_{\boldsymbol s}(u*v) =\frac{u*v}{S^2}, \] proving \eqref{eq:source-permutation-cancel}. Finally, conjugation changes the negative Fourier sign to the positive one and commutes with the real permutation $R$. It preserves both the disk grid and the norm, so conjugating the input and output also preserves an algorithm's error bound. \end{proof} \section{Exact integer multiplication and its cost} \label{sec:assembly}

### Steps derived on other pages

Where this page uses one, state its result and link the page that derives it. Do not derive it here, even where a passage above proves it.

- S7.1 (on [Bluestein chirp identity](bluestein-chirp.md)): Bluestein chirp identity: with $a_{\boldsymbol j}=\exp(-\pi i\sum_is_ij_i^2/t_i)$ (periodic of period $t_i$), the permuted transform is $QF_{\boldsymbol t}u=\overline a\cdot(a*(\overline a\cdot u))/T$, a chirp-weighted normalized convolution.

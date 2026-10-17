## This page in the argument

The step: For even $t_i$ and the chirp $a_{\boldsymbol j}=\exp(-\pi i\sum_is_ij_i^2/t_i)$ one has $QF_{\boldsymbol t}u=\overline a\cdot\bigl(a*(\overline a\cdot u)\bigr)/T$, so the target permutation $Q$ is absorbed into a cyclic convolution of length $T$ and never executed.
How: Expand the coefficient of $u_{\boldsymbol k}$ in the right side using $(j-k)^2=j^2-2jk+k^2$, so that the three chirp phases combine to $\exp(2\pi i\sum s_ij_ik_i/t_i)$, which is the coefficient of $QF_{\boldsymbol t}$ because $Q$ reads frequency $-s_ij_i$.
Why: Bluestein's identity turns a (permuted) discrete Fourier transform into pointwise phases around a convolution; the phases depend on $s_ij_i^2/t_i$ and the permutation $Q$ merely changes which frequency is read, which the identity already includes. (the sources leave this for the reader to work out: spell it out)

Its sub-steps, in order:

- Define $a_{\boldsymbol j}=\exp\left(-\pi i\sum_{i=1}^d s_ij_i^2/t_i\right)$.
- Replacing $j_i$ by $j_i+t_i$ changes the exponent by $-\pi is_i(2j_i+t_i)$, an integral multiple of $2\pi i$ because $t_i$ is even, so $a$ has period $t_i$ in coordinate $i$.
- The coefficient of $u_{\boldsymbol k}$ in the right side is $\frac1T\overline a_{\boldsymbol j}a_{\boldsymbol j-\boldsymbol k}\overline a_{\boldsymbol k}=\frac1T\exp(2\pi i\sum_is_ij_ik_i/t_i)$.
- This is precisely the coefficient in $QF_{\boldsymbol t}$, because $Q$ reads frequency $(-s_ij_i)_i$ from the negative-sign transform.

It serves these steps, from the main result down:

- A single deterministic Turing machine with a fixed finite alphabet and a fixed finite number of one-dimensional tapes multiplies two $n$-bit integers exactly in $O(n(\lg n)^{1-\kappa})$ worst-case time with $\kappa=2^{-182}$, disproving the $n\log n$ optimality conjecture of Sch\"onhage and Strassen in this model.; how: Build two power-saving tape procedures (chunk interchange and simultaneous butterfly layer) from fixed finite linear networks; use them to accelerate the synthetic polynomial transforms and ring convolution; use Gaussian resampling with retained permutations and chirps to reduce the prime-length cyclic convolution of the radix-digit coefficients to those power-of-two transforms; assemble the exact product with a proof that the error is below $1/2$; and total the cost to show a power of $\log n$ is saved. (on [Power-saving integer multiplication](main-result.md))
- The Fourier transforms of pairwise coprime prime lengths $s_i$ that the integer product needs are computed through power-of-two lengths $t_i$ by Gaussian resampling and Bluestein chirps, with the source and target coordinate permutations left inside the transform identities, so that none is executed as a sort.; how: Retain both permutations of Gaussian resampling in the transform formula: the target permutation $Q$ is absorbed by the chirp convolution, and the source permutation $R$ cancels across the two forward transforms, their pointwise product and the opposite transform. (on [Retained permutation cancellation](retained-permutations.md))
## Steps this page derives

The argument gives this page these sub-steps of its step (S4.3.1, S4.3.2, S4.3.3, S4.3.4). Derive each here, so the reader can check how it follows from what comes before: stating its result, or citing the source for it, does not derive it. The passages report each step in the source; they ground the derivation and are not to be copied onto the page. In `derivations`, give each step's id and the passage on the page that derives it, copied exactly: submit checks that the passage is on the page, and the audit judges whether it is enough.

**S4.3.1**: Define $a_{\boldsymbol j}=\exp\left(-\pi i\sum_{i=1}^d s_ij_i^2/t_i\right)$.
Source: "=\exp\left(-\pi i\sum_{i=1}^d\frac{s_i j_i^2}{t_i}\right)." (paper)
> \begin{lemma}[Chirp and permutation cancellation] \label{lem:chirp-permutation} With $R,Q,s_i,t_i$ as in Lemma~\ref{lem:tensor-resampling}, assume every $t_i$ is even. For $\boldsymbol j\in\mathbb Z^d$, define \[ a_{\boldsymbol j} =\exp\left(-\pi i\sum_{i=1}^d\frac{s_i j_i^2}{t_i}\right). \] Then $a$ has period $t_i$ in coordinate $i$, and \begin{equation} QF_{\boldsymbol t}u =\overline a\mathbin{\cdot} \frac{a*(\overline a\mathbin{\cdot}u)}{T}. \label{eq:permuted-chirp} \end{equation} If $S=\prod_i s_i$, then \begin{equation} (RF_{\boldsymbol s}^+) \bigl[(RF_{\boldsymbol s}u)\mathbin{\cdot} (RF_{\boldsymbol s}v)\bigr] =\frac{u*v}{S^2}. \label{eq:source-permutation-cancel} \end{equation} Complex conjugation of the input and output of an algorithm for $RF_{\boldsymbol s}$ gives an algorithm for $RF_{\boldsymbol s}^+$. \end{lemma}

**S4.3.2**: Replacing $j_i$ by $j_i+t_i$ changes the exponent by $-\pi is_i(2j_i+t_i)$, an integral multiple of $2\pi i$ because $t_i$ is even, so $a$ has period $t_i$ in coordinate $i$.
Source: "Replacing $j_i$ by $j_i+t_i$ changes the exponent defining $a$ by" (paper)
> \begin{proof} Replacing $j_i$ by $j_i+t_i$ changes the exponent defining $a$ by $-\pi i s_i(2j_i+t_i)$, an integral multiple of $2\pi i$. The coefficient of $u_{\boldsymbol k}$ in the right side of \eqref{eq:permuted-chirp} is \[ \frac1T\overline a_{\boldsymbol j} a_{\boldsymbol j-\boldsymbol k} \overline a_{\boldsymbol k} =\frac1T\exp\left(2\pi i\sum_i \frac{s_i j_i k_i}{t_i}\right). \] This is precisely the coefficient in $QF_{\boldsymbol t}$, because $Q$ reads frequency $(-s_i j_i)_i$ from the negative-sign transform.

**S4.3.3**: The coefficient of $u_{\boldsymbol k}$ in the right side is $\frac1T\overline a_{\boldsymbol j}a_{\boldsymbol j-\boldsymbol k}\overline a_{\boldsymbol k}=\frac1T\exp(2\pi i\sum_is_ij_ik_i/t_i)$.
Source: "=\frac1T\exp\left(2\pi i\sum_i" (paper)
> \begin{proof} Replacing $j_i$ by $j_i+t_i$ changes the exponent defining $a$ by $-\pi i s_i(2j_i+t_i)$, an integral multiple of $2\pi i$. The coefficient of $u_{\boldsymbol k}$ in the right side of \eqref{eq:permuted-chirp} is \[ \frac1T\overline a_{\boldsymbol j} a_{\boldsymbol j-\boldsymbol k} \overline a_{\boldsymbol k} =\frac1T\exp\left(2\pi i\sum_i \frac{s_i j_i k_i}{t_i}\right). \] This is precisely the coefficient in $QF_{\boldsymbol t}$, because $Q$ reads frequency $(-s_i j_i)_i$ from the negative-sign transform.

**S4.3.4**: This is precisely the coefficient in $QF_{\boldsymbol t}$, because $Q$ reads frequency $(-s_ij_i)_i$ from the negative-sign transform.
Source: "This is precisely the coefficient in $QF_{\boldsymbol t}$, because" (paper)
> \begin{proof} Replacing $j_i$ by $j_i+t_i$ changes the exponent defining $a$ by $-\pi i s_i(2j_i+t_i)$, an integral multiple of $2\pi i$. The coefficient of $u_{\boldsymbol k}$ in the right side of \eqref{eq:permuted-chirp} is \[ \frac1T\overline a_{\boldsymbol j} a_{\boldsymbol j-\boldsymbol k} \overline a_{\boldsymbol k} =\frac1T\exp\left(2\pi i\sum_i \frac{s_i j_i k_i}{t_i}\right). \] This is precisely the coefficient in $QF_{\boldsymbol t}$, because $Q$ reads frequency $(-s_i j_i)_i$ from the negative-sign transform.

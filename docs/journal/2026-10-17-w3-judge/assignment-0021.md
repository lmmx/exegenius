## This page in the argument

The step: Integer product as digit convolution: writing each input as $q=\lceil n/b\rceil$ digits in base $2^b$, the product polynomial has degree at most $2q-2<2n/b\le T/2<S$ and coefficients $c_j\le q(2^b-1)^2<2^{3b}$, so cyclic convolution modulo $x^S-1$ has no wraparound.
How: Count degrees and bound a sum of at most $q\le n\le2^b$ products of digits below $2^b$.
Why: Because the zero-padded length exceeds the product degree, cyclic and ordinary convolution agree. (the sources leave this for the reader to work out: spell it out)

Its sub-steps, in order:

- Write the two inputs as polynomials in $2^b$, each with $q=\lceil n/b\rceil$ nonnegative digits.
- The product has degree at most $2q-2<2n/b\le T/2<S$, so convolution modulo $x^S-1$ has no nonzero wraparound.
- Every product coefficient obeys $0\le c_j\le q(2^b-1)^2<2^{3b}$ since $q\le n\le2^b$.

It serves these steps, from the main result down:

- A single deterministic multitape Turing machine multiplies two $n$-bit integers exactly in $O(n(\lg n)^{1-\kappa})$ worst-case time with $\kappa=2^{-182}$.; how: Reduce the product to a convolution of short integer coefficients, evaluate the convolution through power-of-two transforms over $\mathbb C[y]/(y^r+1)$, and replace the two tape operations that each cost a factor $\log n$ (address rearrangement and butterfly layers) by procedures whose cost per bit is a power of their parameter below one; then total the costs. (on [Integer multiplication below n log n](main-theorem.md))
- The multiplication algorithm: form base-$2^b$ digits, place them on distinct prime cyclic axes by the Chinese-remainder map $\Phi$, run two source transforms (Gaussian resampling plus chirp convolution through a twisted ring convolution), multiply pointwise, run the opposite transform, rescale, round and propagate carries, obtaining the exact $2n$-bit product in $O(Tp)$ steps beyond the layer costs.; how: Choose compatible parameters, reduce the product to a cyclic convolution of digit coefficients, evaluate the convolution through Gaussian-resampled transforms with all permutations left in the formulas, and recover exact integer coefficients from the approximations by an error bound below one half. (on [The multiplication algorithm](multiplication-algorithm.md))
## Steps this page derives

The argument gives this page these sub-steps of its step (S8.2.1, S8.2.2, S8.2.3). Derive each here, so the reader can check how it follows from what comes before: stating its result, or citing the source for it, does not derive it. The passages report each step in the source; they ground the derivation and are not to be copied onto the page. In `derivations`, give each step's id and the passage on the page that derives it, copied exactly: submit checks that the passage is on the page, and the audit judges whether it is enough.

**S8.2.1**: Write the two inputs as polynomials in $2^b$, each with $q=\lceil n/b\rceil$ nonnegative digits.
Source: "Write the two inputs as polynomials in $2^b$, each with" (paper)
> Write the two inputs as polynomials in $2^b$, each with $q=\lceil n/b\rceil$ nonnegative digits. Their product has degree at most \[ 2q-2<2n/b\le T/2<S. \] Thus convolution modulo $x^S-1$ has no nonzero wraparound. Every product coefficient obeys \begin{equation}\label{eq:capacity} 0\le c_j\le q(2^b-1)^2<2^{3b}, \end{equation} since $q\le n\le2^b$.

**S8.2.2**: The product has degree at most $2q-2<2n/b\le T/2<S$, so convolution modulo $x^S-1$ has no nonzero wraparound.
Source: "2q-2<2n/b\le T/2<S." (paper)
> Write the two inputs as polynomials in $2^b$, each with $q=\lceil n/b\rceil$ nonnegative digits. Their product has degree at most \[ 2q-2<2n/b\le T/2<S. \] Thus convolution modulo $x^S-1$ has no nonzero wraparound. Every product coefficient obeys \begin{equation}\label{eq:capacity} 0\le c_j\le q(2^b-1)^2<2^{3b}, \end{equation} since $q\le n\le2^b$.

**S8.2.3**: Every product coefficient obeys $0\le c_j\le q(2^b-1)^2<2^{3b}$ since $q\le n\le2^b$.
Source: "0\le c_j\le q(2^b-1)^2<2^{3b}," (paper)
> Write the two inputs as polynomials in $2^b$, each with $q=\lceil n/b\rceil$ nonnegative digits. Their product has degree at most \[ 2q-2<2n/b\le T/2<S. \] Thus convolution modulo $x^S-1$ has no nonzero wraparound. Every product coefficient obeys \begin{equation}\label{eq:capacity} 0\le c_j\le q(2^b-1)^2<2^{3b}, \end{equation} since $q\le n\le2^b$.

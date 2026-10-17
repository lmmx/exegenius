## This page in the argument

The step: Writing the two inputs in radix $2^b$ with $q=\lceil n/b\rceil$ digits each, the product polynomial has degree at most $2q-2<2n/b\le T/2<S$, so convolution modulo $x^S-1$ has no wraparound, and every coefficient $c_j\le q(2^b-1)^2<2^{3b}$.
How: Count degrees and bound the coefficients: a product of two polynomials with $q$ digits below $2^b$ has degree $2q-2$ and coefficients that are sums of at most $q$ products.
Why: Splitting an integer into radix digits turns the product into a convolution of short integer coefficients; the cyclic convolution of length $S$ equals the ordinary product as long as the degree stays below $S$, and the coefficient bound shows rounding errors below 1/2 will identify them. (the sources leave this for the reader to work out: spell it out)

Its sub-steps, in order:

- Write the two inputs as polynomials in $2^b$, each with $q=\lceil n/b\rceil$ nonnegative digits; their product has degree at most $2q-2$.
- Since $2q-2<2n/b\le T/2<S$, convolution modulo $x^S-1$ has no nonzero wraparound.
- Every product coefficient obeys $0\le c_j\le q(2^b-1)^2<2^{3b}$, since $q\le n\le2^b$.

It serves these steps, from the main result down:

- A single deterministic Turing machine with a fixed finite alphabet and a fixed finite number of one-dimensional tapes multiplies two $n$-bit integers exactly in $O(n(\lg n)^{1-\kappa})$ worst-case time with $\kappa=2^{-182}$, disproving the $n\log n$ optimality conjecture of Sch\"onhage and Strassen in this model.; how: Build two power-saving tape procedures (chunk interchange and simultaneous butterfly layer) from fixed finite linear networks; use them to accelerate the synthetic polynomial transforms and ring convolution; use Gaussian resampling with retained permutations and chirps to reduce the prime-length cyclic convolution of the radix-digit coefficients to those power-of-two transforms; assemble the exact product with a proof that the error is below $1/2$; and total the cost to show a power of $\log n$ is saved. (on [Power-saving integer multiplication](main-result.md))
- The exact $2n$-bit product of the two inputs is obtained from the radix-digit convolution by the chain: radix digits, Chinese-remainder address map to a product of prime cyclic axes, two forward source transforms (resampling and chirps), pointwise product, opposite transform, rescaling, rounding, and carry propagation, with total error below $1/2$.; how: Reduce the integer product to a cyclic convolution of $S$ short coefficients and compute it through the chain of reductions (digits, CRT address map, Gaussian resampling with chirps, twist to the polynomial ring, synthetic-transform convolution); recover exact integers because the accumulated error is below $1/2$. (on [The multiplication algorithm](multiplication-algorithm.md))
## Steps this page derives

The argument gives this page these sub-steps of its step (S5.2.1, S5.2.2, S5.2.3). Derive each here, so the reader can check how it follows from what comes before: stating its result, or citing the source for it, does not derive it. The passages report each step in the source; they ground the derivation and are not to be copied onto the page. In `derivations`, give each step's id and the passage on the page that derives it, copied exactly: submit checks that the passage is on the page, and the audit judges whether it is enough.

**S5.2.1**: Write the two inputs as polynomials in $2^b$, each with $q=\lceil n/b\rceil$ nonnegative digits; their product has degree at most $2q-2$.
Source: "$q=\lceil n/b\rceil$ nonnegative digits.  Their product has degree at most" (paper)
> Write the two inputs as polynomials in $2^b$, each with $q=\lceil n/b\rceil$ nonnegative digits. Their product has degree at most \[ 2q-2<2n/b\le T/2<S. \] Thus convolution modulo $x^S-1$ has no nonzero wraparound. Every product coefficient obeys \begin{equation}\label{eq:capacity} 0\le c_j\le q(2^b-1)^2<2^{3b}, \end{equation} since $q\le n\le2^b$.

**S5.2.2**: Since $2q-2<2n/b\le T/2<S$, convolution modulo $x^S-1$ has no nonzero wraparound.
Source: "2q-2<2n/b\le T/2<S." (paper)
> Write the two inputs as polynomials in $2^b$, each with $q=\lceil n/b\rceil$ nonnegative digits. Their product has degree at most \[ 2q-2<2n/b\le T/2<S. \] Thus convolution modulo $x^S-1$ has no nonzero wraparound. Every product coefficient obeys \begin{equation}\label{eq:capacity} 0\le c_j\le q(2^b-1)^2<2^{3b}, \end{equation} since $q\le n\le2^b$.

**S5.2.3**: Every product coefficient obeys $0\le c_j\le q(2^b-1)^2<2^{3b}$, since $q\le n\le2^b$.
Source: "0\le c_j\le q(2^b-1)^2<2^{3b}," (paper)
> Write the two inputs as polynomials in $2^b$, each with $q=\lceil n/b\rceil$ nonnegative digits. Their product has degree at most \[ 2q-2<2n/b\le T/2<S. \] Thus convolution modulo $x^S-1$ has no nonzero wraparound. Every product coefficient obeys \begin{equation}\label{eq:capacity} 0\le c_j\le q(2^b-1)^2<2^{3b}, \end{equation} since $q\le n\le2^b$.

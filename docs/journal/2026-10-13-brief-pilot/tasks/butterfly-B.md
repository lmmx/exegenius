# exegenius task 0039-write-simultaneous-butterfly-layer

Project directory: `<project>` (paths below are relative to it)

Write one page of an explanatory wiki. The wiki explains a target result from source documents to a specific reader, one concept per page, in reading order.

- Write for this reader's next uncertainty: at each point, give what they need to follow, and no more. The task lists the step of the argument this page presents, its motivations, and the pages around it. They are material for deciding what to say, not a list to cover: explain the transitions this reader would find hard, and leave familiar ones to ordinary sentences.
- Open with a sentence that says what the concept is and what it is for on the way to the target.
- If the page presents a step of the argument, say how its method shows its claim, and give its main motivation in a sentence. Add another motivation only if the reader would be stuck without it.
- Explain the page's concept so that this reader can follow it using only what they already understand (listed below), what this page itself explains, and the linked prerequisite pages.
- Define any other technical term on this page, where it is first used.
- Say what each symbol denotes where you introduce it: what it stands for, and for a size or count, what it counts ("$r$, the number of complex coefficients in a record", not "a power of two $r$"). A use in an example ("take $r = 4$") does not say what $r$ denotes. Symbols listed as already bound can be used without restating them: the page shows the reader their meanings. Using one of those letters with another meaning confuses readers: pick another letter, or say on the page that the letter changes meaning here.
- Link each prerequisite page the first time you rely on it, using the exact markdown link given. You may also link the other pages listed. Submit rejects links to anything else in the wiki; external URLs are allowed.
- For each prerequisite page, give a relation: the prerequisite's id, a predicate, and the sentence that links or names it, copied exactly. Use the sentence where the page relies on it; do not add a sentence for the purpose. The page shows the relation as "<this page's concept> *<predicate>* <prerequisite>", so pick a predicate that makes that a true sentence, with this page's concept as subject: "is reduced to", "computes its transforms by", "is based on", "follows from", "bounds the cost of". Build the predicate from the sentence's own words (other forms of a word count: "reduces" for "reduced"), and pick one that says what the prerequisite does for this page: "uses" says little.
- Stay accurate to the sources. The paragraphs around the evidence are given below; read further in the sources only if they are not enough. State only what the sources support or what is standard background; do not invent results, constants or attributions.
- Cite the results and constants a reader may want to check, right after the claim: `<cite data-source="id">text copied word for word from that source</cite>`. Readers see these as "(see p. 5)"-style references, hidden until they ask for them, so the sentence must read complete without the quoted words: write "the cost is $O(n \log n)$ <cite …>…</cite>", not "the paper states the cost as <cite …>…</cite>". Submit rejects a citation right after "that", "as", "states", "says", "is", "gives", "reads" or a colon, and text not found in the named source. Do not cite common background.
- A citation says where a claim comes from; it does not explain it. If the page's concept is an identity, a construction or a definition, state it. Explain how the concept works and how it shows its claim. For a result the page uses from prior work, say what the result states and why it applies here, and cite the source for its proof: the page need not reproduce it.
- Keep the page short. The task gives its word limit, counted without mathematics and citations, and submit rejects a longer page. Give a small worked example where it makes the idea clearer. No filler, no rhetorical flourishes, no announcing what the page will do.
- Write mathematics in LaTeX: $...$ inline, $$...$$ on its own lines for display.
- Do not write a title heading, a prerequisites list or a sources section: those are added automatically. Use ## subheadings only if the page is long.

## Reader

Undergraduate computer science: big-O notation, polynomials, complex numbers, basic Turing machines and tapes

For these sources: The reader has an undergraduate computer science degree: asymptotic analysis, discrete mathematics (sets, functions, combinatorial counting, induction, basic modular arithmetic and number theory such as primes, gcd and the Euclidean algorithm), recursion and elementary algorithms (sorting, stacks, divide and conquer), basic Turing machines with several tapes, and the first-year mathematics that goes with it: single-variable calculus (limits, convergence of sequences and series, basic integrals, geometric series), matrices and vectors, polynomials and complex numbers. The reader has not met the discrete Fourier transform or the FFT, abstract algebra beyond modular arithmetic (rings, quotient rings, algebra isomorphisms, cyclic-group structure), Gaussian integers, number-theoretic results on the density of primes, or any integer-multiplication algorithm faster than the schoolbook method.

The reader already understands these concepts:

- Big-O notation
- Polynomials
- Complex numbers
- Turing machines and tapes
- Fixed finite-tape Turing machine model
- Schönhage–Strassen optimality conjecture (also: n log n conjecture)
- Established O(n log n) multiplier (Harvey–van der Hoeven)
- Arrays as tape streams and elementary scans
- Controlled cyclic shifts and rational scalings
- Linear maps, rank, projections and orthogonal complements
- Cyclic convolution and the discrete Fourier transform
- Multidimensional (tensor) transforms
- Multiplication as convolution of radix digits
- Chinese-remainder axis map to prime cyclic axes
- Two-point butterfly and decimation-in-frequency layer
- Gaussian resampling
- Bluestein chirp identity
- Resampling with the permutations left in the transform
- Polynomial ring modulo y^r + 1
  - $y$ (ring variable): The polynomial indeterminate, whose powers act as roots of unity by signed shifts of the coefficient list.
  - $r$ (record length): The number of complex coefficients in a polynomial record, a power of two.
- Phase twist from cyclic to negacyclic convolution
- Synthetic polynomial transforms (Nussbaumer)
- Fixed-point grids, truncation and contraction error bounds
- Restoring arbitrary scratch values
- Three-element subset gadget
- Three-stage bank exchange network
- Frames: changing representation along the network
- Nested subspace labels and orthogonal residuals
- Strict total rank deficit s < W·m
- Lower-triangular (Bruhat) factorization of a matrix
- Bit network as an address shear
- Role streams and fixed-tape depth-first scheduling
- Recursive power saving from s/W < m
- Fast address-chunk interchange
- Packed selected-bit addition with guard repair
- Phase-frame (complex) network for the butterfly
- Recursive implementation of the phase network on selected bits
- Exact Gaussian dyadic arithmetic and guard widths
- Signed packing (Kronecker substitution) for ring products
- Logarithms and exponentials (also: log, lg, log_2, base-2 logarithm, natural logarithm, powers of two)
- Floor, ceiling and rounding (also: floor function, ceiling function, round to nearest integer, integer part)
- Binary representation of integers (also: binary numerals, bit strings, radix representation, positional notation, base-b digits, most significant bit, leading zeros, bits)
- Schoolbook arithmetic and carries (also: long multiplication, grade-school multiplication, carry propagation, addition with carries, O(n^2) multiplication)
- Bitwise operations and XOR (also: exclusive or, XOR gate, addition mod 2, parity, bit flip)
- Modular arithmetic and residues (also: congruence, mod, integers modulo m, Z/mZ, residue ring, residue classes, modular inverse)
- Primes, divisibility and coprimality (also: prime numbers, gcd, greatest common divisor, relatively prime, pairwise coprime, extended Euclidean algorithm, trial division, odd prime)
- Roots of unity (also: nth roots of unity, primitive root of unity, e^{2 pi i/n}, twiddle factor, unit-modulus phase)
- Complex modulus, conjugate and triangle inequality (also: absolute value of a complex number, |z|, real and imaginary parts, complex conjugate, unit disk, triangle inequality)
- Max norm and error bounds (also: sup norm, infinity norm, maximum norm, worst-case error, absolute error, round-off error)
- Little-o, big-Omega and Theta notation (also: o(.), Omega, Theta, asymptotic lower bound, tight bound, polylogarithmic factor, power saving)
- Series and exponential decay estimates (also: geometric series, summing a recurrence, convergence of series, exponentially small, 2^{-p})
- Recurrences and recursion (also: recursive calls, divide and conquer, recursion depth, recurrence relation, unrolling a recurrence, induction on the parameter)
- Sorting and stacks (also: sorting a list, stack data structure, last-in first-out, push and pop, merge sort)
- Sets, subsets and counting (also: three-element subsets, set intersection, binomial coefficient, n choose k, Cartesian product, inclusion-exclusion, indicator vector)
- Permutations and bijections (also: permutation, bijection, partial permutation, permutation matrix, inverse permutation, rearranging indices)
- Lexicographic order and mixed-radix indices (also: lexicographic ordering, multi-index, row-major order, multidimensional array index, flattening an array, mixed radix)
- Matrices, matrix-vector products and inverses (also: matrix, matrix multiplication, identity matrix, matrix inverse, diagonal matrix, triangular matrix, transpose, all-ones matrix, invertible matrix)
- Kronecker product of matrices (also: tensor product of matrices, Kronecker power, tensor power, A tensor B)
- Hadamard matrix and the 2x2 butterfly matrix (also: Walsh-Hadamard transform, Hadamard transform, H = [[1,1],[1,-1]], Pauli X, swap matrix)
- Inner products, orthogonality and eigenvalues (also: dot product, orthogonal vectors, orthogonal complement, eigenvalue, eigenvector, symmetric matrix, bilinear form)
- Polynomial evaluation and interpolation (also: evaluate a polynomial at a point, Kronecker substitution, polynomial multiplication by evaluation, degree of a polynomial, coefficient list)
- Fixed-point and floating-point numbers (also: fixed-point arithmetic, dyadic rationals, binary fractions, fractional bits, truncation, significand, precision)
- Karatsuba and the history of fast multiplication (also: Karatsuba multiplication, Toom-Cook, FFT multiplication, Schonhage-Strassen, integer multiplication complexity)
- Square roots and quotient with remainder (also: integer division, quotient and remainder, integer square root, Newton iteration for division)

## This page

Concept: Simultaneous butterfly layer in O(V·d^λ′)
Gloss: The second tape procedure: applying H₀ at the same bit offset in up to d equal-width address chunks of polynomial records at once, with one truncation, in time O(V·d^λ′) for a fixed λ′ < 1 rather than d separate passes.

## This page in the argument

The step: The map $H_0(u,v)=((u+v)/2,(u-v)/2)$ is applied at the same bit offset in up to $d$ equal-width address chunks of polynomial records at once, with one final truncation, in time $O(Vd^{\lambda'})$ for a fixed $\lambda'<1$.
How: Compute the tensor power of the kernel $C$ with the complex bank-exchange network on selected bits, implement the basis changes by packed whole-chunk shifts, and bound the exact dyadic values
Why: Doing $D$ separate butterfly passes costs $D$ whole-array passes, whereas the network recursion on groups of axes saves a power of $d$. (the sources say so: "factors would require $D$ whole-array passes." (paper))
Striking feature: Applying the butterfly on $d$ chunks together costs $d^{\lambda'}$ passes with $\lambda'<1$, rather than $d$. (explain why it holds, from the method)

Its sub-steps, in order:

- An edge's phase-frame change is $\prod_j(aI+bX_{v_j})^{\epsilon_j}$; conjugated by a binary change of basis it needs one child call of $C^{\otimes f}$ per residual basis vector, giving $s$ child calls on $f=e/m$ axes each, and $H_0^{\otimes D}=b^D\mathcal S_DC^{\otimes D}\mathcal S_D$ converts the $C$ layer to a normalized layer. (on [Recursive implementation of the phase network on selected bits](phase-edge-recursion.md))
- A binary row addition $y_{j_i}\leftarrow y_{j_i}\oplus x_{j_i}$ on the same selected bit of $f$ chunks of $K$ bits is done by eight whole-chunk cyclic shifts, valid away from a set of addresses of fraction at most $80(f-1)2^{-K}$, which a deterministic sort repairs, in time $O(V(L^\tau+1))$. (on [Packed selected-bit addition with guard repair](packed-selected-bit-addition.md))
- Splitting rows among the $W$ roles gives children of volume $V/W$ and the recurrence $F(e)\le(s/W)F(e/m)+O((eK)^\tau+1)$, which unrolls to $O(d^{\lambda'})$ per unit volume for a fixed $\lambda'<1$.
- Reserving $p+\Delta$ fractional bits and $\Delta+3$ integer and sign bits with $\Delta=\lceil C_0d^{C_1}\rceil=o(p)$ represents every intermediate value of the layer exactly, so only one truncation is needed after the completed contractive layer. (on [Exact Gaussian dyadic arithmetic and guard widths](exact-dyadic-guard.md))
- Combining the row construction, the recurrence and the guard widths, the layer costs $O(Vd^{\lambda'})$ on fixed tapes and truncation toward zero contributes an error below $\sqrt2\,2^{-p}$.

It serves these steps, from the main result down:

- One deterministic fixed-tape Turing machine multiplies $n$-bit integers exactly in $O(n(\lg n)^{1-\kappa})$ time with $\kappa=2^{-182}$, disproving the $n\log n$ optimality conjecture in this model.; how: Reduce multiplication to a convolution computed by synthetic polynomial transforms after Gaussian resampling and chirps, and speed up their address moves and butterfly layers with two network-based tape procedures (on [Power-saving integer multiplication](power-saving-multiplication.md))

## Brief: what this page must establish

The plan of the paper's proof gives this page the step above. This brief passes on what the plan says about it: which parts this page derives, which other pages derive, and where in the paper each part is.

### Steps this page derives

**S3.3.** Splitting rows among the $W$ roles gives children of volume $V/W$ and the recurrence $F(e)\le(s/W)F(e/m)+O((eK)^\tau+1)$, which unrolls to $O(d^{\lambda'})$ per unit volume for a fixed $\lambda'<1$.

How: Supply rows from the first few chunks, apply the recursion-saving recurrence of the complex network, and sum over a base-$m$ partition of the chunks

- **S3.3.1.** A row index $u=Wg+w$ is sent to role $w$, so each recursive edge call acts on a role stream with exactly $1/W$ of the parent's logical volume.

  Source: "At a node, write a row index as \(u=Wg+w\), with \(0\le w<W\)," (paper)
  The paragraph around it:

  > At a node, write a row index as \(u=Wg+w\), with \(0\le w<W\),
  > and send that entire row to physical wire role \(w\), indexed there
  > by \(g\).  Every role stream has the same remaining address shape.
  > Thus a pointwise gate reads the records with the same \(g\) and the
  > same remaining address in its touched roles.  The scalar network
  > allows these role values to be arbitrary, so the split supplies its
  > scratch roles from existing rows.  Each recursive edge call acts on
  > one role stream with volume exactly \(1/W\) of its parent's logical
  > volume.


- **S3.3.2.** The $s$ child calls run on streams of volume $V/W$, so $F(e)\le(s/W)F(e/m)+O((eK)^\tau+1)$ with leaves at $e<d^\beta$.

  Source: "Indeed, the \(s\) child calls are executed on streams of volume" (paper)
  The paragraph around it:

  > \paragraph{Unrolling the recurrence.}
  > Fix \(\beta=1/2\), and choose real exponents satisfying
  > \[
  >  0<\tau<1,\qquad
  >  \max\{0,\log_m(s/W)\}\le\sigma<1,
  >  \qquad \max\{\tau,\sigma\}<\lambda<1,
  >  \qquad \tau(1+c/\beta)<\lambda.
  > \]
  > The exponents can all be chosen rational, with a strict upper bound
  > in the condition on \(\sigma\).  Stop recursion when
  > \(e<d^\beta\) and apply the \(e\) \(C\) kernels individually.  If
  > \(F(e)\) denotes time divided by the current logical stream volume,
  > then
  > \[
  >  F(e)\le (s/W)F(e/m)+O((eK)^\tau+1),
  >  \qquad F(e)=O(e)\quad(e<d^\beta).
  > \]
  > Indeed, the \(s\) child calls are executed on streams of volume
  > \(V/W\), so their total cost is \(s(V/W)F(e/m)\).  Dividing by
  > the current volume \(V\) gives the factor \(s/W\).
  > At an internal node \(e\ge d^\beta\), hence
  > \(K\le d^c\le e^{c/\beta}\).  The overhead is therefore
  > \(O(e^{\tau(1+c/\beta)}+1)=O(e^\lambda)\).
  > Put \(a=s/W\).  Since \(a\le m^\sigma<m^\lambda\), the internal
  > costs sum to at most
  > \[
  >  C e^\lambda\sum_{j\ge0}(a/m^\lambda)^j=O(e^\lambda).
  > \]
  > If the leaves after \(j\) levels contain \(u=e/m^j<d^\beta\)
  > active chunks each, their combined normalized
  > contribution is at most
  > \[
  >  O(a^ju)\le O(e^\sigma u^{1-\sigma})
  >           \le O(e^\sigma d^{\beta(1-\sigma)}).
  > \]
  > The first inequality uses
  > \(a^j\le m^{j\sigma}=(e/u)^\sigma\).  It also covers \(a<1\)
  > by taking \(\sigma\ge0\).


- **S3.3.3.** Summing the internal costs, the leaf costs and the at most $(m-1)(1+\lfloor\log_m d\rfloor)$ base-$m$ pieces gives total cost $O(d^{\lambda'})$ per unit volume.

  Source: "Adding their costs and the row preprocessing gives" (paper)
  The paragraph around it:

  > Write \(D-q_0=\sum_j a_jm^j\) in base \(m\), with \(0\le a_j<m\).
  > Partition the remaining chunks into \(a_j\) consecutive pieces
  > containing \(m^j\) chunks for each \(j\).
  > There are at most \((m-1)(1+\lfloor\log_m d\rfloor)\) pieces.
  > Adding their costs and the row preprocessing gives
  > \[
  >  O\!\left(\log(2d)
  >     \bigl[d^\lambda+d^{\sigma+\beta(1-\sigma)}+1\bigr]\right)
  >    =O(d^{\lambda'})
  > \]
  > for any fixed
  > \[
  >  \max\{\lambda,\sigma+\beta(1-\sigma)\}<\lambda'<1.
  > \]
  > The case \(D\le q_0\), in which preprocessing would consume all
  > active chunks, was handled by individual kernels.  Thus all constants
  > and all tape counts are independent of \(p,d,r\) and the input data.


**S3.5.** Combining the row construction, the recurrence and the guard widths, the layer costs $O(Vd^{\lambda'})$ on fixed tapes and truncation toward zero contributes an error below $\sqrt2\,2^{-p}$.

Source: "above give total time $O(Vd^{\lambda'})$ on fixed tapes, including" (paper)
The paragraph around it:

> Uniformly for large $p$, the bands give $f\le d\le p$ and
> \[
>  K\ge\tfrac12a_d^c p^{\epsilon c},\qquad
>  r\ge2^{(a_r/b_d)p^{1-\epsilon}}.
> \]
> The first inequality follows from $K=\lfloor d^c\rfloor$ once
> $d^c\ge2$. In particular, $K\ge6$ and $K/\log p\to\infty$, while
> $r$ dominates every fixed polynomial. Substituting the common bound
> $A_*\le C_Ap$ into the two additional normalized
> terms in \eqref{eq:packed-rectangle-time} gives
> \[
> \begin{aligned}
>  \frac{A_*^3}{R_{\rm g}}
>  &\le\frac{C_A^3p^2}{2r},\\
>  80(f-1)2^{-K}A_*\left(1+\frac{A_*}{R_{\rm g}}\right)
>  &\le80C_Ap^2 2^{-K}\left(1+\frac{C_A}{2r}\right).
> \end{aligned}
> \]
> Both bounds tend to zero uniformly over all current allowed $d,r$.
> Thus Lemma~\ref{lem:packed-selected-bit-rectangle} has the required
> $O(M_{\rm child}R_{\rm g}(L^\tau+1))$ cost for every packed call,
> with one common time constant.
> The row construction and the recurrence analysis
> above give total time $O(Vd^{\lambda'})$ on fixed tapes, including
> all padded and auxiliary streams. Equation~\eqref{eq:phase-butterfly-layer}
> converts the resulting exact $C$-layer to $\mathcal H$.


### Steps derived on other pages

- **S3.1**, on [Recursive implementation of the phase network on selected bits](phase-edge-recursion.md): An edge's phase-frame change is $\prod_j(aI+bX_{v_j})^{\epsilon_j}$; conjugated by a binary change of basis it needs one child call of $C^{\otimes f}$ per residual basis vector, giving $s$ child calls on $f=e/m$ axes each, and $H_0^{\otimes D}=b^D\mathcal S_DC^{\otimes D}\mathcal S_D$ converts the $C$ layer to a normalized layer.

- **S3.2**, on [Packed selected-bit addition with guard repair](packed-selected-bit-addition.md): A binary row addition $y_{j_i}\leftarrow y_{j_i}\oplus x_{j_i}$ on the same selected bit of $f$ chunks of $K$ bits is done by eight whole-chunk cyclic shifts, valid away from a set of addresses of fraction at most $80(f-1)2^{-K}$, which a deterministic sort repairs, in time $O(V(L^\tau+1))$.

- **S3.4**, on [Exact Gaussian dyadic arithmetic and guard widths](exact-dyadic-guard.md): Reserving $p+\Delta$ fractional bits and $\Delta+3$ integer and sign bits with $\Delta=\lceil C_0d^{C_1}\rceil=o(p)$ represents every intermediate value of the layer exactly, so only one truncation is needed after the completed contractive layer.


### What these steps rely on, besides the prerequisite pages

Pages (link each where the derivation uses it; do not derive their results again):

- [Recursive power saving from s/W < m](recursive-power-saving.md)
- [Fast address-chunk interchange](fast-chunk-interchange.md)
- [Arrays as tape streams and elementary scans](stream-scans.md)
- [Controlled cyclic shifts and rational scalings](controlled-shifts.md)
- [Phase-frame (complex) network for the butterfly](phase-network.md)
- [Fixed-point grids, truncation and contraction error bounds](fixed-point-precision.md)
- [Two-point butterfly and decimation-in-frequency layer](two-point-butterfly.md)
- [Polynomial ring modulo y^r + 1](polynomial-ring-negacyclic.md)
- [Packed selected-bit addition with guard repair](packed-selected-bit-addition.md)

Concepts the reader already knows: Series and exponential decay estimates

### Acceptance criteria

- Derive each step this page derives (S3.3, S3.5): show how each follows from the ones before, so the reader can check it. Stating its result, or citing the paper for it, is not enough.
- For a step derived on another page, state its result in a sentence where this page uses it, and link that page. Do not derive it again.
- The quotes and paragraphs above ground the derivation. Do not copy them onto the page; cite only a result or constant a reader may want to check.
- The word limit below still holds. If the steps cannot all be derived within it, derive the ones the page's claim most depends on, and say so in your reply.

## Prerequisite pages (link each on first use, exactly as written, and give a relation for each)

- [Recursive implementation of the phase network on selected bits](phase-edge-recursion.md): A group of e = m·f selected axes is split into m slots of f chunks; basis changes by invertible binary matrices (products of row additions) and one child call of C^{⊗f} per residual basis vector implement each edge, giving s child calls each on volume V/W, and H₀^{⊗D} = b^D S_D C^{⊗D} S_D converts the result to a normalized layer.
  Shown as: Simultaneous butterfly layer in O(V·d^λ′) *<predicate>* Recursive implementation of the phase network on selected bits
- [Exact Gaussian dyadic arithmetic and guard widths](exact-dyadic-guard.md): The whole accelerated layer is computed exactly on Gaussian dyadics with explicit extra integer and fractional guard bits (growing like d^20 = o(p)), so only one truncation to the p-bit grid is applied after the completed contractive layer.
  Shown as: Simultaneous butterfly layer in O(V·d^λ′) *<predicate>* Exact Gaussian dyadic arithmetic and guard widths
- [Role streams and fixed-tape depth-first scheduling](role-streams.md): Rows are distributed cyclically among the W wire roles so each recursive child works on one stream of volume V/W with the same local address set, and the recursion runs depth-first on a fixed number of tapes using a parking stack for inactive streams.
  Shown as: Simultaneous butterfly layer in O(V·d^λ′) *<predicate>* Role streams and fixed-tape depth-first scheduling

## Pages that build on this one

- Transform layout and retained frequency order: The d−1 transform axes are rearranged by single-bit moves and width-K chunk exchanges so that each round's selected bits lie at a common offset in consecutive chunks; the forward transform outputs frequencies in a fixed permuted order that the pointwise product and the opposite transform consume without unpermuting.
- Power-saving integer multiplication: The main theorem: one deterministic fixed-tape Turing machine multiplies n-bit integers exactly in O(n (lg n)^{1−κ}) time with κ = 2^{-182}, disproving the n log n optimality conjecture in this model, by combining the fast address interchange and simultaneous butterfly layers (each saving a power of the chunk width) with Gaussian resampling, chirps and synthetic polynomial transforms.

## Other pages you may link to

- [Polynomial ring modulo y^r + 1](polynomial-ring-negacyclic.md)
- [Linear maps, rank, projections and orthogonal complements](linear-algebra-basics.md)
- [Three-element subset gadget](subset-gadget.md)
- [Fixed finite-tape Turing machine model](fixed-tape-model.md)
- [Schönhage–Strassen optimality conjecture](nlogn-conjecture.md)
- [Established O(n log n) multiplier (Harvey–van der Hoeven)](hvdh-multiplier.md)
- [Arrays as tape streams and elementary scans](stream-scans.md)
- [Controlled cyclic shifts and rational scalings](controlled-shifts.md)
- [Restoring arbitrary scratch values](restored-scratch.md)
- [Three-stage bank exchange network](bank-exchange-network.md)
- [Frames: changing representation along the network](frames.md)
- [Nested subspace labels and orthogonal residuals](subspace-labels.md)
- [Strict total rank deficit s < W·m](rank-saving.md)
- [Recursive power saving from s/W < m](recursive-power-saving.md)
- [Lower-triangular (Bruhat) factorization of a matrix](lower-triangular-factorization.md)
- [Bit network as an address shear](bit-network-shear.md)
- [Fast address-chunk interchange](fast-chunk-interchange.md)
- [Packed selected-bit addition with guard repair](packed-selected-bit-addition.md)
- [Fixed-point grids, truncation and contraction error bounds](fixed-point-precision.md)
- [Signed packing (Kronecker substitution) for ring products](signed-packing.md)
- [Cyclic convolution and the discrete Fourier transform](dft-convolution.md)
- [Multiplication as convolution of radix digits](radix-digit-convolution.md)
- [Multidimensional (tensor) transforms](multidim-transform.md)
- [Two-point butterfly and decimation-in-frequency layer](two-point-butterfly.md)
- [Synthetic polynomial transforms (Nussbaumer)](synthetic-transform.md)
- [Phase-frame (complex) network for the butterfly](phase-network.md)
- [Chinese-remainder axis map to prime cyclic axes](crt-axis-map.md)
- [Gaussian resampling](gaussian-resampling.md)
- [Bluestein chirp identity](bluestein-chirp.md)
- [Phase twist from cyclic to negacyclic convolution](phase-twist.md)
- [Resampling with the permutations left in the transform](permutations-left-in-transform.md)
- [Transform layout and retained frequency order](transform-layout-order.md)
- [Normalized convolution over the polynomial ring](ring-convolution.md)
- [Padded power-of-two box and linewise tensor maps](padded-box-lines.md)
- [Prime source lengths near the powers of two](prime-lengths.md)
- [Complex cyclic convolution by a coefficient twist](complex-convolution.md)
- [Source transform: compress, chirp convolution, expand](source-transform.md)
- [Parameter choices and the cost table giving κ](parameter-choices.md)
- [Exact recovery by rounding and carry propagation](exact-recovery.md)
- [The assembled multiplication algorithm](multiplication-algorithm.md)
- [Power-saving integer multiplication](power-saving-multiplication.md)

## Evidence from the sources, with the paragraph around each quote

- "one selects the same bit position in each of at most $d$ equal-width address chunks, and the cost is $O(Vd^{\lambda'})$ for a fixed $\lambda'<1$." (paper)

  > The second procedure applies \[ H_0(u,v)=\bigl((u+v)/2,(u-v)/2\bigr) \] simultaneously on selected coordinate bits of the numerical arrays. Each record is a polynomial with complex fixed-point coefficients, and $H_0$ acts coefficientwise. On the layouts used in the multiplication algorithm, one selects the same bit position in each of at most $d$ equal-width address chunks, and the cost is $O(Vd^{\lambda'})$ for a fixed $\lambda'<1$. Intermediate values are exact Gaussian dyadics, whose real and imaginary parts have power-of-two denominators. A single truncation follows the completed contractive layer. Section~\ref{sec:tape-interchange} proves the address bound, and Section~\ref{sec:fast-butterfly-layers} gives the numerical layout, precision hypotheses, and layer bound.

- "whereas separate butterfly factors would require $D$ whole-array passes." (paper)

  > The parameters will satisfy \[ d=\Theta(p^\epsilon),\qquad K=\lfloor d^c\rfloor, \qquad r=2^{\Theta(p/d)},\qquad c>0,\quad 0<\epsilon<1. \] The exponents and comparison constants are fixed, and we impose their further restrictions in Proposition~\ref{prop:simultaneous-layer}. Two consequences explain the layout. The polynomial length $r$ grows faster than every fixed power of $p$, so a scan of a record pays for polynomial work on its address. Also $K/\log p\to\infty$, which will make the exceptional set in a packed change of address bits small enough to repair by sorting. We will obtain cost $O(Vd^{\lambda'})$, with $\lambda'<1$ and $V$ the logical bit volume, whereas separate butterfly factors would require $D$ whole-array passes.


## Sources (read from disk as needed)

- `sources/paper.tex`: source id `paper`, "Integer multiplication below n log n (OpenAI, 2026)"

## Length

At most 350 words, not counting mathematics and citations: submit rejects a longer page.

## Answer

The answer is two files: the page body as markdown (not JSON), and a JSON object with the symbols the page binds and its relations to its prerequisite pages. In JSON strings, write each backslash of LaTeX twice (`"\\kappa"`). The JSON follows this schema:

```json
{
  "$defs": {
    "Edge": {
      "additionalProperties": false,
      "description": "What this page uses one prerequisite page for.",
      "properties": {
        "predicate": {
          "description": "A short predicate built from the sentence's own words, such that \"<this page's\nconcept> <predicate> <prerequisite>\" is a true sentence: \"is reduced to\", \"computes\nits transforms by\", \"is based on\".",
          "type": "string"
        },
        "prerequisite": {
          "description": "Id of the prerequisite page.",
          "type": "string"
        },
        "quote": {
          "description": "The sentence on this page that states it, copied exactly. It must link or name the\nprerequisite and contain the predicate's words.",
          "type": "string"
        }
      },
      "required": [
        "prerequisite",
        "predicate",
        "quote"
      ],
      "type": "object"
    },
    "PageBinding": {
      "additionalProperties": false,
      "description": "What a symbol a page introduces denotes, and where the page says so.",
      "properties": {
        "meaning": {
          "description": "One sentence saying what the symbol denotes; for a count, what it counts.",
          "type": "string"
        },
        "name": {
          "description": "Display name of one to four words, e.g. \"record length\".",
          "type": "string"
        },
        "quote": {
          "description": "The sentence on the page that says what the symbol denotes, copied exactly. A use\nin an example (\"take $r = 4$\") does not say what a symbol denotes.",
          "type": "string"
        },
        "relation": {
          "$ref": "#/$defs/Relation"
        },
        "symbol": {
          "description": "The symbol in LaTeX, as it appears in the page's mathematics, without dollar signs,\ne.g. `r` or `\\kappa`.",
          "type": "string"
        }
      },
      "required": [
        "symbol",
        "name",
        "meaning",
        "relation",
        "quote"
      ],
      "type": "object"
    },
    "Relation": {
      "oneOf": [
        {
          "const": "direct",
          "description": "The symbol denotes a thing: \"ω, a primitive root of unity\".",
          "type": "string"
        },
        {
          "const": "count",
          "description": "The symbol is the number of instances of something: \"r, the number of complex\ncoefficients in a record\".",
          "type": "string"
        }
      ]
    }
  },
  "additionalProperties": false,
  "description": "The structured part of a write answer; the body is the markdown answer file.",
  "properties": {
    "relations": {
      "description": "One relation for each prerequisite page: what this page uses it for. Empty if the\npage has no prerequisite pages.",
      "items": {
        "$ref": "#/$defs/Edge"
      },
      "type": "array"
    },
    "symbols": {
      "description": "Every symbol the page binds: each symbol it introduces and says what it denotes.\nLeave out symbols listed as already bound, unless the page gives them a new meaning.\nEmpty if the page introduces no symbols.",
      "items": {
        "$ref": "#/$defs/PageBinding"
      },
      "type": "array"
    }
  },
  "required": [
    "symbols",
    "relations"
  ],
  "title": "WriteAnswer",
  "type": "object"
}
```

Write the body to `exegenius/tasks/0039-write-simultaneous-butterfly-layer.answer.md` and the JSON to `exegenius/tasks/0039-write-simultaneous-butterfly-layer.answer.json`, then run `exegenius submit 0039-write-simultaneous-butterfly-layer` in the project directory. If submit rejects the answer, fix the answer and submit again.

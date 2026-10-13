# exegenius task 0052-write-time-accounting

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

For these sources: The reader is an undergraduate computer scientist: discrete mathematics (induction, modular arithmetic, gcd, permutations, relations, graphs), algorithms (divide and conquer, recurrences, schoolbook and Karatsuba multiplication, recursion), single-variable calculus (limits, convergence of sequences and series, basic integrals, exponentials), and introductory linear algebra (matrices, linear maps, invertibility, Gaussian elimination). They know complex numbers with roots of unity, polynomials, big-O and its relatives, and elementary Turing machines with tapes. They have not studied abstract algebra beyond residue rings, Fourier analysis, the fast Fourier transform or the discrete Fourier transform, numerical error analysis, analytic number theory, or multitape-machine algorithm design.

The reader already understands these concepts:

- Big-O notation
- Polynomials
- Complex numbers
- Basic Turing machines and tapes
- Fixed multitape Turing machine model
- Schönhage–Strassen n log n conjecture and prior bounds (also: n log n optimality conjecture)
- Harvey–van der Hoeven O(n log n) multiplier
- Cyclic convolution
- Normalized discrete Fourier transform (also: Fourier transform)
- Convolution via transforms
- Butterfly layers (fast Fourier transform) (also: FFT, decimation in frequency, butterfly)
- Multidimensional transform over several axes
- Multiplication as radix-digit convolution
- Chinese-remainder map to prime cyclic axes (also: Agarwal–Cooley map, coefficient-address map Φ)
- Disk-grid arrays, contractions and truncation (also: Q_p truncation)
- Gaussian dyadic numbers
- Gaussian resampling
- Resampling with the permutations left in the transform
- Bluestein chirp identity (also: chirp)
- Cancellation of the retained source permutation
- Polynomial ring modulo y^r + 1 (also: negacyclic ring, R_r)
  - $y$ (indeterminate): The formal variable of the polynomials in the ring.
  - $r$ (record length): The number of complex coefficients in one polynomial record, a power of two.
- Synthetic roots of unity
- Synthetic polynomial transform (also: polynomial transform)
- Coefficient phase twist
- Signed packing for exact polynomial products (also: Kronecker substitution)
- Elementary stream operations
- Controlled cyclic shifts
- Fixed-tape depth-first recursion schedule
- Subspaces, rank and orthogonal complements
- Lower triangular factorization of a matrix (also: Bruhat factorization)
- Scratch-restoring linear network (also: bank exchange network, transparent computation)
- Frames and the common-frame identity
- Nested subspace labels from three-element subsets (also: orthogonal residual)
- Rank saving s < Wm
  - $s$ (total call count): The number of smaller operations, equal to the sum of the ranks of all edge changes in the network.
  - $W$ (wire count): The number of wires, including scratch wires, in the fixed network.
  - $m$ (label dimension): The number of coordinates of the ambient label space, which is also the number of fields in an address chunk.
- Volume-normalized power-saving recurrence
- Cost of a rational matrix shear
- Bit network and rational matrix interface
- Power-width address interchange
- Fast address-chunk interchange (also: chunk swap)
- Complex phase network (also: binary phase interface)
- Packed selected-bit addition
- Guard bits and a single final truncation
- Simultaneous normalized butterfly layer
- Synthetic transform layout
- Convolution from the retained transform layout
- Tensor resampling on a padded box
- Parameter choices for the padded box
- Prime source lengths near powers of two
- Source transform procedure
- The multiplication algorithm
- Precision and exact recovery
- Asymptotic notation beyond big-O (also: Theta, Omega, little-o, o(n), liminf, Θ, Ω)
- Logarithms and exponentials (also: log, lg, log base 2, natural logarithm, ln, exponential function, e^x, iterated logarithm, log star)
- Floor and ceiling (also: integer part, round down, round up, ⌊x⌋, ⌈x⌉, rounding to nearest integer)
- Binary representation of integers (also: binary expansion, bits, most significant bit, padding with leading zeros, binary fractions, two's complement, signed integers)
- Modular arithmetic (also: congruence, residues, integers modulo q, Z/qZ, residue ring, units modulo q, invertible modulo q, reduction mod q)
- Division with remainder and the Euclidean algorithm (also: gcd, greatest common divisor, extended Euclidean algorithm, coprime, relatively prime, Bezout identity, integer quotient and remainder)
- Prime numbers (also: primes, odd prime, prime factorization, trial division, composite number)
- Permutations and bijections (also: permutation, bijection, one-to-one correspondence, sorting a list into a given order, partial permutation, permutation matrix)
- Matrices and linear maps (also: matrix multiplication, invertible matrix, matrix inverse, linear operator, identity matrix, diagonal matrix, lower triangular matrix, Gaussian elimination, linear combination, transpose, matrix transposition)
- Inner product, orthogonality and norms (also: dot product, orthogonal vectors, orthonormal basis, maximum norm, sup norm, infinity norm, modulus of a vector, triangle inequality, operator norm)
- Euler's formula and complex exponentials (also: e^{iθ}, exp(iθ), unit circle, modulus and argument, complex conjugate, phase, polar form, unit disk)
- Sums, geometric series and sigma notation (also: summation, Σ, geometric series, geometric sum, telescoping sum, double sum, interchanging sums)
- Limits, convergence and error bounds (also: epsilon-delta, convergence of a sequence, convergent series, tends to zero, upper bound on error, inequalities, rounding error, Gaussian function, e^{-πx²})
- Mathematical induction and recurrences (also: proof by induction, recurrence relation, solving a recurrence, divide and conquer, recursion)
- Schoolbook and Karatsuba multiplication (also: long multiplication, grade-school multiplication, Karatsuba, O(n^2) multiplication, divide-and-conquer multiplication)
- Positional notation in a general radix (also: base b digits, radix, digits, carry propagation, carries, base 2^b, digit expansion)
- Cartesian product and mixed-radix addresses (also: row-major order, lexicographic order, multi-index, array indexing, tuple, address of an array entry, cell address)
- Bitwise XOR and the field with two elements (also: exclusive or, XOR, addition mod 2, F_2, GF(2), parity)
- Rings and fields, algebraic structure basics (also: ring, field, group, isomorphism, homomorphism, rational numbers, commutative ring, quotient by a relation, algebra isomorphism)
- Fixed-point and exact arithmetic (also: fixed-point numbers, dyadic rationals, rounding to nearest integer, truncation, round to the nearest integer, precision in bits)
- Finite directed acyclic networks (also: circuit, linear circuit, gate, wire, vertex and edge, graph, directed graph, acyclic graph, network)
- Stacks and recursion on tapes (also: call stack, stack, program counter, subroutine call, depth-first search, descriptor)
- Linear-time tape scans and sorting cost (also: linear scan, single pass over the tape, copying a tape, sorting, comparison sort)
- Newton's method (also: Newton iteration, Newton–Raphson, reciprocal by iteration, quadratic convergence)

## This page

Concept: Total time bound and the power saving
Gloss: Dividing every cost by Tp gives seven dominant powers of p, each at most 1−2κ; the smallest margin 2^{-181} = 2κ makes the total O(n (lg n)^{1-κ}).

## This page in the argument

The step: Dividing every cost by $Tp$ gives seven dominant powers of $p$, each at most $1-2\kappa$, so the total running time is $O(n(\lg n)^{1-\kappa})$ for every input length.
How: Tabulate the normalized cost of each stage as a power of $p$, justify each row from the earlier cost bounds, and compare every exponent with $1-2\kappa$.
Why: The cost of each stage is a power of $p$ times the volume $Tp$, so the whole bound reduces to comparing exponents. (the sources leave this for the reader to work out: spell it out)
Striking feature: The smallest margin below one, $2^{-181}$ from the chunk-exchange row, is exactly $2\kappa$, which is what fixes $\kappa=2^{-182}$. (explain why it holds, from the method)

Its sub-steps, in order:

- After division by $V=Tp$ each stage costs a power of $p$ (prefix moves $p^{\epsilon(1+c)}$, chunk exchanges $p^{1-\epsilon c(1-\tau)}$, simultaneous layers $p^{1-\epsilon(1-\lambda')}$, and so on).
- Seven dominant powers have margins below one, $g_1,\dots,g_7$, with $g_2=2^{-181}$, $g_3=2^{-129}$, $g_7=2^{-75}$ and the others larger, so $\min_i g_i=2^{-181}=2\kappa$.
- Each normalized cost row is bounded by the cost of the procedure proved earlier for that stage.
- Each of the seven powers is at most $1-2\kappa$ and any fixed power of $\log p$ is at most $p^\kappa$ eventually, so the total time is $O(Tp\,p^{1-\kappa})=O(n(\log n)^{1-\kappa})$, with schoolbook multiplication below a fixed cutoff.

It serves these steps, from the main result down:

- A single deterministic Turing machine with a fixed alphabet and a fixed number of one-dimensional tapes multiplies two $n$-bit integers exactly in $O(n(\lg n)^{1-\kappa})$ worst-case time with $\kappa=2^{-182}$, refuting the $n\log n$ optimality conjecture in this model.; how: Convert the product into a convolution computed by synthetic transforms after Gaussian resampling and a chirp, speed up the transforms with two network-based tape procedures (address interchange and simultaneous butterfly layers), then prove the product exact and add up the time. (on [Integer multiplication in O(n (lg n)^(1−κ)) time](main-theorem.md))

## Steps this page derives

The argument gives this page these sub-steps of its step (S6.A, S6.B, S6.C, S6.D). Derive each here, so the reader can check how it follows from what comes before: stating its result, or citing the source for it, does not derive it. The passages report each step in the source; they ground the derivation and are not to be copied onto the page. In `derivations`, give each step's id and the passage on the page that derives it, copied exactly: submit checks that the passage is on the page, and the audit judges whether it is enough.

**S6.A**: After division by $V=Tp$ each stage costs a power of $p$ (prefix moves $p^{\epsilon(1+c)}$, chunk exchanges $p^{1-\epsilon c(1-\tau)}$, simultaneous layers $p^{1-\epsilon(1-\lambda')}$, and so on).
Source: "Operation & Normalized cost & Power of $p$\\" (paper)
> For the cost calculation put $V=Tp=\Theta(n)$. The fixed-width coefficient formats use $\Theta(p)$ bits per coefficient, so a full padded array has $\Theta(V)$ stored bits. Table~\ref{tab:costs} lists costs after division by $V$; a fixed number of repeated transforms and convolutions is included in its constants. \begin{table}[ht] \centering \small \begin{tabular}{@{}>{\raggedright\arraybackslash}p{.40\linewidth}ll@{}} \toprule Operation & Normalized cost & Power of $p$\\ \midrule Prefix-slot moves and individual butterfly rounds & $dK$ & $\epsilon(1+c)$\\ Chunk exchanges for transform layout & $pK^{\tau-1}$ & $1-\epsilon c(1-\tau)$\\ Simultaneous butterfly rounds & $\ell d^{\lambda'}$ & $1-\epsilon(1-\lambda')$\\ CRT and axis layouts & $d^2(1+\ell^\tau)$ & $\tau+\epsilon(2-\tau)$\\ Gaussian line maps & $dp^{1/2+\delta}\alpha$ & $3/4+\delta+3\epsilon/2$\\ Chirps, twists and scalar products & $dp^\delta$ & $\epsilon+\delta$\\ Packed polynomial products & $\log(rp)$ & $1-\epsilon$\\ Final scaling and rounding & $p^\delta$ & absorbed above\\ Input, padding, carries and output & $1$ & absorbed above\\ \bottomrule \end{tabular} \caption{Costs divided by $Tp$; each coefficient representation has $O(p)$ bits.}\label{tab:costs} \end{table}

**S6.B**: Seven dominant powers have margins below one, $g_1,\dots,g_7$, with $g_2=2^{-181}$, $g_3=2^{-129}$, $g_7=2^{-75}$ and the others larger, so $\min_i g_i=2^{-181}=2\kappa$.
Source: "g_1>\frac12,\quad g_2=2^{-181},\quad g_3=2^{-129},\quad" (paper)
> The seven dominant powers in Table~\ref{tab:costs} have the following margins below one: \begin{equation}\label{eq:margin-list} \begin{split} g_1&=1-\epsilon(1+c),\qquad g_2=\epsilon c(1-\tau),\qquad g_3=\epsilon(1-\lambda'),\\ g_4&=1-\tau-\epsilon(2-\tau),\qquad g_5=1/4-\delta-3\epsilon/2,\\ g_6&=1-\delta-\epsilon,\qquad g_7=\epsilon. \end{split} \end{equation} Direct substitution gives \[ g_1>\frac12,\quad g_2=2^{-181},\quad g_3=2^{-129},\quad g_4>2^{-51},\quad g_5>\frac18,\quad g_6>\frac12,\quad g_7=2^{-75}. \] Thus $\min_i g_i=2^{-181}=2\kappa$. This explicit slack will cover any remaining fixed power of $\log p$.

**S6.C**: Each normalized cost row is bounded by the cost of the procedure proved earlier for that stage.
How: Bound each stage by the time of the previously proved procedure and divide by the volume $Tp$.
- S6.C1: The first three rows are the costs of the synthetic transform: prefix and individual rounds $O(dK)$, chunk exchanges $O(pK^{\tau-1})$ and simultaneous rounds $O(\ell d^{\lambda'})$ per unit volume.
  Source: "The first three rows are the terms in" (paper)
  > Here $d^2$ is absorbed by $d^2\ell^\tau$, and $\log(rp)=O(p^{1-\epsilon}+\log p)=O(p^{1-\epsilon})$. The first three rows are the terms in \eqref{eq:synthetic-transform-cost}. The prefix-slot moves and individual butterfly rounds in Lemma~\ref{lem:synthetic-transform-cost} use $O(dK)$ linear scans. Its positional layout uses $O(p/K)$ exchanges of width-$K$ chunks, because $d\ell=O(p)$. Each exchange costs $O(VK^\tau)$ by Lemma~\ref{lem:chunk-swap}, so the normalized total is \[ O(p/K)\,K^\tau=O(pK^{\tau-1}). \] For the at most $\ell$ simultaneous rounds, Proposition~\ref{prop:simultaneous-layer} supplies $O(Vd^{\lambda'})$ per round. Its bound already includes the binary basis changes, deterministic repair, row padding and removal, all base-$m$ groups, and fixed-tape cleanup.
- S6.C2: Axis exposure and restoration take $O(d^2)$ adjacent moves, each $O(V(1+\ell^\tau))$, and the CRT rotations add only $O(dV)$.
  Source: "Axis exposure and restoration require $O(d^2)$ adjacent axis moves." (paper)
  > Axis exposure and restoration require $O(d^2)$ adjacent axis moves. Each costs $O(V(1+\ell^\tau))$: peel a single excess bit when widths differ by one, exchange their equal-width parts, and restore the bit. The $d$ CRT rotations copy valid and invalid intervals in $O(dV)$ time. Their offset setup was bounded above by $O(dTp^C/r)=o(Tp)$, using at most $T/t_i$ control prefixes for target $i$.
- S6.C3: Gaussian resampling processes at most $T/t_i$ lines of length $t_i$ on axis $i$, summing to $O(dTp^{3/2+\delta}\alpha)$.
  Source: "For Gaussian resampling, axis $i$ has at most $T/t_i$ processed lines." (paper)
  > For Gaussian resampling, axis $i$ has at most $T/t_i$ processed lines. The line-machine costs therefore sum to \[ \sum_{i=1}^d\frac{T}{t_i} O(t_i p^{3/2+\delta}\alpha) =O(dTp^{3/2+\delta}\alpha). \] Dividing by $V=Tp$ gives the fifth row. The factor $\alpha=\Theta(p^{1/4+\epsilon/2})$ explains its displayed power of $p$.
- S6.C4: Chirps, twists and scalar products cost $O(dp^{1+\delta})$ per entry and the packed polynomial products cost $O(Tp\log(rp))$ using the established multiplier, which is not the improved algorithm.
  Source: "Chirp numerators have $O(p)$ bits and can be computed modulo $2r$ as" (paper)
  > Chirp numerators have $O(p)$ bits and can be computed modulo $2r$ as sums of $d$ terms $s_i j_i^2(r/t_i)$. Computing their squares and products with the previously established $O(N\log(2N))$ multiplier on $N$-bit inputs, then generating the exponentials, costs $O(dp^{1+\delta})$ per entry. The smaller twist and scalar pointwise-product costs fit the same row: they use a fixed number of exact products on $O(p)$-bit component numerators, followed by linear truncation scans. The two final products by $S$ also cost $O(p\log p)$ per entry, so their normalized cost is $O(\log p)=O(p^\delta)$ as listed separately. There are $T/r$ polynomial records in each pointwise ring product. Four integer products on $O(rp)$ bits per record therefore cost $O(Tp\log(rp))$ in total. Signed packing, centered extraction, negacyclic wrapping, and copying are linear stream passes. These calls use the previously established multiplier on records of length $O(rp)=o(n)$; they do not call the improved algorithm being constructed. The exact source scaling by $2^\gamma$ is a shift of $O(p)$-bit numerators by $\gamma=O(p)$ places, so it is a linear scan per entry and fits the final-scaling row. The scalar-format adapters and the exact removal of sign-extension bits after the ring and source scales also use linear scans of their $O(p)$-bit components, within the listed scan costs.
- S6.C5: Setup at recursion nodes, descriptor work and the sorting of exceptional streams are $o(Tp)$ in total, since $r$ exceeds every fixed power of $p$ and the exceptional fraction is $O(d2^{-K})$.
  Source: "\paragraph{Setup and exceptional streams across all calls.}" (paper)
  > \paragraph{Setup and exceptional streams across all calls.} We next separate setup at recursion nodes from work repeated for every polynomial record or every Gaussian line. Here a visit means a node of the layer or address-layout recursions, or one of their round and group invocations. The internal recursion of the established integer multiplier and the iterations of the quoted Gaussian algorithms are already included in their separately charged running times. The layer and address-layout recursions have constant branching and depth $O(\log p)$, and the number of their rounds and base-$m$ groups is polynomial in $p$. Choose a fixed exponent $A$ so that their total number of visits is at most $p^A$, and a fixed $C$ bounding descriptor setup performed once per visit by $p^C$ steps. This setup then costs at most $p^{A+C}=o(Tp)$.

**S6.D**: Each of the seven powers is at most $1-2\kappa$ and any fixed power of $\log p$ is at most $p^\kappa$ eventually, so the total time is $O(Tp\,p^{1-\kappa})=O(n(\log n)^{1-\kappa})$, with schoolbook multiplication below a fixed cutoff.
Source: "Each of the seven displayed powers is at most $1-2\kappa$ by" (paper)
> Each of the seven displayed powers is at most $1-2\kappa$ by \eqref{eq:margin-list}. Any fixed power of $\log p$ is at most $p^\kappa$ eventually. The complete large-input bound is consequently \[ O(Tp\,p^{1-\kappa})=O(n(\log n)^{1-\kappa}). \] Fix one cutoff beyond all the size, precision, margin and layout conditions, and use schoolbook multiplication below it. The finitely many smaller lengths are included by enlarging the implicit constant. With $\lg n=\max\{\lceil\log_2n\rceil,1\}$, the resulting single fixed-alphabet, fixed-tape deterministic machine has worst-case time $O(n(\lg n)^{1-\kappa})$ for every input length and produces precisely the required $2n$-bit product. \section{Exact division and integer square root} \label{sec:exact-arithmetic-proof}

### Other pages these steps use

Link each where the derivation uses it, stating what it gives; do not derive its results again.

- [Synthetic transform layout](transform-layout.md)
- [Signed packing for exact polynomial products](kronecker-packing.md)
- [Chinese-remainder map to prime cyclic axes](crt-prime-axes.md)
- [Resampling with the permutations left in the transform](sort-free-resampling.md)
- [Harvey–van der Hoeven O(n log n) multiplier](hvdh-multiplier.md)
- [Source transform procedure](source-transform.md)
- [Coefficient phase twist](phase-twist.md)
- [Packed selected-bit addition](packed-selected-bit-addition.md)
- [Fixed-tape depth-first recursion schedule](fixed-tape-recursion.md)

## Prerequisite pages (link each on first use, exactly as written, and give a relation for each)

- [The multiplication algorithm](multiplication-algorithm.md): Digits are placed on prime axes by the CRT map, two source transforms are taken, multiplied pointwise, the opposite-sign transform and scalings applied, results rounded to integers, mapped back, and carries propagated to give the 2n-bit product.
  Shown as: Total time bound and the power saving *<predicate>* The multiplication algorithm
- [Fast address-chunk interchange](address-chunk-interchange.md): Swapping two disjoint equal-width address chunks of u bits in an array of V bits in O(V u^τ) time for a fixed τ < 1, via XOR-combined intermediate data though the final effect is an exact permutation.
  Shown as: Total time bound and the power saving *<predicate>* Fast address-chunk interchange
- [Simultaneous normalized butterfly layer](simultaneous-butterfly-layer.md): Applying H₀ on the same bit position of up to d address chunks at once in O(V d^λ′) time, λ′ < 1, by the recursive phase network instead of d separate passes.
  Shown as: Total time bound and the power saving *<predicate>* Simultaneous normalized butterfly layer
- [Tensor resampling on a padded box](tensor-resampling.md): Applying the one-dimensional resampling maps A and B₀ line by line along each of d axes of a zero-padded power-of-two box, moving axes into place with chunk swaps.
  Shown as: Total time bound and the power saving *<predicate>* Tensor resampling on a padded box
- [Convolution from the retained transform layout](synthetic-convolution.md): Convolution over the polynomial ring: two forward synthetic transforms, pointwise signed-packed polynomial products in the retained order, then the opposite transform and exact scaling, with error below √2·M(3ℓ+1)2^{−p}.
  Shown as: Total time bound and the power saving *<predicate>* Convolution from the retained transform layout
- [Parameter choices for the padded box](parameter-choice.md): The sizes b = ⌈log₂n⌉, precision p = 6b, number of axes d = ⌊b^ε⌋, chunk width K = ⌊d^c⌋, transform size T ≈ n/b and axis lengths r/2 or r with r = 2^{Θ(p/d)}.
  Shown as: Total time bound and the power saving *<predicate>* Parameter choices for the padded box

## Pages that build on this one

- Integer multiplication in O(n (lg n)^(1−κ)) time: A deterministic fixed-tape Turing machine multiplies two n-bit integers exactly in O(n (lg n)^{1−κ}) worst-case time with κ = 2^{-182}, disproving the n log n optimality conjecture in this model; it works by two fast tape procedures (address interchange and simultaneous butterfly layers) inside a Gaussian-resampling, chirp and synthetic-transform convolution.

## Other pages you may link to

- [Cyclic convolution](convolution.md)
- [Polynomial ring modulo y^r + 1](ring-y-r-plus-1.md)
- [Gaussian dyadic numbers](gaussian-dyadics.md)
- [Coefficient phase twist](phase-twist.md)
- [Fixed multitape Turing machine model](multitape-model.md)
- [Schönhage–Strassen n log n conjecture and prior bounds](nlogn-conjecture-history.md)
- [Harvey–van der Hoeven O(n log n) multiplier](hvdh-multiplier.md)
- [Signed packing for exact polynomial products](kronecker-packing.md)
- [Elementary stream operations](stream-ops.md)
- [Controlled cyclic shifts](controlled-shifts.md)
- [Subspaces, rank and orthogonal complements](linear-algebra.md)
- [Scratch-restoring linear network](scratch-network.md)
- [Frames and the common-frame identity](frames.md)
- [Nested subspace labels from three-element subsets](subspace-labels.md)
- [Rank saving s < Wm](rank-saving.md)
- [Bit network and rational matrix interface](bit-network-interface.md)
- [Complex phase network](phase-network.md)
- [Lower triangular factorization of a matrix](bruhat-factorization.md)
- [Cost of a rational matrix shear](matrix-shear.md)
- [Volume-normalized power-saving recurrence](power-saving-recurrence.md)
- [Fixed-tape depth-first recursion schedule](fixed-tape-recursion.md)
- [Power-width address interchange](power-width-interchange.md)
- [Packed selected-bit addition](packed-selected-bit-addition.md)
- [Disk-grid arrays, contractions and truncation](disk-grid-precision.md)
- [Guard bits and a single final truncation](guard-width-exactness.md)
- [Normalized discrete Fourier transform](dft.md)
- [Synthetic roots of unity](synthetic-roots.md)
- [Butterfly layers (fast Fourier transform)](fft-butterfly.md)
- [Convolution via transforms](convolution-theorem.md)
- [Multidimensional transform over several axes](multidim-transform.md)
- [Synthetic polynomial transform](synthetic-transform.md)
- [Gaussian resampling](gaussian-resampling.md)
- [Bluestein chirp identity](bluestein-chirp.md)
- [Synthetic transform layout](transform-layout.md)
- [Resampling with the permutations left in the transform](sort-free-resampling.md)
- [Cancellation of the retained source permutation](source-permutation-cancellation.md)
- [Multiplication as radix-digit convolution](radix-digit-reduction.md)
- [Chinese-remainder map to prime cyclic axes](crt-prime-axes.md)
- [Prime source lengths near powers of two](prime-source-lengths.md)
- [Source transform procedure](source-transform.md)
- [Precision and exact recovery](exact-recovery.md)
- [Integer multiplication in O(n (lg n)^(1−κ)) time](main-theorem.md)

## Evidence from the sources, with the paragraph around each quote

- "Thus $\min_i g_i=2^{-181}=2\kappa$." (paper)

  > The seven dominant powers in Table~\ref{tab:costs} have the following margins below one: \begin{equation}\label{eq:margin-list} \begin{split} g_1&=1-\epsilon(1+c),\qquad g_2=\epsilon c(1-\tau),\qquad g_3=\epsilon(1-\lambda'),\\ g_4&=1-\tau-\epsilon(2-\tau),\qquad g_5=1/4-\delta-3\epsilon/2,\\ g_6&=1-\delta-\epsilon,\qquad g_7=\epsilon. \end{split} \end{equation} Direct substitution gives \[ g_1>\frac12,\quad g_2=2^{-181},\quad g_3=2^{-129},\quad g_4>2^{-51},\quad g_5>\frac18,\quad g_6>\frac12,\quad g_7=2^{-75}. \] Thus $\min_i g_i=2^{-181}=2\kappa$. This explicit slack will cover any remaining fixed power of $\log p$.

- "Each of the seven displayed powers is at most $1-2\kappa$ by" (paper)

  > Each of the seven displayed powers is at most $1-2\kappa$ by \eqref{eq:margin-list}. Any fixed power of $\log p$ is at most $p^\kappa$ eventually. The complete large-input bound is consequently \[ O(Tp\,p^{1-\kappa})=O(n(\log n)^{1-\kappa}). \] Fix one cutoff beyond all the size, precision, margin and layout conditions, and use schoolbook multiplication below it. The finitely many smaller lengths are included by enlarging the implicit constant. With $\lg n=\max\{\lceil\log_2n\rceil,1\}$, the resulting single fixed-alphabet, fixed-tape deterministic machine has worst-case time $O(n(\lg n)^{1-\kappa})$ for every input length and produces precisely the required $2n$-bit product. \section{Exact division and integer square root} \label{sec:exact-arithmetic-proof}


## Sources (read from disk as needed)

- `sources/paper.tex`: source id `paper`, "Integer multiplication below n log n (OpenAI, 2026)"

## Length

At most 350 words, not counting mathematics and citations: submit rejects a longer page.

## Answer

The answer is two files: the page body as markdown (not JSON), and a JSON object with the symbols the page binds and its relations to its prerequisite pages. In JSON strings, write each backslash of LaTeX twice (`"\\kappa"`). The JSON follows this schema:

```json
{
  "$defs": {
    "Derivation": {
      "additionalProperties": false,
      "description": "Where the page derives one of the steps the argument gives it.",
      "properties": {
        "quote": {
          "description": "The passage on this page that derives the step, copied exactly: a sentence or more,\noutside citations.",
          "type": "string"
        },
        "step": {
          "description": "The step's id, as listed under \"Steps this page derives\".",
          "type": "string"
        }
      },
      "required": [
        "step",
        "quote"
      ],
      "type": "object"
    },
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
    "derivations": {
      "default": [],
      "description": "For each step listed under \"Steps this page derives\", the passage on the page that\nderives it. Empty if the task lists none.",
      "items": {
        "$ref": "#/$defs/Derivation"
      },
      "type": "array"
    },
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

Write the body to `exegenius/tasks/0052-write-time-accounting.answer.md` and the JSON to `exegenius/tasks/0052-write-time-accounting.answer.json`, then run `exegenius submit 0052-write-time-accounting` in the project directory. If submit rejects the answer, fix the answer and submit again.

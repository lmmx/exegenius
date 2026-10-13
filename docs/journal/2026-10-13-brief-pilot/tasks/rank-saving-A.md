# exegenius task 0018-write-rank-saving

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
- Polynomial ring modulo y^r + 1
  - $y$ (ring variable): The polynomial indeterminate, whose powers act as roots of unity by signed shifts of the coefficient list.
  - $r$ (record length): The number of complex coefficients in a polynomial record, a power of two.
- Restoring arbitrary scratch values
- Three-element subset gadget
- Three-stage bank exchange network
- Frames: changing representation along the network
- Nested subspace labels and orthogonal residuals
- Role streams and fixed-tape depth-first scheduling
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

Concept: Strict total rank deficit s < W·m
Gloss: The sum s of edge ranks (bit network) or residual dimensions (complex network) over a network with W wires and m-dimensional labels is strictly less than W·m, so the number of smaller recursive calls per wire is below m.

## This page in the argument

The step: Two fixed linear networks with $W$ wires exchange two banks of values, restore every scratch wire, and have edge ranks (bit network) or residual dimensions (complex network) summing to $s<Wm$, where $m$ is the dimension of the label space.
How: Build the networks from the three-subset gadget, attach frames and nested subspace labels to their gates, and sum the dimension changes along the edges
Why: The recursion saves a power only if the number of smaller calls per wire is below the dimension, and the labels turn the cost of changing representation along an edge into a dimension change that can be counted exactly. (the sources leave this for the reader to work out: spell it out)
Striking feature: The deficit $Wm-s$ is only a minute fraction of $Wm$ (of order $10^{-11}$), yet its strict positivity is all the power saving needs. (explain why it holds, from the method)

Its sub-steps, in order:

- A forward invocation of the three-subset gadget adds $x_S$ to $y_S$ for arbitrary scratch values, and applying it at stage 1, its inverse at stage 2 and it again at stage 3 sends $(X_a,Y_a)$ to $(-Y_a,X_a)$ with every scratch wire restored. (on [Three-stage bank exchange network](bank-exchange-network.md))
- If every source, gate and sink carries an invertible frame $D_v$ and each edge applies $D_vD_u^{-1}$, the common frame commutes with each pointwise linear gate, so intermediate frames cancel and a route applies only the endpoint operator. (on [Frames: changing representation along the network](frames.md))
- Gate and terminal labels are nondegenerate subspaces, adjacent labels are nested, an edge's orthogonal residual has dimension equal to the change in label dimension, and only the central edges from time 3 to time 4 decrease, each losing $h$ dimensions. (on [Nested subspace labels and orthogonal residuals](subspace-labels.md))
- For the bit network the rational ranks of all edge differences sum to $s_{\rm b}=W_{\rm b}m-N+2L_{\rm b}<W_{\rm b}m$, and the endpoint difference on each role is $I_m$.
- In the complex network with binary labels each edge's frame change is a product of two-point kernels $aI+bX_v$, one per residual basis vector, the residual dimensions sum to $s_{\rm c}<W_{\rm c}m$, and the endpoint operator on every role is $C^{\otimes m}$. (on [Phase-frame (complex) network for the butterfly](phase-network.md))
- With $\tau=\sigma=1-2^{-50}$ the ratios satisfy $s_{\rm b}/W_{\rm b}<m^\tau$ and $s_{\rm c}/W_{\rm c}<m^\sigma$, so each wire makes fewer than $m^\tau$ smaller calls.

It serves these steps, from the main result down:

- One deterministic fixed-tape Turing machine multiplies $n$-bit integers exactly in $O(n(\lg n)^{1-\kappa})$ time with $\kappa=2^{-182}$, disproving the $n\log n$ optimality conjecture in this model.; how: Reduce multiplication to a convolution computed by synthetic polynomial transforms after Gaussian resampling and chirps, and speed up their address moves and butterfly layers with two network-based tape procedures (on [Power-saving integer multiplication](power-saving-multiplication.md))

## Prerequisite pages (link each on first use, exactly as written, and give a relation for each)

- [Nested subspace labels and orthogonal residuals](subspace-labels.md): Each gate and terminal gets a nondegenerate subspace of a fixed bilinear space, adjacent labels are nested, and an edge's orthogonal residual (the complement of the smaller label in the larger) has dimension equal to the change in label dimension; the gadget's triple lines are chosen so the total dimension lost stays small.
  Shown as: Strict total rank deficit s < W·m *<predicate>* Nested subspace labels and orthogonal residuals
- [Three-stage bank exchange network](bank-exchange-network.md): Applying the forward invocation at stage 1, its inverse at stage 2 and the forward invocation at stage 3 sends (X,Y) to (−Y,X) with every scratch wire restored; the bit version is a pure exchange using XOR gates, the complex version uses Gaussian dyadic values.
  Shown as: Strict total rank deficit s < W·m *<predicate>* Three-stage bank exchange network

## Pages that build on this one

- Recursive power saving from s/W < m: A call on parameter m·f makes s calls on parameter f over volume V/W each, so normalized cost satisfies F_k ≤ (s/W)F_{k-1} + O(1) with s/W < m, which sums to O(m^{kτ}) for some τ < 1.
- Bit network as an address shear: With frames given by shears Φ_M that map address (H,D) to (H+MD,D), the bit network's endpoint operator on every role is the shear H ← H+D, and each edge costs as many smaller interchanges as its rational rank.
- Phase-frame (complex) network for the butterfly: The complex bank-exchange network with binary labels and frames C_U = H^{⊗m} diag(i^{wt(P_U x) mod 4}) H^{⊗m}, in which an edge's frame change is a product of two-point kernels aI + bX_v, one per residual basis vector, and the endpoint operator is the tensor power C^{⊗m} of the kernel C = H·diag(1,i)·H.

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
- [Role streams and fixed-tape depth-first scheduling](role-streams.md)
- [Frames: changing representation along the network](frames.md)
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
- [Exact Gaussian dyadic arithmetic and guard widths](exact-dyadic-guard.md)
- [Chinese-remainder axis map to prime cyclic axes](crt-axis-map.md)
- [Gaussian resampling](gaussian-resampling.md)
- [Bluestein chirp identity](bluestein-chirp.md)
- [Phase twist from cyclic to negacyclic convolution](phase-twist.md)
- [Resampling with the permutations left in the transform](permutations-left-in-transform.md)
- [Recursive implementation of the phase network on selected bits](phase-edge-recursion.md)
- [Simultaneous butterfly layer in O(V·d^λ′)](simultaneous-butterfly-layer.md)
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

- "The decisive property is that the total number $s$ of smaller calls satisfies $s<Wm$." (paper)

  > The finite construction in Section~\ref{sec:finite-networks} attaches subspaces of a fixed $m$-dimensional space to its gates and terminals. The subspaces at adjacent vertices are nested. Their dimension changes count the smaller operations needed on an edge, with an additional source contribution in the bit construction. The construction uses three-element subsets: gather and scatter create coefficients depending on intersection size, and auxiliary wires cancel the unwanted coefficients. Orthogonality of the corresponding indicator vectors permits the subspace assignments. The decisive property is that the total number $s$ of smaller calls satisfies $s<Wm$.

- "We will obtain $L<N/2$." (paper)

  > The ambient label space is $\mathcal F=F^{\otimes3}$ with its tensor bilinear form. It has dimension $m$ and is nondegenerate. Write \[ u_a=t_{a_1}\otimes t_{a_2}\otimes t_{a_3},\qquad U_a=\langle u_a\rangle. \] The prescribed terminal labels are \[ \begin{array}{c|cc} \text{role}&\text{source label}&\text{sink label}\\ \hline X_a&U_a&\mathcal F\\ Y_a&0&U_a^\perp\\ \text{each scratch role}&0&\mathcal F. \end{array} \] Write $W$ for the wire count in the construction under consideration. The source dimensions sum to $N$ and the sink dimensions to $Wm-N$. Thus the signed dimension changes will sum to $Wm-2N$, because the contributions at gates cancel. If decreasing edges lose a total dimension $L$, the sum of absolute changes is $Wm-2N+2L$. We will obtain $L<N/2$. The rational interface below will require an additional $N$ source ranks, so this inequality will leave a strict saving in both constructions.


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

Write the body to `exegenius/tasks/0018-write-rank-saving.answer.md` and the JSON to `exegenius/tasks/0018-write-rank-saving.answer.json`, then run `exegenius submit 0018-write-rank-saving` in the project directory. If submit rejects the answer, fix the answer and submit again.

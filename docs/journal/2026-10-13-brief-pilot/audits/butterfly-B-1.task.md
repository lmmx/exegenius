# audit of `simultaneous-butterfly-layer`

Audit one page of an explanatory wiki on behalf of a specific reader. Use only this task file: do not read the sources, other pages or any other file.

The reader knows the concepts listed as known, and everything the reader profile covers: what anyone with that background has been taught, whether or not the profile names it (a profile of undergraduate computer science covers logarithms and proof by induction without naming them). Anything outside that, the reader does not know: treat it as unknown to them, however elementary it seems to you. You are simulating that reader's lack of knowledge, so use your own knowledge only to recognise what the reader is missing.

Symbols: account for every symbol in the page's mathematics, each once, saying where the reader learns what it denotes:
- `here`: the page says what it denotes; quote the sentence that says it. A use in an example ("take $r = 4$") or a range ("a power of two $r$") does not say what $r$ denotes or counts. If the page uses the symbol before that sentence, submit records that the meaning comes too late.
- `page`: an earlier page binds it, as listed under "Symbols bound on earlier pages".
- `concept`: the name of a known concept contains it and binds it, as listed under that concept. A symbol in a known concept's name that is listed with no meaning is not known.
- `background`: standard notation the reader profile covers; say what covers it.
- `unbound`: nothing tells the reader what it denotes.
Submit checks that each symbol is in the page's mathematics, that each quote is on the page and contains the symbol, and that the named page or concept binds it.

Terms and steps: report what would stop this reader at this point on the page. A term or step is followable if the reader knows it, the page explains it, or the page links it to another wiki page. A claim marked "(cited)" is attributed to the source. A citation lets the reader check a result, not follow an argument: a cited result the page uses is followable if the page says what it states and why it applies where it is used, and its proof can be left to the source. A cited claim that stands in for how the page's own concept works, or whose content or role the page does not say, is a step.
- term: a named concept the reader does not know, which the page neither explains nor links. References to the documents themselves (the paper, the sources, the target result) are not terms.
- step: an inference, calculation or claim that the page presents as following from what it gives, which the reader cannot check from what the page gives. A claim about an identity, construction or definition that the page never states is a step.

Copy each quote from the page exactly, character for character, and keep it short: submit rejects quotes it cannot find on the page. Do not report anything the reader can follow. Answer with an empty list of gaps if the reader can follow the whole page.

## Reader profile

Undergraduate computer science: big-O notation, polynomials, complex numbers, basic Turing machines and tapes

For these sources: The reader has an undergraduate computer science degree: asymptotic analysis, discrete mathematics (sets, functions, combinatorial counting, induction, basic modular arithmetic and number theory such as primes, gcd and the Euclidean algorithm), recursion and elementary algorithms (sorting, stacks, divide and conquer), basic Turing machines with several tapes, and the first-year mathematics that goes with it: single-variable calculus (limits, convergence of sequences and series, basic integrals, geometric series), matrices and vectors, polynomials and complex numbers. The reader has not met the discrete Fourier transform or the FFT, abstract algebra beyond modular arithmetic (rings, quotient rings, algebra isomorphisms, cyclic-group structure), Gaussian integers, number-theoretic results on the density of primes, or any integer-multiplication algorithm faster than the schoolbook method.

## Known concepts

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

## Symbols bound on earlier pages

None of this page's symbols is bound on an earlier page.

## Pages in the wiki

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
- [Role streams and fixed-tape depth-first scheduling](role-streams.md)
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

## Page to audit (`simultaneous-butterfly-layer`)

````markdown
# Simultaneous butterfly layer in O(V·d^λ′)

A simultaneous butterfly layer is the second tape procedure of the main result. Let $d$ bound the number of chunks, and $D\le d$ be the number in a layer, each of $K=\lfloor d^c\rfloor$ bits, where $c>0$ is a fixed number. It applies the two-point butterfly $H_0(u,v)=((u+v)/2,(u-v)/2)$ at one bit offset in up to $d$ address chunks. Write $V$ for the logical bit volume and $\lambda'<1$ for a fixed constant. It costs $O(Vd^{\lambda'})$ (cited) rather than one whole-array pass per chunk.

Coefficients lie on $2^{-p}Z[i]$, with $p$ the number of fractional bits.

Let $C_{00}=C_{11}=(1+i)/2$ and $C_{01}=C_{10}=(1-i)/2$ be the kernel $C$, and let $\Xi$ be the diagonal matrix with entries $1$ and $i$. Since $H_0=((1-i)/2)\,\Xi C\,\Xi$, the layer is
$$H_0^{\otimes D}=((1-i)/2)^D\,\Xi^{\otimes D}\,C^{\otimes D}\,\Xi^{\otimes D}.$$
Here $\Xi^{\otimes D}$ multiplies address $x$ by $i^{x_1+\cdots+x_D}$, its selected bits $x_1,\ldots,x_D$.

The network has $W$ wire roles. It splits $e=mf$ chunks into $m$ slots of $f$ chunks, and makes $s$ child calls, each computing $C^{\otimes f}$ on one slot. Thus $C^{\otimes e}$ is computed by [Recursive implementation of the phase network on selected bits](phase-edge-recursion.md).

The recursion is scheduled by [Role streams and fixed-tape depth-first scheduling](role-streams.md). A row $h=Wg+w$ goes to role $w$ of $W$, indexed there by $g$.

Let $F(e)$ be the time per unit volume. Put $a=s/W$. The binary changes of basis cost $O(V(L^\tau+1))$ by [Packed selected-bit addition with guard repair](packed-selected-bit-addition.md), with $L=fK$ and fixed $0<\tau<1$. Hence $F(e)\le aF(e/m)+O((eK)^\tau+1)$, since the $s$ child calls run on volume $sV/W$. Here $a<m$ by [Recursive power saving from s/W < m](recursive-power-saving.md): each level divides chunks by $m$ but multiplies work by $a$.

Fix exponents $\sigma$ and $\mu$ with $\max\{0,\log_m a\}\le\sigma$, $\max\{\tau,\sigma\}<\mu<1$ and $\tau(1+2c)<\mu$. Recurse while $e\ge\sqrt d$, else apply $e$ kernels individually. There $K\le e^{2c}$, so overhead is $O(e^\mu)$. Depth-$j$ nodes have $e/m^j$ chunks and weight $a^j$; as $a<m^\mu$, they sum to
$$\sum_j a^j\,O\bigl((e/m^j)^\mu\bigr)=O(e^\mu).$$
At the stopping depth, leaf groups of $k=e/m^j<\sqrt d$ chunks cost $a^jk\le e^\sigma k^{1-\sigma}\le e^\sigma d^{(1-\sigma)/2}$. Hence $F(e)=O(d^\mu+d^{\sigma+(1-\sigma)/2})$ for $e\le d$.

The first $q_0=O(\log d)$ chunks supply the row indices, at cost $O(V\log d)$. Writing $D-q_0=\sum_\ell\delta_\ell m^\ell$ in base $m$ gives $\delta_\ell$ blocks of $m^\ell$ chunks. Summing gives
$$V\cdot O\bigl(\log(2d)\,[d^\mu+d^{\sigma+(1-\sigma)/2}+1]\bigr)=O(Vd^{\lambda'})$$
for any $\max\{\mu,\sigma+(1-\sigma)/2\}<\lambda'<1$.

The whole layer is evaluated exactly in [Exact Gaussian dyadic arithmetic and guard widths](exact-dyadic-guard.md), with $p+\Delta$ fractional bits and $\Delta+3$ integer and sign bits, where $\Delta=o(p)$ is the guard width. Packed repairs stay cheap: $80(f-1)2^{-K}\to0$.

One truncation toward zero moves each component by less than $2^{-p}$, so the error is below $\sqrt2\,2^{-p}$. Values stay in the unit disk, since $\sqrt2\,H_0$ is unitary.
````

## Answer

The answer is one JSON object following this schema:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "AuditAnswer",
  "type": "object",
  "properties": {
    "gaps": {
      "description": "Every term or step the reader cannot follow; empty if the reader can follow them all.",
      "type": "array",
      "items": {
        "$ref": "#/$defs/Gap"
      }
    },
    "symbols": {
      "description": "Every symbol in the page's mathematics, each once, with where the reader learns what\nit denotes. Empty only if the page has no mathematics.",
      "type": "array",
      "items": {
        "$ref": "#/$defs/SymbolUse"
      }
    }
  },
  "additionalProperties": false,
  "required": [
    "symbols",
    "gaps"
  ],
  "$defs": {
    "Anchor": {
      "description": "Where the reader learns what a symbol denotes.",
      "oneOf": [
        {
          "description": "This page says what the symbol denotes. A use in an example (\"take $r = 4$\") does not.",
          "type": "object",
          "properties": {
            "kind": {
              "type": "string",
              "const": "here"
            },
            "quote": {
              "description": "The sentence that says it, copied exactly from the page.",
              "type": "string"
            }
          },
          "additionalProperties": false,
          "required": [
            "kind",
            "quote"
          ]
        },
        {
          "description": "An earlier page binds it: one listed under \"Symbols bound on earlier pages\".",
          "type": "object",
          "properties": {
            "kind": {
              "type": "string",
              "const": "page"
            },
            "page": {
              "description": "Id of that page.",
              "type": "string"
            }
          },
          "additionalProperties": false,
          "required": [
            "kind",
            "page"
          ]
        },
        {
          "description": "The name of a known concept contains it and binds it, as listed under the known\nconcepts.",
          "type": "object",
          "properties": {
            "concept": {
              "description": "Id of that concept.",
              "type": "string"
            },
            "kind": {
              "type": "string",
              "const": "concept"
            }
          },
          "additionalProperties": false,
          "required": [
            "kind",
            "concept"
          ]
        },
        {
          "description": "Standard notation the reader profile covers, such as $n$ for an input size.",
          "type": "object",
          "properties": {
            "kind": {
              "type": "string",
              "const": "background"
            },
            "note": {
              "description": "What in the reader profile covers it.",
              "type": "string"
            }
          },
          "additionalProperties": false,
          "required": [
            "kind",
            "note"
          ]
        },
        {
          "description": "Nothing tells the reader what it denotes.",
          "type": "object",
          "properties": {
            "kind": {
              "type": "string",
              "const": "unbound"
            },
            "note": {
              "description": "What the reader cannot tell.",
              "type": "string"
            },
            "quote": {
              "description": "Where it is first used, copied exactly from the page.",
              "type": "string"
            }
          },
          "additionalProperties": false,
          "required": [
            "kind",
            "quote",
            "note"
          ]
        }
      ]
    },
    "Gap": {
      "type": "object",
      "properties": {
        "kind": {
          "$ref": "#/$defs/GapKind"
        },
        "note": {
          "description": "Why this reader cannot follow it.",
          "type": "string"
        },
        "quote": {
          "description": "The passage on the page where the gap occurs, copied exactly (a phrase or one sentence).",
          "type": "string"
        },
        "term": {
          "description": "The term, or a short label for the step.",
          "type": "string"
        }
      },
      "additionalProperties": false,
      "required": [
        "term",
        "quote",
        "kind",
        "note"
      ]
    },
    "GapKind": {
      "oneOf": [
        {
          "description": "A named concept the reader does not know, which the page neither explains nor links.",
          "type": "string",
          "const": "term"
        },
        {
          "description": "An inference, calculation or claim the reader cannot check from what the page gives.",
          "type": "string",
          "const": "step"
        }
      ]
    },
    "SymbolUse": {
      "type": "object",
      "properties": {
        "anchor": {
          "$ref": "#/$defs/Anchor"
        },
        "symbol": {
          "description": "The symbol in LaTeX, as it appears in the page's mathematics, without dollar signs.",
          "type": "string"
        }
      },
      "additionalProperties": false,
      "required": [
        "symbol",
        "anchor"
      ]
    }
  }
}
```

Write the answer as JSON to `<audits>/bfly-B-1.audit.json` (an absolute path). Do not run any other command.

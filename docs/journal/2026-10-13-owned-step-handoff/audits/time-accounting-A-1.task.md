# audit of `time-accounting`

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

For these sources: The reader is an undergraduate computer scientist: discrete mathematics (induction, modular arithmetic, gcd, permutations, relations, graphs), algorithms (divide and conquer, recurrences, schoolbook and Karatsuba multiplication, recursion), single-variable calculus (limits, convergence of sequences and series, basic integrals, exponentials), and introductory linear algebra (matrices, linear maps, invertibility, Gaussian elimination). They know complex numbers with roots of unity, polynomials, big-O and its relatives, and elementary Turing machines with tapes. They have not studied abstract algebra beyond residue rings, Fourier analysis, the fast Fourier transform or the discrete Fourier transform, numerical error analysis, analytic number theory, or multitape-machine algorithm design.

## Known concepts

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

## Symbols bound on earlier pages

None of this page's symbols is bound on an earlier page.

## Pages in the wiki

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
- [Fast address-chunk interchange](address-chunk-interchange.md)
- [Packed selected-bit addition](packed-selected-bit-addition.md)
- [Disk-grid arrays, contractions and truncation](disk-grid-precision.md)
- [Guard bits and a single final truncation](guard-width-exactness.md)
- [Normalized discrete Fourier transform](dft.md)
- [Synthetic roots of unity](synthetic-roots.md)
- [Butterfly layers (fast Fourier transform)](fft-butterfly.md)
- [Simultaneous normalized butterfly layer](simultaneous-butterfly-layer.md)
- [Convolution via transforms](convolution-theorem.md)
- [Multidimensional transform over several axes](multidim-transform.md)
- [Synthetic polynomial transform](synthetic-transform.md)
- [Gaussian resampling](gaussian-resampling.md)
- [Bluestein chirp identity](bluestein-chirp.md)
- [Synthetic transform layout](transform-layout.md)
- [Convolution from the retained transform layout](synthetic-convolution.md)
- [Resampling with the permutations left in the transform](sort-free-resampling.md)
- [Cancellation of the retained source permutation](source-permutation-cancellation.md)
- [Tensor resampling on a padded box](tensor-resampling.md)
- [Multiplication as radix-digit convolution](radix-digit-reduction.md)
- [Parameter choices for the padded box](parameter-choice.md)
- [Chinese-remainder map to prime cyclic axes](crt-prime-axes.md)
- [Prime source lengths near powers of two](prime-source-lengths.md)
- [Source transform procedure](source-transform.md)
- [The multiplication algorithm](multiplication-algorithm.md)
- [Precision and exact recovery](exact-recovery.md)
- [Total time bound and the power saving](time-accounting.md)
- [Integer multiplication in O(n (lg n)^(1−κ)) time](main-theorem.md)

## Page to audit (`time-accounting`)

````markdown
# Total time bound and the power saving

The total time bound and the power saving turn the stage costs into the running-time claim for the target. Here $n$ is the bit length of each input, $T$ the transform size and $p$ the working precision in bits. Summing them bounds the running time of [The multiplication algorithm](multiplication-algorithm.md) on $n$-bit inputs.

Each stage is measured after division by the volume $V=Tp$, the number of stored bits up to a constant. Each stage then costs a power of $p$. Each power of $p$ below is set by the sizes in [Parameter choices for the padded box](parameter-choice.md). Here $d$ is the number of axes, $K$ the chunk width in bits and $\ell=\log_2 r$ the number of address bits in a record. The fixed exponents are $\epsilon$ and $c$ in these sizes, $\tau$ and $\lambda'$ in the chunk and butterfly costs, and $\delta$ in the Gaussian costs. The Gaussian maps use an integer parameter $\alpha=\Theta(p^{1/4+\epsilon/2})$.

Each row is the cost proved earlier for its stage, divided by $V$:

- prefix moves and butterfly rounds: $dK=p^{\epsilon(1+c)}$;
- chunk exchanges: $pK^{\tau-1}=p^{1-\epsilon c(1-\tau)}$;
- simultaneous butterfly rounds: $\ell d^{\lambda'}=p^{1-\epsilon(1-\lambda')}$;
- axis layouts: $d^2(1+\ell^\tau)=p^{\tau+\epsilon(2-\tau)}$;
- Gaussian maps: $dp^{1/2+\delta}\alpha=p^{3/4+\delta+3\epsilon/2}$;
- chirps and scalar products: $dp^\delta=p^{\epsilon+\delta}$;
- packed products: $\log(rp)=O(\ell+\log p)=O(p^{1-\epsilon})$.

The chunk row rests on the cost of [Fast address-chunk interchange](address-chunk-interchange.md), with $O(VK^\tau)$ per exchange. The simultaneous row is based on the cost of [Simultaneous normalized butterfly layer](simultaneous-butterfly-layer.md), which costs $O(Vd^{\lambda'})$ per round. The Gaussian row follows from the line counts of [Tensor resampling on a padded box](tensor-resampling.md). Together the rows bound the cost of [Convolution from the retained transform layout](synthetic-convolution.md).

The seven largest exponents have margins $g_1,\dots,g_7$ below one, so row $i$ costs $p^{1-g_i}$:
$$
g_1=1-\epsilon(1+c),\quad g_2=\epsilon c(1-\tau),\quad g_3=\epsilon(1-\lambda'),\quad g_4=1-\tau-\epsilon(2-\tau),\quad g_5=\tfrac14-\delta-\tfrac32\epsilon,\quad g_6=1-\delta-\epsilon,\quad g_7=\epsilon.
$$
With the fixed choices $\epsilon=2^{-75}$, $c=2^{-56}$, $\tau=1-2^{-50}$, $\lambda'=1-2^{-54}$ and $\delta=1/16$, we get $g_2=2^{-181}$, $g_3=2^{-129}$ and $g_7=2^{-75}$, while the others exceed $2^{-51}$. The exponent saving $\kappa=2^{-182}$ is half the smallest margin. Thus (cited).

Each row costs at most $p^{1-2\kappa}$ exactly when its margin is at least $2\kappa$, so $\kappa$ is the largest saving all seven allow. The remaining $\kappa$ is spare for logarithms, giving total exponent $(1-2\kappa)+\kappa=1-\kappa$.

The other rows have exponents at most $\delta<1-2\kappa$. Any fixed power of $\log p$ is at most $p^\kappa$ for large $p$, so the cost is $O(Tp\,p^{1-\kappa})$. Since $Tp=\Theta(n)$ and $p=\Theta(\lg n)$, this is $O(n(\lg n)^{1-\kappa})$.
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

Write the answer as JSON to `<audits>/time-A-1.audit.json` (an absolute path). Do not run any other command.

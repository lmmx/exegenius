# exegenius task 0004-argument

Project directory: `<project>` (paths below are relative to it)

Reconstruct the plan of the argument the sources make for the target result, as a tree of steps. Writers of the wiki's pages are given their page's step, and readers are shown where each page fits, so the plan says what each page is for.

- The root is the intention to show the target result. Each `show` step names a `method` that splits its claim into sub-steps, and a `motivation`: why that method suits the claim. Its `children` are the sub-steps, in the order the method takes them. An `infer` step is a single step the sources report: give its `quote`, copied exactly, and no children.
- Read the plan top down, as a reader of the sources would: "we show that", "it suffices to", "the key step is" and section structure mark the steps. Go down to the level the pages need: every page below is either the module that presents one step (`page`) or a concept some step relies on (`from`). Deeper steps of a page's own proof can stop at an `infer` step quoting where the sources carry them out.
- The sources report the plan in order: the infer steps' quotes must appear in the sources in the order of a depth-first walk of the tree (submit checks it). Mark an infer step `deferred` if the sources report it out of order.
- `motivation.source`: `explicit` if the sources state why the method is used (quote them), `recoverable` if a reader could work it out from what they say, `reconstructed` if you supply it. Do not mark a motivation explicit unless you can quote it. Writers are told to give readers every reconstructed motivation, since the sources will not.
- `from`: the concepts a step relies on, as ids: known concepts, or pages that come before the step's page in the reading order below. A step's page is its own `page` or the nearest ancestor's.
- `salient`: for a step whose claim has a feature a reader would find striking (a symmetry, a uniformity across cases, a surprising value or bound), that feature. A proof explains a result only relative to such a feature (Lange), so its page is asked to explain why the feature holds. Most steps have none.
- `adds`: symbols a step introduces for its sub-steps, each occurring in the mathematics ($...$) of the claims below it, with a display name, meaning and relation as for concept names.
- The target page presents the root. Give each other page at most one step; a page that presents no step must be in some step's `from`.

## Target

`power-saving-multiplication`: Power-saving integer multiplication: The main theorem: one deterministic fixed-tape Turing machine multiplies n-bit integers exactly in O(n (lg n)^{1−κ}) time with κ = 2^{-182}, disproving the n log n optimality conjecture in this model, by combining the fast address interchange and simultaneous butterfly layers (each saving a power of the chunk width) with Gaussian resampling, chirps and synthetic polynomial transforms.

## Reader

Undergraduate computer science: big-O notation, polynomials, complex numbers, basic Turing machines and tapes

For these sources: The reader has an undergraduate computer science degree: asymptotic analysis, discrete mathematics (sets, functions, combinatorial counting, induction, basic modular arithmetic and number theory such as primes, gcd and the Euclidean algorithm), recursion and elementary algorithms (sorting, stacks, divide and conquer), basic Turing machines with several tapes, and the first-year mathematics that goes with it: single-variable calculus (limits, convergence of sequences and series, basic integrals, geometric series), matrices and vectors, polynomials and complex numbers. The reader has not met the discrete Fourier transform or the FFT, abstract algebra beyond modular arithmetic (rings, quotient rings, algebra isomorphisms, cyclic-group structure), Gaussian integers, number-theoretic results on the density of primes, or any integer-multiplication algorithm faster than the schoolbook method.

## Pages, in reading order

- `polynomial-ring-negacyclic`: Polynomial ring modulo y^r + 1: The ring of complex polynomials of degree less than r with y^r = −1, where multiplying by a power of y is a signed cyclic shift of the coefficient list and so acts as a cheap root of unity.
- `linear-algebra-basics`: Linear maps, rank, projections and orthogonal complements: Invertible linear operators and matrices, rank, subspaces with a bilinear form, projection onto a nondegenerate subspace along its orthogonal complement, and the same ideas over the two-element field.
- `subset-gadget`: Three-element subset gadget: Indexing data by three-element subsets of {1..100}, gather and scatter through central wires create coefficients depending on intersection size and side wires on neighboring triples cancel the unwanted ones, leaving exactly y_S ← y_S + x_S.
- `fixed-tape-model`: Fixed finite-tape Turing machine model: The complexity model of the paper: one machine with a fixed finite alphabet and a fixed finite number of one-dimensional tapes, where time counts single-cell steps and fixed extra tracks only change time by a constant factor.
- `nlogn-conjecture`: Schönhage–Strassen optimality conjecture: Schönhage and Strassen's 1971 O(n log n log log n) multitape multiplication, with their proposal that n log n is the optimal growth order; Harvey and van der Hoeven later reached O(n log n), and the paper refutes optimality of n log n in this model.
- `hvdh-multiplier`: Established O(n log n) multiplier (Harvey–van der Hoeven): The known fixed-tape integer multiplication algorithm running in O(N log N) on N-bit inputs, used by the paper only as an ordinary subroutine on shorter records.
- `stream-scans`: Arrays as tape streams and elementary scans: Arrays stored in lexicographic order on tape with implicit addresses; logical volume V is the number of stored bits, and digit moves, digit-pair permutations, splitting on a digit and cyclic row splitting into W streams all take O(V) steps with counters included.
- `controlled-shifts`: Controlled cyclic shifts and rational scalings: In O(V) steps a target address field can be cyclically shifted by an offset computed from earlier control fields, or multiplied by a fixed unit modulo q^b, by splitting each fiber into pieces and merging them in another order.
- `restored-scratch`: Restoring arbitrary scratch values: A linear schedule of eight rows (subtract old side and center contributions, copy, gather, scatter, side-inject, undo) that adds the wanted update to a target and returns every scratch wire to its arbitrary initial value.
- `bank-exchange-network`: Three-stage bank exchange network: Applying the forward invocation at stage 1, its inverse at stage 2 and the forward invocation at stage 3 sends (X,Y) to (−Y,X) with every scratch wire restored; the bit version is a pure exchange using XOR gates, the complex version uses Gaussian dyadic values.
- `role-streams`: Role streams and fixed-tape depth-first scheduling: Rows are distributed cyclically among the W wire roles so each recursive child works on one stream of volume V/W with the same local address set, and the recursion runs depth-first on a fixed number of tapes using a parking stack for inactive streams.
- `frames`: Frames: changing representation along the network: An invertible linear operator D_v attached to each source, gate and sink: the stored array is D_v applied to the logical array, an edge applies D_vD_u^{-1}, and since the gate is pointwise linear the common frame commutes with it, so intermediate frames cancel and only endpoint frames matter.
- `subspace-labels`: Nested subspace labels and orthogonal residuals: Each gate and terminal gets a nondegenerate subspace of a fixed bilinear space, adjacent labels are nested, and an edge's orthogonal residual (the complement of the smaller label in the larger) has dimension equal to the change in label dimension; the gadget's triple lines are chosen so the total dimension lost stays small.
- `rank-saving`: Strict total rank deficit s < W·m: The sum s of edge ranks (bit network) or residual dimensions (complex network) over a network with W wires and m-dimensional labels is strictly less than W·m, so the number of smaller recursive calls per wire is below m.
- `recursive-power-saving`: Recursive power saving from s/W < m: A call on parameter m·f makes s calls on parameter f over volume V/W each, so normalized cost satisfies F_k ≤ (s/W)F_{k-1} + O(1) with s/W < m, which sums to O(m^{kτ}) for some τ < 1.
- `lower-triangular-factorization`: Lower-triangular (Bruhat) factorization of a matrix: Every rational m×m matrix A of rank a factors as E₁ΠE₂ with E₁, E₂ invertible lower triangular and Π a partial permutation with exactly a ones, so a shear by A costs exactly a interchanges plus linear-cost triangular updates in tape order.
- `bit-network-shear`: Bit network as an address shear: With frames given by shears Φ_M that map address (H,D) to (H+MD,D), the bit network's endpoint operator on every role is the shear H ← H+D, and each edge costs as many smaller interchanges as its rational rank.
- `fast-chunk-interchange`: Fast address-chunk interchange: The first tape procedure: two disjoint u-bit address chunks of a bit array are swapped in O(V·u^τ) time for a fixed τ < 1, by the shear D←D−H, H←H+D, D←H−D where the middle shear is done by the bit network recursively, with padding to supply rows and handling arbitrary widths.
- `packed-selected-bit-addition`: Packed selected-bit addition with guard repair: A binary row addition on the same selected bit in many K-bit chunks at once, done by a constant number of whole-chunk cyclic shifts (eight rotations) plus chunk swaps, then a deterministic sort-based repair of the exponentially small set of addresses whose guard values are too close to the segment ends.
- `fixed-point-precision`: Fixed-point grids, truncation and contraction error bounds: Complex numbers stored with p fractional bits in the unit disk, truncation toward zero to the grid costing less than √2·2^{-p} per step, and linear contractions propagating errors without growth so errors of composed steps add.
- `signed-packing`: Signed packing (Kronecker substitution) for ring products: Real and imaginary parts of two polynomial records are evaluated at B = 2^{4p} as signed integers, multiplied with the established multiplier, and the coefficients recovered by centered residues and reduced modulo y^r + 1, in time O(rp log(rp)).
- `dft-convolution`: Cyclic convolution and the discrete Fourier transform: The normalized DFT F_q turns cyclic convolution into pointwise multiplication (F(u*v) = q·Fu·Fv), and the opposite-sign transform inverts it up to scaling.
- `radix-digit-convolution`: Multiplication as convolution of radix digits: Writing each n-bit input as a polynomial in 2^b with b-bit digits, the integer product is the convolution of the digit lists followed by carry propagation.
- `multidim-transform`: Multidimensional (tensor) transforms: A transform over several axes is the tensor product of one-dimensional transforms, applied line by line along each axis; convolution on a product of cyclic groups is computed by it.
- `two-point-butterfly`: Two-point butterfly and decimation-in-frequency layer: The map (u,v) → ((u+v)/2, (u−v)/2) applied on pairs of entries; the radix-2 FFT recursion pairs z_k with z_{k+t/2}, applies this butterfly, multiplies by a twiddle, and recurses on the two halves.
- `synthetic-transform`: Synthetic polynomial transforms (Nussbaumer): Normalized DFTs over the polynomial ring whose roots of unity ω_t = y^{2r/t} are signed coefficient shifts, so the radix-2 butterfly levels need no general multiplications; they are contractions and invertible up to scaling.
- `phase-network`: Phase-frame (complex) network for the butterfly: The complex bank-exchange network with binary labels and frames C_U = H^{⊗m} diag(i^{wt(P_U x) mod 4}) H^{⊗m}, in which an edge's frame change is a product of two-point kernels aI + bX_v, one per residual basis vector, and the endpoint operator is the tensor power C^{⊗m} of the kernel C = H·diag(1,i)·H.
- `exact-dyadic-guard`: Exact Gaussian dyadic arithmetic and guard widths: The whole accelerated layer is computed exactly on Gaussian dyadics with explicit extra integer and fractional guard bits (growing like d^20 = o(p)), so only one truncation to the p-bit grid is applied after the completed contractive layer.
- `crt-axis-map`: Chinese-remainder axis map to prime cyclic axes: An algebra isomorphism sending x to a product of x_i^{μ_i} turns one cyclic convolution of length S = s_1⋯s_d (pairwise coprime) into a convolution on d prime-length axes; it is realized by padded layouts and triangular controlled shifts.
- `gaussian-resampling`: Gaussian resampling: The Harvey–van der Hoeven technique that expresses a Fourier transform of prime length s through a transform of the larger power-of-two length t via Gaussian-weighted maps, up to a small controlled error.
- `bluestein-chirp`: Bluestein chirp identity: A DFT-like sum is rewritten using the product of two chirp phases so that it becomes a pointwise phase multiplication, a cyclic convolution with a fixed chirp array, and another phase multiplication.
- `phase-twist`: Phase twist from cyclic to negacyclic convolution: Multiplying coefficient k by e^{πik/r} maps cyclic convolution of length r isomorphically onto convolution modulo y^r + 1, and the inverse twist recovers the cyclic result with the correct wrap sign.
- `permutations-left-in-transform`: Resampling with the permutations left in the transform: Instead of sorting the data twice per coordinate, the source and target coordinate permutations are kept in the transform formula, the middle transform becomes a chirp convolution, and the source permutation cancels across two forward transforms, their pointwise product and the opposite-sign transform.
- `phase-edge-recursion`: Recursive implementation of the phase network on selected bits: A group of e = m·f selected axes is split into m slots of f chunks; basis changes by invertible binary matrices (products of row additions) and one child call of C^{⊗f} per residual basis vector implement each edge, giving s child calls each on volume V/W, and H₀^{⊗D} = b^D S_D C^{⊗D} S_D converts the result to a normalized layer.
- `simultaneous-butterfly-layer`: Simultaneous butterfly layer in O(V·d^λ′): The second tape procedure: applying H₀ at the same bit offset in up to d equal-width address chunks of polynomial records at once, with one truncation, in time O(V·d^λ′) for a fixed λ′ < 1 rather than d separate passes.
- `transform-layout-order`: Transform layout and retained frequency order: The d−1 transform axes are rearranged by single-bit moves and width-K chunk exchanges so that each round's selected bits lie at a common offset in consecutive chunks; the forward transform outputs frequencies in a fixed permuted order that the pointwise product and the opposite transform consume without unpermuting.
- `ring-convolution`: Normalized convolution over the polynomial ring: Two forward synthetic transforms in retained order, M pointwise polynomial products by signed packing, and the opposite transform compute convolution divided by T over the remaining axes with polynomial-ring values, with error O(M·ℓ·2^{-p}) and cost O(Tp[dK + pK^{τ−1} + ℓd^{λ′} + log(rp)]).
- `padded-box-lines`: Padded power-of-two box and linewise tensor maps: Source arrays live in the full padded box of T entries; a one-dimensional map is applied to each line of an exposed axis in turn, with axis exposure and restoration by adjacent chunk interchanges costing O(Tp d²(1+ℓ^τ)).
- `prime-lengths`: Prime source lengths near the powers of two: Distinct odd primes s_i in ((1−2η)t_i, (1−η)t_i] are found by deterministic scanning so that their product exceeds T/2 and each ratio t_i/s_i leaves the separation Gaussian resampling needs.
- `complex-convolution`: Complex cyclic convolution by a coefficient twist: Cyclic convolution over all d scalar axes is computed by twisting the last axis into the polynomial ring, convolving the other d−1 axes with ring convolution, and untwisting, with disk-margin and error accounting.
- `source-transform`: Source transform: compress, chirp convolution, expand: The procedure that computes the permuted prime-length transform of a tensor of digits: linewise expansion by the Gaussian map, the chirp transform via complex convolution, linewise compression, and an exact scaling by 2^γ.
- `parameter-choices`: Parameter choices and the cost table giving κ: Fixed rational exponents (τ = 1 − 2^-50, ε = 2^-75, c = 2^-56, λ′ = 1 − 2^-54, δ = 1/16, …) and sizes p = 6⌈log₂n⌉, d ≈ p^ε, T ≈ n/b, r = 2^ℓ make every normalized cost term at most p^{1−2κ}, giving total time O(n (lg n)^{1−κ}) with κ = 2^{-182}.
- `exact-recovery`: Exact recovery by rounding and carry propagation: The computed convolution has error below 1/2 per coefficient, so rounding gives the exact integer digit-convolution coefficients, which are bounded by 2^{3b} and turned into the binary product by radix carry propagation.
- `multiplication-algorithm`: The assembled multiplication algorithm: Split inputs into b-bit digits, place them by the Chinese-remainder map, compute two source transforms, multiply pointwise, apply the opposite-sign source transform with the S-scalings that undo the normalizations, round, unmap, and propagate carries to output the 2n-bit product.
- `power-saving-multiplication`: Power-saving integer multiplication: The main theorem: one deterministic fixed-tape Turing machine multiplies n-bit integers exactly in O(n (lg n)^{1−κ}) time with κ = 2^{-182}, disproving the n log n optimality conjecture in this model, by combining the fast address interchange and simultaneous butterfly layers (each saving a power of the chunk width) with Gaussian resampling, chirps and synthetic polynomial transforms.

## Concepts the reader knows

- `big-o-notation`: Big-O notation
- `polynomials`: Polynomials
- `complex-numbers`: Complex numbers
- `turing-machine-tapes`: Turing machines and tapes
- `logarithms-and-exponentials`: Logarithms and exponentials
- `floor-ceiling-and-rounding`: Floor, ceiling and rounding
- `binary-representation-of-integers`: Binary representation of integers
- `schoolbook-arithmetic-and-carries`: Schoolbook arithmetic and carries
- `bitwise-operations-and-xor`: Bitwise operations and XOR
- `modular-arithmetic-and-residues`: Modular arithmetic and residues
- `primes-divisibility-and-coprimality`: Primes, divisibility and coprimality
- `roots-of-unity`: Roots of unity
- `complex-modulus-conjugate-and-triangle-inequality`: Complex modulus, conjugate and triangle inequality
- `max-norm-and-error-bounds`: Max norm and error bounds
- `little-o-big-omega-and-theta-notation`: Little-o, big-Omega and Theta notation
- `series-and-exponential-decay-estimates`: Series and exponential decay estimates
- `recurrences-and-recursion`: Recurrences and recursion
- `sorting-and-stacks`: Sorting and stacks
- `sets-subsets-and-counting`: Sets, subsets and counting
- `permutations-and-bijections`: Permutations and bijections
- `lexicographic-order-and-mixed-radix-indices`: Lexicographic order and mixed-radix indices
- `matrices-matrix-vector-products-and-inverses`: Matrices, matrix-vector products and inverses
- `kronecker-product-of-matrices`: Kronecker product of matrices
- `hadamard-matrix-and-the-2x2-butterfly-matrix`: Hadamard matrix and the 2x2 butterfly matrix
- `inner-products-orthogonality-and-eigenvalues`: Inner products, orthogonality and eigenvalues
- `polynomial-evaluation-and-interpolation`: Polynomial evaluation and interpolation
- `fixed-point-and-floating-point-numbers`: Fixed-point and floating-point numbers
- `karatsuba-and-the-history-of-fast-multiplication`: Karatsuba and the history of fast multiplication
- `square-roots-and-quotient-with-remainder`: Square roots and quotient with remainder

## Sources (read them from disk)

- `sources/paper.tex`: source id `paper`, "Integer multiplication below n log n (OpenAI, 2026)"

## Answer

The answer is one JSON object following this schema:

```json
{
  "$defs": {
    "Binding": {
      "additionalProperties": false,
      "description": "What a symbol in a concept's name denotes.",
      "properties": {
        "meaning": {
          "description": "One sentence saying what the symbol denotes; for a count, what it counts.",
          "type": "string"
        },
        "name": {
          "description": "Display name of one to four words, e.g. \"record length\".",
          "type": "string"
        },
        "relation": {
          "$ref": "#/$defs/Relation"
        },
        "symbol": {
          "description": "The symbol in LaTeX, as it appears in the title's mathematics, without dollar signs,\ne.g. `r` or `\\kappa`.",
          "type": "string"
        }
      },
      "required": [
        "symbol",
        "name",
        "meaning",
        "relation"
      ],
      "type": "object"
    },
    "Evidence": {
      "additionalProperties": false,
      "properties": {
        "quote": {
          "description": "Text copied character for character from that source.",
          "type": "string"
        },
        "source": {
          "description": "Id of the source the quote comes from.",
          "type": "string"
        }
      },
      "required": [
        "source",
        "quote"
      ],
      "type": "object"
    },
    "Intention": {
      "oneOf": [
        {
          "const": "show",
          "description": "Show a claim by a method that splits it into sub-steps.",
          "type": "string"
        },
        {
          "const": "infer",
          "description": "Infer one claim in a single step the sources report.",
          "type": "string"
        }
      ]
    },
    "Motivation": {
      "additionalProperties": false,
      "description": "Why a method suits the claim it shows.",
      "properties": {
        "quote": {
          "anyOf": [
            {
              "$ref": "#/$defs/Evidence"
            },
            {
              "type": "null"
            }
          ],
          "description": "For an explicit motivation, the sources' words, copied exactly."
        },
        "source": {
          "$ref": "#/$defs/Provenance",
          "description": "Whether the sources state the reason, leave it to be worked out, or neither."
        },
        "text": {
          "description": "The reason, in one or two sentences.",
          "type": "string"
        }
      },
      "required": [
        "text",
        "source"
      ],
      "type": "object"
    },
    "Node": {
      "additionalProperties": false,
      "description": "One step of the argument.",
      "properties": {
        "adds": {
          "description": "Symbols the step introduces, in force in its sub-steps.",
          "items": {
            "$ref": "#/$defs/Binding"
          },
          "type": "array"
        },
        "children": {
          "description": "For a show step: its sub-steps, in the order the method takes them.",
          "items": {
            "$ref": "#/$defs/Node"
          },
          "type": "array"
        },
        "claim": {
          "description": "What the step shows or infers, in one sentence, with mathematics in LaTeX ($...$).",
          "type": "string"
        },
        "deferred": {
          "description": "For an infer step the sources report out of order, before or after the steps around\nit.",
          "type": "boolean"
        },
        "from": {
          "default": [],
          "description": "Ids of the concepts the step relies on: known concepts, or pages before the step's\npage.",
          "items": {
            "type": "string"
          },
          "type": "array"
        },
        "id": {
          "description": "A short label, unique in the tree, such as \"S0\" or \"S2.1\".",
          "type": "string"
        },
        "intention": {
          "$ref": "#/$defs/Intention"
        },
        "method": {
          "description": "For a show step: how the claim is shown, in a phrase (\"reduce to a cyclic\nconvolution and bound its cost\").",
          "type": [
            "string",
            "null"
          ]
        },
        "motivation": {
          "anyOf": [
            {
              "$ref": "#/$defs/Motivation"
            },
            {
              "type": "null"
            }
          ],
          "description": "For a show step: why that method suits the claim."
        },
        "page": {
          "description": "Id of the page that presents this step and its sub-steps, if one does.",
          "type": [
            "string",
            "null"
          ]
        },
        "quote": {
          "anyOf": [
            {
              "$ref": "#/$defs/Evidence"
            },
            {
              "type": "null"
            }
          ],
          "description": "For an infer step: the step as the sources report it, copied exactly."
        },
        "salient": {
          "description": "For a step whose claim has a feature a reader would find striking (a symmetry, a\nuniformity across cases, a surprising value or bound): that feature. The page should\nexplain why it holds, from a matching feature of the method.",
          "type": [
            "string",
            "null"
          ]
        }
      },
      "required": [
        "id",
        "intention",
        "claim"
      ],
      "type": "object"
    },
    "Provenance": {
      "oneOf": [
        {
          "const": "explicit",
          "description": "The sources state the reason; quote them.",
          "type": "string"
        },
        {
          "const": "recoverable",
          "description": "The sources do not state it, but a reader can work it out from what they say.",
          "type": "string"
        },
        {
          "const": "reconstructed",
          "description": "The sources neither state it nor make it recoverable: you supply it.",
          "type": "string"
        }
      ]
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
  "properties": {
    "root": {
      "$ref": "#/$defs/Node",
      "description": "The main step: to show the target result."
    }
  },
  "required": [
    "root"
  ],
  "title": "ArgumentAnswer",
  "type": "object"
}
```

Write the answer to `exegenius/tasks/0004-argument.answer.json`, then run `exegenius submit 0004-argument` in the project directory. If submit rejects the answer, fix the answer and submit again.

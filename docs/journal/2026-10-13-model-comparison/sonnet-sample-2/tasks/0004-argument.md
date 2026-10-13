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

`main-theorem`: Integer multiplication in O(n (lg n)^(1−κ)) time: A deterministic fixed-tape Turing machine multiplies two n-bit integers exactly in O(n (lg n)^{1−κ}) worst-case time with κ = 2^{-182}, disproving the n log n optimality conjecture in this model; it works by two fast tape procedures (address interchange and simultaneous butterfly layers) inside a Gaussian-resampling, chirp and synthetic-transform convolution.

## Reader

Undergraduate computer science: big-O notation, polynomials, complex numbers, basic Turing machines and tapes

For these sources: The reader is an undergraduate computer scientist: discrete mathematics (induction, modular arithmetic, gcd, permutations, relations, graphs), algorithms (divide and conquer, recurrences, schoolbook and Karatsuba multiplication, recursion), single-variable calculus (limits, convergence of sequences and series, basic integrals, exponentials), and introductory linear algebra (matrices, linear maps, invertibility, Gaussian elimination). They know complex numbers with roots of unity, polynomials, big-O and its relatives, and elementary Turing machines with tapes. They have not studied abstract algebra beyond residue rings, Fourier analysis, the fast Fourier transform or the discrete Fourier transform, numerical error analysis, analytic number theory, or multitape-machine algorithm design.

## Pages, in reading order

- `convolution`: Cyclic convolution: The product of two coefficient lists (u∗v)ₖ = Σ uⱼ v_{k−j} with indices wrapping around modulo the length; equivalent to polynomial multiplication modulo xᴺ−1.
- `ring-y-r-plus-1`: Polynomial ring modulo y^r + 1: Polynomials with complex coefficients of degree below r with multiplication reduced by y^r = −1 (negacyclic wrap), so multiplying by a power of y is a signed shift of the coefficient list.
- `gaussian-dyadics`: Gaussian dyadic numbers: Complex numbers whose real and imaginary parts are integers divided by a power of two, so addition, subtraction, halving and multiplication by i are exact in binary.
- `phase-twist`: Coefficient phase twist: Multiplying coefficient k by ζ^k with ζ = e^{πi/r} maps a cyclic convolution in x^r−1 to a convolution modulo y^r+1, whose negative wrap exactly reproduces the cyclic wrap after untwisting.
- `multitape-model`: Fixed multitape Turing machine model: One fixed finite alphabet and a fixed finite number of one-dimensional tapes; each step reads/writes one cell per tape and moves each head by at most one cell, so moving data costs time proportional to the distance travelled.
- `nlogn-conjecture-history`: Schönhage–Strassen n log n conjecture and prior bounds: Schönhage–Strassen multiplication takes O(n log n log log n) on multitape machines, they conjectured n log n optimal, and later work reached O(n log n).
- `hvdh-multiplier`: Harvey–van der Hoeven O(n log n) multiplier: The established unconditional O(n log n) multitape integer multiplication algorithm, used here only as an ordinary subroutine on short records.
- `kronecker-packing`: Signed packing for exact polynomial products: Polynomial products over the ring are computed by packing coefficients into signed integers (a form of Kronecker substitution) and calling the established O(N log N) integer multiplier.
- `stream-ops`: Elementary stream operations: Linear-time tape scans: swapping two digit values, moving a digit, splitting an array into aligned streams and merging them back, and splitting whole rows cyclically among W streams.
- `controlled-shifts`: Controlled cyclic shifts: Updating a target field y ↦ y + a(controls) mod Q (and multiplying by units) in O(V) tape time when the target field follows its control fields.
- `linear-algebra`: Subspaces, rank and orthogonal complements: Vector subspaces of a bilinear space, their dimension, matrix rank, nondegeneracy and orthogonal complements, over the rationals or over the field with two elements.
- `scratch-network`: Scratch-restoring linear network: A fixed network of linear gates on wires that exchanges two banks of values with signs while returning every auxiliary (scratch) wire to its arbitrary initial value.
- `frames`: Frames and the common-frame identity: Replacing each wire's scalar by an array and giving each vertex an invertible operator D_v, so gates commute with the frame and the network implements D_out S D_in^{-1}; edges change frames by D_v D_u^{-1}.
- `subspace-labels`: Nested subspace labels from three-element subsets: Subspaces attached to gates and terminals, built from indicator vectors of three-element subsets and tensor products, nested along edges so that the dimension change on an edge counts the smaller operations needed.
- `rank-saving`: Rank saving s < Wm: The total rank s of all edge changes in the network is strictly smaller than the number of wires W times the label dimension m, which makes the recursive saving a power of the chunk width.
- `bit-network-interface`: Bit network and rational matrix interface: The F₂ network of XOR gates with rational matrix labels M_v whose endpoint differences are the identity, sending each input to its exchanged role, with total edge rank s_b < W_b m.
- `phase-network`: Complex phase network: The same finite network with binary labels and Gaussian dyadic scalars, whose phase frames C_U change by translation kernels aI+bX_v, so that telescoping gives the two-point kernel C = HSH on every role with total call count s_c < W_c m.
- `bruhat-factorization`: Lower triangular factorization of a matrix: Every rational matrix A of rank a factors as E₁ΠE₂ with E₁,E₂ invertible lower triangular and Π a partial permutation matrix with exactly a ones.
- `matrix-shear`: Cost of a rational matrix shear: The address permutation (H,D) ↦ (H+AD,D) over ℤ/q^bℤ with A of rank a is done with exactly a interchanges of b-digit fields plus linear work, using the triangular factorization and controlled shifts.
- `power-saving-recurrence`: Volume-normalized power-saving recurrence: The recurrence F_k ≤ (s/W)·F_{k−1} + O(1) for cost per bit, which solves to O(e^τ) at width e = m^k because s/W < m^τ for some τ < 1.
- `fixed-tape-recursion`: Fixed-tape depth-first recursion schedule: Running a recursion on a fixed number of tapes by parking inactive role streams on a stack tape and storing descriptors on a stack, so each call pays only linear cost in its own volume.
- `power-width-interchange`: Power-width address interchange: Interchanging two chunks of e = m^k digits in O(V e^τ) time by splitting rows among the W roles, applying shears on each edge as smaller interchanges, and recursing.
- `address-chunk-interchange`: Fast address-chunk interchange: Swapping two disjoint equal-width address chunks of u bits in an array of V bits in O(V u^τ) time for a fixed τ < 1, via XOR-combined intermediate data though the final effect is an exact permutation.
- `packed-selected-bit-addition`: Packed selected-bit addition: XORing corresponding selected bits of two address chunks (a binary row addition) using cyclic shifts of whole chunk ranges and a deterministic repair of a small explicitly known set of exceptional addresses.
- `disk-grid-precision`: Disk-grid arrays, contractions and truncation: Fixed-point complex values in the unit disk on a grid 2^{−p}ℤ[i], linear maps that never increase the maximum norm (contractions), and truncation toward zero Q_p whose error is below √2·2^{−p}.
- `guard-width-exactness`: Guard bits and a single final truncation: Extra integer and fractional guard bits keep every intermediate Gaussian dyadic value of a layer exact, with only one truncation after the whole contractive layer.
- `dft`: Normalized discrete Fourier transform: The map (F_q u)ₖ = (1/q) Σⱼ uⱼ e^{−2πijk/q} that evaluates a length-q sequence at the q-th roots of unity, with a 1/q normalization making it a contraction.
- `synthetic-roots`: Synthetic roots of unity: In the ring modulo y^r+1 the power ω_t = y^{2r/t} acts as a t-th root of unity whose multiplication is only a signed cyclic shift of coefficients.
- `fft-butterfly`: Butterfly layers (fast Fourier transform): A length-t transform is computed in log₂ t rounds, each pairing entries z_k and z_{k+t/2} with the two-point butterfly H₀(u,v) = ((u+v)/2,(u−v)/2) and a twiddle multiplication.
- `simultaneous-butterfly-layer`: Simultaneous normalized butterfly layer: Applying H₀ on the same bit position of up to d address chunks at once in O(V d^λ′) time, λ′ < 1, by the recursive phase network instead of d separate passes.
- `convolution-theorem`: Convolution via transforms: A Fourier transform turns convolution into pointwise multiplication, so u∗v is obtained by transforming both inputs, multiplying coordinatewise and applying the opposite transform (with normalization factors).
- `multidim-transform`: Multidimensional transform over several axes: A transform on an array indexed by several axes, equal to the tensor product of one-dimensional transforms applied along each axis in turn.
- `synthetic-transform`: Synthetic polynomial transform: A Nussbaumer–Quandalle style transform over the polynomial ring evaluated by butterfly rounds and monomial twiddles, writing frequencies in bit-reversed stored order that the opposite transform consumes.
- `gaussian-resampling`: Gaussian resampling: A Harvey–van der Hoeven technique that expresses a Fourier transform of a length s (coprime) by a transform whose length t is a slightly larger power of two, using Gaussian-weighted contractions A and B₀ between the two grids.
- `bluestein-chirp`: Bluestein chirp identity: A transform with a permuted frequency order is rewritten as pointwise phase multiplications around a cyclic convolution with a chirp array a, with entries exp(−πi·s j²/t).
- `transform-layout`: Synthetic transform layout: A positional permutation of record addresses that places the selected bits of all axes at a common offset in consecutive K-bit chunks, plus the retained bit-reversed frequency order, so that simultaneous layers apply in every round.
- `synthetic-convolution`: Convolution from the retained transform layout: Convolution over the polynomial ring: two forward synthetic transforms, pointwise signed-packed polynomial products in the retained order, then the opposite transform and exact scaling, with error below √2·M(3ℓ+1)2^{−p}.
- `sort-free-resampling`: Resampling with the permutations left in the transform: The identity P_s F_s = 2^{2α²} B₀ P_t F_t A that keeps the two coordinate permutations inside the transform formula, so they never need to be executed as separate full sorts.
- `source-permutation-cancellation`: Cancellation of the retained source permutation: The permutation R kept during resampling cancels across two forward source transforms, their pointwise product and the opposite-sign transform, giving (u∗v)/S².
- `tensor-resampling`: Tensor resampling on a padded box: Applying the one-dimensional resampling maps A and B₀ line by line along each of d axes of a zero-padded power-of-two box, moving axes into place with chunk swaps.
- `radix-digit-reduction`: Multiplication as radix-digit convolution: Writing both integers in base 2^b turns the product into a convolution of short digit lists whose coefficients are bounded below 2^{3b}; carries are propagated afterwards.
- `parameter-choice`: Parameter choices for the padded box: The sizes b = ⌈log₂n⌉, precision p = 6b, number of axes d = ⌊b^ε⌋, chunk width K = ⌊d^c⌋, transform size T ≈ n/b and axis lengths r/2 or r with r = 2^{Θ(p/d)}.
- `crt-prime-axes`: Chinese-remainder map to prime cyclic axes: An algebra isomorphism that rewrites one cyclic convolution of coprime length S = ∏ sᵢ as a multidimensional cyclic convolution on axes of the distinct prime lengths sᵢ, implemented on tape by triangular controlled shifts and axis moves.
- `prime-source-lengths`: Prime source lengths near powers of two: Distinct odd primes sᵢ with (1−2η)tᵢ < sᵢ ≤ (1−η)tᵢ, found by deterministic search, so that their product fills at least half the padded box and each tᵢ/sᵢ leaves enough room for resampling.
- `source-transform`: Source transform procedure: The computed transform RF_s of a source array: linewise A, then a chirp transform via complex convolution on power-of-two axes (twisted into the polynomial ring), then linewise B₀ and exact scaling by 2^γ.
- `multiplication-algorithm`: The multiplication algorithm: Digits are placed on prime axes by the CRT map, two source transforms are taken, multiplied pointwise, the opposite-sign transform and scalings applied, results rounded to integers, mapped back, and carries propagated to give the 2n-bit product.
- `exact-recovery`: Precision and exact recovery: All errors from truncation and approximate maps are bounded so every intermediate array stays in the disk and the recovered convolution coefficients are within 1/2 of integers, so rounding and carries give the exact product.
- `time-accounting`: Total time bound and the power saving: Dividing every cost by Tp gives seven dominant powers of p, each at most 1−2κ; the smallest margin 2^{-181} = 2κ makes the total O(n (lg n)^{1-κ}).
- `main-theorem`: Integer multiplication in O(n (lg n)^(1−κ)) time: A deterministic fixed-tape Turing machine multiplies two n-bit integers exactly in O(n (lg n)^{1−κ}) worst-case time with κ = 2^{-182}, disproving the n log n optimality conjecture in this model; it works by two fast tape procedures (address interchange and simultaneous butterfly layers) inside a Gaussian-resampling, chirp and synthetic-transform convolution.

## Concepts the reader knows

- `big-o`: Big-O notation
- `polynomials`: Polynomials
- `complex-numbers`: Complex numbers
- `tm-tapes`: Basic Turing machines and tapes
- `asymptotic-notation-beyond-big-o`: Asymptotic notation beyond big-O
- `logarithms-and-exponentials`: Logarithms and exponentials
- `floor-and-ceiling`: Floor and ceiling
- `binary-representation-of-integers`: Binary representation of integers
- `modular-arithmetic`: Modular arithmetic
- `division-with-remainder-and-the-euclidean-algorithm`: Division with remainder and the Euclidean algorithm
- `prime-numbers`: Prime numbers
- `permutations-and-bijections`: Permutations and bijections
- `matrices-and-linear-maps`: Matrices and linear maps
- `inner-product-orthogonality-and-norms`: Inner product, orthogonality and norms
- `euler-s-formula-and-complex-exponentials`: Euler's formula and complex exponentials
- `sums-geometric-series-and-sigma-notation`: Sums, geometric series and sigma notation
- `limits-convergence-and-error-bounds`: Limits, convergence and error bounds
- `mathematical-induction-and-recurrences`: Mathematical induction and recurrences
- `schoolbook-and-karatsuba-multiplication`: Schoolbook and Karatsuba multiplication
- `positional-notation-in-a-general-radix`: Positional notation in a general radix
- `cartesian-product-and-mixed-radix-addresses`: Cartesian product and mixed-radix addresses
- `bitwise-xor-and-the-field-with-two-elements`: Bitwise XOR and the field with two elements
- `rings-and-fields-algebraic-structure-basics`: Rings and fields, algebraic structure basics
- `fixed-point-and-exact-arithmetic`: Fixed-point and exact arithmetic
- `finite-directed-acyclic-networks`: Finite directed acyclic networks
- `stacks-and-recursion-on-tapes`: Stacks and recursion on tapes
- `linear-time-tape-scans-and-sorting-cost`: Linear-time tape scans and sorting cost
- `newton-s-method`: Newton's method

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

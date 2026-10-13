# exegenius task 0002-background

Project directory: `<project>` (paths below are relative to it)

Say what a specific reader already knows, for these sources. The reader profile describes the reader in a few words, and its examples do not list everything it covers: it stands for a level of study. Make it concrete for the sources.

The sources were written for readers of their own, and use without explanation what those readers know. Read the sources and find that background: the notions, notation and standard results they use without defining. For each, decide whether a reader at the profile's level has been taught it, in the profile's subject or in the subjects that subject rests on (an undergraduate degree in physics teaches linear algebra, Fourier series and probability as well as mechanics, whether or not a profile names them).

- The reader knows what their level of study teaches to everyone at that level. Do not withhold it because the profile does not name it.
- The reader does not know what is taught only in some courses, or in later or specialist study, nor what the sources introduce, define or prove: that is what the wiki explains. When unsure, count it as not known.
- The existing concepts were planned by an earlier task and stay as they are: this answer only adds background they lack.
- `summary`: the subjects and how far into each this reader has gone, as they bear on the sources. Everyone who writes or checks the wiki is shown it with the profile.
- `known`: each concept from that background that the reader knows and no existing concept covers, with the aliases a writer might use for it. Include what the sources use and what an explanation of them would naturally use. Known concepts get no page.

## Target

The main result of the paper: a deterministic multitape Turing machine algorithm that multiplies n-bit integers in O(n (lg n)^(1-kappa)) time, disproving the n log n optimality conjecture in that model, and how it works

## Reader profile

Undergraduate computer science: big-O notation, polynomials, complex numbers, basic Turing machines and tapes

## Existing concepts

- `big-o`: Big-O notation (known to the reader): Asymptotic upper-bound notation O(f(n)) for running times.
- `polynomials`: Polynomials (known to the reader): Expressions Σ aₖ xᵏ with coefficients, added and multiplied term by term.
- `complex-numbers`: Complex numbers (known to the reader): Numbers a + bi with i² = −1, with modulus and roots of unity e^{2πi/q}.
- `tm-tapes`: Basic Turing machines and tapes (known to the reader): A machine with a finite control reading and writing cells on tapes, one step at a time.
- `multitape-model`: Fixed multitape Turing machine model: One fixed finite alphabet and a fixed finite number of one-dimensional tapes; each step reads/writes one cell per tape and moves each head by at most one cell, so moving data costs time proportional to the distance travelled.
- `nlogn-conjecture-history`: Schönhage–Strassen n log n conjecture and prior bounds: Schönhage–Strassen multiplication takes O(n log n log log n) on multitape machines, they conjectured n log n optimal, and later work reached O(n log n).
- `hvdh-multiplier`: Harvey–van der Hoeven O(n log n) multiplier: The established unconditional O(n log n) multitape integer multiplication algorithm, used here only as an ordinary subroutine on short records.
- `convolution`: Cyclic convolution: The product of two coefficient lists (u∗v)ₖ = Σ uⱼ v_{k−j} with indices wrapping around modulo the length; equivalent to polynomial multiplication modulo xᴺ−1.
- `dft`: Normalized discrete Fourier transform: The map (F_q u)ₖ = (1/q) Σⱼ uⱼ e^{−2πijk/q} that evaluates a length-q sequence at the q-th roots of unity, with a 1/q normalization making it a contraction.
- `convolution-theorem`: Convolution via transforms: A Fourier transform turns convolution into pointwise multiplication, so u∗v is obtained by transforming both inputs, multiplying coordinatewise and applying the opposite transform (with normalization factors).
- `fft-butterfly`: Butterfly layers (fast Fourier transform): A length-t transform is computed in log₂ t rounds, each pairing entries z_k and z_{k+t/2} with the two-point butterfly H₀(u,v) = ((u+v)/2,(u−v)/2) and a twiddle multiplication.
- `multidim-transform`: Multidimensional transform over several axes: A transform on an array indexed by several axes, equal to the tensor product of one-dimensional transforms applied along each axis in turn.
- `radix-digit-reduction`: Multiplication as radix-digit convolution: Writing both integers in base 2^b turns the product into a convolution of short digit lists whose coefficients are bounded below 2^{3b}; carries are propagated afterwards.
- `crt-prime-axes`: Chinese-remainder map to prime cyclic axes: An algebra isomorphism that rewrites one cyclic convolution of coprime length S = ∏ sᵢ as a multidimensional cyclic convolution on axes of the distinct prime lengths sᵢ, implemented on tape by triangular controlled shifts and axis moves.
- `disk-grid-precision`: Disk-grid arrays, contractions and truncation: Fixed-point complex values in the unit disk on a grid 2^{−p}ℤ[i], linear maps that never increase the maximum norm (contractions), and truncation toward zero Q_p whose error is below √2·2^{−p}.
- `gaussian-dyadics`: Gaussian dyadic numbers: Complex numbers whose real and imaginary parts are integers divided by a power of two, so addition, subtraction, halving and multiplication by i are exact in binary.
- `gaussian-resampling`: Gaussian resampling: A Harvey–van der Hoeven technique that expresses a Fourier transform of a length s (coprime) by a transform whose length t is a slightly larger power of two, using Gaussian-weighted contractions A and B₀ between the two grids.
- `sort-free-resampling`: Resampling with the permutations left in the transform: The identity P_s F_s = 2^{2α²} B₀ P_t F_t A that keeps the two coordinate permutations inside the transform formula, so they never need to be executed as separate full sorts.
- `bluestein-chirp`: Bluestein chirp identity: A transform with a permuted frequency order is rewritten as pointwise phase multiplications around a cyclic convolution with a chirp array a, with entries exp(−πi·s j²/t).
- `source-permutation-cancellation`: Cancellation of the retained source permutation: The permutation R kept during resampling cancels across two forward source transforms, their pointwise product and the opposite-sign transform, giving (u∗v)/S².
- `ring-y-r-plus-1`: Polynomial ring modulo y^r + 1: Polynomials with complex coefficients of degree below r with multiplication reduced by y^r = −1 (negacyclic wrap), so multiplying by a power of y is a signed shift of the coefficient list.
- `synthetic-roots`: Synthetic roots of unity: In the ring modulo y^r+1 the power ω_t = y^{2r/t} acts as a t-th root of unity whose multiplication is only a signed cyclic shift of coefficients.
- `synthetic-transform`: Synthetic polynomial transform: A Nussbaumer–Quandalle style transform over the polynomial ring evaluated by butterfly rounds and monomial twiddles, writing frequencies in bit-reversed stored order that the opposite transform consumes.
- `phase-twist`: Coefficient phase twist: Multiplying coefficient k by ζ^k with ζ = e^{πi/r} maps a cyclic convolution in x^r−1 to a convolution modulo y^r+1, whose negative wrap exactly reproduces the cyclic wrap after untwisting.
- `kronecker-packing`: Signed packing for exact polynomial products: Polynomial products over the ring are computed by packing coefficients into signed integers (a form of Kronecker substitution) and calling the established O(N log N) integer multiplier.
- `stream-ops`: Elementary stream operations: Linear-time tape scans: swapping two digit values, moving a digit, splitting an array into aligned streams and merging them back, and splitting whole rows cyclically among W streams.
- `controlled-shifts`: Controlled cyclic shifts: Updating a target field y ↦ y + a(controls) mod Q (and multiplying by units) in O(V) tape time when the target field follows its control fields.
- `fixed-tape-recursion`: Fixed-tape depth-first recursion schedule: Running a recursion on a fixed number of tapes by parking inactive role streams on a stack tape and storing descriptors on a stack, so each call pays only linear cost in its own volume.
- `linear-algebra`: Subspaces, rank and orthogonal complements: Vector subspaces of a bilinear space, their dimension, matrix rank, nondegeneracy and orthogonal complements, over the rationals or over the field with two elements.
- `bruhat-factorization`: Lower triangular factorization of a matrix: Every rational matrix A of rank a factors as E₁ΠE₂ with E₁,E₂ invertible lower triangular and Π a partial permutation matrix with exactly a ones.
- `scratch-network`: Scratch-restoring linear network: A fixed network of linear gates on wires that exchanges two banks of values with signs while returning every auxiliary (scratch) wire to its arbitrary initial value.
- `frames`: Frames and the common-frame identity: Replacing each wire's scalar by an array and giving each vertex an invertible operator D_v, so gates commute with the frame and the network implements D_out S D_in^{-1}; edges change frames by D_v D_u^{-1}.
- `subspace-labels`: Nested subspace labels from three-element subsets: Subspaces attached to gates and terminals, built from indicator vectors of three-element subsets and tensor products, nested along edges so that the dimension change on an edge counts the smaller operations needed.
- `rank-saving`: Rank saving s < Wm: The total rank s of all edge changes in the network is strictly smaller than the number of wires W times the label dimension m, which makes the recursive saving a power of the chunk width.
- `power-saving-recurrence`: Volume-normalized power-saving recurrence: The recurrence F_k ≤ (s/W)·F_{k−1} + O(1) for cost per bit, which solves to O(e^τ) at width e = m^k because s/W < m^τ for some τ < 1.
- `matrix-shear`: Cost of a rational matrix shear: The address permutation (H,D) ↦ (H+AD,D) over ℤ/q^bℤ with A of rank a is done with exactly a interchanges of b-digit fields plus linear work, using the triangular factorization and controlled shifts.
- `bit-network-interface`: Bit network and rational matrix interface: The F₂ network of XOR gates with rational matrix labels M_v whose endpoint differences are the identity, sending each input to its exchanged role, with total edge rank s_b < W_b m.
- `power-width-interchange`: Power-width address interchange: Interchanging two chunks of e = m^k digits in O(V e^τ) time by splitting rows among the W roles, applying shears on each edge as smaller interchanges, and recursing.
- `address-chunk-interchange`: Fast address-chunk interchange: Swapping two disjoint equal-width address chunks of u bits in an array of V bits in O(V u^τ) time for a fixed τ < 1, via XOR-combined intermediate data though the final effect is an exact permutation.
- `phase-network`: Complex phase network: The same finite network with binary labels and Gaussian dyadic scalars, whose phase frames C_U change by translation kernels aI+bX_v, so that telescoping gives the two-point kernel C = HSH on every role with total call count s_c < W_c m.
- `packed-selected-bit-addition`: Packed selected-bit addition: XORing corresponding selected bits of two address chunks (a binary row addition) using cyclic shifts of whole chunk ranges and a deterministic repair of a small explicitly known set of exceptional addresses.
- `guard-width-exactness`: Guard bits and a single final truncation: Extra integer and fractional guard bits keep every intermediate Gaussian dyadic value of a layer exact, with only one truncation after the whole contractive layer.
- `simultaneous-butterfly-layer`: Simultaneous normalized butterfly layer: Applying H₀ on the same bit position of up to d address chunks at once in O(V d^λ′) time, λ′ < 1, by the recursive phase network instead of d separate passes.
- `transform-layout`: Synthetic transform layout: A positional permutation of record addresses that places the selected bits of all axes at a common offset in consecutive K-bit chunks, plus the retained bit-reversed frequency order, so that simultaneous layers apply in every round.
- `synthetic-convolution`: Convolution from the retained transform layout: Convolution over the polynomial ring: two forward synthetic transforms, pointwise signed-packed polynomial products in the retained order, then the opposite transform and exact scaling, with error below √2·M(3ℓ+1)2^{−p}.
- `tensor-resampling`: Tensor resampling on a padded box: Applying the one-dimensional resampling maps A and B₀ line by line along each of d axes of a zero-padded power-of-two box, moving axes into place with chunk swaps.
- `parameter-choice`: Parameter choices for the padded box: The sizes b = ⌈log₂n⌉, precision p = 6b, number of axes d = ⌊b^ε⌋, chunk width K = ⌊d^c⌋, transform size T ≈ n/b and axis lengths r/2 or r with r = 2^{Θ(p/d)}.
- `prime-source-lengths`: Prime source lengths near powers of two: Distinct odd primes sᵢ with (1−2η)tᵢ < sᵢ ≤ (1−η)tᵢ, found by deterministic search, so that their product fills at least half the padded box and each tᵢ/sᵢ leaves enough room for resampling.
- `source-transform`: Source transform procedure: The computed transform RF_s of a source array: linewise A, then a chirp transform via complex convolution on power-of-two axes (twisted into the polynomial ring), then linewise B₀ and exact scaling by 2^γ.
- `multiplication-algorithm`: The multiplication algorithm: Digits are placed on prime axes by the CRT map, two source transforms are taken, multiplied pointwise, the opposite-sign transform and scalings applied, results rounded to integers, mapped back, and carries propagated to give the 2n-bit product.
- `exact-recovery`: Precision and exact recovery: All errors from truncation and approximate maps are bounded so every intermediate array stays in the disk and the recovered convolution coefficients are within 1/2 of integers, so rounding and carries give the exact product.
- `time-accounting`: Total time bound and the power saving: Dividing every cost by Tp gives seven dominant powers of p, each at most 1−2κ; the smallest margin 2^{-181} = 2κ makes the total O(n (lg n)^{1-κ}).
- `main-theorem`: Integer multiplication in O(n (lg n)^(1−κ)) time: A deterministic fixed-tape Turing machine multiplies two n-bit integers exactly in O(n (lg n)^{1−κ}) worst-case time with κ = 2^{-182}, disproving the n log n optimality conjecture in this model; it works by two fast tape procedures (address interchange and simultaneous butterfly layers) inside a Gaussian-resampling, chirp and synthetic-transform convolution.

## Sources (read them from disk)

- `sources/paper.tex`: source id `paper`, "Integer multiplication below n log n (OpenAI, 2026)"

## Answer

The answer is one JSON object following this schema:

```json
{
  "$defs": {
    "KnownConcept": {
      "additionalProperties": false,
      "properties": {
        "aliases": {
          "description": "Other names and abbreviations for it, in the sources or in standard texts.",
          "items": {
            "type": "string"
          },
          "type": "array"
        },
        "gloss": {
          "description": "One precise sentence saying what it is.",
          "type": "string"
        },
        "name": {
          "description": "Display name in plain text, e.g. \"Fourier series\".",
          "type": "string"
        }
      },
      "required": [
        "name",
        "aliases",
        "gloss"
      ],
      "type": "object"
    }
  },
  "additionalProperties": false,
  "properties": {
    "known": {
      "description": "Concepts the reader knows that the sources rely on and no existing concept covers.",
      "items": {
        "$ref": "#/$defs/KnownConcept"
      },
      "type": "array"
    },
    "summary": {
      "description": "Two to four sentences: the subjects this reader has studied that bear on the sources,\nand how far into each.",
      "type": "string"
    }
  },
  "required": [
    "summary",
    "known"
  ],
  "title": "BackgroundAnswer",
  "type": "object"
}
```

Write the answer to `exegenius/tasks/0002-background.answer.json`, then run `exegenius submit 0002-background` in the project directory. If submit rejects the answer, fix the answer and submit again.

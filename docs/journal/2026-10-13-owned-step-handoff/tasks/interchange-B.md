# exegenius task 0027-write-address-chunk-interchange

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
- Gaussian dyadic numbers
- Polynomial ring modulo y^r + 1 (also: negacyclic ring, R_r)
  - $y$ (indeterminate): The formal variable of the polynomials in the ring.
  - $r$ (record length): The number of complex coefficients in one polynomial record, a power of two.
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
- Complex phase network (also: binary phase interface)
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

Concept: Fast address-chunk interchange
Gloss: Swapping two disjoint equal-width address chunks of u bits in an array of V bits in O(V u^τ) time for a fixed τ < 1, via XOR-combined intermediate data though the final effect is an exact permutation.
Also called: chunk swap

## This page in the argument

The step: Two disjoint contiguous address chunks of $u$ bits in an array of $V$ stored bits can be interchanged in $O(Vu^\tau)$ time for a fixed $\tau<1$, although the intermediate steps XOR-combine data.
How: Realise the shear $H\leftarrow H+D$ by a finite bit network whose edge changes are smaller shears, solve the resulting recurrence for power widths, and extend to every width.
Why: Because the sum of the edge ranks is less than the number of wires times $m$, the recursion saves a power of the chunk width. (the sources say so: "the resulting recurrence saves a power of the chunk width." (paper))
Striking feature: The cost exponent $\tau<1$ is a genuine power saving over the width, and the result is an exact permutation of the data for arbitrary initial scratch contents even though intermediate data are XOR combinations. (explain why it holds, from the method)

Its sub-steps, in order:

- Splitting an array on a digit, moving a digit, and splitting whole rows cyclically into $W$ streams (and merging them back) each take $O(V)$ time on a fixed number of tapes. (on [Elementary stream operations](stream-ops.md))
- A target field $y$ that follows its control fields can be updated $y\mapsto y+a(\text{controls}) \pmod Q$, or multiplied by a unit, in $O(V)$ time. (on [Controlled cyclic shifts](controlled-shifts.md))
- There is a fixed network of $W$ XOR gates on wires with rational $m\times m$ matrix labels $M_v$ that sends every input role to its exchanged role, with endpoint difference $I_m$ and total edge rank $s<Wm$. (on [Bit network and rational matrix interface](bit-network-interface.md))
- A shear $(H,D)\mapsto(H+AD,D)$ over $\mathbb Z/q^b\mathbb Z$ with $A$ of rank $a$ is done with exactly $a$ interchanges of $b$-digit fields and $O(V)$ further work. (on [Cost of a rational matrix shear](matrix-shear.md))
- Two disjoint chunks of $e=m^k$ radix-$q$ digits each can be interchanged in $O(Ve^\tau)$ time when a preceding row field has length divisible by $W^k$. (on [Power-width address interchange](power-width-interchange.md))
- Two disjoint chunks of any width $u$ bits in an array of $V$ bits, with no row restriction, can be interchanged in $O(Vu^\tau)$ time.

It serves these steps, from the main result down:

- A single deterministic Turing machine with a fixed alphabet and a fixed number of one-dimensional tapes multiplies two $n$-bit integers exactly in $O(n(\lg n)^{1-\kappa})$ worst-case time with $\kappa=2^{-182}$, refuting the $n\log n$ optimality conjecture in this model.; how: Convert the product into a convolution computed by synthetic transforms after Gaussian resampling and a chirp, speed up the transforms with two network-based tape procedures (address interchange and simultaneous butterfly layers), then prove the product exact and add up the time. (on [Integer multiplication in O(n (lg n)^(1−κ)) time](main-theorem.md))

## Steps this page derives

The argument gives this page these sub-steps of its step (S1.6). Derive each here, so the reader can check how it follows from what comes before: stating its result, or citing the source for it, does not derive it. The passages report each step in the source; they ground the derivation and are not to be copied onto the page. In `derivations`, give each step's id and the passage on the page that derives it, copied exactly: submit checks that the passage is on the page, and the audit judges whether it is enough.

**S1.6**: Two disjoint chunks of any width $u$ bits in an array of $V$ bits, with no row restriction, can be interchanged in $O(Vu^\tau)$ time.
How: Supply the missing rows from the chunks' own high digits, split the width in base $m$ into power-width pieces, and pad binary chunks to powers of $q$.
- S1.6.1: Interchange the top $\rho=O(\log e)$ digits of the chunks one pair at a time, join them into a row field, and pad the row count to a multiple of $W^k$.
  Source: "Write each chunk as its top $\rho$ digits followed by its remaining" (paper)
  > Write each chunk as its top $\rho$ digits followed by its remaining digits, denoted $h^+,h^-$ and $d^+,d^-$, respectively. Interchange the corresponding high digits one pair at a time. The high positions now contain $d^+$ and $h^+$; move the digits of the latter, preserving their order, into consecutive positions immediately after $d^+$. Suppressing the prefix and suffix, the field orders during the construction are \[ \begin{aligned} &(h^+,h^-,g,d^+,d^-),\\ &(d^+,h^+,h^-,g,d^-) &&\text{after exchanging and joining the high parts},\\ &(d^+,h^+,d^-,g,h^-) &&\text{after interchanging the remaining parts},\\ &(d^+,d^-,g,h^+,h^-) &&\text{after separating the high parts again}. \end{aligned} \] In the two middle orders, the first two fields form a row field of range \[ R=q^{2\rho}\ge W^k \] before both remaining chunks. The joining moves preserve the order of the remaining digits and the gap field. Exchanging and joining the high parts costs $O(V\rho)$ by Lemma~\ref{lem:elementary-streams}. Append zero rows, separately within each preceding prefix, until the row count is \[ R'=W^k\left\lceil R/W^k\right\rceil<2R. \] This changes the row field's length; it need not remain a power of $q$ during the computation. The padded logical volume is less than $2V$.
- S1.6.2: Write the remaining width in base $m$ as $\sum_j a_jm^j$ and apply the power-width interchange to $a_j$ pieces of width $m^j$; the total cost is $O(Ve^\tau)$.
  Source: "Write the remaining width in base $m$:" (paper)
  > Write the remaining width in base $m$: \[ e-\rho=\sum_{j=0}^{k} a_jm^j,\qquad 0\le a_j<m. \] Partition each remaining chunk into corresponding consecutive pieces, with $a_j$ pieces of width $m^j$. Apply Proposition~\ref{prop:power-interchange} to each corresponding pair. All these calls use the same row field of length $R'$, which is divisible by each required $W^j$. At completion every call preserves its row address, so it neither mixes genuine rows with padded rows nor depends on the values used for padding. The padded zero rows remain zero at these completed-call boundaries. The sum of the costs is bounded by \[ O(V)\sum_j a_jm^{j\tau}=O(Ve^\tau). \] The fixed bound $a_j<m$ and the inequality $m^k<me$ make this a geometric sum of the stated order. Scan away the padded rows, then reverse only the moves that joined the high parts. This is the last transition in the field-order display: $h^+$ returns to the later high position, while $d^+$ stays in the earlier one. The final order is therefore the required interchange of the two full chunks. The remaining movement cost $O(V\log(2e))$ is $O(Ve^\tau)$. Computing the piece list and every new shape is polynomial in $\log(2V)$ per piece. There are at most $(m-1)(k+1)=O(\log(2e))$ pieces; since $V\ge q^{2e}$, their total descriptor cost is a fixed power of $\log(2V)$ and hence $O(V)$. Each power-width call uses the fixed schedule proved above and leaves its work tapes clean.
- S1.6.3: Pad the two binary ranges of $u$ bits to radix-$q$ ranges, which enlarges the volume by a fixed factor, interchange, and scan away the padding, giving $O(Vu^\tau)$.
  Source: "Finally, for binary chunks let $e$ be the least integer with" (paper)
  > Finally, for binary chunks let $e$ be the least integer with $q^e\ge2^u$. Pad each of the two chunk ranges numerically from $[2^u]$ to $[q^e]$, putting zero in an entry if either chunk coordinate is invalid. The increase in volume is less than $q^2$, a fixed factor. The padding is a nested scan through consecutive valid and invalid intervals, with their lengths prepared from the shape. It costs $O(V)$ with a constant depending on $q$. Lexicographic radix-$q$ order of $[q^e]$ is its numerical order, just as binary order of $[2^u]$ is its numerical order. Padding therefore preserves the order of the existing entries without converting the digits of every address. The completed interchange preserves the set on which both chunk coordinates lie below $2^u$. Scan away the other entries to recover the desired binary array. Since $e=O(u)$ for fixed $q$, the bound is $O(Vu^\tau)$. \end{proof}

### Steps derived on other pages

Where this page uses one, state its result and link the page that derives it. Do not derive it here, even where a passage above proves it.

- S1.1 (on [Elementary stream operations](stream-ops.md)): Splitting an array on a digit, moving a digit, and splitting whole rows cyclically into $W$ streams (and merging them back) each take $O(V)$ time on a fixed number of tapes.
- S1.2 (on [Controlled cyclic shifts](controlled-shifts.md)): A target field $y$ that follows its control fields can be updated $y\mapsto y+a(\text{controls}) \pmod Q$, or multiplied by a unit, in $O(V)$ time.
- S1.3 (on [Bit network and rational matrix interface](bit-network-interface.md)): There is a fixed network of $W$ XOR gates on wires with rational $m\times m$ matrix labels $M_v$ that sends every input role to its exchanged role, with endpoint difference $I_m$ and total edge rank $s<Wm$.
- S1.4 (on [Cost of a rational matrix shear](matrix-shear.md)): A shear $(H,D)\mapsto(H+AD,D)$ over $\mathbb Z/q^b\mathbb Z$ with $A$ of rank $a$ is done with exactly $a$ interchanges of $b$-digit fields and $O(V)$ further work.
- S1.5 (on [Power-width address interchange](power-width-interchange.md)): Two disjoint chunks of $e=m^k$ radix-$q$ digits each can be interchanged in $O(Ve^\tau)$ time when a preceding row field has length divisible by $W^k$.

### Other pages these steps use

Link each where the derivation uses it, stating what it gives; do not derive its results again.

- [Elementary stream operations](stream-ops.md)

## Prerequisite pages (link each on first use, exactly as written, and give a relation for each)

- [Power-width address interchange](power-width-interchange.md): Interchanging two chunks of e = m^k digits in O(V e^τ) time by splitting rows among the W roles, applying shears on each edge as smaller interchanges, and recursing.
  Shown as: Fast address-chunk interchange *<predicate>* Power-width address interchange

## Pages that build on this one

- Packed selected-bit addition: XORing corresponding selected bits of two address chunks (a binary row addition) using cyclic shifts of whole chunk ranges and a deterministic repair of a small explicitly known set of exceptional addresses.
- Synthetic transform layout: A positional permutation of record addresses that places the selected bits of all axes at a common offset in consecutive K-bit chunks, plus the retained bit-reversed frequency order, so that simultaneous layers apply in every round.
- Tensor resampling on a padded box: Applying the one-dimensional resampling maps A and B₀ line by line along each of d axes of a zero-padded power-of-two box, moving axes into place with chunk swaps.
- Chinese-remainder map to prime cyclic axes: An algebra isomorphism that rewrites one cyclic convolution of coprime length S = ∏ sᵢ as a multidimensional cyclic convolution on axes of the distinct prime lengths sᵢ, implemented on tape by triangular controlled shifts and axis moves.
- Total time bound and the power saving: Dividing every cost by Tp gives seven dominant powers of p, each at most 1−2κ; the smallest margin 2^{-181} = 2κ makes the total O(n (lg n)^{1-κ}).

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

## Evidence from the sources, with the paragraph around each quote

- "The first procedure interchanges two disjoint contiguous address fields" (paper)

  > The first procedure interchanges two disjoint contiguous address fields of $u$ bits in an array whose addresses form a complete Cartesian product. Its cost is $O(Vu^\tau)$ for a fixed $\tau<1$. Although its final effect is an exact permutation, its intermediate steps form pointwise XOR combinations of data. The final permutation is exact for arbitrary initial auxiliary values.

- "(p,h,g,d,z)\longmapsto(p,d,g,h,z)" (paper)

  > \begin{lemma}[Arbitrary-width interchange] \label{lem:chunk-swap} There exist fixed $0<\tau<1$ and one fixed finite-alphabet multitape procedure with the following property. For positive integers $P,G,B$ and $u\ge1$, it transforms an arbitrary bit array on \[ [P]\times[2^u]\times[G]\times[2^u]\times[B] \] by the address permutation \[ (p,h,g,d,z)\longmapsto(p,d,g,h,z) \] in $O(Vu^\tau)$ steps, where $V=PG B\,2^{2u}$. The integers specifying the shape are part of the input. The time includes their processing, all padding and unpadding, and fixed-tape workspace cleanup. In particular, $B=1$ is allowed. \end{lemma}


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

Write the body to `exegenius/tasks/0027-write-address-chunk-interchange.answer.md` and the JSON to `exegenius/tasks/0027-write-address-chunk-interchange.answer.json`, then run `exegenius submit 0027-write-address-chunk-interchange` in the project directory. If submit rejects the answer, fix the answer and submit again.

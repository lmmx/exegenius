**This page derives no steps of the argument: there are no items to judge. Give an empty `items` list, and judge the page's mathematical errors only.**

## This page

Concept: Controlled cyclic shifts
Gloss: An update y ↦ y + a(controls) mod Q of a target field that follows its control fields on tape, done in O(V) by splitting each fiber at the offset and merging the two pieces in the opposite order; also multiplication by a fixed unit and negation of the target.
Also called: controlled shifts, ordered affine streams
## This page in the argument

The step: Controlled cyclic shift: an update $y\mapsto y+a(\text{controls})\pmod Q$ of a target field following its control fields costs $O(V)$, by splitting each fiber at the offset and merging the two pieces in opposite order; multiplication by a fixed unit and negation also cost $O(V)$.

It serves these steps, from the main result down:

- A single deterministic multitape Turing machine multiplies two $n$-bit integers exactly in $O(n(\lg n)^{1-\kappa})$ worst-case time with $\kappa=2^{-182}$.; how: Reduce the product to a convolution of short integer coefficients, evaluate the convolution through power-of-two transforms over $\mathbb C[y]/(y^r+1)$, and replace the two tape operations that each cost a factor $\log n$ (address rearrangement and butterfly layers) by procedures whose cost per bit is a power of their parameter below one; then total the costs. (on [Integer multiplication below n log n](main-theorem.md))

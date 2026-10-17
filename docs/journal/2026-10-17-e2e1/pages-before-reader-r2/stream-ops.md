Elementary stream operations are linear-time scans of an array stored on tape in lexicographic order; they are the data-movement primitives from which the later procedures for interchanging address fields are assembled, and they set the unit of cost, one pass, that those procedures must beat.

Throughout, $V$ is the volume of an array, the number of digits it stores, and $\beta\ge 2$ is the radix, the number of values one digit can take. Here $\beta$ is a radix, not a sequence length as on the discrete Fourier transform and cyclic convolution pages.

## Why one pass is the unit

All four operations below run on the [Fixed finite-tape machine model](machine-model.md), where each head moves at most one cell per step, so one pass over the array costs time proportional to $V$. The $n$-bit inputs occupy $V=\Theta(n)$ stored bits, so a pass costs $\Theta(n)$.

A transform built from [Butterflies and the fast transform](butterfly.md) applies two-point steps, each on one address bit, so it has $\Theta(\log n)$ levels, and each level needs the data rearranged for the paired entries to meet, then the arithmetic. Done naively, that is one pass per level, and scanning an array of $\Theta(n)$ stored bits at each of $\Theta(\log n)$ transform levels already costs $\Theta(n\log n)$ <cite data-source="paper">Scanning an array of $\Theta(n)$ stored bits at each of $\Theta(\log n)$ transform levels already costs $\Theta(n\log n)$</cite>, the bound to be beaten.

The number of levels is fixed by the two-point structure, so what must shrink is the cost of a level: the same source concludes that savings are needed in both movement and arithmetic, and that two tape procedures supply them. The operations here are the baseline: each costs one pass, so they can be used a constant number of times but not once per level. Later pages build on them procedures that move address fields, or perform several levels together, in less than one pass per level.

## The operations

For a fixed radix, each of the following takes $O(V)$ steps on a fixed number of one-dimensional tapes <cite data-source="paper">Each of the following operations on an array of volume $V$ takes $O(V)$ steps on a fixed number of one-dimensional tapes</cite>:

1. Permuting the pair of values at two chosen digit positions, adjacent or not.
2. Moving one chosen digit to another position, keeping the order of the positions in between.
3. Splitting on one chosen digit, applying a fixed pointwise map to the aligned streams, and merging them back.
4. Dealing whole rows cyclically into $W$ streams, where $W$ is the fixed number of streams, and, when $W$ divides the number of rows, merging the streams in any fixed order of the row classes.

In item 4 the array is indexed by $[P]\times[R]\times[B]$, where $P$ is the number of prefix values, $R$ is the number of rows (here $R$ is a row count, not the scalar ring) and $B$ is the number of entries per row.

The bounds include setting up counters and returning the work tapes to their initial state, and the constants depend on the radix, the pointwise map and $W$ <cite data-source="paper">The bounds include shape preparation, counter initialization and resets, and returning or clearing the visited portions of work tapes.</cite>

## Splitting

Splitting is a stable partition. A head scanning the array tracks its position with a counter and copies each entry onto the tape belonging to the value of the chosen digit, preserving order within each tape. Reading the $\beta$ tapes one after another lists the entries grouped by that digit, with the remaining digits still in their original order, which is exactly that digit moved to the front. For example, with digits $(a,b,c)$ and $b$ chosen, the stream for each value $j$ of $b$ holds the entries $(a,j,c)$ in their original order, and concatenating the streams gives the order $(j,a,c)$.

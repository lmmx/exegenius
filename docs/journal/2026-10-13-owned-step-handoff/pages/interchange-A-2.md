Fast address-chunk interchange is a procedure that swaps two disjoint equal-width chunks of address bits in an array, exactly, in time that grows more slowly than the chunk width. The multiplication uses it to move the axes of a multidimensional array into place at several stages, where a plain scan of the whole array for each move would cost too much.

Here $u$ is the chunk width, the number of bits in each chunk, and $V$ is the number of stored bits in the array. The exponent $\tau$ is a fixed number with $0<\tau<1$, the same for every width. The time is $O(Vu^\tau)$ <cite data-source="paper">in $O(Vu^\tau)$ steps</cite>, and the final effect is an exact permutation.

Per array bit the cost is $O(u^\tau)$, and $u^\tau/u=u^{\tau-1}$ tends to zero as $u$ grows, so the saving is a power of the width.

The intermediate steps combine data by XOR, which makes exactness less obvious. Over $\mathbb Z/2\mathbb Z$ the sum is XOR. A shear $H\leftarrow H+D$ replaces the block $H$ by $H+D$ bitwise, where $H$ and $D$ are two equal blocks of bits. Three shears in turn swap the blocks: $H\leftarrow H+D$, then $D\leftarrow D+H$, then $H\leftarrow H+D$. For one-bit blocks starting at $H=1$, $D=0$, the three steps give $(H,D)=(1,0)$, then $(1,1)$, then $(0,1)$.

The method realises the swap as a fixed network of XOR gates on $W$ wires whose edge changes are smaller shears. A shear is done by as many smaller interchanges as the rank of its matrix, so the total number $s$ of smaller operations satisfies $s<Wm$, and the recursion saves a power of the chunk width <cite data-source="paper">the resulting recurrence saves a power of the chunk width</cite>.

The construction is based on the [Power-width address interchange](power-width-interchange.md), which handles chunks whose width is a power of $m$ under a row restriction; this page extends it to every width and drops that restriction.

The XOR combinations are undone by later steps, and the network restores its scratch wires whatever they held at the start, so the data end in exactly the swapped order <cite data-source="paper">exact for arbitrary initial auxiliary values</cite>.

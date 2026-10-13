Fast address-chunk interchange is the tape procedure that swaps two disjoint chunks of equal width in an array of stored bits. A chunk is a contiguous group of address fields, and the procedure leaves all other fields in place. The multiplication algorithm uses it to move array axes.

Here $V$ is the number of stored bits in the array. Here $u$ is the number of bits in each chunk. Here $\tau$ is a fixed exponent below one. The cost is $O(Vu^\tau)$ <cite data-source="paper">Its cost is $O(Vu^\tau)$ for a fixed $\tau<1$</cite>.

The swap reduces to one shear. Write $H$ and $D$ for the values of the two chunks. Three updates take $(H,D)$ to $(D,H)$:

$$D\leftarrow D-H,\qquad H\leftarrow H+D,\qquad D\leftarrow H-D.$$

With $H=3$ and $D=5$ they give $(3,2)$, then $(5,2)$, then $(5,3)$. The other two updates are linear, so the middle one, which adds $D$ to $H$, sets the cost.

A fixed circuit of $W$ wires with pointwise XOR gates realises this shear. Gates and the circuit's inputs and outputs get rational $m\times m$ matrices; an edge is a wire segment between two of them, and its change is the difference of their matrices. An edge whose change has a given rank costs that many smaller interchanges, each on a $1/W$ share of the array. So the number of recursive calls is the total rank $s$. The rank-saving bound $s<Wm$ <cite data-source="paper">s<Wm.</cite> gives $s/W<m$, so some fixed $\tau<1$ satisfies $s/W<m^\tau$. Each level multiplies the time per stored bit by at most $s/W$, plus a constant, so that time is at most a constant times the width to the power $\tau$. The power-width case is worked out in [Power-width address interchange](power-width-interchange.md).

The power-width case needs a row field, which the array supplies from its high digits. Binary chunks are padded to a fixed radix. The rest of the width is written in base $m$ and cut into pieces whose widths are powers of $m$; each piece is one such call, so the general width is reduced to [Power-width address interchange](power-width-interchange.md).

The intermediate steps XOR-combine data, yet the result is an exact permutation. Each wire's output label minus its input label is the identity, so the composed address map is the identity whatever the scratch wires hold <cite data-source="paper">exact for arbitrary initial auxiliary values</cite>.

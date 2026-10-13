A simultaneous butterfly layer applies the normalized butterfly $H_0(u,v)=((u+v)/2,(u-v)/2)$ to one common bit offset in several address chunks at once, with one final truncation. It speeds up the multiplication algorithm's butterfly layers.

Each address holds a record of $r$ complex coefficients; $p$ is the fractional-bit count per component.

Here $D$ is the number of chunks, $K$ the bits per chunk, and $d$ bounds $D$. Naively, $H_0$ takes $D$ whole-array passes. Here $V$ is the logical bit volume, the number of stored bits. Here $\lambda'$ is a fixed exponent. The layer instead runs in $O(Vd^{\lambda'})$ time <cite data-source="paper">one selects the same bit position in each of at most $d$ equal-width address chunks, and the cost is $O(Vd^{\lambda'})$ for a fixed $\lambda'<1$.</cite>

The method rewrites $H_0$ via the two-point kernel $C$, whose diagonal entries are $\frac{1+i}{2}$ and whose off-diagonal entries are $\frac{1-i}{2}$. Let $\mathcal S_D$ multiply each record by $i$ to the power of its selected bits equal to $1$. Then $H_0^{\otimes D}=\left(\frac{1-i}{2}\right)^D\mathcal S_D\,C^{\otimes D}\,\mathcal S_D$, where $C^{\otimes D}$ is the $D$-fold tensor power. Scalars and phases are exact Gaussian dyadics (complex numbers with dyadic parts), so the layer reduces to $C^{\otimes D}$.

A contraction does not enlarge the maximum coefficient norm. $C$ is not one: its row absolute sums are $\sqrt2$, so $C^{\otimes D}$ can enlarge values by up to $2^{D/2}$. $H_0$ has row absolute sums $1$, so the layer runs exactly and truncates once.

The layer computes the tensor power by the [Recursive implementation of the phase network on selected bits](phase-edge-recursion.md). Here $e$ counts the selected axes in a group, split into $m$ slots, the label-space dimension. After basis changes, the network makes $s$ recursive calls in all.

Here $W$ counts the wire roles. The layer schedules these calls by the [Role streams and fixed-tape depth-first scheduling](role-streams.md), which distributes rows cyclically among them, so each call works on volume $V/W$. Each level multiplies the volume by $s/W<m$ while the group size shrinks by $m$, so the work is a power of the group size with exponent $\max\{0,\log_m(s/W)\}<1$.

The layer keeps intermediate values exact by the [Exact Gaussian dyadic arithmetic and guard widths](exact-dyadic-guard.md). Here $\Delta=\lceil C_0d^{C_1}\rceil$ is the number of guard bits, with $C_0$ and $C_1$ fixed constants. The layer uses $p+\Delta$ fractional bits and $\Delta+3$ integer and sign bits per real component <cite data-source="paper">The procedure uses at most $p+\lceil C_0d^{C_1}\rceil$ fractional bits and $\lceil C_0d^{C_1}\rceil+3$ integer and sign bits per real component internally</cite>.

Finally, truncation toward zero to $2^{-p}\mathbb Z$ keeps coefficients in the unit disk, with error below $\sqrt2\,2^{-p}$ <cite data-source="paper">componentwise truncation preserves that disk and has error less than $\sqrt2\,2^{-p}$</cite>.

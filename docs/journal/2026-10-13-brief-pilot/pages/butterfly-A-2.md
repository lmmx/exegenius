A simultaneous butterfly layer applies the two-point butterfly to one selected bit in each of several address chunks at once. Let $d$ be the largest number of chunks a layer acts on. A layer acts on $D\le d$ chunks, and $D$ counts them. $V$ counts the bits of data. Here $\lambda'$ is a fixed exponent below $1$. The layer costs $O(Vd^{\lambda'})$ <cite data-source="paper">the cost is $O(Vd^{\lambda'})$ for a fixed $\lambda'<1$</cite>, one of two savings behind the main result. Separate butterfly factors would take $D$ whole-array passes <cite data-source="paper">factors would require $D$ whole-array passes</cite>.

A *chunk* is a block of $K$ address bits, and $K$ counts them. Here $K=\lfloor d^c\rfloor$, where $c$ is a fixed exponent. Let $e$ be the number of chunks in a group. Let $m$ be the number of slots a group is split into. Input parts lie on $2^{-p}\mathbb Z$, where $p$ counts fractional bits.

The layer is the tensor product $H_0^{\otimes D}$ of the two-point butterfly $H_0(u,v)=((u+v)/2,(u-v)/2)$ on the selected bits. Write $C=\frac12\begin{pmatrix}1+i&1-i\\1-i&1+i\end{pmatrix}$ for the complex two-point kernel. Applying $H_0=\frac{1-i}{2}\,\mathrm{diag}(1,i)\,C\,\mathrm{diag}(1,i)$ to every selected bit gives
$$H_0^{\otimes D}=\Big(\tfrac{1-i}{2}\Big)^D\,\mathcal S_D\,C^{\otimes D}\,\mathcal S_D,$$
where $\mathcal S_D$ multiplies each value by $i$ for each selected bit equal to $1$. So it suffices to compute $C^{\otimes D}$.

Here $s$ counts child calls (recursive calls) per group. The layer is built from the [Recursive implementation of the phase network on selected bits](phase-edge-recursion.md): each group's $C^{\otimes e}$ is computed by $s$ child calls, each on one slot.

Here $W$ counts the role streams. The recursion is scheduled by [Role streams and fixed-tape depth-first scheduling](role-streams.md): each child works on a stream of volume $V/W$.

Let $F(e)$ be the time per unit of volume for a group of $e$ chunks, and let $\tau<1$ be a fixed exponent.
$$F(e)\le\frac{s}{W}\,F\!\left(\frac{e}{m}\right)+O\big((eK)^\tau+1\big).$$
The exponent $\theta<1$ is chosen above $\tau(1+2c)$ and $\log_m(s/W)$, the latter below $1$ since $s<Wm$. For $e\ge d^{1/2}$ the overhead is $O(e^\theta)$, since $K\le e^{2c}$ there. Each level is a fixed fraction of the one above, so the top level dominates. Hence the layer costs $d^{\lambda'}$ passes instead of $d$.

Coefficients are Gaussian dyadics (complex numbers with dyadic parts). Here $\Delta$ counts the guard bits; it grows as a fixed power of $d$. The whole layer is evaluated in [Exact Gaussian dyadic arithmetic and guard widths](exact-dyadic-guard.md): each coefficient keeps $p+\Delta$ fractional bits and $\Delta+3$ integer and sign bits. Only then is the result truncated toward zero to $2^{-p}\mathbb Z$ <cite data-source="paper">The procedure evaluates the entire layer exactly before this final truncation.</cite>, an error below $\sqrt2\,2^{-p}$.

A simultaneous butterfly layer is the second tape procedure of the main result. Let $d$ bound the number of chunks, and $D\le d$ be the number in a layer, each of $K=\lfloor d^c\rfloor$ bits, where $c>0$ is a fixed number. It applies the two-point butterfly $H_0(u,v)=((u+v)/2,(u-v)/2)$ at one bit offset in up to $d$ address chunks. Write $V$ for the logical bit volume and $\lambda'<1$ for a fixed constant. It costs $O(Vd^{\lambda'})$ <cite data-source="paper">one selects the same bit position in each of at most $d$ equal-width address chunks, and the cost is $O(Vd^{\lambda'})$ for a fixed $\lambda'<1$.</cite> rather than one whole-array pass per chunk.

Coefficients lie on $2^{-p}Z[i]$, with $p$ the number of fractional bits.

Let $C_{00}=C_{11}=(1+i)/2$ and $C_{01}=C_{10}=(1-i)/2$ be the kernel $C$, and let $\Xi$ be the diagonal matrix with entries $1$ and $i$. Since $H_0=((1-i)/2)\,\Xi C\,\Xi$, the layer is
$$H_0^{\otimes D}=((1-i)/2)^D\,\Xi^{\otimes D}\,C^{\otimes D}\,\Xi^{\otimes D}.$$
Here $\Xi^{\otimes D}$ multiplies address $x$ by $i^{x_1+\cdots+x_D}$, its selected bits $x_1,\ldots,x_D$.

The network has $W$ wire roles. It splits $e=mf$ chunks into $m$ slots of $f$ chunks, and makes $s$ child calls, each computing $C^{\otimes f}$ on one slot. Thus $C^{\otimes e}$ is computed by [Recursive implementation of the phase network on selected bits](phase-edge-recursion.md).

The recursion is scheduled by [Role streams and fixed-tape depth-first scheduling](role-streams.md). A row $h=Wg+w$ goes to role $w$ of $W$, indexed there by $g$.

Let $F(e)$ be the time per unit volume. Put $a=s/W$. The binary changes of basis cost $O(V(L^\tau+1))$ by [Packed selected-bit addition with guard repair](packed-selected-bit-addition.md), with $L=fK$ and fixed $0<\tau<1$. Hence $F(e)\le aF(e/m)+O((eK)^\tau+1)$, since the $s$ child calls run on volume $sV/W$. Here $a<m$ by [Recursive power saving from s/W < m](recursive-power-saving.md): each level divides chunks by $m$ but multiplies work by $a$.

Fix exponents $\sigma$ and $\mu$ with $\max\{0,\log_m a\}\le\sigma$, $\max\{\tau,\sigma\}<\mu<1$ and $\tau(1+2c)<\mu$. Recurse while $e\ge\sqrt d$, else apply $e$ kernels individually. There $K\le e^{2c}$, so overhead is $O(e^\mu)$. Depth-$j$ nodes have $e/m^j$ chunks and weight $a^j$; as $a<m^\mu$, they sum to
$$\sum_j a^j\,O\bigl((e/m^j)^\mu\bigr)=O(e^\mu).$$
At the stopping depth, leaf groups of $k=e/m^j<\sqrt d$ chunks cost $a^jk\le e^\sigma k^{1-\sigma}\le e^\sigma d^{(1-\sigma)/2}$. Hence $F(e)=O(d^\mu+d^{\sigma+(1-\sigma)/2})$ for $e\le d$.

The first $q_0=O(\log d)$ chunks supply the row indices, at cost $O(V\log d)$. Writing $D-q_0=\sum_\ell\delta_\ell m^\ell$ in base $m$ gives $\delta_\ell$ blocks of $m^\ell$ chunks. Summing gives
$$V\cdot O\bigl(\log(2d)\,[d^\mu+d^{\sigma+(1-\sigma)/2}+1]\bigr)=O(Vd^{\lambda'})$$
for any $\max\{\mu,\sigma+(1-\sigma)/2\}<\lambda'<1$.

The whole layer is evaluated exactly in [Exact Gaussian dyadic arithmetic and guard widths](exact-dyadic-guard.md), with $p+\Delta$ fractional bits and $\Delta+3$ integer and sign bits, where $\Delta=o(p)$ is the guard width. Packed repairs stay cheap: $80(f-1)2^{-K}\to0$.

One truncation toward zero moves each component by less than $2^{-p}$, so the error is below $\sqrt2\,2^{-p}$. Values stay in the unit disk, since $\sqrt2\,H_0$ is unitary.

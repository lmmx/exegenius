Cost accounting totals the running time of every step of [The multiplication algorithm](multiplication-algorithm.md) and shows that it is $O(n(\lg n)^{1-\kappa})$, with $\kappa=2^{-182}$ fixed by the smallest margin below.

Let $V=Tp=\Theta(n)$, the number of stored bits when $T$ coefficients carry $\Theta(p)$ bits each. With the fixed exponents of [Parameter and prime-length choice](parameter-choice.md), namely $\epsilon=2^{-75}$, $c=2^{-56}$, $\delta=1/16$, $1-\tau=2^{-50}$ and $1-\lambda'=2^{-54}$, the sizes are $d=\Theta(p^\epsilon)$ axes, chunk width $K=\Theta(p^{\epsilon c})$ bits, $\ell=\Theta(p^{1-\epsilon})$ address bits per axis and $\alpha=\Theta(p^{1/4+\epsilon/2})$.

Dividing by $V$, seven operations dominate, each a power $p^{1-g_i}$ with margin $g_i>0$ below one.

| Operation | Cost$/V$ | Power of $p$ | Margin |
|---|---|---|---|
| Prefix moves | $dK$ | $\epsilon(1+c)$ | $g_1=1-\epsilon(1+c)$ |
| Chunk exchanges | $pK^{\tau-1}$ | $1-\epsilon c(1-\tau)$ | $g_2=\epsilon c(1-\tau)$ |
| Layer rounds | $\ell d^{\lambda'}$ | $1-\epsilon(1-\lambda')$ | $g_3=\epsilon(1-\lambda')$ |
| Axis layouts | $d^2(1+\ell^\tau)$ | $\tau+\epsilon(2-\tau)$ | $g_4=1-\tau-\epsilon(2-\tau)$ |
| Gaussian lines | $dp^{1/2+\delta}\alpha$ | $3/4+\delta+3\epsilon/2$ | $g_5=1/4-\delta-3\epsilon/2$ |
| Chirps | $dp^\delta$ | $\epsilon+\delta$ | $g_6=1-\delta-\epsilon$ |
| Products | $\log(rp)$ | $1-\epsilon$ | $g_7=\epsilon$ |

- Prefix moves and single butterfly rounds are $O(dK)$ linear scans of cost $O(V)$ each, in [Tape layout for synthetic transforms](transform-layout.md).
- The layout rearranges the $d\ell=O(p)$ address bits of the records by exchanging width-$K$ chunks, so it needs $O(p/K)$ exchanges <cite data-source="paper">Its positional layout uses $O(p/K)$ exchanges of width-$K$ chunks, because $d\ell=O(p)$.</cite> Each chunk exchange costs $O(VK^\tau)$ by [Fast address-chunk interchange](address-interchange.md), and the normalized total is $O(p/K)K^\tau=O(pK^{\tau-1})$. The permutations of [Gaussian resampling](gaussian-resampling.md) are never executed, since they cancel ([Retained permutation cancellation](retained-permutations.md)), so no row charges them.
- Each of at most $\ell$ simultaneous rounds costs $O(Vd^{\lambda'})$ by [Simultaneous butterfly layer](simultaneous-layer.md).
- The [Chinese-remainder address map](crt-map.md) needs $O(d^2)$ adjacent axis moves. Each costs $O(V(1+\ell^\tau))$ <cite data-source="paper">Each costs $O(V(1+\ell^\tau))$: peel a single excess bit when widths</cite>. Here $d^2\ell^\tau=p^{2\epsilon}p^{(1-\epsilon)\tau}$.
- In [Gaussian resampling](gaussian-resampling.md), axis $i$ of length $t_i$ has at most $T/t_i$ lines, each costing $O(t_ip^{3/2+\delta}\alpha)$, totalling $O(dTp^{3/2+\delta}\alpha)$.
- The chirp numerators of the [Bluestein chirp identity](chirp.md) have $O(p)$ bits and sum $d$ terms. The [Established O(n log n) multiplier](hvdh-multiplier.md), taking $O(N\log 2N)$ on $N$ bits, forms their squares and products in $O(dp^{1+\delta})$ per entry.
- There are $T/r$ ring-product records, each needing four products on $O(rp)$ bits after [Signed packing](signed-packing.md). These calls have length $o(n)$. The total is $O(Tp\log(rp))$, and $\log(rp)=O(p^{1-\epsilon})$ because $r$ is at most $2^{O(p/d)}$.

Substitution gives $g_1>1/2$, $g_2=2^{-75}2^{-56}2^{-50}=2^{-181}$, $g_3=2^{-75}2^{-54}=2^{-129}$, $g_4>2^{-51}$, $g_5>1/8$, $g_6>1/2$ and $g_7=2^{-75}$, so the smallest margin is $2^{-181}=2\kappa$ <cite data-source="paper">Thus $\min_i g_i=2^{-181}=2\kappa$.</cite>

Plain scans would be $\tau=1$, costing $p$ per unit volume and giving margin $0$; likewise $\lambda'=1$ gives the layer row margin $0$. Address movement saves a power of $p$ only through the two new tape procedures, and each saving is tiny. In $g_3$ the layer saving $1-\lambda'$ is scaled by $\epsilon$, since only $d=\Theta(p^\epsilon)$ axes share a layer. In $g_2$ there is a further factor $c$: an exchange acts on chunks of only $K=\Theta(p^{\epsilon c})$ bits and saves a factor $K^{-(1-\tau)}=p^{-\epsilon c(1-\tau)}$ against a full pass.

For setup, a visit is a node of the layer or address-layout recursions, or a round or group invocation inside them. They have constant branching, depth $O(\log p)$ and polynomially many rounds and groups. Fix an exponent $A$ with at most $p^A$ visits in total, and an exponent $C$ with descriptor setup of at most $p^C$ steps per visit. The setup costs at most $p^{A+C}=o(Tp)$ <cite data-source="paper">costs at most $p^{A+C}=o(Tp)$.</cite> Per-record and per-line setup, $O(Tp^{A+C}/r)$ and $O(dTp^C/r)$, is $o(Tp)$ since $r$ exceeds every fixed power of $p$.

Exceptional addresses, where the packed shifts of [Packed selected-bit addition](packed-bit-addition.md) fail, are bounded to a fraction $O(d2^{-K})$ of addresses. Let $V_{\rm call}$ be the volume of one packed change of basis. A record has $\Theta(rp)$ bits and an exceptional one adds an $O(p)$-bit key, so the stream of exceptional records has length $O\left(\frac{V_{\rm call}}{rp}d2^{-K}(rp+p)\right)=O(V_{\rm call}d2^{-K})$. Stable bit sorting scans it $O(p)$ times, so over at most $p^A$ visits repair costs $O(Vp^{O(1)}2^{-K})=o(V)$, because $2^{-K}$ beats every power of $p$. Global setup and prime search cost $o(V)$.

The table rounds each bound up to a power of $p$. The lemmas may leave fixed powers of $\log p$, for example from the $O(\log p)$ recursion depth; the sources do not itemize them. The slack covers them: a fixed power of $\log p$ is at most $p^\kappa$ for large $p$, since it grows more slowly than any positive power of $p$. Each of the seven powers is at most $p^{1-2\kappa}$, so with these logarithms it is at most $p^{1-2\kappa}p^{\kappa}=p^{1-\kappa}$. Half the smallest margin pays for logarithms and half remains as the saving. Any positive $\kappa$ below $2^{-181}$ would pass the same test; $g_2$ sets the scale of $\kappa$. The total is $O(Tp\,p^{1-\kappa})=O(n(\log n)^{1-\kappa})$ <cite data-source="paper">O(Tp\,p^{1-\kappa})=O(n(\log n)^{1-\kappa}).</cite>

The bound needs inputs beyond a cutoff chosen past all the size, precision, margin and layout conditions. Below it the machine uses schoolbook multiplication; only finitely many lengths lie there, so their running times are bounded by one constant <cite data-source="paper">many smaller lengths are included by enlarging the implicit constant.</cite> With $\lg n=\max\{\lceil\log_2n\rceil,1\}$, within a constant factor of $\log n$ and at least $1$, the single machine therefore runs in $O(n(\lg n)^{1-\kappa})$ for every $n\ge1$.

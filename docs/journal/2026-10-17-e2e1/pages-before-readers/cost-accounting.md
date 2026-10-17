Cost accounting totals the running time of every step of [The multiplication algorithm](multiplication-algorithm.md) and shows that it is $O(n(\lg n)^{1-\kappa})$, where $\kappa=2^{-182}$ is the saved exponent.

Let $V=Tp=\Theta(n)$ be the volume, the number of stored bits of $T$ coefficients of $p$ bits (a volume here, not a label). The fixed exponents of [Parameter and prime-length choice](parameter-choice.md) are $\epsilon=2^{-75}$, $c=2^{-56}$, $\delta=1/16$, $\tau=1-2^{-50}$ and $\lambda'=1-2^{-54}$, giving $d=\Theta(p^\epsilon)$, $K=\Theta(p^{\epsilon c})$, $\ell=\Theta(p^{1-\epsilon})$ and $\alpha=\Theta(p^{1/4+\epsilon/2})$.

Dividing by $V$, seven operations dominate, each a power $p^{1-g_i}$ with margin $g_i>0$ below one (a margin, not a guard value).

| Operation | Cost$/V$ | Power of $p$ | Margin |
|---|---|---|---|
| Prefix moves | $dK$ | $\epsilon(1+c)$ | $g_1=1-\epsilon(1+c)$ |
| Chunk exchanges | $pK^{\tau-1}$ | $1-\epsilon c(1-\tau)$ | $g_2=\epsilon c(1-\tau)$ |
| Layer rounds | $\ell d^{\lambda'}$ | $1-\epsilon(1-\lambda')$ | $g_3=\epsilon(1-\lambda')$ |
| Axis layouts | $d^2(1+\ell^\tau)$ | $\tau+\epsilon(2-\tau)$ | $g_4=1-\tau-\epsilon(2-\tau)$ |
| Gaussian lines | $dp^{1/2+\delta}\alpha$ | $3/4+\delta+3\epsilon/2$ | $g_5=1/4-\delta-3\epsilon/2$ |
| Chirps | $dp^\delta$ | $\epsilon+\delta$ | $g_6=1-\delta-\epsilon$ |
| Products | $\log(rp)$ | $1-\epsilon$ | $g_7=\epsilon$ |

- Prefix moves and single butterfly rounds are $O(dK)$ linear scans, each $O(V)$, in [Tape layout for synthetic transforms](transform-layout.md).
- The layout needs $O(p/K)$ chunk exchanges, as $d\ell=O(p)$, each costing $O(VK^\tau)$ by [Fast address-chunk interchange](address-interchange.md). The total is $O(p/K)K^\tau=O(pK^{\tau-1})$.
- Each of at most $\ell$ rounds costs $O(Vd^{\lambda'})$ by [Simultaneous butterfly layer](simultaneous-layer.md).
- The [Chinese-remainder address map](crt-map.md) needs $O(d^2)$ adjacent axis moves. Each move peels one excess bit, exchanges the equal-width parts and restores it, costing $O(V(1+\ell^\tau))$ <cite data-source="paper">Each costs $O(V(1+\ell^\tau))$: peel a single excess bit when widths</cite>. Here $d^2\ell^\tau=p^{2\epsilon}p^{(1-\epsilon)\tau}$.
- In [Gaussian resampling](gaussian-resampling.md), axis $i$ has at most $T/t_i$ lines of cost $O(t_ip^{3/2+\delta}\alpha)$ each, totalling $O(dTp^{3/2+\delta}\alpha)$.
- A chirp numerator of the [Bluestein chirp identity](chirp.md) has $O(p)$ bits and sums $d$ terms. The [Established O(n log n) multiplier](hvdh-multiplier.md), at cost $O(N\log 2N)$ on $N$ bits, computes their squares and products in $O(dp^{1+\delta})$ per entry.
- The $T/r$ ring-product records each need four products on $O(rp)$ bits after [Signed packing](signed-packing.md) by the established multiplier at length $o(n)$. The total is $O(Tp\log(rp))$, and $\log(rp)=O(p^{1-\epsilon})$ since $r\le 2^{O(p/d)}$.

Substitution gives $g_1>1/2$, $g_2=2^{-75}2^{-56}2^{-50}=2^{-181}$, $g_3=2^{-75}2^{-54}=2^{-129}$, $g_4>2^{-51}$, $g_5>1/8$, $g_6>1/2$ and $g_7=2^{-75}$, so the smallest margin is $2^{-181}=2\kappa$ <cite data-source="paper">Thus $\min_i g_i=2^{-181}=2\kappa$.</cite>

Setup must also be negligible. A visit is a node of the butterfly-layer or address-layout recursions, or one round or group invocation inside them. They have constant branching, depth $O(\log p)$ and polynomially many rounds and groups. Fix an exponent $A$ with at most $p^A$ visits in total, and an exponent $C$ with descriptor setup of at most $p^C$ steps per visit (here $A$ and $C$ are exponents, not the matrices or spaces of earlier pages). The setup costs at most $p^{A+C}=o(Tp)$ Per-record and per-line setup is also small. A visit touches $O(T/r)$ records, so $O(p^C)$ address work per record sums to $O(Tp^{A+C}/r)$; at most $2dT/r$ Gaussian lines per tensor map cost $O(dTp^C/r)$. Both are $o(Tp)$ because $r$ exceeds every fixed power of $p$.

Exceptional addresses, where the packed shifts of [Packed selected-bit addition](packed-bit-addition.md) fail, are bounded to a fraction $O(d2^{-K})$ of addresses. Let $V_{\rm call}$ be the volume of one packed change of basis. A record has $\Theta(rp)$ payload bits and an exceptional one adds an $O(p)$-bit key, so the compact stream of exceptional records has length $O\left(\frac{V_{\rm call}}{rp}d2^{-K}(rp+p)\right)=O(V_{\rm call}d2^{-K})$. Stable bit sorting makes $O(p)$ scans of it, costing $O(V_{\rm call}dp2^{-K})$. Over at most $p^A$ visits of volume $O(V)$, and with $d=O(p^\epsilon)$, the repair costs $O(Vp^{O(1)}2^{-K})=o(V)$, because $K=\Theta(p^{\epsilon c})$ beats every power of $p$. Global setup and the prime search for the axis lengths also cost $o(V)$.

Each of the seven powers is at most $1-2\kappa$, and any fixed power of $\log p$ is at most $p^\kappa$ eventually, which the margin $2\kappa$ leaves room for. The total is $O(Tp\,p^{1-\kappa})=O(n(\log n)^{1-\kappa})$ <cite data-source="paper">O(Tp\,p^{1-\kappa})=O(n(\log n)^{1-\kappa}).</cite> Schoolbook multiplication below a fixed cutoff is absorbed <cite data-source="paper">many smaller lengths are included by enlarging the implicit constant.</cite> One machine thus runs in $O(n(\lg n)^{1-\kappa})$ for every $n\ge1$.

The exponent is only $2^{-182}$ below one because $g_2$ multiplies three small numbers: $\epsilon$ (few axes), $c$ (narrow chunks) and $1-\tau$ (an exchange saves only $K^{-(1-\tau)}$).

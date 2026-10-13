The total time bound and the power saving is the step that sums the stage costs of [The multiplication algorithm](multiplication-algorithm.md) into the main result's running time. Here $n$ is the number of bits in each input integer, and $\kappa$ is a fixed constant, the power saving.

Here $T$ is the number of coefficients in the transform, and $p$ is the precision. The volume $V = Tp$ counts stored bits, up to a constant factor. The sizes $p = \Theta(\lg n)$ and $T \approx n/\lg n$ follow from [Parameter choices for the padded box](parameter-choice.md), so $V = \Theta(n)$.

Dividing a stage's cost by $V$ leaves a power of $p$, up to a constant factor. The table below covers the cost of [Convolution from the retained transform layout](synthetic-convolution.md). Here $\epsilon$, $c$, $\tau$, $\lambda'$ and $\delta$ are fixed exponents, with $c = 2^{-56}$, $\tau = 1-2^{-50}$, $\lambda' = 1-2^{-54}$, $\delta = \frac1{16}$ and $\epsilon = 2^{-75}$.

| Stage | Power of $p$ | Margin |
|---|---|---|
| Prefix moves | $p^{\epsilon(1+c)}$ | $g_1=1-\epsilon(1+c)$ |
| Chunk exchanges | $p^{1-\epsilon c(1-\tau)}$ | $g_2=\epsilon c(1-\tau)$ |
| Simultaneous layers | $p^{1-\epsilon(1-\lambda')}$ | $g_3=\epsilon(1-\lambda')$ |
| CRT and axes | $p^{\tau+\epsilon(2-\tau)}$ | $g_4=1-\tau-\epsilon(2-\tau)$ |
| Gaussian lines | $p^{3/4+\delta+3\epsilon/2}$ | $g_5=1/4-\delta-3\epsilon/2$ |
| Chirps and products | $p^{\epsilon+\delta}$ | $g_6=1-\delta-\epsilon$ |
| Packed products | $p^{1-\epsilon}$ | $g_7=\epsilon$ |

Each row's margin $g_i$ is the amount its exponent lies below one, so its power is $p^{1-g_i}$. The total is $V$ times a sum of seven powers of $p$, so the bound reduces to comparing exponents.

Each row is an earlier cost bound, divided by $V$. The chunk-exchange row is based on [Fast address-chunk interchange](address-chunk-interchange.md). The simultaneous butterfly row is bounded by [Simultaneous normalized butterfly layer](simultaneous-butterfly-layer.md). The Gaussian row sums the line costs of [Tensor resampling on a padded box](tensor-resampling.md).

Direct substitution gives $g_1>\frac12$, $g_3 = 2^{-129}$, $g_4>2^{-51}$, $g_5>\frac18$, $g_6>\frac12$ and $g_7 = 2^{-75}$, while the chunk-exchange margin is $g_2 = 2^{-181}$ <cite data-source="paper">g_2=2^{-181}</cite>, so $\min_i g_i = 2^{-181}$.

The constant $\kappa$ is half this smallest margin, so $\kappa = 2^{-182}$ <cite data-source="paper">\kappa=2^{-182}</cite>. This is the largest saving the rows allow: for $p > 1$, row $i$ fits $p^{1-2\kappa}$ exactly when $g_i \ge 2\kappa$, and the chunk-exchange row has the smallest margin.

Each of the seven powers is at most $p^{1-2\kappa}$, and any fixed power of $\lg p$ is at most $p^{\kappa}$ for large $p$, so the total is $O(Vp^{1-\kappa})$. With $V = \Theta(n)$ and $p = \Theta(\lg n)$, this is $O(n(\lg n)^{1-\kappa})$.

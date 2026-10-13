The total time bound adds the costs of all stages and divides the sum by one common volume, so each stage costs a power of one parameter times that volume. Comparing the exponents of those powers is the power saving, and it gives the main theorem, [Integer multiplication in O(n (lg n)^(1−κ)) time](main-theorem.md).

Summing the stage costs bounds the cost of [The multiplication algorithm](multiplication-algorithm.md) on $n$-bit inputs. The stage sizes are set by [Parameter choices for the padded box](parameter-choice.md): $p=6b$ with $b=\lceil\log_2 n\rceil$, so $p=\Theta(\log n)$, $d=\Theta(p^\epsilon)$ axes, and chunk width $K=\Theta(p^{\epsilon c})$. Each cost is divided by $V=Tp$, the bit volume of the padded arrays, where $T\approx n/b$ is the transform size, so $V=\Theta(n)$.

The seven dominant stages are prefix moves, chunk exchanges, simultaneous butterfly rounds, axis layouts, Gaussian line maps, chirps and packed products. Write $g_i$ for the margin below one of the $i$th stage's power, so that the power is $p^{1-g_i}$:

$$g_1=1-\epsilon(1+c),\quad g_2=\epsilon c(1-\tau),\quad g_3=\epsilon(1-\lambda'),\quad g_4=1-\tau-\epsilon(2-\tau),$$
$$g_5=\tfrac14-\delta-\tfrac32\epsilon,\quad g_6=1-\delta-\epsilon,\quad g_7=\epsilon.$$

Here $\epsilon=2^{-75}$, $c=2^{-56}$, $\tau=1-2^{-50}$, $\lambda'=1-2^{-54}$ and $\delta=1/16$ are the fixed constants of the cited pages.

The chunk-exchange row follows from [Fast address-chunk interchange](address-chunk-interchange.md), which gives $O(VK^\tau)$ for each exchange of width-$K$ chunks. There are $O(p/K)$ exchanges, so the normalized cost is $pK^{\tau-1}=p^{1-g_2}$.

The simultaneous butterfly rounds are based on [Simultaneous normalized butterfly layer](simultaneous-butterfly-layer.md), each costing $O(Vd^{\lambda'})$. The Gaussian row is built from the line passes of [Tensor resampling on a padded box](tensor-resampling.md) along the $d$ axes. The bound accounts for the stages of [Convolution from the retained transform layout](synthetic-convolution.md): two forward transforms, pointwise products and the opposite transform.

Substituting the values gives $g_1,g_6>\tfrac12$, $g_5>\tfrac18$, $g_4>2^{-51}$, $g_3=2^{-129}$, $g_7=2^{-75}$ and $g_2=2^{-181}$, so the smallest margin is $g_2$. <cite data-source="paper">Thus $\min_i g_i=2^{-181}=2\kappa$.</cite> The constant $\kappa=2^{-182}$ is therefore half the smallest margin, the largest value the argument allows. Hence each dominant power is at most $p^{1-2\kappa}$. <cite data-source="paper">Each of the seven displayed powers is at most $1-2\kappa$ by</cite>

The margin is $2\kappa$ rather than $\kappa$ because the logarithmic factors need room: any fixed power of $\log p$ is at most $p^\kappa$ for large $p$, so it adds at most $\kappa$ to the exponent, and $1-2\kappa+\kappa=1-\kappa$. Since $V=\Theta(n)$ and $p=\Theta(\log n)$, the total time is $O(Vp^{1-\kappa})=O(n(\lg n)^{1-\kappa})$. <cite data-source="paper">O(Tp\,p^{1-\kappa})=O(n(\log n)^{1-\kappa}).</cite>

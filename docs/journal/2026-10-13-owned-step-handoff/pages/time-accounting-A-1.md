The total time bound and the power saving turn the stage costs into the running-time claim for the target. Here $n$ is the bit length of each input, $T$ the transform size and $p$ the working precision in bits. Summing them bounds the running time of [The multiplication algorithm](multiplication-algorithm.md) on $n$-bit inputs.

Each stage is measured after division by the volume $V=Tp$, the number of stored bits up to a constant. Each stage then costs a power of $p$. Each power of $p$ below is set by the sizes in [Parameter choices for the padded box](parameter-choice.md). Here $d$ is the number of axes, $K$ the chunk width in bits and $\ell=\log_2 r$ the number of address bits in a record. The fixed exponents are $\epsilon$ and $c$ in these sizes, $\tau$ and $\lambda'$ in the chunk and butterfly costs, and $\delta$ in the Gaussian costs. The Gaussian maps use an integer parameter $\alpha=\Theta(p^{1/4+\epsilon/2})$.

Each row is the cost proved earlier for its stage, divided by $V$:

- prefix moves and butterfly rounds: $dK=p^{\epsilon(1+c)}$;
- chunk exchanges: $pK^{\tau-1}=p^{1-\epsilon c(1-\tau)}$;
- simultaneous butterfly rounds: $\ell d^{\lambda'}=p^{1-\epsilon(1-\lambda')}$;
- axis layouts: $d^2(1+\ell^\tau)=p^{\tau+\epsilon(2-\tau)}$;
- Gaussian maps: $dp^{1/2+\delta}\alpha=p^{3/4+\delta+3\epsilon/2}$;
- chirps and scalar products: $dp^\delta=p^{\epsilon+\delta}$;
- packed products: $\log(rp)=O(\ell+\log p)=O(p^{1-\epsilon})$.

The chunk row rests on the cost of [Fast address-chunk interchange](address-chunk-interchange.md), with $O(VK^\tau)$ per exchange. The simultaneous row is based on the cost of [Simultaneous normalized butterfly layer](simultaneous-butterfly-layer.md), which costs $O(Vd^{\lambda'})$ per round. The Gaussian row follows from the line counts of [Tensor resampling on a padded box](tensor-resampling.md). Together the rows bound the cost of [Convolution from the retained transform layout](synthetic-convolution.md).

The seven largest exponents have margins $g_1,\dots,g_7$ below one, so row $i$ costs $p^{1-g_i}$:
$$
g_1=1-\epsilon(1+c),\quad g_2=\epsilon c(1-\tau),\quad g_3=\epsilon(1-\lambda'),\quad g_4=1-\tau-\epsilon(2-\tau),\quad g_5=\tfrac14-\delta-\tfrac32\epsilon,\quad g_6=1-\delta-\epsilon,\quad g_7=\epsilon.
$$
With the fixed choices $\epsilon=2^{-75}$, $c=2^{-56}$, $\tau=1-2^{-50}$, $\lambda'=1-2^{-54}$ and $\delta=1/16$, we get $g_2=2^{-181}$, $g_3=2^{-129}$ and $g_7=2^{-75}$, while the others exceed $2^{-51}$. The exponent saving $\kappa=2^{-182}$ is half the smallest margin. Thus <cite data-source="paper">$\min_i g_i=2^{-181}=2\kappa$</cite>.

Each row costs at most $p^{1-2\kappa}$ exactly when its margin is at least $2\kappa$, so $\kappa$ is the largest saving all seven allow. The remaining $\kappa$ is spare for logarithms, giving total exponent $(1-2\kappa)+\kappa=1-\kappa$.

The other rows have exponents at most $\delta<1-2\kappa$. Any fixed power of $\log p$ is at most $p^\kappa$ for large $p$, so the cost is $O(Tp\,p^{1-\kappa})$. Since $Tp=\Theta(n)$ and $p=\Theta(\lg n)$, this is $O(n(\lg n)^{1-\kappa})$.

The phase twist is a diagonal change of basis that turns the last cyclic axis of a complex array into the coordinate of a negacyclic polynomial ring, so that cyclic convolution can be computed by ring convolution, and that is undone afterwards.

Let $f$ and $g$ be two complex arrays with $d$ axes, the last of length $r$, and $T$ entries in all. Convolution along the last axis is multiplication in $\mathbb C[x_d]/(x_d^r-1)$, where $x_d$ is the variable of that axis. The twist converts the last cyclic axis into the coordinate of the [Polynomial ring modulo y^r + 1](ring-y.md), written $\mathcal R_r$, where $y^r=-1$ instead.

## The map

Let $\zeta=e^{\pi i/r}$, so $\zeta^r=-1$. Define the ring map $\Theta$ from $\mathbb C[x_d]/(x_d^r-1)$ to $\mathcal R_r$ by $x_d\mapsto\zeta y$. It is well defined because $(\zeta y)^r=(-1)(-1)=1$.

Here $k$ and $l$ index coefficients, not operations. Since $\Theta(x_d^k)=\zeta^k y^k$, $\Theta$ is diagonal on the bases $(x_d^k)$ and $(y^k)$, multiplying coefficient $k$ by $\zeta^k\ne0$. It is therefore an isomorphism, with inverse multiplying by $\zeta^{-k}$. Both maps preserve the coefficient sup norm, since $|\zeta^{\pm k}|=1$ <cite data-source="paper">Both exact maps preserve the coefficient sup norm.</cite>

## The wrap

Take $k,l<r$ and write $k+l=j+hr$, with $j<r$ the resulting position and $h\in\{0,1\}$ the wrap indicator. After twisting, the term carries $\zeta^{k+l}$, and negacyclic reduction contributes $(-1)^h$, so the factor is
$$\zeta^{k+l}(-1)^h=\zeta^j(\zeta^r)^h(-1)^h=\zeta^j.$$
The coefficient of $y^j$ is thus $\zeta^j$ times the cyclic coefficient; untwisting by $\zeta^{-j}$ restores it with the correct sign. The twist reproduces [Cyclic convolution](convolution.md) on that axis.

For $r=2$, $\zeta=i$. Twisting $(f_0,f_1)$ and $(g_0,g_1)$ gives $(f_0,if_1)$ and $(g_0,ig_1)$. Their product has $f_0g_0+f_1g_1$ and $i(f_0g_1+f_1g_0)$. Multiplying the second by $\zeta^{-1}=-i$ gives the cyclic result.

## Whole arrays

Then $M=T/r$ is the number of lines along the last axis, and $\iota$ reads each line $(f_{\boldsymbol j,k})_k$ as the polynomial $\sum_k f_{\boldsymbol j,k}y^k$, giving an array of $M$ records in $\mathcal R_r$. Let $(D_\pm f)_{\boldsymbol j,k}=\zeta^{\pm k}f_{\boldsymbol j,k}$ multiply entry $k$ of each line by $\zeta^{\pm k}$. Then $\iota D_+$ applies $\Theta$ to every line, with inverse $D_-\iota^{-1}$.

Let $\mathcal M_r$ be the ring convolution of [Convolution from the retained transform layout](ring-convolution.md): it convolves the first $d-1$ axes over $\mathcal R_r$, dividing by $T$. The composite $\mathcal M_{\mathbb C}$, the normalized complex cyclic convolution, is
$$\mathcal M_{\mathbb C}(f,g)=D_-\iota^{-1}\mathcal M_r(\iota D_+f,\iota D_+g)=\frac{f*g}{T}.$$
Here $*$ is cyclic convolution over all $d$ axes.

## Contraction

Write $\|f\|$ for the sup norm of an array $f$: the largest modulus of a scalar coefficient. A bilinear map is a bilinear contraction if its value has norm at most the product of the norms of its arguments ([Contractions and bilinear contractions](contraction-maps.md)). $\mathcal M_r$ is one: a fixed output coefficient averages $rM$ signed scalar products of coefficients, each at most $\|f\|\|g\|$ in modulus, so $\|\mathcal M_r(f,g)\|\le\|f\|\|g\|$ <cite data-source="paper">$rM$ signed scalar products, and $\mathcal M_r$ is a bilinear</cite>.

The maps $\iota^{\pm1}$ only regroup scalars, and $D_\pm$ multiplies by numbers of modulus $1$, so all are linear and keep the sup norm. Hence
$$\|\mathcal M_{\mathbb C}(f,g)\|=\|\mathcal M_r(\iota D_+f,\iota D_+g)\|\le\|\iota D_+f\|\,\|\iota D_+g\|=\|f\|\,\|g\|,$$
so $\mathcal M_{\mathbb C}$, a bilinear map composed with linear ones, is a bilinear contraction.

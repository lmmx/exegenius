Cyclic convolution is the product of two sequences whose indices wrap around; the multiplication algorithm reduces integer multiplication to it, because it is the operation that Fourier-type transforms turn into pointwise multiplication (see [Discrete Fourier transform](dft.md)).

For two sequences $f,g$ indexed by $\mathbb Z/q\mathbb Z$, where $q$ is the number of entries in each sequence, the cyclic convolution is
$$(f*g)_k=\sum_{j\in\mathbb Z/q\mathbb Z} f_j\,g_{k-j},$$
with $k-j$ taken modulo $q$ (here $j,k$ are sequence indices, not the depth or excess degree of earlier pages). Equivalently, $f*g$ is the product of the polynomials $\sum_j f_jx^j$ and $\sum_j g_jx^j$ reduced modulo $x^q-1$: since $x^q=1$, a term $x^m$ with $m\ge q$ lands on index $m-q$.

Take $q=3$, $f=1+2x+3x^2$ and $g=1+x$. The ordinary product is $1+3x+5x^2+3x^3$, and the wrapped term $3x^3$ becomes $3$, giving $4+3x+5x^2$. The formula agrees: $(f*g)_0=f_0g_0+f_1g_2+f_2g_1=1+0+3=4$. If the ordinary product has degree below $q$, nothing wraps and $f*g$ is the ordinary product.

## Arrays over the negacyclic ring

The algorithm uses arrays with $d-1$ cyclic axes, so $d$ counts the cyclic axes plus the coefficient axis, and $t_i$ is the length of cyclic axis $i$. An array $f$ assigns to each address in $\prod_{i<d}\mathbb Z/t_i\mathbb Z$ one entry of the ring of [Polynomial ring modulo y^r + 1](ring-y.md). The convolution $f*g$ wraps each cyclic axis independently and multiplies entries by the ring product, where wrapping past $y^r$ flips the sign.

Let $M=\prod_{i<d}t_i$ be the number of cyclic addresses, and $T=rM$ the total number of complex coefficients in an array. The normalized convolution is
$$\mathcal M_r(f,g)=\frac{f*g}{rM}=\frac{f*g}{T}.$$

Fix one output address and one output coefficient. Each of the $M$ addresses of $f$ determines the address of $g$ it pairs with, and each of the $r$ coefficient indices of $f$ determines the index of $g$ modulo $r$, with a sign $\pm1$ from the wrap. So the output coefficient is the average of $rM$ signed products of one coefficient of $f$ and one of $g$ <cite data-source="paper">Thus that coefficient averages $rM$ signed scalar products, and $\mathcal M_r$ is a bilinear contraction</cite>.

Write $\|\cdot\|_\infty$ for the largest modulus of a coefficient. Each product has modulus at most $\|f\|_\infty\|g\|_\infty$, hence so does the average:
$$\|\mathcal M_r(f,g)\|_\infty\le\|f\|_\infty\|g\|_\infty.$$

## What the bound does and does not give

The bound limits size, not precision. If all coefficients of $f$ and $g$ have modulus at most $1$ (a disk array), so do those of $\mathcal M_r(f,g)$: dividing by $T$ cancels the growth of a sum of $T$ products. That is the reason for the normalization.

It does not make the computed result exact. Stored values lie on a grid of $p$ fractional bits, so the computed output only approximates $\mathcal M_r(f,g)$, and the procedure that computes it returns its components at a width larger than the input's by $\log_2 M$ bits <cite data-source="paper">Put $m_M=\log_2M$ and $w_M=w+m_M$.</cite>

The bound does control how errors propagate. The map $\mathcal M_r$ is linear in each argument, so if $f'$ is $f$ with rounding errors, then $\mathcal M_r(f,g)-\mathcal M_r(f',g)=\mathcal M_r(f-f',g)$, of size at most $\|f-f'\|_\infty\|g\|_\infty$. When $g$ is a disk array this is at most $\|f-f'\|_\infty$: replacing one operand changes the output by at most the norm distance between its two values <cite data-source="paper">If the other operand is a disk array, replacing one operand changes the output by at most the norm distance between its two values.</cite>

A synthetic polynomial transform is a transform over the [Polynomial ring modulo y^r + 1](ring-y.md) whose roots of unity are powers of $y$, so every twiddle multiplication (multiplication by a root of unity) is a signed shift of a coefficient list. It lets the other axes of a multidimensional transform be handled without complex multiplications.

In that ring $y^r=-1$, so multiplying a record by $y^s$ moves each coefficient up $s$ places and negates those that wrap past degree $r-1$. Powers of $y$ can therefore serve as roots of unity <cite data-source="paper">Powers of $y$ then serve as roots of unity in the remaining axes, and multiplication by such a root is a signed shift of the coefficient list.</cite>.

For example, take $r=4$: multiplying $a_0+a_1y+a_2y^2+a_3y^3$ by $y^2$ gives $-a_2-a_3y+a_0y^2+a_1y^3$.

Let $t$ be the transform length, the number of ring elements along one axis, a power of two dividing $2r$. Define $\omega_t=y^{2r/t}$ <cite data-source="paper">For a power of two $t$ dividing $2r$, define $\omega_t=y^{2r/t}$.</cite>. Since $y^{2r}=1$ and $y^r=-1$, we get $\omega_t^t=1$ and $\omega_t^{t/2}=-1$.

The synthetic transforms are given by the formulas of the [Discrete Fourier transform](dft.md), with $\omega_t$ in place of the complex root of unity:

$$(F_t^-z)_j=\frac1t\sum_{k=0}^{t-1}\omega_t^{-jk}z_k,\qquad (F_t^+z)_j=\frac1t\sum_{k=0}^{t-1}\omega_t^{jk}z_k .$$

Here $z=(z_0,\dots,z_{t-1})$ is a list of $t$ ring elements and $0\le j<t$.

## Both transforms are contractions

The size of a ring element $\sum_k a_k y^k$ is $\max_k|a_k|$; the size of a list is the largest size of its entries. A signed shift only permutes coefficients and flips signs, so multiplying by a power of $\omega_t$ leaves size unchanged. An entry of $F_t^\pm z$ is $\frac1t$ times a sum of $t$ shifted entries of $z$; by the triangle inequality on each coefficient, it has size at most that of $z$. A map that never increases size is a contraction, so both transforms are contractions <cite data-source="paper">Every power of $\omega_t$ is an isometry, and each transform entry is an average of $t$ such images.</cite>.

## Undoing a transform

Over the complex numbers one cancels sums of roots of unity by dividing by $\omega_t^j-1$. The ring has zero divisors, so the cancellation is verified directly <cite data-source="paper">The ring has zero divisors, so we verify character cancellation directly instead of dividing by $\omega_t^j-1$.</cite>. For an integer $j$,
$$\sum_{k=0}^{t-1}\omega_t^{jk}=\begin{cases}t,&t\mid j,\\0,&t\nmid j.\end{cases}$$
If $t\nmid j$, write $j$ as $2^e$ times an odd number, with $0\le e<\log_2 t$. Replacing $k$ by $k+t/2^{e+1}$ modulo $t$ permutes the summands and multiplies each by $\omega_t^{jt/2^{e+1}}=y^{r\cdot\text{odd}}=-1$. The sum equals its own negative, so it is $0$.

Expanding $F_t^+F_t^-$, the coefficient of $z_k$ in entry $i$ is $t^{-2}\sum_j\omega_t^{j(i-k)}$, which is $t^{-1}$ if $i=k$ and $0$ otherwise. Hence $F_t^+F_t^-=t^{-1}I$: applying $F_t^+$ undoes $F_t^-$ up to the factor $1/t$.

As in the complex case, $F_t^-$ is computed by [Butterflies and the fast transform](butterfly.md) in $\log_2 t$ layers, each twiddle multiplication being a signed shift.

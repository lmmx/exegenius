The negacyclic ring is a way of storing a list of complex numbers so that multiplying by a root of unity costs only a signed rotation of the list, with no arithmetic; this is what lets the later transforms avoid complex multiplications.

Write $\mathcal R_r=\mathbb C[y]/(y^r+1)$ for it, where $y$ is the ring variable, the indeterminate of the polynomials, and $r$ is the record length, the number of complex coefficients in a polynomial of the ring, always a power of two with $r\geq 2$. An element is
$$z=\sum_{k=0}^{r-1}z_k y^k,\qquad z_k\in\mathbb C,$$
so it is just a record of $r$ complex numbers. Addition is coefficientwise. Multiplication is ordinary polynomial multiplication followed by reduction modulo $y^r+1$, which means imposing $y^r=-1$. Consider a term of degree $r+j$ with $j\geq 0$, so that $j$ is the amount by which its degree exceeds $r$. Then
$$y^{r+j}=y^{r}y^{j}=-y^{j},$$
so the term, with its complex coefficient unchanged apart from the sign, lands at position $j$ with its sign flipped: it wraps around to the front. A product of two records has degree at most $2r-2$, so one such wrap suffices. For this sign flip the ring is also called the negacyclic ring. We always keep the representative of degree below $r$ <cite data-source="paper">We always use representatives of degree less than $r$.</cite>

## Powers of $y$ are signed shifts

Multiplying by $y$ moves each coefficient up one place and sends the top coefficient $z_{r-1}$ to the bottom as $-z_{r-1}$. For $r=4$:
$$y\,(z_0+z_1y+z_2y^2+z_3y^3)=-z_3+z_0y+z_1y^2+z_2y^3 .$$
Since $y^r=-1$, we get $y^{2r}=1$, so $y$ is a $2r$-th root of unity. Let $a$ be any integer exponent. Multiplying by $y^a$ applies the one-place move $a$ times (for negative $a$, the inverse move). It sends coefficient $z_k$ to position $(k+a)\bmod r$, with its sign multiplied by $(-1)^{\lfloor (k+a)/r\rfloor}$, one flip for each time it wraps past the end. It permutes the coefficients and changes some signs, so it is a linear-time pass over the list.

The size of an element is measured by the largest coefficient modulus,
$$\|z\|_\infty=\max_{0\leq k<r}|z_k|.$$
A signed permutation of coefficients leaves this maximum unchanged, so multiplication by $y^a$ is an isometry for this norm <cite data-source="paper">some signs; consequently it is an isometry for this norm.</cite> Shifts therefore add no rounding error when coefficients are stored at fixed precision.

## Why this ring

Taking one coordinate of a multidimensional array to be a record in $\mathcal R_r$ makes powers of $y$ available as roots of unity in the remaining axes, where a multiplication by such a root is a signed shift of the coefficient list <cite data-source="paper">multiplication by such a root is a signed shift of the coefficient list</cite>. Transforms built this way are described on [Synthetic polynomial transform](synthetic-transform.md). The ring also receives ordinary cyclic data once its coefficients are twisted by phases, as explained on [Phase twist](phase-twist.md).

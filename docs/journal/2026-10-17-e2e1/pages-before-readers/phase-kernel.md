The two-point kernel is a $2\times 2$ matrix that squares to the bit flip and differs from the butterfly step only by phase factors; it is the building block of the phase networks used later.

Let $a=\frac{1+i}{2}$ and $b=\frac{1-i}{2}$ be two complex constants (here $b$ is this coefficient, not a bit index, and $a$ is not a bank index). Let $I$ be the $2\times 2$ identity matrix, and let $X=\begin{pmatrix}0&1\\1&0\end{pmatrix}$ be the bit flip, which swaps the two entries of a pair $(u,v)$. The kernel is $C=aI+bX$. Let $\mathsf S=\operatorname{diag}(1,i)$ be the phase matrix that multiplies the second entry of a pair by $i$. Let $H_0=\frac12\begin{pmatrix}1&1\\1&-1\end{pmatrix}$ be the matrix of the averaging and differencing step $(u,v)\mapsto\bigl(\tfrac{u+v}{2},\tfrac{u-v}{2}\bigr)$, and let $H=\sqrt2\,H_0$ be the normalized Hadamard matrix, which is symmetric and satisfies $H^2=I$.

## Why it is a square root of the flip

Since $a^2+b^2=\frac{i}{2}-\frac{i}{2}=0$ and $2ab=1$, we get $C^2=(a^2+b^2)I+2ab\,X=X$, and therefore $C^4=X^2=I$. In terms of eigenvectors, $X$ has eigenvalue $1$ on $(1,1)$ and $-1$ on $(1,-1)$, while $C$ has eigenvalues $a+b=1$ and $a-b=i$ on the same vectors. Equivalently $C=H\mathsf SH$ <cite data-source="paper">C=H\mathsf SH=aI+bX</cite>. Thus $C$ applies the phase $\mathsf S$ in the basis where $X$ is diagonal, and one application of $C$ is "half" a flip.

## Link to the butterfly

The kernel $C$ is obtained from the two-point step of [Butterflies and the fast transform](butterfly.md) by conjugating with $\mathsf S$ and rescaling:
$$H_0=b\,\mathsf S\,C\,\mathsf S,\qquad C^2=X$$
<cite data-source="paper">H_0=b\mathsf SC\mathsf S,\qquad C^2=X</cite>. One can check the first identity by multiplying out: $\mathsf SC\mathsf S=\begin{pmatrix}a&ib\\ib&-a\end{pmatrix}$, and multiplying by $b$ uses $ab=\frac12$ and $ib^2=\frac12$.

The reason to prefer $C$ is that it is a combination $aI+bX$ of the identity and a bit flip. When the flip acts on one address bit of a larger array, such a combination is a translation-like kernel, and the networks built from these kernels are described on the page on binary phase frames. The coefficients $a$ and $b$ are Gaussian dyadic (real and imaginary parts are dyadic rationals), so $C$ and its inverse $C^{-1}=bI+aX$ are exact on fixed-point records. The Hadamard factors in $C=H\mathsf SH$ serve to prove identities and need not be executed by the machine.

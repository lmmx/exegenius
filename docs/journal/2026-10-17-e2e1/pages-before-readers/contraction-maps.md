A contraction is a linear map that cannot increase the size of a vector, and a bilinear contraction is its two-argument analogue; they let the error analysis show that errors in the inputs of a transform or product do not grow in its output.

Size means the sup norm $\|u\|$, the largest modulus among the entries of $u$. A linear map $L$ is a *contraction* if $\|Lu\|\le\|u\|$ for every $u$. A map $\mathcal M(f,g)$ that is linear in each operand $f$ and $g$ separately is a *bilinear contraction* if $\|\mathcal M(f,g)\|\le\|f\|\,\|g\|$.

## Why errors stay controlled

Suppose a computed input $u'$ replaces the exact input $u$. Linearity gives $Lu'-Lu=L(u'-u)$, so the output error is at most the input error <cite data-source="paper">A linear contraction propagates an input error without increasing it.</cite>

For a bilinear contraction, replace operands $f,g$ by $f',g'$. Adding and subtracting $\mathcal M(f,g')$ gives
$$\mathcal M(f',g')-\mathcal M(f,g)=\mathcal M(f'-f,g')+\mathcal M(f,g'-g),$$
so the output error is at most $\|f'-f\|\,\|g'\|+\|f\|\,\|g'-g\|$. If all four operands have norm at most $1$ (every entry in the unit disk), the two input errors simply add <cite data-source="paper">When all four operands are in the disk, their two errors therefore add.</cite>

Errors of successive steps also add: insert the exact map at the computed input and apply its contraction bound to the accumulated input error <cite data-source="paper">Composing such algorithms adds their errors</cite>. Only the exact map must be a contraction; the computed approximation may be nonlinear.

## Where it holds

Contraction applies to the normalized [Discrete Fourier transform](dft.md),
$$(F_q u)_k=\frac1q\sum_{j=0}^{q-1}u_j e^{-2\pi i jk/q},$$
where $F_q$ is the transform on sequences of $q$ entries. Each output entry averages inputs multiplied by complex numbers of modulus one, so by the triangle inequality its modulus is at most the largest input modulus <cite data-source="paper">Each normalized Fourier transform above is a contraction, since each output averages inputs multiplied by complex numbers of modulus one.</cite> For $q=2$ the outputs are $(u_0+u_1)/2$ and $(u_0-u_1)/2$, both of modulus at most $\max(|u_0|,|u_1|)$. The factor $1/q$ is what makes this work: the unnormalized sum could grow by a factor $q$.

Bilinear contraction applies to the normalized [Cyclic convolution](convolution.md) $f*g/T$, where $T$ counts all entries involved. A fixed output coefficient averages $T$ signed products of one entry of $f$ with one entry of $g$ <cite data-source="paper">Thus that coefficient averages $rM$ signed scalar products, and $\mathcal M_r$ is a bilinear contraction</cite>, each of modulus at most $\|f\|\,\|g\|$, hence $\|f*g/T\|\le\|f\|\,\|g\|$.

Coordinate permutations and multiplication by unit-modulus phases leave the norm unchanged, so they are contractions too. A composite of such maps, such as twist, convolution and untwist, is again a contraction or bilinear contraction

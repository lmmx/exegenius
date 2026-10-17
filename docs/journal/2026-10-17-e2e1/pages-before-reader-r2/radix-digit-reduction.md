Splitting each input into digits in a large power-of-two radix turns the integer product into a convolution of short integer sequences, and the sizes chosen here make a [Cyclic convolution](convolution.md) agree exactly with the ordinary product.

Call the two inputs $u$ and $u'$, each with $n$ bits. Let $b=\lceil\log_2 n\rceil$ be the number of bits in one digit, so $n\le 2^b$. Two lengths come from [parameter and prime-length choice](parameter-choice.md). $T$ is a power of two with $4n/b\le T<8n/b$, the size of the padded array on which the later power-of-two transforms act. $S$ is the length of the cyclic convolution, with $T/2<S<T$. It is a product of distinct odd primes, each slightly below a power of two, because the later [Chinese-remainder address map](crt-map.md) splits a length-$S$ convolution into prime-length axes only if the lengths are coprime, and [Gaussian resampling](gaussian-resampling.md) then moves each prime length to a nearby power of two. This page needs only $T/2<S$.

Let $q=\lceil n/b\rceil$ be the number of digits in each input, padding with leading zeros. Write $u=\sum_{i=0}^{q-1}u_i2^{bi}$ and $u'=\sum_{k=0}^{q-1}u'_k2^{bk}$, where the digits $u_i$ and $u'_k$ are integers with $0\le u_i,u'_k<2^b$. Let $x$ be a polynomial variable, and form the polynomials in $x$ with the $u_i$ and with the $u'_k$ as coefficients. Evaluating them at $x=2^b$ returns $u$ and $u'$, so $uu'$ is the value at $2^b$ of their product. Let $c_j=\sum_{i+k=j}u_iu'_k$ be the coefficient of $x^j$ in that product. The $c_j$ can exceed $2^b$, so the last step is to evaluate at $2^b$ with carries, which gives the binary number.

## No wraparound

Each digit polynomial has degree at most $q-1$, so the product polynomial has degree at most $2q-2$. Since a ceiling is less than its argument plus 1, $q<n/b+1$ and so $2q-2<2n/b$. Also $2n/b\le T/2$ because $T\ge 4n/b$, and $T/2<S$. Hence $2q-2<2n/b\le T/2<S$ <cite data-source="paper">2q-2<2n/b\le T/2<S.</cite>

The cyclic convolution of length $S$ is the product modulo $x^S-1$, where $x^S=1$ folds the term $x^j$ onto $x^{j \bmod S}$. Every exponent $j$ is at most $2q-2<S$, so nothing folds. The cyclic convolution of the two zero-padded digit sequences therefore equals $c_0,\dots,c_{S-1}$, with zeros after $c_{2q-2}$. For example, with $q=3$ the product has degree 4. Length $S=5$ is safe, but $S=4$ would fold $c_4$ onto $c_0$.

## Size of the coefficients

Each $c_j$ is a sum of at most $q$ products $u_iu'_k$, each between $0$ and $(2^b-1)^2$. Hence $0\le c_j\le q(2^b-1)^2$. Since $q\le n\le 2^b$, this is below $2^b\cdot 2^{2b}=2^{3b}$ <cite data-source="paper">0\le c_j\le q(2^b-1)^2<2^{3b},</cite>.

So every coefficient is a nonnegative integer of at most $3b$ bits. Any approximation of $c_j$ with error below $1/2$ rounds to $c_j$, which is how [error tracking](precision-recovery.md) recovers exact integers after the approximate transforms.

Multiplication therefore reduces to a [Cyclic convolution](convolution.md) of length $S$ on short integer coefficients.

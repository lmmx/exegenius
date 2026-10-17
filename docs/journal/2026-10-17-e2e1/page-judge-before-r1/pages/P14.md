A butterfly is the two-point step $H_0(u,v)=(\frac{u+v}{2},\frac{u-v}{2})$ applied to a pair of values (here $u,v$ are just those two values). Here $t$ is the transform length, a power of two. Repeated over the address bits of an array, with twiddle multiplications between layers, it computes the [Discrete Fourier transform](dft.md) of an array in $\log_2 t$ layers instead of $t^2$ operations. Later transforms consist of address rearrangements, signed shifts and these steps <cite data-source="paper">two-point butterfly operations</cite>.

## One layer

Let $z_k$, for positions $0\le k<t$, be the input values, and let $\omega_t=e^{2\pi i/t}$. The transform is $(F_tz)_m=\frac1t\sum_j z_j\omega_t^{-jm}$, where $m$ is the output index. The decimation-in-frequency step pairs $z_k$ with $z_{k+t/2}$ for $0\le k<t/2$ <cite data-source="paper">the decimation-in-frequency step pairs $z_k$ with $z_{k+t/2}$</cite>; these positions differ exactly in the highest address bit. Splitting the sum over $j=k$ and $j=k+t/2$, since $\omega_t^{t/2}=-1$ the second part carries the factor $(-1)^m$. Write $m=2h+b$, where $b\in\{0,1\}$ is the lowest bit of $m$ and $h$ is the remaining index. Then
$$(F_tz)_{2h+b}=\frac1{t/2}\sum_{k=0}^{t/2-1}(\omega_t^2)^{-hk}\,\omega_t^{-bk}\,\frac{z_k+(-1)^bz_{k+t/2}}{2}.$$
The pair enters only through $\frac{z_k+z_{k+t/2}}2$ (for $b=0$) and $\frac{z_k-z_{k+t/2}}2$ (for $b=1$), the two outputs of $H_0$. Store them in the slots they came from, so the highest address bit now holds $b$. After multiplying the $b=1$ output by the twiddle factor $\omega_t^{-k}$, each branch needs the same transform of length $t/2$ with output index $h$, because $\omega_t^2$ is the primitive $(t/2)$-th root of unity.

## Recursion and output order

One layer of $t/2$ butterflies and $t/2$ twiddle multiplications reduces a length-$t$ transform to two independent length-$t/2$ transforms. After $\log_2 t$ layers every transform has length one, the identity, for $O(t\log t)$ operations in all.

Number the address bits from the highest down. Layer one leaves bit $0$ of $m$ in the highest address bit. Within each branch, layer two repeats the argument for $h$: it pairs entries differing in the next address bit and leaves bit $1$ of $m$ there. Inductively, each layer leaves the next higher bit of $m$ in the next lower address bit. The address bits from high to low therefore hold the bits of $m$ from low to high <cite data-source="paper">contain the frequency bits from low to high</cite>, so output $m$ lands at the bit-reversed position of $m$.

For $t=4$: the first layer forms $\frac{z_0\pm z_2}2$ and $\frac{z_1\pm z_3}2$ and multiplies the two differences by $1$ and $\omega_4^{-1}=-i$. The second layer applies $H_0$ to each pair. Output $m=1$ (bits $01$) sits in position $2$ (bits $10$).

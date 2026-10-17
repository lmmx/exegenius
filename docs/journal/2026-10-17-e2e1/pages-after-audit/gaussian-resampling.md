Gaussian resampling expresses a Fourier transform of length $s$ through one of a nearby larger length $t$ (a power of two there), so that only power-of-two transforms need a fast algorithm. Some earlier letters are reused here.

Let $s<t<2^p$ be coprime integers, where $p>100$ counts the fractional bits of each real and imaginary component, and let $\alpha$, the width parameter of the Gaussian weights below, be an integer with $2\le\alpha<\sqrt p$. Assume $\theta:=t/s-1>p/\alpha^4$. Here $F_q$ is the normalised [Discrete Fourier transform](dft.md) of length $q$, and the resampling expresses $F_s$ through $F_t$. Define the permutations $(P_su)_j=u_{tj}$ and $(P_tv)_k=v_{-sk}$ (coordinate indices $j$, $k$ taken modulo $s$, $t$), which stay in the identity, never executed as sorts. There are linear contractions $A$, $B_0$ with $P_sF_s=2^{2\alpha^2}B_0P_tF_tA$. Applied along each axis of a [Multidimensional transforms](multidim-transform.md), it replaces $s$ by $t$.

## Exact construction

Harvey and van der Hoeven's maps $\mathsf S,\mathsf T$ from length-$s$ to length-$t$ complex vectors are, for $0\le k<t$, with $u$ extended periodically and $j$ over all integers,
$$(\mathsf Su)_k=\frac1\alpha\sum_j e^{-\pi\alpha^{-2}(j-sk/t)^2}u_j,\qquad (\mathsf Tu)_k=\sum_j e^{-\pi\alpha^2(tj/s-k)^2}u_j.$$
They satisfy $\mathsf TP_sF_s=P_tF_t\mathsf S$ <cite data-source="paper">\mathsf T P_sF_s=P_tF_t\mathsf S,</cite>.

To undo $\mathsf T$, let $q_j$ be the integer nearest $tj/s$ and $\beta_j=tj/s-q_j$. Let $C$ select coordinates from length-$t$ to length-$s$ vectors, $(Cv)_j=v_{q_j}$, and $(\mathsf Du)_j=e^{\pi\alpha^2\beta_j^2}u_j$. Put $N=C\mathsf T\mathsf D$. In $(Nu)_j$, the summand with index $j$ is exactly $u_j$: the factor in $\mathsf D$ cancels the Gaussian factor at the selected coordinate $q_j$, because $tj/s-q_j=\beta_j$. The other summands form $((N-I)u)_j$, and the cited estimate gives $\|N-I\|<2.01e^{-\pi\alpha^2\theta/2}<2^{-\alpha^2\theta}$ <cite data-source="paper">\|N-I\|<2.01e^{-\pi\alpha^2\theta/2}<2^{-\alpha^2\theta}.</cite>.

Since $\alpha^2<p$ and $\theta>p/\alpha^4$, $\alpha^2\theta>p/\alpha^2>1$, so $\|N-I\|<1/2$ and $N$ is invertible by the Neumann series $N^{-1}=\sum_i(-1)^i(N-I)^i$.

Set $\mathsf S'=\mathsf S/2$, $\mathsf J'=N^{-1}/2$, $\mathsf D'=2^{-(2\alpha^2-2)}\mathsf D$, and put $A=\mathsf S'$ and $B_0=\mathsf D'\mathsf J'C$. These are contractions, since $\|C\|=1$ and the three factors have norms below $3/4$, $7/8$, $1$ <cite data-source="paper">\|\mathsf S'\|<\frac34,\qquad \|\mathsf J'\|<\frac78,\qquad \|\mathsf D'\|<1.</cite>.

As $C\mathsf T=N\mathsf D^{-1}$, the map $\mathsf DN^{-1}C$ is a left inverse of $\mathsf T$; applying it to the identity gives $\mathsf DN^{-1}CP_tF_t\mathsf S=P_sF_s$. The normalizations multiply to $2^{-(2\alpha^2-2)}2^{-1}2^{-1}=2^{-2\alpha^2}$, so $B_0P_tF_tA=2^{-2\alpha^2}P_sF_s$.

## Computing it

Let $\mathbb D_p$ be the grid of [Fixed-point Gaussian dyadic records](fixed-point.md). An approximation $\widetilde L$ of a contraction $L$ has error below a bound if $2^p\|\widetilde Lu-Lu\|$ is less than it for all inputs $u\in\mathbb D_p$; composing approximations adds errors. Fix $0<\delta<1/8$, the time-exponent slack. The cited algorithms give approximations $\widetilde{\mathsf S}'$, $\widetilde{\mathsf J}'$, $\widetilde{\mathsf D}'$ with errors $16p$, $3p^2/4$, $4$ and times $O(tp^{3/2+\delta}\alpha)$, $O(tp^{3/2+\delta}\alpha)$, $O(tp^{1+\delta})$ <cite data-source="paper">\widetilde{\mathsf S}'&16p&O(tp^{3/2+\delta}\alpha)\\</cite><cite data-source="paper">\widetilde{\mathsf J}'&3p^2/4&O(tp^{3/2+\delta}\alpha)\\</cite><cite data-source="paper">\widetilde{\mathsf D}'&4&O(tp^{1+\delta}).</cite>.

The selection $C$ picks an ordered subsequence (since $t/s>1$), and the indices are generated in order with $O(p)$ work each, so $C$ takes $O(tp)$ time with zero error <cite data-source="paper">also cost $O(tp)$, so $C$ takes $O(tp)$ time with zero numerical error.</cite>.

Take $\widetilde A=\widetilde{\mathsf S}'$ and $\widetilde B_0=\widetilde{\mathsf D}'\widetilde{\mathsf J}'C$. Contraction of the exact maps yields errors less than $16p<p^2$ and $4+3p^2/4<p^2$ (as $p>100$), and the costs add to $O(tp^{3/2+\delta}\alpha)$.

Let $E=N-I$ and $k=\lceil p/(\alpha^2\theta)\rceil$, the number of series terms kept (a new meaning of $k$). As $\|E\|<2^{-\alpha^2\theta}$ and $\alpha^2\theta>1$, the omitted terms sum to less than $2\cdot2^{-p}$. Let $\widetilde E$ be the approximation of $E$ on $\mathbb D_p$ from the cited Lemma 4.11: for every $z\in\mathbb D_p^s$ $\widetilde Ez\in\mathbb D_p^s$ and $\|\widetilde Ez\|<0.42\|z\|+(p/3)2^{-p}<1$ <cite data-source="paper">\|\widetilde E z\|<0.42\|z\|+(p/3)2^{-p}<1</cite>. Start from $\widetilde v^{(0)}$, the rounding of $u/2$, and set $\widetilde v^{(j)}=\widetilde E\widetilde v^{(j-1)}$. The Neumann series with $E=N-I$ and $k=\lceil p/(\alpha^2\theta)\rceil$ is evaluated as $\widetilde{\mathsf J}'u=\sum_{j=0}^{k-1}(-1)^j\widetilde v^{(j)}$ with every iterate kept in $\mathbb D_p$.

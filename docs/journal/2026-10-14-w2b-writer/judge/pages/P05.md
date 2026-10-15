Subspace labels attach nested nondegenerate subspaces to the vertices of the [Linear exchange network with restored scratch wires](linear-network.md), so that its total rank stays below $Wm$ ($W$ wires, label dimension $m$). Labels price each change of frame, from [Frames on wire arrays](common-frame-principle.md), at its residual's dimension.

Let $h=100$ be the ground-set size, $\mathcal T$ the 3-element subsets of $\{1,\dots,h\}$, $N=\binom h3^3$ the number of indices $a\in\mathcal T^3$, $I$ the invocation count ($I/N=3/\binom h3$), and $c$ the central wires per invocation ($c_{\rm b}=100$, $c_{\rm c}=101$). An edge's residual is the orthogonal complement of the smaller label in the larger, defined in [Rank, subspaces and orthogonal complements](linear-algebra-basics.md).

Bit network: label field $F=\mathbb Q^h$ with $\langle x,y\rangle=x^{\mathsf T}(\mathbf 1-J/9)y$, $J=\mathbf 1\mathbf 1^{\mathsf T}$ (eigenvalues $1,-91/9$). With $t_T$ the indicator of a triple $T$, $t_S^{\mathsf T}t_T=|S\cap T|$ and $t_S^{\mathsf T}Jt_T=9$ give $\langle t_S,t_T\rangle=|S\cap T|-1$, so each line $\langle t_T\rangle$ is nondegenerate and neighbors ($|S\cap T|=1$) are orthogonal <cite data-source="paper">\langle t_S,t_T\rangle=|S\cap T|-1,</cite>. Complex network: $F=\mathbb F_2^h$, dot product, $t_T\cdot t_T=1$; even-intersection neighbors are orthogonal. Labels live in $\mathcal F=F^{\otimes3}$ of dimension $m=h^3$, with $u_a=t_{a_1}\otimes t_{a_2}\otimes t_{a_3}$, $U_a=\langle u_a\rangle$.

Source-to-sink labels ($X_a,Y_a$ data roles):
$$X_a:\ U_a\to\mathcal F,\qquad Y_a:\ 0\to U_a^\perp,\qquad \mathrm{scratch}:\ 0\to\mathcal F.$$
Sources total $N$, sinks $Wm-N$.

At stage $j\in\{1,2,3\}$ let $A=F^{\otimes(j-1)}$, $P=\langle t_{a_1}\otimes\cdots\otimes t_{a_{j-1}}\rangle$, $B=P^\perp\subset A$, $t$ the $j$th indicator, $R$ the line of later factors:
$$X:\ A\otimes\langle t\rangle\otimes R\to(A\otimes F)\otimes R,\qquad Y:\ B\otimes\langle t\rangle\otimes R\to(P\otimes\langle t\rangle)^\perp_{A\otimes F}\otimes R.$$
With $A'=A\otimes F$, $P'=P\otimes\langle t\rangle$, $B'=(P')^\perp_{A'}$, $R=\langle t_{a_{j+1}}\rangle\otimes R'$, so outgoing equals next incoming.

Gate labels at times $0,\dots,7$ (tensored with $R$) for neighbors $S,T$ at $X,Y$:
$$B\otimes\langle t\rangle,\ B\otimes F,\ (B\otimes F)\perp(P\otimes\langle t_S\rangle),\ A\otimes F,\ B\otimes F,\ (B\otimes F)\perp(P\otimes t_T^\perp),\ A\otimes F,\ A\otimes F.$$
Orthogonal neighbors give $\langle t_S\rangle\subset t_T^\perp$, so edge $2\to5$ nests. Only central edges $3\to4$ decrease, losing $P\otimes F\otimes R$ of dimension $h$, so the total dimension lost is $L=Ich$. Signed changes telescope at each gate to $Wm-2N$; absolute values add $2L$. So $L_{\rm b}/N=100/539$ and $L_{\rm c}/N=101/539$, both below $1/2$ <cite data-source="paper">In particular, both are smaller than $1/2$.</cite>

Bit interface: assign each vertex the projection $P_U$ onto its label, and $-P_{U_a}$ at the source of $X_a$. For nested nondegenerate $U\subset V$, $P_V-P_U$ is the identity on $V\cap U^\perp$ and zero elsewhere, so has rank $\dim V-\dim U$. On the $N$ first edges of $X_a$, $2P_{U_a}$ has rank $1$, not $0$. So $s_{\rm b}=Wm-N+2L_{\rm b}<Wm$, and at the endpoints $P_{U_a^\perp}+P_{U_a}=\mathbf 1$.

Complex interface: define
$$x\in\mathbb F_2^m,\ H=\tfrac1{\sqrt2}\begin{pmatrix}1&1\\1&-1\end{pmatrix},\ Z=\operatorname{diag}(1,-1),\ \tau=\begin{pmatrix}0&1\\1&0\end{pmatrix},\ [b]\in\{0,1\},\ q_U(x)=\operatorname{wt}(P_Ux)\bmod4,$$
$$C=H\operatorname{diag}(1,i)H=\tfrac{1+i}2\mathbf 1+\tfrac{1-i}2\tau,\qquad \mathcal C_U=H^{\otimes m}\operatorname{diag}_x(i^{q_U(x)})H^{\otimes m}.$$
Orthogonal binary $z,w$ add weights mod 4: $\operatorname{wt}(z+w)=\operatorname{wt}z+\operatorname{wt}w-2|\operatorname{supp}z\cap\operatorname{supp}w|$. For $V=U\perp E$ with $\mathcal B$ an orthonormal basis of $E$ <cite data-source="paper">every nonzero orthogonal residual of an edge has an orthonormal basis</cite>, $q_V-q_U=\sum_{e\in\mathcal B}\operatorname{wt}(e)[e\cdot x]$. Each $\operatorname{wt}(e)$ is odd, so each factor is $i^{\varepsilon[e\cdot x]}=\frac{1+i^\varepsilon}2+\frac{1-i^\varepsilon}2(-1)^{e\cdot x}$, $\varepsilon=\pm1$. As $HZH=\tau$, it is one kernel $\frac{1+i^\varepsilon}2\mathbf 1+\frac{1-i^\varepsilon}2\tau_e$ per basis vector ($\tau_e$ translates by $e$), so $s_{\rm c}=Wm-2N+2L_{\rm c}<Wm$. For $X_a\to Y_a$, $q_{U_a^\perp}-q_{U_a}\equiv\operatorname{wt}(x)+2[u_a\cdot x]$ gives $C^{-1}$ on the $27$ coordinates of $\operatorname{supp}u_a$; since $C=iZC^{-1}Z$, signs $Z$ before and $i^{27}Z$ after give $C$. Undoing the bank exchange gives all roles $C^{\otimes m}$.

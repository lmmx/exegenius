Subspace labels from three-element subsets are nested nondegenerate subspaces that keep a network's total edge rank below $Wm$, where $W$ counts wires and $m$ is the label dimension. The labels are attached to the gates and terminals of a [Linear exchange network with restored scratch wires](linear-network.md).

The labels are based on [Rank, subspaces and orthogonal complements](linear-algebra-basics.md). Each label $U$ has projection $P_U,$ and for nested labels $U\subset V,$ $P_V-P_U$ is the identity on the residual $V\cap U^\perp$ and zero on its complement, so an edge costs rank $\dim V-\dim U.$

They are used as frames in [Frames on wire arrays](common-frame-principle.md): vertex $v$ carries $M_v=P_{U_v}.$

Triples $S,T$ in a ground set of $h=100$ points have indicators $t_S,t_T.$ For the bit network $F=\mathbb Q^h$ with $\langle p,g\rangle=p^{\mathsf T}(I-J/9)g,$ $J$ the all-ones matrix; for the complex network $F=\mathbb F_2^h$ with the dot product.

Since $t_S,t_T$ have three ones, $\langle t_S,t_T\rangle=|S\cap T|-1$ and $\langle t_T,t_T\rangle=2,$ so each triple line is nondegenerate.

The label space $\mathcal F=F^{\otimes3}$ has dimension $m=h^3.$ An input $a=(a_1,a_2,a_3)$ of three triples has label $U_a=\langle u_a\rangle,$ where $u_a=t_{a_1}\otimes t_{a_2}\otimes t_{a_3}.$

$X_a$ has source $U_a$ and sink $\mathcal F,$ $Y_a$ has source $0$ and sink $U_a^\perp,$ and scratch roles have source $0$ and sink $\mathcal F.$ Each $U_a$ is a line, so sources total $N,$ one per input, and sinks $Wm-N.$

At stage $j$ let $A=F^{\otimes(j-1)},$ $R=\langle t_{a_1}\otimes\cdots\otimes t_{a_{j-1}}\rangle$ and $B=R^\perp$ in $A.$ The next stage has $A'=A\otimes F,$ $R'=R\otimes\langle t_{a_j}\rangle$ and $B'=(R')^\perp,$ so outgoing labels equal the next incoming ones.

Dimension drops only on central edges from time $3$ to $4,$ each by $h.$ With $\nu$ invocations of $c$ central wires each, total loss $L=\nu ch.$ Gate labels at times $0$ to $7$ are $B\otimes\langle t_T\rangle,$ $B\otimes F,$ $(B\otimes F)\oplus(R\otimes\langle t_T\rangle),$ $A\otimes F,$ $B\otimes F,$ $(B\otimes F)\oplus(R\otimes\langle t_T\rangle^\perp),$ $A\otimes F,$ $A\otimes F,$ with $\oplus$ the orthogonal sum.

The side edge has residual $R\otimes\langle t_S,t_T\rangle^\perp$ of dimension $h-2,$ since neighbouring triples satisfy $t_S\perp t_T.$ Signed changes telescope to $Wm-2N,$ and absolute changes total $Wm-2N+2L.$

The loss ratios are $L_{\rm b}/N=100/539$ <cite data-source="paper">\frac{L_{\rm b}}{N}=\frac{100}{539}</cite> and $L_{\rm c}/N=101/539,$ both below one half, so $N$ extra source ranks leave a strict saving.

Bit network: $-P_{U_a}$ replaces $M_v$ at the source of $X_a;$ then $P_{U_a^\perp}+P_{U_a}=I_m$ makes every endpoint difference $I_m.$ Edge ranks equal dimension changes, except at the $N$ source edges, where $2P_{U_a}$ has rank one. Write $s$ for the total edge rank, so $s_{\rm b}=W_{\rm b}m-N+2L_{\rm b}<W_{\rm b}m.$

Complex network: $C=H\operatorname{diag}(1,i)H=\alpha I+\beta\sigma,$ with $H$ the Hadamard matrix, $\alpha=\frac{1+i}{2},$ $\beta=\frac{1-i}{2}$ and $\sigma$ the coordinate swap. Frames over binary $x$ are $\mathcal C_U=H^{\otimes m}\operatorname{diag}_x(i^{q_U(x)})H^{\otimes m},$ with $q_U(x)=|P_Ux|\bmod4$ and $|\cdot|$ the Hamming weight. For an orthonormal basis $\mathcal K$ of a residual $E,$ $q_V(x)-q_U(x)=\sum_{w\in\mathcal K}|w|\,(w\cdot x\bmod2)\pmod4,$ with $|w|$ odd. Each factor $i^{\pm(w\cdot x\bmod2)}$ is a kernel $\alpha I+\beta\tau_w$ or its inverse, since $H\operatorname{diag}(1,-1)H=\sigma$ turns $(-1)^{w\cdot x}$ into a translation $\tau_w.$ Over $\mathbb F_2$ residuals have orthonormal bases, so $s_{\rm c}=W_{\rm c}m-2N+2L_{\rm c}<W_{\rm c}m.$ The pair $X_a\to Y_a$ has inverse kernels, which $C=iZC^{-1}Z$ with $Z=\operatorname{diag}(1,-1)$ turns into $C,$ so every role carries $C^{\otimes m}.$

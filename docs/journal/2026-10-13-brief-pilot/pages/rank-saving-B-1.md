The concept is the inequality $s<Wm$ for a fixed linear network, where $W$ is its number of wires, $m$ is the dimension of its label space, and $s$ is the total rank of its edges. The recursion needs it: each wire then makes fewer than $m$ smaller calls.

The network is the [Three-stage bank exchange network](bank-exchange-network.md), which exchanges two banks with signs and restores scratch wires, and $s$ is a sum over its edges. Each gate or terminal $v$ carries a label $U$, a nondegenerate subspace, and a matrix $M_v=P_U$, the projection onto $U$ along $U^\perp$; sources carry $-P_U$ or $0$. [Frames](frames.md) cancel along each route, so each edge applies the difference of its endpoint matrices.

For labels $U\subset V$, $P_V-P_U$ is the identity on $V\cap U^\perp$ and zero on its complement, so its rank is $\dim V-\dim U$. Apart from sources, the total rank is counted by label dimension changes on the edges of the [nested labels](subspace-labels.md). The exceptions are the first edges from the $N$ sources of one bank, where $N$ is the number of entries in a bank; each has matrix $2P_U$ of rank one but no dimension change.

Along each wire the signed changes telescope, since each gate keeps the label. Sinks total $Wm-N$ and sources total $N$, so the signed changes total $Wm-2N$. Only the central edges from time 3 to time 4 decrease, each losing $h=100$ dimensions, the size of the ground set of triples. With $L$ the total dimension lost on decreasing edges, the absolute changes sum to $Wm-2N+2L$. Adding the $N$ extra source ranks gives
$$s=Wm-N+2L.$$

So $Wm-s=N-2L$, which is positive because $L<N/2$ <cite data-source="paper">In particular, both are smaller than $1/2$.</cite> Since $Wm$ is far larger than $N$, the relative deficit $\eta=(Wm-s)/(Wm)$ of the bit-valued network is only about $1.5\cdot10^{-11}$ <cite data-source="paper">\frac{339}{22587335000000}</cite>.

Each unit of edge rank is one smaller call, so $s/W$ is the number per wire. Take $\tau=1-2^{-50}$. Since $m=10^6<2^{20}$, $\ln m<20$, so $m^{\tau}=m\exp(-2^{-50}\ln m)>m(1-20\cdot2^{-50})$. As $\eta>20\cdot2^{-50}$, $s/W=m(1-\eta)<m(1-20\cdot2^{-50})<m^{\tau}$. The complex [network](phase-network.md) has relative deficit about $3.7\cdot10^{-12}$ <cite data-source="paper">\frac{73}{19906842167500}</cite>, also above $20\cdot2^{-50}$, so it satisfies the same bound.

A call at parameter $m$ times a smaller one makes $s$ calls at the smaller one ([Recursive power saving](recursive-power-saving.md)). Per unit volume, each multiplication of the parameter by $m$ multiplies the cost by $s/W$, so a power is saved when $s/W<m^{\tau}$ with $\tau<1$.

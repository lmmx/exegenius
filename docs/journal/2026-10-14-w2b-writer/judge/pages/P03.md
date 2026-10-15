Recursive rank saving is the recurrence that turns a fixed linear exchange network into a tape procedure whose time per unit of data grows only as a power of its parameter below one; the two fast tape procedures that follow are unrolled from it.

Take the network of [Linear exchange network with restored scratch wires](linear-network.md) on $W$ wires ($W$ counts the wires). Replace each scalar by an array of logical volume $V$, the total size of the array, and use [Elementary stream operations](elementary-stream-ops.md) to split it into $W$ role streams of volume $V/W$. With [Frames on wire arrays](common-frame-principle.md), every edge pays only a change of frame, whose rank is the number of smaller calls the edge needs. Let $s$ be the total number of smaller calls over all edges, and $m$ the dimension of the fixed space in which [Subspace labels from three-element subsets](subset-intersection-labels.md) places its nested subspaces. These labels give $s<Wm$ <cite data-source="paper">number $s$ of smaller calls satisfies $s<Wm$.</cite>

## The recurrence

A call at parameter $e=m^k$ makes $s$ calls at parameter $m^{k-1}$, each on a stream of volume $V/W$; here $k$ is the number of recursion levels. Splitting, merging and gates cost $O(V)$ in total. Let $F_k$ be a uniform upper bound for time divided by volume at parameter $m^k$. Then $F_0=O(1)$ and $F_k\le(s/W)F_{k-1}+O(1)$ <cite data-source="paper">F_0=O(1),\qquad F_k\le(s/W)F_{k-1}+O(1).</cite>

Unrolling gives $F_k=O(1)\sum_{j=0}^{k}(s/W)^j$. Suppose $s/W<m^\tau$ for an exponent $\tau$ with $0<\tau<1$. If $s/W\le1$ the sum is at most $k+1$; otherwise it is $O((s/W)^k)$. Either way $F_k=O(m^{k\tau})=O(e^\tau)$. Had $s/W=m$, the sum would be of order $m^k=e$, with no saving. So multiplying the parameter by $m$ multiplies the recursive contribution by only $s/W<m$, and this strict inequality is the source of the power saving <cite data-source="paper">increases the recursive contribution by $s/W<m$. This strict inequality</cite>.

## Why a tiny deficit is enough

The relative rank deficits are $\eta=(Wm-s)/(Wm)$, equal to $339/22587335000000$ for the bit network and $73/19906842167500$ for the complex network, both greater than $20\cdot2^{-50}$. <cite data-source="paper">\eta_{\rm b}=\frac{W_{\rm b}m-s_{\rm b}}{W_{\rm b}m}</cite> Subscripts b and c mark the bit and complex networks.

Since $m<2^{20}$ and $\log 2<1$, we have $\log m<20$. <cite data-source="paper">Since $m<2^{20}$ and $\log 2<1$, we have $\log m<20$, whence</cite> Hence $m^{1-2^{-50}}=m\exp(-2^{-50}\log m)>m(1-20\cdot2^{-50})>m(1-\eta)$ for either deficit $\eta$, using $e^{-x}>1-x$ for $x>0$. Since $s/W=m(1-\eta)$, consequently $s_{\rm b}/W_{\rm b}<m^\tau$ and $s_{\rm c}/W_{\rm c}<m^\sigma$ with $0<\tau=\sigma=1-2^{-50}<1$, $\sigma$ being the complex network's exponent.

The deficits are only about $10^{-11}$ of $Wm$, yet the exponent is so close to one that $m^\tau$ falls short of $m$ by a still smaller fraction.

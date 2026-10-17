Frames let a network of scalar gates act on whole arrays stored differently on different wires, so that only the representations at the two ends of a route matter. They extend the [linear exchange network with restored scratch wires](exchange-network.md) from scalars to arrays.

Let $R$ be the ring of scalars carried by the wires, and let $\Omega$ be a finite set of addresses indexing every array. Replace each scalar wire value by an array in $R^\Omega$ and apply every gate pointwise, at each address separately. Write $\mathsf S_\Omega$ for the resulting network on all the wire arrays together. Here $u$, $v$, $a$, $b$ are vertices, not their earlier meanings.

At each source, gate and sink $v$, assign an invertible $R$-linear operator $D_v$ on $R^\Omega$, called its frame. A frame may mix entries of one array across addresses, whereas a gate combines different wires at one fixed address. All incidences of a gate use the same $D_v$, and an edge from $u$ to $v$ applies $D_vD_u^{-1}$ to the array it carries.

The invariant is that the stored array at $v$ equals $D_v$ applied to the logical array. It holds at the sources by definition: with $D_{\rm in}=\bigoplus_w D_{\mathrm{source}(w)}$ stacking the source frames over the input roles $w$, a physical input $f$ is interpreted as the logical input $g=D_{\rm in}^{-1}f$, which needs no preprocessing. It continues along an edge: if the logical array is $\varphi$, the stored array $D_u\varphi$ becomes $D_vD_u^{-1}D_u\varphi=D_v\varphi$.

At a gate $G$ with $t$ input arrays, every incoming array has the common frame $D_v$, and a gate output combines the logical inputs $\varphi_1,\dots,\varphi_t$ with coefficients $c_j\in R$. Because $D_v$ is $R$-linear, $\sum_j c_jD_v\varphi_j=D_v\bigl(\sum_j c_j\varphi_j\bigr)$. So $G(D_vg_1,\dots,D_vg_t)=D_vG(g_1,\dots,g_t)$ for any arrays $g_1,\dots,g_t$: the gate applied to stored arrays gives the stored form of the logical output, so the invariant continues to the sinks <cite data-source="paper">the common frame commutes with the pointwise gate</cite>.

With $D_{\rm out}=\bigoplus_w D_{\mathrm{sink}(w)}$, the implemented operator is therefore
$$D_{\rm out}\,\mathsf S_\Omega\,D_{\rm in}^{-1}.$$

On a route from a source $u$ through gates $a$ and $b$ to a sink $v$, the edges apply $D_aD_u^{-1}$, $D_bD_a^{-1}$ and $D_vD_b^{-1}$, whose product is $D_vD_u^{-1}$: the intermediate frames telescope away. So if the scalar network is a signed role permutation sending input role $w$ to output role $\rho(w)$ with scalar sign $\varepsilon_w$, that route applies $\varepsilon_wD_{\mathrm{sink}(\rho(w))}D_{\mathrm{source}(w)}^{-1}$ to its input array. This holds for every role, scratch roles included.

## What the free frames are for

Once the scalar routing is fixed, the endpoint frames determine the array operator on each route <cite data-source="paper">the endpoint frames determine the array operator on each route</cite>. Interior frames do not change that operator, but each sets what the edges beside it must compute, and an edge costs whatever applying $D_vD_u^{-1}$ costs. So the intermediate frames can be chosen to make each edge change inexpensive <cite data-source="paper">intermediate frames can then be chosen to make each edge change inexpensive.</cite>

The later page [Nested subspace labels and rank deficit](subspace-labels.md) makes this choice: it uses nested subspaces as frames, so that an edge costs one smaller call per unit of rank change. The number of such calls over the network is what the [Recursive power saving](rank-recurrence.md) recurrence counts.

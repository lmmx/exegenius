Frames extend the [Linear exchange network with restored scratch wires](linear-network.md) from scalars to arrays and show that interior choices do not change the result. Replace each scalar by an array, an element of $R^\Omega$, where $R$ is the ring of coefficients and $\Omega$ is the set of addresses within one wire's array. A gate still acts pointwise: at each address it combines wire values with coefficients $c_j\in R$.

A *frame* assigns to each source, gate and sink $v$ an invertible $R$-linear operator $D_v$ on $R^\Omega$ <cite data-source="paper">At each source, gate and sink $v$, assign an invertible $R$-linear operator</cite>. Unlike a gate, it may mix addresses. All incidences of one gate use the same $D_v$. On an edge from $u$ to $v$ the network applies $D_vD_u^{-1}$ to that wire's array. Later pages bound the cost of each change of frame through its rank, as in [Rank, subspaces and orthogonal complements](linear-algebra-basics.md).

Let $D_{\rm in}$ be the direct sum of the source frames over the input roles and $D_{\rm out}$ the direct sum of the sink frames over the output roles. Let $\mathsf S_\Omega$ be the operator the scalar network applies independently at every address. For a physical input $f$, define the logical input $g=D_{\rm in}^{-1}f$; this needs no preprocessing.

**Invariant.** At every vertex $v$ the stored array equals $D_v$ applied to the logical array, the one the scalar network would hold on $g$. Induct along the wires.

- At a source this is the definition of $g$.
- On an edge from $u$ to $v$ with logical array $\varphi$, the stored array $D_u\varphi$ becomes $D_vD_u^{-1}D_u\varphi=D_v\varphi$.
- At a gate with logical inputs $\varphi_j$, the stored inputs are $D_v\varphi_j$, because all incidences use the same frame. Each output is $\sum_j c_jD_v\varphi_j=D_v\big(\sum_j c_j\varphi_j\big)$ by linearity, which is $D_v$ applied to the logical output.

At the sinks the stored arrays are therefore $D_{\rm out}\mathsf S_\Omega D_{\rm in}^{-1}f$, so the network implements
$$D_{\rm out}\,\mathsf S_\Omega\,D_{\rm in}^{-1}.$$
<cite data-source="paper">D_{\rm out}\,\mathsf S_\Omega\,D_{\rm in}^{-1}.</cite>

**Interior frames cancel.** A route through a vertex $m$ from a source $s$ to a sink $t$ applies $D_mD_s^{-1}$ and then $D_tD_m^{-1}$, whose product is $D_tD_s^{-1}$ for every $D_m$.

Suppose the scalar network is a signed role permutation sending input role $w$ to output role $\rho(w)$ with sign $\varepsilon_w$. Then that route applies $\varepsilon_wD_{\mathrm{sink}(\rho(w))}D_{\mathrm{source}(w)}^{-1}$ to its array, auxiliary roles included. Once the routing is fixed, <cite data-source="paper">the endpoint frames determine the array operator on each route</cite>, and the interior frames can be chosen purely to make each edge change inexpensive.

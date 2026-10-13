# Reference pack for judging plans of `paper.tex`

Each item is a result the main theorem's proof depends on, from a draft list that has not been reviewed: treat its statement as a guide and the paper's text as the authority. Line numbers are those of `paper.tex`.


## bank-exchange-network (core): Scalar bank-exchange network with restored scratch wires (Lemma lem:scalar-motifs; eight-row forward schedule eq:motif-schedule)

**Draft statement.** An eight-row schedule of copy, gather, scatter and side-injection gates on three-element subsets of $[100]$ adds $x_S$ to $y_S$ whatever the initial scratch values, because the net change $(\mathcal J\mathcal V+\mathcal R\mathcal G)x$ is the identity (central coefficient $|S\cap T|\bmod 2$ or $(|S\cap T|-1)/2$, side wires cancel the rest). Three stages (forward, inverse, forward) give $(X_a,Y_a)\mapsto(-Y_a,X_a)$ and restore every auxiliary wire.

**Role.** Supplies the fixed scalar network (wire count W, signed role permutation rho) on which both the bit and complex constructions put frames; the exchange of banks is what becomes an address interchange or a butterfly layer.

**Derived at lines:** 741-783 (proof; neighbour counts and wire counts eq:bit-wire-count, eq:complex-wire-count)

**The paper where it is stated:**

```
  653  Put
  654  \[
  655   h=100,\qquad v=\binom h3=161700,\qquad
  656   N=v^3=4227952113000000,\qquad m=h^3=1000000.
  657  \]
  658  Let $\mathcal T$ be the set of three-element subsets of $[h]$.
  659  There are two data banks $X_a,Y_a$, indexed by
  660  $a=(a_1,a_2,a_3)\in\mathcal T^3$.
  661  A wire carries one scalar.  A gate is a fixed linear map on a specified
  662  set of wires; untouched wires retain their values.  The vertices on a
  663  wire are its source, the gates that touch it in time order, and its sink;
  664  its edges join consecutive vertices.  We call the name of a wire its
  665  \emph{role}, so that we can distinguish a fixed place in the network
  666  from the value currently carried there.
  667  
  668  \subsection{Scalar operations and restored auxiliary values}
  669  
  670  There are three stages.  At stage $j$, fix the other two coordinates and
  671  operate on the $v$ values in each bank obtained by varying coordinate $j$.  Thus
  672  there are $v^2$ separate invocations at each stage.
  673  Each such invocation has its own auxiliary wires, distinct from all
  674  other invocations, including those in other stages.  There are
  675  \[
  676   I=3v^2=78440670000
  677  \]
  678  invocations.  In a forward invocation, write $x_T$ for the data used as
  679  controls and $y_S$ for the data to be updated; $S,T\in\mathcal T$ are
  680  their varying $j$th coordinates.  These are the logical source and
  681  target banks of the invocation.  For each ordered pair $(S,T)$ that is
  682  neighboring under the rule below, there is an auxiliary wire $A_{ST}$.
  683  There are also central wires $C_i$, $i\in[h]$, and in the second
  684  construction a wire $C_\ast$.  We call all these auxiliary wires
  685  \emph{scratch wires}; their initial values may be arbitrary.
  686  
  687  The following maps specify the construction.
  688  \begin{itemize}
  689  \item In the bit construction, scalars lie in $\mathbb F_2$, and
  690  $S,T$ are neighbors when $|S\cap T|=1$.
  691  The copy map adds $x_T$ to $A_{ST}$ for each neighbor pair.
  692  The side injection adds $\sum_{T:\,|S\cap T|=1}A_{ST}$ to $y_S$.
  693  The gather adds $\sum_{T\ni i}x_T$ to $C_i$, and the scatter adds
  694  $\sum_{i\in S}C_i$ to $y_S$.
  695  \item In the complex construction, scalars lie in
  696  $\mathbb Z[i,1/2]$, and neighbors are distinct triples with even
  697  intersection.  Copy is unchanged.  Side injection adds
  698  \[
  699   -\sum_{T\text{ neighboring }S}\frac{|S\cap T|-1}{2}A_{ST}
  700  \]
  701  to $y_S$.  Gather adds $\sum_{T\ni i}x_T$ to $C_i$ and
  702  $\sum_Tx_T$ to $C_\ast$.  Scatter adds
  703  $(\sum_{i\in S}C_i-C_\ast)/2$ to $y_S$.
  704  \end{itemize}
  705  Each update leaves its control wires unchanged.  The first two rows of
  706  the following schedule remove the effect of the old scratch values;
  707  the next four introduce the source values and add their effect to the
  708  target; the last two restore the scratch values.
  709  This cancellation of arbitrary initial scratch values is a linear
  710  instance of the restoration construction for transparent computation in
  711  Buhrman, Cleve, Kouck\'y, Loff and Speelman~\cite[Section~3]{BuhrmanCleveKouckyLoffSpeelman2014}.
  712  The forward schedule is
  713  \begin{equation}
  714  \begin{array}{c|l|l}
  715  \text{row}&\text{operation}&\text{gate grouping}\\ \hline
  716  0&\text{subtract side injection}&\text{one gate per target}\\
  717  1&\text{subtract scatter}&\text{one gate on targets and center}\\
  718  2&\text{copy}&\text{one gate per source}\\
  719  3&\text{gather}&\text{one gate on sources and center}\\
  720  4&\text{scatter}&\text{one gate on targets and center}\\
  721  5&\text{side injection}&\text{one gate per target}\\
  722  6&\text{undo gather}&\text{one gate on sources and center}\\
  723  7&\text{undo copy}&\text{one gate per source}.
  724  \end{array}
  725  \label{eq:motif-schedule}
  726  \end{equation}
```

```
  728  \begin{lemma}\label{lem:scalar-motifs}
  729  For arbitrary initial values of its auxiliary wires, a forward
  730  invocation changes $y_S$ to $y_S+x_S$ and leaves every other value
  731  unchanged.  Performing a forward invocation from $X$ to $Y$ at stage 1,
  732  the inverse forward schedule from $Y$ to $X$ at stage 2, and a forward
  733  invocation from $X$ to $Y$ at stage 3 sends
  734  \[
  735   (X_a,Y_a)\longmapsto(-Y_a,X_a)
  736  \]
  737  and restores every auxiliary value.  In the bit construction this is the
  738  permutation exchanging $X_a$ and $Y_a$.
  739  \end{lemma}
```


## common-frame-identity (core): Frames and the common-frame identity (eq:common-frame-identity)

**Draft statement.** Give every source, gate and sink an invertible operator $D_v$ (its frame) and apply $D_vD_u^{-1}$ on each edge; since a common frame commutes with a pointwise linear gate, the network on arrays implements $D_{\rm out}\,\mathsf S_\Omega\,D_{\rm in}^{-1}$, so a route $w\to\rho(w)$ applies only $\varepsilon_wD_{\mathrm{sink}(\rho(w))}D_{\mathrm{source}(w)}^{-1}$.

**Role.** Lets the endpoint frames fix the operator on every role (a shear for the bit network, a C-tensor for the complex one) while intermediate frames are chosen so each edge change is cheap; used by bit-shear-interface, complex-phase-interface, interchange-recurrence and phase-frame-recursion.

**Derived at lines:** 804-843

**The paper where it is stated:**

```
  793  At each source, gate and sink $v$, assign an invertible $R$-linear operator
  794  $D_v$ on $R^\Omega$, called its \emph{frame}.  A frame acts on the values
  795  indexed by addresses within one wire's array and may mix them; a gate instead
  796  combines wire values at each fixed address.  All incoming and outgoing
  797  incidences of one gate use the same $D_v$.  On an edge from $u$ to $v$, apply
  798  $D_vD_u^{-1}$ to that wire's array.  Define the operators on all input and
  799  output roles by
  800  \[
  801   D_{\rm in}=\bigoplus_w D_{\mathrm{source}(w)},\qquad
  802   D_{\rm out}=\bigoplus_w D_{\mathrm{sink}(w)}.
  803  \]
```


## label-residual-budget (core): Nested subspace labels and the dimension budget (Lemma lem:motif-residuals; eq:motif-labels, eq:rank-budget-labels)

**Draft statement.** Gates and terminals get nondegenerate subspaces of $\mathcal F=F^{\otimes3}$ (dimension $m=h^3$) built from triple indicator vectors, which are orthogonal for neighbouring triples; every edge joins nested labels and the only decreasing edges are the central wires from time 3 to 4, each losing $h$. Hence $L=Ich$ and $\sum_e|\Delta\dim|=Wm-2N+2L$, with $L_{\rm b}/N=100/539$ and $L_{\rm c}/N=101/539$.

**Role.** Converts the network into a count of smaller operations per edge (orthogonal residual dimension); the bound L<N/2 is what produces the strict saving in rank-saving.

**Derived at lines:** 1012-1111 (proof incl. orthonormal bases over F_2; loss ratios at 1105-1109)

**The paper where it is stated:**

```
  845  \subsection{Subspaces attached to the gates}
  846  
  847  The scalar network already has the required signed exchange. We now
  848  choose its intermediate labels to make the changes of frame inexpensive.
  849  A label is a nondegenerate subspace of a fixed bilinear space. On every
  850  edge one label will contain the other. For such an edge, the
  851  \emph{orthogonal residual} is the orthogonal complement of the smaller label inside
  852  the larger one; its dimension is the absolute change in label dimension.
  853  The saving will come from keeping the total dimension lost on decreasing
  854  edges small relative to $N$.
  855  
  856  For the bit network, let $F=\mathbb Q^h$ with bilinear form
  857  \[
  858   \langle x,y\rangle=x^{\mathsf T}(I-J/9)y,
  859  \]
  860  where $J$ is the all-one matrix.  This form has eigenvalues $1$ and
  861  $1-h/9=-91/9$, so it is nondegenerate.  For a triple $T$ let $t_T$ be
  862  its indicator vector.  Then
  863  \[
  864   \langle t_S,t_T\rangle=|S\cap T|-1,
  865   \qquad \langle t_T,t_T\rangle=2.
  866  \]
  867  For the complex network, take instead $F=\mathbb F_2^h$ with the
  868  ordinary dot product; here $t_T\cdot t_T=1$.
  869  Thus every triple line is nondegenerate, and neighboring triple lines
  870  are orthogonal, in the label field belonging to either construction.
  871  The scalar domain need not equal the label field.
  872  These degree-one intersection formulas specialize the construction
  873  described by Alon~\cite[Section~3, remark after Theorem~1.1]{Alon1998}
  874  to the prime $2$, with the ground-set size changed to $h=100$.
  875  
  876  The ambient label space is $\mathcal F=F^{\otimes3}$ with its tensor
  877  bilinear form.  It has dimension $m$ and is nondegenerate.  Write
  878  \[
  879   u_a=t_{a_1}\otimes t_{a_2}\otimes t_{a_3},\qquad U_a=\langle u_a\rangle.
  880  \]
  881  The prescribed terminal labels are
  882  \[
  883  \begin{array}{c|cc}
  884  \text{role}&\text{source label}&\text{sink label}\\ \hline
  885  X_a&U_a&\mathcal F\\
  886  Y_a&0&U_a^\perp\\
  887  \text{each scratch role}&0&\mathcal F.
  888  \end{array}
  889  \]
  890  Write $W$ for the wire count in the construction under consideration.
  891  The source dimensions sum to $N$ and the sink dimensions to $Wm-N$.
  892  Thus the signed dimension changes will sum to $Wm-2N$, because the
  893  contributions at gates cancel. If decreasing edges lose a total
  894  dimension $L$, the sum of absolute changes is $Wm-2N+2L$.
  895  We will obtain $L<N/2$. The rational interface below will require
  896  an additional $N$ source ranks, so this inequality will leave a strict
  897  saving in both constructions.
  898  
  899  At stage $j$, fix an invocation.  Put
  900  \[
  901   A=F^{\otimes(j-1)},\qquad
  902   P=\langle t_{a_1}\otimes\cdots\otimes t_{a_{j-1}}\rangle,
  903   \qquad B=P^\perp\subset A.
  904  \]
  905  An empty tensor product is the one-dimensional ground field; thus
  906  $B=0$ at stage 1.  Let
  907  $Q=\langle t_{a_{j+1}}\otimes\cdots\otimes t_{a_3}\rangle
  908  \subset F^{\otimes(3-j)}$, again using the empty tensor product at
  909  stage 3.  Since $P$ is nondegenerate,
  910  $A=B\mathbin{\perp}P$.  
  911  We prescribe the following transition for each data wire in the stage,
  912  where $t$ is the indicator of that wire's $j$th triple:
  913  \[
  914  \begin{array}{c|c|c}
  915   &\text{incoming label}&\text{outgoing label}\\ \hline
  916   X&A\otimes\langle t\rangle\otimes Q&(A\otimes F)\otimes Q\\
  917   Y&B\otimes\langle t\rangle\otimes Q&
  918   \bigl((B\otimes F)\mathbin{\perp}(P\otimes t^\perp)\bigr)\otimes Q.
  919  \end{array}
  920  \]
  921  The outgoing $Y$ label can also be written
  922  $(P\otimes\langle t\rangle)^\perp_{A\otimes F}\otimes Q$.
  923  These transitions expose one tensor factor at each stage. To see that
  924  they fit together, for $j<3$ write
  925  $Q=\langle t_{a_{j+1}}\rangle\otimes Q'$. The next stage has
```

```
  997  \begin{lemma}\label{lem:motif-residuals}
  998  Every edge joins comparable nondegenerate labels.  The only decreasing
  999  edges are the central-wire edges from time 3 to time 4; their dimension
 1000  loss is $h$.  Consequently, if $c$ is the number of central wires per
 1001  invocation and $W$ the total number of wires, then
 1002  \begin{equation}
 1003   L=Ich,\qquad
 1004   \sum_{e}\bigl|\dim U_{\mathrm{head}(e)}-
 1005                         \dim U_{\mathrm{tail}(e)}\bigr|
 1006   =Wm-2N+2L.                                      \label{eq:rank-budget-labels}
 1007  \end{equation}
 1008  For the binary label space, every nonzero orthogonal residual of an
 1009  edge has an orthonormal basis.
 1010  \end{lemma}
```


## bit-shear-interface (core): Rational matrix interface of the bit network (Proposition prop:bit-motif-interface)

**Draft statement.** Assigning projections $P_U$ to gates and sinks and $-P_{U_a}$ to sources $X_a$ gives endpoint differences $M_{\mathrm{sink}(\rho(w))}-M_{\mathrm{source}(w)}=I_m$ on every role, so with frames $\Phi_M:(H,D)\mapsto(H+MD,D)$ the XOR network applies the shear $H\gets H+D$ on every role; the edge ranks sum to $s_{\rm b}=W_{\rm b}m-N+2L_{\rm b}<W_{\rm b}m$.

**Role.** The bit-procedure contract (eq:finite-shear-contract) that interchange-recurrence consumes: endpoint identity plus total rank s<Wm.

**Derived at lines:** 1156-1172

**The paper where it is stated:**

```
 1138  \begin{proposition}\label{prop:bit-motif-interface}
 1139  The bit network has $W_{\rm b}$ wires, uses only pointwise XOR gates,
 1140  and sends its input vector to the permutation $\rho$ of that vector.
 1141  Its rational matrices are common to all incidences of a gate and obey
 1142  \[
 1143   M_{\mathrm{sink}(\rho(w))}-M_{\mathrm{source}(w)}=I_m
 1144   \quad\text{for every input role }w.
 1145  \]
 1146  The sum of the rational ranks of all edge differences is
 1147  \begin{align}
 1148   s_{\rm b}
 1149   &=\sum_{e}\operatorname{rank}_{\mathbb Q}
 1150        (M_{\mathrm{head}(e)}-M_{\mathrm{tail}(e)})\\
 1151   &=W_{\rm b}m-N+2L_{\rm b}
 1152    =177176569088785861287000000<W_{\rm b}m.
 1153  \end{align}
 1154  \end{proposition}
```


## complex-phase-interface (core): Binary phase interface of the complex network (Proposition prop:complex-motif-interface)

**Draft statement.** With phase frames $\mathcal C_U=H^{\otimes m}\operatorname{diag}(i^{\operatorname{wt}(P_Ux)\bmod 4})H^{\otimes m}$, each edge change $\mathcal C_V\mathcal C_U^{-1}$ is a product of one translation kernel $aI+bX_v$ (or inverse) per orthonormal residual vector, $s_{\rm c}=W_{\rm c}m-2N+2L_{\rm c}<W_{\rm c}m$ in total, and after diagonal corrections every role receives $C^{\otimes m}$ (also on $k$ columns by tensoring).

**Role.** The numerical-procedure contract: it turns a C-tensor on e selected bits into s_c child C-tensors on e/m bits; consumed by phase-frame-recursion and layer-recurrence.

**Derived at lines:** 1231-1315

**The paper where it is stated:**

```
 1174  \subsection{The binary phase interface}
 1175  
 1176  Use the complex scalar network and binary labels.  For a binary vector
 1177  $x$, let $\operatorname{wt}(x)$ denote its integer Hamming weight, and
 1178  write $[b]\in\{0,1\}$ for the integer representative of $b\in\mathbb F_2$.
 1179  For a label $U$, let
 1180  \[
 1181   q_U(x)=\operatorname{wt}(P_Ux)\pmod4.
 1182  \]
 1183  The projection here is over $\mathbb F_2$.
 1184  Let
 1185  \[
 1186   H=\frac1{\sqrt2}\begin{pmatrix}1&1\\1&-1\end{pmatrix},\quad
 1187   C=H\begin{pmatrix}1&0\\0&i\end{pmatrix}H
 1188     =aI+bX,\qquad a=\frac{1+i}{2},\quad b=\frac{1-i}{2},
 1189  \]
 1190  where $X$ interchanges the two coordinates.  On functions of
 1191  $x\in\mathbb F_2^m$ define
 1192  \[
 1193   \mathcal C_U=H^{\otimes m}\operatorname{diag}_x(i^{q_U(x)})H^{\otimes m}.
 1194  \]
 1195  The use of $H$ in this definition is an identity of matrices.  Each entry
 1196  of $\mathcal C_U$ and its inverse is a Gaussian integer divided by $2^m$,
 1197  so both preserve arrays over $\mathbb Z[i,1/2]$.  The kernels implemented
 1198  below have Gaussian dyadic coefficients.
 1199  
 1200  To apply the scalar network to these functions, replace the scalar on
 1201  each wire $w$ by an array
 1202  $f_w:\mathbb F_2^m\to\mathbb Z[i,1/2]$.  At each address, a gate
 1203  applies its scalar linear map to the values on the wires it touches.
 1204  The frame in \eqref{eq:common-frame-identity} is $D_v=\mathcal C_U$ at a
 1205  vertex with label $U$.  On an edge from label $U$ to label $V$, apply
 1206  $\mathcal C_V\mathcal C_U^{-1}$ to that wire's array.
 1207  All frames are diagonalized by the same matrix $H^{\otimes m}$.
 1208  For an integer $k\ge1$, an address for $k$ columns is
 1209  $(x^{(1)},\ldots,x^{(k)})\in(\mathbb F_2^m)^k$; every frame and edge
 1210  operator is then the tensor product of its one-column version over
 1211  these $k$ coordinates.  Each scalar gate is still applied once at each
 1212  joint address, to the vector of values on the roles it touches.
 1213  
 1214  \begin{proposition}\label{prop:complex-motif-interface}
 1215  For each edge of the complex network, write $U,V$ for its tail and
 1216  head labels.  Its phase-frame difference
 1217  $\mathcal C_V\mathcal C_U^{-1}$ is a product of one translation
 1218  kernel $aI+bX_v$, or its inverse, per vector in an orthonormal basis
 1219  of the edge residual.  Here $(X_vf)(x)=f(x+v)$.
 1220  The sum of these basis lengths is
 1221  \[
 1222   s_{\rm c}=W_{\rm c}m-2N+2L_{\rm c}
 1223   =1873807244636671267308000000<W_{\rm c}m.
 1224  \]
 1225  After source and sink diagonal corrections, and undoing the signed
 1226  bank exchange, telescoping these phase frames gives $C^{\otimes m}$
 1227  on every role, including auxiliary roles.  The same assertion holds
 1228  on any positive number of columns by taking tensor products over columns.
 1229  \end{proposition}
```


## rank-saving (core): The saving s<Wm and the exponent tau=sigma=1-2^{-50} (subsection 'Explicit rational bounds for the two recurrences'; eq:explicit-motif-exponents)

**Draft statement.** Because $L<N/2$, both networks make fewer smaller calls than wires times dimension: $s_{\rm b}<W_{\rm b}m$, $s_{\rm c}<W_{\rm c}m$, with relative deficits $\eta_{\rm b}=339/22587335000000$, $\eta_{\rm c}=73/19906842167500$, both above $20\cdot2^{-50}$. Since $\log m<20$, $s/W<m^{\tau}$ with $\tau=\sigma=1-2^{-50}$.

**Role.** The strict inequality s/W<m is the source of the power saving; it is the ratio in both recurrences (interchange-recurrence, layer-recurrence) and fixes tau, sigma used in time-accounting.

**Derived at lines:** 1105-1110 (L/N<1/2), 1331-1346

**The paper where it is stated:**

```
 1151   &=W_{\rm b}m-N+2L_{\rm b}
 1152    =177176569088785861287000000<W_{\rm b}m.
```

```
 1222   s_{\rm c}=W_{\rm c}m-2N+2L_{\rm c}
 1223   =1873807244636671267308000000<W_{\rm c}m.
```

```
 1319  The relative rank deficits are
 1320  \[
 1321   \eta_{\rm b}=\frac{W_{\rm b}m-s_{\rm b}}{W_{\rm b}m}
 1322         =\frac{339}{22587335000000},\qquad
 1323   \eta_{\rm c}=\frac{W_{\rm c}m-s_{\rm c}}{W_{\rm c}m}
 1324         =\frac{73}{19906842167500}.
 1325  \]
 1326  For both recurrences we may use the fixed rational exponent
 1327  \begin{equation}
 1328   \tau=\sigma=1-2^{-50}.
 1329   \label{eq:explicit-motif-exponents}
 1330  \end{equation}
```


## stream-primitives (supporting): Elementary stream operations and controlled shifts (Lemmas lem:elementary-streams, lem:ordered-affine-streams)

**Draft statement.** On a fixed number of tapes, in $O(V)$: permute or move single radix-$q$ digits, split and merge on a digit, split whole rows cyclically into $W$ streams of volume $V/W$; and update a later field $y\mapsto y+a(\text{controls})\pmod{q^b}$ or multiply it by a fixed rational unit, provided the offset costs $b^{O(1)}$.

**Role.** Linear-cost building blocks behind the role-stream split, the triangular shears in matrix-shear, the packed rotations, and the CRT map.

**Derived at lines:** 513-550, 574-639

**The paper where it is stated:**

```
  491  \begin{lemma}[Elementary stream operations]
  492  \label{lem:elementary-streams}
  493  Fix a radix $q\ge2$.  Each of the following operations on an array of
  494  volume $V$ takes $O(V)$ steps on a fixed number of one-dimensional tapes:
  495  \begin{enumerate}
  496  \item any fixed permutation of the pair of digit values at two specified
  497  radix-$q$ positions, whether or not the positions are adjacent;
  498  \item moving one specified radix-$q$ digit to another position, preserving
  499  the order of the intervening positions;
  500  \item splitting on a specified radix-$q$ digit, applying a fixed
  501  pointwise map to aligned streams, and merging them back;
  502  \item for positive integers $P,R,B$, within each prefix of an array on
  503  $[P]\times[R]\times[B]$, splitting
  504  whole rows cyclically into a fixed number $W$ of streams, and, when $W$
  505  divides $R$, merging those streams in any fixed order of the row classes.
  506  \end{enumerate}
  507  The bounds include shape preparation, counter initialization and resets,
  508  and returning or clearing the visited portions of work tapes.  Their
  509  constants may depend on the fixed radix, the fixed pointwise map, and the
  510  fixed number of row classes.
  511  \end{lemma}
```

```
  555  \begin{lemma}[Controlled shifts and fixed rational scalings]
  556  \label{lem:ordered-affine-streams}
  557  Fix a prime $q$, a fixed number of control fields, and finitely many
  558  rational coefficients whose denominators are prime to $q$.  Whenever a
  559  coefficient is used as a unit, also require its numerator to be prime to
  560  $q$.  Put $Q=q^b$, $b\ge1$.
  561  Suppose a target field $y\in[Q]$ follows its control fields, each control
  562  used in an offset having $O(b)$ digits.  An update
  563  \[
  564   y\longmapsto y+a(\text{controls})\pmod Q
  565  \]
  566  costs $O(V)$ if the offset can be prepared in $b^{O(1)}$ time from the
  567  fixed number of control values.  In particular, this holds for a fixed
  568  rational linear combination of them, specialized modulo $Q$.
  569  Multiplication of $y$ by any of the specified rational units, and its
  570  inverse, also costs $O(V)$.  Arbitrarily long intervening spectator
  571  fields and suffix records are permitted.
  572  \end{lemma}
```


## matrix-shear (core): Lower triangular factorization and the cost of a rational matrix shear (Lemmas lem:lower-lower, lem:matrix-shear)

**Draft statement.** Every rational $m\times m$ matrix of rank $a$ factors as $A=E_1\Pi E_2$ with invertible lower-triangular $E_i$ and a partial permutation $\Pi$ with $a$ ones (a reversed Bruhat factorization); hence $(H,D)\mapsto(H+AD,D)$ over $\mathbb Z/q^b$ costs exactly $a$ interchanges of $b$-digit fields plus $O(V)$.

**Role.** Makes the number of recursive calls on an edge equal to its rational rank, so the total per node is s_b; used by interchange-recurrence.

**Derived at lines:** 1375-1392, 1412-1447

**The paper where it is stated:**

```
 1364  \begin{lemma}[Lower triangular factorization]
 1365  \label{lem:lower-lower}
 1366  Every rational $m\times m$ matrix $A$ of rank $a$ has a factorization
 1367  \[
 1368   A=E_1\Pi E_2,
 1369  \]
 1370  where $E_1,E_2$ are invertible lower triangular rational matrices and
 1371  $\Pi$ has exactly $a$ entries equal to one, with at most one in each row
 1372  and column, and all remaining entries zero.
 1373  \end{lemma}
```

```
 1394  \begin{lemma}[The cost of a rational matrix shear]
 1395  \label{lem:matrix-shear}
 1396  Let $\mathcal A$ be a fixed finite collection of rational $m\times m$
 1397  matrices.  There is a fixed odd prime $q$ such that the following holds.
 1398  Let $H=(H_1,\ldots,H_m)$ and $D=(D_1,\ldots,D_m)$ be two ordered
 1399  groups of fields, every field ranging over $[q^b]$, with every $H_i$
 1400  before every $D_j$.  For $A\in\mathcal A$ of rational rank $a$, the
 1401  permutation
 1402  \[
 1403   (H,D)\longmapsto(H+AD,D)
 1404   \quad\text{over }\mathbb Z/q^b\mathbb Z
 1405  \]
 1406  can be performed with exactly $a$ interchanges of pairs of $b$-digit
 1407  fields and $O(V)$ further work.  Every interchange is between an $H$
 1408  field and a $D$ field.  Spectators between any of the formal fields are
 1409  allowed.
 1410  \end{lemma}
```


## interchange-recurrence (core): Power-width interchange and its recurrence (Proposition prop:power-interchange; eq:swap-recurrence)

**Draft statement.** For chunks of $e=m^k$ radix-$q$ digits with a row field divisible by $W^k$: split rows cyclically into $W$ role streams, run the bit network with shear frames (each edge by recursive interchanges, matrix-shear), get $H\gets H+D$ on every row, and interchange by $D\gets D-H,\ H\gets H+D,\ D\gets H-D$. With $s$ calls on volume $V/W$ and a fixed-tape depth-first schedule costing $O(V)$ per node, $F_0=O(1)$, $F_k\le(s/W)F_{k-1}+O(1)$, so $F_k=O(e^\tau)$.

**Role.** The recursive saving for address movement: the O(V e^tau) bound that arbitrary-width-interchange extends to all widths.

**Derived at lines:** 1491-1616 (recurrence 1543-1557; fixed-tape schedule 1559-1615)

**The paper where it is stated:**

```
 1475  \begin{proposition}[Power-width interchange with rows]
 1476  \label{prop:power-interchange}
 1477  Assume the fixed circuit data satisfy
 1478  \eqref{eq:finite-shear-contract}, and let $q$ be a prime as in
 1479  Lemma~\ref{lem:matrix-shear} for its edge matrices, also avoiding the
 1480  denominators of every assigned terminal and gate matrix $M$.
 1481  Choose a rational number $\tau\in(0,1)$ with $s/W<m^\tau$.
 1482  For every $e=m^k$, $k\ge0$, consider two disjoint contiguous address
 1483  chunks of $e$ radix-$q$ digits each.  Suppose a row field preceding both
 1484  chunks has length divisible by $W^k$.  There may be additional fields
 1485  before and after the row field, between the chunks, and after them.
 1486  The chunks can be interchanged in $O(Ve^\tau)$ time, preserving every
 1487  other address field, on a fixed number of finite-alphabet tapes.
 1488  The constant is independent of all field lengths and of the data.
 1489  \end{proposition}
```


## arbitrary-width-interchange (supporting): Arbitrary-width interchange (Lemma lem:chunk-swap)

**Draft statement.** Any two disjoint $u$-bit address chunks of a bit array of volume $V$ can be interchanged in $O(Vu^\tau)$: take $\rho=O(\log e)$ high digits as the row field, pad rows to a multiple of $W^k$, split the rest of the width into base-$m$ pieces handled by the power-width procedure, and pad binary ranges to $[q^e]$.

**Role.** The movement primitive used by the packed selected-bit addition, the transform layout, axis exposure for resampling and the CRT layout; its O(V u^tau) appears in several rows of the cost table.

**Derived at lines:** 1649-1762 (incl. unequal widths 1740-1762)

**The paper where it is stated:**

```
 1631  \begin{lemma}[Arbitrary-width interchange]
 1632  \label{lem:chunk-swap}
 1633  There exist fixed $0<\tau<1$ and one fixed finite-alphabet multitape
 1634  procedure with the following property.  For positive integers $P,G,B$
 1635  and $u\ge1$, it transforms an arbitrary bit array on
 1636  \[
 1637   [P]\times[2^u]\times[G]\times[2^u]\times[B]
 1638  \]
 1639  by the address permutation
 1640  \[
 1641   (p,h,g,d,z)\longmapsto(p,d,g,h,z)
 1642  \]
 1643  in $O(Vu^\tau)$ steps, where $V=PG B\,2^{2u}$.
 1644  The integers specifying the shape are part of the input.  The time
 1645  includes their processing, all padding and unpadding, and fixed-tape
 1646  workspace cleanup.  In particular, $B=1$ is allowed.
 1647  \end{lemma}
```


## phase-frame-recursion (core): Phase frames for a simultaneous butterfly layer (subsection subsec:phase-frames; eq:phase-edge-coordinate, eq:phase-inverse-child, eq:phase-butterfly-layer)

**Draft statement.** Conjugating by a binary change of basis $\mathcal P_M$ turns each edge's kernels $C_{v_j}$ into kernels on coordinate slots, so a group of $e=mf$ selected axes needs exactly $s_{\rm c}$ forward child calls of $C^{\otimes f}$ (inverses via $(C^{-1})^{\otimes f}=(-i)^fZ^{\otimes f}C^{\otimes f}Z^{\otimes f}$) plus a fixed number of row additions; finally $H_0^{\otimes D}=b^D\mathcal S_DC^{\otimes D}\mathcal S_D$ recovers the normalized layer.

**Role.** Defines the recursion of the numerical procedure (children per node = s_c, child size e/m) and reduces its non-recursive work to binary row additions, which packed-selected-bit-addition performs.

**Derived at lines:** 1867-1940

**The paper where it is stated:**

```
 1806  \subsection{Phase frames for a simultaneous butterfly layer}
 1807  \label{subsec:phase-frames}
 1808  
 1809  Hold the unselected address bits fixed and regard the stored complex
 1810  values on one wire role as a function of the selected bits.  The phase
 1811  identities below express an edge's residual operator as a product of
 1812  two-point kernels on selected directions.
 1813  
 1814  Put
 1815  \[
 1816   H_0=\frac12\begin{pmatrix}1&1\\1&-1\end{pmatrix},\qquad
 1817   H=\sqrt2 H_0,\qquad \mathsf S=\operatorname{diag}(1,i),\qquad
 1818   Z=\operatorname{diag}(1,-1).
 1819  \]
 1820  For the bit-flip matrix $X=\bigl(\begin{smallmatrix}0&1\\1&0\end{smallmatrix}\bigr)$,
 1821  define
 1822  \begin{equation}\label{eq:phase-C}
 1823   C=H\mathsf SH=aI+bX,\qquad a=\frac{1+i}{2},\quad b=\frac{1-i}{2}.
 1824  \end{equation}
 1825  Direct multiplication gives
 1826  \begin{equation}\label{eq:phase-basic-identities}
 1827   H_0=b\mathsf SC\mathsf S,\qquad C^2=X,\qquad C^4=I,\qquad
 1828   C^{-1}=bI+aX=-iZCZ.
 1829  \end{equation}
 1830  In particular, $C=iZC^{-1}Z$.  These identities use only Gaussian dyadic
 1831  coefficients.  The Hadamard matrices in the definition of
 1832  $\mathcal C_U$ are used to prove identities; they need not be executed
 1833  by the machine.
```

```
 1835  Consider an edge from label $U_-$ to label $U_+$, and choose an
 1836  orthonormal basis $v_1,\ldots,v_{\rho_{\rm edge}}$ of its residual.
 1837  Proposition~\ref{prop:complex-motif-interface} gives the frame change
 1838  \[
 1839   \mathcal C_{U_+}\mathcal C_{U_-}^{-1}
 1840   =\prod_{j=1}^{\rho_{\rm edge}}(aI+bX_{v_j})^{\epsilon_j},
 1841   \qquad \epsilon_j\in\{1,-1\}.
 1842  \]
 1843  Here $(X_vf)(x)=f(x+v)$, and we write $C_v=aI+bX_v$.
 1844  The number of directional kernels is precisely
 1845  $\rho_{\rm edge}$, including on decreasing edges. The next step puts those directions
 1846  into coordinate positions without performing a general transpose.
 1847  
 1848  \paragraph{Implementing an arbitrary residual basis.}
 1849  For $M\in\operatorname{GL}(m,\mathbb F_2)$, let $\mathcal P_M$ be the
 1850  address permutation that moves the value at $x$ to $Mx$.  Thus
 1851  $(\mathcal P_Mf)(x)=f(M^{-1}x)$, and
 1852  \begin{equation}\label{eq:phase-conjugation}
 1853   \mathcal P_MX_v\mathcal P_M^{-1}=X_{Mv}.
 1854  \end{equation}
 1855  Extend $v_1,\ldots,v_{\rho_{\rm edge}}$ to a basis of $\mathbb F_2^m$ and take these vectors as
 1856  the first $\rho_{\rm edge}$ columns of $M$.  The displayed edge identity
 1857  then becomes
 1858  \begin{equation}\label{eq:phase-edge-coordinate}
 1859   \mathcal C_{U_+}\mathcal C_{U_-}^{-1}
 1860   =\mathcal P_M\left(\prod_{j=1}^{\rho_{\rm edge}}C_{e_j}^{\epsilon_j}\right)
 1861     \mathcal P_M^{-1},
 1862  \end{equation}
 1863  where $e_j$ is the $j$th standard basis vector.  The matrix $M$ need not
 1864  preserve the dot product: equation~\eqref{eq:phase-conjugation} only
 1865  uses its invertibility.
 1866  
```


## packed-selected-bit-addition (core): Packed selected-bit addition on one rectangle (Lemma lem:packed-selected-bit-rectangle)

**Draft statement.** The XORs $y_{j_i}\gets y_{j_i}\oplus x_{j_i}$ at positions $j_i=\rho+iK$ are done by eight whole-chunk cyclic shifts (eq:eight-packed-additions, using a third chunk as restored scratch) that are correct off a bad set of fraction $\delta_{\rm bad}=\min\{1,80(f-1)2^{-K}\}$; since $T_0S_0^{-1}$ permutes only that set, it is repaired by extracting and radix-sorting those records. Cost $O(V(L^\tau+1)+MA^3+\delta_{\rm bad}MA(R+A))$, i.e. $O(V(L^\tau+1))$ in the parameter family.

**Role.** Implements each binary change of basis in phase-frame-recursion at cost O(V((eK)^tau+1)) using chunk swaps; this is the node overhead in layer-recurrence.

**Derived at lines:** 2013-2231 (application to a fixed change of basis 2210-2231)

**The paper where it is stated:**

```
 1974  \begin{lemma}[Packed selected-bit addition on one rectangle]
 1975  \label{lem:packed-selected-bit-rectangle}
 1976  Let $R\ge1$, $K\ge6$, $f\ge1$, $L=fK$, and $0\le\rho<K$.
 1977  Let $N_0,N_1,N_2,N_3$ be positive integers, and let
 1978  $(\chi_1,\chi_2,\chi_3)$ be a permutation of $(x,y,z)$.
 1979  Consider $R$-bit records in lexicographic order on
 1980  \[
 1981   [N_0]\times[2^L]_{\chi_1}\times[N_1]\times[2^L]_{\chi_2}
 1982   \times[N_2]\times[2^L]_{\chi_3}\times[N_3].
 1983  \]
 1984  The subscript names the chunk at that position. The input supplies
 1985  $N_0,\ldots,N_3,R,K,f,\rho$ as a fixed number of canonical binary
 1986  integers, together with the finite permutation of the chunk names, as
 1987  in Section~\ref{sec:streams}. Put
 1988  \[
 1989   M=2^{3L}\prod_{h=0}^3N_h,\qquad V=MR,\qquad
 1990   A_{\rm rect}=\lceil\log_2(2V)\rceil.
 1991  \]
 1992  At the selected positions $j_i=\rho+iK$, numbered from the least
 1993  significant bit, one fixed multitape procedure performs
 1994  \[
 1995   y_{j_i}\longleftarrow y_{j_i}\mathbin\oplus x_{j_i}
 1996   \quad(0\le i<f).
 1997  \]
 1998  It preserves the spectator coordinates, all other chunk bits, and every
 1999  record payload; in particular, it restores the entire chunk $z$ for
 2000  every initial value.
 2001  With $\delta_{\rm bad}=\min\{1,80(f-1)2^{-K}\}$, its time is
 2002  \begin{equation}
 2003  \label{eq:packed-rectangle-time}
 2004   O\!\left(V(L^\tau+1)+MA_{\rm rect}^3+
 2005         \delta_{\rm bad}MA_{\rm rect}(R+A_{\rm rect})\right).
 2006  \end{equation}
 2007  When $A_{\rm rect}=O(p)$, $R$ exceeds every fixed polynomial in $p$,
 2008  $f\le d\le p$, and $K/\log p\to\infty$, this is
 2009  $O(V(L^\tau+1))$, uniformly over the permitted lengths and payloads
 2010  in the fixed family.
 2011  \end{lemma}
```


## layer-recurrence (core): The butterfly layer's recurrence (paragraph 'Unrolling the recurrence' in 'Batching a common layer on a fixed number of tapes')

**Draft statement.** Taking the first $q_0=O(\log d)$ chunks as a row index and splitting rows among the $W$ roles, children have volume $V/W$, so the normalized cost satisfies $F(e)\le(s/W)F(e/m)+O((eK)^\tau+1)$ with leaves $F(e)=O(e)$ for $e<d^\beta$; with $a=s/W\le m^\sigma$ and $K\le e^{c/\beta}$ this sums, over base-$m$ pieces, to $O(d^{\lambda'})$ for any $\max\{\lambda,\sigma+\beta(1-\sigma)\}<\lambda'<1$.

**Role.** Gives the O(V d^{lambda'}) cost of a simultaneous layer, the 'Simultaneous butterfly rounds' row of the cost table.

**Derived at lines:** 2286-2304 (node overhead), 2325-2391, 2409-2448

**The paper where it is stated:**

```
 2306  \paragraph{Where the rows come from.}
 2307  Put
 2308  \[
 2309   q_0=\lceil\log_2 W\rceil\,
 2310         \lceil\log_m(2d)\rceil,
 2311   \qquad k_0=\lceil\log_m d\rceil.
 2312  \]
 2313  If \(D\le q_0\), use individual \(C\) kernels, at cost \(O(V\log d)\).
 2314  Otherwise apply \(C\) individually on the selected bits of the first
 2315  \(q_0\) chunks, in \(O(V\log d)\) time.  These chunks remain a
 2316  complete consecutive address block before all the remaining chunks;
 2317  regard their concatenated bits as one row index.  Its range has
 2318  length \(R_{\rm row}=2^{q_0K}\ge W^{k_0}\), since \(K\ge1\) for sufficiently
 2319  large \(p\) and \(q_0K\ge k_0\log_2W\).  Within each preceding prefix, add
 2320  the same number of zero rows to reach the next multiple of \(W^{k_0}\);
 2321  the volume grows
 2322  by a factor at most two.  At any recursive depth \(j\le k_0\), the
 2323  row count is divisible by \(W^{k_0-j}\).
```

```
 2393  \paragraph{Unrolling the recurrence.}
 2394  Fix \(\beta=1/2\), and choose real exponents satisfying
 2395  \[
 2396   0<\tau<1,\qquad
 2397   \max\{0,\log_m(s/W)\}\le\sigma<1,
 2398   \qquad \max\{\tau,\sigma\}<\lambda<1,
 2399   \qquad \tau(1+c/\beta)<\lambda.
 2400  \]
 2401  The exponents can all be chosen rational, with a strict upper bound
 2402  in the condition on \(\sigma\).  Stop recursion when
 2403  \(e<d^\beta\) and apply the \(e\) \(C\) kernels individually.  If
 2404  \(F(e)\) denotes time divided by the current logical stream volume,
 2405  then
 2406  \[
 2407   F(e)\le (s/W)F(e/m)+O((eK)^\tau+1),
 2408   \qquad F(e)=O(e)\quad(e<d^\beta).
```


## simultaneous-layer (core): Simultaneous normalized butterfly layer (Proposition prop:simultaneous-layer), with the explicit guard-width bound

**Draft statement.** For $D\le d$ chunks of $K=\lfloor d^c\rfloor$ bits holding polynomial records of length $r$, the layer $\mathcal H=H_0^{\otimes D}$ on one selected bit per chunk is returned as $Q_p(\mathcal Hz)$ in $O(Vd^{\lambda'})$, with error $<\sqrt2\,2^{-p}$. The whole layer is evaluated exactly in Gaussian dyadics with $\Delta=\lceil C_0d^{C_1}\rceil=o(p)$ guard bits (arithmetic depth $A(e)\le sA(e/m)+E$), so one truncation suffices.

**Role.** The second fast tape procedure; synthetic-transform calls it once per round at cost O(Tp d^{lambda'}).

**Derived at lines:** 2450-2586 (guard widths), 2686-2758 (proof)

**The paper where it is stated:**

```
 2588  \begin{proposition}[Simultaneous normalized butterfly layer]
 2589  \label{prop:simultaneous-layer}
 2590  Fix
 2591  \[
 2592   \tau=\sigma=1-2^{-50},\qquad \beta=\frac12,\qquad C_1=20.
 2593  \]
 2594  Let $c,\epsilon,\lambda,\lambda'$ be fixed positive rational numbers
 2595  satisfying
 2596  \[
 2597   \max\{\tau,\sigma\}<\lambda<1,\qquad
 2598   \tau(1+c/\beta)<\lambda,\qquad
 2599   \max\{\lambda,\sigma+\beta(1-\sigma)\}<\lambda'<1,
 2600   \qquad \epsilon C_1<1.
 2601  \]
 2602  Fix positive constants $a_d\le b_d$ and $a_r\le b_r$, and fix
 2603  positive constants $C_M,C_{\rm desc},C_w$ with $C_w>1$.
 2604  For these fixed choices
 2605  there are a cutoff $p_0$, one fixed finite-alphabet multitape procedure,
 2606  and an implicit time constant with the following guarantee. The cutoff
 2607  and time constant may depend on the fixed rational parameters
 2608  and these constants. For every choice of positive integers $p,d,r$
 2609  with $p\ge p_0$ and
 2610  \[
 2611   a_dp^\epsilon\le d\le b_dp^\epsilon,\qquad
 2612   2^{a_rp/d}\le r\le 2^{b_rp/d},
 2613  \]
 2614  put $K=\lfloor d^c\rfloor$, and let $1\le D\le d$, $P,S\ge1$, and
 2615  $0\le\rho<K$ be integers. Let $w\ge1$ be an integer component width.
 2616  The input supplies on a descriptor tape
 2617  the self-delimiting header
 2618  \[
 2619   \mathcal E=
 2620   \Gamma(p)\Gamma(d)\Gamma(r)\Gamma(D)\Gamma(P)\Gamma(S)
 2621   \Gamma(K)\Gamma(\rho+1)\Gamma(w),
 2622  \]
 2623  using the integer code from
 2624  \eqref{eq:integer-descriptor-code}. Assume that this header
 2625  is valid with the stated values and that the accompanying array is
 2626  in lexicographic order on the complete address set
 2627  \[
 2628   [P]\times[2^K]^D\times[S],
 2629   \qquad M=PS\,2^{KD},
 2630  \]
 2631  with the following record format. For every address
 2632  $x\in[P]\times[2^K]^D\times[S]$, write its record as
 2633  \[
 2634   z(x)=\sum_{j=0}^{r-1}z_{x,j}y^j,
 2635  \]
 2636  stored in coefficient order. For every $x$ and $0\le j<r$, assume
 2637  $\lvert z_{x,j}\rvert\le1$ and
 2638  $\Re z_{x,j},\Im z_{x,j}\in2^{-p}\mathbb Z$.
 2639  Each component is $2^{-p}$ times a signed $w$-bit two's-complement
 2640  integer, written most significant bit first; the real component
 2641  precedes the imaginary component. Require the pointwise bounds
 2642  \[
 2643   \log_2(2M)\le C_Mp,\qquad
 2644   |\mathcal E|\le C_{\rm desc}p,\qquad
 2645   p+2\le w\le C_wp,
 2646  \]
 2647  where $|\mathcal E|$ is the header's bit length. The condition
 2648  $w\ge p+2$ allows every permitted disk-grid component, including
 2649  $-1$ and $1$. The logical bit volume is $V=2Mrw=\Theta(Mrp)$.
 2650  
 2651  On each of the $D$ consecutive $K$-bit address chunks, select bit
 2652  $\rho$, numbered from the least significant bit, and let
 2653  $\mathcal H$ be the tensor product of
 2654  \[
 2655   H_0=\frac12\begin{pmatrix}1&1\\1&-1\end{pmatrix}
 2656  \]
 2657  on those selected bits. It acts coefficientwise and fixes every
 2658  unselected address bit and both spectator fields. Let $Q_p$ truncate
 2659  the real and imaginary parts of every coefficient toward zero to
 2660  $2^{-p}\mathbb Z$.
 2661  
 2662  The procedure returns $Q_p(\mathcal H z)$ in the same array,
 2663  coefficient order, and component format in $O(Vd^{\lambda'})$ time,
 2664  uniformly over all these
 2665  choices of $p,d,r,D,P,S,\rho,w$ and $z$. Every returned coefficient belongs
 2666  to the complex unit disk, and, in the maximum coefficient norm,
 2667  \[
 2668   \|Q_p(\mathcal H z)-\mathcal H z\|_\infty
```


## synthetic-transform (core): Synthetic transforms over C[y]/(y^r+1): character cancellation, stored bit-reversed order, and the transform layout and cost (Lemmas lem:synthetic-characters, lem:synthetic-transform-cost; eq:stored-opposite)

**Draft statement.** With $\omega_t=y^{2r/t}$, normalized $F_t^\pm$ satisfy $F_t^+F_t^-=t^{-1}I$; decimation in frequency computes $B_tF_t^-$ in bit-reversed order and the reversed rounds compute $F_t^+B_t^{-1}$. A layout permutation $L$ (prefix-slot moves and width-$K$ chunk exchanges) puts each round's bits at a common chunk offset, giving exact counterparts $LBF_{\boldsymbol t}^-$ and $F_{\boldsymbol t}^+B^{-1}L^{-1}$, error $<\sqrt2\ell2^{-p}$, cost $O(Tp[dK+pK^{\tau-1}+\ell d^{\lambda'}])$.

**Role.** The power-of-two transforms whose cost gives the first three rows of the cost table; consumed by synthetic-convolution.

**Derived at lines:** 2819-2915, 2990-3206

**The paper where it is stated:**

```
 2776  For an integer $r\geq 2$ that is a power of two, put
 2777  \[
 2778   \mathcal R_r=\mathbb C[y]/(y^r+1),\qquad
 2779   \left\|\sum_{k=0}^{r-1}z_k y^k\right\|_\infty
 2780         =\max_{0\leq k<r}|z_k|.
 2781  \]
 2782  We always use representatives of degree less than $r$.  Multiplication
 2783  by $y^a$, for any integer $a$, permutes the coefficients and changes
 2784  some signs; consequently it is an isometry for this norm.  For arrays
 2785  over $\mathcal R_r$, the same notation denotes the maximum over all
 2786  coefficients in all entries.  A disk array has norm at most one.  A
 2787  $p$-bit grid array has real and imaginary coefficient parts in
 2788  $2^{-p}\mathbb Z$.  For any array, let $Q_p$ truncate each real and
 2789  imaginary coefficient part toward zero to this grid.  Then
 2790  \begin{equation}
 2791   \|Q_p z-z\|_\infty<\sqrt2\,2^{-p},\qquad
 2792   \|Q_p z\|_\infty\leq\|z\|_\infty.
 2793   \label{eq:transform-truncate}
 2794  \end{equation}
 2795  
 2796  \subsection{Roots, butterflies, and stored frequency order}
 2797  
 2798  For a power of two $t$ dividing $2r$, define $\omega_t=y^{2r/t}$.
 2799  For $z\in\mathcal R_r^t$ and $0\leq j<t$, set
 2800  \[
 2801   (F_t^-z)_j=\frac1t\sum_{k=0}^{t-1}\omega_t^{-jk}z_k,
 2802   \qquad
 2803   (F_t^+z)_j=\frac1t\sum_{k=0}^{t-1}\omega_t^{jk}z_k.
 2804  \]
 2805  Every power of $\omega_t$ is an isometry, and each transform entry is
 2806  an average of $t$ such images.  Thus both transforms are contractions.
 2807  The ring has zero divisors, so we verify character cancellation
 2808  directly instead of dividing by $\omega_t^j-1$.
 2809  
 2810  \begin{lemma}[Synthetic character cancellation]
 2811  \label{lem:synthetic-characters}
 2812  If $j$ is an integer, then
 2813  \[
 2814   \sum_{k=0}^{t-1}\omega_t^{jk}
 2815   =\begin{cases}t,&t\mid j,\\0,&t\nmid j.\end{cases}
 2816   \qquad F_t^+F_t^-=t^{-1}I.
 2817  \]
 2818  \end{lemma}
```

```
 2929  \begin{lemma}[Transform layout and cost]
 2930  \label{lem:synthetic-transform-cost}
 2931  Fix the rational parameters and positive comparison constants
 2932  $a_d\leq b_d$, $a_r\leq b_r$ of
 2933  Proposition~\ref{prop:simultaneous-layer}. Fix constants $C_\ell>0$
 2934  and $C_w>1$, and choose the layer constants once as
 2935  \[
 2936   C_M=C_\ell+1,\qquad
 2937   C_{\rm desc}=32(C_\ell+C_w+1).
 2938  \]
 2939  Use these constants and $C_w$ in
 2940  Proposition~\ref{prop:simultaneous-layer}. These fixed choices determine
 2941  a cutoff $p_1$, one fixed-tape procedure, and an implicit time constant
 2942  with the following guarantee. For every choice of positive integers
 2943  $p,d,r$ with $p\geq p_1$ and
 2944  \[
 2945   a_dp^\epsilon\leq d\leq b_dp^\epsilon,\qquad
 2946   2^{a_rp/d}\leq r\leq2^{b_rp/d},
 2947  \]
 2948  suppose $d\geq2$ and $r=2^\ell$ with an integer $2\leq\ell<p$, and require
 2949  \[
 2950   d\ell\leq C_\ell p,\qquad
 2951   1\leq K=\lfloor d^c\rfloor\leq\ell-1.
 2952  \]
 2953  Let
 2954  $t_i\in\{2^{\ell-1},2^\ell\}$ for $1\leq i\leq d-1$, put
 2955  $M=\prod_{i=1}^{d-1}t_i$ and $T=rM$, and let $w$ be an integer with
 2956  $p+2\leq w\leq C_wp$. The cutoff and time constant are independent of
 2957  $p,d,r$, these axis choices, the round headers constructed from them,
 2958  and $w$ within the displayed bounds.
 2959  
 2960  The input supplies the self-delimiting header
 2961  \[
 2962   \mathcal T=\Gamma(p)\Gamma(d)\Gamma(r)\Gamma(K)\Gamma(w)
 2963         \prod_{i=1}^{d-1}\Gamma(t_i),
 2964  \]
 2965  where the product is concatenation in axis order and $\Gamma$ is the
 2966  integer code of \eqref{eq:integer-descriptor-code}.
 2967  The supplied $d$ determines the number of axis entries, and
 2968  $\ell=\log_2r$ is obtained from the supplied power of two $r$.
 2969  Assume that the header is valid with the stated values and that the
 2970  input records use the two's-complement component format of
 2971  Proposition~\ref{prop:simultaneous-layer} with width $w$.
 2972  
 2973  The procedure constructs an explicit positional
 2974  layout permutation $L$ of record addresses, preserving their polynomial
 2975  suffixes, and transforms disk grid arrays of $M$ contiguous
 2976  degree-less-than-$r$ polynomial records. Its exact counterpart is
 2977  $LB F_{\boldsymbol t}^-$.  Its opposite procedure consumes
 2978  that order and has exact counterpart
 2979  $F_{\boldsymbol t}^+B^{-1}L^{-1}$.  Each procedure has error less than
 2980  $\sqrt2\ell\,2^{-p}$, returns disk grid coefficients in the same
 2981  component format, and costs
 2982  \begin{equation}
 2983   O\left(Tp\left[dK+pK^{\tau-1}+\ell d^{\lambda'}\right]\right).
 2984   \label{eq:synthetic-transform-cost}
 2985  \end{equation}
 2986  Both layout construction and final restoration are charged in this
 2987  bound.  The forward procedure may retain its output layout for a
 2988  pointwise product and the following opposite procedure.
 2989  \end{lemma}
```


## signed-packing (supporting): Signed packing and normalized ring multiplication (Lemma lem:signed-ring-product)

**Draft statement.** Two disk-grid polynomials in $\mathcal R_r$ are multiplied exactly by evaluating their real and imaginary integer numerator polynomials at $B_0=2^{4p}$, making four products with the established $O(N\log N)$ multiplier, recovering coefficients by centered residues ($|h_j|<B_0/2$) and reducing modulo $y^r+1$; $Q_p(fg/r)$ is returned in $O(rp\log(rp))$.

**Role.** The pointwise products inside synthetic-convolution; cost row 'Packed polynomial products' (log(rp), power 1-epsilon). It calls the Harvey-van der Hoeven multiplier, not the new algorithm.

**Derived at lines:** 3233-3368

**The paper where it is stated:**

```
 3221  \begin{lemma}[Signed packing and normalized ring multiplication]
 3222  \label{lem:signed-ring-product}
 3223  Let $r=2^\ell<2^p$ with integers $\ell\geq1$ and $p\geq2$, and fix $C_w>1$.
 3224  Let $w$ be an integer with $p+2\leq w\leq C_wp$.
 3225  The input supplies $\Gamma(p)\Gamma(r)\Gamma(w)$ and two disk grid
 3226  polynomials $f,g\in\mathcal R_r$ in the component format of
 3227  Proposition~\ref{prop:simultaneous-layer} with width $w$.
 3228  One can compute the exact Gaussian integer
 3229  numerators of $fg$, and then return $Q_p(fg/r)$ in that same format, in time
 3230  $O(rp\log(rp))$ on a fixed number of tapes.  The output is a disk
 3231  grid polynomial with error less than $\sqrt2\,2^{-p}$ from $fg/r$.
 3232  \end{lemma}
```


## synthetic-convolution (core): Convolution from the retained transform layout (Proposition prop:synthetic-convolution)

**Draft statement.** Normalized convolution $\mathcal M_r(f,g)=(f*g)/(rM)$ over $\mathcal R_r$ is computed by two forward synthetic transforms kept in their common $LB$ order, $M$ pointwise signed-packed products, the opposite transform and an exact scale by $M$, because $P(LBu,LBv)=LB\,P(u,v)$; error $<\sqrt2M(3\ell+1)2^{-p}$, cost $O(Tp[dK+pK^{\tau-1}+\ell d^{\lambda'}+\log(rp)])$.

**Role.** The ring convolution that phase-twist turns into the complex convolution used by the chirp transform.

**Derived at lines:** 3202-3206, 3414-3495

**The paper where it is stated:**

```
 3370  \subsection{The normalized convolution interface}
 3371  
 3372  For arrays on $\prod_i\mathbb Z/t_i\mathbb Z$ with values in
 3373  $\mathcal R_r$, let $*$ denote cyclic convolution in the displayed
 3374  axes and define
 3375  \[
 3376         \mathcal M_r(f,g)=\frac{f*g}{rM}=\frac{f*g}{T}.
 3377  \]
 3378  For a fixed output coefficient, each of the $M$ first cyclic
 3379  addresses determines one second address, and each of the $r$ first
 3380  polynomial indices determines one second index modulo $r$.  The
 3381  negacyclic wrap supplies the sign.  Thus that coefficient averages
 3382  $rM$ signed scalar products, and $\mathcal M_r$ is a bilinear
 3383  contraction:
 3384  \[
 3385   \|\mathcal M_r(f,g)\|_\infty
 3386         \leq\|f\|_\infty\|g\|_\infty.
 3387  \]
 3388  If the other operand is a disk array, replacing one operand changes
 3389  the output by at most the norm distance between its two values.
 3390  
 3391  \begin{proposition}[Convolution from the retained transform layout]
 3392  \label{prop:synthetic-convolution}
 3393  Under the hypotheses of Lemma~\ref{lem:synthetic-transform-cost},
 3394  $\mathcal M_r$ on disk grid operands has an approximation with error
 3395  less than
 3396  \begin{equation}
 3397         \sqrt2\,M(3\ell+1)2^{-p}
 3398   \label{eq:synthetic-convolution-error}
 3399  \end{equation}
 3400  and cost
 3401  \[
 3402  O\left(Tp\left[dK+pK^{\tau-1}+\ell d^{\lambda'}
 3403                      +\log(rp)\right]\right).
 3404  \]
 3405  Put $m_M=\log_2M$ and $w_M=w+m_M$. The returned grid array is in the
 3406  original array and coefficient order, with the same two's-complement
 3407  component format at width $w_M$. The procedure supplies
 3408  $\Gamma(w_M)$ on the output descriptor tape, so the caller knows every
 3409  component boundary from this width and the retained shape parameters.
 3410  If one operand has norm at most $\rho<1$ and the bound in
 3411  \eqref{eq:synthetic-convolution-error} is at most $1-\rho$, the
 3412  returned grid array is a disk array.
 3413  \end{proposition}
```


## gaussian-resampling (core): Gaussian resampling with the permutations left in the transform (Lemma lem:no-sort-resampling; tensor form Lemma lem:tensor-resampling)

**Draft statement.** For coprime $s<t$ with $\theta=t/s-1>p/\alpha^4$ there are contractions $A,B_0$ with $P_sF_s=2^{2\alpha^2}B_0P_tF_tA$, where $(P_su)_j=u_{tj}$, $(P_tv)_k=v_{-sk}$ stay inside the identity instead of being executed as sorts; $\widetilde A,\widetilde B_0$ have error $<p^2$ and cost $O(tp^{3/2+\delta}\alpha)$. Tensoring gives $RF_{\boldsymbol s}=2^{\gamma}\mathcal B_0QF_{\boldsymbol t}\mathcal A$, $\gamma=2d\alpha^2$, applied linewise on the padded box with axis exposure costing $O(Tpd^2(1+\ell^\tau))$.

**Role.** Replaces transforms of prime lengths s_i by power-of-two transforms; rows 'Gaussian line maps' and part of 'CRT and axis layouts' in the cost table; the retained R and Q are removed by chirp-permutation-cancellation.

**Derived at lines:** 3576-3792, 3836-3897

**The paper where it is stated:**

```
 3548  \begin{lemma}[Resampling without executing the two permutations]
 3549  \label{lem:no-sort-resampling}
 3550  Fix $0<\delta<1/8$. Let $p>100$, $2\leq s<t<2^p$ and $2\leq\alpha<\sqrt p$
 3551  be integers, with $\gcd(s,t)=1$ and
 3552  \[
 3553   \theta:=t/s-1>p/\alpha^4.
 3554  \]
 3555  Define coordinate permutations by
 3556  \[
 3557   (P_su)_j=u_{tj},\qquad (P_tv)_k=v_{-sk}.
 3558  \]
 3559  There are linear contractions $A:\mathbb C^s\to\mathbb C^t$ and
 3560  $B_0:\mathbb C^t\to\mathbb C^s$ satisfying
 3561  \begin{equation}
 3562   P_sF_s=2^{2\alpha^2}B_0P_tF_tA.
 3563   \label{eq:no-sort-one-dimensional}
 3564  \end{equation}
 3565  There are fixed multitape algorithms for approximations
 3566  $\widetilde A:\mathbb D_p^s\to\mathbb D_p^t$ and
 3567  $\widetilde B_0:\mathbb D_p^t\to\mathbb D_p^s$, each with error less
 3568  than $p^2$ and time
 3569  \[
 3570   O(tp^{3/2+\delta}\alpha).
 3571  \]
 3572  The constant is uniform in $p,s,t,\alpha$. Inputs and outputs use the
 3573  scalar component format above and occur in increasing coordinate order.
 3574  \end{lemma}
```

```
 3800  \begin{lemma}[Tensor interface]
 3801  \label{lem:tensor-resampling}
 3802  Suppose every pair $(s_i,t_i)$, $1\leq i\leq d$, satisfies
 3803  Lemma~\ref{lem:no-sort-resampling} with the same $p,\alpha,\delta$.
 3804  Put $T=\prod_i t_i$, $\gamma=2d\alpha^2$, and
 3805  \[
 3806   R=\bigotimes_iP_{s_i},\quad Q=\bigotimes_iP_{t_i},\quad
 3807   \mathcal A=\bigotimes_i A_i,\quad
 3808   \mathcal B_0=\bigotimes_i B_{0,i}.
 3809  \]
 3810  Then
 3811  \begin{equation}
 3812   RF_{\boldsymbol s}
 3813     =2^\gamma\mathcal B_0QF_{\boldsymbol t}\mathcal A,
 3814   \qquad F_{\boldsymbol q}:=\bigotimes_iF_{q_i}.
 3815   \label{eq:no-sort-tensor}
 3816  \end{equation}
 3817  Also write $F_{\boldsymbol q}^+=\bigotimes_iF_{q_i}^+$.
 3818  Both $\mathcal A$ and $\mathcal B_0$ are contractions. The linewise
 3819  implementations of $\mathcal A$ and $\mathcal B_0$ each have error
 3820  less than $dp^2$. The total time spent in the
 3821  one-dimensional algorithms for both maps is
 3822  $O(dTp^{3/2+\delta}\alpha)$, with a constant independent of $d$.
 3823  
 3824  If each $t_i\in\{r/2,r\}$ with $r=2^\ell$, the array is kept in the
 3825  full padded box of $T$ entries, and each entry occupies $O(p)$ bits,
 3826  then exposing and restoring all the lines costs
 3827  \[
 3828   O\bigl(Tp\,d^2(1+\ell^\tau)\bigr)
 3829  \]
 3830  using the equal-width chunk-swap bound $O(Vu^\tau)$ and the fixed-digit
 3831  movement primitive. Any fixed polynomial in $p$ of setup per line is
 3832  negligible compared with $Tp$ when $r$ grows faster than every such
 3833  polynomial and $d$ is polynomially bounded in $p$.
 3834  \end{lemma}
```


## chirp-permutation-cancellation (core): Chirp and permutation cancellation (Lemma lem:chirp-permutation; eq:permuted-chirp, eq:source-permutation-cancel)

**Draft statement.** With the chirp $a_{\boldsymbol j}=\exp(-\pi i\sum_i s_ij_i^2/t_i)$, the middle factor including its target permutation is a cyclic convolution, $QF_{\boldsymbol t}u=\overline a\cdot\frac{a*(\overline a\cdot u)}{T}$ (Bluestein); and the retained source permutation cancels, $(RF_{\boldsymbol s}^+)[(RF_{\boldsymbol s}u)\cdot(RF_{\boldsymbol s}v)]=(u*v)/S^2$, with $RF^+_{\boldsymbol s}$ obtained by conjugation.

**Role.** Why neither permutation is ever executed: Q is absorbed into a convolution computed by phase-twist and synthetic-convolution, and R cancels in multiplication-algorithm.

**Derived at lines:** 3938-3988

**The paper where it is stated:**

```
 3911  \begin{lemma}[Chirp and permutation cancellation]
 3912  \label{lem:chirp-permutation}
 3913  With $R,Q,s_i,t_i$ as in Lemma~\ref{lem:tensor-resampling},
 3914  assume every $t_i$ is even. For $\boldsymbol j\in\mathbb Z^d$, define
 3915  \[
 3916   a_{\boldsymbol j}
 3917   =\exp\left(-\pi i\sum_{i=1}^d\frac{s_i j_i^2}{t_i}\right).
 3918  \]
 3919  Then $a$ has period $t_i$ in coordinate $i$, and
 3920  \begin{equation}
 3921   QF_{\boldsymbol t}u
 3922   =\overline a\mathbin{\cdot}
 3923         \frac{a*(\overline a\mathbin{\cdot}u)}{T}.
 3924   \label{eq:permuted-chirp}
 3925  \end{equation}
 3926  If $S=\prod_i s_i$, then
 3927  \begin{equation}
 3928   (RF_{\boldsymbol s}^+)
 3929     \bigl[(RF_{\boldsymbol s}u)\mathbin{\cdot}
 3930           (RF_{\boldsymbol s}v)\bigr]
 3931   =\frac{u*v}{S^2}.
 3932   \label{eq:source-permutation-cancel}
 3933  \end{equation}
 3934  Complex conjugation of the input and output of an algorithm for
 3935  $RF_{\boldsymbol s}$ gives an algorithm for $RF_{\boldsymbol s}^+$.
 3936  \end{lemma}
```


## parameters-and-primes (supporting): Fixed rational choice, sizes and prime source lengths (eq:fixed-parameters, eq:sizes, eq:gamma, eq:capacity)

**Draft statement.** $\tau=\sigma=1-2^{-50}$, $\lambda=1-2^{-52}$, $c=2^{-56}$, $\lambda'=1-2^{-54}$, $\epsilon=2^{-75}$, $\delta=1/16$, $\kappa=2^{-182}$; $b=\lceil\log_2n\rceil$, $p=6b$, $d=\lfloor b^\epsilon\rfloor$, $K=\lfloor d^c\rfloor$, $T\in[4n/b,8n/b)$ with $t_i\in\{r/2,r\}$, so $Tp=\Theta(n)$, $r=2^{\Theta(p^{1-\epsilon})}$. Distinct odd primes $(1-2\eta)t_i<s_i\le(1-\eta)t_i$, $\eta=1/(4d)$, exist (Rosser-Schoenfeld via HvdH Lemma 5.1), are found in $n^{o(1)}$ time, give $T/2<S<T$ and $\alpha^4\theta_i>p$.

**Role.** Makes every procedure's hypotheses hold simultaneously and fixes the exponents in time-accounting; digit polynomials have degree < S so there is no wraparound.

**Derived at lines:** 4016-4036, 4073-4088, 4103-4193

**The paper where it is stated:**

```
 4004  \subsection{A fixed rational choice}
 4005  
 4006  The two motif exponents were fixed in \eqref{eq:explicit-motif-exponents}.
 4007  Take the following fixed rational numbers:
 4008  \begin{equation}\label{eq:fixed-parameters}
 4009  \begin{gathered}
 4010   \tau=\sigma=1-2^{-50},\qquad \beta=\frac12,\qquad \delta=\frac1{16},\\
 4011   \lambda=1-2^{-52},\qquad c=2^{-56},\qquad
 4012   \lambda'=1-2^{-54},\\
 4013   C_1=20,\qquad \epsilon=2^{-75},\qquad \kappa=2^{-182}.
 4014  \end{gathered}
 4015  \end{equation}
```

```
 4049  \subsubsection{The padded box and working precision}
 4050  
 4051  After obtaining $n$, compute the size parameters below with the
 4052  separate setup costs stated in this subsection.  For sufficiently
 4053  large $n$, put
 4054  \[
 4055   b=\lceil\log_2n\rceil,\quad p=6b,\quad d=\lfloor b^\epsilon\rfloor,
 4056   \quad K=\lfloor d^c\rfloor.
 4057  \]
 4058  Take the power of two $T$ in $[4n/b,8n/b)$, and let
 4059  $r=2^{\lceil(\log_2T)/d\rceil}$ and $\ell=\log_2r$.  Choose
 4060  $d\ell-\log_2T$ of the $d$ axis lengths to be $r/2$ and the others to
 4061  be $r$.  This gives
 4062  \[
 4063   t_i\in\{r/2,r\},\qquad \prod_i t_i=T,\qquad t_d=r.
 4064  \]
 4065  Indeed $d\ell-\log_2T$ is an integer in $[0,d)$, so at least one axis
 4066  has length $r$; choose it as the last axis.
 4067  These definitions give
 4068  \begin{equation}\label{eq:sizes}
 4069   Tp=\Theta(n),\quad d=\Theta(p^\epsilon),\quad
 4070   K=\Theta(p^{\epsilon c}),\quad
 4071   \ell=\Theta(p^{1-\epsilon}),\quad r=2^{\Theta(p^{1-\epsilon})}.
 4072  \end{equation}
```

```
 4089  Set
 4090  \[
 4091   \alpha=\left\lceil(12d^2b)^{1/4}\right\rceil,
 4092   \qquad \gamma=2d\alpha^2,\qquad \eta=\frac1{4d}.
 4093  \]
 4094  The ceiling does not affect the needed bounds: for $A\ge1$,
 4095  $\lceil A\rceil\le2A$, and hence
 4096  \begin{equation}\label{eq:gamma}
 4097   \alpha\le2(12d^2b)^{1/4},\qquad
 4098   \gamma<28d^2\sqrt b\le28b^{1/2+2\epsilon}\le28b^{2/3}.
 4099  \end{equation}
 4100  It follows that $2\le\alpha<\sqrt p$ eventually and $\gamma=o(b)$.
 4101  For example $b\ge2^{24}$ implies $\gamma\le b/4$.
```

```
 4135   (1-2\eta)t_i<s_i\leq(1-\eta)t_i.
```


## crt-address-map (supporting): Chinese-remainder coefficient-address map Phi (subsection 'An explicit cyclic-convolution layout'; eq:triangular-crt)

**Draft statement.** $\Phi:x\mapsto\prod_ix_i^{\mu_i}$, $\mu_i=P_i^{-1}\bmod s_i$, is an algebra isomorphism $\mathbb C[x]/(x^S-1)\to\bigotimes_i\mathbb C[x_i]/(x_i^{s_i}-1)$, so it preserves cyclic convolution; on mixed-radix digits it is triangular, $b_i=a_i+\mu_i\sum_{j<i}a_jP_j\pmod{s_i}$, implemented by prefix-controlled cyclic shifts in the padded box, with axis reversal by chunk swaps, in $O(Tpd^2(1+\ell^\tau))$.

**Role.** First reduction: from one cyclic convolution of length S to a convolution on d distinct prime axes, the input shape for gaussian-resampling; cost row 'CRT and axis layouts'.

**Derived at lines:** 4216-4296

**The paper where it is stated:**

```
 4204  Let $s_1,\ldots,s_d$ be the pairwise coprime
 4205  source lengths, let $S=\prod_i s_i$, and put
 4206  \[
 4207   P_i=\prod_{j<i}s_j,\qquad \mu_i=P_i^{-1}\pmod{s_i}.
 4208  \]
 4209  Here $P_i$ denotes an integer prefix product, not a Fourier-coordinate
 4210  permutation.  The assignment
 4211  \[
 4212   \Phi:\mathbb C[x]/(x^S-1)\longrightarrow
 4213   \mathbb C[x_1,\ldots,x_d]/(x_1^{s_1}-1,\ldots,x_d^{s_d}-1),
 4214   \qquad x\longmapsto\prod_i x_i^{\mu_i}
 4215  \]
 4216  respects $x^S=1$, so it defines an algebra map.  The target monomial
 4217  group has size $S$, and the image of $x$ has order $S$, because each
 4218  $\mu_i$ is a unit and the $s_i$ are pairwise coprime.  Its powers
 4219  therefore run through all target basis monomials exactly once.  Hence
 4220  $\Phi$ is an isomorphism and preserves cyclic convolution.
 4221  
 4222  Every ordinary coefficient index has a unique mixed-radix expression
 4223  \[
 4224   k=\sum_{j=1}^d a_jP_j,\qquad 0\le a_j<s_j.
 4225  \]
 4226  The coordinate of $\Phi(x^k)$ in axis $i$ is
 4227  \begin{equation}\label{eq:triangular-crt}
 4228   b_i=a_i+\mu_i\sum_{j<i}a_jP_j\pmod{s_i}.
```


## phase-twist (supporting): Complex convolution by a coefficient twist (subsection 'Complex convolution by a coefficient twist')

**Draft statement.** $\Theta:x_d\mapsto\zeta y$, $\zeta=e^{\pi i/r}$, is an isometric isomorphism $\mathbb C[x_d]/(x_d^r-1)\to\mathbb C[y]/(y^r+1)$ (the negative wrap $\zeta^{k+l}(-1)^h=\zeta^j$ reproduces the cyclic wrap), so $\mathcal M_{\mathbb C}(f,g)=D_-\iota^{-1}\mathcal M_r(\iota D_+f,\iota D_+g)=(f*g)/T$; the computed version adds twist errors below $4\cdot2^{-p}$ each.

**Role.** Connects the chirp convolution on power-of-two axes to the ring convolution of synthetic-convolution.

**Derived at lines:** 4349-4413

**The paper where it is stated:**

```
 4298  \subsection{Complex convolution by a coefficient twist}
 4299  
 4300  The power-of-two complex convolution used in the chirp identity is
 4301  obtained from the synthetic convolution by a coefficient twist.
 4302  Put $\mathcal R=\mathcal R_r=\mathbb C[y]/(y^r+1)$ and
 4303  $\zeta=e^{\pi i/r}$.  The map
 4304  \[
 4305   \Theta:\mathbb C[x_d]/(x_d^r-1)\longrightarrow\mathcal R,
 4306   \qquad x_d\longmapsto\zeta y
 4307  \]
 4308  is well defined, since $(\zeta y)^r=(-1)(-1)=1$.  On the bases
 4309  $(x_d^k)_{0\le k<r}$ and $(y^k)_{0\le k<r}$ it is diagonal,
 4310  multiplying coefficient $k$ by the nonzero number $\zeta^k$.  It is therefore an
 4311  isomorphism, with inverse multiplying that coefficient by
 4312  $\zeta^{-k}$.  Both exact maps preserve the coefficient sup norm.
 4313  For an explicit check on the wrap, write $k+l=j+hr$, where
 4314  $h\in\{0,1\}$.  A product term after twisting and negacyclic reduction
 4315  has factor
 4316  \[
 4317   \zeta^{k+l}(-1)^h=\zeta^j(\zeta^r)^h(-1)^h=\zeta^j.
 4318  \]
 4319  Untwisting therefore recovers the cyclic coefficient with the correct
 4320  sign.  We next specify how this linewise map meets the ring-convolution
 4321  interface.  Put $M=\prod_{i<d}t_i=T/r$.  For a complex array $f$ on
 4322  $\prod_{i=1}^d\mathbb Z/t_i\mathbb Z$, with $t_d=r$, define
 4323  \[
 4324   (\iota f)_{\boldsymbol j}
 4325     =\sum_{k=0}^{r-1}f_{\boldsymbol j,k}y^k,
 4326   \qquad
 4327   \boldsymbol j\in\prod_{i<d}\mathbb Z/t_i\mathbb Z.
 4328  \]
 4329  This identifies the scalar tensor with an array of $M$ records in
 4330  $\mathcal R$. As maps of values, $\iota$ and $\iota^{-1}$ preserve norms
 4331  and grids. Write
 4332  $(D_\pm f)_{\boldsymbol j,k}=\zeta^{\pm k}f_{\boldsymbol j,k}$.
 4333  The linewise extension of $\Theta$ is $\iota D_+$, with inverse
 4334  $D_-\iota^{-1}$.  Consequently, with $\mathcal M_r$ from
 4335  Section~\ref{sec:exact-ring-products}, normalized complex cyclic
 4336  convolution is
 4337  \[
 4338   \mathcal M_{\mathbb C}(f,g)
 4339     :=D_-\iota^{-1}
 4340         \mathcal M_r(\iota D_+f,\iota D_+g)
 4341      =\frac{f*g}{T}.
 4342  \]
 4343  Here $\mathcal M_r$ convolves the first $d-1$ axes over $\mathcal R$,
 4344  whereas $*$ on the right convolves all $d$ scalar axes.  The division
 4345  by $T$ is already part of $\mathcal M_r$.  The maps $D_\pm$ and
 4346  $\iota^{\pm1}$ are isometries, so $\mathcal M_{\mathbb C}$ is also a
 4347  bilinear contraction.
```


## multiplication-algorithm (core): Source-transform procedure and the six-step multiplication algorithm (subsections 'Chirp and source-transform procedures', 'The multiplication algorithm')

**Draft statement.** The source transform $\widetilde{\mathscr F}$ applies linewise $\widetilde{\mathcal A}$, the computed chirp transform (twist, complex convolution, untwist) and linewise $\widetilde{\mathcal B}_0$, then scales by $2^\gamma$; its exact counterpart is $\mathscr F=2^\gamma\mathcal B_0QF_{\boldsymbol t}\mathcal A=RF_{\boldsymbol s}$, and $\widetilde{\mathscr F}^+(z)=\overline{\widetilde{\mathscr F}(\overline z)}$. The algorithm: digits scaled by $2^{-b-2}$, $\Phi$, two source transforms, pointwise product, $\widetilde{\mathscr F}^+$, scales by $S$ and $2^{2b+4}S$, rounding, $\Phi^{-1}$, carries.

**Role.** Assembles the reductions of fig:multiplication-flow; the two occurrences of S undo the normalization (u*v)/S^2 from chirp-permutation-cancellation.

**Derived at lines:** 4486-4544

**The paper where it is stated:**

```
 4415  \subsection{Chirp and source-transform procedures}
 4416  
 4417  We next use the complex convolution to evaluate the middle factor
 4418  $QF_{\boldsymbol t}$ in \eqref{eq:no-sort-tensor}, and then compose it
 4419  with the Gaussian maps. For the chirp array $a$ in
 4420  \eqref{eq:permuted-chirp}, the phase numerator
 4421  at tensor address $j$ is computed modulo $2r$ as
 4422  \[
 4423   -\sum_i s_i j_i^2(r/t_i)\pmod{2r}.
 4424  \]
 4425  All integers have $O(p)$ bits and $r/t_i\in\{1,2\}$.  Scan the stored
 4426  axis descriptors and compute the $d$ squares and products with the
 4427  previously established integer multiplier.  Additions, reductions
 4428  modulo a power of two, and restarting local counters cost $O(p)$ per
 4429  term; multiplication costs $O(p\log p)=O(p^{1+\delta})$.  Exponential
 4430  evaluation then has the same $O(p^{1+\delta})$ bound.  Thus chirp
 4431  construction costs $O(Td p^{1+\delta})$ in total.  In particular, this
 4432  cost includes the scan of the axis descriptors at every coefficient.
 4433  
 4434  Let $\widetilde a$ be the resulting disk-grid array, so
 4435  $\|\widetilde a-a\|_\infty<2\,2^{-p}$.  For a disk-grid target tensor
 4436  $x$ of norm at most $.6$, the computed chirp transform forms the arrays
 4437  \[
 4438   \begin{aligned}
 4439   x_0&=Q_p(\overline{\widetilde a}\mathbin{\cdot}x),\\
 4440   y_0&=\widetilde{\mathcal M}_{\mathbb C}(\widetilde a,x_0),\\
 4441   \widetilde{\mathscr C}(x)
 4442       &=Q_p(\overline{\widetilde a}\mathbin{\cdot}y_0).
 4443   \end{aligned}
 4444  \]
 4445  Thus $\widetilde a$ is the convolution kernel and $x_0$ is the data
 4446  operand after the first endpoint phase.  Replacing the approximations
 4447  by exact phases and exact convolution gives
 4448  $\overline a\mathbin{\cdot}\mathcal M_{\mathbb C}
 4449  (a,\overline a\mathbin{\cdot}x)=QF_{\boldsymbol t}x$ by
 4450  \eqref{eq:permuted-chirp}.  Below we show that $\|x\|_\infty\le.6$
 4451  implies $\|x_0\|_\infty\le.6$, which supplies the smaller operand
 4452  required by the computed convolution.
 4453  
 4454  \subsubsection{The source transform}
 4455  
 4456  For a source disk-grid tensor of norm at most $.5$, write
 4457  $\widetilde{\mathscr F}$ for the following procedure: apply the linewise
 4458  approximation to $\mathcal A$, apply the
 4459  computed chirp transform $\widetilde{\mathscr C}$, apply the linewise
 4460  approximation to $\mathcal B_0$, and multiply the grid numerators
 4461  exactly by $2^\gamma$.  Its exact counterpart is
 4462  \[
 4463   \mathscr F=2^\gamma\mathcal B_0QF_{\boldsymbol t}\mathcal A
 4464              =RF_{\boldsymbol s}.
 4465  \]
 4466  Here an exact counterpart replaces every numerical subroutine by the
 4467  exact map it approximates and omits truncations.  The source arrays are
 4468  stored in the padded power-of-two box, with zeros outside the valid
 4469  source coordinates.
 4470  The compressed scalar output uses $w_{\rm c}$-bit components. Appending
 4471  $\gamma$ zero bits to each word gives the exact scaled numerator in the
 4472  known signed width $w_{\rm c}+\gamma$. For the stated input norm, the
 4473  disk estimate below proves that every scaled coordinate still belongs to
 4474  $\mathbb D_p$. Its component numerators therefore have magnitude at most
 4475  $2^p$, so a scan removes only leading sign-extension bits and returns
 4476  the same integers in $w_{\rm c}$ bits. This scan costs
 4477  $O(T(w_{\rm c}+\gamma))=O(Tp)$ and leaves the padded zero records zero.
 4478  Define the opposite-sign procedure by
 4479  \[
 4480   \widetilde{\mathscr F}^{+}(z)
 4481     =\overline{\widetilde{\mathscr F}(\overline z)};
 4482  \]
 4483  its exact counterpart is $RF_{\boldsymbol s}^{+}$ by
 4484  Lemma~\ref{lem:chirp-permutation}.
```

```
 4506  With these streams prepared, perform the following operations.
 4507  \begin{enumerate}
 4508  \item Apply the coefficient-address map $\Phi$, with its padded layout, to obtain
 4509  source arrays $u,v$.  Their coefficients are exact $p$-bit grid
 4510  values of norm less than $1/4$.
 4511  \item Compute $\widetilde{\mathscr F}(u)$ and
 4512  $\widetilde{\mathscr F}(v)$.
 4513  \item Multiply corresponding complex coefficients exactly and truncate
 4514  each real and imaginary part toward zero to $p$ fractional bits, writing
 4515  the result in the common $w_{\rm c}$-bit component format.
 4516  Denote the resulting source array by $z$.
 4517  \item Compute $\widetilde{\mathscr F}^{+}(z)$ and multiply its grid
 4518  numerators exactly by $S$, retaining the denominator $2^p$ and writing
 4519  each component in signed width $w_{\rm c}+b$. Denote the represented
 4520  dyadic array by $\widetilde w$.
 4521  \item Multiply the grid numerators of $\widetilde w$ exactly by
 4522  $2^{2b+4}S$, again retaining the denominator $2^p$, and write each
 4523  component in signed width $w_{\rm c}+4b+4$. Round the represented real
 4524  values to the nearest integers. At each valid address, write the rounded
 4525  coefficient in one signed $(3b+1)$-bit word, most significant bit first;
 4526  at every padded address,
 4527  write a zero word of that width. Supply this local payload width to
 4528  $\Phi^{-1}$ and apply it, including removal of the address padding, to
 4529  restore the ordinary coefficient order.
 4530  \item Propagate radix-$2^b$ carries in increasing coefficient order.
 4531  Write the resulting bits from least to most significant on a work tape,
 4532  then reverse that stream and output exactly $2n$ bits in
 4533  most-significant-first order.
 4534  \end{enumerate}
```


## exact-recovery (core): Precision and exact recovery (subsection 'Precision and exact recovery'; eq:source-error, eq:final-error)

**Draft statement.** Error bounds chain through contractions with disk margins: ring convolution $E_{\rm ring}<5TL$, chirp $<8TL$, source transform $E_s=9\cdot2^\gamma TL$, then $E_w=S(3E_s+2)<28\cdot2^\gamma T^2L$; after the scales the coefficient error is $<448b\,2^{-b+\gamma}\le448b\,2^{-3b/4}<\tfrac12$ (using $\gamma\le b/4$), so rounding gives the exact integers $0\le c_j<2^{3b}$, and radix-$2^b$ carry propagation gives the $2n$-bit product.

**Role.** Proves the output is exactly correct, which the main theorem asserts for every input; this is where p=6b and gamma=o(b) are needed.

**Derived at lines:** 4546-4806

**The paper where it is stated:**

```
 4651  \begin{equation}\label{eq:source-error}
 4652   E_s=9\,2^\gamma TL,
 4653  \end{equation}
 4654  because $2dp^2<TL$ eventually.  The exact source transform
```

```
 4708  \[
 4709   E_w:=S(3E_s+2)<28\,2^\gamma T^2L.
```

```
 4723  \begin{equation}\label{eq:final-error}
 4724   2^{2b+4}S\cdot28\,2^\gamma T^2L\cdot2^{-6b}
 4725   <448b\,2^{-b+\gamma}
 4726   \le448b\,2^{-3b/4}<\frac12.
 4727  \end{equation}
```


## time-accounting (core): The complete time bound (subsection 'The complete time bound'; Table tab:costs, eq:margin-list)

**Draft statement.** With $V=Tp=\Theta(n)$, every cost divided by $V$ is a power of $p$: $dK$, $pK^{\tau-1}$, $\ell d^{\lambda'}$, $d^2(1+\ell^\tau)$, $dp^{1/2+\delta}\alpha$, $dp^\delta$, $\log(rp)$; their margins below one are $g_1..g_7$ with $\min_ig_i=g_2=\epsilon c(1-\tau)=2^{-181}=2\kappa$. Setup and exceptional-address sorting are $o(V)$, any fixed power of $\log p$ is $\le p^\kappa$, so the time is $O(Tp\,p^{1-\kappa})=O(n(\lg n)^{1-\kappa})$, with schoolbook multiplication below a cutoff.

**Role.** The final step to Theorem thm:main.

**Derived at lines:** 4857-4995

**The paper where it is stated:**

```
 4815  \begin{table}[ht]
 4816  \centering
 4817  \small
 4818  \begin{tabular}{@{}>{\raggedright\arraybackslash}p{.40\linewidth}ll@{}}
 4819  \toprule
 4820  Operation & Normalized cost & Power of $p$\\
 4821  \midrule
 4822  Prefix-slot moves and individual butterfly rounds & $dK$ & $\epsilon(1+c)$\\
 4823  Chunk exchanges for transform layout & $pK^{\tau-1}$ & $1-\epsilon c(1-\tau)$\\
 4824  Simultaneous butterfly rounds & $\ell d^{\lambda'}$ & $1-\epsilon(1-\lambda')$\\
 4825  CRT and axis layouts & $d^2(1+\ell^\tau)$ & $\tau+\epsilon(2-\tau)$\\
 4826  Gaussian line maps & $dp^{1/2+\delta}\alpha$ & $3/4+\delta+3\epsilon/2$\\
 4827  Chirps, twists and scalar products & $dp^\delta$ & $\epsilon+\delta$\\
 4828  Packed polynomial products & $\log(rp)$ & $1-\epsilon$\\
 4829  Final scaling and rounding & $p^\delta$ & absorbed above\\
 4830  Input, padding, carries and output & $1$ & absorbed above\\
 4831  \bottomrule
 4832  \end{tabular}
 4833  \caption{Costs divided by $Tp$; each coefficient
 4834  representation has $O(p)$ bits.}\label{tab:costs}
 4835  \end{table}
 4836  
 4837  The seven dominant powers in Table~\ref{tab:costs} have the following
 4838  margins below one:
 4839  \begin{equation}\label{eq:margin-list}
 4840  \begin{split}
 4841   g_1&=1-\epsilon(1+c),\qquad
 4842   g_2=\epsilon c(1-\tau),\qquad
 4843   g_3=\epsilon(1-\lambda'),\\
 4844   g_4&=1-\tau-\epsilon(2-\tau),\qquad
 4845   g_5=1/4-\delta-3\epsilon/2,\\
 4846   g_6&=1-\delta-\epsilon,\qquad g_7=\epsilon.
 4847  \end{split}
 4848  \end{equation}
 4849  Direct substitution gives
 4850  \[
 4851   g_1>\frac12,\quad g_2=2^{-181},\quad g_3=2^{-129},\quad
 4852   g_4>2^{-51},\quad g_5>\frac18,\quad g_6>\frac12,\quad g_7=2^{-75}.
 4853  \]
 4854  Thus $\min_i g_i=2^{-181}=2\kappa$.  This explicit slack will cover
```


## conjecture-negation (supporting): Why the bound disproves the n log n conjecture in this model ('Lower-bound scope and matrix transposition')

**Draft statement.** An eventual lower bound would require $T_A(n)\ge c_An\lg n$ for all large $n$ for every machine; one machine with $\liminf T_A(n)/(n\lg n)=0$ refutes it, and $O(n(\lg n)^{1-\kappa})$ gives limit zero along all lengths.

**Role.** Turns the time bound into the 'disproving' clause of the wiki's target.

**Derived at lines:** 379-387 (intro only; no body derivation)

**The paper where it is stated:**

```
  379  Sch\"onhage and Strassen conjectured $n\log n$ as the optimal order of
  380  multiplication cost~\cite[Section~1]{SchonhageStrassen1971}. In the present
  381  fixed-tape model, an eventual lower bound for every multiplication machine means that, for
  382  each fixed machine $A$, there are constants $c_A>0$ and $N_A$ such that
  383  $T_A(n)\ge c_A n\lg n$ whenever $n\ge N_A$.
  384  Its negation requires one machine satisfying
  385  $\liminf_{n\to\infty}T_A(n)/(n\lg n)=0$.
  386  The power saving proved here gives a limit of zero through all input lengths,
  387  which is a stronger conclusion.
```

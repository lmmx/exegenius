Matrix shear by triangular factorization adds a fixed rational matrix times one group of fields to another group while spending only as many field interchanges as the matrix has rank.

Let $H=(H_1,\dots,H_m)$ and $D=(D_1,\dots,D_m)$ be two groups of fields, $H$ before $D$, where $m$ is the number of fields in each group ($m$ counts fields here; it is not an output index). Let $A$ be a rational $m\times m$ matrix. Here $q$ is a fixed odd prime, the base of the digits, not a sequence length. Here $b$ is the number of base-$q$ digits in one field, not the lowest bit of an index. Each field is a residue modulo $q^b$. Here $a$ is the rank of $A$, not an offset added to a field. The shear is $(H,D)\mapsto(H+AD,D)$. For $A$ in a fixed finite set, it can be performed with exactly $a$ interchanges of $b$-digit fields and $O(V)$ further work <cite data-source="paper">can be performed with exactly $a$ interchanges of pairs of $b$-digit</cite>. Lower triangular changes of coordinates are cheap, so all the rank of $A$ is pushed into a partial permutation.

## Factoring

Every such $A$ factors as $A=E_1\Pi E_2$, where $E_1,E_2$ are invertible lower triangular and $\Pi$ has exactly $a$ ones, at most one per row and column <cite data-source="paper">Every rational $m\times m$ matrix $A$ of rank $a$ has a factorization</cite>. Elimination finds it. Take the topmost nonzero row and its rightmost nonzero entry, the pivot. Right multiplication by lower triangular elementary matrices clears the row to the left of the pivot, and left multiplication by such matrices clears the column below it; scaling makes the pivot one. Delete the pivot row and column and repeat. The pivots become the ones of $\Pi$, and their number is the rank of $A$, because $E_1,E_2$ are invertible and so do not change rank.

## Reducing modulo $q^b$

Choose the factorization of every matrix in the finite set once and for all, and list the factors and the inverses of $E_1$ and $E_2$ in one finite table of rational numbers. A fraction whose denominator is not divisible by $q$ is meaningful modulo $q^b$, since that denominator is a unit there. If $q$ avoids every denominator in the table, every entry reduces modulo $q^b$, and identities such as $A=E_1\Pi E_2$ still hold, since reduction preserves sums and products. A triangular matrix is invertible exactly when its diagonal entries are units, and a fraction is a unit modulo $q^b$ when $q$ divides neither its numerator nor its denominator. Hence choose $q$ to avoid every denominator in the table and every numerator of a diagonal entry of an invertible factor, so that the identities specialise to $\mathbb Z/q^b\mathbb Z$ and the triangular factors remain invertible. Any prime larger than all nonzero numerators and denominators in the table works.

## Triangular parts

Each lower triangular transformation of $H$ or of $D$ is carried out by [Controlled shifts and rational scalings](controlled-shifts.md). Update coordinates in descending physical order. Coordinate $i$ becomes its diagonal unit times itself plus a fixed combination of earlier coordinates, which still hold their original values. With $m$ fixed, this costs $O(V)$. Inverses are also lower triangular.

## Shearing

Transform $D$ by $E_2$ and $H$ by $E_1^{-1}$, add $\Pi D$ to $H$, then transform $H$ by $E_1$ and $D$ by $E_2^{-1}$. The net effect on $H$ is $E_1(E_1^{-1}H+\Pi E_2D)=H+AD$, and $D$ is restored.

Each one of $\Pi$, in row $i$ and column $j$, calls for $H_i\gets H_i+D_j$. Do it by $D_j\gets D_j+H_i$, then interchange $(H_i,D_j)$, then $D_j\gets H_i-D_j$. The first and last updates target a later field controlled by an earlier one, so they are controlled shifts. For example, if $H_i=5$ and $D_j=3$, the pair becomes $(5,8)$, then $(8,5)$ after the interchange, then $(8,3)$.

This uses one interchange per one of $\Pi$, and $\Pi$ has $a$ ones, so exactly $a$ interchanges are used. Everything else lies within the $O(V)$ further work, so temporary tapes do not multiply the count.

The established multiplier is the earlier Harvey–van der Hoeven algorithm, which multiplies two integers of $N$ bits each in $O(N\log N)$ time (this $N$ is not the bank size of the exchange network); the new algorithm calls it as an ordinary subroutine for the integer products it cannot avoid.

The multiplier runs in the [Fixed finite-tape machine model](machine-model.md), the same model as the main result, so a call costs its $O(N\log N)$ steps with no change of machine to account for. The paper takes this bound from Harvey and van der Hoeven <cite data-source="paper">We use the established $O(N\log N)$ integer multiplier of</cite>; its proof is in their paper and is not reproduced here. It is the bound that Schönhage and Strassen conjectured optimal ([Schönhage–Strassen optimality conjecture](conjecture.md)), so it is the benchmark the main result beats.

## How it is used

The multiplier is not the source of the saving. It is called on two kinds of product.

The multiplier is called on the first kind once the new algorithm reduces its [Fourier transforms](dft.md) to polynomial products over $\mathbb C[y]/(y^r+1)$. These ring products go through signed packing <cite data-source="paper">use signed packing and the established $O(n\log n)$ multiplier as an</cite>: [Signed packing](signed-packing.md) turns each into four integer products on $O(rp)$ bits, where $r$ is the record length and $p$ the number of fractional bits.

The second kind is the chirp numerators, which have $O(p)$ bits and are squared and multiplied directly <cite data-source="paper">Computing their squares and products with the previously established $O(N\log(2N))$ multiplier</cite>.

## Why the calls add no recursion

The new algorithm has recursion of its own, in its layer and address-layout recursions. The multiplier calls contribute none of it, for two reasons.

First, each call is an ordinary subroutine call on records of $O(rp)$ bits, fewer than the input, so it is not a call of the improved algorithm on a smaller input <cite data-source="paper">they do not call the improved algorithm being constructed.</cite> Hence no recurrence for the new algorithm passes through the multiplier.

Second, whatever recursion the Harvey–van der Hoeven algorithm uses internally is already paid for by its $O(N\log N)$ bound <cite data-source="paper">The internal recursion of the established integer</cite>. Its cost is therefore charged as a separate term, not folded into a recurrence <cite data-source="paper">already included in their separately charged running times.</cite>

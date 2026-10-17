The fixed finite-tape machine model is the multitape Turing machine in which all running times of the main theorem are measured; fixing the model is what gives the statement "multiplication in fewer than $n\log n$ steps" a precise meaning.

A machine has a single finite alphabet and a finite number of one-dimensional tapes, both fixed independently of the input <cite data-source="paper">A machine has one fixed finite alphabet and one fixed finite number of one-dimensional tapes.</cite>. Each tape is a one-way or two-way infinite line of cells, each holding one symbol, with one head per tape. In one step the machine reads and writes at most one cell on each tape and moves each head by at most one cell. Time is the number of steps.

Two consequences matter for the rest of the wiki.

**Constant factors are free, structure is not.** Splitting each cell into a fixed number of tracks, or adding finitely many work tapes, changes the running time only by a fixed factor <cite data-source="paper">Fixed tracks and finitely many work tapes change time by a fixed factor.</cite> Since the target bound is an exponent saving, not a constant improvement, such factors do not matter, and an algorithm may keep several arrays side by side on separate tapes or tracks.

**Data movement costs time.** A head crosses one cell per step, so reaching a distant cell costs the distance. A linear pass over an array is cheap, but an arbitrary rearrangement of an array is not. The pages on [elementary stream operations](stream-ops.md) list the scans that cost linear time, and every such scan is charged for its counters and for returning its heads. Moving one digit in a fixed radix takes a bounded number of scans, whereas interchanging fields whose digit counts grow is a genuine permutation problem, handled in [fast address-chunk interchange](address-interchange.md).

The model is the one in which the [Schönhage–Strassen optimality conjecture](conjecture.md) was posed, and it is the setting of the [main result](main-result.md). It is also the setting in which the earlier [established $O(n\log n)$ multiplier](hvdh-multiplier.md) runs, so that algorithm can be used here as an ordinary subroutine.

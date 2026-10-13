# 6. Adapting the work to the source

## One reader or several

Haiku is known to drop information on large tasks like this one, and that is something I had seen from it before.

**The idea:** fan out. Split the reading across several copies of a subagent. Rather than a fixed task, give the planner an adaptive one: it can come back and say "give this work to more copies of the subagent". The same pipeline would then suit both a single short source and a 70-page frontier mathematics paper.

**What splitting costs:**
- **Interference.** With one agent doing the extraction, nothing treads on its feet. Seventy separate calls would take longer, and their findings would overlap.
- **Integration.** The overlapping findings have to be merged into one coherent graph and argument.

**Status:** hypothesis, not a decision. I have not thought this design through: I was trying to get the first step done.

## The question this note leaves open

ChatGPT framed the options:
- a single planner reading systematically, section by section;
- a planner that delegates the reading of sections;
- a first pass that judges the source and chooses between those two.

The order matters. The immediate failure was a planner that chose not to read past line 440, though it had the outline. A single planner told to read section by section may be enough to fix it, and that should be tried before splitting the work. Fan-out matters only if a single reader cannot cover the source.

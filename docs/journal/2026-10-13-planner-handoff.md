# 2026-10-13: What the plan hands the writer

2026-10-13-planner-agent routes the planning tasks to Sonnet. Before approving a run, ChatGPT and the author asked for two things:
- the journal should separate what that change does from the design it is a step towards;
- an inspection of how much of the planner's reasoning reaches the Haiku writer, with the butterfly layer as the example.

No code changed for this entry.

## The change and the design

**What is built: a stronger planner.** Sonnet does extraction, background, argument and extension. Haiku writes and audits. Sonnet's reasoning reaches the writer only through what the write task already carries from the graph and the argument.

**What the author proposed: a supervisor.** A larger model passes its reasoning on as guidance the cheaper worker can carry out, and corrections help the worker converge. ChatGPT set this out as four roles, which need not be four agents:
1. **Planner, or supervisor (Sonnet at first):**
   - resolves the conceptual problem;
   - decides where each derivation belongs and what it depends on;
   - writes a brief with the mathematical steps and their source evidence.
2. **Worker (Haiku):** writes the page from the brief, and flags a contradiction or missing evidence rather than following an inadequate brief.
3. **Reviewer (Haiku at first, a larger model where justified):** names the exact unsupported step and the smallest correction that fixes it, and rejects only when a targeted repair is not enough.
4. **Improvement loop:** repeated failures change the planner's instructions, the brief's format, the graph rules and the validation, not only the page.

None of this is built. The experiments stay separate, so that each result can be read on its own:
1. Does Sonnet planning improve the wiki, with the depth policy unchanged? This is the run proposed in 2026-10-13-planner-agent.
2. Does an explicit brief improve Haiku's pages over the plan as it is handed over now?
3. Do targeted corrections reduce rejections and rewrite cycles without loss of correctness?

## The handoff, inspected

**Method.**
- Took a copy of the first Sonnet planning sample from 2026-10-13-model-comparison: its graph, background and argument.
- Ran `exegenius next` on the copy. It handed out all 45 write tasks.
- Compared the task for `simultaneous-butterfly-layer` (0039, 3,496 words) with that page's subtree of `argument.json`.

**What the write task carries.** Under "This page in the argument", from `write.rs`'s `argument_section`:
- **For the page's own step S3:**
  - the claim;
  - the method: "Compute the tensor power of the kernel $C$ with the complex bank-exchange network on selected bits, implement the basis changes by packed whole-chunk shifts, and bound the exact dyadic values";
  - the motivation, with its quote ("factors would require $D$ whole-array passes");
  - the striking feature: $d^{\lambda'}$ passes rather than $d$.
- **For each sub-step:** its claim in one line, and the page that presents it, where one does:
  - S3.1 on `phase-edge-recursion`;
  - S3.2 on `packed-selected-bit-addition`;
  - S3.4 on `exact-dyadic-guard`;
  - S3.3 and S3.5 with no page.
- **The chain above it:** the claim and method of the main result.

**What the plan holds and the task does not:**

| In the plan | In the write task |
|---|---|
| S3.3, the recursion, has a method: "Supply rows from the first few chunks, apply the recursion-saving recurrence of the complex network, and sum over a base-$m$ partition of the chunks" | only S3.3's claim |
| S3.3's three inferred steps: a row index $u = Wg + w$ goes to role $w$, so each child acts on volume $V/W$; the $s$ child calls give $F(e) \le (s/W)F(e/m) + O((eK)^\tau+1)$, with leaves at $e < d^\beta$; the sum over at most $(m-1)(1+\lfloor\log_m d\rfloor)$ base-$m$ pieces gives $O(d^{\lambda'})$ | none |
| a word-for-word quote from `paper.tex` for each inferred step, which locates it | none; the evidence section holds extraction's two quotes, which state the bound, not the recursion |
| S3's `from`: nine concepts, including `recursive-power-saving` and `fast-chunk-interchange` | not as dependencies. The prerequisites given are the graph's three: `phase-edge-recursion`, `exact-dyadic-guard`, `role-streams`. `recursive-power-saving`, `fast-chunk-interchange` and `rank-saving` appear only under "Other pages you may link to". |
| that S3.3 and S3.5 belong to this page | only implied: they lack the "(on …)" the others carry |
| | no criteria for the finished page |

**After the page is written.**
- Submit checks the budget (350 words), new letters, citations and relations.
- The audit task does not see the argument at all. It judges the page against the reader, not against the plan.
- Nothing compares the page with the sub-steps it was given.

**The owner of $s < Wm$ gets the same treatment.** The task for `rank-saving` gives S1.4's claim, $s_{\rm b} = W_{\rm b}m - N + 2L_{\rm b} < W_{\rm b}m$, as one line. It gives S1's method, "sum the dimension changes along the edges", and S1.4's position among the sub-steps. It does not give S1.4's quote, which locates the derivation in `paper.tex`. The derivation itself (the signed sum $Wm - 2N$, then $2L$, then $N$) is in neither the plan nor the task.

## What this shows

**Some of Sonnet's reasoning reaches the writer:**
- what the page's step is;
- the method in a phrase, and why;
- the striking feature to explain;
- which sub-steps are proved elsewhere and where.

That is more than Haiku's plan gave in the explanation-depth run, which had no recursion step at all.

**The derivation the page owns does not reach it.** The plan holds it, but only as S3.3's method, inferred steps and quotes, and the handoff drops all three:
- **The page's own mathematics:** the writer gets a one-line statement of the recurrence and must recover the derivation from `paper.tex` without being told where it is.
- **Dependencies:** the plan's dependencies are wider than the graph's. The writer is told to rely on the three prerequisites, and not on the pages for the recurrence's saving or for $s < Wm$.
- **Acceptance:** no criteria are passed on, and no check compares the page with the plan.

**For the proposed run:**
- It tests whether a better plan, passed on through the present handoff, improves the wiki.
- If the butterfly page again states the recurrence without deriving it, the likely cause is the handoff, not the planner.
- A positive result would still understate what the plan could give.

**For the brief:** it would not need new reasoning from Sonnet first. Much of it is already in `argument.json`:
- the page's own sub-steps with their methods, inferred steps and quotes;
- the `from` dependencies;
- which sub-steps belong to the page.

What the plan lacks is acceptance criteria, and a way for the writer to flag a brief it cannot follow.

## Current State
- A write task gives the page's own argument step with its claim, method, motivation and striking feature, the one-line claims of its sub-steps with the pages that present them, and the chain of steps above it
- A write task does not give its sub-steps' methods, the inferred steps under them, the quotes of inferred steps, or the step's `from` dependencies
- The audit task does not see the argument
- In the first Sonnet sample's task for `simultaneous-butterfly-layer`, the recursion that gives $O(d^{\lambda'})$ appears as one sub-step claim, while its method, three inferred steps and their quotes stay in `argument.json`

## Missing
- No execution brief: the page's own sub-steps, inferred steps, quotes and dependencies passed to the writer, with acceptance criteria
- No way for a writer to flag a brief it cannot follow
- No check of a page against the steps its plan assigns it
- No corrective review

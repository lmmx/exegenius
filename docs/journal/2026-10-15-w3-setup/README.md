# W3 setup (2026-10-15, 09:28–09:29 UTC; no model calls)

- **Project copies:** 18, in the scratchpad's `e6/W3{WS,WO}P{page}`. Each is a copy of E9F4Y's project, with `exegenius-params next` run once (exegenius 0.6.0 built from e8efaa3; sha256 688a3b50…). That issued all 35 write tasks, as in W2 and W2b.
- **Checks:** for each page, the target write task file is identical across WS and WO apart from the root path.
- **`prompts.tsv`:** each run's description, model, effort and prompt, word for word as `next` printed it. Each run does its page's task only.
- **`assigned.json`:** the step ids scored per key-step page. They are fixed here, before any run, by the rule in `plans/W3.md`. The controls derive no steps.
- **To launch:** 18 `general-purpose` subagents, up to 20 at a time, with the description from column 1 and model and effort from columns 2 and 3. Pass the prompt in column 5 word for word. Not launched; it waits on the author's order and a meter reading.

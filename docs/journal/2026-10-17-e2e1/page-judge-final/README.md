# P4, P5 and S1: the final pages under the page judge

- **Pages:** the 39 pages a write task wrote, as they stand in the final wiki (`exegenius/pages/` of `lmmx/openai-nlogn-wiki` at 42727e0, the commit whose `docs/` is the built site; checked byte-identical), copied to `pages/Pnn.md` in an order shuffled with seed 20261018. `key.json` maps each Pnn to its page, its latest write task, and the step ids it is scored on.
- **Assignments:** `assignments/<page>.md`, from the page's latest write task: "This page in the argument" and "Steps this page derives" for the 23 pages with steps; "This page" and the argument's use of it for the 16 controls, which are judged for errors only.
- **Judges:** J24's page-judge prompt (`judge-prompt.md`), three blind Opus judges per page, each launched with "Read the instructions in … and follow them exactly. Read only that file and the files it names."
- **Completeness (plans/E2E1.md, 22:58):** 7 verdicts omitted scored ids and went back once to the same judge with the list: P22, P22-b, P22-c (simultaneous-layer), P29, P29-b (subspace-labels), P30, P30-b (cost-accounting). Their first-written versions are in `verdicts-first/`.
- **Scores:** `results.json` (completed verdicts) and `results-first.json` (first-written), from `python3 -I calibration/tools/e2e1pages.py read … [--verdicts verdicts-first] --json …`.

## Result

- **P4 fails:** packed-bit-addition has median majors 1 (judges 1, 1, 0). Its procedure drops the final XOR at $j_{f-1}$ (paper lines 2054, 2187–2192). All other pages have median majors 0.
- **P5 fails:** subspace-labels has median faithful 22 of 31 (judges 22, 22, 28), below the 28 the standard needs. The other pages the chain walk cites meet it.
- **S1:** 32 of 39 pages meet the full standard, 16 of the 23 with steps (first-written: 31 and 15).

Both scorings give the same P4 and P5.

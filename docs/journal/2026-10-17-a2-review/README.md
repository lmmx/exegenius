# A2 independent alarm review (prepared 2026-10-17 08:40 UTC; not yet run)

The design and its fixed reading are in `calibration/plans/A2.md`, "Independent review of the clean-page alarms". It is an exploratory sensitivity analysis: A2's preregistered result and `../2026-10-17-a2-run/adjudication.json` are not changed by it.

| File | What it is |
|---|---|
| `items.json` | the reviewer's 19 items, R01 to R19: the 15 clean-page alarms with notes masked, and 4 constructed decoys. A copy of the packet's file |
| `task.md` | the reviewer's instructions: the rule, the output format |
| `prompt.txt`, `launch.txt` | the launch prompt, word for word, and the subagent settings |
| `key.json` | which item is which: an A2 row (arm via `../2026-10-17-a2-run/adjudication-items.unblind.json`) or a decoy, with why the decoy is wrong; the seed; sha256 of every packet file and of the original adjudication |

The packet itself (items, the six clean pages, `paper.tex`, `task.md`) is in the scratchpad at `a2-review/`, outside the repository. To rebuild it: `python3 -I calibration/tools/a2review.py build PACKET_DIR docs/journal/2026-10-17-a2-review`. To score the reviewer's `verdicts.json`: `python3 -I calibration/tools/a2review.py score docs/journal/2026-10-17-a2-review VERDICTS.json --transcript T.jsonl`.

# Key-step designation of this wiki's pages, blind to the writer results (2026-10-15)

**Why:** the author's writer exception (Opus `xhigh` for key-step pages, Sonnet `high` otherwise; `calibration/policy.md`) needs every page designated by the rule before any writer is chosen. The designation must be made from the plan alone, without the W2 and W2b outcomes. The author asked for a reviewer who had not seen those results.

**The rule (the author's wording):** a page is key-step when it introduces or carries a central derivation, proof transition, or result whose omission or distortion would break the explanatory chain.

**How it was made:**
- **`pages.md` (the packet):** built from the 35 write task files of E9F4Y's plan (rendered by `exegenius-params`, e8efaa3). Only the sections "This page", "This page in the argument" / "Where the argument relies on this page" and "Steps this page derives" are kept. Source quotations are stripped for length. Nothing in it names a model, an experiment or a result, which was checked by search.
- **The reviewer:** a fresh `general-purpose` subagent, Opus, given only `prompt.txt` and the packet. Its prompt does not mention W2, W2b, writers, models or any page in particular.
- **Isolation, checked from its transcript:** it read `prompt.txt` and `pages.md` and nothing else. Its one shell call validated its own JSON.
- **The designation itself:** `designation.json`, unedited.

**The result:**
- **Key-step: 23 of 35:** 0012, 0013, 0014, 0015, 0016, 0017, 0018, 0019, 0021, 0022, 0023, 0027, 0028, 0029, 0030, 0031, 0032, 0033, 0034, 0035, 0036, 0037, 0038.
- **Not key-step: 12:** 0004, 0005, 0006, 0007, 0008, 0009, 0010, 0011, 0020, 0024, 0025, 0026.
- **Borderline: 8:** 0010, 0018, 0019, 0021, 0022, 0028, 0034, 0035. Of these, 0010 is marked not key-step; the others are marked key-step.

**Against the author's earlier designation:**
- **0015 and 0016:** key-step, in agreement.
- **0014 (the common-frame principle):** key-step, not borderline. The reviewer's reason: "Without this transition, edge costs could not be chosen freely as cheap rank changes, and the recursion would not be justified."
- **What governs:** the author's earlier "0014 does not qualify" was made after seeing the writer results, so it is not independent evidence. By the author's instruction, the blind designation governs.

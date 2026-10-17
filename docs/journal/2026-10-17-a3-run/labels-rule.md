# A3 hand labels: the rule

**The rule is A2's, unchanged** (`../2026-10-17-a2-run/labels-rule.md`: A1's rule plus A2's two clarifications, on notes that state the page contradicts the paper and on missing hypotheses). `1` means the note asserts that something on the page is false; `0` means it says only that something is unexplained, unjustified, undefined or uncheckable.

**What is labelled:** `A3-notes-masked.tsv`: the 132 gaps of arm R's 30 audits (`plant.py score --notes`), with paper line references and the words paper and source(s) masked by `tools/mask_notes.py`.

**Who labels, and what is not blind:** this session, as A3 preregisters. A3 has one arm, so the labeller knows every note comes from R (stated in `plans/A3.md`, "Weaknesses"). Row order is plant.py's shuffle; codes are shown, as in A2. F_A, the measure most open to labeller bias, is decided by the independent reviewer, not by these labels.

**Order of work:** these labels are committed before they are used to rescore (`tools/plant_labels.py`), before agreement with `ASSERTS` is computed, and before the clean-page alarms are selected for the reviewer.

**Consistency decisions taken while labelling** (each applied to every note of its kind):
- "Does not follow from the preceding sentence; the paper's choice of q is different" (rows 2, 48, 105): `1`, borderline. Rows 53 and 124 add that the statement is false (a zero numerator), so they are `1`.
- Missing row-divisibility hypothesis where the note says the page states the recursion or the cost for all arrays (rows 38, 44, 79, 93, 121): `1`. Row 17 says the general step "does not follow": `1`, borderline.
- Missing tape-order (layout) hypothesis where the note says only that the linear-time claim is unsupported (rows 75, 118): `0`, borderline.
- The shear "is such an invocation" where the note says the paper does not identify them (rows 60, 76): `1`, borderline.

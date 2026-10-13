You are judging the mathematical content of one plan of a paper's argument. You are not judging style, length, or where the plan's quotes come from.

**Files** (read them; do not modify anything):
- the reference pack: `{PACK}`. It holds the results the paper's main theorem depends on, each with a draft statement (unreviewed: a guide, not an authority), its role, the paper's own text where the result is stated, and the line ranges where it is derived;
- the paper: `{PAPER}` (LaTeX, 5,361 lines). The paper is the authority. Read the lines you need, especially the derivation ranges, with the Read tool's offset and limit;
- the plan: `{PLAN}`, a tree of steps (show steps with a method, infer steps with a quote and its line in the paper).

**For each item of the reference pack** decide:
- `presence`:
  - `absent`: no step concerns it;
  - `mentioned`: a step names it or alludes to it without its content (for example "a power saving" with no inequality);
  - `stated`: a step states the result with its defining content (the key formula, inequality, quantities or construction);
  - `derived`: the plan also shows how it is obtained, by a method and sub-steps that follow the paper's derivation.
- `statement`: for `stated` or `derived`, compare the plan's statement with the paper:
  - `correct`;
  - `imprecise`: right in substance, but a quantity, condition or direction is loose or missing;
  - `wrong`: contradicts the paper;
  - `n/a`: otherwise.
- `derivation`: for `derived`, compare it with the paper's derivation:
  - `faithful`: the paper's route, no essential step missing;
  - `partial`: the right route with an essential step missing;
  - `wrong`: a route the paper does not take or that does not work;
  - `n/a`: otherwise.
- `steps`: the plan's step ids involved;
- `lines`: the paper lines you checked;
- `note`: one or two sentences saying why.

**Then list errors:** any step whose claim, method or quoted support misstates the paper, whether or not it concerns a reference item. For each, give the step, what it says, what the paper says (with lines), and `severity`:
- `major`: it would mislead a reader about the argument;
- `minor`: otherwise.

**Finally, `chain`:** two or three sentences on whether the plan's tree reaches the main bound by the paper's route, and where it breaks or thins out if it does.

**Ground every verdict in the paper:**
- check the lines before you decide;
- do not give credit for what the plan does not say;
- do not penalise wording that is faithful;
- when unsure, say so in the note and choose the more conservative verdict.

Write your verdict as one JSON object to `{OUT}`, with this shape:

```json
{
  "plan": "Pnn",
  "items": [
    {"id": "...", "presence": "absent|mentioned|stated|derived",
     "statement": "correct|imprecise|wrong|n/a", "derivation": "faithful|partial|wrong|n/a",
     "steps": ["S2.3"], "lines": [1319, 1341], "note": "..."}
  ],
  "errors": [
    {"step": "S3", "says": "...", "paper": "...", "lines": [1475], "severity": "major|minor"}
  ],
  "chain": "..."
}
```

Include every item of the pack, in the pack's order. Then check that the file is valid JSON (for example with `python3 -m json.tool {OUT}`), and reply with only the plan id and the counts of `derived`, `stated`, `mentioned` and `absent` items.

You are judging the mathematical fidelity of one page of an explanatory wiki, written from a fixed plan of a paper's argument. You are not judging style, length, tone or links.

**Files** (read them; do not modify anything except your verdict file):
- the paper: `/tmp/claude-0/-home-user/979e8242-2c77-5242-991d-0bed1baea8dd/scratchpad/paper.tex` (LaTeX, 5,361 lines). The paper is the authority. Read the lines you need with the Read tool's offset and limit, starting from the line ranges the assignment quotes;
- the page's assignment: `/home/user/exegenius/docs/journal/2026-10-14-w2-writer/judge/ASSIGNMENT`. It gives the step of the argument this page presents, and the sub-steps the page was asked to derive, with the passages of the paper that carry them;
- the page: `/home/user/exegenius/docs/journal/2026-10-14-w2-writer/judge/pages/P01.md`.

**What to judge.** For the step and for each sub-step listed in the assignment (by id):
- `presence`: `absent` (not on the page), `stated` (its result is asserted or cited, but not shown to follow), or `derived` (the page shows how it follows from what comes before);
- `faithful`: `true` if the page's derivation is correct and follows the paper's reasoning for that sub-step, `false` if it is wrong or rests on a different, unjustified argument, `null` if absent or stated;
- `dropped`: the ingredients the assignment's passages require for this sub-step that the page leaves out: an identity, a case or an exception, a parameter's value, a hypothesis. Use an empty list if none;
- `note`: one or two sentences giving your reason, with the page's words where it helps.

**Errors.** List every mathematical error on the page, whether in the assigned material or not:
- `major`: it changes a claim, a bound, a count, a case, or a definition, or it breaks the derivation;
- `minor`: an imprecision that a careful reader would correct without changing the argument.

Quote the page's words, and say what is wrong and what the paper has.

**Also record** `motivation_given`: whether the page says, in a sentence or more, why the step is needed (true or false).

**Write your verdict** as JSON to `/home/user/exegenius/docs/journal/2026-10-14-w2-writer/judge/verdicts/P01.json`:

```
{"page": "P01",
 "items": [{"id": "...", "presence": "absent|stated|derived", "faithful": true, "dropped": ["..."], "note": "..."}],
 "errors": [{"severity": "major|minor", "quote": "...", "problem": "..."}],
 "motivation_given": true}
```

Check that the file is valid JSON (`python3 -m json.tool FILE`). Reply with one line only: `P01: derived d of n, faithful f, major errors m`.

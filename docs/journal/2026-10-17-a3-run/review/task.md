# Review of alarms raised against explanatory pages

You are reviewing alarms that automated auditors raised against six pages of an explanatory wiki about a mathematics paper. Each alarm quotes a statement from a page and says, in a note, what is wrong with it. Some alarms identify real misstatements and some do not. Decide each one yourself, from the page and the paper.

**Files (use only these; do not read any other file or directory):**
- `/tmp/claude-0/-home-user/979e8242-2c77-5242-991d-0bed1baea8dd/scratchpad/a3-review/items.json`: the alarms, each with an `id`, the `page`, the `quote` and the `note`.
- `/tmp/claude-0/-home-user/979e8242-2c77-5242-991d-0bed1baea8dd/scratchpad/a3-review/pages/<page>.md`: the six pages, in Markdown with LaTeX mathematics. `<cite data-source="paper">…</cite>` marks a verbatim quotation of the paper.
- `/tmp/claude-0/-home-user/979e8242-2c77-5242-991d-0bed1baea8dd/scratchpad/a3-review/paper.tex`: the paper (LaTeX, about 5,400 lines). Search it for the terms and symbols of each statement, and read the passages around each match.

**The notes have had their citations removed,** and what a note says the source states is an unverified claim. Check every claim, in the page and in the paper. A note can be confident and wrong.

**The rule.** An alarm is **genuine** if the quoted statement, read in the context of its page, misstates the mathematics:
(a) it is false as stated, or inconsistent with the rest of its page;
(b) it is presented as following from what precedes it, but does not follow; or
(c) it omits a hypothesis that the paper requires for that claim, so that as stated it is false or wider than what the paper establishes.

An alarm is **not genuine** if the quoted statement is correct (the objection's mathematics is wrong, or it misreads the page), or if the objection is only that the page does not explain, justify or motivate a correct statement. Judge the statement, not the note: a note can be partly wrong and still identify a genuine misstatement; then say which part holds. Mark a verdict **borderline** when competent mathematicians could reasonably disagree, for example when the statement is an informal simplification that an explanatory page might legitimately make.

**Output.** Write `/tmp/claude-0/-home-user/979e8242-2c77-5242-991d-0bed1baea8dd/scratchpad/a3-review/verdicts.json`, one entry per alarm, in this form, and nothing else:

```json
{"items": [
  {"id": "R01", "verdict": "genuine", "confidence": "clear",
    "page_evidence": "what the page says that bears on it",
    "paper_evidence": "the paper's line numbers and what they say",
    "reason": "two to four sentences: why the statement is or is not a misstatement"}
]}
```

`verdict` is `genuine` or `not_genuine`; `confidence` is `clear` or `borderline`. Give a verdict for every id. Give paper line numbers wherever the paper bears on the decision. Then reply with one line: the number of verdicts written.

# 2026-10-11: Symbol bindings

This entry implements the first research entry, 2026-10-11-symbol-anchoring (proposals I-90 and I-91), in the order the hub entry 2026-10-11-explanatory-structure-research sets. It records what was built, where the build departs from the research entry and why, and a check against the demo project's own pages.

## What was built

The research entry found that the demo's audit missed `r` for three reasons: knowledge was per concept, so knowing "Polynomial ring modulo y^r + 1" implied knowing `r`; the audit reported only failures, so an auditor that noticed nothing passed the page; and meanings travelled as one-line glosses that gave `r` a range but not what it counts. Each of the three now has a counterpart.

### Bindings

A binding says what a symbol denotes. It has the symbol in LaTeX, a display name of one to four words ("record length"), a one-sentence meaning, and a relation, `direct` or `count`, after Symlink's DIRECT and COUNT (exegenius-core/src/symbols.rs).

Bindings come from two places.
- **Concept names.** A concept whose name contains mathematics gets a `title`, the name with the mathematics in LaTeX, and a `symbols` list binding every symbol in it. Extraction and extension propose them. Submit rejects a binding whose symbol is not in the title's mathematics, and symbols with no title.
- **Pages.** A write answer is now two files: the body as markdown, and a JSON object listing the symbols the page binds. Each binding carries the sentence on the page that says what the symbol denotes. Submit rejects a binding whose symbol is not in the page's mathematics, whose quote is not on the page, or whose quote does not contain the symbol.

Code never reads mathematics for meaning. It checks one thing: that a named symbol occurs in a piece of LaTeX as whole TeX tokens. Control words, control symbols and single characters are tokens; spacing and grouping braces are dropped (exegenius-core/src/symbols.rs:69). So `r` occurs in `y^{r} + 1` but not inside `\frac`, which the plain substring comparison of the old notation check would have matched.

### What the tasks show

Every list of known concepts in write and audit tasks prints each concept's name bindings under its name. A writer or auditor who meets "Polynomial ring modulo y^r + 1" is shown "$r$ (record length): …", or, if the concept binds nothing, no meaning for `r` at all.

The write task also lists the bindings stated on earlier pages, with the page that binds each. The writer is told it may use these without restating them, and that reusing one of the letters with another meaning needs a new letter or a sentence saying the meaning changes.

The audit task lists the bindings on earlier pages only for symbols that occur in the page's mathematics. Bindings for symbols the page does not use cannot help the auditor. Leaving them out also keeps the audit input, and so its hash, unchanged when unrelated pages are rewritten. Without that, every rewrite of an early page would send every later page back to audit.

### The audit accounts for every symbol

The audit answer gains a `symbols` list. For each symbol in the page's mathematics, the auditor gives one anchor:
- `here`, with the quote that says what it denotes;
- `page`, naming an earlier page that binds it;
- `concept`, naming a known concept whose name binds it;
- `background`, saying what in the reader profile covers it;
- `unbound`, with the quote where it is first used and what the reader cannot tell.

Submit checks each anchor:
- every symbol occurs in the page's mathematics, and each is listed once;
- `here` and `unbound` quotes are on the page and contain the symbol;
- a `page` anchor names an earlier page whose writer bound the symbol;
- a `concept` anchor names a known concept whose name binds the symbol.

The demo's case is the last check. A `concept` anchor to "Polynomial ring modulo y^r + 1" is rejected unless that concept binds `r`. The auditor then has to find the binding on the page or report `r` unbound.

Completeness, whether the auditor listed every symbol, cannot be checked without parsing the mathematics. The research entry proposed a second audit on a sample of pages. What is built is a floor instead: every symbol the page's writer bound must be accounted for, or the answer is rejected with the missing symbols named. The writer's meanings are not shown to the auditor, so the auditor cannot copy them.

Each `unbound` symbol becomes a rewrite note asking the writer to say what it denotes, and for a size or count, what it counts. It is recorded as a gap of kind `symbol`, so it counts towards the round's gaps as before. Auditors can no longer report symbols as gaps: the schema leaves the `symbol` kind out, and submit rejects it.

### Letters with two meanings

The notation task no longer lists symbols. In its place, an overloads task judges candidates that code finds: a page binds a letter that a page or concept name it builds on also binds, with a different display name and meaning (exegenius-core/src/notation.rs:163). The agent says, for each candidate, whether the two denote the same thing. A letter with two meanings becomes a rewrite note on the later page. Verdicts are kept in `exegenius/overloads.json`, so each candidate is judged once. The task is handed out with the outline, and only when there are candidates with no verdict.

Candidates follow prerequisite edges only, as the research entry proposed. A letter bound on two unrelated pages is not a candidate: a reader carries a meaning in from the pages a page builds on.

### The page and the notation page

- **Headings.** A page whose concept has a title uses it as its heading, so the mathematics renders. The navigation, diagrams and links keep the plain name.
- **Inherited symbols.** Under "Before this page", a folded note lists each symbol the page takes from an earlier page or a concept name, per the page's latest audit, with its display name, meaning and where it is bound (exegenius-core/src/build.rs:388). Symbols no longer in the page's mathematics are left out, so a page rewritten after its last audit does not list symbols it dropped.
- **The notation page** is derived from the bindings and the audits, with no agent. It lists each binding that a page other than its own uses, with its display name, meaning, where it is bound and the pages that use it. Its introduction says that symbols used on one page are explained there and not listed, so it does not imply completeness.
- **Simulated readers** see the same note on every page, so the simulation measures it. The note's display names are searchable: "record length" finds the pages that use `r`.

### Existing projects

Graphs, pages and audit records written before bindings load unchanged: the new fields default to empty. Such a project has no bindings, so its notation page and inherited-symbols notes are empty until pages are rewritten and audited. A project that is still in its audit rounds re-audits every page on its next round, since the audit task's text changed. The old `exegenius/notation.json` is no longer read.

## Where the build departs from the research entry

1. **The write answer is not one JSON object.** The research entry had the body inside JSON. A JSON string turns `\frac` into a form feed followed by "rac", `\theta` and `\times` into a tab, `\beta` into a backspace, and `\nu` and `\rho` into a newline and a carriage return, all without an error. The body therefore stays a markdown file, and only the bindings are JSON. In a binding, the symbol check catches the same mistake: a mangled symbol is not in the page's mathematics, so submit rejects it.
2. **A `page` anchor names the page, without a quote.** The binding on that page already carries a quote, checked when that page was written.
3. **The bindings in force are those of every earlier page, not only prerequisites,** matching what the reader is assumed to know (`Plan::known_at`). A reader who arrives by a deep link is covered by the inherited-symbols note, which links the binding page.
4. **The note is titled "Symbols from earlier pages"** rather than "Symbols on this page", since it lists only inherited symbols.
5. **Concept-name bindings are checked against a LaTeX title,** not against the name. Names are plain text with Unicode symbols, and a token check cannot match `κ` in a name against `\kappa` in a binding without a table of symbols. The title doubles as the rendered heading the research entry asked for.
6. **The notation task became the overloads task.** The research entry kept the notation task to confirm overload candidates. With the notation page derived, confirming overloads is all that is left of it, and the task is named for that.

## Check against the demo

The demo project (lmmx/openai-nlogn-wiki) has no sources in its repository, and its graph, pages and audit records predate bindings. On a copy, a source file was rebuilt from the graph's evidence quotes, written in the reading order of the last audit round. That reproduces the demo's reading order for every page before the eleventh, which covers the pages checked. The state was reset so that `next` handed out a fresh audit round. Four of its audit tasks went to fresh subagents on the smallest model, each given only the prompt `next` prints. These were the three pages where the research entry found symbols bound nearby but not on the page, and `cyclic-convolution`.

All four answers were accepted, two on the first submit. Each of the other two was first rejected once for a quote that started inside a formula, or was copied from inside a display formula. Those quotes were on the page, but the check looked for the symbol only in mathematics the quote itself delimited. The check was then changed: it locates the quote in the page and looks for the symbol in the page's mathematics that the quote covers (`symbols::quoted`, exegenius-core/src/symbols.rs:127). Both rejected quotes are cases in its test.

| Page | Symbol gaps in the old audits, rounds 2 and 3 | Symbols reported `unbound` now |
|---|---|---|
| `synthetic-transform` | none | `r`, `k`, `s`, `t` |
| `signed-packing` | none | `r`, `p`, `g`, `G` |
| `gaussian-dyadic` | none | `u`, `v` |
| `cyclic-convolution` | `q` in round 2, none in round 3 | none |

- **`r` on `synthetic-transform`.** The auditor's note: "Nothing says what r denotes. The only value, r = 4, is an example. The ring concept's name contains r but binds nothing." These are the two reasons the research entry found, stated by the auditor without being told them. The old graph has no name bindings, so no `concept` anchor for `r` could have been accepted.
- **`signed-packing` and `gaussian-dyadic`.** The symbols the research entry found by hand on these pages (`r` and `p`; `u` and `v`) are all reported. `H_0` was anchored `here` to the sentence that says what the butterfly map computes.
- **`cyclic-convolution`.** `q` was anchored `here` to "where $q$ is the number of digits of each input", the sentence the round 2 rewrite added. So the new audit agrees with the old one where the old one acted.
- **Judgement calls.** `g` and `G` on `signed-packing` are given only by a worked example, which the instructions say does not bind them, so `unbound` follows the instructions. Coefficients such as `a_0` and indices such as `i` were anchored to the reader's background (polynomials).

The check shows that the accounting catches the symbols the research entry found on these pages, and what the auditor writes about each. It does not show how many symbols auditors leave out. Code checks only the symbols an auditor lists, and none of these pages had writer bindings to set a floor.

## Current State
- `symbols::occurs` checks that a symbol occurs in LaTeX as whole TeX tokens, ignoring spacing and grouping braces (exegenius-core/src/symbols.rs:69, exegenius-core/src/symbols.rs:161)
- `symbols::quoted` locates a quote on a page with `text::locate` and checks the symbol in the page's mathematics that the quote covers, so quotes may start, end or lie inside a formula (exegenius-core/src/symbols.rs:127, exegenius-core/src/text.rs:40)
- `Concept` carries an optional `title` with LaTeX and a `symbols` list of name bindings; `check_name` rejects bindings without a title, symbols not in the title's mathematics, display names outside one to four words, empty meanings and duplicates, and `clean` in extraction applies it (exegenius-core/src/graph.rs, exegenius-core/src/symbols.rs:202, exegenius-core/src/extract.rs)
- A write task has two answer files, `<id>.answer.md` for the body and `<id>.answer.json` for `WriteAnswer`, whose `symbols` list holds the page's bindings (exegenius-core/src/task.rs:122, exegenius-core/src/write.rs:26)
- `apply_write` rejects a binding whose symbol is not in the page's mathematics or whose quote is not on the page or lacks the symbol, and stores accepted bindings in `graph.bindings` (exegenius-core/src/write.rs:221, exegenius-core/src/symbols.rs:238)
- `symbols::in_force` gives the bindings of the concepts known at a page plus the page bindings of every earlier page (exegenius-core/src/symbols.rs:328)
- Write tasks list known concepts with their name bindings, the page's own name bindings, and the page bindings in force (exegenius-core/src/write.rs:72, exegenius-core/src/write.rs:101)
- Audit tasks list the page bindings in force whose symbol occurs in the audited page's mathematics, and the heading uses the title (exegenius-core/src/audit.rs:178)
- `AuditAnswer` requires a `symbols` list of anchors (`here`, `page`, `concept`, `background`, `unbound`); `check_symbols` rejects symbols not in the page's mathematics, duplicates, quotes not on the page or without the symbol, `page` and `concept` anchors that bind nothing, and answers that leave out a symbol the writer bound (exegenius-core/src/audit.rs:224)
- `apply_audit` turns each `unbound` symbol into a rewrite note and a gap of kind `symbol`, and records the accepted anchors in `exegenius/audit.json` (exegenius-core/src/audit.rs:302)
- `notation::candidates` lists page bindings that rebind a letter bound by the page's own name or by a page or name it builds on, skipping pairs with the same display name or meaning (exegenius-core/src/notation.rs:163)
- The overloads task is issued with the outline when candidates lack a verdict; `apply_overloads` requires one verdict per candidate, queues a rewrite note for each letter with two meanings, and appends the verdicts to `exegenius/overloads.json` (exegenius-core/src/task.rs, exegenius-core/src/notation.rs:241)
- `build` uses a concept's title as its page and glossary heading, writes a folded "Symbols from earlier pages" note from each page's latest audit, and derives `docs/notation.md` from the bindings that other pages use (exegenius-core/src/build.rs:383, exegenius-core/src/build.rs:388, exegenius-core/src/build.rs:536)
- The simulation lens shows each page's inherited symbols with display names and links (exegenius-core/src/simulate.rs)
- `status` reports the number of page bindings and of confirmed and pending overloads (exegenius-cli/src/main.rs)

## Missing
- No `\texttip` tooltips on symbols; whether the MathJax bundle the build loads includes the `action` extension is unchecked
- No `role` on bindings, so symbols used on many pages are not introduced as actors
- No second audit on a sample of pages to estimate how many symbols auditors leave out
- No scoping of bindings by plan node; bindings hold from their page onwards in reading order
- No report count of stuck points about symbols

# 2026-10-12: The paper's LaTeX as the source

2026-10-12-first-level-pilot traced its one false claim to the source text.
- `sources/paper.txt`, converted from the PDF, put "√" on lines of their own and wrote superscripts inline. Lemma 19's $2 \le \alpha < \sqrt p$ read as "2 ≤ α < p".
- ChatGPT, reviewing the pilot, noted the deeper consequence: writer and auditor read the same corrupted text, so their agreement is no check against the paper.

The author found the paper's LaTeX source and prepared a pilot from it, with ChatGPT's help, on lmmx/openai-nlogn-wiki branch `pilot-v0.6-latex-source` (1b7e73b). This entry records that preparation, what it changes about the experiment, and one change to exegenius for LaTeX sources.

## The source

The OpenAI math release of the paper, dated 23 September 2026, is a directory with `README.md`, `paper.pdf` and `build/`:
- `main.tex`;
- `sections/00-introduction.tex` to `11-grouped-rectangles.tex`;
- `references.bib`;
- two figure sources.

Its PDF is byte for byte the PDF that `paper.txt` came from. The author committed the directory under `sources/` and built a single source file from it:

```
cd sources/Integer-multiplication-below-n-log-n-September-23-2026/build
latexpand main.tex > ../../paper.tex
```

`latexpand` inlines each `\input` in order. A first build also appended `references.bib` after `\end{document}`; ChatGPT caught it, and it was rebuilt without. The result, `sources/paper.tex`, is 253 KB. It ends with `\bibliography{references}` and `\end{document}`, and has no `\input` left.

`exegenius.toml` names `sources/paper.tex` as source `paper`, with the release's GitHub URL. `sources/paper.txt` and `extract-answer-0.5.0.json` were removed. The extract answer's evidence quotes are checked word for word against the source, and they quote the old text.

**What the LaTeX contains:**
- **Citations.** 49 `\cite` commands. Their keys name the work, such as `HarveyHoeven2021` and `Schonhage1980`, and the prose names the authors, so the bibliography is not needed to tell which work is meant. It stays in the directory for a writer who wants it.
- **Macros.** One custom macro, `\norm`, defined and never used, so no writer copies a macro MathJax cannot render.

## What this changes about the experiment

Extraction runs again, on a different text. The concepts, the reading order and the pilot concept may all differ from those of `pilot-v0.6-first-level`, which the pilot keeps as its baseline. The comparison is therefore of the mathematics and the explanation on whatever pages the pilot explains, not page against page. ChatGPT asked for the source change to be treated as its own experiment, with no other change alongside it. The only change to exegenius is the one below, which affects how citations are displayed and nothing a task sees.

The branch as pushed still holds the first-level pilot's output: `exegenius/` with its state at round 3, done; `docs/`; `zensical.toml`; `run-status.txt`. A run started on it would find `exegenius next` printing `done`. Removing them, then `exegenius init`, comes before the run.

## Section locators for LaTeX

A citation shows a locator such as "§3.2 Gaussian resampling, p. 5". Pages come from form feeds, and sections from markdown or numbered headings. LaTeX has neither, so citations to `paper.tex` would have had none.

`\section`, `\subsection` and `\subsubsection` lines are now headings, numbered as LaTeX numbers them (exegenius-core/src/project.rs `latex_heading`):
- a short title in brackets is skipped;
- a starred command gets no number;
- nested braces in the title are kept.

On `paper.tex`, "We retain both permutations in the transform formula" is located in "§8 Resampling with the permutations left in the transform", the eighth `\section`, built from `07-resampling.tex`.

## Current State
- `pilot-v0.6-latex-source` names `sources/paper.tex`, the paper's LaTeX assembled by `latexpand`, as its source, and has no `paper.txt` or reused extract answer
- `sources/paper.tex` has 49 citations by self-describing keys and one custom macro, `\norm`, which it never uses
- The branch as pushed holds the first-level pilot's `exegenius/` state, `docs/`, `zensical.toml` and `run-status.txt`
- LaTeX sectioning commands give citation locators, numbered as LaTeX numbers them (exegenius-core/src/project.rs)

## Missing
- No run on the LaTeX source
- No page numbers in locators for a LaTeX source
- No check that a page uses only macros MathJax renders

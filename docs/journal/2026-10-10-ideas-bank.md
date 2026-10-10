# 2026-10-10: Ideas bank

Every idea raised in the design rounds (R1, R2, R3; see 2026-10-10-origin) and in the other
2026-10-10 entries, each with a stable ID for steering ("drop I-34", "pull I-38 into v1").
The **From** column records provenance. "new" marks an idea first written down in these
journal entries rather than in the chat rounds.

Targets:

- **v1**: in the first working version, milestones M0–M4 (2026-10-10-v1-scope).
- **v1-exp**: in the experimental simulator milestone, M5.
- **later**: on the roadmap.
- **rejected**: not planned. The reason is in the entry named in the last column.

IDs are not reused. A retired idea keeps its row with its target changed.

## Reader model and planning

| ID | Idea | From | Target | Entry |
|---|---|---|---|---|
| I-01 | Reader state as a set of concepts (KST); entry eligible when its requires ⊆ K; reading *proposes* K ∪ provides | R1, R2 | v1 | reader-model |
| I-02 | Outer fringe as the set of entries a reader can read next; an empty fringe before the goal means the corpus strands the reader | new | v1 | reader-model |
| I-03 | `requires`/`provides` name concepts, not entries; one entry may provide several concepts | new | v1 | reader-model |
| I-04 | One home entry per concept (or a profile item) | new | v1 | reader-model |
| I-05 | Path to a goal as the prerequisite closure minus the background, ordered topologically; hop count = closure size | R1, new | v1 | reader-model |
| I-06 | Page token length reported as a second cost beside hop count | new | v1 | reader-model |
| I-07 | Named background profiles; planning always relative to one profile | R1, new | v1 | reader-model |
| I-08 | Goals declare the concepts they test (competence-based KST skill map) | new | v1 | reader-model |
| I-09 | Typed relations `motivates`, `illustrates`, `formalizes`, `proves` parsed, resolved and rendered, but not gating | R2 | v1 | reader-model |
| I-10 | Entry `kind` (definition, result, intuition, example, procedure) selects which checks apply | R2 | later | reader-model |
| I-11 | Ordering tie-break: finish one branch before starting another, to keep introduced-but-unused time short | new | later | reader-model |
| I-12 | Multiple homes per concept (intuition page and formal page) with AND/OR path planning | R2, new | later | reader-model |
| I-13 | Probabilistic mastery per concept | R1, R2 | later | reader-model |
| I-14 | POMDP reader for adaptive navigation | R1, R2 | later | reader-model |
| I-15 | Per-profile optimisation, motivated by expertise reversal and reverse cohesion | new | later | reader-model |

## Rules, grounding and provenance

| ID | Idea | From | Target | Entry |
|---|---|---|---|---|
| I-20 | Integrity rules: `link.dangling`, `requires.unknown`, `requires.cycle`, `concept.multiple-homes`, `alias.collision`, `claim.unknown-source` | R1, new | v1 | rules-and-grounding |
| I-21 | Pedagogical rules: `link.unrequired`, `mention.unlinked`, `goal.unreachable`, `goal.missing`, `entry.orphan`, `claim.unsourced` | R1, new | v1 | rules-and-grounding |
| I-22 | Split between integrity rules (never suppressible) and pedagogical rules (hypotheses) | new | v1 | rules-and-grounding |
| I-23 | Each pedagogical rule carries an ID, rationale and status (`hypothesis`, `validated`, `retired`), and can be suppressed per instance | R3 | v1 | rules-and-grounding |
| I-24 | Lint metrics are guardrails, never the optimisation objective | R3 | v1 | rules-and-grounding |
| I-25 | Each diagnostic names a suggested author move | R1 | v1 | rules-and-grounding |
| I-26 | Author moves: split, extract, add example, bridge, inline definition, rename | R1 | v1 | rules-and-grounding |
| I-27 | Merge as an author move for `requires` cycles (from learning-space smoothness) | new | v1 | rules-and-grounding |
| I-28 | Stable IDs, with `aliases` keeping old slugs resolvable after renames | R1, new | v1 | reader-model |
| I-29 | Sources table, claim markers citing source IDs, claim status `unverified`/`checked`/`disputed` | R1, R2 | v1 | rules-and-grounding |
| I-30 | Comprehensibility and correctness reported separately and never combined into one score | R1, R2 | v1 | rules-and-grounding |
| I-31 | Typed intentional exceptions with checked obligations: preview, spiral, simplification | R3 | later | rules-and-grounding |
| I-32 | Promotion of patterns found by simulation or confusion logs into rules, with provenance | R3 | later | rules-and-grounding |
| I-33 | Candidate discovery for unregistered terms (noun phrases, symbols, lexical heuristics); kept as candidates, never auto-confirmed | R2 | later | rules-and-grounding |
| I-34 | Semantic review of candidate terms by a model, with evidence | R2 | later | rules-and-grounding |
| I-35 | Grounding of mathematical notation as a second namespace | new | later | rules-and-grounding |
| I-36 | Checking claims against the text of their sources by a model | R1, R2 | later | rules-and-grounding |
| I-37 | Prerequisite inference from link structure (RefD) | R1 | later, as a suggestion source only, never as validation | rules-and-grounding |

## Simulation

| ID | Idea | From | Target | Entry |
|---|---|---|---|---|
| I-40 | Closed-book reader as a direct API call with no tools and only the visited pages in context | R1, new | v1-exp | simulation-validity |
| I-41 | Minimal ESS: known concepts from the profile, evidence from the planned path | R1, R2 | v1-exp | simulation-validity |
| I-42 | Answers cite spans; verbatim presence in a visited page checked by string match | R1, R2 | v1-exp | simulation-validity |
| I-43 | No-page baseline per goal; goals the model answers unaided marked uninformative | R1 | v1-exp | simulation-validity |
| I-44 | Goals carry authored reference answers (key points) graded by a cached LLM judge | new | v1-exp | simulation-validity |
| I-45 | Goal `kind`: recall or transfer | R2 | v1 (field), v1-exp (used) | simulation-validity |
| I-46 | LLM call cache keyed by model, prompt template version and content hash | R1 | v1-exp | simulation-validity |
| I-47 | Provider trait with one Anthropic implementation | R1, new | v1-exp | simulation-validity |
| I-48 | Overlap check between goal reference answers and page text, to flag copy-only goals | new | later | simulation-validity |
| I-49 | Nonce-word substitution of concept names before simulation, to cut recall that depends on names | new | later | simulation-validity |
| I-50 | Ladder of reader models, small to large, as a competence dial | new | later | simulation-validity |
| I-51 | Leave-one-out page ablation for credit per page | new | later | simulation-validity |
| I-52 | Planted-defect mutation testing of the simulator's sensitivity and localisation | new | later | simulation-validity |
| I-53 | Full ESS: unknown concepts, misconception profile, transition rules, response constraints | R2 | later | simulation-validity |
| I-54 | Navigation agent (read, follow, backtrack, give up) with logged trajectories | R1 | later | simulation-validity |
| I-55 | Information-scent analysis of anchor text: weak-scent and competing links, as in CWW | R1, new | later | simulation-validity |
| I-56 | Wikispeedia human paths as a reference for checking navigation-agent behaviour | new | later | simulation-validity |
| I-57 | Adversarial misreader limited to a misconception taxonomy | R1, R2 | later | simulation-validity |
| I-58 | Minimal edit that blocks a misreading, with regression checks | R1, R2 | later | simulation-validity |
| I-59 | Author confusion log (`?` markers) as a calibration set | R1 | later | simulation-validity |
| I-60 | Human calibration with readers of different backgrounds | R2 | later | simulation-validity |
| I-61 | Repeated trials and fixed sampling settings per probe | R2 | v1-exp | simulation-validity |

## Output, integration and project

| ID | Idea | From | Target | Entry |
|---|---|---|---|---|
| I-70 | Static HTML wiki from the binary: resolved links, backlinks, requires/provides panel, goals, unverified-claim count, reading path | new | v1 | v1-scope |
| I-71 | `--json` diagnostics with a stable schema, and exit codes that separate integrity from pedagogical findings | R1, new | v1 | v1-scope |
| I-72 | Claude Code skill file documenting the author loop | new | v1 | v1-scope |
| I-73 | `exegenius graph` export as DOT and JSON | new | v1 | v1-scope |
| I-74 | KaTeX math rendering in the wiki | new | v1 | v1-scope |
| I-75 | Golden corpus: FFT and integer-multiplication cluster | R1, R2 | v1 | v1-scope |
| I-76 | Planted-defect fixtures with insta snapshots of `check --json` | R2, new | v1 | v1-scope |
| I-77 | Golden corpus from the old university notes | R1 | open question | v1-scope |
| I-78 | Three separate experiments: linter recovery, simulator validity, learning effectiveness | R2 | v1 (first), later (others) | v1-scope |
| I-79 | Python bindings (`exegenius-py`, pyo3, maturin) | project brief | later | v1-scope |
| I-80 | Source ingestion from PDF and arXiv | ChatGPT v1 list | later | v1-scope |
| I-81 | Entry generation and knowledge extraction by a model inside exegenius | ChatGPT v1 list | later | v1-scope |
| I-82 | Question generation for goals | ChatGPT v1 list | later | v1-scope |
| I-83 | mkdocs export of the wiki | new | later | v1-scope |
| I-84 | MCP server | ChatGPT v1 list | rejected for now (the CLI with `--json` and a skill file covers it) | v1-scope |

## Roadmap after v1

The order below follows R2's priorities with two changes:

- planted-defect testing of the simulator (I-52) comes before human calibration, because
  it needs no readers;
- leakage controls (I-49, I-51) come before adversarial reading, because adversarial
  findings mean nothing if the reader is leaking.

1. Typed exceptions and their obligations (I-31), once the v1 rules have run on the golden
   corpus and the false positives they produce are known.
2. Simulator hardening: I-48, I-49, I-51, I-52.
3. Adversarial misreading and minimal edits (I-57, I-58).
4. Candidate term discovery and review (I-33, I-34), then grounding of notation (I-35).
5. Claim checking against sources (I-36), together with source ingestion (I-80).
6. Navigation agent and scent analysis (I-54, I-55, I-56).
7. Calibration against the author's confusion log and against other readers (I-59, I-60),
   then promoting findings into rules (I-32).
8. Probabilistic mastery, multiple homes, POMDP (I-13, I-12, I-14), when the evidence
   justifies them.
9. Python bindings (I-79).

## Current State
- No idea in docs/journal/2026-10-10-ideas-bank.md has an implementation in the
  repository — the repository holds documentation only (README.md, LICENSE, docs/JOURNAL.md,
  docs/journal/)

# 2026-10-10: Lint rules, term grounding and author moves

The deterministic layer: what it checks, what authority its rules carry, how terms get
grounded, how factual claims are tracked, and how a finding becomes an edit. R1, R2 and
R3 are the three design rounds described in 2026-10-10-origin.

## Verification and discovery

R2 asked whether exegenius checks the structural integrity of explanations or discovers
what works for actual readers. Its answer was both, kept as separate objectives.

R3 put the difference in terms of how each one knows things:

- **Structural checks are verification.** They are checked against a spec the author
  wrote, so each has a definite answer, and that answer can be wrong only if the spec is.
- **Discovering what works is empirical.** It produces hypotheses, and those need a
  reader model, calibration data, and room to be surprised.

The crate split R1 proposed already separates the two: a pure `core` and a `sim` behind a
trait. What still has to be designed is how the two layers talk to each other.

## Structural scores as targets backfire

R3: a corpus can be fully grounded, acyclic and goal-reachable, and still be unreadable.
Optimising a lint score produces two failure modes:

- **Atomisation.** Every concept gets its own stub page, and a reader needs fifteen hops
  to get anywhere. Many Wikipedia mathematics articles fail this way.
- **Unmotivated definition chains.** Every page is valid, but nothing says why the reader
  should care until the end.

2026-10-10-reader-model deals with atomisation in the data model by separating concepts
from pages. The unmotivated chain is not a structural defect at all, so only simulation or
a human reader can detect it.

## Good explanations break the rules on purpose

R3 listed deliberate violations that a strict prerequisite DAG would flag as errors:

- a forward reference used as a motivating preview ("we'll need this for the chirp trick
  later");
- a spiral curriculum (Bruner), which revisits a concept at increasing depth;
- a deliberate simplification corrected later, the "lies-to-children" pattern
  (popularised by Pratchett, Stewart & Cohen in *The Science of Discworld*).

R3's fix is **typed exceptions with obligations**. The author declares a rule-break as
intentional, and the linter then checks that the promise attached to it is kept. That is
still verification, against a richer contract. The obligations can all be checked on the
graph:

| Exception | Declared as | Obligation the linter checks |
|---|---|---|
| preview | a link typed `preview` to entry X | X provides the previewed concept, and X comes after the previewing page in the planned order for every profile whose path includes both |
| spiral | entry B declares `deepens: c` for a concept c homed at A | B requires c, so B comes after A |
| simplification | entry A declares `simplifies: c, corrected_by: B` | B exists, B requires something A provides, and B appears in at least one planned path that includes A |

## Two classes of rule

R3 proposed that every lint rule be a versioned hypothesis, with an ID, a rationale, a
status (`hypothesis`, `validated` or `retired`), and suppression for individual
instances. Patterns found through simulation and the author's own confusion logs (R3's
example: "readers abandon when a page has more than 3 unresolved `requires`") get promoted
into rules, with a record of where each came from. Lint metrics serve as guardrails, never
as the objective. Empirical reader signals are the objective, and rules constrain the
search. R3 called the resulting cycle *spec → verify → simulate/observe → revise spec*,
the dynamic of Lakatos's *Proofs and Refutations*, with confused readers playing the part
of counterexamples.

R3 went too far in one respect. Some rules are not pedagogical hypotheses at all:

- **Integrity rules.** A link to an ID that resolves to nothing, a `requires` naming an
  unknown concept, a cycle in `requires`, two homes for one concept. When any of these
  fails, the corpus cannot be planned or rendered correctly. These rules are not
  hypotheses, so they carry no status and cannot be suppressed.
- **Pedagogical rules.** An unlinked mention, a page with no goals, a long page, an
  orphan page. These rules guess at what helps a reader. They carry the full R3 metadata.

Without this split, a "retired" status could be applied to a dangling link.

## Proposed v1 rules

IDs are dotted slugs so they read well in suppressions and JSON output.

| ID | Class | Fires when | Suggested move |
|---|---|---|---|
| `link.dangling` | integrity | a `[[x]]` link resolves to no entry, concept, alias, or profile item | create entry; fix link |
| `requires.unknown` | integrity | a `requires` item resolves to no concept or profile item | create entry; fix id |
| `requires.cycle` | integrity | `requires` contains a cycle | merge; split |
| `concept.multiple-homes` | integrity | two entries provide one concept | merge; rename |
| `alias.collision` | integrity | one alias resolves to two concepts (polysemy) | rename |
| `link.unrequired` | pedagogical | a body link targets a concept the page neither requires nor provides, and the link is not typed (preview, see) | add to requires; type as preview; inline definition |
| `mention.unlinked` | pedagogical | a known alias appears unlinked in the body of a page that does not provide the concept | link; inline definition |
| `goal.unreachable` | pedagogical | a goal tests a concept outside the page's prerequisite closure from the profile | add requires; bridge |
| `goal.missing` | pedagogical | an entry provides concepts but declares no goals | add goal |
| `entry.orphan` | pedagogical | an entry has no inbound links and is not a declared root | link from parent; delete |
| `claim.unsourced` | pedagogical | a claim marker cites no source | cite source |
| `claim.unknown-source` | integrity | a claim cites a source id with no entry in the sources table | fix id; add source |

The exception obligations (preview, spiral, simplification) come in after v1, once the
basic rules have run on a real corpus.

## Term grounding

R1 proposed that every technical noun phrase resolve to an entry or to an item in the
declared background, with anything else flagged. R1 claimed this catches mannered prose
mechanically. 2026-10-10-origin shows the claim goes too far: "dyadic" is ungrounded but
not decorative.

R2 split grounding into three stages:

1. **Registry.** Canonical concept IDs, aliases, symbols and definitions. This is the
   authority.
2. **Candidate discovery.** Noun phrases, symbols and lexical heuristics flag possibly
   unexplained terms.
3. **Semantic review.** A model classifies each candidate, gives its evidence, and
   proposes a definition, a link, or a new entry.

R2 drew two further lines. First, *not found* is not the same as *known to be missing*.
Second, TF-IDF finds unusual terms, not technically meaningful ones. R2 also warned that
inferring prerequisites from link structure (RefD, Liang et al., 2015) is circular when the
existing links are bad. R2's invariant: every *confirmed* concept reference resolves to a
stable ID, and unresolved candidates stay visible as candidates. A weak heuristic then
creates review work and cannot corrupt the graph.

What v1 does about grounding:

- **Confirmed reference.** A confirmed reference is an explicit `[[id]]` or
  `[[id|anchor]]` link. Resolving it is exact and cheap.
- **Unlinked mentions.** `mention.unlinked` is the one candidate-discovery check in v1.
  It matches only aliases already in the registry, so it is precise but blind to terms the
  registry has never seen.
- **"dyadic".** v1 catches "dyadic" only if "dyadic" has been registered somewhere. For a
  term nobody has registered, catching it needs stage 2, which comes after v1.
- **Mathematical notation.** Notation is a second namespace. An undefined symbol confuses
  a reader as much as an undefined term. Grounding symbols requires parsing the math in
  the markdown, so it also comes after v1.

## Correctness tracked separately from comprehensibility

R1: a page can be perfectly clear and wrong. That is the specific risk with AI-written
explainers of the "below n log n" kind. Factual claims get their own track, recording the
claim, its source and its verification status, and a high comprehensibility score never
implies correctness. R2 agreed and listed automated accuracy validation as a v1 priority.

v1 implements the structural half:

- a sources table (id, citation, URL or path);
- inline claim markers that cite source IDs;
- a status field on each claim: `unverified`, `checked` or `disputed`.

Lint checks that cited sources exist and counts unsourced claims per entry. Checking a
claim against the text of its source needs the source text and a model, so it comes after
v1. Reports on comprehensibility and on correctness never share a score.

## Author moves

R1 named the actions on the author's side of the game:

- split an entry;
- extract a concept into its own entry;
- add a worked example;
- add a bridge entry between two concepts;
- inline a short definition instead of linking out;
- rename an ambiguous term.

Each finding maps to a suggested move, which turns "directing the extension of target
entries" from free rewriting into a choice among a few options. The learning-space reading
of cycles (2026-10-10-reader-model) adds one more: **merge** entries whose concepts can
only be learned together. The "Suggested move" column above is that mapping for v1. It is
advice printed with each diagnostic, not an automated edit.

## Open questions for steering

1. **Integrity versus pedagogical.** Is splitting the rules into integrity rules (never
   suppressible) and pedagogical rules (hypotheses) the right cut?
2. **Claim marker syntax.** Options include a footnote-like `[^src:id]`, an inline
   `{claim src=id}` span, or a fenced block. Recommendation: footnote-like, because it
   renders as plain markdown in any viewer.
3. **Suppression syntax.** Options are per-entry frontmatter (`allow: [mention.unlinked]`),
   an inline HTML comment beside the offending span, or both. Recommendation: frontmatter
   in v1, since inline comments need span tracking through the markdown parser.
4. **`goal.missing` severity.** Should an entry without goals be a warning or only
   information in v1? Background and stub pages may legitimately have no goals.

## Current State
- No source file in the repository implements lint rules, term grounding, claim tracking,
  or author-move suggestions — the design appears only in
  docs/journal/2026-10-10-rules-and-grounding.md

## Missing
- No rule engine, rule registry, or diagnostic type with rule ID, class, severity, and
  suggested move
- No wikilink resolution against entries, aliases, and profile items
- No unlinked-mention scan over entry bodies
- No sources table format, claim marker parser, or claim status field
- No typed-exception declarations or obligation checks

# exegenius task 0001-extract

Project directory: `<project>` (paths below are relative to it)

Decompose the source documents into the concepts a specific reader must understand to reach a target result. Your answer is a prerequisite graph that becomes a wiki with one explanatory page per concept the reader does not already know.

- Make one concept per idea that deserves its own explanatory page. Fold minor terms into the gloss of the concept that uses them instead of giving them their own concept.
- Cover everything the target depends on in the sources, including named techniques, constructions, lemmas and notation-heavy objects. Leave out material the target does not depend on.
- `requires` lists the concepts a reader must understand before this one. Use only ids that appear in your answer. The graph must have no cycles.
- Mark `known: true` for concepts the reader profile already covers. Include known concepts that other concepts require, so prerequisites bottom out in what the reader knows, and give known concepts `requires: []`.
- `evidence` quotes must be copied from the sources character for character (a phrase or one sentence), with the id of the source they come from. Submit rejects any quote it cannot find in that source (it ignores differences in whitespace and in typographic quotes and dashes). Every concept the reader does not know needs at least one quote; known concepts may have none.
- A name with mathematics in it ("Polynomial ring modulo y^r + 1") needs a `title`: the name with the mathematics in LaTeX ("Polynomial ring modulo $y^r + 1$"), and in `symbols` a binding for every symbol in that mathematics: a display name of one to four words ("record length"), its meaning in one sentence, and whether it denotes something directly or counts instances of something (for a count, say what it counts: "the number of complex coefficients in a record", not "a power of two"). A reader who knows the concept is shown these bindings and no other explanation of its symbols. Names without mathematics need neither.
- `target` is the id of the concept for the target result.

## Target

The main result of the paper: a deterministic multitape Turing machine algorithm that multiplies n-bit integers in O(n (lg n)^(1-kappa)) time, disproving the n log n optimality conjecture in that model, and how it works

## Reader profile

Undergraduate computer science: big-O notation, polynomials, complex numbers, basic Turing machines and tapes

## Sources (read them from disk)

- `sources/paper.tex`: source id `paper`, "Integer multiplication below n log n (OpenAI, 2026)"

## Answer

The answer is one JSON object following this schema:

```json
{
  "$defs": {
    "Binding": {
      "additionalProperties": false,
      "description": "What a symbol in a concept's name denotes.",
      "properties": {
        "meaning": {
          "description": "One sentence saying what the symbol denotes; for a count, what it counts.",
          "type": "string"
        },
        "name": {
          "description": "Display name of one to four words, e.g. \"record length\".",
          "type": "string"
        },
        "relation": {
          "$ref": "#/$defs/Relation"
        },
        "symbol": {
          "description": "The symbol in LaTeX, as it appears in the title's mathematics, without dollar signs,\ne.g. `r` or `\\kappa`.",
          "type": "string"
        }
      },
      "required": [
        "symbol",
        "name",
        "meaning",
        "relation"
      ],
      "type": "object"
    },
    "Concept": {
      "additionalProperties": false,
      "description": "One concept. This is also the shape of a concept in an extract answer, so the field\ndocs double as instructions in the task file.",
      "properties": {
        "aliases": {
          "description": "Other names and abbreviations the sources use for this concept.",
          "items": {
            "type": "string"
          },
          "type": "array"
        },
        "evidence": {
          "description": "Short quotes copied exactly from the sources that state or use this concept.",
          "items": {
            "$ref": "#/$defs/Evidence"
          },
          "type": "array"
        },
        "gloss": {
          "description": "One precise sentence saying what the concept is, in plain text with Unicode symbols\n(it is shown as a tooltip on links to the concept).",
          "type": "string"
        },
        "id": {
          "description": "Lowercase slug, unique across the graph, e.g. \"chirp-identity\".",
          "type": "string"
        },
        "known": {
          "description": "True if the reader profile already covers this concept.",
          "type": "boolean"
        },
        "name": {
          "description": "Display name in plain text, e.g. \"Chirp identity\". Write symbols in Unicode (κ, ω,\n≤, n log n), not LaTeX or spelled-out Greek: names appear in the navigation, which\ndoes not render mathematics.",
          "type": "string"
        },
        "requires": {
          "description": "Ids of the concepts a reader must understand before this one.",
          "items": {
            "type": "string"
          },
          "type": "array"
        },
        "symbols": {
          "description": "What each symbol in the title's mathematics denotes, one binding per symbol. A reader\nwho knows the concept is shown these, so give every symbol, however minor.",
          "items": {
            "$ref": "#/$defs/Binding"
          },
          "type": "array"
        },
        "title": {
          "description": "Only for a name with mathematics in it: the name with the mathematics in LaTeX, e.g.\n\"Polynomial ring modulo $y^r + 1$\". It is the heading of the concept's page.",
          "type": [
            "string",
            "null"
          ]
        }
      },
      "required": [
        "id",
        "name",
        "aliases",
        "gloss",
        "requires",
        "known",
        "evidence"
      ],
      "type": "object"
    },
    "Evidence": {
      "additionalProperties": false,
      "properties": {
        "quote": {
          "description": "Text copied character for character from that source.",
          "type": "string"
        },
        "source": {
          "description": "Id of the source the quote comes from.",
          "type": "string"
        }
      },
      "required": [
        "source",
        "quote"
      ],
      "type": "object"
    },
    "Relation": {
      "oneOf": [
        {
          "const": "direct",
          "description": "The symbol denotes a thing: \"ω, a primitive root of unity\".",
          "type": "string"
        },
        {
          "const": "count",
          "description": "The symbol is the number of instances of something: \"r, the number of complex\ncoefficients in a record\".",
          "type": "string"
        }
      ]
    }
  },
  "additionalProperties": false,
  "properties": {
    "concepts": {
      "items": {
        "$ref": "#/$defs/Concept"
      },
      "type": "array"
    },
    "target": {
      "description": "Id of the concept that is the target result.",
      "type": "string"
    }
  },
  "required": [
    "target",
    "concepts"
  ],
  "title": "ExtractAnswer",
  "type": "object"
}
```

Write the answer to `exegenius/tasks/0001-extract.answer.json`, then run `exegenius submit 0001-extract` in the project directory. If submit rejects the answer, fix the answer and submit again.

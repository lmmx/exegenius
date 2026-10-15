# exegenius task 0213-support-fast-address-interchange

Project directory: `/tmp/claude-0/-home-user/979e8242-2c77-5242-991d-0bed1baea8dd/scratchpad/a1/runs/A1AO1/a1-279843/support` (paths below are relative to it)

Check whether each quotation on a page of an explanatory wiki supports the claim it is attached to. Writers back claims taken from the sources with a quotation copied from them; a quotation can exist in the source and still not support what it is attached to.

- Judge each claim against its quotation alone, not against what you know or what the source may say elsewhere.
- `supported`: the quotation states the claim or directly implies it.
- `partial`: the quotation supports part of the claim, or a weaker version of it.
- `unsupported`: the quotation does not address what the claim asserts.
- `contradicted`: the quotation says something incompatible with the claim.
- For every label but `supported`, say in one line what is missing or wrong.
- Label every quotation listed, each once.

## Quotations

### 1

Claim: Here $\tau$, the fixed exponent below 1 in the cost, is the exponent in $O(Vu^\tau)$

Quotation, from Integer multiplication below n log n (OpenAI, 2026): "for a fixed $\tau<1$"

Paragraph:

> Fast interchange of address fields is a fixed-tape procedure that swaps two disjoint contiguous address fields of an array, and the multiplication construction uses it to speed up its address moves. The procedure acts on arrays laid out as in [Address fields and array layout on tapes](address-fields.md), so each field is a block of $u$ contiguous bits. Write $H$ and $D$ for the two fields, each viewed as a vector of entries over a common residue ring; for the bit procedure this ring is $\mathbb F_2$, where subtraction and addition coincide. Here $\tau$, the fixed exponent below 1 in the cost, is the exponent in $O(Vu^\tau)$. (On the address-shear page, $\tau$ is a column index; here it means only this exponent.)

### 2

Claim: For the bit network, $s<Wm$

Quotation, from Integer multiplication below n log n (OpenAI, 2026): "the total number $s$ of smaller calls satisfies $s<Wm$"

Paragraph:

> The cost bound follows from [Recursive power saving from s<Wm](recursive-power-saving.md). The network has $W$ roles, and each of its $s$ smaller calls acts on one role stream of volume $V/W$. For the bit network, $s<Wm$; the paper's proof computes $s$ as $Wm$ minus a positive constant fixed by the construction. Hence $s/W<m$. Since $m^\tau\to m$ as $\tau\to1$, fix $\tau<1$ with $s/W<m^\tau$.

## Answer

The answer is one JSON object following this schema:

```json
{
  "$defs": {
    "Label": {
      "additionalProperties": false,
      "properties": {
        "label": {
          "$ref": "#/$defs/Verdict"
        },
        "mark": {
          "description": "Number of the quotation, as listed.",
          "format": "uint",
          "minimum": 0,
          "type": "integer"
        },
        "reason": {
          "description": "What is missing or wrong; may be empty for `supported`.",
          "type": "string"
        }
      },
      "required": [
        "mark",
        "label",
        "reason"
      ],
      "type": "object"
    },
    "Verdict": {
      "oneOf": [
        {
          "const": "supported",
          "description": "The quotation states the claim or directly implies it.",
          "type": "string"
        },
        {
          "const": "partial",
          "description": "The quotation supports part of the claim, or a weaker version.",
          "type": "string"
        },
        {
          "const": "unsupported",
          "description": "The quotation does not address what the claim asserts.",
          "type": "string"
        },
        {
          "const": "contradicted",
          "description": "The quotation says something incompatible with the claim.",
          "type": "string"
        }
      ]
    }
  },
  "additionalProperties": false,
  "properties": {
    "labels": {
      "items": {
        "$ref": "#/$defs/Label"
      },
      "type": "array"
    }
  },
  "required": [
    "labels"
  ],
  "title": "SupportAnswer",
  "type": "object"
}
```

Write the answer to `exegenius/tasks/0213-support-fast-address-interchange.answer.json`, then run `exegenius submit 0213-support-fast-address-interchange` in the project directory. If submit rejects the answer, fix the answer and submit again.

Keep any other files you make, such as scripts, in `exegenius/tasks/0213-support-fast-address-interchange`. Other tasks run at the same time on this machine, and files elsewhere (in /tmp, for example) can be overwritten or run by them. For the same reason, never stop processes by name or pattern (`pkill`, `killall`), which would stop other tasks' commands too: stop a command of your own by its process id.

# exegenius task 0213-support-linear-network-roles

Project directory: `/tmp/claude-0/-home-user/979e8242-2c77-5242-991d-0bed1baea8dd/scratchpad/a1/runs/A1AO1/a1-edc768/support` (paths below are relative to it)

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

Claim: Each role stream then holds $V/(2W)$ bits

Quotation, from Integer multiplication below n log n (OpenAI, 2026): "contains $V/W$ bits"

Paragraph:

> To run the network on an array, regard a row as the complete ordered suffix of the array's address, at a fixed prefix and row index. These rows are the suffixes of [Address fields and array layout on tapes](address-fields.md), so the network acts on rows indexed by those fields. Distribute the rows cyclically: row $\ell$, the $\ell$th row at a fixed prefix, goes to role $\ell \bmod W$, after the row count for each prefix is padded to a multiple of $W$. Let $V$ be the number of bits stored after padding. Each role stream then holds $V/(2W)$ bits.

### 2

Claim: For a scratch role the construction has $\rho(w)=w$ with sign $+1$

Quotation, from Integer multiplication below n log n (OpenAI, 2026): "each scratch input to its own output with sign $+1$"

Paragraph:

> Take an input role $w$ and its route to output role $\rho(w)$, carrying sign $\varepsilon_w \in \{+1,-1\}$. By the telescoping, the route applies $\varepsilon_w D_{\mathrm{sink}(\rho(w))}D_{\mathrm{source}(w)}^{-1}$, where $\mathrm{source}(w)$ and $\mathrm{sink}(\rho(w))$ are the first and last nodes of that route. Undoing the exchange moves each output back to its input role, and multiplying by $\varepsilon_w$ again removes the sign. For a scratch role the construction has $\rho(w)=w$ with sign $+1$, so the undoing needs no correction there. Every role keeps the endpoint operator, auxiliary roles included.

### 3

Claim: Every role keeps the endpoint operator

Quotation, from Integer multiplication below n log n (OpenAI, 2026): "leaves the endpoint operator on every role array"

Paragraph:

> Take an input role $w$ and its route to output role $\rho(w)$, carrying sign $\varepsilon_w \in \{+1,-1\}$. By the telescoping, the route applies $\varepsilon_w D_{\mathrm{sink}(\rho(w))}D_{\mathrm{source}(w)}^{-1}$, where $\mathrm{source}(w)$ and $\mathrm{sink}(\rho(w))$ are the first and last nodes of that route. Undoing the exchange moves each output back to its input role, and multiplying by $\varepsilon_w$ again removes the sign. For a scratch role the construction has $\rho(w)=w$ with sign $+1$, so the undoing needs no correction there. Every role keeps the endpoint operator, auxiliary roles included.

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

Write the answer to `exegenius/tasks/0213-support-linear-network-roles.answer.json`, then run `exegenius submit 0213-support-linear-network-roles` in the project directory. If submit rejects the answer, fix the answer and submit again.

Keep any other files you make, such as scripts, in `exegenius/tasks/0213-support-linear-network-roles`. Other tasks run at the same time on this machine, and files elsewhere (in /tmp, for example) can be overwritten or run by them. For the same reason, never stop processes by name or pattern (`pkill`, `killall`), which would stop other tasks' commands too: stop a command of your own by its process id.

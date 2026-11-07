---
id: py/dot-format-in-exception
title: "An exception .format() message is assigned before the raise"
languages: [python]
fix: suggest
fix_reason: "the message moves into a variable on the line above, named the first of `msg`, `msg2`, … the file does not use; it is now built even if building the exception fails first"
stability: preview
ported_from: [ruff/EM103]
since: 0.5.0
---

An exception is not raised with a `"...".format(...)` message written into the call; the message is assigned to a variable first.

**Why:** The traceback prints the `raise` line and then the message, so the text appears twice.

```codelaw
match:
  kind: call
  has: {field: function, kind: attribute, has: {field: object, kind: string}, regex: '\.format$'}
  inside:
    kind: argument_list
    inside: {kind: call, inside: {kind: raise_statement}}
  any:
    - all: [{ pattern: $MSG }, { inside: { kind: raise_statement, stopBy: end, pattern: $STMT } }]
    - regex: ''
message: "Assign the message to a variable before raising"
fix:
  - insert-before: { target: $STMT, text: "$FRESH(msg) = $MSG", line: true }
  - replace: { target: $MSG, with: "$FRESH(msg)" }
```

## How to fix

1. Write `msg = "...".format(...)` on the line before, and raise with `msg`.

## Examples

### Python

#### Don't

```python
raise ValueError("bad config: {}".format(path))
```

#### Do

```python
msg = "bad config: {}".format(path)
raise ValueError(msg)
```

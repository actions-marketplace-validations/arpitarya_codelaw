---
id: py/dot-format-in-exception
title: "An exception .format() message is assigned before the raise"
languages: [python]
fix: manual
fix_reason: "the message moves into a `msg` variable on the line above, at the raise's indentation"
fix_pending: "suggest; needs computed-edit (W-162)"
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
message: "Assign the message to a variable before raising"
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

---
id: py/raw-string-in-exception
title: "An exception message is assigned before the raise"
languages: [python]
fix: manual
fix_reason: "the message moves into a `msg` variable on the line above, at the raise's indentation"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [ruff/EM101]
since: 0.5.0
---

An exception is not raised with a string literal written into the call: `raise ValueError("bad")` is reported; the message is assigned to a variable first.

**Why:** The traceback prints the `raise` line and then the message, so a literal there appears twice.

```codelaw
match:
  kind: string
  not: {has: {kind: interpolation}}
  inside:
    kind: argument_list
    inside: {kind: call, inside: {kind: raise_statement}}
message: "Assign the message to a variable before raising"
```

## How to fix

1. Write `msg = "..."` on the line before, and raise with `msg`.

## Examples

### Python

#### Don't

```python
raise ValueError("bad config")
```

#### Do

```python
msg = "bad config"
raise ValueError(msg)
```

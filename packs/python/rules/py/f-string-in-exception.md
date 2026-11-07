---
id: py/f-string-in-exception
title: "An exception f-string is assigned before the raise"
languages: [python]
fix: manual
fix_reason: "the message moves into a `msg` variable on the line above, at the raise's indentation"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [ruff/EM102]
since: 0.5.0
---

An exception is not raised with an f-string written into the call: `raise ValueError(f"bad {x}")` is reported; the message is assigned to a variable first.

**Why:** The traceback prints the `raise` line and then the message, so the text appears twice.

```codelaw
match:
  kind: string
  has: {kind: interpolation}
  inside:
    kind: argument_list
    inside: {kind: call, inside: {kind: raise_statement}}
message: "Assign the message to a variable before raising"
```

## How to fix

1. Write `msg = f"..."` on the line before, and raise with `msg`.

## Examples

### Python

#### Don't

```python
raise ValueError(f"bad config: {path}")
```

#### Do

```python
msg = f"bad config: {path}"
raise ValueError(msg)
```

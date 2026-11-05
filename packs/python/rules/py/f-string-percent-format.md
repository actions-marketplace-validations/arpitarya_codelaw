---
id: py/f-string-percent-format
title: "No % formatting of an f-string"
languages: [python]
fix: manual
fix_reason: "which values belong in the string is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/RUF073]
since: 0.5.0
---

An f-string is not the left operand of `%`.

**Why:** The f-string is already formatted; `%` then formats the result again, usually by mistake (and fails on a literal `%`).

```codelaw
match:
  kind: binary_operator
  has: {field: operator, regex: '^%$'}
  all:
    - has: {field: left, kind: string, regex: '^[rRbB]?[fF]'}
message: "This f-string is formatted twice; put the values in the f-string"
```

## How to fix

1. Move the `%` operands into the f-string's `{}` fields and drop `%`.

## Examples

### Python

#### Don't

```python
msg = f"{user}: %s" % reason
```

#### Do

```python
msg = f"{user}: {reason}"
```

---
id: py/collection-literal-concatenation
title: "Unpack into a literal instead of concatenating"
languages: [python]
fix: suggest
fix_reason: "`[*a, *b]` accepts any iterable where `+` raised on a mismatched type"
stability: preview
ported_from: [ruff/RUF005]
since: 0.5.0
---

A list or tuple literal joined to another sequence with `+` is written as one literal that unpacks the other: `[1, 2, *rest]`.

**Why:** One literal builds the result in one step and reads as the shape it has.

```codelaw
match:
  kind: binary_operator
  pattern: "[$$$A] + $B"
  has: {field: left, has: {nthChild: 1}}
constraints:
  B: {kind: identifier}
message: "Unpack `$B` into the literal instead of concatenating"
fix: "[$$$A, *$B]"
```

## How to fix

1. Write one literal: `[x, y, *rest]`.

## Examples

### Python

#### Don't

```python
items = [first] + rest
```

#### Do

```python
items = [first, *rest]
```

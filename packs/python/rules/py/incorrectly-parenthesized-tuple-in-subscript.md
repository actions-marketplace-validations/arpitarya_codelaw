---
id: py/incorrectly-parenthesized-tuple-in-subscript
title: "No parentheses around a subscript tuple"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF031]
since: 0.5.0
---

A tuple used as a subscript is written without parentheses: `grid[x, y]`.

**Why:** The parentheses add nothing; the tuple is built the same way.

```codelaw
match:
  kind: subscript
  pattern: $D[($A, $$$R)]
  not: {has: {field: subscript, has: {kind: list_splat}}}
message: "Drop the parentheses around the subscript tuple"
fix: "$D[$A, $$$R]"
```

## Examples

### Python

#### Don't

```python
cell = grid[(x, y)]
```

#### Do

```python
cell = grid[x, y]
```

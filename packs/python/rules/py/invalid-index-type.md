---
id: py/invalid-index-type
title: "Index with an int, not another literal"
languages: [python]
fix: manual
fix_reason: "which index was meant is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/RUF016]
since: 0.5.0
---

A list, tuple or string literal is not indexed with a float, string, list, dict or other non-integer literal.

**Why:** That raises TypeError every time the line runs.

```codelaw
match:
  kind: subscript
  all:
    - has: {field: value, any: [{kind: list}, {kind: tuple}, {kind: string}, {kind: list_comprehension}]}
    - has: {field: subscript, any: [{kind: float}, {kind: string}, {kind: list}, {kind: dictionary}, {kind: set}, {kind: none}]}
message: "A sequence literal is indexed with a non-integer; this raises TypeError"
```

## How to fix

1. Use an integer index or a slice.

## Examples

### Python

#### Don't

```python
x = [1, 2, 3]["0"]
```

#### Do

```python
x = [1, 2, 3][0]
```

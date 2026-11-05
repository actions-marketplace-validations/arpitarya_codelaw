---
id: py/unnecessary-comprehension
title: "list(y), not [x for x in y]"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/C416]
since: 0.5.0
---

A list comprehension that only copies its iterable, `[x for x in y]`, is written `list(y)`.

**Why:** `list` says it is a copy.

```codelaw
match:
  kind: list_comprehension
  pattern: "[$X for $X in $Y]"
  not: {has: {any: [{kind: if_clause}, {kind: for_in_clause, nthChild: 3}, {kind: for_in_clause, regex: '^async\s'}]}}
constraints:
  X: {kind: identifier}
message: "Write `list($Y)`"
fix: "list($Y)"
```

## Examples

### Python

#### Don't

```python
copy = [x for x in items]
```

#### Do

```python
copy = list(items)
```

---
id: py/parenthesize-chained-operators
title: "Parenthesize `and` inside `or`"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF021]
since: 0.5.0
---

An `and` expression that is an operand of `or` is parenthesized: `(a and b) or c`.

**Why:** `and` binds tighter than `or`, which few readers remember; the parentheses say which grouping is meant without changing it.

```codelaw
match:
  kind: boolean_operator
  has: {field: operator, regex: '^and$'}
  inside: {kind: boolean_operator, has: {field: operator, regex: '^or$'}}
message: "Parenthesize the `and` inside `or`"
fix: "($$MATCH)"
```

## Examples

### Python

#### Don't

```python
ok = a and b or c
```

#### Do

```python
ok = (a and b) or c
```

---
id: py/unnecessary-literal-within-tuple-call
title: "A tuple literal, not tuple([...])"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/C409]
since: 0.5.0
---

`tuple([1, 2])` is written `(1, 2)`.

**Why:** The literal builds the tuple directly.

```codelaw
match:
  pattern: tuple([$$$A])
  has: {field: arguments, has: {kind: list, has: {nthChild: 2}}}
message: "Write a tuple literal"
fix: "($$$A)"
```

## Examples

### Python

#### Don't

```python
pair = tuple([x, y])
```

#### Do

```python
pair = (x, y)
```

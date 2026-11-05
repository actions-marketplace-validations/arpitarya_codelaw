---
id: py/reimplemented-operator
title: "operator.itemgetter, not a lambda"
languages: [python]
fix: suggest
fix_reason: "`operator` must be imported; the function's name in tracebacks changes"
stability: preview
ported_from: [ruff/FURB118]
since: 0.5.0
---

`lambda x: x[0]` is written `operator.itemgetter(0)`, and `lambda a, b: a + b` `operator.add`.

**Why:** The `operator` functions say what the lambda does, and are faster.

```codelaw
match:
  pattern: "lambda $X: $X[$K]"
constraints:
  K: {any: [{kind: integer}, {kind: string}]}
message: "Use `operator.itemgetter($K)`"
fix: "operator.itemgetter($K)"
```

## Examples

### Python

#### Don't

```python
rows.sort(key=lambda r: r[1])
```

#### Do

```python
rows.sort(key=operator.itemgetter(1))
```

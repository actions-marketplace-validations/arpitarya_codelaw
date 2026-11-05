---
id: py/expr-or-not-expr
title: "No x or not x"
languages: [python]
fix: auto
fix_reason: "auto when the operand is a plain name, in a condition"
stability: preview
ported_from: [ruff/SIM221]
since: 0.5.0
---

`x or not x` is always truthy and is written `True`.

**Why:** It can never be false; a different operand was probably meant.

```codelaw
match:
  pattern: $X or not $X
  inside: {any: [{kind: if_statement}, {kind: elif_clause}, {kind: while_statement}]}
constraints:
  X: {kind: identifier}
message: "`$X or not $X` is always True"
fix: "True"
```

## Examples

### Python

#### Don't

```python
if ready or not ready:
    go()
```

#### Do

```python
if True:
    go()
```

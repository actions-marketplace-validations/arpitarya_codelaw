---
id: py/expr-and-not-expr
title: "No x and not x"
languages: [python]
fix: auto
fix_reason: "auto when the operand is a plain name, which has no side effect"
stability: preview
ported_from: [ruff/SIM220]
since: 0.5.0
---

`x and not x` is always falsy and is written `False`.

**Why:** It can never be true; a different operand was probably meant.

```codelaw
match:
  pattern: $X and not $X
  inside: {any: [{kind: if_statement}, {kind: elif_clause}, {kind: while_statement}]}
constraints:
  X: {kind: identifier}
message: "`$X and not $X` is always False"
fix: "False"
```

## Examples

### Python

#### Don't

```python
if ready and not ready:
    go()
```

#### Do

```python
if False:
    go()
```

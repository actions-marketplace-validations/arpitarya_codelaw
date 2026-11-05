---
id: py/expr-and-false
title: "No `and False` in a condition"
languages: [python]
fix: auto
fix_reason: "auto in a condition when the left operand is a plain name"
stability: preview
ported_from: [ruff/SIM223]
since: 0.5.0
---

In a condition, `x and False` is always false and is written `False`.

**Why:** The branch never runs; the `and False` is usually left over from debugging.

```codelaw
match:
  pattern: $X and False
  inside: {any: [{kind: if_statement}, {kind: elif_clause}, {kind: while_statement}]}
constraints:
  X: {kind: identifier}
message: "This condition is always False"
fix: "False"
```

## Examples

### Python

#### Don't

```python
if debug and False:
    trace()
```

#### Do

```python
if False:
    trace()
```

---
id: py/expr-or-true
title: "No `or True` in a condition"
languages: [python]
fix: auto
fix_reason: "auto in a condition when the left operand is a plain name, so the truth value is all that matters"
stability: preview
ported_from: [ruff/SIM222, pylint/condition-evals-to-constant]
since: 0.5.0
---

In a condition, `x or True` is always true and is written `True`.

**Why:** The condition decides nothing; the `or True` is usually left over from debugging.

```codelaw
match:
  pattern: $X or True
  inside: {any: [{kind: if_statement}, {kind: elif_clause}, {kind: while_statement}]}
constraints:
  X: {kind: identifier}
message: "This condition is always True"
fix: "True"
```

## Examples

### Python

#### Don't

```python
if debug or True:
    trace()
```

#### Do

```python
if True:
    trace()
```

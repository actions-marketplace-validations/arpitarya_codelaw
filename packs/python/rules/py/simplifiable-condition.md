---
id: py/simplifiable-condition
title: "No `and True` in a condition"
languages: [python]
fix: auto
fix_reason: "auto in a condition, where only the truth value is used"
stability: preview
ported_from: [pylint/simplifiable-condition]
since: 0.5.0
---

In an `if` or `while` condition, `x and True` is written `x`.

**Why:** `and True` changes nothing about the test.

```codelaw
match:
  pattern: $X and True
  inside: {any: [{kind: if_statement}, {kind: elif_clause}, {kind: while_statement}]}
message: "Write `$X`"
fix: "$X"
```

## Examples

### Python

#### Don't

```python
if ready and True:
    go()
```

#### Do

```python
if ready:
    go()
```

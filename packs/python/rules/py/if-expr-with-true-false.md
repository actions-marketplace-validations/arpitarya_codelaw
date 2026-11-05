---
id: py/if-expr-with-true-false
title: "bool(c), not True if c else False"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/SIM210, pylint/simplifiable-if-expression]
since: 0.5.0
---

`True if c else False` is written `bool(c)`.

**Why:** The conditional restates the truth value.

```codelaw
match:
  pattern: True if $C else False
message: "Write `bool($C)`"
fix: "bool($C)"
```

## Examples

### Python

#### Don't

```python
flag = True if items else False
```

#### Do

```python
flag = bool(items)
```

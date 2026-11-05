---
id: py/needless-bool
title: "Return the condition, not True or False"
languages: [python]
fix: suggest
fix_reason: "`bool(...)` calls `__bool__`, just as the `if` did; the text changes"
stability: preview
ported_from: [ruff/SIM103, pylint/simplifiable-if-statement]
since: 0.5.0
---

`if cond: return True else: return False` is written `return bool(cond)`.

**Why:** The branches restate the condition.

```codelaw
match:
  kind: if_statement
  pattern: |
    if $C:
        return True
    else:
        return False
message: "Return `bool($C)` directly"
fix: "return bool($C)"
```

## Examples

### Python

#### Don't

```python
def ok(x):
    if x > 0:
        return True
    else:
        return False
```

#### Do

```python
def ok(x):
    return bool(x > 0)
```

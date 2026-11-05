---
id: py/if-exp-instead-of-or-operator
title: "a or b, not a if a else b"
languages: [python]
fix: auto
fix_reason: "auto when `a` is a plain name, which `or` evaluates once just as the conditional did"
stability: preview
ported_from: [ruff/FURB110]
since: 0.5.0
---

`x if x else y` is written `x or y`.

**Why:** `or` says the same in one operator.

```codelaw
match:
  pattern: $X if $X else $Y
constraints:
  X: {kind: identifier}
message: "Write `$X or $Y`"
fix: "$X or $Y"
```

## Examples

### Python

#### Don't

```python
name = given if given else "anonymous"
```

#### Do

```python
name = given or "anonymous"
```

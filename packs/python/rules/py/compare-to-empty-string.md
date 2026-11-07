---
id: py/compare-to-empty-string
title: "No comparison with an empty string"
languages: [python]
fix: manual
fix_reason: "a truth test also accepts `None` and other falsy values; whether that is right depends on what the variable can hold"
fix_exception: value
stability: preview
ported_from: [ruff/PLC1901]
since: 0.5.0
---

`x == ""` and `x != ""` are written as a truth test, `not x` and `x`.

**Why:** An empty string is falsy; the comparison spells it out at length.

```codelaw
match:
  kind: comparison_operator
  regex: '(==|!=)\s*[rbuRBU]*(""|'''')$|^[rbuRBU]*(""|'''')\s*(==|!=)'
message: "Test the string's truth instead"
```

## How to fix

1. Write `not x` for `x == ""` and `x` for `x != ""` when `x` is always a string.

## Examples

### Python

#### Don't

```python
if name == "":
    pass
```

#### Do

```python
if not name:
    pass
```

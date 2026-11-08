---
id: py/positional-only-arguments-expected
title: "Positional-only parameters are passed by position"
languages: [python]
fix: manual
fix_reason: "which positional slot each keyword stands for is clear only to the author when more than one is passed"
fix_exception: value
stability: preview
ported_from: [pylint/positional-only-arguments-expected]
since: 0.5.0
---

A call `f(a=1)` to a plain module-level function of this file, defined `def f(a, /, b)`, that names a positional-only parameter as a keyword. A function that also takes `**kwargs` is not read (the keyword is absorbed).

**Why:** Python raises `TypeError: f() got some positional-only arguments passed as keyword arguments`.

```codelaw
native: py/positional-only-arguments-expected
```

## How to fix

1. Pass the argument by position.

## Examples

### Python

#### Don't

```python
def area(width, height, /):
    return width * height


area(width=2, height=3)
```

#### Do

```python
def area(width, height, /):
    return width * height


area(2, 3)
```

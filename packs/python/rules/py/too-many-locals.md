---
id: py/too-many-locals
title: "A function has at most `max` local variables"
languages: [python]
fix: manual
fix_reason: "splitting the code is a design choice: which parts belong together is the author's"
fix_exception: restructure
stability: preview
options:
  max: { type: "integer", default: 15, description: "the most local variables a function may assign" }
ported_from: [ruff/PLR0914, pylint/too-many-locals]
since: 0.5.0
---

A function that assigns more than `max` distinct local names (15 by default) is reported on its name. As in Ruff, a name counts when its last binding in the function is an assignment (`x = …`, `x += …`, a tuple target); parameters, loop variables, `with … as`, imports and names declared `global` or `nonlocal` do not.

**Why:** Each local is one more thing to keep in mind while reading; many of them mean the function does several things.

```codelaw
native: py/too-many-locals
```

## How to fix

1. Split the function, or group related values in one object.

## Examples

### Python

#### Don't

```python options={"max":2}
def total(items):
    a = 1
    b = 2
    c = 3
    return a + b + c
```

#### Do

```python options={"max":2}
def total(items):
    a = 1
    b = 2
    return a + b
```

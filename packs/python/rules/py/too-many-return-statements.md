---
id: py/too-many-return-statements
title: "A function has at most `max` return statements"
languages: [python]
fix: manual
fix_reason: "splitting the code is a design choice: which parts belong together is the author's"
fix_exception: restructure
stability: preview
options:
  max: { type: "integer", default: 6, description: "the most `return` statements a function may have" }
ported_from: [ruff/PLR0911, pylint/too-many-return-statements]
since: 0.5.0
---

A function (`def` or `async def`, a method included) with more than `max` `return` statements (6 by default) is reported on its name. Returns in a nested function, class or lambda count for that one, not this.

**Why:** Many exits make a function hard to follow and to change; a lookup table or smaller functions usually say it more plainly.

```codelaw
native: py/too-many-return-statements
```

## How to fix

1. Replace a chain of returns with a lookup table, or split the function.

## Examples

### Python

#### Don't

```python options={"max":2}
def grade(n):
    if n > 90:
        return "A"
    if n > 80:
        return "B"
    return "C"
```

#### Do

```python options={"max":2}
def grade(n):
    if n > 90:
        return "A"
    return "B"
```

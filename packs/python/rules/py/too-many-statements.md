---
id: py/too-many-statements
title: "A function has at most `max` statements"
languages: [python]
fix: manual
fix_reason: "splitting the code is a design choice: which parts belong together is the author's"
fix_exception: restructure
stability: preview
options:
  max: { type: "integer", default: 50, description: "the most statements a function's body may hold" }
ported_from: [ruff/PLR0915, pylint/too-many-statements]
since: 0.5.0
---

A function with more than `max` statements (50 by default) is reported on its name, counting as Pylint does: each statement once, an `if`, `elif`, `else`, `while`, `try`, handler, `with` and nested `def` once more than its body, a `finally` twice, a `for` only its body, and a `return` not at all.

**Why:** A long function does many things; each one named in its own function can be read, tested and reused alone.

```codelaw
native: py/too-many-statements
```

## How to fix

1. Split the function into smaller ones, each doing one named thing.

## Examples

### Python

#### Don't

```python options={"max":2}
def run():
    a = 1
    b = 2
    c = 3
```

#### Do

```python options={"max":2}
def run():
    a = 1
    b = 2
```

---
id: py/used-before-assignment
title: "A name is bound before it is read"
languages: [python]
fix: manual
fix_reason: "which value the read was meant to see is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/used-before-assignment]
since: 0.5.0
---

A name the scope binds is read where no path from the start of the scope to the read has bound it yet, or, in an `except` handler, where the only binding is in the `try` body and something that can raise runs before it. The scope is a module or a function: names are bound by assignments, `for`, `with`, `import`, `def`, `class`, `except ... as`, `:=` and `match` captures. Nested functions, lambdas, class bodies and comprehension bodies are not read. Both branches of an `if` that always runs (`if FLAG:` is not special) are ordinary branches, so Pylint's inference of constant conditions is not done.

**Why:** Python raises `UnboundLocalError` (in a function) or `NameError` (at module level) at that read.

```codelaw
native: py/used-before-assignment
```

## How to fix

1. Move the assignment above the read, or
2. give the name a value on every path before it, or
3. rename the read if it meant another variable.

## Examples

### Python

#### Don't

```python
def total(items):
    result = result + 1
    return result
```

#### Do

```python
def total(items):
    result = 0
    result = result + 1
    return result
```

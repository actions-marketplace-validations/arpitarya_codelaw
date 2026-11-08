---
id: py/nonlocal-without-binding
title: "A `nonlocal` name is bound in an enclosing function"
languages: [python]
fix: manual
fix_reason: "which enclosing function should own the name, or whether it should be `global`, is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE0117, pylint/nonlocal-without-binding]
since: 0.5.0
---

A `nonlocal` statement in a function names a variable that no enclosing function binds. Class and module scopes do not count, a `global` declaration in the enclosing function ends the search, and a `del name` counts as a binding, as it does for Python's compiler.

**Why:** Python refuses to compile it: `SyntaxError: no binding for nonlocal 'x' found`.

```codelaw
native: py/nonlocal-without-binding
```

## How to fix

1. Bind the name in the enclosing function the inner one should share, or
2. use `global` if it is a module variable, or drop the `nonlocal` and make the name local.

## Examples

### Python

#### Don't

```python
def counter():
    def bump():
        nonlocal total
        total += 1
    return bump
```

#### Do

```python
def counter():
    total = 0

    def bump():
        nonlocal total
        total += 1
    return bump
```

---
id: py/missing-f-string-syntax
title: "A string that names variables in scope is an f-string"
languages: [python]
fix: suggest
fix_reason: "adding `f` changes the string's value, which is the point but is not safe when the braces were meant literally"
stability: preview
ported_from: [ruff/RUF027]
since: 0.5.0
---

A plain string literal has `{name}` placeholders and every simple name in them is a variable bound in scope, so it was probably meant to be an f-string. Not read: a bare string statement (a docstring), a string a method is called on (`"{x}".format(x=x)`), a `gettext`/`_` argument, a logging call's argument, an annotation, a call that already passes the name (`f("{x}", x=x)`), a name that is a builtin or not in scope, and text that is not a valid f-string.

**Why:** The placeholders print as written instead of being filled.

```codelaw
native: py/missing-f-string-syntax
```

## How to fix

1. Prefix the string with `f`, or
2. escape the braces as `{{` and `}}` if they are meant literally.

## Examples

### Python

#### Don't

```python
def greet(name):
    return "Hello, {name}!"
```

#### Fixed

```python
def greet(name):
    return f"Hello, {name}!"
```

#### Do

```python
def greet(name):
    return f"Hello, {name}!"
```

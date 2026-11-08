---
id: py/f-string-docstring
title: "Docstrings aren't f-strings"
languages: [python]
fix: suggest
fix_reason: "the `f` goes and `{{ }}` are undoubled: the text becomes `__doc__`, placeholders as written; tools that read docstrings (Click, Typer, FastAPI) then see it. An interpolation spanning lines or holding the quote is manual"
fix_exception: restructure
stability: preview
ported_from: [ruff/B021]
since: 0.1.0
---

The first statement of a module, class or function isn't an f-string meant as its docstring.

**Why:** Python treats an f-string there as an expression, so `__doc__` stays empty.

```codelaw
native: py/f-string-docstring
```

## How to fix

1. Remove the `f` prefix and write the text without placeholders.

## Examples

### Python

#### Don't

```py
def greet():
    f"""Say hello to {NAME}."""
```

#### Fixed

```py
def greet():
    """Say hello to {NAME}."""
```

#### Do

```py
def greet():
    """Say hello to the configured name."""
```

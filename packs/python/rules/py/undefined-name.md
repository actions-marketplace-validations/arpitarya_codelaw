---
id: py/undefined-name
title: "Every name read is defined"
languages: [python]
fix: suggest
fix_reason: "the names in scope and the builtins within two edits are choices, the nearest the default; the NameError becomes a read of that name. A name with none near is manual, as the right one is a value the code doesn't hold"
fix_exception: value
stability: preview
options:
  builtins: { type: "string[]", default: [], description: "extra names to treat as builtins" }
ported_from: [ruff/F821, pylint/undefined-variable]
since: 0.1.0
---

Every name the module reads is bound in an enclosing scope, imported, or one of Python's builtins.

**Why:** An undefined name raises `NameError` when the line runs — usually a typo or a missing import.

```codelaw
native: py/undefined-name
```

## How to fix

1. Correct a misspelled name, or import or define it.

## Examples

### Python

#### Don't

```py
def total(items):
    return sum(item.price for item in itmes)
```

#### Do

```py
def total(items):
    return sum(item.price for item in items)
```

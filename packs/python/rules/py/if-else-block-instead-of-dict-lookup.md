---
id: py/if-else-block-instead-of-dict-lookup
title: "A lookup table, not an if chain of returns"
languages: [python]
fix: manual
fix_reason: "the dictionary's name and where it lives (module constant or local) are the author's"
fix_exception: restructure
stability: preview
ported_from: [ruff/SIM116]
since: 0.5.0
---

An `if` / `elif` chain that compares one name with `==` to three or more distinct literals, each branch only `return`ing a value with no call in it (an `else` returning likewise is allowed), is reported: it is a dictionary lookup. A chain with any other statement, comparison or name is not read.

**Why:** A dictionary shows the mapping at a glance and is changed in one place.

```codelaw
native: py/if-else-block-instead-of-dict-lookup
```

## How to fix

1. Write the pairs in a dictionary and return `table.get(name, default)`.

## Examples

### Python

#### Don't

```python
def color(code):
    if code == 1:
        return "red"
    elif code == 2:
        return "green"
    elif code == 3:
        return "blue"
    return None
```

#### Do

```python
COLORS = {1: "red", 2: "green", 3: "blue"}


def color(code):
    return COLORS.get(code)
```

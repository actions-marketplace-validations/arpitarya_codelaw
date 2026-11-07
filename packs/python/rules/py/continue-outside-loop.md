---
id: py/continue-outside-loop
title: "`continue` only inside a loop"
languages: [python]
fix: suggest
fix_reason: "the choices are `return`, in a function, and deleting the `continue` (`pass` when its block empties); a SyntaxError today, and which the author meant is theirs to say, so neither is the default"
stability: preview
ported_from: [ruff/F702]
since: 0.1.0
---

`continue` appears only inside the body of a `for` or `while` loop in the same function.

**Why:** Outside a loop it is a `SyntaxError`.

```codelaw
native: py/continue-outside-loop
```

## How to fix

1. Move the `continue` into the loop, or restructure the code.

## Examples

### Python

#### Don't

```py
def skip():
    continue
```

#### Fixed (return)

```py
def skip():
    return
```

#### Fixed (delete)

```py
def skip():
    pass
```

#### Do

```py
def skip(items):
    for item in items:
        continue
```

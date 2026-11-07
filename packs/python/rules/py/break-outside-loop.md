---
id: py/break-outside-loop
title: "`break` only inside a loop"
languages: [python]
fix: suggest
fix_reason: "the choices are `return`, in a function, and deleting the `break` (`pass` when its block empties); a SyntaxError today, and which the author meant is theirs to say, so neither is the default"
stability: preview
ported_from: [ruff/F701, pylint/not-in-loop]
since: 0.1.0
---

`break` appears only inside the body of a `for` or `while` loop in the same function.

**Why:** Outside a loop it is a `SyntaxError`.

```codelaw
native: py/break-outside-loop
```

## How to fix

1. Use `return` to leave a function, or move the `break` into the loop.

## Examples

### Python

#### Don't

```py
def stop():
    break
```

#### Fixed (return)

```py
def stop():
    return
```

#### Fixed (delete)

```py
def stop():
    pass
```

#### Do

```py
def stop(items):
    for item in items:
        break
```

---
id: py/return-outside-function
title: "`return` only inside a function"
languages: [python]
fix: suggest
fix_reason: "a SyntaxError today, so nothing ran: the `return` goes (`pass` when its block empties), and a value that does something stays as a statement; wrapping it in a function would need a name the code doesn't hold"
stability: preview
ported_from: [ruff/F706]
since: 0.1.0
---

`return` appears only inside a function.

**Why:** At module or class level it is a `SyntaxError`.

```codelaw
native: py/return-outside-function
```

## How to fix

1. Move the code into a function, or remove the `return`.

## Examples

### Python

#### Don't

```py
class Config:
    return None
```

#### Fixed

```py
class Config:
    pass
```

#### Do

```py
class Config:
    def load(self):
        return None
```

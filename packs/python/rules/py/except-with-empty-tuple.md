---
id: py/except-with-empty-tuple
title: "No `except ():`"
languages: [python]
fix: suggest
fix_reason: "the handler never runs, so it goes; the only handler of a `try` without `finally` takes the `try` with it (its body, then its `else`, dedented). The code runs the same but loses what signals a bug, so review it"
stability: preview
ported_from: [ruff/B029]
since: 0.1.0
---

An `except` clause doesn't name an empty tuple.

**Why:** `except ():` catches nothing, so the handler never runs.

```codelaw
native: py/except-with-empty-tuple
```

## How to fix

1. Name the exceptions to catch, or remove the handler.

## Examples

### Python

#### Don't

```py
try:
    run()
except ():
    log()
```

#### Fixed

```py
run()
```

#### Do

```py
try:
    run()
except ValueError:
    log()
```

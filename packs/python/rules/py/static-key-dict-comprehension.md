---
id: py/static-key-dict-comprehension
title: "A dict comprehension's key varies"
languages: [python]
fix: suggest
fix_reason: "one choice per loop target: it becomes the key, and the dict gets an entry per item instead of one; none is the default"
stability: preview
ported_from: [ruff/B035]
since: 0.1.0
---

A dict comprehension's key depends on the loop variables.

**Why:** A constant key gives a dict with one entry, keeping only the last value.

```codelaw
native: py/static-key-dict-comprehension
```

## How to fix

1. Use a key that depends on the loop variable.

## Examples

### Python

#### Don't

```py
sizes = {"size": len(name) for name in names}
```

#### Fixed (name)

```py
sizes = {name: len(name) for name in names}
```

#### Do

```py
sizes = {name: len(name) for name in names}
```

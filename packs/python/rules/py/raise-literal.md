---
id: py/raise-literal
title: "Raise exceptions, not literals"
languages: [python]
fix: suggest
fix_reason: "the literal is wrapped in `RuntimeError(...)`; it raised `TypeError` before, so a handler for that no longer catches it"
stability: preview
ported_from: [ruff/B016, pylint/raising-non-exception, pylint/raising-bad-type]
since: 0.1.0
---

`raise` is given an exception, not a string or number literal.

**Why:** Raising a literal is a `TypeError` in Python 3.

```codelaw
native: py/raise-literal
```

## How to fix

1. Raise an exception class with the literal as its message.

## Examples

### Python

#### Don't

```py
raise "failed"
```

#### Fixed

```py
raise RuntimeError("failed")
```

#### Do

```py
raise RuntimeError("failed")
```

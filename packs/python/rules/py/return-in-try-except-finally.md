---
id: py/return-in-try-except-finally
title: "No return in finally beside one in try"
languages: [python]
fix: manual
fix_reason: "which value should be returned is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/SIM107]
since: 0.5.0
---

A `finally` does not `return` when its `try` or `except` also returns.

**Why:** The `finally` return silently replaces the other, and swallows any exception in flight.

```codelaw
match:
  kind: return_statement
  inside:
    kind: finally_clause
    stopBy: end
    inside: {kind: try_statement, has: {field: body, has: {stopBy: end, kind: return_statement}}}
message: "This return overrides the one in `try`"
```

## How to fix

1. Return once, after the `try` statement, or only in `try`/`except`.

## Examples

### Python

#### Don't

```python
def f():
    try:
        return 1
    finally:
        return 2
```

#### Do

```python
def f():
    try:
        value = 1
    finally:
        cleanup()
    return value
```

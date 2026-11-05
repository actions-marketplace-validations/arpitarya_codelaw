---
id: py/except-with-non-exception-classes
title: "`except` names exception classes"
languages: [python]
fix: manual
fix_reason: "the exception class to catch is the author's choice"
fix_pending: "suggest"
stability: preview
ported_from: [ruff/B030, pylint/catching-non-exception]
since: 0.1.0
---

An `except` clause names exception classes (or tuples of them), not literals or other values.

**Why:** Catching anything else raises `TypeError` the moment an exception reaches the handler.

```codelaw
native: py/except-with-non-exception-classes
```

## How to fix

1. Replace the value with the exception class you meant to catch.

## Examples

### Python

#### Don't

```py
try:
    run()
except "error":
    log()
```

#### Do

```py
try:
    run()
except RuntimeError:
    log()
```

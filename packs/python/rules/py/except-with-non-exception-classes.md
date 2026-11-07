---
id: py/except-with-non-exception-classes
title: "`except` names exception classes"
languages: [python]
fix: suggest
fix_reason: "a string naming a builtin exception becomes that exception, and another literal in a tuple with classes goes; any exception reaching the handler raised `TypeError` before, so what it catches changes. Anything else is manual, as the class meant is a value the code doesn't hold"
fix_exception: value
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
except "ValueError":
    log()
```

#### Fixed

```py
try:
    run()
except ValueError:
    log()
```

#### Do

```py
try:
    run()
except RuntimeError:
    log()
```

---
id: py/function-call-in-default-argument
title: "No function calls in default arguments"
languages: [python]
fix: suggest
fix_reason: "the choices are a `None` default with the call in the body (it runs on every call, likely what was meant) and, for a module-level function, the call as a module constant above it (it runs once, as now, and says so); a call inside a larger default is manual"
fix_exception: restructure
stability: preview
options:
  extend_immutable_calls: { type: "string[]", default: [], description: "more calls whose results are immutable, by qualified name" }
ported_from: [ruff/B008]
since: 0.1.0
---

A parameter's default isn't computed by a function call, except calls known to return immutable values (`tuple()`, `frozenset()`, `int()`, `re.compile()`, `datetime.timedelta()`, …).

**Why:** The call runs once, when the function is defined, so every call shares the same result — a stale timestamp, or one shared object.

```codelaw
native: py/function-call-in-default-argument
```

## How to fix

1. Default to `None` and compute the value in the body, or
2. compute it once into a module-level constant and use that as the default.

## Examples

### Python

#### Don't

```py
import time


def log(message, when=time.time()):
    print(when, message)
```

#### Fixed (None default)

```py
import time


def log(message, when=None):
    if when is None:
        when = time.time()
    print(when, message)
```

#### Fixed (module constant)

```py
import time


_LOG_WHEN_DEFAULT = time.time()


def log(message, when=_LOG_WHEN_DEFAULT):
    print(when, message)
```

#### Do

```py
import time


def log(message, when=None):
    if when is None:
        when = time.time()
    print(when, message)
```

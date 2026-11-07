---
id: py/mutable-contextvar-default
title: "No mutable `ContextVar` defaults"
languages: [python]
fix: suggest
fix_reason: "a list becomes a tuple and a set a frozenset (their comprehensions and `list()`/`set()` calls too); code that changes the value in place stops working, so review the uses. A dict or a computed default is manual, as its immutable form is the author's call"
fix_exception: value
stability: preview
ported_from: [ruff/B039]
since: 0.1.0
---

A `ContextVar`'s `default=` isn't a list, dict, set or the result of a call.

**Why:** The default is one object shared by every context, so mutating it leaks state between them.

```codelaw
native: py/mutable-contextvar-default
```

## How to fix

1. Use an immutable default, or no default and set a fresh value in each context.

## Examples

### Python

#### Don't

```py
from contextvars import ContextVar

items = ContextVar("items", default=[])
```

#### Fixed

```py
from contextvars import ContextVar

items = ContextVar("items", default=())
```

#### Do

```py
from contextvars import ContextVar

items = ContextVar("items", default=())
```

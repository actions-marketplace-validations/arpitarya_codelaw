---
id: py/useless-contextlib-suppress
title: "`contextlib.suppress()` names an exception"
languages: [python]
fix: suggest
fix_reason: "the `with` line goes and its body is dedented: it suppressed nothing, so the code runs the same, but the exception the author forgot to name is gone from view, so review it. A `with` with an `as` target, other context managers or a multi-line string is manual"
fix_exception: restructure
stability: preview
ported_from: [ruff/B022]
since: 0.1.0
---

`contextlib.suppress()` is called with at least one exception class.

**Why:** With no arguments it suppresses nothing, so the `with` block is misleading.

```codelaw
native: py/useless-contextlib-suppress
```

## How to fix

1. Pass the exceptions to suppress, or remove the `with` statement.

## Examples

### Python

#### Don't

```py
import contextlib

with contextlib.suppress():
    remove("cache")
```

#### Fixed

```py
import contextlib

remove("cache")
```

#### Do

```py
import contextlib

with contextlib.suppress(FileNotFoundError):
    remove("cache")
```

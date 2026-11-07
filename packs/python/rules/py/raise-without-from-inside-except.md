---
id: py/raise-without-from-inside-except
title: "Raising in `except` says `from`"
languages: [python]
fix: suggest
fix_reason: "the choices are `from err` (the caught error kept as the cause, bound `as err` when the handler doesn't name it) and `from None` (hidden); either changes the traceback, so neither is the default. A raise over several lines is manual"
fix_exception: restructure
stability: preview
ported_from: [ruff/B904, pylint/raise-missing-from]
since: 0.1.0
---

A `raise` inside an `except` block that raises a new exception says `from err` or `from None`.

**Why:** Without it, the traceback reads as if a second error happened while handling the first.

```codelaw
native: py/raise-without-from-inside-except
```

## How to fix

1. Bind the caught exception (`except E as err:`).
2. Raise with `from err`, or `from None` to hide the original deliberately.

## Examples

### Python

#### Don't

```py
try:
    value = int(text)
except ValueError:
    raise TypeError("not a number")
```

#### Fixed (from err)

```py
try:
    value = int(text)
except ValueError as err:
    raise TypeError("not a number") from err
```

#### Fixed (from None)

```py
try:
    value = int(text)
except ValueError:
    raise TypeError("not a number") from None
```

#### Do

```py
try:
    value = int(text)
except ValueError as err:
    raise TypeError("not a number") from err
```

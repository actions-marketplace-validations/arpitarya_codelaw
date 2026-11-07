---
id: py/jump-statement-in-finally
title: "No `return`, `break` or `continue` in `finally`"
languages: [python]
fix: suggest
fix_reason: "the `return`, `break` or `continue` ending a `finally` moves to right after the `try` (the `finally` goes when that was all it held and a handler stays, else `pass` stands in): an exception now propagates instead of being swallowed, which is the fix and a change. A jump elsewhere in the block is manual"
fix_exception: restructure
stability: preview
ported_from: [ruff/B012, pylint/lost-exception, pylint/return-in-finally, pylint/break-in-finally]
since: 0.1.0
---

A `finally` block doesn't leave through `return`, `break` or `continue`.

**Why:** Leaving `finally` that way silently discards any exception in flight.

```codelaw
native: py/jump-statement-in-finally
```

## How to fix

1. Move the `return`, `break` or `continue` out of the `finally` block.
2. Handle the exception explicitly with `except` if it should be swallowed.

## Examples

### Python

#### Don't

```py
def read(path):
    try:
        return load(path)
    finally:
        return None
```

#### Fixed

```py
def read(path):
    try:
        return load(path)
    finally:
        pass
    return None
```

#### Do

```py
def read(path):
    try:
        return load(path)
    finally:
        close(path)
```

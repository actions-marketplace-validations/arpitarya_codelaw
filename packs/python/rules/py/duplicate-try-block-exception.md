---
id: py/duplicate-try-block-exception
title: "An exception is caught by one handler only"
languages: [python]
fix: auto
fix_reason: "an earlier handler of the `try` catches the class first, so this one never runs for it: a class named by a plain name or attribute goes from the later tuple, and a later handler with nothing else live goes whole; any other repeat (a computed class, `except*`) is manual"
fix_exception: restructure
stability: preview
ported_from: [ruff/B025]
since: 0.1.0
---

Within one `try` statement, each exception class appears in only one `except` clause.

**Why:** Only the first matching handler runs, so the later one is dead code.

```codelaw
native: py/duplicate-try-block-exception
```

## How to fix

1. Merge the handlers' code into the first one, or remove the dead one.

## Examples

### Python

#### Don't

```py
try:
    run()
except ValueError:
    retry()
except ValueError:
    log()
```

#### Fixed

```py
try:
    run()
except ValueError:
    retry()
```

#### Do

```py
try:
    run()
except ValueError:
    retry()
    log()
```

---
id: py/logging-exc-info
title: "logger.exception, not error(exc_info=True)"
languages: [python]
fix: suggest
fix_reason: "`exception` must be called from an exception handler, which this keyword implied"
stability: preview
ported_from: [ruff/G201]
since: 0.5.0
---

Inside an `except` block, `logger.error(..., exc_info=True)` is written `logger.exception(...)`.

**Why:** `exception` says the traceback is attached.

```codelaw
match:
  pattern: $L.error($M, exc_info=True)
  inside: {kind: except_clause, stopBy: end}
message: "Use `$L.exception($M)`"
fix: "$L.exception($M)"
```

## Examples

### Python

#### Don't

```python
try:
    go()
except OSError:
    logger.error("failed", exc_info=True)
```

#### Do

```python
try:
    go()
except OSError:
    logger.exception("failed")
```

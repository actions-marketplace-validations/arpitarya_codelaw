---
id: py/error-instead-of-exception
title: "logger.exception in an except block"
languages: [python]
fix: suggest
fix_reason: "the log record gains the traceback"
stability: preview
ported_from: [ruff/TRY400]
since: 0.5.0
---

Inside an `except` block, `logger.error(...)` is written `logger.exception(...)`.

**Why:** `exception` attaches the traceback the handler is about.

```codelaw
match:
  kind: attribute
  pattern: $L.error
  inside:
    kind: call
    field: function
    not: {has: {field: arguments, has: {kind: keyword_argument, has: {field: name, regex: '^exc_info$'}}}}
    inside: {kind: except_clause, stopBy: {kind: function_definition}}
constraints:
  L: {regex: '^(logging|logger|log|LOGGER|LOG|_logger|_log|self\.logger|self\.log)$'}
message: "Use `$L.exception` to keep the traceback"
fix: "$L.exception"
```

## Examples

### Python

#### Don't

```python
try:
    go()
except OSError:
    logger.error("failed")
```

#### Do

```python
try:
    go()
except OSError:
    logger.exception("failed")
```

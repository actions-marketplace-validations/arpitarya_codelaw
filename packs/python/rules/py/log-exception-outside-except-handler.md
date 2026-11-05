---
id: py/log-exception-outside-except-handler
title: "logger.exception only in an except block"
languages: [python]
fix: suggest
fix_reason: "outside a handler there is no traceback to log, so `error` logs the same"
stability: preview
ported_from: [ruff/LOG004]
since: 0.5.0
---

Outside an `except` block, `logger.exception(...)` is written `logger.error(...)`.

**Why:** There is no exception to attach; `exception` logs `NoneType: None`.

```codelaw
match:
  kind: attribute
  pattern: $L.exception
  inside: {kind: call, field: function, not: {inside: {kind: except_clause, stopBy: end}}}
  not: {inside: {kind: function_definition, stopBy: end}}
constraints:
  L: {regex: '^(logging|logger|log|LOGGER|LOG|_logger|_log)$'}
message: "No active exception here; use `$L.error`"
fix: "$L.error"
```

## Examples

### Python

#### Don't

```python
logger.exception("startup failed")
```

#### Do

```python
logger.error("startup failed")
```

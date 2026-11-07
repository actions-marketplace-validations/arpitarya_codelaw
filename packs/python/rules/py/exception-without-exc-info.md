---
id: py/exception-without-exc-info
title: "No logger.exception with exc_info=False"
languages: [python]
fix: suggest
fix_reason: "the call becomes `.error(...)`, which logs at the same level without the traceback, as `exc_info=False` asked"
stability: preview
ported_from: [ruff/LOG007]
since: 0.5.0
---

`.exception(..., exc_info=False)` is written `.error(...)`.

**Why:** `exception` exists to log the traceback; turning that off leaves an `error` call under a misleading name.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, kind: attribute, has: {field: attribute, regex: '^exception$', pattern: $M}}
    - has: {field: arguments, has: {kind: keyword_argument, pattern: $K, all: [{has: {field: name, regex: '^exc_info$'}}, {has: {field: value, regex: '^(False|0|None)$'}}]}}
message: "Write `.error(...)`"
fix:
  - replace: {target: $M, with: error}
  - remove: $K
```

## Examples

### Python

#### Don't

```python
log.exception("sync failed", exc_info=False)
```

#### Do

```python
log.error("sync failed")
```

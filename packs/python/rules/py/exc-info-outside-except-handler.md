---
id: py/exc-info-outside-except-handler
title: "No exc_info=True outside an except block"
languages: [python]
fix: suggest
fix_reason: "outside a handler there is no traceback, so dropping the keyword logs the same"
stability: preview
ported_from: [ruff/LOG014]
since: 0.5.0
---

Outside an `except` block (and outside functions, which may be called from one) a logging call does not pass `exc_info=True`.

**Why:** There is no exception to attach.

```codelaw
match:
  kind: keyword_argument
  regex: '^exc_info\s*=\s*True$'
  inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^(logging|logger|log|LOGGER|LOG|_logger|_log)\.(debug|info|warning|error|critical|log)$'}}}
  not: {inside: {stopBy: end, any: [{kind: except_clause}, {kind: function_definition}]}}
message: "No active exception here; drop `exc_info=True`"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
logger.error("startup failed", exc_info=True)
```

#### Do

```python
logger.error("startup failed")
```

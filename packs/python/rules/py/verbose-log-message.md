---
id: py/verbose-log-message
title: "logger.exception does not repeat the exception"
languages: [python]
fix: manual
fix_reason: "the message's wording is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/TRY401]
since: 0.5.0
---

Inside `except … as e:`, a `.exception(...)` logging call does not pass `e` again: `logger.exception` already logs the exception and its traceback.

**Why:** The exception's text then appears twice in the log line.

```codelaw
match:
  kind: call
  inside:
    kind: except_clause
    stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
    has: {kind: as_pattern, has: {kind: as_pattern_target, has: {kind: identifier, pattern: $N}}}
  all:
    - has: {field: function, kind: attribute, regex: '\.exception$'}
    - has: {field: arguments, has: {kind: identifier, pattern: $N, stopBy: {any: [{kind: lambda}, {kind: call}]}}}
message: "`$N` is already logged by `.exception`"
```

## How to fix

1. Remove the exception from the call's arguments and from the message's format string.

## Examples

### Python

#### Don't

```python
try:
    sync()
except OSError as e:
    log.exception(f"sync failed: {e}")
```

#### Do

```python
try:
    sync()
except OSError:
    log.exception("sync failed")
```

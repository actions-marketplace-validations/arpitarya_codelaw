---
id: py/timeout-error-alias
title: "TimeoutError, not asyncio.TimeoutError"
languages: [python]
fix: suggest
fix_reason: "the alias is the builtin only from Python 3.11"
stability: preview
ported_from: [ruff/UP041]
since: 0.5.0
---

`asyncio.TimeoutError` and `socket.timeout` are written `TimeoutError` (Python 3.11+).

**Why:** They are aliases of the builtin.

```codelaw
match:
  kind: attribute
  regex: '^(asyncio\.TimeoutError|socket\.timeout)$'
  not: {inside: {kind: attribute}}
message: "Write `TimeoutError`"
fix: "TimeoutError"
```

## Examples

### Python

#### Don't

```python
try:
    await job
except asyncio.TimeoutError:
    pass
```

#### Do

```python
try:
    await job
except TimeoutError:
    pass
```

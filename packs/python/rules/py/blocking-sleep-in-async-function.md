---
id: py/blocking-sleep-in-async-function
title: "No time.sleep in an async function"
languages: [python]
fix: suggest
fix_reason: "`asyncio.sleep` yields to the event loop, which changes scheduling; it needs `asyncio` imported"
stability: preview
ported_from: [ruff/ASYNC251]
since: 0.5.0
---

An `async def` does not call `time.sleep`; it awaits `asyncio.sleep`.

**Why:** `time.sleep` blocks the whole event loop.

```codelaw
match:
  kind: call
  pattern: time.sleep($T)
  inside: {kind: function_definition, stopBy: {kind: function_definition}, regex: '^async\s'}
  not: {inside: {kind: await}}
message: "This blocks the event loop; `await asyncio.sleep($T)`"
fix: "await asyncio.sleep($T)"
```

## Examples

### Python

#### Don't

```python
async def poll():
    time.sleep(1)
```

#### Do

```python
async def poll():
    await asyncio.sleep(1)
```

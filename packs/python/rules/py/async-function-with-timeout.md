---
id: py/async-function-with-timeout
title: "An async function has no timeout parameter"
languages: [python]
fix: manual
fix_reason: "the callers wrap the call in a timeout context instead"
fix_exception: restructure
stability: preview
ported_from: [ruff/ASYNC109]
since: 0.5.0
---

An `async def` does not take a `timeout` parameter; the caller bounds it with `asyncio.timeout(...)` (or trio's and anyio's `fail_after`).

**Why:** A timeout context cancels the whole operation, every await in it; a parameter has to be threaded through by hand and is easy to get wrong.

```codelaw
match:
  kind: identifier
  regex: '^timeout$'
  any:
    - inside: {kind: parameters}
    - inside: {kind: default_parameter, field: name, inside: {kind: parameters}}
    - inside: {kind: typed_default_parameter, field: name, inside: {kind: parameters}}
    - inside: {kind: typed_parameter, inside: {kind: parameters}}
  inside: {kind: function_definition, regex: '^async\s', stopBy: end}
message: "Let the caller set the timeout with a timeout context"
```

## How to fix

1. Remove the parameter and have callers write `async with asyncio.timeout(seconds): await f()`.

## Examples

### Python

#### Don't

```python
async def fetch(url, timeout=10):
    return await get(url, timeout)
```

#### Do

```python
async def fetch(url):
    return await get(url)
```

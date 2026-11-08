---
id: py/blocking-http-call-httpx-in-async-function
title: "An async function does not call a synchronous `httpx.Client`"
languages: [python]
fix: manual
fix_reason: "moving to `httpx.AsyncClient` changes the client's creation, its calls (`await`) and its closing"
fix_exception: restructure
stability: preview
ported_from: [ruff/ASYNC212]
since: 0.5.0
---

In an `async def`, a call `client.get(...)` (or `post`, `put`, `delete`, `head`, `options`, `patch`, `request`, `send`, `stream`, `close`) on a name bound once to `httpx.Client` — by an annotation (alone, optional or in a union), by `client = httpx.Client(...)`, or by `with httpx.Client() as client`. A client built in another function and handed over untyped is unknown to this rule.

**Why:** The call blocks the event loop for the length of the request.

```codelaw
native: py/blocking-http-call-httpx-in-async-function
```

## How to fix

1. Use `httpx.AsyncClient` and `await` its calls.

## Examples

### Python

#### Don't

```python
import httpx


async def fetch(url):
    client = httpx.Client()
    return client.get(url)
```

#### Do

```python
import httpx


async def fetch(url):
    async with httpx.AsyncClient() as client:
        return await client.get(url)
```

---
id: py/blocking-http-call-in-async-function
title: "No blocking HTTP call in an async function"
languages: [python]
fix: manual
fix_reason: "the call is replaced by an async client such as `httpx.AsyncClient`, which the code around it awaits"
fix_exception: restructure
stability: preview
ported_from: [ruff/ASYNC210]
since: 0.5.0
---

An `async def` does not call a blocking HTTP client — `requests.get` and the rest, `httpx.get` and the rest, `urllib.request.urlopen`, `urllib3.request`: it blocks the event loop until it returns. A call in a nested `def` or `lambda` is not reported.

**Why:** While the call blocks, no other task on the loop runs.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^(requests\.(get|post|put|patch|delete|head|options|request)|httpx\.(get|post|put|patch|delete|head|options|request|stream)|urllib\.request\.urlopen|urllib3\.request)$'}
  inside: {kind: function_definition, regex: '^async\s', stopBy: {any: [{kind: function_definition}, {kind: lambda}]}}

message: "This call blocks the event loop"
```

## How to fix

1. Use an async client such as `httpx.AsyncClient`, or run the blocking call in a thread with `await asyncio.to_thread(...)`.

## Examples

### Python

#### Don't

```python
async def fetch(url):
    return requests.get(url).json()
```

#### Do

```python
async def fetch(url):
    async with httpx.AsyncClient() as client:
        return (await client.get(url)).json()
```

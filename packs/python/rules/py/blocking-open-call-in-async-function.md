---
id: py/blocking-open-call-in-async-function
title: "No blocking file open in an async function"
languages: [python]
fix: manual
fix_reason: "the call is replaced by an async file API (`anyio.open_file`, `aiofiles.open`), which the code around it awaits"
fix_exception: restructure
stability: preview
ported_from: [ruff/ASYNC230]
since: 0.5.0
---

An `async def` does not call a function that opens a file — `open`, `io.open`, `io.open_code`, `codecs.open`, `tempfile.TemporaryFile`, `NamedTemporaryFile`, `SpooledTemporaryFile`: it blocks the event loop until it returns. A call in a nested `def` or `lambda` is not reported.

**Why:** While the call blocks, no other task on the loop runs.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^(open|io\.open|io\.open_code|codecs\.open|tempfile\.(TemporaryFile|NamedTemporaryFile|SpooledTemporaryFile))$'}
  inside: {kind: function_definition, regex: '^async\s', stopBy: {any: [{kind: function_definition}, {kind: lambda}]}}

message: "This call blocks the event loop"
```

## How to fix

1. Use an async file API (`anyio.open_file`, `aiofiles.open`), or run the blocking call in a thread with `await asyncio.to_thread(...)`.

## Examples

### Python

#### Don't

```python
async def load(path):
    with open(path) as f:
        return f.read()
```

#### Do

```python
async def load(path):
    async with await anyio.open_file(path) as f:
        return await f.read()
```

---
id: py/create-subprocess-in-async-function
title: "No blocking subprocess creation in an async function"
languages: [python]
fix: manual
fix_reason: "the call is replaced by `asyncio.create_subprocess_exec`, which the code around it awaits"
fix_exception: restructure
stability: preview
ported_from: [ruff/ASYNC220]
since: 0.5.0
---

An `async def` does not call `subprocess.Popen` or `os.popen`: it blocks the event loop until it returns. A call in a nested `def` or `lambda` is not reported.

**Why:** While the call blocks, no other task on the loop runs.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^(subprocess\.Popen|os\.popen)$'}
  inside: {kind: function_definition, regex: '^async\s', stopBy: {any: [{kind: function_definition}, {kind: lambda}]}}

message: "This call blocks the event loop"
```

## How to fix

1. Use `asyncio.create_subprocess_exec`, or run the blocking call in a thread with `await asyncio.to_thread(...)`.

## Examples

### Python

#### Don't

```python
async def start(cmd):
    return subprocess.Popen(cmd)
```

#### Do

```python
async def start(cmd):
    return await asyncio.create_subprocess_exec(*cmd)
```

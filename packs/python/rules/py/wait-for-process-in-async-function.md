---
id: py/wait-for-process-in-async-function
title: "No blocking wait for a process in an async function"
languages: [python]
fix: manual
fix_reason: "the call is replaced by an asyncio subprocess and `await proc.wait()`, which the code around it awaits"
fix_exception: restructure
stability: preview
ported_from: [ruff/ASYNC222]
since: 0.5.0
---

An `async def` does not call `os.wait`, `os.wait3`, `os.wait4`, `os.waitid` or `os.waitpid`: it blocks the event loop until it returns. A call in a nested `def` or `lambda` is not reported.

**Why:** While the call blocks, no other task on the loop runs.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^os\.(wait|wait3|wait4|waitid|waitpid)$'}
  inside: {kind: function_definition, regex: '^async\s', stopBy: {any: [{kind: function_definition}, {kind: lambda}]}}

message: "This call blocks the event loop"
```

## How to fix

1. Use an asyncio subprocess and `await proc.wait()`, or run the blocking call in a thread with `await asyncio.to_thread(...)`.

## Examples

### Python

#### Don't

```python
async def reap(pid):
    os.waitpid(pid, 0)
```

#### Do

```python
async def reap(proc):
    await proc.wait()
```

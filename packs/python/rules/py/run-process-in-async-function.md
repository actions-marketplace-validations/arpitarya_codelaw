---
id: py/run-process-in-async-function
title: "No blocking process run in an async function"
languages: [python]
fix: manual
fix_reason: "the call is replaced by `asyncio.create_subprocess_exec` and `await proc.wait()`, which the code around it awaits"
fix_exception: restructure
stability: preview
ported_from: [ruff/ASYNC221]
since: 0.5.0
---

An `async def` does not call a function that runs a process to its end — `subprocess.run`, `call`, `check_call`, `check_output`, `getoutput`, `getstatusoutput`, `os.system`, `os.posix_spawn`, `os.posix_spawnp`: it blocks the event loop until it returns. A call in a nested `def` or `lambda` is not reported.

**Why:** While the call blocks, no other task on the loop runs.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^(subprocess\.(run|call|check_call|check_output|getoutput|getstatusoutput)|os\.(system|posix_spawn|posix_spawnp))$'}
  inside: {kind: function_definition, regex: '^async\s', stopBy: {any: [{kind: function_definition}, {kind: lambda}]}}

message: "This call blocks the event loop"
```

## How to fix

1. Use `asyncio.create_subprocess_exec` and `await proc.wait()`, or run the blocking call in a thread with `await asyncio.to_thread(...)`.

## Examples

### Python

#### Don't

```python
async def build():
    subprocess.run(["make"])
```

#### Do

```python
async def build():
    proc = await asyncio.create_subprocess_exec("make")
    await proc.wait()
```

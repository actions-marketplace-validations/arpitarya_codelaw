---
id: py/async-busy-wait
title: "No sleep loop waiting for a condition"
languages: [python]
fix: manual
fix_reason: "the waiting side and the side that sets the condition both change to use an event"
fix_exception: restructure
stability: preview
ported_from: [ruff/ASYNC110]
since: 0.5.0
---

A `while` loop whose body is only `await asyncio.sleep(...)` (or `trio.sleep`, `anyio.sleep`) waits on an `asyncio.Event` instead.

**Why:** The loop wakes on a timer, not when the condition changes: it either polls too often or reacts late.

```codelaw
match:
  kind: while_statement
  has:
    field: body
    all:
      - has: {nthChild: 1, kind: expression_statement, has: {kind: await, has: {kind: call, has: {field: function, regex: '^(asyncio|trio|anyio)\.sleep$'}}}}
      - not: {has: {nthChild: 2}}
message: "Wait on an event instead of sleeping in a loop"
```

## How to fix

1. Have the code that makes the condition true set an `asyncio.Event`, and `await event.wait()` here.

## Examples

### Python

#### Don't

```python
async def wait_ready(job):
    while not job.done:
        await asyncio.sleep(0.1)
```

#### Do

```python
async def wait_ready(job):
    await job.done_event.wait()
```

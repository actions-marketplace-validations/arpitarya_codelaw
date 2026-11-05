---
id: py/asyncio-dangling-task
title: "Keep a reference to every asyncio task"
languages: [python]
fix: manual
fix_reason: "the name to keep the task under, and where it is awaited, are the author's"
fix_exception: value
stability: preview
ported_from: [ruff/RUF006]
since: 0.5.0
---

The task `asyncio.create_task()` or `asyncio.ensure_future()` returns is stored, not dropped.

**Why:** The event loop holds only a weak reference to a task; one nobody stores can be garbage-collected before it finishes.

```codelaw
match:
  kind: expression_statement
  has:
    kind: call
    nthChild: 1
    has: {field: function, regex: '^(asyncio\.)?(create_task|ensure_future)$'}
message: "Store this task; a task nobody references can be collected before it runs"
```

## How to fix

1. Assign the task to a name or add it to a set kept for the program's life, and await it or discard it when it completes.

## Examples

### Python

#### Don't

```python
async def main():
    asyncio.create_task(work())
```

#### Do

```python
async def main():
    task = asyncio.create_task(work())
    await task
```

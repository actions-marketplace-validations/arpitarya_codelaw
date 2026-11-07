---
id: py/cancel-scope-no-checkpoint
title: "A timeout scope contains an await"
languages: [python]
fix: manual
fix_reason: "the awaits the timeout was meant to bound belong inside it"
fix_exception: restructure
stability: preview
ported_from: [ruff/ASYNC100]
since: 0.5.0
---

A `with` block opened by a timeout or cancel scope — `asyncio.timeout`, `asyncio.timeout_at`, trio's and anyio's `fail_after`, `fail_at`, `move_on_after`, `move_on_at`, `trio.CancelScope` — contains an `await`, `async for` or `async with`.

**Why:** A scope is only cancelled at an await; without one inside, the timeout can never fire.

```codelaw
match:
  kind: with_statement
  has:
    kind: with_clause
    has: {kind: with_item, has: {kind: call, has: {field: function, regex: '^(asyncio\.(timeout|timeout_at)|(trio|anyio)\.(fail_after|fail_at|move_on_after|move_on_at|CancelScope))$'}}}
  not:
    has:
      field: body
      has:
        stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
        any: [{kind: await}, {kind: for_statement, regex: '^async\s'}, {kind: with_statement, regex: '^async\s'}]
message: "Nothing in this scope awaits, so its timeout never fires"
```

## How to fix

1. Move the awaited calls the timeout should bound into the block, or remove the scope.

## Examples

### Python

#### Don't

```python
async def f():
    async with asyncio.timeout(5):
        compute()
```

#### Do

```python
async def f():
    async with asyncio.timeout(5):
        await fetch()
```

---
id: py/yield-in-context-manager-in-async-generator
title: "An async generator does not yield inside a with block"
languages: [python]
fix: manual
fix_reason: "the values are collected inside the block and yielded after it"
fix_exception: restructure
stability: preview
ported_from: [ruff/ASYNC119]
since: 0.5.0
---

An async generator does not `yield` from inside a `with` or `async with` block. A function marked `@asynccontextmanager` is not reported: yielding inside the block is how it works.

**Why:** The generator may be suspended at the yield and never resumed; the block's exit then runs late, or in another task, or not at all.

```codelaw
match:
  kind: yield
  inside:
    kind: with_statement
    stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
    inside:
      kind: function_definition
      regex: '^async\s'
      stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
      not: {inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(contextlib\.)?asynccontextmanager$'}}}
message: "Do not yield inside a context manager in an async generator"
```

## How to fix

1. Read what is needed inside the block and yield it after the block has closed.

## Examples

### Python

#### Don't

```python
async def rows(path):
    async with open_db(path) as db:
        for row in db.all():
            yield row
```

#### Do

```python
async def rows(path):
    async with open_db(path) as db:
        all_rows = db.all()
    for row in all_rows:
        yield row
```

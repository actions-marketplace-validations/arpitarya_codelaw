---
id: py/yield-from-in-async-function
title: "No yield from in an async function"
languages: [python]
fix: manual
fix_reason: "the delegation is rewritten as `async for` with `yield`"
fix_exception: restructure
stability: preview
ported_from: [ruff/PLE1700]
since: 0.5.0
---

An `async def` does not use `yield from`.

**Why:** That is a SyntaxError.

```codelaw
match:
  kind: yield
  regex: '^yield\s+from\b'
  inside: {kind: function_definition, stopBy: {kind: function_definition}, regex: '^async\s'}
message: "`yield from` is not allowed in an async function"
```

## How to fix

1. Write `async for item in source: yield item`.

## Examples

### Python

#### Don't

```python
async def gen():
    yield from items
```

#### Do

```python
async def gen():
    for item in items:
        yield item
```

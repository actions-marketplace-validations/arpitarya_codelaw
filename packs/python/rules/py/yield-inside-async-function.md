---
id: py/yield-inside-async-function
title: "No yield from in an async function"
languages: [python]
fix: manual
fix_reason: "the delegation is rewritten as a loop"
fix_exception: restructure
stability: preview
ported_from: [pylint/yield-inside-async-function]
since: 0.5.0
---

An `async def` does not use `yield from`.

**Why:** `yield from` is a syntax error in an async function.

```codelaw
match:
  kind: yield
  regex: '^yield\s+from\b'
  inside: {kind: function_definition, regex: '^async\s', stopBy: {any: [{kind: function_definition}, {kind: lambda}]}}
message: "`yield from` is not allowed in `async def`"
```

## How to fix

1. Iterate the sub-generator (`async for` for an async one) and yield each item.

## Examples

### Python

#### Don't

```python
async def rows():
    yield from source()
```

#### Do

```python
async def rows():
    for row in source():
        yield row
```

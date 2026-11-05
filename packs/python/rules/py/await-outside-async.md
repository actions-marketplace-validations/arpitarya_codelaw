---
id: py/await-outside-async
title: "No await outside an async function"
languages: [python]
fix: manual
fix_reason: "making the function async changes every caller"
fix_exception: restructure
stability: preview
ported_from: [ruff/PLE1142]
since: 0.5.0
---

`await` appears only inside an `async def`.

**Why:** Elsewhere it is a SyntaxError.

```codelaw
match:
  kind: await
  inside:
    kind: function_definition
    stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
    not: {regex: '^async\s'}
message: "`await` needs an `async def`"
```

## How to fix

1. Make the function `async def` and await it at its callers.

## Examples

### Python

#### Don't

```python
def fetch():
    return await client.get()
```

#### Do

```python
async def fetch():
    return await client.get()
```

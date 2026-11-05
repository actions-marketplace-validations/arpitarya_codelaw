---
id: py/unused-async
title: "async def awaits something"
languages: [python]
fix: manual
fix_reason: "dropping `async` changes every caller, which must stop awaiting it"
fix_exception: restructure
stability: preview
ported_from: [ruff/RUF029]
since: 0.5.0
---

An `async def` contains an `await`, `async for` or `async with`; otherwise it is a plain function. Decorated functions, methods and nested functions (which usually implement an async interface such as a callback) are left alone.

**Why:** A coroutine that never awaits only adds the cost and ceremony of being awaited.

```codelaw
match:
  kind: function_definition
  regex: '^async\s'
  all:
    - not: {has: {stopBy: end, any: [{kind: await}, {regex: '^async\s+(for|with)\b'}, {kind: yield}]}}
    - not: {inside: {kind: decorated_definition}}
    - not: {inside: {kind: function_definition, stopBy: end}}
    - not: {inside: {kind: block, inside: {kind: class_definition}}}
    - has: {field: body, kind: block, not: {regex: '^(pass|\.\.\.|raise NotImplementedError.*)$'}}
message: "This async function never awaits; make it a plain `def`"
```

## How to fix

1. Remove `async` from the definition.
2. Remove `await` at every call site.

## Examples

### Python

#### Don't

```python
async def total(xs):
    return sum(xs)
```

#### Do

```python
def total(xs):
    return sum(xs)
```

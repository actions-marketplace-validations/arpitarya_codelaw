---
id: py/blocking-input-in-async-function
title: "No input() in an async function"
languages: [python]
fix: manual
fix_reason: "the call is replaced by a read from the terminal in a thread (`await asyncio.to_thread(input, ...)`), which the code around it awaits"
fix_exception: restructure
stability: preview
ported_from: [ruff/ASYNC250]
since: 0.5.0
---

An `async def` does not call `input()`: it blocks the event loop until it returns. A call in a nested `def` or `lambda` is not reported.

**Why:** While the call blocks, no other task on the loop runs.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^input$'}
  inside: {kind: function_definition, regex: '^async\s', stopBy: {any: [{kind: function_definition}, {kind: lambda}]}}

message: "This call blocks the event loop"
```

## How to fix

1. Use a read from the terminal in a thread (`await asyncio.to_thread(input, ...)`), or run the blocking call in a thread with `await asyncio.to_thread(...)`.

## Examples

### Python

#### Don't

```python
async def ask():
    return input("name? ")
```

#### Do

```python
async def ask():
    return await asyncio.to_thread(input, "name? ")
```

---
id: py/async-zero-sleep
title: "lowlevel.checkpoint(), not sleep(0)"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/ASYNC115]
since: 0.5.0
---

`trio.sleep(0)` and `anyio.sleep(0)` are written `trio.lowlevel.checkpoint()` and `anyio.lowlevel.checkpoint()`.

**Why:** The checkpoint says it yields to the scheduler, which is all a zero sleep does.

```codelaw
match:
  pattern: $L.sleep(0)
constraints:
  L: {regex: '^(trio|anyio)$'}
message: "Write `$L.lowlevel.checkpoint()`"
fix: "$L.lowlevel.checkpoint()"
```

## Examples

### Python

#### Don't

```python
async def tick():
    await trio.sleep(0)
```

#### Do

```python
async def tick():
    await trio.lowlevel.checkpoint()
```

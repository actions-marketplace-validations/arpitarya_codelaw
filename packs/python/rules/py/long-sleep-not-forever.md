---
id: py/long-sleep-not-forever
title: "sleep_forever(), not a day-long sleep"
languages: [python]
fix: suggest
fix_reason: "the task then sleeps forever instead of waking after the long delay"
stability: preview
ported_from: [ruff/ASYNC116]
since: 0.5.0
---

`trio.sleep(n)` or `anyio.sleep(n)` with a literal over a day (86 400 s) is written `sleep_forever()`.

**Why:** A sleep that long means forever; saying so is clearer.

```codelaw
match:
  pattern: $L.sleep($N)
constraints:
  L: {regex: '^(trio|anyio)$'}
  N: {kind: integer, regex: '^([1-9]\d{6,}|[2-9]\d{5}|1[0-9]\d{4}|9\d{4}|8[7-9]\d{3}|86[5-9]\d{2}|864[1-9]\d|8640[1-9])$'}
message: "Write `$L.sleep_forever()`"
fix: "$L.sleep_forever()"
```

## Examples

### Python

#### Don't

```python
async def idle():
    await trio.sleep(100000)
```

#### Do

```python
async def idle():
    await trio.sleep_forever()
```

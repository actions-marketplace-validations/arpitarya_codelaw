---
id: py/trio-sync-call
title: "Await trio's async functions"
languages: [python]
fix: suggest
fix_reason: "adding `await` runs the call, which until now only built a coroutine"
stability: preview
ported_from: [ruff/ASYNC105]
since: 0.5.0
---

A call to one of trio's async functions (`trio.sleep`, `trio.open_file`, …) in an `async def` is awaited.

**Why:** Without `await` the coroutine is created and dropped; nothing runs.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: ^trio\.(sleep|sleep_until|sleep_forever|open_file|open_tcp_stream|open_tcp_listeners|open_ssl_over_tcp_stream|serve_tcp|run_process|aclose_forcefully|wait_readable|wait_writable)$}
  - inside: {kind: expression_statement}
  - not:
      inside: {kind: await}
  - inside: {kind: function_definition, stopBy: end, regex: ^async\s}
message: Await this call
fix: await $$MATCH
```

## Examples

### Python

#### Don't

```python
async def main():
    trio.sleep(1)
```

#### Do

```python
async def main():
    await trio.sleep(1)
```

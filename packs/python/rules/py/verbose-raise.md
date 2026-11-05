---
id: py/verbose-raise
title: "raise, not raise e"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/TRY201]
since: 0.5.0
---

In `except E as e:`, re-raising the caught exception is written `raise`, not `raise e`.

**Why:** A bare `raise` keeps the traceback exactly; `raise e` adds a line to it.

```codelaw
match:
  kind: raise_statement
  pattern: raise $E
  inside:
    kind: except_clause
    stopBy: {kind: function_definition}
    has: {kind: as_pattern, has: {kind: as_pattern_target, has: {kind: identifier, pattern: $E}}}
constraints:
  E: {kind: identifier}
message: "Write a bare `raise`"
fix: "raise"
```

## Examples

### Python

#### Don't

```python
try:
    go()
except OSError as e:
    log(e)
    raise e
```

#### Do

```python
try:
    go()
except OSError as e:
    log(e)
    raise
```

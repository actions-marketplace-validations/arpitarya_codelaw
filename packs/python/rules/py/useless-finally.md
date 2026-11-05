---
id: py/useless-finally
title: "No empty finally"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF072]
since: 0.5.0
---

A `finally:` whose body is only `pass` is removed (when the `try` keeps an `except`).

**Why:** It runs nothing.

```codelaw
match:
  kind: finally_clause
  regex: '^finally\s*:\s*(pass|\.\.\.)\s*$'
  inside: {kind: try_statement, has: {kind: except_clause}}
message: "This `finally` does nothing; remove it"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
try:
    go()
except OSError:
    log()
finally:
    pass
```

#### Do

```python
try:
    go()
except OSError:
    log()
```

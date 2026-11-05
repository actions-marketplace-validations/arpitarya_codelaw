---
id: py/os-error-alias
title: "OSError, not its aliases"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP024]
since: 0.5.0
---

`IOError`, `EnvironmentError`, `WindowsError`, `socket.error`, `select.error` and `mmap.error` are written `OSError`.

**Why:** They are all aliases of `OSError` since Python 3.3.

```codelaw
match:
  any: [{kind: identifier}, {kind: attribute}]
  regex: '^(IOError|EnvironmentError|WindowsError|socket\.error|select\.error|mmap\.error)$'
  not: {inside: {kind: attribute}}
  inside: {stopBy: end, any: [{kind: except_clause}, {kind: raise_statement}]}
message: "Write `OSError`"
fix: "OSError"
```

## Examples

### Python

#### Don't

```python
try:
    load()
except IOError:
    pass
```

#### Do

```python
try:
    load()
except OSError:
    pass
```

---
id: py/sys-exit-alias
title: "sys.exit(), not exit()"
languages: [python]
fix: suggest
fix_reason: "`sys` must be imported where the site builtin was used"
stability: preview
ported_from: [ruff/PLR1722, pylint/consider-using-sys-exit]
since: 0.5.0
---

`exit()` and `quit()` are written `sys.exit()`.

**Why:** `exit` and `quit` are added by `site` for the interactive shell and may be missing (`python -S`, frozen apps).

```codelaw
match:
  kind: identifier
  regex: '^(exit|quit)$'
  inside: {kind: call, field: function}
message: "Use `sys.exit`"
fix: "sys.exit"
```

## Examples

### Python

#### Don't

```python
if failed:
    exit(1)
```

#### Do

```python
if failed:
    sys.exit(1)
```

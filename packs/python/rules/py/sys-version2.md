---
id: py/sys-version2
title: "sys.version_info, not sys.version[2]"
languages: [python]
fix: suggest
fix_reason: "the value is the same on Python 3.0–3.9; from 3.10 it is the right minor version, where the slice was wrong"
stability: preview
ported_from: [ruff/YTT102]
since: 0.5.0
---

The minor version is read from `sys.version_info[1]`, not `sys.version[2]`.

**Why:** `sys.version[2]` is one character: `"1"` on Python 3.10.

```codelaw
match:
  pattern: sys.version[2]
message: "`sys.version[2]` breaks on 3.10; use `sys.version_info[1]`"
fix: 'str(sys.version_info[1])'
```

## Examples

### Python

#### Don't

```python
minor = sys.version[2]
```

#### Do

```python
minor = str(sys.version_info[1])
```

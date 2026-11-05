---
id: py/sys-version0
title: "sys.version_info[0], not sys.version[0]"
languages: [python]
fix: suggest
fix_reason: "the same string on every Python whose major version has one digit"
stability: preview
ported_from: [ruff/YTT301]
since: 0.5.0
---

The major version is read from `sys.version_info[0]`, not `sys.version[0]`.

**Why:** `sys.version[0]` is one character and would be `"1"` on a Python 10.

```codelaw
match:
  pattern: sys.version[0]
message: "Use `sys.version_info[0]`"
fix: 'str(sys.version_info[0])'
```

## Examples

### Python

#### Don't

```python
major = sys.version[0]
```

#### Do

```python
major = str(sys.version_info[0])
```

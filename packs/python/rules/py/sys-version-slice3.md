---
id: py/sys-version-slice3
title: "sys.version_info, not sys.version[:3]"
languages: [python]
fix: suggest
fix_reason: "the same string up to 3.9; from 3.10 it is the right version, where the slice was wrong"
stability: preview
ported_from: [ruff/YTT101]
since: 0.5.0
---

The Python version is read from `sys.version_info`, not by slicing `sys.version[:3]`.

**Why:** `sys.version[:3]` is `"3.1"` on Python 3.10 and later.

```codelaw
match:
  pattern: sys.version[:3]
message: "`sys.version[:3]` breaks on 3.10; use `sys.version_info`"
fix: '"%d.%d" % sys.version_info[:2]'
```

## Examples

### Python

#### Don't

```python
v = sys.version[:3]
```

#### Do

```python
v = "%d.%d" % sys.version_info[:2]
```

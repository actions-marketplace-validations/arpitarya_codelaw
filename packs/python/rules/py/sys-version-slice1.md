---
id: py/sys-version-slice1
title: "sys.version_info[0], not sys.version[:1]"
languages: [python]
fix: suggest
fix_reason: "the same string on every Python whose major version has one digit"
stability: preview
ported_from: [ruff/YTT303]
since: 0.5.0
---

The major version is read from `sys.version_info[0]`, not `sys.version[:1]`.

**Why:** One character breaks once the major version has two digits.

```codelaw
match:
  pattern: sys.version[:1]
message: "Use `sys.version_info[0]`"
fix: 'str(sys.version_info[0])'
```

## Examples

### Python

#### Don't

```python
major = sys.version[:1]
```

#### Do

```python
major = str(sys.version_info[0])
```

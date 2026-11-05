---
id: py/sys-version-info0-eq3
title: "sys.version_info >= (3,), not [0] == 3"
languages: [python]
fix: suggest
fix_reason: "`>=` also holds on a future Python 4, which is what the check means"
stability: preview
ported_from: [ruff/YTT201]
since: 0.5.0
---

`sys.version_info[0] == 3` is written `sys.version_info >= (3,)`.

**Why:** The equality is false on Python 4, which then takes the Python 2 branch.

```codelaw
match:
  kind: comparison_operator
  pattern: sys.version_info[0] == 3
message: "Write `sys.version_info >= (3,)`"
fix: "sys.version_info >= (3,)"
```

## Examples

### Python

#### Don't

```python
PY3 = sys.version_info[0] == 3
```

#### Do

```python
PY3 = sys.version_info >= (3,)
```

---
id: py/sys-version-info1-cmp-int
title: "Compare the version tuple, not [1] with an int"
languages: [python]
fix: suggest
fix_reason: "assumes major version 3, which is what a minor-version test meant"
stability: preview
ported_from: [ruff/YTT203]
since: 0.5.0
---

`sys.version_info[1] >= N` is written `sys.version_info >= (3, N)` (Ruff also reports `<`, `>` and `<=`; those are left out).

**Why:** `sys.version_info[1] >= 7` is false on Python 4.0; compare the whole tuple.

```codelaw
match:
  kind: comparison_operator
  pattern: sys.version_info[1] >= $N
constraints:
  N: {kind: integer}
message: "Compare `sys.version_info` with a tuple"
fix: 'sys.version_info >= (3, $N)'
```

## Examples

### Python

#### Don't

```python
if sys.version_info[1] >= 7:
    pass
```

#### Do

```python
if sys.version_info >= (3, 7):
    pass
```

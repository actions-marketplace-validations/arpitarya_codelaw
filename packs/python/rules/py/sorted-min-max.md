---
id: py/sorted-min-max
title: "min(), not sorted()[0]"
languages: [python]
fix: suggest
fix_reason: "`min` returns the first of equal items, `sorted()[0]` may differ for unstable keys"
stability: preview
ported_from: [ruff/FURB192]
since: 0.5.0
---

`sorted(x)[0]` is written `min(x)` and `sorted(x)[-1]` `max(x)`. The fix covers `[0]`.

**Why:** One pass finds the smallest item; sorting does much more work.

```codelaw
match:
  kind: subscript
  pattern: sorted($X)[0]
message: "Write `min($X)`"
fix: "min($X)"
```

## Examples

### Python

#### Don't

```python
lowest = sorted(scores)[0]
```

#### Do

```python
lowest = min(scores)
```

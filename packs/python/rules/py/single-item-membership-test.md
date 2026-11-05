---
id: py/single-item-membership-test
title: "x == y, not x in (y,)"
languages: [python]
fix: suggest
fix_reason: "`in` also matches by identity, `==` only by equality"
stability: preview
ported_from: [ruff/FURB171]
since: 0.5.0
---

Membership in a one-item literal, `x in (y,)` or `x in [y]`, is written `x == y`.

**Why:** A one-item collection is just a comparison.

```codelaw
match:
  kind: comparison_operator
  any:
    - pattern: $X in ($Y,)
    - pattern: $X in [$Y]
message: "Write `$X == $Y`"
fix: "$X == $Y"
```

## Examples

### Python

#### Don't

```python
if mode in ("fast",):
    pass
```

#### Do

```python
if mode == "fast":
    pass
```

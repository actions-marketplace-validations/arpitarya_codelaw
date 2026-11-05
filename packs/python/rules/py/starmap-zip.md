---
id: py/starmap-zip
title: "map(f, a, b), not starmap(f, zip(a, b))"
languages: [python]
fix: suggest
fix_reason: "`map` stops at the shortest input just like `zip`, but a `zip(strict=True)` is not equivalent"
stability: preview
ported_from: [ruff/RUF058]
since: 0.5.0
---

`itertools.starmap(f, zip(a, b))` is written `map(f, a, b)`.

**Why:** `map` takes several iterables directly; the zip and unpacking are extra steps.

```codelaw
match:
  any:
    - pattern: itertools.starmap($F, zip($$$A))
    - pattern: starmap($F, zip($$$A))
constraints:
  A: {not: {kind: keyword_argument}}
message: "Use `map($F, ...)` with the iterables directly"
fix: "map($F, $$$A)"
```

## Examples

### Python

#### Don't

```python
totals = list(itertools.starmap(add, zip(xs, ys)))
```

#### Do

```python
totals = list(map(add, xs, ys))
```

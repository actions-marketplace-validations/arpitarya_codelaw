---
id: py/zip-instead-of-pairwise
title: "itertools.pairwise, not zip(x, x[1:])"
languages: [python]
fix: suggest
fix_reason: "`pairwise` takes any iterable and yields lazily; review a use that indexed the result"
stability: preview
ported_from: [ruff/RUF007]
since: 0.5.0
---

Successive pairs are taken with `itertools.pairwise(x)`, not `zip(x, x[1:])`.

**Why:** `pairwise` says what is meant and does not copy the sequence.

```codelaw
match:
  pattern: zip($X, $X[1:])
constraints:
  X: {any: [{kind: identifier}, {kind: attribute}]}
message: "Use `itertools.pairwise($X)`"
fix: "itertools.pairwise($X)"
```

## Examples

### Python

#### Don't

```python
import itertools
for a, b in zip(points, points[1:]):
    draw(a, b)
```

#### Do

```python
import itertools
for a, b in itertools.pairwise(points):
    draw(a, b)
```

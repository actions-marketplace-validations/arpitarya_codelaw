---
id: numpy/numpy-deprecated-function
title: "No deprecated NumPy functions"
languages: [python]
fix: auto
fix_reason: "each is a plain alias of its replacement, so renaming it keeps the behaviour"
stability: preview
ported_from: [ruff/NPY003]
since: 0.5.0
---

`np.round_`, `np.product`, `np.cumproduct`, `np.sometrue` and `np.alltrue` are not used.

**Why:** They are deprecated aliases (removed in NumPy 2); `round`, `prod`, `cumprod`, `any` and `all` are the names.

```codelaw
match:
  all:
    - pattern: np.$_F
    - any:
        - has: { field: attribute, regex: '^round_$', pattern: $F0 }
        - has: { field: attribute, regex: '^product$', pattern: $F1 }
        - has: { field: attribute, regex: '^cumproduct$', pattern: $F2 }
        - has: { field: attribute, regex: '^sometrue$', pattern: $F3 }
        - has: { field: attribute, regex: '^alltrue$', pattern: $F4 }
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+numpy\b' } }
message: "`$$MATCH` is deprecated; use its current name"
fix:
  - replace: { target: $F0, with: "round" }
  - replace: { target: $F1, with: "prod" }
  - replace: { target: $F2, with: "cumprod" }
  - replace: { target: $F3, with: "any" }
  - replace: { target: $F4, with: "all" }
```

## How to fix

1. Use `np.round`, `np.prod`, `np.cumprod`, `np.any` or `np.all`.

## Examples

### Python

#### Don't

```python
import numpy as np
t = np.product(xs)
```

#### Do

```python
import numpy as np
t = np.prod(xs)
```

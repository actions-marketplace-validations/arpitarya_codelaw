---
id: numpy/numpy-deprecated-function
title: "No deprecated NumPy functions"
languages: [python]
fix: manual
fix_reason: "each has its own replacement name"
stability: preview
ported_from: [ruff/NPY003]
since: 0.5.0
---

`np.round_`, `np.product`, `np.cumproduct`, `np.sometrue` and `np.alltrue` are not used.

**Why:** They are deprecated aliases (removed in NumPy 2); `round`, `prod`, `cumprod`, `any` and `all` are the names.

```codelaw
match:
  pattern: np.$F
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+numpy\b' } }
constraints:
  F: { regex: '^(round_|product|cumproduct|sometrue|alltrue)$' }
message: "`np.$F` is deprecated; use its current name"
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

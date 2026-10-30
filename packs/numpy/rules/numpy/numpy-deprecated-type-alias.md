---
id: numpy/numpy-deprecated-type-alias
title: "Builtins, not NumPy's removed type aliases"
languages: [python]
fix: suggest
fix_reason: "the builtin is what the alias meant; review a use that relied on NumPy's own type"
stability: preview
ported_from: [ruff/NPY001]
since: 0.5.0
---

`np.bool`, `np.int`, `np.float`, `np.complex`, `np.object` and `np.str` are written as the builtins they were aliases of.

**Why:** The aliases were deprecated in NumPy 1.20 and removed in 1.24; code using them fails.

```codelaw
match:
  pattern: np.$T
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+numpy\b' } }
constraints:
  T: { regex: '^(bool|int|float|complex|object|str)$' }
message: "`np.$T` was removed; use the builtin `$T`"
fix: "$T"
```

## Examples

### Python

#### Don't

```python
import numpy as np
x = np.zeros(3, dtype=np.int)
```

#### Do

```python
import numpy as np
x = np.zeros(3, dtype=int)
```

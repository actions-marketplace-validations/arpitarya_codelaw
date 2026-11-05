---
id: numpy/numpy2-deprecation
title: "No names NumPy 2 removed"
languages: [python]
fix: manual
fix_reason: "each removed name has its own replacement, given in the NumPy 2 migration guide"
fix_pending: "auto-subset"
stability: preview
ported_from: [ruff/NPY201]
since: 0.5.0
---

Names that NumPy 2.0 removed from the main namespace are not used.

**Why:** Code that uses them fails on import or at the call under NumPy 2.

```codelaw
match:
  pattern: np.$F
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+numpy\b' } }
constraints:
  F: { regex: '^(add_docstring|add_newdoc|add_newdoc_ufunc|alltrue|asfarray|byte_bounds|cast|cfloat|clongfloat|compat|complex_|cumproduct|deprecate|deprecate_with_doc|disp|fastCopyAndTranspose|find_common_type|float_|geterrobj|Inf|Infinity|infty|in1d|issctype|issubclass_|issubsctype|longcomplex|longfloat|lookfor|mat|maximum_sctype|msort|NaN|NINF|NZERO|obj2sctype|PINF|product|PZERO|recfromcsv|recfromtxt|round_|row_stack|safe_eval|sctype2char|set_numeric_ops|set_string_function|seterrobj|singlecomplex|sometrue|source|string_|tracemalloc_domain|trapz|unicode_|who)$' }
message: "`np.$F` was removed in NumPy 2"
```

## How to fix

1. Use the replacement the NumPy 2 migration guide names (for example `np.inf` for `np.Inf`, `np.float64` for `np.float_`, `np.isin` for `np.in1d`).

## Examples

### Python

#### Don't

```python
import numpy as np
x = np.NaN
```

#### Do

```python
import numpy as np
x = np.nan
```

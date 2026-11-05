---
id: numpy/numpy2-deprecation
title: "No names NumPy 2 removed"
languages: [python]
fix: auto
fix_reason: "auto for the names that are plain aliases of a name that also exists in NumPy 1 (`np.Inf`, `np.float_`, `np.product`, `np.row_stack` and the like); every other removed name needs the migration guide's rewrite"
stability: preview
ported_from: [ruff/NPY201]
since: 0.5.0
---

Names that NumPy 2.0 removed from the main namespace are not used.

**Why:** Code that uses them fails on import or at the call under NumPy 2.

```codelaw
match:
  all:
    - pattern: np.$_F
    - any:
        - has: { field: attribute, regex: '^(alltrue)$', pattern: $G0 }
        - has: { field: attribute, regex: '^(product)$', pattern: $G1 }
        - has: { field: attribute, regex: '^(cumproduct)$', pattern: $G2 }
        - has: { field: attribute, regex: '^(round_)$', pattern: $G3 }
        - has: { field: attribute, regex: '^(sometrue)$', pattern: $G4 }
        - has: { field: attribute, regex: '^(row_stack)$', pattern: $G5 }
        - has: { field: attribute, regex: '^(Inf|Infinity|infty|PINF)$', pattern: $G6 }
        - has: { field: attribute, regex: '^(NaN)$', pattern: $G7 }
        - has: { field: attribute, regex: '^(cfloat|complex_)$', pattern: $G8 }
        - has: { field: attribute, regex: '^(clongfloat|longcomplex)$', pattern: $G9 }
        - has: { field: attribute, regex: '^(longfloat)$', pattern: $G10 }
        - has: { field: attribute, regex: '^(float_)$', pattern: $G11 }
        - has: { field: attribute, regex: '^(singlecomplex)$', pattern: $G12 }
        - has: { field: attribute, regex: '^(string_)$', pattern: $G13 }
        - has: { field: attribute, regex: '^(unicode_)$', pattern: $G14 }
        - has: { field: attribute, regex: '^(mat)$', pattern: $G15 }
        - has: { field: attribute, regex: '^(add_docstring|add_newdoc|add_newdoc_ufunc|alltrue|asfarray|byte_bounds|cast|cfloat|clongfloat|compat|complex_|cumproduct|deprecate|deprecate_with_doc|disp|fastCopyAndTranspose|find_common_type|float_|geterrobj|Inf|Infinity|infty|in1d|issctype|issubclass_|issubsctype|longcomplex|longfloat|lookfor|mat|maximum_sctype|msort|NaN|NINF|NZERO|obj2sctype|PINF|product|PZERO|recfromcsv|recfromtxt|round_|row_stack|safe_eval|sctype2char|set_numeric_ops|set_string_function|seterrobj|singlecomplex|sometrue|source|string_|tracemalloc_domain|trapz|unicode_|who)$' }
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+numpy\b' } }
message: "`$$MATCH` was removed in NumPy 2"
auto_when: { regex: '^np\.(alltrue|product|cumproduct|round_|sometrue|row_stack|Inf|Infinity|infty|PINF|NaN|cfloat|complex_|clongfloat|longcomplex|longfloat|float_|singlecomplex|string_|unicode_|mat)$' }
fix:
  - replace: { target: $G0, with: "all" }
  - replace: { target: $G1, with: "prod" }
  - replace: { target: $G2, with: "cumprod" }
  - replace: { target: $G3, with: "round" }
  - replace: { target: $G4, with: "any" }
  - replace: { target: $G5, with: "vstack" }
  - replace: { target: $G6, with: "inf" }
  - replace: { target: $G7, with: "nan" }
  - replace: { target: $G8, with: "complex128" }
  - replace: { target: $G9, with: "clongdouble" }
  - replace: { target: $G10, with: "longdouble" }
  - replace: { target: $G11, with: "float64" }
  - replace: { target: $G12, with: "complex64" }
  - replace: { target: $G13, with: "bytes_" }
  - replace: { target: $G14, with: "str_" }
  - replace: { target: $G15, with: "asmatrix" }
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

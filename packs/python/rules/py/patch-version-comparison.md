---
id: py/patch-version-comparison
title: "A version check in a stub compares major and minor only"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "which minor version the declaration belongs to is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PYI004]
since: 0.5.0
---

A stub compares `sys.version_info` with a tuple of at most two numbers: `(3, 10)`, not `(3, 10, 2)`.

**Why:** Type checkers ignore the patch level, so a three-part check does not mean what it says.

```codelaw
match:
  kind: comparison_operator
  all:
    - has: {nthChild: 1, regex: '^sys\.version_info'}
    - has: {nthChild: 2, kind: tuple, has: {nthChild: 3}}
message: "Compare only the major and minor version"
```

## How to fix

1. Compare with `(major, minor)` only, picking the first minor version the declaration holds for.

## Examples

### Python

#### Don't

```python path=api.pyi
if sys.version_info >= (3, 10, 2):
    def f() -> None: ...
```

#### Do

```python path=api.pyi
if sys.version_info >= (3, 11):
    def f() -> None: ...
```

---
id: py/bad-version-info-comparison
title: "A version check in a stub uses < or >="
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "the bound and the branches are rewritten around the boundary version"
fix_exception: restructure
stability: preview
ported_from: [ruff/PYI006]
since: 0.5.0
---

A stub compares `sys.version_info` only with `<` or `>=`.

**Why:** With `<` and `>=` the two branches split the versions cleanly; `<=`, `>`, `==` and `!=` leave the boundary version on the wrong side or out.

```codelaw
match:
  kind: comparison_operator
  has: {nthChild: 1, regex: '^sys\.version_info'}
  not: {regex: '^sys\.version_info\s*(<|>=)\s*[^=<>]'}
message: "Compare `sys.version_info` with `<` or `>=`"
```

## How to fix

1. Rewrite the check as `sys.version_info >= (x, y)` or `< (x, y)`, with the boundary version on the side it belongs to.

## Examples

### Python

#### Don't

```python path=api.pyi
if sys.version_info > (3, 9):
    def f() -> None: ...
```

#### Do

```python path=api.pyi
if sys.version_info >= (3, 10):
    def f() -> None: ...
```

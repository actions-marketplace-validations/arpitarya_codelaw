---
id: py/os-sep-split
title: "Path.parts, not split(os.sep)"
languages: [python]
fix: manual
fix_reason: "the code moves to `pathlib`, whose objects and methods differ from the strings it used"
fix_exception: restructure
stability: preview
ported_from: [ruff/PTH206]
since: 0.5.0
---

`p.split(os.sep)` is written `Path(p).parts`.

**Why:** `pathlib` keeps paths as objects with their own operations, one API instead of several string functions.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, regex: '\.split$'}
    - has: {field: arguments, has: {nthChild: 1, regex: '^os\.sep$'}}
message: "Use `Path(p).parts`"
```

## How to fix

1. Rewrite it with `Path(p).parts`, importing `pathlib.Path`.

## Examples

### Python

#### Don't

```python
parts = path.split(os.sep)
```

#### Do

```python
parts = Path(path).parts
```

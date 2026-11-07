---
id: py/unassigned-special-variable-in-stub
title: "A stub gives __all__ its value"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "the names it lists are the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PYI035]
since: 0.5.0
---

In a stub, `__all__`, `__match_args__` and `__slots__` are assigned their value, not only annotated.

**Why:** Type checkers read the value of these names; an annotation alone tells them nothing.

```codelaw
match:
  kind: assignment
  not: {regex: '='}
  all:
    - has: {field: left, regex: '^__(all|match_args|slots)__$'}
    - has: {field: type, kind: type}
message: "Assign the value"
```

## How to fix

1. Assign the value: `__all__ = ["name", ...]`.

## Examples

### Python

#### Don't

```python path=api.pyi
__all__: list[str]
```

#### Do

```python path=api.pyi
__all__ = ["connect"]
```

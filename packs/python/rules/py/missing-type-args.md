---
id: py/missing-type-args
title: "*args is annotated"
languages: [python]
fix: manual
fix_reason: "the parameter's type is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/ANN002]
since: 0.5.0
---

A function's `*args` has a type annotation: `*args: int`.

**Why:** An annotated parameter lets a type checker check every call, and tells the reader what to pass.

```codelaw
match:
  kind: list_splat_pattern
  inside: {kind: parameters, inside: {kind: function_definition}}
  not:
    inside:
      kind: function_definition
      stopBy: end
      inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(typing\.)?overload$'}}
message: "Annotate the parameter"
```

## How to fix

1. Add the parameter's type: `def load(path: str)`.

## Examples

### Python

#### Don't

```python
def total(*nums):
    return sum(nums)
```

#### Do

```python
def total(*nums: int):
    return sum(nums)
```

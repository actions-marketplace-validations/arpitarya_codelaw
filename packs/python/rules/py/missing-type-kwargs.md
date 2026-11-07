---
id: py/missing-type-kwargs
title: "**kwargs is annotated"
languages: [python]
fix: manual
fix_reason: "the parameter's type is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/ANN003]
since: 0.5.0
---

A function's `**kwargs` has a type annotation: `**options: str`.

**Why:** An annotated parameter lets a type checker check every call, and tells the reader what to pass.

```codelaw
match:
  kind: dictionary_splat_pattern
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
def configure(**options):
    pass
```

#### Do

```python
def configure(**options: str):
    pass
```

---
id: py/missing-type-function-argument
title: "A function's parameters are annotated"
languages: [python]
fix: manual
fix_reason: "the parameter's type is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/ANN001]
since: 0.5.0
---

Each parameter of a function has a type annotation, other than a method's first, `self` or `cls`, and `*args` and `**kwargs` (those are `py/missing-type-args` and `py/missing-type-kwargs`).

**Why:** An annotated parameter lets a type checker check every call, and tells the reader what to pass.

```codelaw
match:
  any:
    - kind: identifier
      inside: {kind: parameters, inside: {kind: function_definition}}
      not:
        all:
          - nthChild: 1
          - inside: {kind: parameters, inside: {kind: function_definition, any: [{inside: {kind: block, inside: {kind: class_definition}}}, {inside: {kind: decorated_definition, inside: {kind: block, inside: {kind: class_definition}}, not: {has: {kind: decorator, regex: '^@staticmethod$'}}}}]}}
    - kind: default_parameter
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
def load(path, retries=3):
    pass
```

#### Do

```python
def load(path: str, retries: int = 3):
    pass
```

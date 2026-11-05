---
id: py/post-init-default
title: "No defaults on __post_init__ parameters"
languages: [python]
fix: manual
fix_reason: "the default belongs on the `InitVar` field, which moves it to the class body"
fix_exception: restructure
stability: preview
ported_from: [ruff/RUF033]
since: 0.5.0
---

A dataclass's `__post_init__` gives its parameters no default value.

**Why:** `__post_init__` is called with every `InitVar`, so the default is never used; the default belongs on the field.

```codelaw
match:
  any: [{kind: default_parameter}, {kind: typed_default_parameter}]
  inside:
    kind: parameters
    inside:
      kind: function_definition
      has: {field: name, regex: '^__post_init__$'}
      inside: {kind: block, inside: {kind: class_definition, inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(dataclasses\.)?dataclass\b'}}}}
message: "This default is never used; set it on the `InitVar` field"
```

## How to fix

1. Remove the default here and give the matching `InitVar` field the default: `x: InitVar[int] = 1`.

## Examples

### Python

#### Don't

```python
@dataclass
class A:
    x: InitVar[int]
    def __post_init__(self, x=1):
        pass
```

#### Do

```python
@dataclass
class A:
    x: InitVar[int] = 1
    def __post_init__(self, x):
        pass
```

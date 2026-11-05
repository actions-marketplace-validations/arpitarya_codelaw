---
id: py/bad-staticmethod-argument
title: "A staticmethod takes no self"
languages: [python]
fix: manual
fix_reason: "whether it should be an instance method or the parameter renamed is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLW0211]
since: 0.5.0
---

A `@staticmethod`'s first parameter is not named `self` or `cls`.

**Why:** A static method receives no instance; the name says it does.

```codelaw
match:
  kind: identifier
  regex: '^(self|cls)$'
  inside:
    any: [{kind: parameters}, {kind: typed_parameter, inside: {kind: parameters}}]
    stopBy: end
    inside: {kind: function_definition, inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@staticmethod$'}}}
  not: {inside: {kind: default_parameter}}
message: "A static method gets no instance; drop `@staticmethod` or rename the parameter"
```

## How to fix

1. Remove `@staticmethod` if the method needs the instance, or rename the parameter.

## Examples

### Python

#### Don't

```python
class A:
    @staticmethod
    def make(self):
        pass
```

#### Do

```python
class A:
    @staticmethod
    def make(value):
        pass
```

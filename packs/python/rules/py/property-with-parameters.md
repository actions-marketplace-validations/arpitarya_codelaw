---
id: py/property-with-parameters
title: "A property takes only self"
languages: [python]
fix: manual
fix_reason: "the property becomes a method, and every read of it a call"
fix_exception: restructure
stability: preview
ported_from: [ruff/PLR0206]
since: 0.5.0
---

A method marked `@property` takes no parameter besides `self`.

**Why:** A property is read as `obj.name`; there is no way to pass it an argument.

```codelaw
match:
  kind: function_definition
  has: {field: parameters, has: {nthChild: 2}}
  inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(functools\.cached_)?property$'}}
message: "A property takes only `self`"
```

## How to fix

1. Make it a plain method and call it, or move the parameter onto the object.

## Examples

### Python

#### Don't

```python
class A:
    @property
    def size(self, unit):
        return self.n
```

#### Do

```python
class A:
    def size(self, unit):
        return self.n
```

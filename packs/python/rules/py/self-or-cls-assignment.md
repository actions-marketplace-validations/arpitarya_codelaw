---
id: py/self-or-cls-assignment
title: "No assignment to self or cls"
languages: [python]
fix: manual
fix_reason: "the new name for the value is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLW0642]
since: 0.5.0
---

A method does not rebind its `self` or `cls` parameter.

**Why:** Rebinding changes only the local name; it reads as if it changed the instance.

```codelaw
match:
  kind: assignment
  has: {field: left, kind: identifier, regex: '^(self|cls)$'}
  inside: {kind: function_definition, stopBy: end, has: {field: parameters, regex: '^\(\s*(self|cls)\b'}, inside: {kind: block, inside: {kind: class_definition}}}
message: "Rebinding `self` or `cls` changes only the local name"
```

## How to fix

1. Assign the value to a new name.

## Examples

### Python

#### Don't

```python
class A:
    def copy(self):
        self = A()
        return self
```

#### Do

```python
class A:
    def copy(self):
        other = A()
        return other
```

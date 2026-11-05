---
id: py/no-method-argument
title: "A method takes self"
languages: [python]
fix: suggest
fix_reason: "`self` is added, so the method can be called on an instance; a method meant to be static should say `@staticmethod`"
stability: preview
ported_from: [pylint/no-method-argument]
since: 0.5.0
---

A plain method (no `@staticmethod`) has at least one parameter.

**Why:** Calling it on an instance passes the instance, and the call raises TypeError.

```codelaw
match:
  kind: parameters
  regex: '^\(\s*\)$'
  inside:
    kind: function_definition
    not: {inside: {kind: decorated_definition}}
    inside: {kind: block, inside: {kind: class_definition}}
message: "Add `self`"
fix: "(self)"
```

## Examples

### Python

#### Don't

```python
class A:
    def ping():
        return 1
```

#### Do

```python
class A:
    def ping(self):
        return 1
```

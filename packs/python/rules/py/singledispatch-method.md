---
id: py/singledispatch-method
title: "singledispatchmethod on methods"
languages: [python]
fix: suggest
fix_reason: "the method then dispatches on its first argument after `self`, which is what was meant"
stability: preview
ported_from: [ruff/PLE1519]
since: 0.5.0
---

A method is decorated `@functools.singledispatchmethod`, not `@functools.singledispatch`.

**Why:** `singledispatch` dispatches on the first argument, which for a method is `self`.

```codelaw
match:
  kind: decorator
  regex: '^@functools\.singledispatch$'
  inside: {kind: decorated_definition, inside: {kind: block, inside: {kind: class_definition}}}
message: "On a method, use `singledispatchmethod`"
fix: "@functools.singledispatchmethod"
```

## Examples

### Python

#### Don't

```python
class A:
    @functools.singledispatch
    def handle(self, x):
        pass
```

#### Do

```python
class A:
    @functools.singledispatchmethod
    def handle(self, x):
        pass
```

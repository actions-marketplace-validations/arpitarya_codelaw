---
id: py/singledispatchmethod-function
title: "singledispatch on plain functions"
languages: [python]
fix: suggest
fix_reason: "the function then dispatches on its first argument, which is what was meant"
stability: preview
ported_from: [ruff/PLE1520]
since: 0.5.0
---

A module-level function is decorated `@functools.singledispatch`, not `@functools.singledispatchmethod`.

**Why:** `singledispatchmethod` expects a method; on a function it dispatches on the second argument.

```codelaw
match:
  kind: decorator
  regex: '^@functools\.singledispatchmethod$'
  inside: {kind: decorated_definition, inside: {kind: module}}
message: "On a function, use `singledispatch`"
fix: "@functools.singledispatch"
```

## Examples

### Python

#### Don't

```python
@functools.singledispatchmethod
def handle(x):
    pass
```

#### Do

```python
@functools.singledispatch
def handle(x):
    pass
```

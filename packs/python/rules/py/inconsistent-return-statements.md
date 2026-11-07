---
id: py/inconsistent-return-statements
title: "Either every return gives a value or none does"
languages: [python]
fix: manual
fix_reason: "what the bare paths should return is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/inconsistent-return-statements]
since: 0.5.0
---

A function does not mix `return value` with a bare `return`.

**Why:** A bare `return` returns `None`; mixed with value returns it reads like a forgotten value.

```codelaw
match:
  kind: function_definition
  all:
    - has: {field: body, has: {kind: return_statement, regex: '^return\s*\S', stopBy: {any: [{kind: function_definition}, {kind: lambda}, {kind: class_definition}]}}}
    - has: {field: body, has: {kind: return_statement, regex: '^return\s*$', stopBy: {any: [{kind: function_definition}, {kind: lambda}, {kind: class_definition}]}}}
message: "Return a value on every path, or on none"
```

## How to fix

1. Write `return None` (or the value meant) where the function returns without one.

## Examples

### Python

#### Don't

```python
def find(xs, k):
    for x in xs:
        if x == k:
            return x
    return
```

#### Do

```python
def find(xs, k):
    for x in xs:
        if x == k:
            return x
    return None
```

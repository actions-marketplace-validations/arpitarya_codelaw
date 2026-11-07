---
id: py/unnecessary-direct-lambda-call
title: "No lambda called where it is written"
languages: [python]
fix: manual
fix_reason: "each parameter is replaced by its argument in the body, which may need parentheses"
fix_exception: restructure
stability: preview
ported_from: [ruff/PLC3002]
since: 0.5.0
---

A lambda is not called as soon as it is written: `(lambda x: x * 2)(3)` is `3 * 2`.

**Why:** The lambda only renames its arguments; the expression says the same thing directly.

```codelaw
match:
  kind: call
  has: {field: function, kind: parenthesized_expression, has: {kind: lambda}}
message: "Write the lambda's body directly"
```

## How to fix

1. Replace the call with the lambda's body, each parameter replaced by its argument.

## Examples

### Python

#### Don't

```python
y = (lambda x: x * 2)(3)
```

#### Do

```python
y = 3 * 2
```

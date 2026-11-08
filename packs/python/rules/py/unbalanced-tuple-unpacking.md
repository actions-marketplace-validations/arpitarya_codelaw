---
id: py/unbalanced-tuple-unpacking
title: "An unpacking has as many targets as the literal has values"
languages: [python]
fix: manual
fix_reason: "whether a target or a value is missing is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/unbalanced-tuple-unpacking]
since: 0.5.0
---

Unpacking a tuple, list or set literal into a different number of targets — `a, b = 1, 2, 3` — is reported. A starred target, a starred value and an assignment in a comprehension are not read; Pylint also infers the length of a function's return value, which this rule does not.

**Why:** The assignment always raises `ValueError`.

```codelaw
native: py/unbalanced-tuple-unpacking
```

## How to fix

1. Give the unpacking as many targets as values, or use a starred target for the rest.

## Examples

### Python

#### Don't

```python
x, y = 1, 2, 3
```

#### Do

```python
x, y, z = 1, 2, 3
```

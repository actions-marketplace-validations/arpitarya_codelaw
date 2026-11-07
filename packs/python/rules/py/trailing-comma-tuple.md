---
id: py/trailing-comma-tuple
title: "No tuple made by a trailing comma"
languages: [python]
fix: manual
fix_reason: "whether a tuple was meant is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/trailing-comma-tuple]
since: 0.5.0
---

An assignment's value is not a one-item tuple made by a trailing comma, `x = 1,`.

**Why:** The comma is easy to miss; `x` is a tuple where the reader expects a number.

```codelaw
match:
  kind: expression_list
  not: {has: {nthChild: 2}}
  inside: {kind: assignment, field: right}
message: "A trailing comma makes this a tuple"
```

## How to fix

1. Remove the comma, or write the tuple in parentheses: `(10,)`.

## Examples

### Python

#### Don't

```python
limit = 10,
```

#### Do

```python
limit = 10
```

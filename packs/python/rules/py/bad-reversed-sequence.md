---
id: py/bad-reversed-sequence
title: "reversed() is given a sequence"
languages: [python]
fix: manual
fix_reason: "which order was meant is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/bad-reversed-sequence]
since: 0.5.0
---

`reversed()` is not called on a set or dict literal, a set comprehension or a generator.

**Why:** These have no reverse order (a dict's `reversed` needs 3.8 and reads only keys), and `reversed` raises `TypeError` on most of them.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, regex: '^reversed$'}
    - has: {field: arguments, has: {nthChild: 1, any: [{kind: set}, {kind: set_comprehension}, {kind: generator_expression}]}}
message: "`reversed` needs a sequence"
```

## How to fix

1. Pass a list or tuple (`sorted(...)` for a set), or iterate without reversing.

## Examples

### Python

#### Don't

```python
for x in reversed({1, 2, 3}):
    pass
```

#### Do

```python
for x in reversed([1, 2, 3]):
    pass
```

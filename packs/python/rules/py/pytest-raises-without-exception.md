---
id: py/pytest-raises-without-exception
title: "pytest.raises names an exception"
languages: [python]
fix: manual
fix_reason: "which exception the test expects is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PT010]
since: 0.5.0
---

`pytest.raises()` is given the exception it expects.

**Why:** Without one, the call fails with a `TypeError` instead of testing anything.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, regex: '^pytest\.raises$'}
    - has: {field: arguments, regex: '^\(\s*\)$'}
message: "Name the exception `pytest.raises` expects"
```

## How to fix

1. Pass the exception class the code under test raises: `pytest.raises(ValueError)`.

## Examples

### Python

#### Don't

```python
with pytest.raises():
    parse("x")
```

#### Do

```python
with pytest.raises(ValueError):
    parse("x")
```

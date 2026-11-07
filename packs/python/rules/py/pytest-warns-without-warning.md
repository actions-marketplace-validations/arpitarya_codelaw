---
id: py/pytest-warns-without-warning
title: "pytest.warns names a warning"
languages: [python]
fix: manual
fix_reason: "which warning the test expects is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PT029]
since: 0.5.0
---

`pytest.warns()` is given the warning class it expects.

**Why:** Without one the call fails, or on older pytest accepts any warning.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, regex: '^pytest\.warns$'}
    - has: {field: arguments, regex: '^\(\s*\)$'}
message: "Name the warning `pytest.warns` expects"
```

## How to fix

1. Pass the warning class the code under test emits: `pytest.warns(DeprecationWarning)`.

## Examples

### Python

#### Don't

```python
with pytest.warns():
    old_api()
```

#### Do

```python
with pytest.warns(DeprecationWarning):
    old_api()
```

---
id: py/legacy-form-pytest-raises
title: "pytest.raises as a context manager"
languages: [python]
fix: manual
fix_reason: "the call moves into a `with` block"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [ruff/RUF061]
since: 0.5.0
---

`pytest.raises`, `pytest.warns` and `pytest.deprecated_call` are used as context managers, not called with the function and its arguments.

**Why:** The `with` form shows the code that raises and lets the test inspect the exception.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, regex: '^pytest\.(raises|warns|deprecated_call)$'}
    - has: {field: arguments, has: {nthChild: 2, not: {kind: keyword_argument}}}
    - not: {inside: {kind: with_item, stopBy: end}}
message: "Use `with pytest.raises(...):` around the call"
```

## How to fix

1. Write `with pytest.raises(Error):` and call the function inside the block.

## Examples

### Python

#### Don't

```python
pytest.raises(ValueError, int, "x")
```

#### Do

```python
with pytest.raises(ValueError):
    int("x")
```

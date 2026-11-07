---
id: py/pytest-assert-in-except
title: "No assert on the exception inside except"
languages: [python]
fix: manual
fix_reason: "the test is rewritten around `pytest.raises`, which holds the exception"
fix_exception: restructure
stability: preview
ported_from: [ruff/PT017]
since: 0.5.0
---

An `assert` inside an `except` block does not check the caught exception: use `pytest.raises` and assert on `excinfo` after the block.

**Why:** If nothing is raised, the `except` block never runs and the test passes.

```codelaw
match:
  kind: assert_statement
  has: {kind: identifier, pattern: $N, stopBy: end}
  inside:
    kind: except_clause
    stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
    has: {kind: as_pattern, has: {kind: as_pattern_target, has: {kind: identifier, pattern: $N}}}
message: "Use `pytest.raises` and assert on the exception after it"
```

## How to fix

1. Wrap the call in `with pytest.raises(Error) as excinfo:` and assert on `excinfo.value` after the block.

## Examples

### Python

#### Don't

```python
def test_parse():
    try:
        parse("")
    except ValueError as e:
        assert "empty" in str(e)
```

#### Do

```python
def test_parse():
    with pytest.raises(ValueError) as excinfo:
        parse("")
    assert "empty" in str(excinfo.value)
```

---
id: py/pytest-assert-always-false
title: "pytest.fail, not assert False"
languages: [python]
fix: manual
fix_reason: "the message that says why the test fails is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/PT015]
since: 0.5.0
---

A test does not fail with an assertion that is always false — `assert False`, `assert 0`, `assert None`, `assert ""`, an empty list, tuple or dict — but with `pytest.fail("why")`.

**Why:** An always-false assert reads like a check, and Python drops it under `-O`.

```codelaw
match:
  kind: assert_statement
  has: {nthChild: 1, any: [{kind: 'false'}, {kind: none}, {kind: integer, regex: '^0+$'}, {kind: string, regex: '^[rbuRBU]*(""|'')$'}, {kind: list, not: {has: {nthChild: 1}}}, {kind: tuple, not: {has: {nthChild: 1}}}, {kind: dictionary, not: {has: {nthChild: 1}}}]}
message: "Fail with `pytest.fail(...)`"
```

## How to fix

1. Replace the assert with `pytest.fail("<why the test fails>")`.

## Examples

### Python

#### Don't

```python
def test_x():
    assert False
```

#### Do

```python
def test_x():
    pytest.fail("the parser accepted an empty file")
```

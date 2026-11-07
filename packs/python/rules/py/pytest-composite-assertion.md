---
id: py/pytest-composite-assertion
title: "One condition per assert"
languages: [python]
fix: manual
fix_reason: "the assert is split into one per condition, each on its own line at the assert's indentation"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [ruff/PT018]
since: 0.5.0
---

An `assert a and b` is written as two asserts, `assert a` and `assert b`, and `assert not (a or b)` as `assert not a` and `assert not b`.

**Why:** A failing composite assert does not say which part failed.

```codelaw
match:
  kind: assert_statement
  has: {nthChild: 1, any: [{kind: boolean_operator, regex: '\band\b'}, {kind: not_operator, has: {kind: parenthesized_expression, has: {kind: boolean_operator, regex: '\bor\b'}}}]}
message: "Assert one condition at a time"
```

## How to fix

1. Split the assert into one assert per condition.

## Examples

### Python

#### Don't

```python
def test_user():
    assert user.active and user.admin
```

#### Do

```python
def test_user():
    assert user.active
    assert user.admin
```

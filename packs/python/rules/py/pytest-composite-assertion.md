---
id: py/pytest-composite-assertion
title: "One condition per assert"
languages: [python]
fix: suggest
fix_reason: "each condition becomes its own assert, so a failure names the one that failed; the second is now asserted only if the first held, as before, but an assert with a message is split by hand"
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
  any:
    - all: [{ pattern: assert $A and $B }, { pattern: $AND }]
    - all: [{ pattern: "assert not ($A or $B)" }, { pattern: $NOR }]
    - regex: ''
message: "Assert one condition at a time"
fix:
  - replace: { target: $AND, with: "assert $A" }
  - insert-after: { target: $AND, text: "assert $B", line: true }
  - replace: { target: $NOR, with: "assert not $A" }
  - insert-after: { target: $NOR, text: "assert not $B", line: true }
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

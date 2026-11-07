---
id: py/pytest-fail-without-message
title: "pytest.fail has a message"
languages: [python]
fix: manual
fix_reason: "the message that explains the failure is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/PT016]
since: 0.5.0
---

`pytest.fail()` is given a message saying why: not no argument, and not an empty string.

**Why:** A failure without a reason tells the reader of the test report nothing.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, regex: '^pytest\.fail$'}
    - has:
        field: arguments
        any:
          - regex: '^\(\s*\)$'
          - has: {nthChild: 1, kind: string, regex: '^[rbuRBU]*(""|'''')$'}
          - has: {kind: keyword_argument, all: [{has: {field: name, regex: '^(msg|reason)$'}}, {has: {field: value, regex: '^[rbuRBU]*(""|'''')$'}}]}
message: "Say why the test fails"
```

## How to fix

1. Pass a message that says what went wrong: `pytest.fail("the parser accepted an empty file")`.

## Examples

### Python

#### Don't

```python
def test_parse():
    pytest.fail()
```

#### Do

```python
def test_parse():
    pytest.fail("the parser accepted an empty file")
```

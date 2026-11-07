---
id: py/redundant-unittest-assert
title: "No unittest assert on a constant"
languages: [python]
fix: manual
fix_reason: "the value the test should check is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/redundant-unittest-assert]
since: 0.5.0
---

`self.assertTrue`, `assertFalse`, `assertIsNone` and `assertIsNotNone` are not given a literal: their result is fixed.

**Why:** The assertion tests nothing, and reads as if it did.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, regex: '^self\.(assertTrue|assertFalse|assertIsNone|assertIsNotNone)$'}
    - has: {field: arguments, has: {nthChild: 1, any: [{kind: string}, {kind: integer}, {kind: float}, {kind: 'true'}, {kind: 'false'}, {kind: none}]}}
message: "This assertion's result is fixed"
```

## How to fix

1. Assert on the value under test, or use `self.fail(...)` for a deliberate failure.

## Examples

### Python

#### Don't

```python
class T(unittest.TestCase):
    def test_x(self):
        self.assertTrue("ok")
```

#### Do

```python
class T(unittest.TestCase):
    def test_x(self):
        self.assertTrue(result.ok)
```

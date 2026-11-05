---
id: py/pytest-unittest-assertion
title: "assert, not self.assertEqual"
languages: [python]
fix: suggest
fix_reason: "pytest's assertion rewriting replaces unittest's message; the test class may still need `TestCase` for other reasons"
stability: preview
ported_from: [ruff/PT009]
since: 0.5.0
---

In a pytest suite, `self.assertEqual(a, b)` is written `assert a == b`. (Ruff rewrites every `assert*` method; the fix covers `assertEqual`, the others are reported by `py/pytest-unittest-raises-assertion` and future rules.)

**Why:** Plain `assert` gets pytest's detailed failure output.

```codelaw
match:
  kind: expression_statement
  has: {kind: call, pattern: 'self.assertEqual($A, $B)'}
message: "Write `assert $A == $B`"
fix: "assert $A == $B"
```

## Examples

### Python

#### Don't

```python
class TestMath(unittest.TestCase):
    def test_add(self):
        self.assertEqual(add(1, 2), 3)
```

#### Do

```python
class TestMath(unittest.TestCase):
    def test_add(self):
        assert add(1, 2) == 3
```

---
id: py/pytest-unittest-raises-assertion
title: "pytest.raises, not self.assertRaises"
languages: [python]
fix: suggest
fix_reason: "the context manager's result is pytest's `ExceptionInfo`, not unittest's context"
stability: preview
ported_from: [ruff/PT027]
since: 0.5.0
---

`with self.assertRaises(E):` is written `with pytest.raises(E):`.

**Why:** One idiom across a pytest suite.

```codelaw
match:
  kind: call
  pattern: self.assertRaises($E)
  inside: {kind: with_item}
message: "Use `pytest.raises($E)`"
fix: "pytest.raises($E)"
```

## Examples

### Python

#### Don't

```python
class T(unittest.TestCase):
    def test(self):
        with self.assertRaises(ValueError):
            int("x")
```

#### Do

```python
class T(unittest.TestCase):
    def test(self):
        with pytest.raises(ValueError):
            int("x")
```

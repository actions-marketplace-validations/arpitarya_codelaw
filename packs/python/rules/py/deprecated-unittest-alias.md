---
id: py/deprecated-unittest-alias
title: "No deprecated unittest assert aliases"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP005]
since: 0.5.0
---

`assertEquals`, `failUnless` and the other deprecated `unittest` aliases are written by their current names (`assertEqual`, `assertTrue`).

**Why:** The aliases were removed in Python 3.12.

```codelaw
match:
  kind: attribute
  pattern: $O.$M
  inside: {kind: call, field: function}
constraints:
  O: {regex: '^self$'}
  M: {regex: '^assertEquals$'}
message: "`assertEquals` was removed; use `assertEqual`"
fix: "$O.assertEqual"
```

## Examples

### Python

#### Don't

```python
class T(unittest.TestCase):
    def test(self):
        self.assertEquals(1, 1)
```

#### Do

```python
class T(unittest.TestCase):
    def test(self):
        self.assertEqual(1, 1)
```

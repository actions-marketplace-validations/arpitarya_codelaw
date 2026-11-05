---
id: py/pytest-incorrect-mark-parentheses-style
title: "@pytest.mark.x, not @pytest.mark.x()"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PT023]
since: 0.5.0
---

A pytest mark with no arguments is written without empty parentheses.

**Why:** One style across the test suite.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: ^pytest\.mark\.\w+$, pattern: $M}
  - has: {field: arguments, regex: ^\(\s*\)$}
  - inside: {kind: decorator}
message: Drop the empty parentheses
fix: $M
```

## Examples

### Python

#### Don't

```python
@pytest.mark.slow()
def test_big():
    pass
```

#### Do

```python
@pytest.mark.slow
def test_big():
    pass
```

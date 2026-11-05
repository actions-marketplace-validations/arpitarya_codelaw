---
id: py/pytest-fixture-incorrect-parentheses-style
title: "@pytest.fixture, not @pytest.fixture()"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PT001]
since: 0.5.0
---

A fixture with no arguments is decorated `@pytest.fixture`, without empty parentheses.

**Why:** One style across the test suite; the parentheses add nothing.

```codelaw
match:
  kind: call
  pattern: pytest.fixture()
  inside: {kind: decorator}
message: "Drop the empty parentheses"
fix: "pytest.fixture"
```

## Examples

### Python

#### Don't

```python
@pytest.fixture()
def db():
    return connect()
```

#### Do

```python
@pytest.fixture
def db():
    return connect()
```

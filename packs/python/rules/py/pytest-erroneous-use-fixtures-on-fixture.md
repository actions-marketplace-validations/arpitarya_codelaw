---
id: py/pytest-erroneous-use-fixtures-on-fixture
title: "No usefixtures mark on a fixture"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PT025]
since: 0.5.0
---

`@pytest.mark.usefixtures` is not put on a fixture.

**Why:** It has no effect there; a fixture requests other fixtures as parameters.

```codelaw
match:
  kind: decorator
  regex: '^@pytest\.mark\.usefixtures\b'
  inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@pytest\.fixture\b'}}
message: "This mark does nothing on a fixture"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
@pytest.mark.usefixtures("db")
@pytest.fixture
def user():
    return make()
```

#### Do

```python
@pytest.fixture
def user():
    return make()
```

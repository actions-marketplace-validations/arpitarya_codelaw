---
id: py/pytest-deprecated-yield-fixture
title: "@pytest.fixture, not yield_fixture"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PT020]
since: 0.5.0
---

`@pytest.yield_fixture` is written `@pytest.fixture`.

**Why:** `yield_fixture` is deprecated; `fixture` supports `yield` itself.

```codelaw
match:
  kind: attribute
  regex: '^pytest\.yield_fixture$'
  inside: {kind: decorator, stopBy: end}
message: "Write `pytest.fixture`"
fix: "pytest.fixture"
```

## Examples

### Python

#### Don't

```python
@pytest.yield_fixture
def db():
    yield connect()
```

#### Do

```python
@pytest.fixture
def db():
    yield connect()
```

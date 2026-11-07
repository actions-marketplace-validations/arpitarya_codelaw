---
id: py/pytest-fixture-autouse
title: "A pytest fixture is requested, not `autouse=True`"
languages: [python]
fix: manual
fix_reason: "each test that needs the fixture has to name it, and which tests those are is the author's"
fix_exception: restructure
stability: preview
ported_from: [ruff/pytest-fixture-autouse]
since: 0.5.0
---

A `@pytest.fixture(...)` (or `pytest_asyncio.fixture`) decorator passes `autouse=True`. The fixture then runs for every test in its scope whether the test needs it or not. Ruff gives this rule no code; it is selected by its name. `autouse=False` and a fixture without the keyword are fine.

**Why:** A test's setup should be visible in its signature; an autouse fixture runs work, and can change state, that no test asked for.

```codelaw
match:
  kind: keyword_argument
  regex: '^autouse\s*=\s*True$'
  inside:
    kind: argument_list
    inside:
      kind: call
      pattern: $C
      has: {field: function, regex: '^(pytest|pytest_asyncio)\.fixture$|^fixture$'}
      inside: {kind: decorator}
where:
  $C: {imported-from: [pytest, pytest_asyncio]}
message: "Request this fixture where it is needed instead of `autouse=True`"
```

## How to fix

1. Remove `autouse=True`.
2. Add the fixture's name as a parameter of each test that needs it, or apply it with `@pytest.mark.usefixtures(...)`.

## Examples

### Python

#### Don't

```python
import pytest


@pytest.fixture(autouse=True)
def database():
    yield connect()
```

#### Do

```python
import pytest


@pytest.fixture
def database():
    yield connect()


def test_query(database):
    assert database.ping()
```

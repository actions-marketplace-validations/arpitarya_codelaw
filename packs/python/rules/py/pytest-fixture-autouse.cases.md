# py/pytest-fixture-autouse — cases

### case: false, absent, a bare decorator and a plain function are fine

```py
import pytest

@pytest.fixture(autouse=False)
def a(): ...

@pytest.fixture()
def b(): ...

@pytest.fixture
def c(): ...

def d(autouse=True): ...
```

```expect
(none)
```

### case: imported by name, beside a scope

```py
from pytest import fixture

@fixture(scope="module", autouse=True)
def e(): ...
```

```expect
3:26 py/pytest-fixture-autouse
```

---
id: py/pytest-fixture-finalizer-callback
title: "A fixture tears down after yield, not with addfinalizer"
languages: [python]
fix: manual
fix_reason: "the fixture's body is rewritten around a `yield`"
fix_exception: restructure
stability: preview
ported_from: [ruff/PT021]
since: 0.5.0
---

A fixture cleans up after `yield`, not by registering a callback with `request.addfinalizer`.

**Why:** A yield fixture reads in order — set up, hand over, tear down — and its cleanup runs only if the setup succeeded.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^request\.addfinalizer$'}
  inside:
    kind: function_definition
    stopBy: end
    inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@pytest\.fixture\b'}}
message: "Yield the value and tear down after the `yield`"
```

## How to fix

1. Replace `return value` with `yield value`, and move the finalizer's body after the `yield`.

## Examples

### Python

#### Don't

```python
@pytest.fixture
def db(request):
    conn = connect()
    request.addfinalizer(conn.close)
    return conn
```

#### Do

```python
@pytest.fixture
def db():
    conn = connect()
    yield conn
    conn.close()
```

---
id: py/pytest-fixture-param-without-value
title: "A fixture used only for its effect is in usefixtures"
languages: [python]
fix: manual
fix_reason: "the parameter moves into a `@pytest.mark.usefixtures` decorator"
fix_exception: restructure
stability: preview
ported_from: [ruff/PT019]
since: 0.5.0
---

A test's parameter whose name starts with `_` — a fixture the test needs only for its side effect — is requested with `@pytest.mark.usefixtures("_name")` instead.

**Why:** A parameter the test never reads looks like a mistake; `usefixtures` says it is wanted for its effect.

```codelaw
match:
  kind: identifier
  regex: '^_'
  any:
    - inside: {kind: parameters}
    - inside: {kind: typed_parameter, inside: {kind: parameters}}
  inside:
    kind: parameters
    stopBy: end
    inside: {kind: function_definition, has: {field: name, regex: '^test'}}
message: "Request the fixture with `@pytest.mark.usefixtures`"
```

## How to fix

1. Remove the parameter and add `@pytest.mark.usefixtures("<name>")` above the test.

## Examples

### Python

#### Don't

```python
def test_query(_db, user):
    assert user.name
```

#### Do

```python
@pytest.mark.usefixtures("_db")
def test_query(user):
    assert user.name
```

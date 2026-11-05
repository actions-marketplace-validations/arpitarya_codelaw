---
id: py/pytest-extraneous-scope-function
title: "No scope='function' on a fixture"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PT003]
since: 0.5.0
---

`@pytest.fixture(scope="function")` drops the argument.

**Why:** `function` is the default scope.

```codelaw
match:
  kind: keyword_argument
  regex: '^scope\s*=\s*["'']function["'']$'
  inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^pytest\.fixture$'}}}
message: "`function` is the default scope"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
@pytest.fixture(scope="function", autouse=True)
def db():
    return connect()
```

#### Do

```python
@pytest.fixture(autouse=True)
def db():
    return connect()
```

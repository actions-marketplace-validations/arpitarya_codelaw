---
id: py/pytest-parameter-with-default-argument
title: "No default values on a test function"
languages: [python]
fix: manual
fix_reason: "whether the value belongs in a fixture or in `parametrize` is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PT028]
since: 0.5.0
---

A test function's parameter has no default value.

**Why:** pytest does not treat a parameter with a default as a fixture, so the default is always used and a fixture of that name is silently ignored.

```codelaw
match:
  any:
    - kind: default_parameter
    - kind: typed_default_parameter
  inside:
    kind: parameters
    inside: {kind: function_definition, has: {field: name, regex: '^test'}}
message: "A test's parameters are fixtures; drop the default"
```

## How to fix

1. Drop the default and provide the value with a fixture or with `@pytest.mark.parametrize`.

## Examples

### Python

#### Don't

```python
def test_limit(n=10):
    assert take(n)
```

#### Do

```python
@pytest.mark.parametrize("n", [10])
def test_limit(n):
    assert take(n)
```

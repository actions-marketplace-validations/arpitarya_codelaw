---
id: py/pytest-use-fixtures-without-parameters
title: "No usefixtures() with nothing in it"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PT026]
since: 0.5.0
---

`@pytest.mark.usefixtures()` with no names is removed.

**Why:** It requests nothing.

```codelaw
match:
  kind: decorator
  regex: '^@pytest\.mark\.usefixtures(\(\s*\))?$'
message: "This mark requests no fixtures"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
@pytest.mark.usefixtures()
def test_x():
    pass
```

#### Do

```python
def test_x():
    pass
```

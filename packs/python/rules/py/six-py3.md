---
id: py/six-py3
title: "No six.PY3"
languages: [python]
fix: manual
fix_reason: "the Python 2 branch is removed, which restructures the code"
fix_exception: restructure
stability: preview
ported_from: [ruff/YTT202]
since: 0.5.0
---

`six.PY3` is not tested.

**Why:** It is false on a future Python 4, which then takes the Python 2 branch; and Python 2 is long gone.

```codelaw
match:
  kind: attribute
  regex: '^six\.PY3$'
message: "`six.PY3` is false on Python 4; drop the Python 2 branch"
```

## How to fix

1. Keep the Python 3 branch and delete the test.

## Examples

### Python

#### Don't

```python
if six.PY3:
    text = data.decode()
```

#### Do

```python
text = data.decode()
```

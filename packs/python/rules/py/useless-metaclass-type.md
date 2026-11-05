---
id: py/useless-metaclass-type
title: "No __metaclass__ = type"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP001]
since: 0.5.0
---

A class body does not assign `__metaclass__ = type`.

**Why:** In Python 3 every class is already of type `type`, and `__metaclass__` is ignored.

```codelaw
match:
  kind: expression_statement
  regex: '^__metaclass__\s*=\s*type$'
  inside: {kind: block, inside: {kind: class_definition}}
message: "`__metaclass__` does nothing in Python 3"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
class A:
    __metaclass__ = type
    x = 1
```

#### Do

```python
class A:
    x = 1
```

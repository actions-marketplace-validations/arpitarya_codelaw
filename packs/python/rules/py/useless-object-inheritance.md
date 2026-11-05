---
id: py/useless-object-inheritance
title: "No explicit object base"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP004]
since: 0.5.0
---

A class does not list `object` as its only base.

**Why:** Every Python 3 class inherits from `object`.

```codelaw
match:
  kind: argument_list
  regex: '^\(\s*object\s*\)$'
  inside: {kind: class_definition}
message: "Every class already inherits from `object`"
fix: ""
```

## Examples

### Python

#### Don't

```python
class A(object):
    pass
```

#### Do

```python
class A:
    pass
```

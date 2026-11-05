---
id: py/useless-class-metaclass-type
title: "No metaclass=type"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP050]
since: 0.5.0
---

A class does not pass `metaclass=type`.

**Why:** `type` is already every class's metaclass.

```codelaw
match:
  kind: keyword_argument
  regex: '^metaclass\s*=\s*type$'
  inside: {kind: argument_list, inside: {kind: class_definition}}
message: "`metaclass=type` is the default"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
class A(Base, metaclass=type):
    pass
```

#### Do

```python
class A(Base):
    pass
```

---
id: py/dataclass-enum
title: "No @dataclass on an Enum"
languages: [python]
fix: suggest
fix_reason: "removing the decorator drops the generated methods the enum did not use"
stability: preview
ported_from: [ruff/RUF049]
since: 0.5.0
---

An `Enum` subclass is not decorated `@dataclass`.

**Why:** Enum members are created by the metaclass; the dataclass machinery conflicts with it and the docs say not to combine them.

```codelaw
match:
  kind: decorator
  regex: '^@(dataclasses\.)?dataclass\b'
  inside:
    kind: decorated_definition
    has: {field: definition, kind: class_definition, has: {field: superclasses, regex: '\b(Enum|IntEnum|StrEnum|Flag|IntFlag)\b'}}
message: "An Enum is not a dataclass; remove `@dataclass`"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
@dataclass
class Color(Enum):
    RED = 1
```

#### Do

```python
class Color(Enum):
    RED = 1
```

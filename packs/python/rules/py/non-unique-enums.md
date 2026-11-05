---
id: py/non-unique-enums
title: "Each enum member has its own value"
languages: [python]
fix: manual
fix_reason: "the intended value is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PIE796]
since: 0.5.0
---

Two members of an `Enum` class do not share a value.

**Why:** The second becomes an alias of the first, which is rarely meant.

```codelaw
match:
  kind: expression_statement
  has: {kind: assignment, has: {field: right, pattern: $V, any: [{kind: integer}, {kind: string}, {kind: float}]}}
  follows: {stopBy: end, kind: expression_statement, has: {kind: assignment, has: {field: right, pattern: $V}}}
  inside: {kind: block, inside: {kind: class_definition, has: {field: superclasses, regex: '\b(Enum|IntEnum|StrEnum|Flag|IntFlag)\b'}}}
message: "This value is already used by another member"
```

## How to fix

1. Give the member its own value, or make the alias explicit.

## Examples

### Python

#### Don't

```python
class Color(Enum):
    RED = 1
    BLUE = 1
```

#### Do

```python
class Color(Enum):
    RED = 1
    BLUE = 2
```

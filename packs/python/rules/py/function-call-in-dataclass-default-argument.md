---
id: py/function-call-in-dataclass-default-argument
title: "No call as a dataclass field default"
languages: [python]
fix: manual
fix_reason: "whether the value is meant to be shared or built per instance is the author's call"
fix_pending: "suggest-choices; needs add-import (W-162)"
stability: preview
ported_from: [ruff/RUF009]
since: 0.5.0
---

A dataclass field does not default to the result of a call (other than `field()` and known immutable constructors); the call runs once, at class definition.

**Why:** Every instance shares the one value the call returned, which surprises when it is mutable or time-dependent.

```codelaw
match:
  kind: assignment
  has: {field: type, kind: type}
  all:
    - has:
        field: right
        kind: call
        not: {has: {field: function, regex: '^((dataclasses\.)?field|tuple|frozenset|int|float|str|bool|bytes|complex|Decimal|Fraction|Path|PurePath|re\.compile|datetime\.(date|time|timedelta)|timedelta|NewType|TypeVar)$'}}
    - inside:
        kind: class_definition
        stopBy: end
        inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(dataclasses\.)?dataclass\b'}}
    - not: {inside: {kind: function_definition, stopBy: end}}
message: "This call runs once when the class is defined; use `field(default_factory=...)` for a per-instance value"
```

## How to fix

1. If each instance needs its own value, write `field(default_factory=lambda: <the call>)`.
2. If one shared value is meant, move it to a module constant and use the constant.

## Examples

### Python

#### Don't

```python
@dataclass
class Job:
    created: datetime = datetime.now()
```

#### Do

```python
@dataclass
class Job:
    created: datetime = field(default_factory=datetime.now)
```

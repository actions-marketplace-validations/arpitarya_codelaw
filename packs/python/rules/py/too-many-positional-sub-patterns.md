---
id: py/too-many-positional-sub-patterns
title: "A class pattern has no more positional sub-patterns than the class matches"
languages: [python]
fix: manual
fix_reason: "which attributes the extra patterns meant is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/too-many-positional-sub-patterns]
since: 0.5.0
---

`case C(a, b)` where `C` is a builtin that matches itself (`int`, `str`, `tuple`, ...: one positional sub-pattern) or a class of this file whose `__match_args__` (its own, an in-file ancestor's, or a plain `@dataclass`'s fields) is shorter than the patterns given. A class with an imported base, a `NamedTuple` or a builtin base, an invalid `__match_args__`, or none that can be known is not read.

**Why:** Python raises `TypeError: C() accepts N positional sub-patterns (M given)` when the `case` is reached.

```codelaw
native: py/too-many-positional-sub-patterns
```

## How to fix

1. Drop the extra patterns, or
2. name the attributes with keyword patterns, or extend `__match_args__`.

## Examples

### Python

#### Don't

```python
class Point:
    __match_args__ = ("x", "y")


def where(p):
    match p:
        case Point(0, 0, 0):
            return "origin"
```

#### Do

```python
class Point:
    __match_args__ = ("x", "y")


def where(p):
    match p:
        case Point(0, 0):
            return "origin"
```

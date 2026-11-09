---
id: py/match-class-positional-attributes
title: "A class pattern names attributes with keywords"
languages: [python]
fix: suggest
fix_reason: "the keyword form names the attributes `__match_args__` lists today, which is the class's to change; the edit is offered, not applied"
stability: preview
ported_from: [pylint/match-class-positional-attributes]
since: 0.5.0
---

A class pattern gives positional sub-patterns (`case Point(0, 0)`) for a class of this file whose `__match_args__` is known (its own, an in-file ancestor's, or a plain `@dataclass`'s fields). Builtins that match themselves (`int(2)`, `bool(True)`), `NamedTuple` and builtin-derived classes, and classes the file does not define are not read.

**Why:** A positional pattern depends on the order of `__match_args__`; reordering the attributes there silently changes what every `case` matches.

```codelaw
native: py/match-class-positional-attributes
```

## How to fix

1. Write the attribute names: `case Point(x=0, y=0)`.

## Examples

### Python

#### Don't

```python
class Point:
    __match_args__ = ("x", "y")


def where(p):
    match p:
        case Point(0, 0):
            return "origin"
```

#### Fixed

```python
class Point:
    __match_args__ = ("x", "y")


def where(p):
    match p:
        case Point(x=0, y=0):
            return "origin"
```

#### Do

```python
class Point:
    __match_args__ = ("x", "y")


def where(p):
    match p:
        case Point(x=0, y=0):
            return "origin"
```

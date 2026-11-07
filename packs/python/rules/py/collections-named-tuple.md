---
id: py/collections-named-tuple
title: "A stub declares a NamedTuple class"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "each field's type is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PYI024]
since: 0.5.0
---

A stub declares a named tuple as a `typing.NamedTuple` subclass, not with `collections.namedtuple(...)`.

**Why:** `collections.namedtuple` gives every field the type `Any`.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^(collections\.)?namedtuple$'}
message: "Declare a `typing.NamedTuple` class"
```

## How to fix

1. Declare `class Point(NamedTuple):` with a typed field per line.

## Examples

### Python

#### Don't

```python path=api.pyi
Point = namedtuple("Point", ["x", "y"])
```

#### Do

```python path=api.pyi
class Point(NamedTuple):
    x: int
    y: int
```

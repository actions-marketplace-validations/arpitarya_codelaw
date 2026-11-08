---
id: py/prefer-typing-namedtuple
title: "A named tuple is a `typing.NamedTuple` class"
languages: [python]
fix: manual
fix_reason: "the class needs a type for each field, which only the author knows"
fix_exception: value
stability: preview
ported_from: [pylint/prefer-typing-namedtuple]
since: 0.5.0
---

A call to `collections.namedtuple` — imported from `collections`, or written `collections.namedtuple` — could be a `typing.NamedTuple` class, which declares each field with its type. Pylint keeps this check off even within its optional extension; it is in no preset. Stub files have their own rule, `py/collections-named-tuple`.

**Why:** A class with annotated fields documents the tuple and lets a type checker see it.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^(collections\.)?namedtuple$'}
where:
  $$MATCH: {imported-from: collections}
message: "Declare this as a `typing.NamedTuple` class"
```

## How to fix

1. Write `class Point(NamedTuple):` with one annotated attribute per field, and import `NamedTuple` from `typing`.

## Examples

### Python

#### Don't

```python
from collections import namedtuple

Point = namedtuple("Point", ["x", "y"])
```

#### Do

```python
from typing import NamedTuple


class Point(NamedTuple):
    x: float
    y: float
```

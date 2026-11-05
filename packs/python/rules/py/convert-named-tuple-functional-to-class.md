---
id: py/convert-named-tuple-functional-to-class
title: "NamedTuple as a class"
languages: [python]
fix: manual
fix_reason: "the call becomes a class body with one annotated line per field"
fix_exception: restructure
stability: preview
ported_from: [ruff/UP014]
since: 0.5.0
---

`Point = NamedTuple("Point", [("x", int)])` is written as a `class Point(NamedTuple):` with annotated fields.

**Why:** The class form reads as a type, and supports defaults, docstrings and methods.

```codelaw
match:
  kind: assignment
  has:
    field: right
    kind: call
    all:
    - has: {field: function, regex: '^(typing\.)?NamedTuple$'}
    - has:
        field: arguments
        has: {kind: list}
message: Write the NamedTuple as a class
```

## How to fix

1. Write `class Name(NamedTuple):` with one `field: type` line per entry.

## Examples

### Python

#### Don't

```python
Point = NamedTuple("Point", [("x", int), ("y", int)])
```

#### Do

```python
class Point(NamedTuple):
    x: int
    y: int
```

---
id: py/convert-typed-dict-functional-to-class
title: "TypedDict as a class"
languages: [python]
fix: manual
fix_reason: "the call becomes a class body with one annotated line per key"
fix_exception: restructure
stability: preview
ported_from: [ruff/UP013]
since: 0.5.0
---

`Point = TypedDict("Point", {"x": int})` is written as a `class Point(TypedDict):` with annotated fields.

**Why:** The class form reads as a type, and supports docstrings and inheritance.

```codelaw
match:
  kind: assignment
  has:
    field: right
    kind: call
    all:
    - has: {field: function, regex: '^(typing\.)?TypedDict$'}
    - has:
        field: arguments
        has:
          kind: dictionary
          has:
            kind: pair
            has: {kind: string, nthChild: 1, regex: '^["''][A-Za-z_]\w*["'']$'}
message: Write the TypedDict as a class
```

## How to fix

1. Write `class Name(TypedDict):` with one `key: type` line per entry.

## Examples

### Python

#### Don't

```python
Point = TypedDict("Point", {"x": int, "y": int})
```

#### Do

```python
class Point(TypedDict):
    x: int
    y: int
```

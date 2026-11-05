---
id: py/no-slots-in-namedtuple-subclass
title: "namedtuple subclasses define __slots__"
languages: [python]
fix: suggest
fix_reason: "instances can no longer take attributes of their own; review code that set any"
stability: preview
ported_from: [ruff/SLOT002]
since: 0.5.0
---

A class that subclasses a `namedtuple(...)` call defines `__slots__`.

**Why:** Without it every instance carries a `__dict__`, losing the namedtuple's compactness.

```codelaw
match:
  kind: class_definition
  has: {field: superclasses, regex: '^\(\s*(collections\.)?namedtuple\('}
  not: {has: {field: body, has: {kind: expression_statement, has: {kind: assignment, has: {field: left, regex: '^__slots__$'}}}}}
  all:
    - has: {field: body, has: {nthChild: 1, pattern: $FIRST}}
message: "Define `__slots__ = ()`"
fix:
  - insert-before: {target: $FIRST, text: "__slots__ = ()\n    "}
```

## Examples

### Python

#### Don't

```python
class Point(namedtuple("Point", "x y")):
    pass
```

#### Do

```python
class Point(namedtuple("Point", "x y")):
    __slots__ = ()
    pass
```

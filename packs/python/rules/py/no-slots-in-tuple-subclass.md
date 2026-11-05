---
id: py/no-slots-in-tuple-subclass
title: "tuple subclasses define __slots__"
languages: [python]
fix: suggest
fix_reason: "instances can no longer take attributes of their own; review code that set any"
stability: preview
ported_from: [ruff/SLOT001]
since: 0.5.0
---

A class whose only base is `tuple` (or `tuple[...]`) defines `__slots__`.

**Why:** Without it every instance carries a `__dict__`.

```codelaw
match:
  kind: class_definition
  has: {field: superclasses, regex: '^\(\s*(typing\.)?(tuple|Tuple)(\[.*\])?\s*\)$'}
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
class Point(tuple):
    pass
```

#### Do

```python
class Point(tuple):
    __slots__ = ()
    pass
```

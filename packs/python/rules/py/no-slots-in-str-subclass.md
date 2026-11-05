---
id: py/no-slots-in-str-subclass
title: "str subclasses define __slots__"
languages: [python]
fix: suggest
fix_reason: "instances can no longer take attributes of their own; review code that set any"
stability: preview
ported_from: [ruff/SLOT000]
since: 0.5.0
---

A class whose only base is `str` defines `__slots__`.

**Why:** Without it every instance carries a `__dict__`, which costs memory and allows stray attributes on what should be a string.

```codelaw
match:
  kind: class_definition
  has: {field: superclasses, regex: '^\(\s*str\s*\)$'}
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
class Name(str):
    pass
```

#### Do

```python
class Name(str):
    __slots__ = ()
    pass
```

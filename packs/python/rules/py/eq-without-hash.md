---
id: py/eq-without-hash
title: "A class with __eq__ defines __hash__"
languages: [python]
fix: suggest
fix_reason: "`__hash__ = None` states what Python already does; define a real `__hash__` instead if instances must be hashable"
stability: preview
ported_from: [ruff/PLW1641]
since: 0.5.0
---

A class that defines `__eq__` also defines `__hash__` (or sets it to `None` on purpose).

**Why:** Defining `__eq__` alone sets `__hash__` to `None`: instances silently stop working in sets and as dict keys.

```codelaw
match:
  kind: class_definition
  all:
  - has:
      field: body
      has:
        kind: function_definition
        has: {field: name, regex: ^__eq__$}
  - not:
      has:
        field: body
        has:
          any:
          - kind: function_definition
            has: {field: name, regex: ^__hash__$}
          - kind: expression_statement
            has:
              kind: assignment
              has: {field: left, regex: ^__hash__$}
          - kind: decorated_definition
            has:
              kind: function_definition
              has: {field: name, regex: ^__hash__$}
  - has:
      field: body
      has: {nthChild: 1, pattern: $FIRST}
message: Define `__hash__` (or `__hash__ = None` if instances are unhashable on purpose)
fix:
- insert-before: {target: $FIRST, text: "__hash__ = None\n    "}
```

## Examples

### Python

#### Don't

```python
class Point:
    def __eq__(self, other):
        return self.x == other.x
```

#### Do

```python
class Point:
    __hash__ = None
    def __eq__(self, other):
        return self.x == other.x
```

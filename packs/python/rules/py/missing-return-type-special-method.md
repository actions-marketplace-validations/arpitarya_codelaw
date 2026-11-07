---
id: py/missing-return-type-special-method
title: "A special method annotates its return type"
languages: [python]
fix: suggest
fix_reason: "suggest when the function returns no value, annotated `-> None`; one that returns a value is manual: its type is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/ANN204]
since: 0.5.0
---

A dunder method (`__init__`, `__enter__`, `__eq__`, …) has a return annotation; `__init__` and the others that return nothing take `-> None`.

**Why:** An annotated return type lets a type checker check every caller, and tells the reader what comes back.

```codelaw
match:
  kind: function_definition
  all:
    - not: {has: {field: return_type, kind: type}}
    - not: {inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(typing\.)?overload$'}}}
    - has: {field: name, regex: '^__\w+__$'}
    - {any: [{inside: {kind: block, inside: {kind: class_definition}}}, {inside: {kind: decorated_definition, inside: {kind: block, inside: {kind: class_definition}}}}]}
  any:
    - all: [{not: {has: {field: body, has: {stopBy: {any: [{kind: function_definition}, {kind: lambda}, {kind: class_definition}]}, any: [{kind: return_statement, regex: '^return\s*\S'}, {kind: yield}]}}}}, {has: {field: parameters, pattern: $P}}]
    - {has: {field: body, has: {stopBy: {any: [{kind: function_definition}, {kind: lambda}, {kind: class_definition}]}, any: [{kind: return_statement, regex: '^return\s*\S'}, {kind: yield}]}}}
message: "Annotate the return type"
fix:
  - insert-after: {target: $P, text: " -> None"}
```

## Examples

### Python

#### Don't

```python
class Store:
    def __init__(self, path):
        self.path = path
```

#### Do

```python
class Store:
    def __init__(self, path) -> None:
        self.path = path
```

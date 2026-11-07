---
id: py/missing-return-type-static-method
title: "A static method annotates its return type"
languages: [python]
fix: suggest
fix_reason: "suggest when the function returns no value, annotated `-> None`; one that returns a value is manual: its type is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/ANN205]
since: 0.5.0
---

A method marked `@staticmethod` has a return annotation.

**Why:** An annotated return type lets a type checker check every caller, and tells the reader what comes back.

```codelaw
match:
  kind: function_definition
  all:
    - not: {has: {field: return_type, kind: type}}
    - not: {inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(typing\.)?overload$'}}}
    - {inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@staticmethod$'}}}
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
class Paths:
    @staticmethod
    def ensure(p):
        p.mkdir(exist_ok=True)
```

#### Do

```python
class Paths:
    @staticmethod
    def ensure(p) -> None:
        p.mkdir(exist_ok=True)
```

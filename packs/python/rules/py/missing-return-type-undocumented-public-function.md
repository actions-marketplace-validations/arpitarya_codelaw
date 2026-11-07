---
id: py/missing-return-type-undocumented-public-function
title: "A public function annotates its return type"
languages: [python]
fix: suggest
fix_reason: "suggest when the function returns no value, annotated `-> None`; one that returns a value is manual: its type is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/ANN201]
since: 0.5.0
---

A public function or method — its name does not start with `_` — has a return annotation. Static and class methods are `py/missing-return-type-static-method`'s and `-class-method`'s; `@overload` stubs are not reported.

**Why:** An annotated return type lets a type checker check every caller, and tells the reader what comes back.

```codelaw
match:
  kind: function_definition
  all:
    - not: {has: {field: return_type, kind: type}}
    - not: {inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(typing\.)?overload$'}}}
    - has: {field: name, regex: '^[^_]'}
    - not: {inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@staticmethod$'}}}
    - not: {inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@classmethod$'}}}
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
def log(msg):
    print(msg)
```

#### Do

```python
def log(msg) -> None:
    print(msg)
```

---
id: py/unannotated-assignment-in-stub
title: "A stub annotates its variables"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "the declared type, `int` or `Final` or a literal type, is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PYI052]
since: 0.5.0
---

A module or class variable in a stub assigned a literal (a number, a string, bytes, `True` or `False`) is declared with its type: `limit: int`, not `limit = 10`. `__all__`, `__match_args__` and `__slots__` are not reported.

**Why:** A stub's assignment says the name exists; only an annotation says what it is.

```codelaw
match:
  kind: assignment
  all:
    - has: {field: left, kind: identifier, not: {regex: '^__(all|match_args|slots)__$'}}
    - has: {field: right, any: [{kind: integer}, {kind: float}, {kind: string}, {kind: 'true'}, {kind: 'false'}, {kind: unary_operator}]}
    - not: {has: {field: type, kind: type}}
    - any:
        - inside: {kind: expression_statement, inside: {kind: module}}
        - inside: {kind: expression_statement, inside: {kind: block, inside: {kind: class_definition}}}
message: "Annotate the variable"
```

## How to fix

1. Annotate the name with its type (`limit: int`), or `Final` if it is a constant.

## Examples

### Python

#### Don't

```python path=api.pyi
limit = 10
```

#### Do

```python path=api.pyi
limit: int
```

---
id: py/type-alias-without-annotation
title: "A stub's type alias is annotated TypeAlias"
languages: [python]
files: ["**/*.pyi"]
fix: auto
fix_target: { python: "3.10" }
stability: preview
ported_from: [ruff/PYI026]
since: 0.5.0
---

In a stub, a module or class variable assigned a type — a subscript (`list[int]`, `Optional[str]`), a `|` union of names, attributes, subscripts and `None`, `Any`, or `None` itself — is annotated as a type alias: `Ids: TypeAlias = list[int]`. The fix adds the annotation and imports `TypeAlias` from `typing` where the file does not. `__all__`, `__slots__` and `__match_args__`, an assignment in a function, and `None` in an enum class are not reported. `TypeAlias` is in `typing` from Python 3.10, so the finding is `auto` from that target and `suggest` below it or without one (`typing_extensions` has it for older targets).

**Why:** Without the annotation a type checker reads the name as a variable that holds a type, not as an alias it may use in annotations.

```codelaw
match:
  kind: assignment
  all:
    - has: {field: left, kind: identifier, pattern: $N, not: {regex: '^__(all|match_args|slots)__$'}}
    - not: {has: {field: type, kind: type}}
    - any:
        - inside: {kind: expression_statement, inside: {kind: module}}
        - inside: {kind: expression_statement, inside: {kind: block, inside: {kind: class_definition}}}
    - has:
        field: right
        any:
          - kind: subscript
          - {kind: identifier, regex: '^Any$'}
          - {kind: attribute, regex: '^(typing|typing_extensions)\.Any$'}
          - all:
              - kind: none
              - not: {inside: {kind: assignment, inside: {kind: expression_statement, inside: {kind: block, inside: {kind: class_definition, has: {field: superclasses, regex: 'Enum|Flag'}}}}}}
          - all:
              - kind: binary_operator
              - has: {field: operator, regex: '^\|$'}
              - not:
                  has:
                    stopBy: {kind: subscript}
                    any:
                      - {kind: integer}
                      - {kind: float}
                      - {kind: string}
                      - {kind: call}
                      - {kind: 'true'}
                      - {kind: 'false'}
                      - {kind: list}
                      - {kind: tuple}
                      - {kind: dictionary}
                      - {kind: unary_operator}
                      - {kind: lambda}
                      - {kind: binary_operator, not: {has: {field: operator, regex: '^\|$'}}}
message: "Annotate `$N` as a `TypeAlias`"
fix:
  - replace: { target: $N, with: "$N: TypeAlias" }
  - ensure-import: { module: typing, name: TypeAlias }
```

## How to fix

1. Annotate the name `TypeAlias`, imported from `typing` (`typing_extensions` before Python 3.10).

## Examples

### Python

#### Don't

```python path=api.pyi
from typing import Optional

MaybeId = Optional[int]
```

#### Do

```python path=api.pyi
from typing import Optional, TypeAlias

MaybeId: TypeAlias = Optional[int]
```

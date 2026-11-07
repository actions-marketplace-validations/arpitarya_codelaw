---
id: py/redundant-literal-union
title: "A union does not hold both a literal and its type"
languages: [python]
fix: manual
fix_reason: "dropping the literal may leave an empty `Literal[]` or a one-member union to rewrite; whether the type or the literal was meant is the author's"
fix_exception: restructure
stability: preview
ported_from: [ruff/PYI051]
since: 0.5.0
---

A union names a type and also a `Literal[...]` of that type: `str | Literal["auto"]`, `Union[Literal[5], int]`. The literal adds nothing, since the type already covers it. Each literal member whose type is in the union is reported: a string with `str`, a bytes string with `bytes`, an integer with `int`, `True` or `False` with `bool`. Types match exactly, as upstream: `Literal[True] | int` and `Literal[5] | float` are left alone. A union written with `|` or with `Union[...]` is read, in any file.

**Why:** The literal reads as a special value the code treats differently, and the type says any value of it is accepted.

```codelaw
utils:
  member-str: {kind: identifier, regex: '^str$', inside: {any: [{kind: binary_operator}, {kind: subscript, has: {field: value, regex: '^((typing|typing_extensions)\.)?Union$'}}, {kind: type, inside: {kind: type_parameter, inside: {kind: generic_type, has: {nthChild: 1, regex: '^((typing|typing_extensions)\.)?Union$'}}}}]}}
  member-bytes: {kind: identifier, regex: '^bytes$', inside: {any: [{kind: binary_operator}, {kind: subscript, has: {field: value, regex: '^((typing|typing_extensions)\.)?Union$'}}, {kind: type, inside: {kind: type_parameter, inside: {kind: generic_type, has: {nthChild: 1, regex: '^((typing|typing_extensions)\.)?Union$'}}}}]}}
  member-int: {kind: identifier, regex: '^int$', inside: {any: [{kind: binary_operator}, {kind: subscript, has: {field: value, regex: '^((typing|typing_extensions)\.)?Union$'}}, {kind: type, inside: {kind: type_parameter, inside: {kind: generic_type, has: {nthChild: 1, regex: '^((typing|typing_extensions)\.)?Union$'}}}}]}}
  member-bool: {kind: identifier, regex: '^bool$', inside: {any: [{kind: binary_operator}, {kind: subscript, has: {field: value, regex: '^((typing|typing_extensions)\.)?Union$'}}, {kind: type, inside: {kind: type_parameter, inside: {kind: generic_type, has: {nthChild: 1, regex: '^((typing|typing_extensions)\.)?Union$'}}}}]}}
  in-literal:
    inside:
      stopBy: {any: [{kind: subscript}, {kind: generic_type}]}
      any: [{kind: subscript}, {kind: generic_type}]
      has: {nthChild: 1, regex: '^((typing|typing_extensions)\.)?Literal$'}
match:
  all:
    - matches: in-literal
    - any:
        - all:
            - {kind: string, not: {regex: '^[rRuU]*[bB]'}}
            - inside: {stopBy: end, any: [{kind: binary_operator}, {kind: subscript, has: {field: value, regex: '^((typing|typing_extensions)\.)?Union$'}}, {kind: generic_type, has: {nthChild: 1, regex: '^((typing|typing_extensions)\.)?Union$'}}], has: {stopBy: end, matches: member-str}}
        - all:
            - {kind: string, regex: '^[rR]?[bB]'}
            - inside: {stopBy: end, any: [{kind: binary_operator}, {kind: subscript, has: {field: value, regex: '^((typing|typing_extensions)\.)?Union$'}}, {kind: generic_type, has: {nthChild: 1, regex: '^((typing|typing_extensions)\.)?Union$'}}], has: {stopBy: end, matches: member-bytes}}
        - all:
            - {kind: integer}
            - inside: {stopBy: end, any: [{kind: binary_operator}, {kind: subscript, has: {field: value, regex: '^((typing|typing_extensions)\.)?Union$'}}, {kind: generic_type, has: {nthChild: 1, regex: '^((typing|typing_extensions)\.)?Union$'}}], has: {stopBy: end, matches: member-int}}
        - all:
            - {any: [{kind: "true"}, {kind: "false"}]}
            - inside: {stopBy: end, any: [{kind: binary_operator}, {kind: subscript, has: {field: value, regex: '^((typing|typing_extensions)\.)?Union$'}}, {kind: generic_type, has: {nthChild: 1, regex: '^((typing|typing_extensions)\.)?Union$'}}], has: {stopBy: end, matches: member-bool}}
message: "The union already holds this literal's type"
```

## How to fix

1. Remove the literal member, or the type, whichever was not meant.

## Examples

### Python

#### Don't

```python
from typing import Literal


def run(mode: str | Literal["auto"]) -> None: ...
```

#### Do

```python
from typing import Literal


def run(mode: Literal["auto", "manual"]) -> None: ...
```

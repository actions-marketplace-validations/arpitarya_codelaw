---
id: py/replace-str-enum
title: "A string enum inherits StrEnum"
languages: [python]
fix: auto
fix_reason: "`StrEnum` formats a member as its value, as `str, Enum` did before Python 3.11; code that relies on 3.11's `Foo.BAR` formatting sees the value again"
fix_target: { python: "3.11" }
stability: preview
ported_from: [ruff/UP042]
since: 0.5.0
---

A class whose bases are `str` and `enum.Enum` (`Enum` imported from `enum`, or `enum.Enum`) inherits `enum.StrEnum` instead. With exactly those two bases the fix writes `(StrEnum)`, importing it from `enum`, or `(enum.StrEnum)`; a class with other bases or keywords is reported and changed by hand. Ruff reports it only when the target is 3.11 or later; here the finding is `auto` from 3.11 and `suggest` below or without a target, since `StrEnum` is new in 3.11 — on an older project, leave the rule off.

**Why:** `StrEnum` says the members are strings in one base, and keeps `f"{Color.RED}"` the value on every Python from 3.11, where `str, Enum` changed to print the member's name.

```codelaw
match:
  kind: class_definition
  all:
    - has: {field: superclasses, has: {kind: identifier, regex: '^str$'}}
    - any:
        - all:
            - has: {field: superclasses, has: {kind: identifier, regex: '^Enum$'}}
            - inside: {kind: module, stopBy: end, has: {kind: import_from_statement, regex: '^from\s+enum\s+import\b[^\n]*\bEnum\b'}}
            - any:
                - all: [{has: {field: superclasses, pattern: $ARGS, all: [{has: {nthChild: 2}}, {not: {has: {nthChild: 3}}}, {not: {has: {kind: keyword_argument}}}]}}, {has: {field: superclasses, pattern: $BARE}}]
                - regex: ''
        - all:
            - has: {field: superclasses, has: {kind: attribute, regex: '^enum\.Enum$'}}
            - any:
                - all: [{has: {field: superclasses, all: [{has: {nthChild: 2}}, {not: {has: {nthChild: 3}}}, {not: {has: {kind: keyword_argument}}}]}}, {has: {field: superclasses, pattern: $QUAL}}]
                - regex: ''
    - has: {field: name, pattern: $NAME}
message: "`$NAME` inherits `str` and `Enum`: inherit `StrEnum`"
fix:
  - replace: { target: $BARE, with: "(StrEnum)" }
  - replace: { target: $QUAL, with: "(enum.StrEnum)" }
  - ensure-import: { target: $BARE, module: enum, name: StrEnum }
```

## How to fix

1. Replace the two bases with `StrEnum`, imported from `enum`.

## Examples

### Python

#### Don't

```python
from enum import Enum


class Color(str, Enum):
    RED = "red"
```

#### Do

```python
from enum import Enum, StrEnum


class Color(StrEnum):
    RED = "red"
```

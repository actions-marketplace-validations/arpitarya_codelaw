---
id: py/mutable-class-default
title: "Mutable class attribute is ClassVar"
languages: [python]
fix: manual
fix_reason: "the fix needs `ClassVar` imported, and whether sharing was meant is the author's"
fix_pending: "suggest; needs add-import (W-162)"
stability: preview
ported_from: [ruff/RUF012]
since: 0.5.0
---

A class attribute whose default is a list, dict or set literal and whose annotation is not `ClassVar` is reported (outside dataclasses, which RUF008 covers).

**Why:** The one value is shared by every instance; annotating it `ClassVar` makes that visible to readers and type checkers.

```codelaw
match:
  kind: assignment
  all:
    - has: {field: type, pattern: $T}
    - has: {field: right, any: [{kind: list}, {kind: dictionary}, {kind: set}]}
    - inside: {kind: expression_statement, inside: {kind: block, inside: {kind: class_definition}}}
    - not: {inside: {kind: decorated_definition, stopBy: end, has: {kind: decorator, regex: '^@(dataclasses\.)?dataclass\b'}}}
constraints:
  T: {not: {regex: '^(typing\.)?(ClassVar|Final)\b'}}
message: "A mutable class default is shared; annotate it `ClassVar[...]`"
```

## How to fix

1. Annotate it `ClassVar[<type>]`, or make it immutable (a tuple or `frozenset`).

## Examples

### Python

#### Don't

```python
class Registry:
    names: list[str] = []
```

#### Do

```python
class Registry:
    names: ClassVar[list[str]] = []
```

---
id: py/non-pep604-annotation-union
title: "X | Y, not Union[X, Y]"
languages: [python]
fix: suggest
fix_reason: "`|` in annotations needs Python 3.10, or `from __future__ import annotations`"
stability: preview
ported_from: [ruff/UP007, pylint/consider-alternative-union-syntax]
since: 0.5.0
---

A two-member `Union[X, Y]` annotation is written `X | Y` (Python 3.10+).

**Why:** The operator is shorter and needs no import.

```codelaw
match:
  kind: generic_type
  regex: ^(typing\.)?Union\[
  all:
  - has:
      kind: type_parameter
      has: {kind: type, nthChild: 1, pattern: $A}
      not:
        has: {kind: type, nthChild: 3}
  - has:
      kind: type_parameter
      has: {kind: type, nthChild: 2, pattern: $B}
message: Write `$A | $B`
fix: $A | $B
```

## Examples

### Python

#### Don't

```python
def f(x: Union[int, str]) -> None:
    pass
```

#### Do

```python
def f(x: int | str) -> None:
    pass
```

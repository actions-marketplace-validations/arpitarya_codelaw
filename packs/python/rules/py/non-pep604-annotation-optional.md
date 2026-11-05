---
id: py/non-pep604-annotation-optional
title: "X | None, not Optional[X]"
languages: [python]
fix: suggest
fix_reason: "`|` in annotations needs Python 3.10, or `from __future__ import annotations`"
stability: preview
ported_from: [ruff/UP045]
since: 0.5.0
---

`Optional[X]` is written `X | None` (Python 3.10+).

**Why:** The operator is shorter and needs no import.

```codelaw
match:
  kind: generic_type
  regex: '^(typing\.)?Optional\['
  has: {kind: type_parameter, has: {kind: type, nthChild: 1, pattern: $X}, not: {has: {kind: type, nthChild: 2}}}
message: "Write `$X | None`"
fix: "$X | None"
```

## Examples

### Python

#### Don't

```python
def f(x: Optional[int]) -> None:
    pass
```

#### Do

```python
def f(x: int | None) -> None:
    pass
```

---
id: py/redundant-typehint-argument
title: "A variable's union annotation names each type once"
languages: [python]
fix: auto
stability: preview
ported_from: [pylint/redundant-typehint-argument]
since: 0.5.0
---

A variable annotation names one type twice in a union — `x: int | int`, `x: Union[str, str]` — or wraps `None` in `Optional`: `x: Optional[None]`. The duplicate is dropped. Only a variable's annotation is read, as Pylint's check does; a union of two that are the same, written side by side, is the shape fixed here, and a longer union with a repeat further along is not reported.

**Why:** The repeat adds nothing to the type and suggests a different type was meant.

```codelaw
match:
  any:
    - {kind: binary_operator, all: [{pattern: $A | $A}, {pattern: $U}]}
    - kind: generic_type
      pattern: $U
      all:
        - has: {nthChild: 1, kind: identifier, regex: '^Union$'}
        - has:
            kind: type_parameter
            not: {has: {nthChild: 3}}
            has: {nthChild: 1, kind: type, pattern: $A, precedes: {stopBy: end, kind: type, pattern: $A}}
    - kind: generic_type
      pattern: $N
      all:
        - has: {nthChild: 1, kind: identifier, regex: '^Optional$'}
        - has: {kind: type_parameter, not: {has: {nthChild: 2}}, has: {kind: type, regex: '^None$'}}
  inside:
    kind: type
    inside: {kind: assignment, field: type}
message: "This union repeats a type"
fix:
  - replace: { target: $U, with: "$A" }
  - replace: { target: $N, with: "None" }
```

## Examples

### Python

#### Don't

```python
count: int | int = 0
```

#### Do

```python
count: int = 0
```

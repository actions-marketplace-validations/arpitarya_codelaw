---
id: py/unnecessary-literal-union
title: "One Literal, not a union of Literals"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PYI030]
since: 0.5.0
---

`Literal[1] | Literal[2]` is written `Literal[1, 2]`.

**Why:** One `Literal` lists every value.

```codelaw
match:
  kind: union_type
  all:
  - not:
      has: {kind: type, nthChild: 3}
  - has:
      kind: type
      nthChild: 1
      has:
        kind: generic_type
        regex: ^Literal\[
        has:
          kind: type_parameter
          not:
            has: {kind: type, nthChild: 2}
          has: {kind: type, pattern: $A}
  - has:
      kind: type
      nthChild: 2
      has:
        kind: generic_type
        regex: ^Literal\[
        has:
          kind: type_parameter
          not:
            has: {kind: type, nthChild: 2}
          has: {kind: type, pattern: $B}
message: Write `Literal[$A, $B]`
fix: Literal[$A, $B]
```

## Examples

### Python

#### Don't

```python
def f(x: Literal[1] | Literal[2]) -> None: ...
```

#### Do

```python
def f(x: Literal[1, 2]) -> None: ...
```

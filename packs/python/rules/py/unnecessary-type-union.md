---
id: py/unnecessary-type-union
title: "type[A | B], not type[A] | type[B]"
languages: [python]
fix: suggest
fix_reason: "the annotation text changes; the type is the same"
stability: preview
ported_from: [ruff/PYI055]
since: 0.5.0
---

`type[A] | type[B]` is written `type[A | B]`.

**Why:** One `type[...]` reads as the one thing it is.

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
        regex: ^type\[
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
        regex: ^type\[
        has:
          kind: type_parameter
          not:
            has: {kind: type, nthChild: 2}
          has: {kind: type, pattern: $B}
message: Write `type[$A | $B]`
fix: type[$A | $B]
```

## Examples

### Python

#### Don't

```python
def f(x: type[int] | type[str]) -> None: ...
```

#### Do

```python
def f(x: type[int | str]) -> None: ...
```

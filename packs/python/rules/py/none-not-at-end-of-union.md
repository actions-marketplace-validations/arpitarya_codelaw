---
id: py/none-not-at-end-of-union
title: "None goes last in a union"
languages: [python]
fix: manual
fix_reason: "reordering a union of more than two members is left to the author"
fix_pending: "suggest; needs computed-edit"
stability: preview
ported_from: [ruff/RUF036]
since: 0.5.0
---

In a `|` union, `None` is the last member: `int | str | None`.

**Why:** One place for `None` makes optional types read the same everywhere.

```codelaw
match:
  kind: binary_operator
  has: {field: operator, regex: '^\|$'}
  all:
    - has: {field: left, stopBy: end, kind: none}
    - not: {inside: {kind: binary_operator, has: {field: operator, regex: '^\|$'}}}
  inside: {kind: type, stopBy: end}
message: "Put `None` last in the union"
```

## How to fix

1. Move `None` to the end of the union.

## Examples

### Python

#### Don't

```python
def f(x: None | int) -> None:
    pass
```

#### Do

```python
def f(x: int | None) -> None:
    pass
```

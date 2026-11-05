---
id: py/never-union
title: "No Never or NoReturn in a union"
languages: [python]
fix: suggest
fix_reason: "removing the member keeps the type equal; the annotation text changes"
stability: preview
ported_from: [ruff/RUF020]
since: 0.5.0
---

`Never` and `NoReturn` are not members of a union; they add nothing to it.

**Why:** `T | Never` is `T`; the extra member suggests a meaning it does not have.

```codelaw
match:
  kind: binary_operator
  has: {field: operator, regex: '^\|$'}
  any:
    - pattern: $T | Never
    - pattern: $T | NoReturn
    - pattern: $T | typing.Never
    - pattern: $T | typing.NoReturn
message: "`Never` in a union adds nothing; write `$T`"
fix: "$T"
```

## Examples

### Python

#### Don't

```python
def f(x: int | Never) -> None:
    pass
```

#### Do

```python
def f(x: int) -> None:
    pass
```

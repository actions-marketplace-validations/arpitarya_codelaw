---
id: py/none-not-at-end-of-union
title: "None goes last in a union"
languages: [python]
fix: suggest
fix_reason: "a union's order is what `typing.get_args` returns at run time, which code may read; a `None` inside a member (`list[None] | int`) is moved by hand"
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
fix:
  - edit-text: { pattern: '\ANone[ \t]*\|[ \t]*([^\n]*)\z', replace: '${1} | None' }
  - edit-text: { pattern: '\A([^\n]*?)[ \t]*\|[ \t]*None[ \t]*\|([^\n]*)\z', replace: '${1} |${2} | None' }
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

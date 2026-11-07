---
id: py/pep484-style-positional-only-parameter
title: "Positional-only parameters use /"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "the parameter is renamed and a `/` placed after the last positional-only one"
fix_exception: restructure
stability: preview
ported_from: [ruff/PYI063]
since: 0.5.0
---

A stub marks positional-only parameters with `/`, not with PEP 484's double-underscore prefix (`def f(__x)`).

**Why:** `/` is the syntax for it since Python 3.8; the prefix is a convention only type checkers understand.

```codelaw
match:
  kind: identifier
  regex: '^__\w*[^_]_?$'
  any:
    - inside: {kind: parameters}
    - inside: {kind: typed_parameter, inside: {kind: parameters}}
    - inside: {kind: typed_default_parameter, field: name, inside: {kind: parameters}}
  not: {inside: {kind: parameters, has: {kind: positional_separator}}}
message: "Mark positional-only parameters with `/`"
```

## How to fix

1. Rename the parameters without the `__` prefix and put `/` after the last positional-only one.

## Examples

### Python

#### Don't

```python path=api.pyi
def f(__x: int) -> None: ...
```

#### Do

```python path=api.pyi
def f(x: int, /) -> None: ...
```

---
id: py/typed-argument-default-in-stub
title: "Simple defaults only in stubs"
languages: [python]
files: ["**/*.pyi"]
fix: auto
stability: preview
ported_from: [ruff/PYI011]
since: 0.5.0
---

A typed parameter's default in a stub is `...` unless it is a short literal (a number, short string, `None`, `True`, `False`).

**Why:** The default's value is not part of the interface type checkers read; `...` keeps stubs simple.

```codelaw
match:
  kind: typed_default_parameter
  has: {field: value, pattern: $V, not: {any: [{kind: integer}, {kind: float}, {kind: none}, {kind: 'true'}, {kind: 'false'}, {kind: ellipsis}, {kind: string, regex: '^.{0,52}$'}, {kind: unary_operator, regex: '^-\d'}]}}
message: "Write the default as `...`"
fix:
  - replace: {target: $V, with: "..."}
```

## Examples

### Python

#### Don't

```python path=api.pyi
def f(x: list[int] = [1, 2]) -> None: ...
```

#### Do

```python path=api.pyi
def f(x: list[int] = ...) -> None: ...
```

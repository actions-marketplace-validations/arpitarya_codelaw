---
id: py/implicit-optional
title: "Optional parameters say Optional"
languages: [python]
fix: suggest
fix_reason: "the annotation changes; review where the type is read at runtime"
stability: preview
ported_from: [ruff/RUF013]
since: 0.5.0
---

A parameter that defaults to `None` is annotated `T | None`, not just `T`.

**Why:** PEP 484 removed implicit Optional; type checkers then report every call that passes `None`.

```codelaw
match:
  kind: typed_default_parameter
  all:
    - has: {field: value, kind: none}
    - has: {field: type, pattern: $T}
constraints:
  T: {not: {regex: '(None|Optional|Any|object)\b'}}
message: "Annotate `$T | None`; a `None` default does not make the type optional"
fix:
  - replace: {target: $T, with: "$T | None"}
```

## Examples

### Python

#### Don't

```python
def greet(name: str = None):
    pass
```

#### Do

```python
def greet(name: str | None = None):
    pass
```

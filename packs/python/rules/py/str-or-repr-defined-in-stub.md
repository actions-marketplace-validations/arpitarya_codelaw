---
id: py/str-or-repr-defined-in-stub
title: "No __str__ or __repr__ in stubs"
languages: [python]
files: ["**/*.pyi"]
fix: auto
stability: preview
ported_from: [ruff/PYI029]
since: 0.5.0
---

A stub class does not declare `__str__` or `__repr__` returning `str`.

**Why:** `object` already declares them; the declaration adds nothing.

```codelaw
match:
  kind: function_definition
  all:
  - has: {field: name, regex: ^__(str|repr)__$}
  - has: {field: parameters, regex: ^\(\s*self\s*\)$}
  - has: {field: return_type, regex: ^str$}
  - not:
      inside: {kind: decorated_definition}
  - inside:
      kind: block
      inside: {kind: class_definition}
message: '`object` already declares this'
fix: [remove]
```

## Examples

### Python

#### Don't

```python path=api.pyi
class A:
    x: int
    def __repr__(self) -> str: ...
```

#### Do

```python path=api.pyi
class A:
    x: int
```

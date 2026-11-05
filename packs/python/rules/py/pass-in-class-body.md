---
id: py/pass-in-class-body
title: "No pass in a stub class with other members"
languages: [python]
files: ["**/*.pyi"]
fix: auto
stability: preview
ported_from: [ruff/PYI012]
since: 0.5.0
---

A stub class body that has other statements has no `pass`.

**Why:** `pass` is only needed in an empty body, and stubs use `...` for that.

```codelaw
match:
  kind: pass_statement
  inside: {kind: block, has: {nthChild: 2}, inside: {kind: class_definition}}
message: "Remove the `pass`"
fix: [remove]
```

## Examples

### Python

#### Don't

```python path=api.pyi
class A:
    x: int
    pass
```

#### Do

```python path=api.pyi
class A:
    x: int
```

---
id: py/ellipsis-in-non-empty-class-body
title: "No ... in a stub class with other members"
languages: [python]
files: ["**/*.pyi"]
fix: auto
stability: preview
ported_from: [ruff/PYI013]
since: 0.5.0
---

A stub class body that has other statements has no `...` statement.

**Why:** `...` is only needed in an empty body.

```codelaw
match:
  kind: expression_statement
  regex: '^\.\.\.$'
  inside: {kind: block, has: {nthChild: 2}, inside: {kind: class_definition}}
message: "Remove the `...`"
fix: [remove]
```

## Examples

### Python

#### Don't

```python path=api.pyi
class A:
    x: int
    ...
```

#### Do

```python path=api.pyi
class A:
    x: int
```

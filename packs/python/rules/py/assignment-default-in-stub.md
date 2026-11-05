---
id: py/assignment-default-in-stub
title: "Simple values only in stub assignments"
languages: [python]
files: ["**/*.pyi"]
fix: auto
stability: preview
ported_from: [ruff/PYI015]
since: 0.5.0
---

An annotated module or class attribute in a stub is assigned `...` unless the value is a short literal.

**Why:** Type checkers read the annotation; the value only adds noise.

```codelaw
match:
  kind: assignment
  all:
  - has: {field: type, kind: type}
  - has:
      field: right
      pattern: $V
      not:
        any:
        - {kind: integer}
        - {kind: float}
        - {kind: none}
        - {kind: 'true'}
        - {kind: 'false'}
        - {kind: ellipsis}
        - {kind: string, regex: '^.{0,52}$'}
        - {kind: unary_operator, regex: ^-\d}
  - not:
      has: {field: type, regex: TypeAlias}
message: Write the value as `...`
fix:
- replace: {target: $V, with: '...'}
```

## Examples

### Python

#### Don't

```python path=api.pyi
TIMEOUT: float = 60 * 5
```

#### Do

```python path=api.pyi
TIMEOUT: float = ...
```

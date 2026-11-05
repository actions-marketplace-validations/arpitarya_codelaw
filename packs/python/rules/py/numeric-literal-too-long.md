---
id: py/numeric-literal-too-long
title: "No long numeric literals in stubs"
languages: [python]
files: ["**/*.pyi"]
fix: auto
stability: preview
ported_from: [ruff/PYI054]
since: 0.5.0
---

A number with more than 10 characters in a stub is written `...`.

**Why:** Long values are implementation detail, not interface.

```codelaw
match:
  any: [{kind: integer}, {kind: float}]
  regex: '^.{11,}$'
message: "Write `...`"
fix: "..."
```

## Examples

### Python

#### Don't

```python path=api.pyi
LIMIT: int = 12345678901
```

#### Do

```python path=api.pyi
LIMIT: int = ...
```

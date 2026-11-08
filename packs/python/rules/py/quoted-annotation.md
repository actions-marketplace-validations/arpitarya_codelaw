---
id: py/quoted-annotation
title: "No quoted annotation under postponed evaluation"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP037]
since: 0.5.0
---

In a module that has `from __future__ import annotations`, a parameter, return or variable annotation is not written in quotes: `def f(x: User)`, not `def f(x: "User")`.

**Why:** With postponed evaluation every annotation is already a string; the quotes only hide the name from tools.

```codelaw
match:
  kind: string
  pattern: $S
  not: {has: {kind: interpolation}}
  has: {kind: string_content, pattern: $C, regex: '^[A-Za-z_][\w\[\]., |]*$'}
  all:
    - inside: {kind: type}
    - inside:
        kind: module
        stopBy: end
        has: {kind: future_import_statement, regex: '\bannotations\b'}
message: "Drop the quotes around the annotation"
fix:
  - replace: {target: $S, with: $C}
```

## Examples

### Python

#### Don't

```python
from __future__ import annotations


def save(user: "User") -> "Result":
    pass
```

#### Do

```python
from __future__ import annotations


def save(user: User) -> Result:
    pass
```

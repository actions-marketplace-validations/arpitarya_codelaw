---
id: py/future-annotations-in-stub
title: "No __future__ annotations in stubs"
languages: [python]
files: ["**/*.pyi"]
fix: auto
stability: preview
ported_from: [ruff/PYI044]
since: 0.5.0
---

A stub does not import `annotations` from `__future__`.

**Why:** Stubs are never executed; the import does nothing.

```codelaw
match:
  kind: future_import_statement
  regex: '^from\s+__future__\s+import\s+annotations$'
message: "Remove the import"
fix: [remove]
```

## Examples

### Python

#### Don't

```python path=api.pyi
from __future__ import annotations
def f() -> None: ...
```

#### Do

```python path=api.pyi
def f() -> None: ...
```

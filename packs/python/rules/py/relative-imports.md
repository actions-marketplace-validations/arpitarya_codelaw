---
id: py/relative-imports
title: "No relative import from a parent package"
languages: [python]
fix: manual
fix_reason: "the absolute module path depends on the package layout"
fix_exception: value
stability: preview
ported_from: [ruff/TID252]
since: 0.5.0
---

An import does not climb to a parent package with `..`: `from ..models import User` is written with the absolute path. A sibling import (`from . import x`) is not reported.

**Why:** A parent-relative import ties a module to where it sits in the tree, and reads as a puzzle.

```codelaw
match:
  kind: relative_import
  regex: '^\.\.'
message: "Import from the absolute module path"
```

## How to fix

1. Write the import with the package's absolute path: `from app.models import User`.

## Examples

### Python

#### Don't

```python
from ..models import User
```

#### Do

```python
from app.models import User
```

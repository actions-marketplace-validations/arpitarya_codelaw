---
id: py/non-empty-init-module
title: "__init__.py holds imports only"
languages: [python]
files: ["**/__init__.py"]
fix: manual
fix_reason: "where the code moves (a submodule) is the author's"
stability: preview
ported_from: [ruff/RUF067]
since: 0.5.0
---

A package's `__init__.py` defines no functions or classes.

**Why:** Code in `__init__.py` runs on every import of the package and is hard to find; a submodule is clearer.

```codelaw
match:
  any: [{kind: function_definition}, {kind: class_definition}, {kind: decorated_definition}]
  inside: {kind: module}
message: "Move this definition to a submodule and import it here"
```

## How to fix

1. Move the definition to a submodule.
2. Import it in `__init__.py` (and list it in `__all__`).

## Examples

### Python

#### Don't

```python path=pkg/__init__.py
def helper():
    return 1
```

#### Do

```python path=pkg/__init__.py
from .helpers import helper
```

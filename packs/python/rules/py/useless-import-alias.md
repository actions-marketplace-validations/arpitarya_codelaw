---
id: py/useless-import-alias
title: "No import x as x"
languages: [python]
exclude: ["**/*.pyi", "**/__init__.py"]
fix: auto
stability: preview
ported_from: [ruff/PLC0414]
since: 0.5.0
---

`import os as os` and `from a import b as b` drop the alias.

**Why:** The alias renames nothing. (In a stub or `__init__.py` it marks a re-export; those files are left alone.)

```codelaw
match:
  kind: aliased_import
  all:
  - has: {field: name, pattern: $N}
  - has: {field: alias, pattern: $N}
message: The alias renames nothing
fix: $N
```

## Examples

### Python

#### Don't

```python
import os as os
```

#### Do

```python
import os
```

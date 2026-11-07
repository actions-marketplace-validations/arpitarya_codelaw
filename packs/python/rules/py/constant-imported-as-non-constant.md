---
id: py/constant-imported-as-non-constant
title: "A CONSTANT is not imported under a lowercase alias"
languages: [python]
fix: manual
fix_reason: "which alias to give the import is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/N811]
since: 0.5.0
---

An imported name written in capitals, a constant, keeps a capitals alias: `from config import MAX_SIZE as max_size` is reported.

**Why:** The alias hides that the name is a constant.

```codelaw
match:
  kind: aliased_import
  all:
    - has: {field: name, regex: '^_*[A-Z][A-Z0-9_]*$', not: {regex: '\.'}}
    - has: {field: alias, pattern: $A, not: {regex: '^_*[A-Z][A-Z0-9_]*$'}}
message: "`$A` is a constant's alias; write it in capitals"
```

## How to fix

1. Drop the alias, or write it in capitals (`as LIMIT`), and update its uses.

## Examples

### Python

#### Don't

```python
from config import MAX_SIZE as max_size
```

#### Do

```python
from config import MAX_SIZE as LIMIT
```

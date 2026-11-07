---
id: py/lowercase-imported-as-non-lowercase
title: "A lowercase name is not imported under a capitalised alias"
languages: [python]
fix: manual
fix_reason: "which alias to give the import is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/N812]
since: 0.5.0
---

An imported name in lowercase keeps a lowercase alias: `from os import path as Path` is reported.

**Why:** A capitalised alias makes a function or module read as a class or a constant.

```codelaw
match:
  kind: aliased_import
  all:
    - has: {field: name, regex: '^[a-z0-9_.]*[a-z][a-z0-9_.]*$', not: {regex: '[A-Z]'}}
    - has: {field: alias, pattern: $A, regex: '[A-Z]'}
message: "`$A` is a lowercase name's alias; write it in lowercase"
```

## How to fix

1. Drop the alias or write it in lowercase, and update its uses.

## Examples

### Python

#### Don't

```python
from os import path as Path
```

#### Do

```python
from os import path as os_path
```
